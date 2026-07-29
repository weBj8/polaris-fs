unsafe extern "C" {
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
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
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn fs_path_lookup(
        base_inode: uint32_t,
        pleng: uint32_t,
        path: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        parent_inode: *mut uint32_t,
        last_inode: *mut uint32_t,
        nleng: *mut uint8_t,
        name: *mut uint8_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
}
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
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _path_lookup_cache {
    pub hash: uint32_t,
    pub refresh_in_progress: uint8_t,
    pub cond: pthread_cond_t,
    pub base_inode: uint32_t,
    pub pleng: uint32_t,
    pub path: [uint8_t; 1024],
    pub uid: uint32_t,
    pub gidcnt: uint32_t,
    pub gidtab: [uint32_t; 256],
    pub parent: uint32_t,
    pub inode: uint32_t,
    pub nleng: uint8_t,
    pub name: [uint8_t; 256],
    pub attr: [uint8_t; 36],
    pub validts: ::core::ffi::c_double,
}
pub type path_lookup_cache = _path_lookup_cache;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFS_NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const MFS_PATH_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_ENAMETOOLONG: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const ATTR_RECORD_SIZE: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const LCACHE_HENTRIES: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const LCACHE_QENTRIES: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut lcache: [[path_lookup_cache; 8]; 64] = [[path_lookup_cache {
    hash: 0,
    refresh_in_progress: 0,
    cond: pthread_cond_t {
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
    },
    base_inode: 0,
    pleng: 0,
    path: [0; 1024],
    uid: 0,
    gidcnt: 0,
    gidtab: [0; 256],
    parent: 0,
    inode: 0,
    nleng: 0,
    name: [0; 256],
    attr: [0; 36],
    validts: 0.,
}; 8]; 64];
static mut lcache_lock: [pthread_mutex_t; 64] = [pthread_mutex_t {
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
}; 64];
static mut lcache_retention: ::core::ffi::c_double = 0.;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcache_path_normalize(
    mut pleng: uint32_t,
    mut path: *const uint8_t,
    mut rpleng: *mut uint32_t,
    mut rpath: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut rleng: uint32_t = 0;
        let mut pptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pend: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut partlen: uint32_t = 0;
        pptr = path;
        pend = path.offset(pleng as isize);
        partlen = 0 as uint32_t;
        rleng = 0 as uint32_t;
        while *pptr as ::core::ffi::c_int != 0 && pptr < pend {
            if *pptr as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                if partlen > 0 as uint32_t {
                    if partlen == 2 as uint32_t
                        && *rpath.offset(rleng.wrapping_sub(1 as uint32_t) as isize)
                            as ::core::ffi::c_int
                            == '.' as ::core::ffi::c_int
                        && *rpath.offset(rleng.wrapping_sub(2 as uint32_t) as isize)
                            as ::core::ffi::c_int
                            == '.' as ::core::ffi::c_int
                    {
                        if rleng < 3 as uint32_t {
                            return MFS_ERROR_EINVAL as uint8_t;
                        }
                        rleng = rleng.wrapping_sub(3 as uint32_t);
                        while rleng > 0 as uint32_t
                            && *rpath.offset(rleng.wrapping_sub(1 as uint32_t) as isize)
                                as ::core::ffi::c_int
                                != '/' as ::core::ffi::c_int
                        {
                            rleng = rleng.wrapping_sub(1);
                        }
                    } else if partlen == 1 as uint32_t
                        && *rpath.offset(rleng.wrapping_sub(1 as uint32_t) as isize)
                            as ::core::ffi::c_int
                            == '.' as ::core::ffi::c_int
                    {
                        rleng = rleng.wrapping_sub(1);
                    } else {
                        if rleng >= MFS_PATH_MAX as uint32_t {
                            return MFS_ERROR_ENAMETOOLONG as uint8_t;
                        }
                        let c2rust_fresh0 = rleng;
                        rleng = rleng.wrapping_add(1);
                        *rpath.offset(c2rust_fresh0 as isize) = '/' as uint8_t;
                    }
                }
                partlen = 0 as uint32_t;
            } else {
                if partlen >= MFS_NAME_MAX as uint32_t {
                    return MFS_ERROR_ENAMETOOLONG as uint8_t;
                }
                if rleng >= MFS_PATH_MAX as uint32_t {
                    return MFS_ERROR_ENAMETOOLONG as uint8_t;
                }
                let c2rust_fresh1 = rleng;
                rleng = rleng.wrapping_add(1);
                *rpath.offset(c2rust_fresh1 as isize) = *pptr;
                partlen = partlen.wrapping_add(1);
            }
            pptr = pptr.offset(1);
        }
        if rleng >= MFS_PATH_MAX as uint32_t {
            return MFS_ERROR_ENAMETOOLONG as uint8_t;
        }
        *rpath.offset(rleng as isize) = '\0' as uint8_t;
        *rpleng = rleng;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcache_hash(
    mut base_inode: uint32_t,
    mut pleng: uint32_t,
    mut path: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut i: uint32_t = 0;
        hash = base_inode;
        i = 0 as uint32_t;
        while i < pleng {
            hash = hash
                .wrapping_mul(33 as uint32_t)
                .wrapping_add(*path.offset(i as isize) as uint32_t);
            i = i.wrapping_add(1);
        }
        return hash;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcache_path_lookup(
    mut base_inode: uint32_t,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
    mut uid: uint32_t,
    mut gidcnt: uint32_t,
    mut gidtab: *mut uint32_t,
    mut parent_inode: *mut uint32_t,
    mut last_inode: *mut uint32_t,
    mut nleng: *mut uint8_t,
    mut name: *mut uint8_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut rpath: [uint8_t; 1024] = [0; 1024];
        let mut rpleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut hash: uint32_t = 0;
        let mut hind: uint32_t = 0;
        let mut qind: uint32_t = 0;
        let mut plc: *mut path_lookup_cache = ::core::ptr::null_mut::<path_lookup_cache>();
        let mut minplc: *mut path_lookup_cache = ::core::ptr::null_mut::<path_lookup_cache>();
        let mut ts: ::core::ffi::c_double = 0.;
        ts = monotonic_seconds();
        status =
            lcache_path_normalize(pleng, path, &raw mut rpleng, &raw mut rpath as *mut uint8_t);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        hash = lcache_hash(base_inode, rpleng, &raw mut rpath as *mut uint8_t);
        hind = hash.wrapping_rem(LCACHE_HENTRIES as uint32_t);
        pthread_mutex_lock((&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize));
        minplc = ::core::ptr::null_mut::<path_lookup_cache>();
        qind = 0 as uint32_t;
        while qind < LCACHE_QENTRIES as uint32_t {
            plc = (&raw mut *(&raw mut lcache as *mut [path_lookup_cache; 8]).offset(hind as isize)
                as *mut path_lookup_cache)
                .offset(qind as isize);
            if (*plc).hash == hash
                && (*plc).base_inode == base_inode
                && (*plc).pleng == rpleng
                && (*plc).uid == uid
                && (*plc).gidcnt == gidcnt
                && memcmp(
                    &raw mut (*plc).path as *mut uint8_t as *const ::core::ffi::c_void,
                    &raw mut rpath as *mut uint8_t as *const ::core::ffi::c_void,
                    rpleng as size_t,
                ) == 0 as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*plc).gidtab as *mut uint32_t as *const ::core::ffi::c_void,
                    gidtab as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<uint32_t>().wrapping_mul(gidcnt as size_t),
                ) == 0 as ::core::ffi::c_int
            {
                while (*plc).refresh_in_progress != 0 {
                    pthread_cond_wait(
                        &raw mut (*plc).cond,
                        (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
                    );
                }
                if (*plc).validts <= ts {
                    (*plc).refresh_in_progress = 1 as uint8_t;
                    pthread_mutex_unlock(
                        (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
                    );
                    status = fs_path_lookup(
                        base_inode,
                        rpleng,
                        &raw mut rpath as *mut uint8_t,
                        uid,
                        gidcnt,
                        gidtab,
                        &raw mut (*plc).parent,
                        &raw mut (*plc).inode,
                        &raw mut (*plc).nleng,
                        &raw mut (*plc).name as *mut uint8_t,
                        &raw mut (*plc).attr as *mut uint8_t,
                    );
                    ts = monotonic_seconds();
                    pthread_mutex_lock(
                        (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
                    );
                    (*plc).refresh_in_progress = 0 as uint8_t;
                    pthread_cond_broadcast(&raw mut (*plc).cond);
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        pthread_mutex_unlock(
                            (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
                        );
                        return status;
                    }
                    (*plc).validts = ts + lcache_retention;
                }
                if !parent_inode.is_null() {
                    *parent_inode = (*plc).parent;
                }
                if !last_inode.is_null() {
                    *last_inode = (*plc).inode;
                }
                if !nleng.is_null() {
                    *nleng = (*plc).nleng;
                }
                memcpy(
                    name as *mut ::core::ffi::c_void,
                    &raw mut (*plc).name as *mut uint8_t as *const ::core::ffi::c_void,
                    (*plc).nleng as size_t,
                );
                memcpy(
                    attr as *mut ::core::ffi::c_void,
                    &raw mut (*plc).attr as *mut uint8_t as *const ::core::ffi::c_void,
                    ATTR_RECORD_SIZE as size_t,
                );
                pthread_mutex_unlock(
                    (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
                );
                return MFS_STATUS_OK as uint8_t;
            } else if (*plc).refresh_in_progress as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if minplc.is_null() || (*plc).validts < (*minplc).validts {
                    minplc = plc;
                }
            }
            qind = qind.wrapping_add(1);
        }
        if minplc.is_null() {
            pthread_mutex_unlock(
                (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
            );
            return fs_path_lookup(
                base_inode,
                rpleng,
                &raw mut rpath as *mut uint8_t,
                uid,
                gidcnt,
                gidtab,
                parent_inode,
                last_inode,
                nleng,
                name,
                attr,
            );
        }
        plc = minplc;
        (*plc).hash = hash;
        (*plc).refresh_in_progress = 1 as uint8_t;
        (*plc).base_inode = base_inode;
        (*plc).pleng = rpleng;
        memcpy(
            &raw mut (*plc).path as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut rpath as *mut uint8_t as *const ::core::ffi::c_void,
            rpleng as size_t,
        );
        (*plc).uid = uid;
        (*plc).gidcnt = gidcnt;
        memcpy(
            &raw mut (*plc).gidtab as *mut uint32_t as *mut ::core::ffi::c_void,
            gidtab as *const ::core::ffi::c_void,
            ::core::mem::size_of::<uint32_t>().wrapping_mul(gidcnt as size_t),
        );
        pthread_mutex_unlock((&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize));
        status = fs_path_lookup(
            base_inode,
            rpleng,
            &raw mut rpath as *mut uint8_t,
            uid,
            gidcnt,
            gidtab,
            &raw mut (*plc).parent,
            &raw mut (*plc).inode,
            &raw mut (*plc).nleng,
            &raw mut (*plc).name as *mut uint8_t,
            &raw mut (*plc).attr as *mut uint8_t,
        );
        ts = monotonic_seconds();
        pthread_mutex_lock((&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize));
        (*plc).refresh_in_progress = 0 as uint8_t;
        pthread_cond_broadcast(&raw mut (*plc).cond);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            (*plc).validts = ts;
            pthread_mutex_unlock(
                (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
            );
            return status;
        }
        (*plc).validts = ts + lcache_retention;
        if !parent_inode.is_null() {
            *parent_inode = (*plc).parent;
        }
        if !last_inode.is_null() {
            *last_inode = (*plc).inode;
        }
        if !nleng.is_null() {
            *nleng = (*plc).nleng;
        }
        memcpy(
            name as *mut ::core::ffi::c_void,
            &raw mut (*plc).name as *mut uint8_t as *const ::core::ffi::c_void,
            (*plc).nleng as size_t,
        );
        memcpy(
            attr as *mut ::core::ffi::c_void,
            &raw mut (*plc).attr as *mut uint8_t as *const ::core::ffi::c_void,
            ATTR_RECORD_SIZE as size_t,
        );
        pthread_mutex_unlock((&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize));
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcache_path_invalidate(
    mut base_inode: uint32_t,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
) {
    unsafe {
        let mut rpath: [uint8_t; 1024] = [0; 1024];
        let mut rpleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut hash: uint32_t = 0;
        let mut hind: uint32_t = 0;
        let mut qind: uint32_t = 0;
        let mut plc: *mut path_lookup_cache = ::core::ptr::null_mut::<path_lookup_cache>();
        let mut ts: ::core::ffi::c_double = 0.;
        ts = monotonic_seconds();
        status =
            lcache_path_normalize(pleng, path, &raw mut rpleng, &raw mut rpath as *mut uint8_t);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return;
        }
        hash = lcache_hash(base_inode, rpleng, &raw mut rpath as *mut uint8_t);
        hind = hash.wrapping_rem(LCACHE_HENTRIES as uint32_t);
        pthread_mutex_lock((&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize));
        qind = 0 as uint32_t;
        while qind < LCACHE_QENTRIES as uint32_t {
            plc = (&raw mut *(&raw mut lcache as *mut [path_lookup_cache; 8]).offset(hind as isize)
                as *mut path_lookup_cache)
                .offset(qind as isize);
            if (*plc).hash == hash
                && (*plc).base_inode == base_inode
                && (*plc).pleng == rpleng
                && memcmp(
                    &raw mut (*plc).path as *mut uint8_t as *const ::core::ffi::c_void,
                    &raw mut rpath as *mut uint8_t as *const ::core::ffi::c_void,
                    rpleng as size_t,
                ) == 0 as ::core::ffi::c_int
                && (*plc).refresh_in_progress as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*plc).validts = ts;
            }
            qind = qind.wrapping_add(1);
        }
        pthread_mutex_unlock((&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcache_inode_invalidate(mut inode: uint32_t) {
    unsafe {
        let mut hind: uint32_t = 0;
        let mut qind: uint32_t = 0;
        let mut plc: *mut path_lookup_cache = ::core::ptr::null_mut::<path_lookup_cache>();
        let mut ts: ::core::ffi::c_double = 0.;
        ts = monotonic_seconds();
        hind = 0 as uint32_t;
        while hind < LCACHE_HENTRIES as uint32_t {
            pthread_mutex_lock(
                (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
            );
            qind = 0 as uint32_t;
            while qind < LCACHE_QENTRIES as uint32_t {
                plc = (&raw mut *(&raw mut lcache as *mut [path_lookup_cache; 8])
                    .offset(hind as isize) as *mut path_lookup_cache)
                    .offset(qind as isize);
                if (*plc).inode == inode
                    && (*plc).refresh_in_progress as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    (*plc).validts = ts;
                }
                qind = qind.wrapping_add(1);
            }
            pthread_mutex_unlock(
                (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
            );
            hind = hind.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcache_term() {
    unsafe {
        let mut hind: uint32_t = 0;
        let mut qind: uint32_t = 0;
        hind = 0 as uint32_t;
        while hind < LCACHE_HENTRIES as uint32_t {
            pthread_mutex_destroy(
                (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
            );
            qind = 0 as uint32_t;
            while qind < LCACHE_QENTRIES as uint32_t {
                pthread_cond_destroy(
                    &raw mut (*(&raw mut *(&raw mut lcache as *mut [path_lookup_cache; 8])
                        .offset(hind as isize)
                        as *mut path_lookup_cache)
                        .offset(qind as isize))
                    .cond,
                );
                qind = qind.wrapping_add(1);
            }
            hind = hind.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcache_init(
    mut lc_retention: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hind: uint32_t = 0;
        let mut qind: uint32_t = 0;
        let mut ts: ::core::ffi::c_double = 0.;
        ts = monotonic_seconds();
        hind = 0 as uint32_t;
        while hind < LCACHE_HENTRIES as uint32_t {
            qind = 0 as uint32_t;
            while qind < LCACHE_QENTRIES as uint32_t {
                lcache[hind as usize][qind as usize].validts = ts;
                if pthread_cond_init(
                    &raw mut (*(&raw mut *(&raw mut lcache as *mut [path_lookup_cache; 8])
                        .offset(hind as isize)
                        as *mut path_lookup_cache)
                        .offset(qind as isize))
                    .cond,
                    ::core::ptr::null::<pthread_condattr_t>(),
                ) < 0 as ::core::ffi::c_int
                {
                    return -1 as ::core::ffi::c_int;
                }
                qind = qind.wrapping_add(1);
            }
            if pthread_mutex_init(
                (&raw mut lcache_lock as *mut pthread_mutex_t).offset(hind as isize),
                ::core::ptr::null::<pthread_mutexattr_t>(),
            ) < 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            hind = hind.wrapping_add(1);
        }
        lcache_retention = lc_retention;
        return 0 as ::core::ffi::c_int;
    }
}
