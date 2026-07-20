//! I/O backend: O_DIRECT-aligned buffers, device opening (with the
//! O_DIRECT/buffered decision), and batched I/O execution. Two drivers share
//! one closure interface (`drive`):
//!
//! - `run_ring_pipelined`: single-threaded software pipeline on io_uring.
//!   CPU-side `prepare` of later ops overlaps in-flight device I/O of earlier
//!   ops (fill SQ, submit immediately, reap, refill). Every CQE result is
//!   checked; on the first error no new ops are prepared and all in-flight
//!   I/Os are drained.
//! - `run_sequential`: plain pread/pwrite for the buffered/tmpfs fallback.
//!
//! Adapted from the porfs-store reference engine (same project lineage).

use std::alloc::{Layout, alloc, alloc_zeroed, dealloc};
use std::fs::{File, OpenOptions};
use std::io;
use std::ops::{Deref, DerefMut};
use std::os::unix::fs::{FileExt, OpenOptionsExt};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr::NonNull;

use io_uring::{IoUring, opcode, types};

use crate::{ArenaError, Result};

/// io_uring queue depth (contract-agnostic engine constant).
pub(crate) const QUEUE_DEPTH: u32 = 32;

/// Heap buffer aligned to 4096 bytes, required for O_DIRECT.
///
/// Two flavors: `zeroed` (fully initialized) and `uninit` (no memset, for the
/// hot ring path). An uninit buffer is never read before being fully
/// initialized: write paths fill header+data+pad via `write_bytes` /
/// `zero_range` before the SQE is submitted, and read paths are initialized
/// by the kernel DMA itself before the buffer is ever sliced.
pub(crate) struct AlignedBuf {
    ptr: NonNull<u8>,
    layout: Layout,
}

impl AlignedBuf {
    pub(crate) const ALIGN: usize = 4096;

    /// Allocate `len` zeroed bytes; `len` must be a non-zero multiple of 4096.
    pub(crate) fn zeroed(len: usize) -> io::Result<Self> {
        let layout = checked_layout(len)?;
        // SAFETY: `layout` has non-zero size (checked above); `alloc_zeroed`
        // returns either a valid `len`-byte allocation or null, rejected here.
        let ptr = NonNull::new(unsafe { alloc_zeroed(layout) }).ok_or_else(|| {
            io::Error::new(io::ErrorKind::OutOfMemory, "aligned allocation failed")
        })?;
        Ok(Self { ptr, layout })
    }

    /// Allocate `len` bytes WITHOUT initializing them (same layout rules as
    /// `zeroed`). Callers must uphold the module-level initialization
    /// invariant before any slice is materialized.
    pub(crate) fn uninit(len: usize) -> io::Result<Self> {
        let layout = checked_layout(len)?;
        // SAFETY: `layout` has non-zero size (checked above); `alloc` returns
        // either a valid `len`-byte allocation or null, rejected here.
        let ptr = NonNull::new(unsafe { alloc(layout) }).ok_or_else(|| {
            io::Error::new(io::ErrorKind::OutOfMemory, "aligned allocation failed")
        })?;
        Ok(Self { ptr, layout })
    }

    /// Copy `bytes` into the buffer at `offset`; valid on uninit regions.
    pub(crate) fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        debug_assert!(offset + bytes.len() <= self.layout.size());
        // SAFETY: the debug assertion bounds the destination to the
        // allocation; source and destination never overlap (source is a
        // shared slice from outside this buffer by API contract).
        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.ptr.as_ptr().add(offset),
                bytes.len(),
            )
        }
    }

    /// Zero `len` bytes at `offset`; valid on uninit regions.
    pub(crate) fn zero_range(&mut self, offset: usize, len: usize) {
        debug_assert!(offset + len <= self.layout.size());
        // SAFETY: the debug assertion bounds the range to the allocation;
        // `write_bytes` is a plain memset.
        unsafe { std::ptr::write_bytes(self.ptr.as_ptr().add(offset), 0, len) }
    }

    /// Raw mutable pointer for building io_uring SQEs.
    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }
}

fn checked_layout(len: usize) -> io::Result<Layout> {
    if len == 0 || !len.is_multiple_of(AlignedBuf::ALIGN) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "aligned buffer length must be a non-zero multiple of 4096",
        ));
    }
    Layout::from_size_align(len, AlignedBuf::ALIGN)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid aligned buffer layout"))
}

impl Deref for AlignedBuf {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        // SAFETY: `ptr` is valid for `layout.size()` bytes for the lifetime
        // of `self`; for `zeroed` buffers the memory is initialized at
        // allocation, for `uninit` buffers callers uphold the module-level
        // invariant that the region is fully initialized (explicit writes or
        // kernel DMA) before any slice is materialized. No mutable alias can
        // exist because obtaining one requires `&mut self` (DerefMut).
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.layout.size()) }
    }
}

impl DerefMut for AlignedBuf {
    fn deref_mut(&mut self) -> &mut [u8] {
        // SAFETY: `ptr` is valid for `layout.size()` bytes (same
        // initialization invariant as `deref`), and the `&mut self` borrow
        // guarantees exclusive access.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.layout.size()) }
    }
}

impl Drop for AlignedBuf {
    fn drop(&mut self) {
        // SAFETY: `ptr` was allocated with exactly this `layout` in `zeroed`
        // or `uninit` and is deallocated exactly once, here.
        unsafe { dealloc(self.ptr.as_ptr(), self.layout) }
    }
}

/// What one op must transfer, produced by the `prepare` closure.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IoPrep {
    pub(crate) offset: u64,
    pub(crate) len: usize,
    pub(crate) write: bool,
}

/// The I/O engine: owns the device file, an optional io_uring, and an idle
/// buffer pool (buffers are only checked out inside `drive`, which reaps
/// every completion before returning, so the pool is always idle between
/// calls and can be rebuilt or dropped freely).
pub(crate) struct IoEngine {
    pub(crate) file: File,
    direct: bool,
    ring: Option<IoUring>,
    pool: Vec<AlignedBuf>,
    pool_buf_len: usize,
}

impl IoEngine {
    /// Open `path` read/write preferring O_DIRECT + io_uring; fall back to
    /// buffered syscalls when O_DIRECT is rejected or meaningless (tmpfs).
    pub(crate) fn open(path: &Path, create: bool) -> Result<Self> {
        let (file, direct) = open_device(path, create)?;
        let ring = if direct {
            IoUring::new(QUEUE_DEPTH).ok()
        } else {
            None
        };
        Ok(Self {
            file,
            direct: direct && ring.is_some(),
            ring,
            pool: Vec::new(),
            pool_buf_len: 0,
        })
    }

    /// True when running O_DIRECT + io_uring (false = buffered syscalls).
    pub(crate) fn is_direct(&self) -> bool {
        self.direct
    }

    /// Drop all pooled buffers (called after large scans so big windows are
    /// not held idle for the arena's lifetime).
    pub(crate) fn reset_pool(&mut self) {
        self.pool = Vec::new();
        self.pool_buf_len = 0;
    }

    /// Ensure the pool holds at least `slots` buffers of at least `buf_len`.
    fn ensure_pool(&mut self, buf_len: usize, slots: usize) -> Result<()> {
        if self.pool.len() >= slots && self.pool_buf_len >= buf_len {
            return Ok(());
        }
        let buf_len = self.pool_buf_len.max(buf_len);
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
        Ok(())
    }

    /// Run `n` I/O ops through the best available driver. Same closures, same
    /// semantics in both: all transfers are checked, errors fail the batch.
    pub(crate) fn drive<P, C>(
        &mut self,
        n: usize,
        buf_len: usize,
        prepare: P,
        complete: C,
    ) -> Result<()>
    where
        P: FnMut(usize, &mut AlignedBuf) -> Result<IoPrep>,
        C: FnMut(usize, &AlignedBuf) -> Result<()>,
    {
        if n == 0 {
            return Ok(());
        }
        if n > u32::MAX as usize {
            return Err(internal("batch too large"));
        }
        let slots = (QUEUE_DEPTH as usize).min(n);
        self.ensure_pool(buf_len, slots)?;
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

/// Probe io_uring availability (kernel + seccomp).
pub(crate) fn io_uring_available() -> bool {
    IoUring::new(8).is_ok()
}

/// Open the device read/write, preferring O_DIRECT. Falls back to buffered
/// I/O when the filesystem rejects O_DIRECT at open, and when O_DIRECT is
/// accepted but meaningless because the filesystem is tmpfs (kernel >= 6.x
/// accepts the flag there and silently stays buffered).
pub(crate) fn open_device(path: &Path, create: bool) -> io::Result<(File, bool)> {
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
    const TMPFS_MAGIC: libc::__fsword_t = 0x0102_1994;
    // SAFETY: `statfs` is a plain-old-data C struct; zero-initializing it is
    // valid, and `fstatfs` writes at most `size_of::<statfs>()` bytes into it.
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: `file` owns a valid open file descriptor; `stat` points to a
    // valid, properly sized `libc::statfs` as required by `fstatfs(2)`.
    let rc = unsafe { libc::fstatfs(file.as_raw_fd(), &mut stat) };
    rc == 0 && stat.f_type == TMPFS_MAGIC
}

/// True if `file` is a block device (S_ISBLK).
pub(crate) fn is_block_device(file: &File) -> bool {
    // SAFETY: `stat` is a plain-old-data C struct; zero-initializing it is
    // valid, and `fstat` writes at most `size_of::<stat>()` bytes into it.
    let mut st: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: `file` owns a valid open file descriptor; `st` points to a
    // valid, properly sized `libc::stat` as required by `fstat(2)`.
    let rc = unsafe { libc::fstat(file.as_raw_fd(), &mut st) };
    rc == 0 && (st.st_mode & libc::S_IFMT) == libc::S_IFBLK
}

/// Block device size in bytes via BLKGETSIZE64.
pub(crate) fn block_device_size(file: &File) -> io::Result<u64> {
    const BLKGETSIZE64: libc::c_ulong = 0x8008_1272;
    let mut size: u64 = 0;
    // SAFETY: `file` owns a valid open fd; `size` is a valid u64 out-pointer
    // exactly as BLKGETSIZE64 expects.
    let rc = unsafe { libc::ioctl(file.as_raw_fd(), BLKGETSIZE64, &mut size) };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(size)
}

/// BLKDISCARD `[offset, offset+len)`; returns the ioctl result.
pub(crate) fn blk_discard(file: &File, offset: u64, len: u64) -> io::Result<()> {
    const BLKDISCARD: libc::c_ulong = 0x1277;
    let range: [u64; 2] = [offset, len];
    // SAFETY: `file` owns a valid open fd; `range` points to the two u64s
    // (start, length) exactly as BLKDISCARD expects.
    let rc = unsafe { libc::ioctl(file.as_raw_fd(), BLKDISCARD, range.as_ptr()) };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Sequential pread/pwrite driver (buffered fallback): prepare, transfer,
/// complete, one op at a time. Buffers come from the engine pool.
fn run_sequential<P, C>(
    file: &File,
    n: usize,
    pool: &mut [AlignedBuf],
    mut prepare: P,
    mut complete: C,
) -> Result<()>
where
    P: FnMut(usize, &mut AlignedBuf) -> Result<IoPrep>,
    C: FnMut(usize, &AlignedBuf) -> Result<()>,
{
    if pool.is_empty() {
        return Err(internal("empty buffer pool"));
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
/// `user_data` packs `(op_idx << 32) | buf_idx`. Every CQE result is
/// checked; on the first error no new ops are prepared, all in-flight I/Os
/// are drained, and the error is returned.
fn run_ring_pipelined<P, C>(
    ring: &mut IoUring,
    fd: RawFd,
    n: usize,
    pool: &mut [AlignedBuf],
    mut prepare: P,
    mut complete: C,
) -> Result<()>
where
    P: FnMut(usize, &mut AlignedBuf) -> Result<IoPrep>,
    C: FnMut(usize, &AlignedBuf) -> Result<()>,
{
    if pool.is_empty() {
        return Err(internal("empty buffer pool"));
    }
    let mut free: Vec<usize> = (0..pool.len()).rev().collect();
    let mut lens: Vec<usize> = Vec::with_capacity(n);
    let mut submitted = 0usize;
    let mut completed = 0usize;
    let mut first_err: Option<ArenaError> = None;
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
                .ok_or_else(|| internal("pool index"))?;
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
            // exclusively to this in-flight op (tracked via `free`); it
            // returns to `free` only after its CQE is reaped, so the kernel
            // never touches a buffer that is in other use. Write ops are
            // fully initialized before push; read ops are initialized by the
            // kernel DMA before userspace slices them.
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
            if first_err.is_none() {
                first_err = Some(ArenaError::Io(io::Error::other(
                    "io_uring submission failed",
                )));
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
        for cqe in ring.completion() {
            completed += 1;
            let op_idx = (cqe.user_data() >> 32) as usize;
            let buf_idx = cqe.user_data() as usize & 0xFFFF_FFFF;
            let result = cqe.result();
            if first_err.is_none() {
                if result < 0 {
                    first_err = Some(io::Error::from_raw_os_error(-result).into());
                } else if result as usize != lens[op_idx] {
                    first_err = Some(ArenaError::Io(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!("short io: expected {} bytes, got {result}", lens[op_idx]),
                    )));
                } else if let Err(err) = complete(
                    op_idx,
                    pool.get(buf_idx).ok_or_else(|| internal("pool index"))?,
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

fn internal(msg: &'static str) -> ArenaError {
    ArenaError::Io(io::Error::other(msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(0);

    fn temp_path(tag: &str) -> std::path::PathBuf {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        crate::test_tmpdir().join(format!("plfs-arena-io-{tag}-{}-{n}", std::process::id()))
    }

    #[test]
    fn aligned_buf_write_and_zero() {
        let mut buf = AlignedBuf::uninit(4096).expect("alloc");
        buf.zero_range(0, 4096);
        buf.write_bytes(64, b"hello");
        assert_eq!(&buf[64..69], b"hello");
        assert_eq!(buf[0], 0);
        assert_eq!(buf[4095], 0);
        assert!(AlignedBuf::zeroed(1000).is_err());
    }

    struct Cleanup(std::path::PathBuf);

    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    #[test]
    fn drive_writes_then_reads_back() {
        let path = temp_path("drive");
        let _guard = Cleanup(path.clone());
        {
            let mut engine = IoEngine::open(&path, true).expect("open");
            engine.file.set_len(1 << 20).expect("set_len");
            let payload = vec![0x5Au8; 8192];
            let src = payload.clone();
            engine
                .drive(
                    2,
                    4096,
                    |i, buf| {
                        buf.write_bytes(0, &src[i * 4096..(i + 1) * 4096]);
                        Ok(IoPrep {
                            offset: (i * 4096) as u64,
                            len: 4096,
                            write: true,
                        })
                    },
                    |_, _| Ok(()),
                )
                .expect("write");
            engine.file.sync_data().expect("sync");
            let mut out = vec![0u8; 2 * 4096];
            engine
                .drive(
                    2,
                    4096,
                    |i, _| {
                        Ok(IoPrep {
                            offset: (i * 4096) as u64,
                            len: 4096,
                            write: false,
                        })
                    },
                    |i, buf| {
                        out[i * 4096..(i + 1) * 4096].copy_from_slice(&buf[..4096]);
                        Ok(())
                    },
                )
                .expect("read");
            assert_eq!(out, payload);
        }
    }
}
