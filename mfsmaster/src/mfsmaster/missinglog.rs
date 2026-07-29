extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn chunk_remove_from_missing_log(chunkid: uint64_t) -> uint8_t;
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlogentry {
    pub chunkid: uint64_t,
    pub inode: uint32_t,
    pub indx: uint32_t,
    pub r#type: uint8_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put64bit(mut ptr: *mut *mut uint8_t, mut val: uint64_t) {
    val = val.swap_bytes() as uint64_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        8 as size_t,
    );
    *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    val = val.swap_bytes() as uint32_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn put8bit(mut ptr: *mut *mut uint8_t, mut val: uint8_t) {
    *(*ptr).offset(0 as isize) =
        (val as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    *ptr = (*ptr).offset(1);
}
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut mloghash: *mut mlogentry = ::core::ptr::null_mut::<mlogentry>();
static mut mloghashprev: *mut mlogentry = ::core::ptr::null_mut::<mlogentry>();
static mut mloghashsize: uint32_t = 0;
static mut mloghashprevsize: uint32_t = 0;
static mut mloghashelements: uint32_t = 0;
static mut mloghashprevelements: uint32_t = 0;
static mut mloghashcapacity: uint32_t = 0;
static mut blocked: uint8_t = 0;
#[no_mangle]
pub unsafe extern "C" fn missing_log_insert(
    mut chunkid: uint64_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut r#type: uint8_t,
) {
    let mut hash: uint32_t = 0;
    let mut disp: uint32_t = 0;
    if blocked as ::core::ffi::c_int != 0
        || mloghashelements >= mloghashcapacity
        || chunkid == 0 as uint64_t
    {
        return;
    }
    hash = ((inode.wrapping_mul(1363546567 as uint32_t) ^ indx).wrapping_mul(985732289 as uint32_t)
        as uint64_t
        ^ chunkid) as uint32_t;
    disp = (((inode as ::core::ffi::c_long * 2345123993 as ::core::ffi::c_long
        + indx as ::core::ffi::c_long)
        * 746344009 as ::core::ffi::c_long) as uint64_t)
        .wrapping_add(chunkid) as uint32_t;
    hash = hash.wrapping_rem(mloghashsize);
    disp = disp.wrapping_rem(mloghashsize);
    disp |= 1 as uint32_t;
    while (*mloghash.offset(hash as isize)).chunkid != 0 as uint64_t {
        if (*mloghash.offset(hash as isize)).chunkid == chunkid
            && (*mloghash.offset(hash as isize)).inode == inode
            && (*mloghash.offset(hash as isize)).indx == indx
        {
            return;
        }
        hash = hash.wrapping_add(disp);
        hash = hash.wrapping_rem(mloghashsize);
    }
    (*mloghash.offset(hash as isize)).chunkid = chunkid;
    (*mloghash.offset(hash as isize)).inode = inode;
    (*mloghash.offset(hash as isize)).indx = indx;
    (*mloghash.offset(hash as isize)).r#type = r#type;
    mloghashelements = mloghashelements.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn missing_log_swap() {
    let mut mloghashtmp: *mut mlogentry = ::core::ptr::null_mut::<mlogentry>();
    if blocked != 0 {
        blocked = 0 as uint8_t;
    } else {
        if mloghashsize == mloghashprevsize {
            mloghashtmp = mloghashprev;
            mloghashprev = mloghash;
            mloghash = mloghashtmp;
        } else {
            free(mloghashprev as *mut ::core::ffi::c_void);
            mloghashprev = mloghash;
            mloghash =
                malloc(::core::mem::size_of::<mlogentry>().wrapping_mul(mloghashsize as size_t))
                    as *mut mlogentry;
        }
        memset(
            mloghash as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mlogentry>().wrapping_mul(mloghashsize as size_t),
        );
        mloghashprevsize = mloghashsize;
        mloghashprevelements = mloghashelements;
        mloghashelements = 0 as uint32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn missing_log_getdata(
    mut buff: *mut uint8_t,
    mut mode: uint8_t,
) -> uint32_t {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    if buff.is_null() {
        i = 0 as uint32_t;
        while i < mloghashprevsize {
            if (*mloghashprev.offset(i as isize)).chunkid != 0 as uint64_t {
                if chunk_remove_from_missing_log((*mloghashprev.offset(i as isize)).chunkid) != 0 {
                    (*mloghashprev.offset(i as isize)).chunkid = 0 as uint64_t;
                    mloghashprevelements = mloghashprevelements.wrapping_sub(1);
                }
            }
            i = i.wrapping_add(1);
        }
        return mloghashprevelements.wrapping_mul(
            (if mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                16 as ::core::ffi::c_int
            } else {
                17 as ::core::ffi::c_int
            }) as uint32_t,
        );
    } else {
        j = 0 as uint32_t;
        i = 0 as uint32_t;
        while i < mloghashprevsize && j < mloghashprevelements {
            if (*mloghashprev.offset(i as isize)).chunkid != 0 as uint64_t {
                put64bit(&raw mut buff, (*mloghashprev.offset(i as isize)).chunkid);
                put32bit(&raw mut buff, (*mloghashprev.offset(i as isize)).inode);
                put32bit(&raw mut buff, (*mloghashprev.offset(i as isize)).indx);
                if mode != 0 {
                    put8bit(&raw mut buff, (*mloghashprev.offset(i as isize)).r#type);
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        return 0 as uint32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn missing_log_reload() {
    let mut ncapacity: uint32_t = 0;
    ncapacity = cfg_getuint32(
        b"MISSING_LOG_CAPACITY\0".as_ptr() as *const ::core::ffi::c_char,
        100000 as uint32_t,
    );
    if ncapacity < 1000 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"MISSING_LOG_CAPACITY to low - increased to 1000\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if ncapacity > 1000000 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"MISSING_LOG_CAPACITY to high - decreased to 1000000\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if ncapacity != mloghashcapacity {
        if !mloghash.is_null() {
            free(mloghash as *mut ::core::ffi::c_void);
        }
        mloghashsize = 1024 as uint32_t;
        while mloghashsize
            < ncapacity
                .wrapping_mul(3 as uint32_t)
                .wrapping_div(2 as uint32_t)
        {
            mloghashsize <<= 1 as ::core::ffi::c_int;
        }
        mloghash = malloc(::core::mem::size_of::<mlogentry>().wrapping_mul(mloghashsize as size_t))
            as *mut mlogentry;
        memset(
            mloghash as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mlogentry>().wrapping_mul(mloghashsize as size_t),
        );
        mloghashelements = 0 as uint32_t;
        if mloghashcapacity == 0 as uint32_t {
            blocked = 0 as uint8_t;
        } else {
            blocked = 1 as uint8_t;
        }
        mloghashcapacity = ncapacity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn missing_log_init() -> ::core::ffi::c_int {
    mloghash = ::core::ptr::null_mut::<mlogentry>();
    mloghashprev = ::core::ptr::null_mut::<mlogentry>();
    mloghashsize = 0 as uint32_t;
    mloghashprevsize = 0 as uint32_t;
    mloghashelements = 0 as uint32_t;
    mloghashprevelements = 0 as uint32_t;
    mloghashcapacity = 0 as uint32_t;
    blocked = 1 as uint8_t;
    missing_log_reload();
    main_reload_register_fname(
        Some(missing_log_reload as unsafe extern "C" fn() -> ()),
        b"missing_log_reload\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 1 as ::core::ffi::c_int;
}
