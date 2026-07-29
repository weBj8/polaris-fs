use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn fs_isopen(inode: uint32_t) -> ::core::ffi::c_int;
    fn mfs_dentry_invalidate(parent: uint32_t, nleng: uint8_t, name: *const ::core::ffi::c_char);
    fn monotonic_seconds() -> ::core::ffi::c_double;
    fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dinval_element {
    pub parent: uint32_t,
    pub nleng: uint8_t,
    pub name: *mut uint8_t,
    pub inode: uint32_t,
    pub timestamp: ::core::ffi::c_double,
    pub queue_next: *mut _dinval_element,
    pub queue_prev: *mut *mut _dinval_element,
    pub hash_next: *mut _dinval_element,
    pub hash_prev: *mut *mut _dinval_element,
}
pub type dinval_element = _dinval_element;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn portable_usleep(mut usec: uint64_t) {
    let mut req: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut rem: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut s: ::core::ffi::c_int = 0;
    req.tv_sec = usec.wrapping_div(1000000 as uint64_t) as __time_t;
    req.tv_nsec = usec
        .wrapping_rem(1000000 as uint64_t)
        .wrapping_mul(1000 as uint64_t) as __syscall_slong_t;
    loop {
        s = nanosleep(&raw mut req, &raw mut rem);
        if s < 0 as ::core::ffi::c_int {
            req = rem;
        }
        if s >= 0 as ::core::ffi::c_int {
            break;
        }
    }
}
pub const HASH_SIZE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const HASH_MASK: ::core::ffi::c_int = 0x7fff as ::core::ffi::c_int;
pub const MAX_ELEMENTS: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const MIN_TIMEOUT: ::core::ffi::c_double = 30.0f64;
static mut hashtab: *mut *mut dinval_element = ::core::ptr::null_mut::<*mut dinval_element>();
static mut elementcnt: uint32_t = 0;
static mut queue_prev: *mut *mut dinval_element = ::core::ptr::null_mut::<*mut dinval_element>();
static mut queue_head: *mut dinval_element = ::core::ptr::null_mut::<dinval_element>();
static mut main_timeout: ::core::ffi::c_double = 0.;
static mut glock: pthread_mutex_t = pthread_mutex_t {
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
#[inline]
unsafe extern "C" fn dinval_calc_hash(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
) -> uint32_t {
    let mut hash: uint32_t = 5381 as uint32_t;
    while nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        hash = (hash << 5 as ::core::ffi::c_int).wrapping_add(hash) ^ *name as uint32_t;
        name = name.offset(1);
        nleng = nleng.wrapping_sub(1);
    }
    hash ^= parent;
    return hash & HASH_MASK as uint32_t;
}
#[inline]
unsafe extern "C" fn dinval_calc_elem_hash(mut dielem: *mut dinval_element) -> uint32_t {
    return dinval_calc_hash((*dielem).parent, (*dielem).nleng, (*dielem).name);
}
#[inline]
unsafe extern "C" fn dinval_queue_detach(mut dielem: *mut dinval_element) {
    if !(*dielem).queue_next.is_null() {
        (*(*dielem).queue_next).queue_prev = (*dielem).queue_prev;
    } else {
        queue_prev = (*dielem).queue_prev as *mut *mut dinval_element;
    }
    *(*dielem).queue_prev = (*dielem).queue_next;
}
#[inline]
unsafe extern "C" fn dinval_element_detach(mut dielem: *mut dinval_element) {
    if !(*dielem).queue_next.is_null() {
        (*(*dielem).queue_next).queue_prev = (*dielem).queue_prev;
    } else {
        queue_prev = (*dielem).queue_prev as *mut *mut dinval_element;
    }
    *(*dielem).queue_prev = (*dielem).queue_next;
    if !(*dielem).hash_next.is_null() {
        (*(*dielem).hash_next).hash_prev = (*dielem).hash_prev;
    }
    *(*dielem).hash_prev = (*dielem).hash_next;
}
#[inline]
unsafe extern "C" fn dinval_queue_attach(mut dielem: *mut dinval_element) {
    (*dielem).queue_next = ::core::ptr::null_mut::<_dinval_element>();
    (*dielem).queue_prev = queue_prev as *mut *mut _dinval_element;
    *queue_prev = dielem;
    queue_prev = &raw mut (*dielem).queue_next as *mut *mut dinval_element;
    (*dielem).timestamp = monotonic_seconds();
}
#[inline]
unsafe extern "C" fn dinval_element_attach(
    mut hashhint: uint32_t,
    mut dielem: *mut dinval_element,
) {
    let mut hash: uint32_t = 0;
    if hashhint < HASH_SIZE as uint32_t {
        hash = hashhint;
    } else {
        hash = dinval_calc_elem_hash(dielem);
    }
    (*dielem).queue_next = ::core::ptr::null_mut::<_dinval_element>();
    (*dielem).queue_prev = queue_prev as *mut *mut _dinval_element;
    *queue_prev = dielem;
    queue_prev = &raw mut (*dielem).queue_next as *mut *mut dinval_element;
    (*dielem).hash_next = *hashtab.offset(hash as isize) as *mut _dinval_element;
    if !(*dielem).hash_next.is_null() {
        (*(*dielem).hash_next).hash_prev = &raw mut (*dielem).hash_next;
    }
    (*dielem).hash_prev = hashtab.offset(hash as isize) as *mut *mut _dinval_element;
    *hashtab.offset(hash as isize) = dielem;
    (*dielem).timestamp = monotonic_seconds();
}
#[inline]
unsafe extern "C" fn dinval_element_find(
    mut hashhint: *mut uint32_t,
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
) -> *mut dinval_element {
    let mut hash: uint32_t = 0;
    let mut dielem: *mut dinval_element = ::core::ptr::null_mut::<dinval_element>();
    hash = dinval_calc_hash(parent, nleng, name);
    dielem = *hashtab.offset(hash as isize);
    while !dielem.is_null() {
        if (*dielem).parent == parent
            && (*dielem).nleng as ::core::ffi::c_int == nleng as ::core::ffi::c_int
            && memcmp(
                (*dielem).name as *const ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                nleng as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            return dielem;
        }
        dielem = (*dielem).hash_next as *mut dinval_element;
    }
    if !hashhint.is_null() {
        *hashhint = hash;
    }
    return ::core::ptr::null_mut::<dinval_element>();
}
#[no_mangle]
pub unsafe extern "C" fn dinval_add(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut inode: uint32_t,
) {
    let mut dielem: *mut dinval_element = ::core::ptr::null_mut::<dinval_element>();
    let mut hashhint: uint32_t = 0;
    pthread_mutex_lock(&raw mut glock);
    dielem = dinval_element_find(&raw mut hashhint, parent, nleng, name);
    if !dielem.is_null() {
        dinval_queue_detach(dielem);
        (*dielem).inode = inode;
        dinval_queue_attach(dielem);
    } else {
        dielem = malloc(::core::mem::size_of::<dinval_element>()) as *mut dinval_element;
        if dielem.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dielem\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dielem\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if dielem
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut dinval_element
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dielem\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dielem\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*dielem).parent = parent;
        (*dielem).nleng = nleng;
        (*dielem).name = malloc((nleng as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            as *mut uint8_t;
        if (*dielem).name.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dielem->name\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dielem->name\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*dielem).name
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dielem->name\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dielem->name\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        memcpy(
            (*dielem).name as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        *(*dielem).name.offset(nleng as isize) = 0 as uint8_t;
        (*dielem).inode = inode;
        dinval_element_attach(hashhint, dielem);
        elementcnt = elementcnt.wrapping_add(1);
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[no_mangle]
pub unsafe extern "C" fn dinval_remove(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
) {
    let mut dielem: *mut dinval_element = ::core::ptr::null_mut::<dinval_element>();
    pthread_mutex_lock(&raw mut glock);
    dielem = dinval_element_find(::core::ptr::null_mut::<uint32_t>(), parent, nleng, name);
    if !dielem.is_null() {
        dinval_element_detach(dielem);
        free((*dielem).name as *mut ::core::ffi::c_void);
        free(dielem as *mut ::core::ffi::c_void);
        elementcnt = elementcnt.wrapping_sub(1);
    }
    pthread_mutex_unlock(&raw mut glock);
}
#[no_mangle]
pub unsafe extern "C" fn dinval_invalthread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut timeout: ::core::ffi::c_double = 0.;
    let mut now: ::core::ffi::c_double = 0.;
    let mut i: uint32_t = 0;
    let mut dielem: *mut dinval_element = ::core::ptr::null_mut::<dinval_element>();
    timeout = main_timeout;
    loop {
        now = monotonic_seconds();
        pthread_mutex_lock(&raw mut glock);
        i = 100 as uint32_t;
        while i > 0 as uint32_t
            && (elementcnt > MAX_ELEMENTS as uint32_t
                && (*queue_head).timestamp + MIN_TIMEOUT < now
                || !queue_head.is_null() && (*queue_head).timestamp + timeout < now)
        {
            dielem = queue_head;
            if fs_isopen((*dielem).inode) != 0 {
                dinval_queue_detach(dielem);
                dinval_queue_attach(dielem);
            } else {
                dinval_element_detach(dielem);
                pthread_mutex_unlock(&raw mut glock);
                mfs_dentry_invalidate(
                    (*dielem).parent,
                    (*dielem).nleng,
                    (*dielem).name as *const ::core::ffi::c_char,
                );
                pthread_mutex_lock(&raw mut glock);
                free((*dielem).name as *mut ::core::ffi::c_void);
                free(dielem as *mut ::core::ffi::c_void);
                elementcnt = elementcnt.wrapping_sub(1);
            }
            i = i.wrapping_sub(1);
        }
        pthread_mutex_unlock(&raw mut glock);
        portable_usleep(10000 as uint64_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dinval_init(mut timeout: ::core::ffi::c_double) {
    let mut i: uint32_t = 0;
    let mut th: pthread_t = 0;
    hashtab =
        malloc(::core::mem::size_of::<*mut dinval_element>().wrapping_mul(HASH_SIZE as size_t))
            as *mut *mut dinval_element;
    i = 0 as uint32_t;
    while i < HASH_SIZE as uint32_t {
        *hashtab.offset(i as isize) = ::core::ptr::null_mut::<dinval_element>();
        i = i.wrapping_add(1);
    }
    elementcnt = 0 as uint32_t;
    queue_head = ::core::ptr::null_mut::<dinval_element>();
    queue_prev = &raw mut queue_head;
    if timeout > MIN_TIMEOUT {
        main_timeout = timeout;
    } else {
        main_timeout = MIN_TIMEOUT;
    }
    let mut _mfs_assert_ret: ::core::ffi::c_int =
        pthread_mutex_init(&raw mut glock, ::core::ptr::null::<pthread_mutexattr_t>());
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
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dentry_invalidator.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    lwt_minthread_create(
        &raw mut th,
        1 as uint8_t,
        Some(
            dinval_invalthread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    );
}
