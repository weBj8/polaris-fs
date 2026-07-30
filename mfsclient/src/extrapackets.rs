pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
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
    unsafe fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn chunksdatacache_clear_inode(inode: uint32_t, chindx: uint32_t);
    unsafe fn chunksdatacache_change(
        inode: uint32_t,
        chindx: uint32_t,
        chunkid: uint64_t,
        version: uint32_t,
    );
    unsafe fn read_inode_clear_cache(inode: uint32_t, offset: uint64_t, leng: uint64_t);
    unsafe fn read_inode_set_length_passive(inode: uint32_t, newlength: uint64_t);
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
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
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const EXIT: C2Rust_Unnamed_0 = 2;
pub const FLENG_CHANGED: C2Rust_Unnamed_0 = 1;
pub const CHUNK_CHANGED: C2Rust_Unnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _extra_packets {
    pub cmd: uint32_t,
    pub inode: uint32_t,
    pub chindx: uint32_t,
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub fleng: uint64_t,
    pub offset: uint32_t,
    pub size: uint32_t,
    pub truncflag: uint8_t,
    pub next: *mut _extra_packets,
}
pub type extra_packets = _extra_packets;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_UNUSED_CNT: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
static mut ep_tail: *mut *mut extra_packets = ::core::ptr::null_mut::<*mut extra_packets>();
static mut ep_head: *mut extra_packets = ::core::ptr::null_mut::<extra_packets>();
static mut ep_unused: *mut extra_packets = ::core::ptr::null_mut::<extra_packets>();
static mut ep_unused_cnt: uint32_t = 0;
static mut ep_lock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __glibc_reserved: 0,
        __list: __pthread_list_t {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut ep_cond: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter { __value64: 0 },
        __g1_start: __atomic_wide_counter { __value64: 0 },
        __g_size: [0; 2],
        __g1_orig_size: 0,
        __wrefs: 0,
        __g_signals: [0; 2],
        __unused_initialized_1: 0,
        __unused_initialized_2: 0,
    },
};
static mut ep_worker: pthread_t = 0;
#[inline]
unsafe extern "C" fn ep_get_packet() -> *mut extra_packets {
    unsafe {
        let mut ep: *mut extra_packets = ::core::ptr::null_mut::<extra_packets>();
        if !ep_unused.is_null() {
            ep = ep_unused;
            ep_unused = (*ep_unused).next as *mut extra_packets;
            ep_unused_cnt = ep_unused_cnt.wrapping_sub(1);
        } else {
            ep = malloc(::core::mem::size_of::<extra_packets>()) as *mut extra_packets;
            if ep.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ep\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ep\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if ep
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut extra_packets
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ep\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ep\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        return ep;
    }
}
#[inline]
unsafe extern "C" fn ep_append_packet(mut ep: *mut extra_packets) {
    unsafe {
        let mut wakeup: uint8_t = 0;
        wakeup = (if ep_head.is_null() {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        (*ep).next = ::core::ptr::null_mut::<_extra_packets>();
        *ep_tail = ep;
        ep_tail = &raw mut (*ep).next as *mut *mut extra_packets;
        if wakeup != 0 {
            pthread_cond_signal(&raw mut ep_cond);
        }
    }
}
#[inline]
unsafe extern "C" fn ep_free_packet(mut ep: *mut extra_packets) {
    unsafe {
        if ep_unused_cnt >= MAX_UNUSED_CNT as uint32_t {
            free(ep as *mut ::core::ffi::c_void);
        } else {
            (*ep).next = ep_unused as *mut _extra_packets;
            ep_unused = ep;
            ep_unused_cnt = ep_unused_cnt.wrapping_add(1);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ep_thread(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut ep: *mut extra_packets = ::core::ptr::null_mut::<extra_packets>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut ep_lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        loop {
            while ep_head.is_null() {
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_cond_wait(&raw mut ep_cond, &raw mut ep_lock);
                if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&ep_cond,&ep_lock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&ep_cond,&ep_lock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                    } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_0);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&ep_cond,&ep_lock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&ep_cond,&ep_lock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                    } else {
                        let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_0);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&ep_cond,&ep_lock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&ep_cond,&ep_lock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                    }
                    abort();
                }
            }
            ep = ep_head;
            ep_head = (*ep).next as *mut extra_packets;
            if ep_head.is_null() {
                ep_tail = &raw mut ep_head;
            }
            let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut ep_lock);
            if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_3,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_3,
                    );
                } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_1);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_4,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_4,
                    );
                } else {
                    let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_1);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_err_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_err_1,
                    );
                }
                abort();
            }
            match (*ep).cmd {
                0 => {
                    chunksdatacache_change((*ep).inode, (*ep).chindx, (*ep).chunkid, (*ep).version);
                    if (*ep).truncflag != 0 {
                        chunksdatacache_clear_inode(
                            (*ep).inode,
                            (*ep).chindx.wrapping_add(1 as uint32_t),
                        );
                        read_inode_clear_cache(
                            (*ep).inode,
                            ((*ep).chindx as uint64_t)
                                .wrapping_mul(MFSCHUNKSIZE as uint64_t)
                                .wrapping_add((*ep).offset as uint64_t),
                            0 as uint64_t,
                        );
                        read_inode_set_length_passive((*ep).inode, (*ep).fleng);
                    } else if (*ep).size > 0 as uint32_t {
                        read_inode_clear_cache(
                            (*ep).inode,
                            ((*ep).chindx as uint64_t)
                                .wrapping_mul(MFSCHUNKSIZE as uint64_t)
                                .wrapping_add((*ep).offset as uint64_t),
                            (*ep).size as uint64_t,
                        );
                    }
                }
                1 => {
                    read_inode_set_length_passive((*ep).inode, (*ep).fleng);
                }
                _ => {
                    free(ep as *mut ::core::ffi::c_void);
                    return arg;
                }
            }
            let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_lock(&raw mut ep_lock);
            if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_5,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_5,
                    );
                } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_2);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_6,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_6,
                    );
                } else {
                    let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_2);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                }
                abort();
            }
            ep_free_packet(ep);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ep_chunk_has_changed(
    mut inode: uint32_t,
    mut chindx: uint32_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut fleng: uint64_t,
    mut truncflag: uint8_t,
    mut offset: uint32_t,
    mut size: uint32_t,
) {
    unsafe {
        let mut ep: *mut extra_packets = ::core::ptr::null_mut::<extra_packets>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut ep_lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        ep = ep_get_packet();
        (*ep).cmd = CHUNK_CHANGED as ::core::ffi::c_int as uint32_t;
        (*ep).inode = inode;
        (*ep).chindx = chindx;
        (*ep).chunkid = chunkid;
        (*ep).version = version;
        (*ep).fleng = fleng;
        (*ep).truncflag = truncflag;
        (*ep).offset = offset;
        (*ep).size = size;
        ep_append_packet(ep);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut ep_lock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ep_fleng_has_changed(mut inode: uint32_t, mut fleng: uint64_t) {
    unsafe {
        let mut ep: *mut extra_packets = ::core::ptr::null_mut::<extra_packets>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut ep_lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        ep = ep_get_packet();
        (*ep).cmd = FLENG_CHANGED as ::core::ffi::c_int as uint32_t;
        (*ep).inode = inode;
        (*ep).fleng = fleng;
        ep_append_packet(ep);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut ep_lock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ep_term() {
    unsafe {
        let mut ep: *mut extra_packets = ::core::ptr::null_mut::<extra_packets>();
        let mut epn: *mut extra_packets = ::core::ptr::null_mut::<extra_packets>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut ep_lock);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    178 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    178 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    178 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    178 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    178 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    178 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        ep = ep_get_packet();
        (*ep).cmd = EXIT as ::core::ffi::c_int as uint32_t;
        ep_append_packet(ep);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut ep_lock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        pthread_join(
            ep_worker,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        ep = ep_head;
        while !ep.is_null() {
            epn = (*ep).next as *mut extra_packets;
            free(ep as *mut ::core::ffi::c_void);
            ep = epn;
        }
        ep = ep_unused;
        while !ep.is_null() {
            epn = (*ep).next as *mut extra_packets;
            free(ep as *mut ::core::ffi::c_void);
            ep = epn;
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_cond_destroy(&raw mut ep_cond);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&ep_cond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&ep_cond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&ep_cond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&ep_cond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&ep_cond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&ep_cond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut ep_lock);
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
            } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
            } else {
                let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_2);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&ep_lock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ep_init() {
    unsafe {
        ep_head = ::core::ptr::null_mut::<extra_packets>();
        ep_tail = &raw mut ep_head;
        ep_unused = ::core::ptr::null_mut::<extra_packets>();
        ep_unused_cnt = 0 as uint32_t;
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_mutex_init(&raw mut ep_lock, ::core::ptr::null::<pthread_mutexattr_t>());
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&ep_lock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&ep_lock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&ep_lock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&ep_lock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&ep_lock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&ep_lock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_cond_init(&raw mut ep_cond, ::core::ptr::null::<pthread_condattr_t>());
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    202 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&ep_cond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    202 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&ep_cond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    202 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&ep_cond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    202 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&ep_cond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    202 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&ep_cond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/extrapackets.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    202 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&ep_cond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        lwt_minthread_create(
            &raw mut ep_worker,
            0 as uint8_t,
            Some(
                ep_thread
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            NULL,
        );
    }
}
