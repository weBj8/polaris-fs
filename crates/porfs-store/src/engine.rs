//! The I/O backend: device opening (with the `O_DIRECT`/buffered decision) and
//! batched I/O execution. Two drivers share one closure interface:
//!
//! - [`run_ring_pipelined`]: a single-threaded software pipeline on io_uring.
//!   CPU-side `prepare` of later ops overlaps in-flight device I/O of earlier
//!   ops (fill the SQ, submit, reap, refill).
//! - [`run_sequential`]: plain pread/pwrite for the buffered/tmpfs fallback
//!   (the correctness path). Every transfer result is checked in both.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::{FileExt, OpenOptionsExt};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;

use io_uring::{IoUring, opcode, types};

use crate::StoreError;
use crate::aligned::AlignedBuf;
use crate::store::ExtentStore;

/// Upper bound for io_uring entries (kernel limit for unprivileged rings).
const MAX_RING_ENTRIES: u32 = 32768;
/// Cap on pooled buffers; bounds pool memory for absurd queue depths (the
/// pipeline window is then limited by the pool, never by correctness).
const MAX_POOL_SLOTS: usize = 1024;

impl ExtentStore {
    /// Ensure the pool holds one buffer per pipeline slot, each at least
    /// `required` bytes. Rebuilds when the pool is empty, too small, or stale.
    pub(crate) fn ensure_pool(&mut self, required: usize) -> Result<(), StoreError> {
        let slots = (self.queue_depth as usize).min(MAX_POOL_SLOTS);
        if !self.pool_dirty && self.pool.len() == slots && self.pool_buf_len >= required {
            return Ok(());
        }
        let buf_len = self.pool_buf_len.max(required);
        let mut pool = Vec::with_capacity(slots);
        for _ in 0..slots {
            // The ring path keeps buffers uninitialized (explicit writes and
            // kernel DMA initialize them); the syscall fallback transfers via
            // `&mut [u8]` and therefore needs pre-zeroed buffers.
            pool.push(if self.ring.is_some() {
                AlignedBuf::uninit(buf_len)?
            } else {
                AlignedBuf::zeroed(buf_len)?
            });
        }
        self.pool = pool;
        self.pool_buf_len = buf_len;
        self.pool_dirty = false;
        Ok(())
    }

    /// Run `n` I/O ops through the best available driver: the io_uring software
    /// pipeline (CPU-side prepare overlaps in-flight device I/O) or, on the
    /// buffered fallback, a sequential syscall loop. Same closures, same
    /// semantics: all transfers are checked, errors fail the whole batch.
    pub(crate) fn drive<P, C>(
        &mut self,
        n: usize,
        prepare: P,
        complete: C,
    ) -> Result<(), StoreError>
    where
        P: FnMut(usize, &mut AlignedBuf) -> Result<IoPrep, StoreError>,
        C: FnMut(usize, &AlignedBuf) -> Result<(), StoreError>,
    {
        if n > u32::MAX as usize {
            return Err(StoreError::Internal("batch too large"));
        }
        match self.ring.as_mut() {
            Some(ring) => run_ring_pipelined(
                ring,
                self.file.as_raw_fd(),
                n,
                &mut self.pool,
                prepare,
                complete,
            ),
            None => run_sequential(&self.file, n, &mut self.pool, prepare, complete),
        }
    }
}

/// What one op must transfer, produced by the `prepare` closure.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IoPrep {
    pub(crate) offset: u64,
    pub(crate) len: usize,
    pub(crate) write: bool,
}

/// Probe io_uring availability; see [`crate::io_uring_available`].
pub(crate) fn io_uring_available() -> bool {
    IoUring::new(8).is_ok()
}

/// Create a ring when the device uses O_DIRECT; fall back to `None`
/// (syscalls) when io_uring is unavailable or the device is buffered.
pub(crate) fn new_ring(direct: bool, queue_depth: u32) -> Option<IoUring> {
    if !direct {
        return None;
    }
    let entries = queue_depth.max(2).next_power_of_two().min(MAX_RING_ENTRIES);
    IoUring::new(entries).ok()
}

/// Open the device read/write, preferring `O_DIRECT`. Falls back to buffered
/// I/O when the filesystem rejects `O_DIRECT` at open, and when `O_DIRECT` is
/// accepted but meaningless because the filesystem is tmpfs (e.g. /dev/shm —
/// kernel >= 6.x accepts the flag there and silently stays buffered).
pub(crate) fn open_device(path: &Path, create: bool) -> Result<(File, bool), StoreError> {
    let build = |odirect: bool| {
        let mut opts = OpenOptions::new();
        opts.read(true).write(true);
        if create {
            opts.create(true);
        }
        if odirect {
            opts.custom_flags(libc::O_DIRECT);
        }
        opts.open(path)
    };
    match build(true) {
        Ok(file) if !is_tmpfs(&file) => Ok((file, true)),
        Ok(_) | Err(_) => Ok((build(false)?, false)),
    }
}

/// True if `file` lives on a tmpfs instance (f_type == TMPFS_MAGIC).
fn is_tmpfs(file: &File) -> bool {
    const TMPFS_MAGIC: i64 = 0x0102_1994;
    // SAFETY: `statfs` is a plain-old-data C struct; zero-initializing it is
    // valid, and `fstatfs` writes at most `size_of::<statfs>()` bytes into it.
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: `file` owns a valid open file descriptor; `stat` points to a
    // valid, properly sized `libc::statfs` as required by `fstatfs(2)`.
    let rc = unsafe { libc::fstatfs(file.as_raw_fd(), &mut stat) };
    rc == 0 && stat.f_type == TMPFS_MAGIC
}

/// Sequential pread/pwrite driver (buffered fallback): prepare, transfer,
/// complete, one op at a time. Buffers come from the store pool.
pub(crate) fn run_sequential<P, C>(
    file: &File,
    n: usize,
    pool: &mut [AlignedBuf],
    mut prepare: P,
    mut complete: C,
) -> Result<(), StoreError>
where
    P: FnMut(usize, &mut AlignedBuf) -> Result<IoPrep, StoreError>,
    C: FnMut(usize, &AlignedBuf) -> Result<(), StoreError>,
{
    if pool.is_empty() {
        return Err(StoreError::Internal("empty buffer pool"));
    }
    for op_idx in 0..n {
        let buf = &mut pool[op_idx % pool.len()];
        let prep = prepare(op_idx, buf)?;
        if prep.write {
            file.write_all_at(&buf[..prep.len], prep.offset)?;
        } else {
            file.read_exact_at(&mut buf[..prep.len], prep.offset)?;
            complete(op_idx, buf)?;
        }
    }
    Ok(())
}

/// io_uring software pipeline: while any op is unfinished, fill the SQ with
/// prepared ops (one pool buffer each), submit, reap completions, refill.
/// `user_data` packs `(op_idx << 32) | buf_idx`. Every CQE result is checked;
/// on the first error no new ops are prepared, all in-flight I/Os are drained,
/// and the error is returned (same semantics as a plain batch).
pub(crate) fn run_ring_pipelined<P, C>(
    ring: &mut IoUring,
    fd: RawFd,
    n: usize,
    pool: &mut [AlignedBuf],
    mut prepare: P,
    mut complete: C,
) -> Result<(), StoreError>
where
    P: FnMut(usize, &mut AlignedBuf) -> Result<IoPrep, StoreError>,
    C: FnMut(usize, &AlignedBuf) -> Result<(), StoreError>,
{
    if pool.is_empty() {
        return Err(StoreError::Internal("empty buffer pool"));
    }
    let mut free: Vec<usize> = (0..pool.len()).rev().collect();
    let mut lens: Vec<usize> = Vec::with_capacity(n);
    let mut submitted = 0usize;
    let mut completed = 0usize;
    let mut first_err: Option<StoreError> = None;
    while completed < n {
        while first_err.is_none()
            && submitted < n
            && !free.is_empty()
            && !ring.submission().is_full()
        {
            let op_idx = submitted;
            let Some(buf_idx) = free.pop() else { break };
            let buf = pool
                .get_mut(buf_idx)
                .ok_or(StoreError::Internal("pool index out of range"))?;
            let prep = match prepare(op_idx, buf) {
                Ok(prep) => prep,
                Err(err) => {
                    first_err = Some(err);
                    free.push(buf_idx);
                    break;
                }
            };
            let ptr = buf.as_mut_ptr();
            let entry = if prep.write {
                opcode::Write::new(types::Fd(fd), ptr, prep.len as u32)
                    .offset(prep.offset)
                    .build()
            } else {
                opcode::Read::new(types::Fd(fd), ptr, prep.len as u32)
                    .offset(prep.offset)
                    .build()
            }
            .user_data(((op_idx as u64) << 32) | buf_idx as u64);
            // SAFETY: buffer `buf_idx` is owned by the pool and checked out
            // exclusively to this in-flight op (tracked via `free`); it returns
            // to `free` only after its CQE is reaped, so the kernel never
            // touches a buffer that is in other use. Write ops are fully
            // initialized (header+data+pad) before push; read ops are
            // initialized by the kernel DMA before userspace slices them.
            if unsafe { ring.submission().push(&entry) }.is_err() {
                free.push(buf_idx);
                break;
            }
            lens.push(prep.len);
            submitted += 1;
            // Submit immediately: the device starts on this op while the CPU
            // prepares the next one — this is what makes CPU and I/O overlap.
            if let Err(err) = ring.submit() {
                first_err = Some(err.into());
                break;
            }
        }
        if submitted == completed {
            // Nothing in flight but we could not queue more: cannot progress.
            if first_err.is_none() {
                first_err = Some(StoreError::SubmissionFailed);
            }
            break;
        }
        if let Err(err) = ring.submit_and_wait(1) {
            // Best effort: reap what already completed so checked-out buffers
            // return to the free list before reporting the ring error.
            for cqe in ring.completion() {
                free.push(cqe.user_data() as usize & 0xFFFF_FFFF);
            }
            return Err(err.into());
        }
        while let Some(cqe) = ring.completion().next() {
            completed += 1;
            let op_idx = (cqe.user_data() >> 32) as usize;
            let buf_idx = cqe.user_data() as usize & 0xFFFF_FFFF;
            let result = cqe.result();
            if first_err.is_none() {
                if result < 0 {
                    first_err = Some(std::io::Error::from_raw_os_error(-result).into());
                } else if result as usize != lens[op_idx] {
                    first_err = Some(StoreError::ShortIo {
                        expected: lens[op_idx],
                        got: result as usize,
                    });
                } else if let Err(err) = complete(
                    op_idx,
                    pool.get(buf_idx)
                        .ok_or(StoreError::Internal("pool index out of range"))?,
                ) {
                    first_err = Some(err);
                }
            }
            free.push(buf_idx);
        }
    }
    match first_err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}
