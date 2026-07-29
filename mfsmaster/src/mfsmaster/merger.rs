use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn shp_new(
        pointer: *mut ::core::ffi::c_void,
        freefn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> *mut ::core::ffi::c_void;
    fn shp_get(vs: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn shp_dec(vs: *mut ::core::ffi::c_void);
    fn restore_file(
        shfilename: *mut ::core::ffi::c_void,
        lv: uint64_t,
        ptr: *const ::core::ffi::c_char,
        verblevel: uint8_t,
    ) -> ::core::ffi::c_int;
    fn monotonic_useconds() -> uint64_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type int64_t = i64;
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hentry {
    pub fd: *mut FILE,
    pub shfilename: *mut ::core::ffi::c_void,
    pub buff: *mut ::core::ffi::c_char,
    pub bsize: size_t,
    pub ptr: *mut ::core::ffi::c_char,
    pub nextid: int64_t,
}
pub type hentry = _hentry;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
}
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BSIZE: ::core::ffi::c_int = 200000 as ::core::ffi::c_int;
static mut heap: *mut hentry = ::core::ptr::null_mut::<hentry>();
static mut heapsize: uint32_t = 0;
static mut maxidhole: int64_t = 0;
static mut lastlv: uint64_t = 0;
static mut firstlv: uint64_t = 0;
#[no_mangle]
pub unsafe extern "C" fn merger_heap_sort_down() {
    let mut l: uint32_t = 0;
    let mut r: uint32_t = 0;
    let mut m: uint32_t = 0;
    let mut pos: uint32_t = 0 as uint32_t;
    let mut x: hentry = hentry {
        fd: ::core::ptr::null_mut::<FILE>(),
        shfilename: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        buff: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bsize: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nextid: 0,
    };
    while pos < heapsize {
        l = pos.wrapping_mul(2 as uint32_t).wrapping_add(1 as uint32_t);
        r = l.wrapping_add(1 as uint32_t);
        if l >= heapsize {
            return;
        }
        m = l;
        if r < heapsize && (*heap.offset(r as isize)).nextid < (*heap.offset(l as isize)).nextid {
            m = r;
        }
        if (*heap.offset(pos as isize)).nextid <= (*heap.offset(m as isize)).nextid {
            return;
        }
        x = *heap.offset(pos as isize);
        *heap.offset(pos as isize) = *heap.offset(m as isize);
        *heap.offset(m as isize) = x;
        pos = m;
    }
}
#[no_mangle]
pub unsafe extern "C" fn merger_heap_sort_up() {
    let mut pos: uint32_t = heapsize.wrapping_sub(1 as uint32_t);
    let mut p: uint32_t = 0;
    let mut x: hentry = hentry {
        fd: ::core::ptr::null_mut::<FILE>(),
        shfilename: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        buff: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bsize: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nextid: 0,
    };
    while pos > 0 as uint32_t {
        p = pos.wrapping_sub(1 as uint32_t).wrapping_div(2 as uint32_t);
        if (*heap.offset(pos as isize)).nextid >= (*heap.offset(p as isize)).nextid {
            return;
        }
        x = *heap.offset(pos as isize);
        *heap.offset(pos as isize) = *heap.offset(p as isize);
        *heap.offset(p as isize) = x;
        pos = p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn merger_nextentry(mut pos: uint32_t) {
    if getline(
        &raw mut (*heap.offset(pos as isize)).buff,
        &raw mut (*heap.offset(pos as isize)).bsize,
        (*heap.offset(pos as isize)).fd,
    ) != -1 as __ssize_t
    {
        let mut nextid: int64_t = strtoll(
            (*heap.offset(pos as isize)).buff,
            &raw mut (*heap.offset(pos as isize)).ptr,
            10 as ::core::ffi::c_int,
        ) as int64_t;
        if *(*heap.offset(pos as isize)).ptr.offset(0 as isize) as ::core::ffi::c_int
            == ':' as ::core::ffi::c_int
            && *(*heap.offset(pos as isize)).ptr.offset(1 as isize) as ::core::ffi::c_int
                == ' ' as ::core::ffi::c_int
        {
            (*heap.offset(pos as isize)).ptr = (*heap.offset(pos as isize))
                .ptr
                .offset(2 as ::core::ffi::c_int as isize);
        }
        if (*heap.offset(pos as isize)).nextid < 0 as int64_t
            || nextid > (*heap.offset(pos as isize)).nextid
                && nextid < (*heap.offset(pos as isize)).nextid + maxidhole
        {
            (*heap.offset(pos as isize)).nextid = nextid;
        } else {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"found garbage at the end of file: %s (last correct id: %lu)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                shp_get((*heap.offset(pos as isize)).shfilename) as *mut ::core::ffi::c_char,
                (*heap.offset(pos as isize)).nextid,
            );
            (*heap.offset(pos as isize)).nextid = -1 as ::core::ffi::c_long as int64_t;
        }
    } else {
        (*heap.offset(pos as isize)).nextid = -1 as ::core::ffi::c_long as int64_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn merger_delete_entry() {
    if !(*heap.offset(heapsize as isize)).fd.is_null() {
        fclose((*heap.offset(heapsize as isize)).fd);
    }
    if !(*heap.offset(heapsize as isize)).shfilename.is_null() {
        shp_dec((*heap.offset(heapsize as isize)).shfilename);
    }
    if !(*heap.offset(heapsize as isize)).buff.is_null() {
        free((*heap.offset(heapsize as isize)).buff as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn merger_new_entry(mut filename: *const ::core::ffi::c_char) {
    (*heap.offset(heapsize as isize)).fd =
        fopen(filename, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if !(*heap.offset(heapsize as isize)).fd.is_null() {
        (*heap.offset(heapsize as isize)).shfilename = shp_new(
            strdup(filename) as *mut ::core::ffi::c_void,
            Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        (*heap.offset(heapsize as isize)).buff =
            malloc(BSIZE as size_t) as *mut ::core::ffi::c_char;
        (*heap.offset(heapsize as isize)).bsize = BSIZE as size_t;
        (*heap.offset(heapsize as isize)).ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*heap.offset(heapsize as isize)).nextid = -1 as ::core::ffi::c_long as int64_t;
        merger_nextentry(heapsize);
    } else {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"can't open changelog file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
        );
        (*heap.offset(heapsize as isize)).shfilename = NULL;
        (*heap.offset(heapsize as isize)).buff = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*heap.offset(heapsize as isize)).bsize = 0 as size_t;
        (*heap.offset(heapsize as isize)).ptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*heap.offset(heapsize as isize)).nextid = -1 as ::core::ffi::c_long as int64_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn merger_start(
    mut files: uint32_t,
    mut filenames: *mut *mut ::core::ffi::c_char,
    mut maxhole: uint64_t,
    mut minid: uint64_t,
    mut maxid: uint64_t,
) -> ::core::ffi::c_int {
    let mut i: uint32_t = 0;
    heapsize = 0 as uint32_t;
    heap = malloc(::core::mem::size_of::<hentry>().wrapping_mul(files as size_t)) as *mut hentry;
    if heap.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    i = 0 as uint32_t;
    while i < files {
        merger_new_entry(*filenames.offset(i as isize));
        if (*heap.offset(heapsize as isize)).nextid < 0 as int64_t {
            merger_delete_entry();
        } else {
            heapsize = heapsize.wrapping_add(1);
            merger_heap_sort_up();
        }
        i = i.wrapping_add(1);
    }
    maxidhole = maxhole as int64_t;
    firstlv = minid;
    lastlv = maxid;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn merger_loop(mut verblevel: uint8_t) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut perc: uint8_t = 0;
    let mut etaok: uint8_t = 0;
    let mut eta: uint32_t = 0;
    let mut st: uint64_t = 0;
    let mut cu: uint64_t = 0;
    let mut h: hentry = hentry {
        fd: ::core::ptr::null_mut::<FILE>(),
        shfilename: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        buff: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bsize: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        nextid: 0,
    };
    st = monotonic_useconds();
    while heapsize != 0 {
        if (*heap.offset(0 as isize)).nextid % 2497 as int64_t == 0 as int64_t && lastlv > firstlv {
            if (*heap.offset(0 as isize)).nextid < firstlv as int64_t {
                perc = 0 as uint8_t;
                eta = 0 as uint32_t;
                etaok = 0 as uint8_t;
                st = monotonic_useconds();
            } else if (*heap.offset(0 as isize)).nextid > lastlv as int64_t {
                perc = 100 as uint8_t;
                eta = 0 as uint32_t;
                etaok = 1 as uint8_t;
            } else {
                cu = monotonic_useconds();
                perc = ((*heap.offset(0 as isize)).nextid as uint64_t)
                    .wrapping_sub(firstlv)
                    .wrapping_mul(100 as uint64_t)
                    .wrapping_div(lastlv.wrapping_sub(firstlv)) as uint8_t;
                eta = lastlv
                    .wrapping_sub((*heap.offset(0 as isize)).nextid as uint64_t)
                    .wrapping_mul(cu.wrapping_sub(st))
                    .wrapping_div(
                        ((*heap.offset(0 as isize)).nextid as uint64_t).wrapping_sub(firstlv),
                    )
                    .wrapping_div(1000000 as uint64_t) as uint32_t;
                etaok = 1 as uint8_t;
            }
            printf(
                b"progress: current change: %lu (first:%lu - last:%lu - %hhu%%\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*heap.offset(0 as isize)).nextid,
                firstlv,
                lastlv,
                perc as ::core::ffi::c_int,
            );
            if etaok != 0 {
                printf(
                    b" - ETA:%02u:%02us)\r\0".as_ptr() as *const ::core::ffi::c_char,
                    eta.wrapping_div(60 as uint32_t) as ::core::ffi::c_uint,
                    eta.wrapping_rem(60 as uint32_t) as ::core::ffi::c_uint,
                );
            } else {
                printf(b" - ETA:__:__s)\r\0".as_ptr() as *const ::core::ffi::c_char);
            }
            fflush(stdout);
        }
        status = restore_file(
            (*heap.offset(0 as isize)).shfilename,
            (*heap.offset(0 as isize)).nextid as uint64_t,
            (*heap.offset(0 as isize)).ptr,
            verblevel,
        );
        if status < 0 as ::core::ffi::c_int {
            while heapsize != 0 {
                heapsize = heapsize.wrapping_sub(1);
                merger_delete_entry();
            }
            printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            return status;
        }
        merger_nextentry(0 as uint32_t);
        if (*heap.offset(0 as isize)).nextid < 0 as int64_t {
            heapsize = heapsize.wrapping_sub(1);
            h = *heap.offset(0 as isize);
            *heap.offset(0 as isize) = *heap.offset(heapsize as isize);
            *heap.offset(heapsize as isize) = h;
            merger_delete_entry();
        }
        merger_heap_sort_down();
    }
    printf(
        b"progress: current change: %lu (first:%lu - last:%lu - 100%% - ETA:finished)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        lastlv,
        firstlv,
        lastlv,
    );
    return 0 as ::core::ffi::c_int;
}
