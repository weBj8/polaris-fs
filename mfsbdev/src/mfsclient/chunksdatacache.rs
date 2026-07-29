pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
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
pub struct _chunks_inode_entry {
    pub inode: uint32_t,
    pub data_head: *mut _chunks_data_entry,
    pub prev: *mut *mut _chunks_inode_entry,
    pub next: *mut _chunks_inode_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunks_data_entry {
    pub inode: uint32_t,
    pub chindx: uint32_t,
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub csdataver: uint8_t,
    pub csdata: *mut uint8_t,
    pub csdatasize: uint32_t,
    pub parent: *mut _chunks_inode_entry,
    pub previnode: *mut *mut _chunks_data_entry,
    pub nextinode: *mut _chunks_data_entry,
    pub prevdata: *mut *mut _chunks_data_entry,
    pub nextdata: *mut _chunks_data_entry,
}
pub type chunks_data_entry = _chunks_data_entry;
pub type chunks_inode_entry = _chunks_inode_entry;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHUNKS_INODE_HASH_SIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const CHUNKS_DATA_HASH_SIZE: ::core::ffi::c_int = 524288 as ::core::ffi::c_int;
static mut chunks_inode_hash: *mut *mut chunks_inode_entry =
    ::core::ptr::null_mut::<*mut chunks_inode_entry>();
static mut chunks_data_hash: *mut *mut chunks_data_entry =
    ::core::ptr::null_mut::<*mut chunks_data_entry>();
static mut lock: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn chunks_inode_hash_fn(mut inode: uint32_t) -> uint32_t {
    unsafe {
        return inode.wrapping_mul(0x72b5f387 as uint32_t)
            & (CHUNKS_INODE_HASH_SIZE - 1 as ::core::ffi::c_int) as uint32_t;
    }
}
#[inline]
unsafe extern "C" fn chunks_data_hash_fn(mut inode: uint32_t, mut chindx: uint32_t) -> uint32_t {
    unsafe {
        return inode
            .wrapping_mul(0x72b5f387 as uint32_t)
            .wrapping_add(chindx)
            .wrapping_mul(0x56bf7623 as uint32_t)
            & (CHUNKS_DATA_HASH_SIZE - 1 as ::core::ffi::c_int) as uint32_t;
    }
}
#[inline]
unsafe extern "C" fn chunks_try_remove_inode(mut ih: *mut chunks_inode_entry) {
    unsafe {
        if (*ih).data_head.is_null() {
            *(*ih).prev = (*ih).next;
            if !(*ih).next.is_null() {
                (*(*ih).next).prev = (*ih).prev;
            }
            free(ih as *mut ::core::ffi::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn chunks_remove_entry(mut ca: *mut chunks_data_entry) {
    unsafe {
        *(*ca).previnode = (*ca).nextinode;
        if !(*ca).nextinode.is_null() {
            (*(*ca).nextinode).previnode = (*ca).previnode;
        }
        *(*ca).prevdata = (*ca).nextdata;
        if !(*ca).nextdata.is_null() {
            (*(*ca).nextdata).prevdata = (*ca).prevdata;
        }
        if !(*ca).csdata.is_null() {
            free((*ca).csdata as *mut ::core::ffi::c_void);
        }
        chunks_try_remove_inode((*ca).parent as *mut chunks_inode_entry);
        free(ca as *mut ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C" fn chunks_new_entry(
    mut inode: uint32_t,
    mut chindx: uint32_t,
) -> *mut chunks_data_entry {
    unsafe {
        let mut ih: *mut chunks_inode_entry = ::core::ptr::null_mut::<chunks_inode_entry>();
        let mut ca: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut hash: uint32_t = 0;
        let mut ihash: uint32_t = 0;
        ihash = chunks_inode_hash_fn(inode);
        ih = *chunks_inode_hash.offset(ihash as isize);
        while !ih.is_null() && (*ih).inode != inode {
            ih = (*ih).next as *mut chunks_inode_entry;
        }
        if ih.is_null() {
            ih = malloc(::core::mem::size_of::<chunks_inode_entry>()) as *mut chunks_inode_entry;
            (*ih).inode = inode;
            (*ih).data_head = ::core::ptr::null_mut::<_chunks_data_entry>();
            (*ih).next = *chunks_inode_hash.offset(ihash as isize) as *mut _chunks_inode_entry;
            if !(*ih).next.is_null() {
                (*(*ih).next).prev = &raw mut (*ih).next;
            }
            (*ih).prev = chunks_inode_hash.offset(ihash as isize) as *mut *mut _chunks_inode_entry;
            *chunks_inode_hash.offset(ihash as isize) = ih;
        }
        hash = chunks_data_hash_fn(inode, chindx);
        ca = malloc(::core::mem::size_of::<chunks_data_entry>()) as *mut chunks_data_entry;
        (*ca).inode = inode;
        (*ca).chindx = chindx;
        (*ca).chunkid = 0 as uint64_t;
        (*ca).version = 0 as uint32_t;
        (*ca).csdata = ::core::ptr::null_mut::<uint8_t>();
        (*ca).csdatasize = 0 as uint32_t;
        (*ca).csdataver = 0 as uint8_t;
        (*ca).parent = ih as *mut _chunks_inode_entry;
        (*ca).nextinode = (*ih).data_head;
        if !(*ca).nextinode.is_null() {
            (*(*ca).nextinode).previnode = &raw mut (*ca).nextinode;
        }
        (*ca).previnode = &raw mut (*ih).data_head;
        (*ih).data_head = ca as *mut _chunks_data_entry;
        (*ca).nextdata = *chunks_data_hash.offset(hash as isize) as *mut _chunks_data_entry;
        if !(*ca).nextdata.is_null() {
            (*(*ca).nextdata).prevdata = &raw mut (*ca).nextdata;
        }
        (*ca).prevdata = chunks_data_hash.offset(hash as isize) as *mut *mut _chunks_data_entry;
        *chunks_data_hash.offset(hash as isize) = ca;
        return ca;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_clear_inode(mut inode: uint32_t, mut chindx: uint32_t) {
    unsafe {
        let mut ih: *mut chunks_inode_entry = ::core::ptr::null_mut::<chunks_inode_entry>();
        let mut ihn: *mut chunks_inode_entry = ::core::ptr::null_mut::<chunks_inode_entry>();
        let mut ca: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut can: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        pthread_mutex_lock(&raw mut lock);
        ih = *chunks_inode_hash.offset(chunks_inode_hash_fn(inode) as isize);
        while !ih.is_null() {
            ihn = (*ih).next as *mut chunks_inode_entry;
            if (*ih).inode == inode {
                ca = (*ih).data_head as *mut chunks_data_entry;
                while !ca.is_null() {
                    can = (*ca).nextinode as *mut chunks_data_entry;
                    if (*ca).chindx >= chindx {
                        chunks_remove_entry(ca);
                    }
                    ca = can;
                }
            }
            ih = ihn;
        }
        pthread_mutex_unlock(&raw mut lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_invalidate(mut inode: uint32_t, mut chindx: uint32_t) {
    unsafe {
        let mut ca: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut hash: uint32_t = 0;
        pthread_mutex_lock(&raw mut lock);
        hash = chunks_data_hash_fn(inode, chindx);
        ca = *chunks_data_hash.offset(hash as isize);
        while !ca.is_null() {
            if (*ca).inode == inode && (*ca).chindx == chindx {
                chunks_remove_entry(ca);
                pthread_mutex_unlock(&raw mut lock);
                return;
            }
            ca = (*ca).nextdata as *mut chunks_data_entry;
        }
        pthread_mutex_unlock(&raw mut lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_check(
    mut inode: uint32_t,
    mut chindx: uint32_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
) -> uint8_t {
    unsafe {
        let mut ca: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut hash: uint32_t = 0;
        pthread_mutex_lock(&raw mut lock);
        hash = chunks_data_hash_fn(inode, chindx);
        ca = *chunks_data_hash.offset(hash as isize);
        while !ca.is_null() {
            if (*ca).inode == inode && (*ca).chindx == chindx {
                if (*ca).chunkid == chunkid && (*ca).version == version {
                    pthread_mutex_unlock(&raw mut lock);
                    return 1 as uint8_t;
                } else {
                    pthread_mutex_unlock(&raw mut lock);
                    return 0 as uint8_t;
                }
            }
            ca = (*ca).nextdata as *mut chunks_data_entry;
        }
        pthread_mutex_unlock(&raw mut lock);
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_change(
    mut inode: uint32_t,
    mut chindx: uint32_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
) {
    unsafe {
        let mut ca: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut hash: uint32_t = 0;
        pthread_mutex_lock(&raw mut lock);
        hash = chunks_data_hash_fn(inode, chindx);
        ca = *chunks_data_hash.offset(hash as isize);
        while !ca.is_null() {
            if (*ca).inode == inode && (*ca).chindx == chindx {
                (*ca).chunkid = chunkid;
                (*ca).version = version;
                pthread_mutex_unlock(&raw mut lock);
                return;
            }
            ca = (*ca).nextdata as *mut chunks_data_entry;
        }
        pthread_mutex_unlock(&raw mut lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_insert(
    mut inode: uint32_t,
    mut chindx: uint32_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut csdataver: uint8_t,
    mut csdata: *const uint8_t,
    mut csdatasize: uint32_t,
) {
    unsafe {
        let mut ca: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut hash: uint32_t = 0;
        pthread_mutex_lock(&raw mut lock);
        hash = chunks_data_hash_fn(inode, chindx);
        ca = *chunks_data_hash.offset(hash as isize);
        while !ca.is_null() {
            if (*ca).inode == inode && (*ca).chindx == chindx {
                break;
            }
            ca = (*ca).nextdata as *mut chunks_data_entry;
        }
        if ca.is_null() {
            ca = chunks_new_entry(inode, chindx);
        }
        (*ca).chunkid = chunkid;
        (*ca).version = version;
        (*ca).csdataver = csdataver;
        if (*ca).csdatasize == csdatasize {
            if csdatasize > 0 as uint32_t {
                memcpy(
                    (*ca).csdata as *mut ::core::ffi::c_void,
                    csdata as *const ::core::ffi::c_void,
                    csdatasize as size_t,
                );
            }
        } else {
            if !(*ca).csdata.is_null() {
                free((*ca).csdata as *mut ::core::ffi::c_void);
            }
            if csdatasize > 0 as uint32_t {
                (*ca).csdata = malloc(csdatasize as size_t) as *mut uint8_t;
                memcpy(
                    (*ca).csdata as *mut ::core::ffi::c_void,
                    csdata as *const ::core::ffi::c_void,
                    csdatasize as size_t,
                );
            } else {
                (*ca).csdata = ::core::ptr::null_mut::<uint8_t>();
            }
            (*ca).csdatasize = csdatasize;
        }
        pthread_mutex_unlock(&raw mut lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_find(
    mut inode: uint32_t,
    mut chindx: uint32_t,
    mut chunkid: *mut uint64_t,
    mut version: *mut uint32_t,
    mut csdataver: *mut uint8_t,
    mut csdata: *mut uint8_t,
    mut csdatasize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut ca: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut hash: uint32_t = 0;
        pthread_mutex_lock(&raw mut lock);
        hash = chunks_data_hash_fn(inode, chindx);
        ca = *chunks_data_hash.offset(hash as isize);
        while !ca.is_null() {
            if (*ca).inode == inode && (*ca).chindx == chindx {
                if *csdatasize < (*ca).csdatasize {
                    pthread_mutex_unlock(&raw mut lock);
                    return 0 as uint8_t;
                }
                *chunkid = (*ca).chunkid;
                *version = (*ca).version;
                *csdataver = (*ca).csdataver;
                memcpy(
                    csdata as *mut ::core::ffi::c_void,
                    (*ca).csdata as *const ::core::ffi::c_void,
                    (*ca).csdatasize as size_t,
                );
                *csdatasize = (*ca).csdatasize;
                pthread_mutex_unlock(&raw mut lock);
                return 1 as uint8_t;
            }
            ca = (*ca).nextdata as *mut chunks_data_entry;
        }
        pthread_mutex_unlock(&raw mut lock);
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_cleanup() {
    unsafe {
        let mut ih: *mut chunks_inode_entry = ::core::ptr::null_mut::<chunks_inode_entry>();
        let mut ihn: *mut chunks_inode_entry = ::core::ptr::null_mut::<chunks_inode_entry>();
        let mut ca: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut can: *mut chunks_data_entry = ::core::ptr::null_mut::<chunks_data_entry>();
        let mut hash: uint32_t = 0;
        pthread_mutex_lock(&raw mut lock);
        hash = 0 as uint32_t;
        while hash < CHUNKS_INODE_HASH_SIZE as uint32_t {
            ih = *chunks_inode_hash.offset(hash as isize);
            while !ih.is_null() {
                ihn = (*ih).next as *mut chunks_inode_entry;
                free(ih as *mut ::core::ffi::c_void);
                ih = ihn;
            }
            *chunks_inode_hash.offset(hash as isize) =
                ::core::ptr::null_mut::<chunks_inode_entry>();
            hash = hash.wrapping_add(1);
        }
        hash = 0 as uint32_t;
        while hash < CHUNKS_DATA_HASH_SIZE as uint32_t {
            ca = *chunks_data_hash.offset(hash as isize);
            while !ca.is_null() {
                can = (*ca).nextdata as *mut chunks_data_entry;
                if !(*ca).csdata.is_null() {
                    free((*ca).csdata as *mut ::core::ffi::c_void);
                }
                free(ca as *mut ::core::ffi::c_void);
                ca = can;
            }
            *chunks_data_hash.offset(hash as isize) = ::core::ptr::null_mut::<chunks_data_entry>();
            hash = hash.wrapping_add(1);
        }
        pthread_mutex_unlock(&raw mut lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_term() {
    unsafe {
        chunksdatacache_cleanup();
        free(chunks_inode_hash as *mut ::core::ffi::c_void);
        free(chunks_data_hash as *mut ::core::ffi::c_void);
        pthread_mutex_destroy(&raw mut lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunksdatacache_init() {
    unsafe {
        let mut hash: uint32_t = 0;
        chunks_inode_hash = malloc(
            ::core::mem::size_of::<*mut chunks_inode_entry>()
                .wrapping_mul(CHUNKS_INODE_HASH_SIZE as size_t),
        ) as *mut *mut chunks_inode_entry;
        if chunks_inode_hash.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/chunksdatacache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                354 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chunks_inode_hash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/chunksdatacache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                354 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chunks_inode_hash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if chunks_inode_hash
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut chunks_inode_entry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/chunksdatacache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                354 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chunks_inode_hash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/chunksdatacache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                354 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chunks_inode_hash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunks_data_hash = malloc(
            ::core::mem::size_of::<*mut chunks_data_entry>()
                .wrapping_mul(CHUNKS_DATA_HASH_SIZE as size_t),
        ) as *mut *mut chunks_data_entry;
        if chunks_data_hash.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/chunksdatacache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                357 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chunks_data_hash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/chunksdatacache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                357 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chunks_data_hash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if chunks_data_hash
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut chunks_data_entry
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/chunksdatacache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                357 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chunks_data_hash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/chunksdatacache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                357 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chunks_data_hash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        hash = 0 as uint32_t;
        while hash < CHUNKS_INODE_HASH_SIZE as uint32_t {
            *chunks_inode_hash.offset(hash as isize) =
                ::core::ptr::null_mut::<chunks_inode_entry>();
            hash = hash.wrapping_add(1);
        }
        hash = 0 as uint32_t;
        while hash < CHUNKS_DATA_HASH_SIZE as uint32_t {
            *chunks_data_hash.offset(hash as isize) = ::core::ptr::null_mut::<chunks_data_entry>();
            hash = hash.wrapping_add(1);
        }
        pthread_mutex_init(&raw mut lock, ::core::ptr::null::<pthread_mutexattr_t>());
    }
}
