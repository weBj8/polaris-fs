pub enum _bio {}
pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    unsafe fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn meta_version_inc() -> uint64_t;
    unsafe fn flock_file_closed(sessionid: uint32_t, inode: uint32_t);
    unsafe fn posix_lock_file_closed(sessionid: uint32_t, inode: uint32_t);
    unsafe fn sessions_find_session(sessionid: uint32_t) -> *mut ::core::ffi::c_void;
    unsafe fn changelog(format: *const ::core::ffi::c_char, ...);
    unsafe fn main_time() -> uint32_t;
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
pub type int32_t = i32;
pub type int64_t = i64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
pub type ofrelation = _ofrelation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ofrelation {
    pub sessionid: uint32_t,
    pub inode: uint32_t,
    pub snext: *mut _ofrelation,
    pub sprev: *mut *mut _ofrelation,
    pub inext: *mut _ofrelation,
    pub iprev: *mut *mut _ofrelation,
}
pub type FILE = _IO_FILE;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OF_INODE_HASHSIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const OF_SESSION_HASHSIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut sessionhash: [*mut ofrelation; 4096] = [::core::ptr::null_mut::<ofrelation>(); 4096];
static mut inodehash: [*mut ofrelation; 65536] = [::core::ptr::null_mut::<ofrelation>(); 65536];
#[inline]
unsafe extern "C" fn of_newnode(mut sessionid: uint32_t, mut inode: uint32_t) {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut shashpos: uint32_t = sessionid.wrapping_rem(OF_SESSION_HASHSIZE as uint32_t);
        let mut ihashpos: uint32_t = inode.wrapping_rem(OF_INODE_HASHSIZE as uint32_t);
        ofr = malloc(::core::mem::size_of::<ofrelation>()) as *mut ofrelation;
        (*ofr).sessionid = sessionid;
        (*ofr).inode = inode;
        (*ofr).inext = inodehash[ihashpos as usize] as *mut _ofrelation;
        (*ofr).iprev = (&raw mut inodehash as *mut *mut ofrelation).offset(ihashpos as isize)
            as *mut *mut _ofrelation;
        if !(*ofr).inext.is_null() {
            (*(*ofr).inext).iprev = &raw mut (*ofr).inext;
        }
        inodehash[ihashpos as usize] = ofr;
        (*ofr).snext = sessionhash[shashpos as usize] as *mut _ofrelation;
        (*ofr).sprev = (&raw mut sessionhash as *mut *mut ofrelation).offset(shashpos as isize)
            as *mut *mut _ofrelation;
        if !(*ofr).snext.is_null() {
            (*(*ofr).snext).sprev = &raw mut (*ofr).snext;
        }
        sessionhash[shashpos as usize] = ofr;
    }
}
#[inline]
unsafe extern "C" fn of_delnode(mut ofr: *mut ofrelation) {
    unsafe {
        flock_file_closed((*ofr).sessionid, (*ofr).inode);
        posix_lock_file_closed((*ofr).sessionid, (*ofr).inode);
        *(*ofr).iprev = (*ofr).inext;
        if !(*ofr).inext.is_null() {
            (*(*ofr).inext).iprev = (*ofr).iprev;
        }
        *(*ofr).sprev = (*ofr).snext;
        if !(*ofr).snext.is_null() {
            (*(*ofr).snext).sprev = (*ofr).sprev;
        }
        free(ofr as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_checknode(mut sessionid: uint32_t, mut inode: uint32_t) -> uint8_t {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut ihashpos: uint32_t = inode.wrapping_rem(OF_INODE_HASHSIZE as uint32_t);
        ofr = inodehash[ihashpos as usize];
        while !ofr.is_null() {
            if (*ofr).sessionid == sessionid && (*ofr).inode == inode {
                return 1 as uint8_t;
            }
            ofr = (*ofr).inext as *mut ofrelation;
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn of_bisearch(
    mut search: uint32_t,
    mut array: *const uint32_t,
    mut n: uint32_t,
) -> int32_t {
    unsafe {
        let mut first: int32_t = 0;
        let mut last: int32_t = 0;
        let mut middle: int32_t = 0;
        first = 0 as ::core::ffi::c_int as int32_t;
        last = n.wrapping_sub(1 as uint32_t) as int32_t;
        middle = (first + last) / 2 as int32_t;
        while first <= last {
            if *array.offset(middle as isize) < search {
                first = middle + 1 as int32_t;
            } else if *array.offset(middle as isize) > search {
                last = middle - 1 as int32_t;
            } else {
                return middle;
            }
            middle = (first + last) / 2 as int32_t;
        }
        return -1 as int32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_inodecmp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aa: uint32_t = *(a as *const uint32_t);
        let mut bb: uint32_t = *(b as *const uint32_t);
        if aa < bb {
            return -1 as ::core::ffi::c_int;
        } else if aa > bb {
            return 1 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_sync(
    mut sessionid: uint32_t,
    mut inodes: *mut uint32_t,
    mut inodecnt: uint32_t,
) {
    unsafe {
        static mut bitmask: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut bitmasksize: uint32_t = 0 as uint32_t;
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut nofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut ipos: int32_t = 0;
        let mut i: uint32_t = 0;
        let mut shashpos: uint32_t = sessionid.wrapping_rem(OF_SESSION_HASHSIZE as uint32_t);
        let mut inode: uint32_t = 0;
        if inodecnt > bitmasksize.wrapping_mul(32 as uint32_t) || bitmask.is_null() {
            if !bitmask.is_null() {
                free(bitmask as *mut ::core::ffi::c_void);
            }
            bitmasksize = inodecnt
                .wrapping_add(31 as uint32_t)
                .wrapping_div(32 as uint32_t)
                .wrapping_add(10 as uint32_t);
            bitmask = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(bitmasksize as size_t))
                as *mut uint32_t;
            if bitmask.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/openfiles.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    159 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bitmask\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/openfiles.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    159 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bitmask\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if bitmask
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/openfiles.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    159 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bitmask\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/openfiles.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    159 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bitmask\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        memset(
            bitmask as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<uint32_t>().wrapping_mul(bitmasksize as size_t),
        );
        qsort(
            inodes as *mut ::core::ffi::c_void,
            inodecnt as size_t,
            ::core::mem::size_of::<uint32_t>(),
            Some(
                of_inodecmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        ofr = sessionhash[shashpos as usize];
        while !ofr.is_null() {
            nofr = (*ofr).snext as *mut ofrelation;
            if (*ofr).sessionid == sessionid {
                ipos = of_bisearch((*ofr).inode, inodes, inodecnt);
                if ipos < 0 as int32_t {
                    inode = (*ofr).inode;
                    of_delnode(ofr);
                    changelog(
                        b"%u|RELEASE(%u,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                        main_time(),
                        sessionid,
                        inode,
                    );
                } else {
                    *bitmask.offset((ipos >> 5 as ::core::ffi::c_int) as isize) = (*bitmask
                        .offset((ipos >> 5 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint) << (ipos & 0x1f as int32_t))
                        as uint32_t;
                }
            }
            ofr = nofr;
        }
        i = 0 as uint32_t;
        while i < inodecnt {
            if *bitmask.offset((i >> 5 as ::core::ffi::c_int) as isize)
                & (1 as uint32_t) << (i & 0x1f as uint32_t)
                == 0 as uint32_t
            {
                changelog(
                    b"%u|ACQUIRE(%u,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    main_time(),
                    sessionid,
                    *inodes.offset(i as isize),
                );
                of_newnode(sessionid, *inodes.offset(i as isize));
            }
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_openfile(mut sessionid: uint32_t, mut inode: uint32_t) {
    unsafe {
        if of_checknode(sessionid, inode) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|ACQUIRE(%u,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                sessionid,
                inode,
            );
            of_newnode(sessionid, inode);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_session_removed(mut sessionid: uint32_t) {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut nofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut shashpos: uint32_t = sessionid.wrapping_rem(OF_SESSION_HASHSIZE as uint32_t);
        ofr = sessionhash[shashpos as usize];
        while !ofr.is_null() {
            nofr = (*ofr).snext as *mut ofrelation;
            if (*ofr).sessionid == sessionid {
                of_delnode(ofr);
            }
            ofr = nofr;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_sessions_info_for_inode(
    mut inode: uint32_t,
    mut dbuff: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut ihashpos: uint32_t = inode.wrapping_rem(OF_INODE_HASHSIZE as uint32_t);
        let mut sescount: uint32_t = 0;
        sescount = 0 as uint32_t;
        ofr = inodehash[ihashpos as usize];
        while !ofr.is_null() {
            if (*ofr).inode == inode {
                sescount = sescount.wrapping_add(1);
            }
            ofr = (*ofr).inext as *mut ofrelation;
        }
        if !dbuff.is_null() {
            put32bit(&raw mut dbuff, sescount);
            ofr = inodehash[ihashpos as usize];
            while !ofr.is_null() {
                if (*ofr).inode == inode {
                    put32bit(&raw mut dbuff, (*ofr).sessionid);
                }
                ofr = (*ofr).inext as *mut ofrelation;
            }
        }
        return (4 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(sescount));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_isfileopen(mut inode: uint32_t) -> uint8_t {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut ihashpos: uint32_t = inode.wrapping_rem(OF_INODE_HASHSIZE as uint32_t);
        ofr = inodehash[ihashpos as usize];
        while !ofr.is_null() {
            if (*ofr).inode == inode {
                return 1 as uint8_t;
            }
            ofr = (*ofr).inext as *mut ofrelation;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_isfileopened_by_session(
    mut inode: uint32_t,
    mut sessionid: uint32_t,
) -> uint8_t {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut ihashpos: uint32_t = inode.wrapping_rem(OF_INODE_HASHSIZE as uint32_t);
        ofr = inodehash[ihashpos as usize];
        while !ofr.is_null() {
            if (*ofr).inode == inode && (*ofr).sessionid == sessionid {
                return 1 as uint8_t;
            }
            ofr = (*ofr).inext as *mut ofrelation;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_noofopenedfiles(mut sessionid: uint32_t) -> uint32_t {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut shashpos: uint32_t = sessionid.wrapping_rem(OF_SESSION_HASHSIZE as uint32_t);
        let mut cnt: uint32_t = 0 as uint32_t;
        ofr = sessionhash[shashpos as usize];
        while !ofr.is_null() {
            if (*ofr).sessionid == sessionid {
                cnt = cnt.wrapping_add(1);
            }
            ofr = (*ofr).snext as *mut ofrelation;
        }
        return cnt;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_lsof(mut sessionid: uint32_t, mut buff: *mut uint8_t) -> uint32_t {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut shashpos: uint32_t = 0;
        let mut ret: uint32_t = 0 as uint32_t;
        if sessionid == 0 as uint32_t {
            shashpos = 0 as uint32_t;
            while shashpos < OF_SESSION_HASHSIZE as uint32_t {
                ofr = sessionhash[shashpos as usize];
                while !ofr.is_null() {
                    if buff.is_null() {
                        ret = ret.wrapping_add(8 as uint32_t);
                    } else {
                        put32bit(&raw mut buff, (*ofr).sessionid);
                        put32bit(&raw mut buff, (*ofr).inode);
                    }
                    ofr = (*ofr).snext as *mut ofrelation;
                }
                shashpos = shashpos.wrapping_add(1);
            }
        } else {
            shashpos = sessionid.wrapping_rem(OF_SESSION_HASHSIZE as uint32_t);
            ofr = sessionhash[shashpos as usize];
            while !ofr.is_null() {
                if (*ofr).sessionid == sessionid {
                    if buff.is_null() {
                        ret = ret.wrapping_add(4 as uint32_t);
                    } else {
                        put32bit(&raw mut buff, (*ofr).inode);
                    }
                }
                ofr = (*ofr).snext as *mut ofrelation;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_mr_acquire(
    mut sessionid: uint32_t,
    mut inode: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        if of_checknode(sessionid, inode) != 0 {
            return MFS_ERROR_MISMATCH;
        }
        of_newnode(sessionid, inode);
        meta_version_inc();
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_mr_release(
    mut sessionid: uint32_t,
    mut inode: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut nofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut ihashpos: uint32_t = inode.wrapping_rem(OF_INODE_HASHSIZE as uint32_t);
        ofr = inodehash[ihashpos as usize];
        while !ofr.is_null() {
            nofr = (*ofr).inext as *mut ofrelation;
            if (*ofr).sessionid == sessionid && (*ofr).inode == inode {
                of_delnode(ofr);
                meta_version_inc();
                return MFS_STATUS_OK;
            }
            ofr = nofr;
        }
        return MFS_ERROR_MISMATCH;
    }
}
pub const OF_REC_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_store(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut storebuff: [uint8_t; 8] = [0; 8];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut i: uint32_t = 0;
        if fd.is_null() {
            return 0x10 as uint8_t;
        }
        i = 0 as uint32_t;
        while i < OF_SESSION_HASHSIZE as uint32_t {
            ofr = sessionhash[i as usize];
            while !ofr.is_null() {
                ptr = &raw mut storebuff as *mut uint8_t;
                put32bit(&raw mut ptr, (*ofr).sessionid);
                put32bit(&raw mut ptr, (*ofr).inode);
                if bio_write(
                    fd,
                    &raw mut storebuff as *mut uint8_t as *const ::core::ffi::c_void,
                    OF_REC_SIZE as uint64_t,
                ) != OF_REC_SIZE as int64_t
                {
                    return 0xff as uint8_t;
                }
                ofr = (*ofr).snext as *mut ofrelation;
            }
            i = i.wrapping_add(1);
        }
        memset(
            &raw mut storebuff as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            OF_REC_SIZE as size_t,
        );
        if bio_write(
            fd,
            &raw mut storebuff as *mut uint8_t as *const ::core::ffi::c_void,
            OF_REC_SIZE as uint64_t,
        ) != OF_REC_SIZE as int64_t
        {
            return 0xff as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_load(mut fd: *mut bio, mut mver: uint8_t) -> ::core::ffi::c_int {
    unsafe {
        let mut loadbuff: [uint8_t; 8] = [0; 8];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut r: int32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        if mver as ::core::ffi::c_int != 0x10 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        loop {
            r = bio_read(
                fd,
                &raw mut loadbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                OF_REC_SIZE as uint64_t,
            ) as int32_t;
            if r != OF_REC_SIZE as int32_t {
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut loadbuff as *mut uint8_t;
            sessionid = get32bit(&raw mut ptr);
            inode = get32bit(&raw mut ptr);
            if sessionid > 0 as uint32_t && inode > 0 as uint32_t {
                if !sessions_find_session(sessionid).is_null() {
                    of_newnode(sessionid, inode);
                }
            } else if sessionid == 0 as uint32_t && inode == 0 as uint32_t {
                return 0 as ::core::ffi::c_int;
            } else {
                return -1 as ::core::ffi::c_int;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_cleanup() {
    unsafe {
        let mut ofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut nofr: *mut ofrelation = ::core::ptr::null_mut::<ofrelation>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < OF_SESSION_HASHSIZE as uint32_t {
            ofr = sessionhash[i as usize];
            while !ofr.is_null() {
                nofr = (*ofr).snext as *mut ofrelation;
                free(ofr as *mut ::core::ffi::c_void);
                ofr = nofr;
            }
            sessionhash[i as usize] = ::core::ptr::null_mut::<ofrelation>();
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < OF_INODE_HASHSIZE as uint32_t {
            inodehash[i as usize] = ::core::ptr::null_mut::<ofrelation>();
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_init() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < OF_SESSION_HASHSIZE as uint32_t {
            sessionhash[i as usize] = ::core::ptr::null_mut::<ofrelation>();
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < OF_INODE_HASHSIZE as uint32_t {
            inodehash[i as usize] = ::core::ptr::null_mut::<ofrelation>();
            i = i.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
