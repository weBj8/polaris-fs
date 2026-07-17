//! 4KiB-aligned heap buffers, required for `O_DIRECT` I/O.
//!
//! Two allocation flavors: [`AlignedBuf::zeroed`] (fully initialized) and
//! [`AlignedBuf::uninit`] (no memset, for the hot batch path). An uninit
//! buffer is never read before being fully initialized: the write path
//! initializes header + data + pad via [`AlignedBuf::write_bytes`] /
//! [`AlignedBuf::zero_range`] before its SQE is submitted, and the read path
//! is initialized by the kernel DMA itself before the buffer is ever sliced.

use std::alloc::{Layout, alloc, alloc_zeroed, dealloc};
use std::io;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// A heap buffer aligned to [`AlignedBuf::ALIGN`] bytes.
pub(crate) struct AlignedBuf {
    ptr: NonNull<u8>,
    layout: Layout,
}

impl AlignedBuf {
    /// Buffer alignment; matches the on-disk block size and `O_DIRECT` needs.
    pub(crate) const ALIGN: usize = 4096;

    /// Allocate `len` zeroed bytes; `len` must be a non-zero multiple of `ALIGN`.
    pub(crate) fn zeroed(len: usize) -> io::Result<Self> {
        let layout = checked_layout(len)?;
        // SAFETY: `layout` has non-zero size (checked above); `alloc_zeroed`
        // returns either a valid `len`-byte allocation or null, which we reject.
        let ptr = NonNull::new(unsafe { alloc_zeroed(layout) }).ok_or_else(|| {
            io::Error::new(io::ErrorKind::OutOfMemory, "aligned allocation failed")
        })?;
        Ok(Self { ptr, layout })
    }

    /// Allocate `len` bytes WITHOUT initializing them; same layout rules as
    /// [`AlignedBuf::zeroed`]. See the module docs for the initialization
    /// invariant the caller must uphold.
    pub(crate) fn uninit(len: usize) -> io::Result<Self> {
        let layout = checked_layout(len)?;
        // SAFETY: `layout` has non-zero size (checked above); `alloc` returns
        // either a valid `len`-byte allocation or null, which we reject.
        let ptr = NonNull::new(unsafe { alloc(layout) }).ok_or_else(|| {
            io::Error::new(io::ErrorKind::OutOfMemory, "aligned allocation failed")
        })?;
        Ok(Self { ptr, layout })
    }

    /// Copy `bytes` into the buffer at `offset`. Raw-pointer write, so it is
    /// valid on not-yet-initialized regions of an `uninit` buffer.
    pub(crate) fn write_bytes(&mut self, offset: usize, bytes: &[u8]) {
        debug_assert!(offset + bytes.len() <= self.layout.size());
        // SAFETY: the debug assertion bounds the destination to the allocation;
        // source and destination never overlap (source is a shared slice from
        // outside this buffer by API contract).
        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.ptr.as_ptr().add(offset),
                bytes.len(),
            )
        }
    }

    /// Zero `len` bytes at `offset`. Raw-pointer write, valid on `uninit` buffers.
    pub(crate) fn zero_range(&mut self, offset: usize, len: usize) {
        debug_assert!(offset + len <= self.layout.size());
        // SAFETY: the debug assertion bounds the range to the allocation;
        // `write_bytes` is a plain memset.
        unsafe { std::ptr::write_bytes(self.ptr.as_ptr().add(offset), 0, len) }
    }

    /// Raw mutable pointer to the buffer, for building io_uring SQEs.
    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr.as_ptr()
    }
}

/// Shared layout computation: `len` must be a non-zero multiple of `ALIGN`.
fn checked_layout(len: usize) -> io::Result<Layout> {
    if len == 0 || len % AlignedBuf::ALIGN != 0 {
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
        // SAFETY: `ptr` is valid for `layout.size()` bytes for the lifetime of
        // `self`; for `zeroed` buffers the memory is initialized at allocation,
        // for `uninit` buffers callers uphold the module-level invariant that
        // the region is fully initialized (explicit writes or kernel DMA)
        // before any slice is materialized. No mutable alias can exist because
        // obtaining one requires `&mut self` (DerefMut).
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.layout.size()) }
    }
}

impl DerefMut for AlignedBuf {
    fn deref_mut(&mut self) -> &mut [u8] {
        // SAFETY: `ptr` is valid for `layout.size()` bytes (same initialization
        // invariant as `deref`), and the `&mut self` borrow guarantees
        // exclusive access.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.layout.size()) }
    }
}

impl Drop for AlignedBuf {
    fn drop(&mut self) {
        // SAFETY: `ptr` was allocated with exactly this `layout` in `zeroed` or
        // `uninit` and is deallocated exactly once, here.
        unsafe { dealloc(self.ptr.as_ptr(), self.layout) }
    }
}
