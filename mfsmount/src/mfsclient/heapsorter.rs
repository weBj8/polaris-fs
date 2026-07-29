pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint32_t = u32;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mfsrealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pptr: *mut ::core::ffi::c_void = realloc(ptr, size);
        if pptr.is_null() {
            free(ptr);
        }
        return pptr;
    }
}
static mut heap: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
static mut heapsize: uint32_t = 0 as uint32_t;
static mut heapelements: uint32_t = 0 as uint32_t;
#[inline]
unsafe extern "C" fn heap_sort_down() {
    unsafe {
        let mut l: uint32_t = 0;
        let mut r: uint32_t = 0;
        let mut m: uint32_t = 0;
        let mut pos: uint32_t = 0 as uint32_t;
        let mut x: uint32_t = 0;
        while pos < heapelements {
            l = pos.wrapping_mul(2 as uint32_t).wrapping_add(1 as uint32_t);
            r = l.wrapping_add(1 as uint32_t);
            if l >= heapelements {
                return;
            }
            m = l;
            if r < heapelements && *heap.offset(r as isize) < *heap.offset(l as isize) {
                m = r;
            }
            if *heap.offset(pos as isize) <= *heap.offset(m as isize) {
                return;
            }
            x = *heap.offset(pos as isize);
            *heap.offset(pos as isize) = *heap.offset(m as isize);
            *heap.offset(m as isize) = x;
            pos = m;
        }
    }
}
#[inline]
unsafe extern "C" fn heap_sort_up() {
    unsafe {
        let mut pos: uint32_t = heapelements.wrapping_sub(1 as uint32_t);
        let mut p: uint32_t = 0;
        let mut x: uint32_t = 0;
        while pos > 0 as uint32_t {
            p = pos.wrapping_sub(1 as uint32_t).wrapping_div(2 as uint32_t);
            if *heap.offset(pos as isize) >= *heap.offset(p as isize) {
                return;
            }
            x = *heap.offset(pos as isize);
            *heap.offset(pos as isize) = *heap.offset(p as isize);
            *heap.offset(p as isize) = x;
            pos = p;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn heap_cleanup() {
    unsafe {
        heapelements = 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn heap_push(mut element: uint32_t) {
    unsafe {
        if heapelements >= heapsize {
            if heap.is_null() {
                heapsize = 1024 as uint32_t;
                heap = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(heapsize as size_t))
                    as *mut uint32_t;
            } else {
                heapsize <<= 1 as ::core::ffi::c_int;
                heap = mfsrealloc(
                    heap as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<uint32_t>().wrapping_mul(heapsize as size_t),
                ) as *mut uint32_t;
            }
            if heap.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/heapsorter.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/heapsorter.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if heap
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/heapsorter.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/heapsorter.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        *heap.offset(heapelements as isize) = element;
        heapelements = heapelements.wrapping_add(1);
        heap_sort_up();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn heap_pop() -> uint32_t {
    unsafe {
        let mut element: uint32_t = 0;
        if heapelements == 0 as uint32_t {
            return 0 as uint32_t;
        }
        element = *heap.offset(0 as isize);
        heapelements = heapelements.wrapping_sub(1);
        if heapelements > 0 as uint32_t {
            *heap.offset(0 as isize) = *heap.offset(heapelements as isize);
            heap_sort_down();
        }
        return element;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn heap_elements() -> uint32_t {
    unsafe {
        return heapelements;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn heap_term() {
    unsafe {
        if !heap.is_null() {
            free(heap as *mut ::core::ffi::c_void);
        }
        heap = ::core::ptr::null_mut::<uint32_t>();
        heapsize = 0 as uint32_t;
        heapelements = 0 as uint32_t;
    }
}
