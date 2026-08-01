pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum _bio {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn matoclserv_fuse_flock_wake_up(
        veptr: *mut ::core::ffi::c_void,
        msgid: uint32_t,
        status: uint8_t,
    );
    unsafe fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn of_checknode(sessionid: uint32_t, inode: uint32_t) -> uint8_t;
    unsafe fn meta_version_inc() -> uint64_t;
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_info_register_fname(
        fun: Option<unsafe extern "C" fn(*mut FILE) -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_time() -> uint32_t;
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
    unsafe fn changelog(format: *const ::core::ffi::c_char, ...);
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _instance {
    pub msgid: uint32_t,
    pub reqid: uint32_t,
    pub next: *mut _instance,
}
pub type instance = _instance;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lock {
    pub owner: uint64_t,
    pub connptr: *mut ::core::ffi::c_void,
    pub sessionid: uint32_t,
    pub state: uint8_t,
    pub ltype: uint8_t,
    pub lock_instances: *mut instance,
    pub parent: *mut _inodelocks,
    pub next: *mut _lock,
    pub prev: *mut *mut _lock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _inodelocks {
    pub inode: uint32_t,
    pub active: *mut lock,
    pub waiting_head: *mut lock,
    pub waiting_tail: *mut *mut lock,
    pub next: *mut _inodelocks,
}
pub type lock = _lock;
pub type inodelocks = _inodelocks;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTOPENED: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_ERROR_WAITING: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const MFS_ERROR_EAGAIN: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const MFS_ERROR_EINTR: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const MFS_ERROR_ECANCELED: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FLOCK_UNLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLOCK_TRY_SHARED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLOCK_LOCK_SHARED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FLOCK_TRY_EXCLUSIVE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FLOCK_LOCK_EXCLUSIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FLOCK_INTERRUPT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FLOCK_RELEASE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_LOCK_TYPE_UNKNOWN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_LOCK_TYPE_SHARED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_LOCK_TYPE_EXCLUSIVE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put64bit(mut ptr: *mut *mut uint8_t, mut val: uint64_t) {
    unsafe {
        val = val.swap_bytes() as uint64_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            8 as size_t,
        );
        *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    unsafe {
        val = val.swap_bytes() as uint32_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put8bit(mut ptr: *mut *mut uint8_t, mut val: uint8_t) {
    unsafe {
        *(*ptr).offset(0 as isize) =
            (val as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *ptr = (*ptr).offset(1);
    }
}
#[inline]
unsafe extern "C" fn get64bit(mut ptr: *mut *const uint8_t) -> uint64_t {
    unsafe {
        let mut t64: uint64_t = 0;
        memcpy(
            &raw mut t64 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            8 as size_t,
        );
        *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
        return t64.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    unsafe {
        let mut t32: uint32_t = 0;
        memcpy(
            &raw mut t32 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
        return t32.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get8bit(mut ptr: *mut *const uint8_t) -> uint8_t {
    unsafe {
        let mut t8: uint8_t = 0;
        t8 = *(*ptr).offset(0 as isize);
        *ptr = (*ptr).offset(1);
        return t8;
    }
}
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MODE_CORRECT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MODE_LINUX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STATE_WAITING: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STATE_ACTIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LTYPE_READER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LTYPE_WRITER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLOCK_INODE_HASHSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut inodehash: *mut *mut inodelocks = ::core::ptr::null_mut::<*mut inodelocks>();
static mut FlocksMode: uint8_t = 0;
static mut DebugInfo: uint8_t = 0;
#[inline]
unsafe extern "C" fn flock_inode_find(mut inode: uint32_t) -> *mut inodelocks {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        il = *inodehash.offset(
            inode
                .wrapping_mul(0x738a2379 as uint32_t)
                .wrapping_rem(FLOCK_INODE_HASHSIZE as uint32_t) as isize,
        );
        while !il.is_null() {
            if (*il).inode == inode {
                return il;
            }
            il = (*il).next as *mut inodelocks;
        }
        return ::core::ptr::null_mut::<inodelocks>();
    }
}
#[inline]
unsafe extern "C" fn flock_inode_new(mut inode: uint32_t) -> *mut inodelocks {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut hash: uint32_t = 0;
        il = malloc(::core::mem::size_of::<inodelocks>()) as *mut inodelocks;
        if il.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if il
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut inodelocks
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*il).inode = inode;
        (*il).active = ::core::ptr::null_mut::<lock>();
        (*il).waiting_head = ::core::ptr::null_mut::<lock>();
        (*il).waiting_tail = &raw mut (*il).waiting_head;
        hash = inode
            .wrapping_mul(0x738a2379 as uint32_t)
            .wrapping_rem(FLOCK_INODE_HASHSIZE as uint32_t);
        (*il).next = *inodehash.offset(hash as isize) as *mut _inodelocks;
        *inodehash.offset(hash as isize) = il;
        return il;
    }
}
#[inline]
unsafe extern "C" fn flock_inode_remove(mut inode: uint32_t) {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut ilp: *mut *mut inodelocks = ::core::ptr::null_mut::<*mut inodelocks>();
        let mut hash: uint32_t = 0;
        hash = inode
            .wrapping_mul(0x738a2379 as uint32_t)
            .wrapping_rem(FLOCK_INODE_HASHSIZE as uint32_t);
        ilp = inodehash.offset(hash as isize);
        loop {
            il = *ilp;
            if il.is_null() {
                break;
            }
            if (*il).inode == inode {
                if (*il).active.is_null() && (*il).waiting_head.is_null() {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        166 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"il->active==NULL && il->waiting_head==NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"inode flock record not empty !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        166 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"il->active==NULL && il->waiting_head==NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"inode flock record not empty !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                *ilp = (*il).next as *mut inodelocks;
                free(il as *mut ::core::ffi::c_void);
            } else {
                ilp = &raw mut (*il).next as *mut *mut inodelocks;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn flock_lock_inode_detach(mut l: *mut lock) {
    unsafe {
        if !(*l).next.is_null() {
            (*(*l).next).prev = (*l).prev;
        } else if (*l).state as ::core::ffi::c_int == STATE_WAITING {
            (*(*l).parent).waiting_tail = (*l).prev as *mut *mut lock;
        }
        *(*l).prev = (*l).next;
    }
}
#[inline]
unsafe extern "C" fn flock_do_lock_inode_attach(mut l: *mut lock) {
    unsafe {
        if (*l).state as ::core::ffi::c_int == STATE_WAITING {
            (*l).next = ::core::ptr::null_mut::<_lock>();
            (*l).prev = (*(*l).parent).waiting_tail as *mut *mut _lock;
            *(*(*l).parent).waiting_tail = l;
            (*(*l).parent).waiting_tail = &raw mut (*l).next as *mut *mut lock;
        } else {
            (*l).next = (*(*l).parent).active as *mut _lock;
            if !(*l).next.is_null() {
                (*(*l).next).prev = &raw mut (*l).next;
            }
            (*l).prev = &raw mut (*(*l).parent).active as *mut *mut _lock;
            (*(*l).parent).active = l;
        };
    }
}
#[inline]
unsafe extern "C" fn flock_lock_inode_attach(mut l: *mut lock) {
    unsafe {
        if (*l).state as ::core::ffi::c_int == STATE_ACTIVE {
            changelog(
                b"%u|FLOCK(%u,%u,%lu,%c)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*(*l).parent).inode,
                (*l).sessionid,
                (*l).owner,
                if (*l).ltype as ::core::ffi::c_int == LTYPE_READER {
                    'R' as ::core::ffi::c_int
                } else {
                    'W' as ::core::ffi::c_int
                },
            );
        }
        flock_do_lock_inode_attach(l);
    }
}
#[inline]
unsafe extern "C" fn flock_lock_wake_up_one(
    mut l: *mut lock,
    mut reqid: uint32_t,
    mut status: uint8_t,
) {
    unsafe {
        let mut i: *mut instance = ::core::ptr::null_mut::<instance>();
        let mut iptr: *mut *mut instance = ::core::ptr::null_mut::<*mut instance>();
        iptr = &raw mut (*l).lock_instances;
        loop {
            i = *iptr;
            if i.is_null() {
                break;
            }
            if (*i).reqid == reqid {
                matoclserv_fuse_flock_wake_up((*l).connptr, (*i).msgid, status);
                *iptr = (*i).next as *mut instance;
                free(i as *mut ::core::ffi::c_void);
            } else {
                iptr = &raw mut (*i).next as *mut *mut instance;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn flock_lock_wake_up_all(mut l: *mut lock, mut status: uint8_t) {
    unsafe {
        let mut i: *mut instance = ::core::ptr::null_mut::<instance>();
        let mut ni: *mut instance = ::core::ptr::null_mut::<instance>();
        i = (*l).lock_instances;
        while !i.is_null() {
            ni = (*i).next as *mut instance;
            matoclserv_fuse_flock_wake_up((*l).connptr, (*i).msgid, status);
            free(i as *mut ::core::ffi::c_void);
            i = ni;
        }
        (*l).lock_instances = ::core::ptr::null_mut::<instance>();
    }
}
#[inline]
unsafe extern "C" fn flock_lock_append_req(
    mut l: *mut lock,
    mut msgid: uint32_t,
    mut reqid: uint32_t,
) {
    unsafe {
        let mut i: *mut instance = ::core::ptr::null_mut::<instance>();
        i = (*l).lock_instances;
        while !i.is_null() {
            if (*i).reqid == reqid {
                (*i).msgid = msgid;
                return;
            }
            i = (*i).next as *mut instance;
        }
        i = malloc(::core::mem::size_of::<instance>()) as *mut instance;
        if i.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"i\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"i\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if i
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut instance
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"i\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"i\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*i).msgid = msgid;
        (*i).reqid = reqid;
        (*i).next = (*l).lock_instances as *mut _instance;
        (*l).lock_instances = i;
    }
}
#[inline]
unsafe extern "C" fn flock_do_lock_remove(mut l: *mut lock) {
    unsafe {
        let mut i: *mut instance = ::core::ptr::null_mut::<instance>();
        let mut ni: *mut instance = ::core::ptr::null_mut::<instance>();
        i = (*l).lock_instances;
        while !i.is_null() {
            ni = (*i).next as *mut instance;
            free(i as *mut ::core::ffi::c_void);
            i = ni;
        }
        if !(*l).next.is_null() {
            (*(*l).next).prev = (*l).prev;
        } else if (*l).state as ::core::ffi::c_int == STATE_WAITING {
            (*(*l).parent).waiting_tail = (*l).prev as *mut *mut lock;
        }
        *(*l).prev = (*l).next;
        free(l as *mut ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C" fn flock_lock_remove(mut l: *mut lock) {
    unsafe {
        if (*l).state as ::core::ffi::c_int == STATE_ACTIVE {
            changelog(
                b"%u|FLOCK(%u,%u,%lu,U)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*(*l).parent).inode,
                (*l).sessionid,
                (*l).owner,
            );
        }
        flock_do_lock_remove(l);
    }
}
#[inline]
unsafe extern "C" fn flock_check(mut il: *mut inodelocks, mut ltype: uint8_t) -> uint8_t {
    unsafe {
        if ltype as ::core::ffi::c_int == LTYPE_READER {
            if !(*il).active.is_null()
                && (*(*il).active).ltype as ::core::ffi::c_int == LTYPE_WRITER
            {
                return 1 as uint8_t;
            }
            if FlocksMode as ::core::ffi::c_int == MODE_CORRECT {
                if !(*il).waiting_head.is_null() {
                    return 1 as uint8_t;
                }
            }
        } else if !(*il).active.is_null() {
            return 1 as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn flock_lock_new(
    mut il: *mut inodelocks,
    mut ltype: uint8_t,
    mut connptr: *mut ::core::ffi::c_void,
    mut sessionid: uint32_t,
    mut msgid: uint32_t,
    mut reqid: uint32_t,
    mut owner: uint64_t,
) -> uint8_t {
    unsafe {
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        l = malloc(::core::mem::size_of::<lock>()) as *mut lock;
        (*l).owner = owner;
        (*l).connptr = connptr;
        (*l).sessionid = sessionid;
        (*l).state = STATE_ACTIVE as uint8_t;
        (*l).ltype = ltype;
        (*l).lock_instances = ::core::ptr::null_mut::<instance>();
        (*l).parent = il as *mut _inodelocks;
        (*l).next = ::core::ptr::null_mut::<_lock>();
        (*l).prev = ::core::ptr::null_mut::<*mut _lock>();
        if flock_check(il, ltype) != 0 {
            (*l).state = STATE_WAITING as uint8_t;
            flock_lock_append_req(l, msgid, reqid);
            flock_lock_inode_attach(l);
            return MFS_ERROR_WAITING as uint8_t;
        }
        flock_lock_inode_attach(l);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn flock_lock_check_waiting(mut il: *mut inodelocks) {
    unsafe {
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut nl: *mut lock = ::core::ptr::null_mut::<lock>();
        l = (*il).waiting_head;
        if l.is_null() {
            return;
        }
        if (*il).active.is_null() && (*l).ltype as ::core::ffi::c_int == LTYPE_WRITER {
            flock_lock_inode_detach(l);
            (*l).state = STATE_ACTIVE as uint8_t;
            flock_lock_inode_attach(l);
            flock_lock_wake_up_all(l, MFS_STATUS_OK as uint8_t);
        }
        if (*il).active.is_null() || (*(*il).active).ltype as ::core::ffi::c_int == LTYPE_READER {
            if FlocksMode as ::core::ffi::c_int == MODE_LINUX {
                while !l.is_null() {
                    nl = (*l).next as *mut lock;
                    if (*l).ltype as ::core::ffi::c_int == LTYPE_READER {
                        flock_lock_inode_detach(l);
                        (*l).state = STATE_ACTIVE as uint8_t;
                        flock_lock_inode_attach(l);
                        flock_lock_wake_up_all(l, MFS_STATUS_OK as uint8_t);
                    }
                    l = nl;
                }
            } else {
                while !l.is_null() && (*l).ltype as ::core::ffi::c_int == LTYPE_READER {
                    nl = (*l).next as *mut lock;
                    flock_lock_inode_detach(l);
                    (*l).state = STATE_ACTIVE as uint8_t;
                    flock_lock_inode_attach(l);
                    flock_lock_wake_up_all(l, MFS_STATUS_OK as uint8_t);
                    l = nl;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn flock_lock_unlock(mut il: *mut inodelocks, mut l: *mut lock) {
    unsafe {
        if il == (*l).parent {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                359 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il==l->parent\0".as_ptr() as *const ::core::ffi::c_char,
                b"flock data structures mismatch\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/flocklocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                359 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il==l->parent\0".as_ptr() as *const ::core::ffi::c_char,
                b"flock data structures mismatch\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        flock_lock_remove(l);
        if (*il).active.is_null() && !(*il).waiting_head.is_null() {
            flock_lock_check_waiting(il);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_disconnected(mut connptr: *mut ::core::ffi::c_void) {
    unsafe {
        let mut h: uint32_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut nl: *mut lock = ::core::ptr::null_mut::<lock>();
        h = 0 as uint32_t;
        while h < FLOCK_INODE_HASHSIZE as uint32_t {
            il = *inodehash.offset(h as isize);
            while !il.is_null() {
                l = (*il).waiting_head;
                while !l.is_null() {
                    nl = (*l).next as *mut lock;
                    if (*l).connptr == connptr {
                        flock_do_lock_remove(l);
                    }
                    l = nl;
                }
                il = (*il).next as *mut inodelocks;
            }
            h = h.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_locks_cmd(
    mut connptr: *mut ::core::ffi::c_void,
    mut sessionid: uint32_t,
    mut msgid: uint32_t,
    mut reqid: uint32_t,
    mut inode: uint32_t,
    mut owner: uint64_t,
    mut op: uint8_t,
) -> uint8_t {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut nl: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut ltype: uint8_t = 0;
        if op as ::core::ffi::c_int != FLOCK_INTERRUPT && op as ::core::ffi::c_int != FLOCK_RELEASE
        {
            if of_checknode(sessionid, inode) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return MFS_ERROR_NOTOPENED as uint8_t;
            }
        }
        il = flock_inode_find(inode);
        if il.is_null() {
            if op as ::core::ffi::c_int == FLOCK_UNLOCK
                || op as ::core::ffi::c_int == FLOCK_INTERRUPT
                || op as ::core::ffi::c_int == FLOCK_RELEASE
            {
                return MFS_STATUS_OK as uint8_t;
            }
            il = flock_inode_new(inode);
        }
        if op as ::core::ffi::c_int == FLOCK_INTERRUPT {
            l = (*il).waiting_head;
            while !l.is_null() {
                nl = (*l).next as *mut lock;
                if (*l).connptr == connptr && (*l).sessionid == sessionid && (*l).owner == owner {
                    flock_lock_wake_up_one(l, reqid, MFS_ERROR_EINTR as uint8_t);
                    if (*l).lock_instances.is_null() {
                        flock_lock_remove(l);
                    }
                }
                l = nl;
            }
            return MFS_STATUS_OK as uint8_t;
        }
        l = (*il).active;
        while !l.is_null() {
            if (*l).sessionid == sessionid && (*l).owner == owner {
                if op as ::core::ffi::c_int == FLOCK_UNLOCK
                    || op as ::core::ffi::c_int == FLOCK_RELEASE
                {
                    flock_lock_unlock(il, l);
                    if (*il).waiting_head.is_null() && (*il).active.is_null() {
                        flock_inode_remove((*il).inode);
                    }
                    return MFS_STATUS_OK as uint8_t;
                } else if op as ::core::ffi::c_int == FLOCK_TRY_SHARED {
                    if (*l).ltype as ::core::ffi::c_int == LTYPE_READER {
                        return MFS_STATUS_OK as uint8_t;
                    } else {
                        (*l).ltype = LTYPE_READER as uint8_t;
                        flock_lock_check_waiting(il);
                        return MFS_STATUS_OK as uint8_t;
                    }
                } else if op as ::core::ffi::c_int == FLOCK_LOCK_SHARED {
                    if (*l).ltype as ::core::ffi::c_int == LTYPE_READER {
                        return MFS_STATUS_OK as uint8_t;
                    } else {
                        flock_lock_unlock(il, l);
                        return flock_lock_new(
                            il,
                            LTYPE_READER as uint8_t,
                            connptr,
                            sessionid,
                            msgid,
                            reqid,
                            owner,
                        );
                    }
                } else if op as ::core::ffi::c_int == FLOCK_TRY_EXCLUSIVE {
                    if (*l).ltype as ::core::ffi::c_int == LTYPE_WRITER {
                        return MFS_STATUS_OK as uint8_t;
                    } else {
                        if (*(*il).active).next.is_null() {
                            (*l).ltype = LTYPE_WRITER as uint8_t;
                            return MFS_STATUS_OK as uint8_t;
                        }
                        return MFS_ERROR_EAGAIN as uint8_t;
                    }
                } else if op as ::core::ffi::c_int == FLOCK_LOCK_EXCLUSIVE {
                    if (*l).ltype as ::core::ffi::c_int == LTYPE_WRITER {
                        return MFS_STATUS_OK as uint8_t;
                    } else {
                        flock_lock_unlock(il, l);
                        return flock_lock_new(
                            il,
                            LTYPE_WRITER as uint8_t,
                            connptr,
                            sessionid,
                            msgid,
                            reqid,
                            owner,
                        );
                    }
                }
                return MFS_ERROR_EINVAL as uint8_t;
            }
            l = (*l).next as *mut lock;
        }
        l = (*il).waiting_head;
        while !l.is_null() {
            if (*l).connptr == connptr && (*l).sessionid == sessionid && (*l).owner == owner {
                if op as ::core::ffi::c_int == FLOCK_RELEASE {
                    flock_lock_wake_up_all(l, MFS_ERROR_ECANCELED as uint8_t);
                    flock_lock_remove(l);
                    return MFS_STATUS_OK as uint8_t;
                } else if op as ::core::ffi::c_int == FLOCK_UNLOCK {
                    if FlocksMode as ::core::ffi::c_int == MODE_CORRECT {
                        flock_lock_wake_up_all(l, MFS_ERROR_ECANCELED as uint8_t);
                        flock_lock_remove(l);
                    }
                    return MFS_STATUS_OK as uint8_t;
                } else if op as ::core::ffi::c_int == FLOCK_TRY_SHARED
                    || op as ::core::ffi::c_int == FLOCK_TRY_EXCLUSIVE
                {
                    return MFS_ERROR_EAGAIN as uint8_t;
                } else if op as ::core::ffi::c_int == FLOCK_LOCK_SHARED {
                    if (*l).ltype as ::core::ffi::c_int == LTYPE_READER {
                        flock_lock_append_req(l, msgid, reqid);
                        return MFS_ERROR_WAITING as uint8_t;
                    } else {
                        flock_lock_wake_up_all(l, MFS_ERROR_ECANCELED as uint8_t);
                        (*l).ltype = LTYPE_READER as uint8_t;
                        flock_lock_append_req(l, msgid, reqid);
                        return MFS_ERROR_WAITING as uint8_t;
                    }
                } else if op as ::core::ffi::c_int == FLOCK_LOCK_EXCLUSIVE {
                    if (*l).ltype as ::core::ffi::c_int == LTYPE_WRITER {
                        flock_lock_append_req(l, msgid, reqid);
                        return MFS_ERROR_WAITING as uint8_t;
                    } else {
                        flock_lock_wake_up_all(l, MFS_ERROR_ECANCELED as uint8_t);
                        (*l).ltype = LTYPE_WRITER as uint8_t;
                        flock_lock_append_req(l, msgid, reqid);
                        return MFS_ERROR_WAITING as uint8_t;
                    }
                }
                return MFS_ERROR_EINVAL as uint8_t;
            }
            l = (*l).next as *mut lock;
        }
        if op as ::core::ffi::c_int == FLOCK_UNLOCK || op as ::core::ffi::c_int == FLOCK_RELEASE {
            return MFS_STATUS_OK as uint8_t;
        }
        ltype = (if op as ::core::ffi::c_int == FLOCK_TRY_SHARED
            || op as ::core::ffi::c_int == FLOCK_LOCK_SHARED
        {
            LTYPE_READER
        } else {
            LTYPE_WRITER
        }) as uint8_t;
        if op as ::core::ffi::c_int == FLOCK_TRY_SHARED
            || op as ::core::ffi::c_int == FLOCK_TRY_EXCLUSIVE
        {
            if flock_check(il, ltype) != 0 {
                return MFS_ERROR_EAGAIN as uint8_t;
            }
        }
        return flock_lock_new(il, ltype, connptr, sessionid, msgid, reqid, owner);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_file_closed(mut sessionid: uint32_t, mut inode: uint32_t) {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut nl: *mut lock = ::core::ptr::null_mut::<lock>();
        il = flock_inode_find(inode);
        if il.is_null() {
            return;
        }
        l = (*il).waiting_head;
        while !l.is_null() {
            nl = (*l).next as *mut lock;
            if (*l).sessionid == sessionid {
                flock_lock_remove(l);
            }
            l = nl;
        }
        l = (*il).active;
        while !l.is_null() {
            nl = (*l).next as *mut lock;
            if (*l).sessionid == sessionid {
                flock_lock_unlock(il, l);
            }
            l = nl;
        }
        if (*il).waiting_head.is_null() && (*il).active.is_null() {
            flock_inode_remove((*il).inode);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_list(mut inode: uint32_t, mut buff: *mut uint8_t) -> uint32_t {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut h: uint32_t = 0;
        let mut ret: uint32_t = 0 as uint32_t;
        if inode == 0 as uint32_t {
            h = 0 as uint32_t;
            while h < FLOCK_INODE_HASHSIZE as uint32_t {
                il = *inodehash.offset(h as isize);
                while !il.is_null() {
                    l = (*il).active;
                    while !l.is_null() {
                        if buff.is_null() {
                            ret = ret.wrapping_add(37 as uint32_t);
                        } else {
                            put32bit(&raw mut buff, (*il).inode);
                            put32bit(&raw mut buff, (*l).sessionid);
                            put64bit(&raw mut buff, (*l).owner);
                            memset(
                                buff as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                20 as size_t,
                            );
                            buff = buff.offset(20 as ::core::ffi::c_int as isize);
                            match (*l).ltype as ::core::ffi::c_int {
                                LTYPE_READER => {
                                    put8bit(&raw mut buff, MFS_LOCK_TYPE_SHARED as uint8_t);
                                }
                                LTYPE_WRITER => {
                                    put8bit(&raw mut buff, MFS_LOCK_TYPE_EXCLUSIVE as uint8_t);
                                }
                                _ => {
                                    put8bit(&raw mut buff, MFS_LOCK_TYPE_UNKNOWN as uint8_t);
                                }
                            }
                        }
                        l = (*l).next as *mut lock;
                    }
                    il = (*il).next as *mut inodelocks;
                }
                h = h.wrapping_add(1);
            }
        } else {
            il = flock_inode_find(inode);
            if !il.is_null() {
                l = (*il).active;
                while !l.is_null() {
                    if buff.is_null() {
                        ret = ret.wrapping_add(33 as uint32_t);
                    } else {
                        put32bit(&raw mut buff, (*l).sessionid);
                        put64bit(&raw mut buff, (*l).owner);
                        memset(
                            buff as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            20 as size_t,
                        );
                        buff = buff.offset(20 as ::core::ffi::c_int as isize);
                        match (*l).ltype as ::core::ffi::c_int {
                            LTYPE_READER => {
                                put8bit(&raw mut buff, MFS_LOCK_TYPE_SHARED as uint8_t);
                            }
                            LTYPE_WRITER => {
                                put8bit(&raw mut buff, MFS_LOCK_TYPE_EXCLUSIVE as uint8_t);
                            }
                            _ => {
                                put8bit(&raw mut buff, MFS_LOCK_TYPE_UNKNOWN as uint8_t);
                            }
                        }
                    }
                    l = (*l).next as *mut lock;
                }
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_mr_change(
    mut inode: uint32_t,
    mut sessionid: uint32_t,
    mut owner: uint64_t,
    mut cmd: ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut nl: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut ltype: uint8_t = 0;
        if cmd as ::core::ffi::c_int == 'U' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'u' as ::core::ffi::c_int
        {
            il = flock_inode_find(inode);
            if il.is_null() {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            l = (*il).active;
            while !l.is_null() {
                nl = (*l).next as *mut lock;
                if (*l).sessionid == sessionid && (*l).owner == owner {
                    flock_do_lock_remove(l);
                    meta_version_inc();
                }
                l = nl;
            }
            if (*il).waiting_head.is_null() && (*il).active.is_null() {
                flock_inode_remove((*il).inode);
            }
            return MFS_STATUS_OK as uint8_t;
        } else if cmd as ::core::ffi::c_int == 'R' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'r' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 's' as ::core::ffi::c_int
        {
            ltype = LTYPE_READER as uint8_t;
        } else if cmd as ::core::ffi::c_int == 'W' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'w' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'E' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'e' as ::core::ffi::c_int
        {
            ltype = LTYPE_WRITER as uint8_t;
        } else {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        il = flock_inode_find(inode);
        if il.is_null() {
            il = flock_inode_new(inode);
        }
        if !(*il).active.is_null()
            && ((*(*il).active).ltype as ::core::ffi::c_int == LTYPE_WRITER
                || ltype as ::core::ffi::c_int == LTYPE_WRITER)
        {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        l = malloc(::core::mem::size_of::<lock>()) as *mut lock;
        (*l).owner = owner;
        (*l).sessionid = sessionid;
        (*l).state = STATE_ACTIVE as uint8_t;
        (*l).ltype = ltype;
        (*l).lock_instances = ::core::ptr::null_mut::<instance>();
        (*l).parent = il as *mut _inodelocks;
        (*l).next = ::core::ptr::null_mut::<_lock>();
        (*l).prev = ::core::ptr::null_mut::<*mut _lock>();
        flock_do_lock_inode_attach(l);
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
pub const FLOCK_REC_SIZE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_store(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut storebuff: [uint8_t; 17] = [0; 17];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut h: uint32_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        if fd.is_null() {
            return 0x10 as uint8_t;
        }
        h = 0 as uint32_t;
        while h < FLOCK_INODE_HASHSIZE as uint32_t {
            il = *inodehash.offset(h as isize);
            while !il.is_null() {
                l = (*il).active;
                while !l.is_null() {
                    ptr = &raw mut storebuff as *mut uint8_t;
                    put32bit(&raw mut ptr, (*il).inode);
                    put64bit(&raw mut ptr, (*l).owner);
                    put32bit(&raw mut ptr, (*l).sessionid);
                    put8bit(&raw mut ptr, (*l).ltype);
                    if bio_write(
                        fd,
                        &raw mut storebuff as *mut uint8_t as *const ::core::ffi::c_void,
                        FLOCK_REC_SIZE as uint64_t,
                    ) != FLOCK_REC_SIZE as int64_t
                    {
                        return 0xff as uint8_t;
                    }
                    l = (*l).next as *mut lock;
                }
                il = (*il).next as *mut inodelocks;
            }
            h = h.wrapping_add(1);
        }
        memset(
            &raw mut storebuff as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            FLOCK_REC_SIZE as size_t,
        );
        if bio_write(
            fd,
            &raw mut storebuff as *mut uint8_t as *const ::core::ffi::c_void,
            FLOCK_REC_SIZE as uint64_t,
        ) != FLOCK_REC_SIZE as int64_t
        {
            return 0xff as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_load(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut loadbuff: [uint8_t; 17] = [0; 17];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut r: int32_t = 0;
        let mut inode: uint32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut owner: uint64_t = 0;
        let mut ltype: uint8_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        if mver as ::core::ffi::c_int != 0x10 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        loop {
            r = bio_read(
                fd,
                &raw mut loadbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                FLOCK_REC_SIZE as uint64_t,
            ) as int32_t;
            if r != FLOCK_REC_SIZE as int32_t {
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut loadbuff as *mut uint8_t;
            inode = get32bit(&raw mut ptr);
            owner = get64bit(&raw mut ptr);
            sessionid = get32bit(&raw mut ptr);
            ltype = get8bit(&raw mut ptr);
            if inode == 0 as uint32_t && owner == 0 as uint64_t && sessionid == 0 as uint32_t {
                return 0 as ::core::ffi::c_int;
            }
            if of_checknode(sessionid, inode) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"loading flock_locks: lock on closed file !!! (ignoring)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading flock_locks: lock on closed file !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                il = flock_inode_find(inode);
                if il.is_null() {
                    il = flock_inode_new(inode);
                }
                if !(*il).active.is_null()
                    && ((*(*il).active).ltype as ::core::ffi::c_int == LTYPE_WRITER
                        || ltype as ::core::ffi::c_int == LTYPE_WRITER)
                {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading flock_locks: wrong lock !!! (ignoring)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading flock_locks: wrong lock !!!\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    l = malloc(::core::mem::size_of::<lock>()) as *mut lock;
                    (*l).owner = owner;
                    (*l).sessionid = sessionid;
                    (*l).state = STATE_ACTIVE as uint8_t;
                    (*l).ltype = ltype;
                    (*l).lock_instances = ::core::ptr::null_mut::<instance>();
                    (*l).parent = il as *mut _inodelocks;
                    (*l).next = ::core::ptr::null_mut::<_lock>();
                    (*l).prev = ::core::ptr::null_mut::<*mut _lock>();
                    flock_do_lock_inode_attach(l);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_cleanup() {
    unsafe {
        let mut h: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut nil: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut nl: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut i: *mut instance = ::core::ptr::null_mut::<instance>();
        let mut ni: *mut instance = ::core::ptr::null_mut::<instance>();
        h = 0 as uint32_t;
        while h < FLOCK_INODE_HASHSIZE as uint32_t {
            il = *inodehash.offset(h as isize);
            while !il.is_null() {
                nil = (*il).next as *mut inodelocks;
                j = 0 as uint32_t;
                while j < 2 as uint32_t {
                    l = if j != 0 {
                        (*il).active
                    } else {
                        (*il).waiting_head
                    };
                    while !l.is_null() {
                        nl = (*l).next as *mut lock;
                        i = (*l).lock_instances;
                        while !i.is_null() {
                            ni = (*i).next as *mut instance;
                            free(i as *mut ::core::ffi::c_void);
                            i = ni;
                        }
                        free(l as *mut ::core::ffi::c_void);
                        l = nl;
                    }
                    j = j.wrapping_add(1);
                }
                free(il as *mut ::core::ffi::c_void);
                il = nil;
            }
            *inodehash.offset(h as isize) = ::core::ptr::null_mut::<inodelocks>();
            h = h.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_info(mut fd: *mut FILE) {
    unsafe {
        let mut h: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut l: *mut lock = ::core::ptr::null_mut::<lock>();
        let mut i: *mut instance = ::core::ptr::null_mut::<instance>();
        let mut lname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if DebugInfo != 0 {
            fprintf(
                fd,
                b"[flock locks]\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            h = 0 as uint32_t;
            while h < FLOCK_INODE_HASHSIZE as uint32_t {
                il = *inodehash.offset(h as isize);
                while !il.is_null() {
                    fprintf(
                        fd,
                        b"- inode: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                        (*il).inode,
                    );
                    j = 0 as uint32_t;
                    while j < 2 as uint32_t {
                        l = if j != 0 {
                            (*il).active
                        } else {
                            (*il).waiting_head
                        };
                        lname = if j != 0 {
                            b"active\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"waiting\0".as_ptr() as *const ::core::ffi::c_char
                        };
                        if !l.is_null() {
                            while !l.is_null() {
                                fprintf(
                                    fd,
                                    b"  - %s_lock: owner: %lu, sessionid: %u, state: %c, type: %c\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    lname,
                                    (*l).owner,
                                    (*l).sessionid,
                                    if (*l).state as ::core::ffi::c_int == STATE_ACTIVE {
                                        'A' as ::core::ffi::c_int
                                    } else if (*l).state as ::core::ffi::c_int == STATE_WAITING
                                    {
                                        'W' as ::core::ffi::c_int
                                    } else {
                                        '?' as ::core::ffi::c_int
                                    },
                                    if (*l).ltype as ::core::ffi::c_int == LTYPE_READER {
                                        'R' as ::core::ffi::c_int
                                    } else if (*l).ltype as ::core::ffi::c_int == LTYPE_WRITER {
                                        'W' as ::core::ffi::c_int
                                    } else {
                                        '?' as ::core::ffi::c_int
                                    },
                                );
                                i = (*l).lock_instances;
                                while !i.is_null() {
                                    fprintf(
                                        fd,
                                        b"    - instance: msgid: %u, reqid: %u\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*i).msgid,
                                        (*i).reqid,
                                    );
                                    i = (*i).next as *mut instance;
                                }
                                l = (*l).next as *mut lock;
                            }
                        }
                        j = j.wrapping_add(1);
                    }
                    il = (*il).next as *mut inodelocks;
                }
                h = h.wrapping_add(1);
            }
            fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_reload() {
    unsafe {
        DebugInfo = cfg_getuint8(
            b"EXTRA_DEBUG_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint8_t,
        );
        FlocksMode = cfg_getuint8(
            b"FLOCK_MODE\0".as_ptr() as *const ::core::ffi::c_char,
            MODE_CORRECT as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn flock_init() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        inodehash = malloc(
            ::core::mem::size_of::<*mut inodelocks>().wrapping_mul(FLOCK_INODE_HASHSIZE as size_t),
        ) as *mut *mut inodelocks;
        i = 0 as uint32_t;
        while i < FLOCK_INODE_HASHSIZE as uint32_t {
            *inodehash.offset(i as isize) = ::core::ptr::null_mut::<inodelocks>();
            i = i.wrapping_add(1);
        }
        flock_reload();
        main_reload_register_fname(
            Some(flock_reload as unsafe extern "C" fn() -> ()),
            b"flock_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_info_register_fname(
            Some(flock_info as unsafe extern "C" fn(*mut FILE) -> ()),
            b"flock_info\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
