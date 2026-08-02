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
pub type uint8_t = u8;
pub type uint32_t = u32;
#[repr(C)]
pub struct _path_lookup_cache {
    pub hash: uint32_t,
    pub refresh_in_progress: uint8_t,
    // C: pthread_cond_t cond (waited on with lcache_lock[hind]).
    pub cond: std::sync::Condvar,
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
static mut lcache: [[path_lookup_cache; 8]; 64] = [const {
    [const {
        path_lookup_cache {
            hash: 0,
            refresh_in_progress: 0,
            cond: std::sync::Condvar::new(),
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
        }
    }; 8]
}; 64];
// Per-bucket locks. C: pthread_mutex_t lcache_lock[LCACHE_HENTRIES], manually
// locked/unlocked across complex flow (unlocked around fs_path_lookup;
// cond_wait releases/reacquires the same bucket mutex). std Mutex is
// RAII-only, so emulate pthread-style manual lock/unlock by stashing the
// guard in a thread_local slot together with the bucket index: lock() parks
// the guard, unlock() drops it (same guard-slot pattern as
// plfsclient/readdata.rs INODE_LOCK/IND_LOCK_GUARD).
// INVARIANT: every lock/unlock/wait pair runs on the same thread and no
// thread ever holds two bucket locks at once (verified against
// mfsioint_lookupcache.c: lcache_inode_invalidate releases hind before
// hind+1; lookup/invalidate hold one bucket each); index mismatch panics,
// catching misuse pthread would UB on. Poisoning is ignored (into_inner)
// for pthread parity.
// ponytail: guard-slot emulates pthread_mutex_t; upgrade path is a full RAII
// restructure of every lock region (large diff, separate wave).
static LCACHE_LOCK: [std::sync::Mutex<()>; 64] = [const { std::sync::Mutex::new(()) }; 64];
thread_local! {
    static LCACHE_LOCK_GUARD: std::cell::RefCell<
        Option<(usize, std::sync::MutexGuard<'static, ()>)>,
    > = const { std::cell::RefCell::new(None) };
}
fn lcache_bucket_lock(hind: usize) {
    let guard = LCACHE_LOCK[hind].lock().unwrap_or_else(|e| e.into_inner());
    LCACHE_LOCK_GUARD.with(|slot| {
        let mut slot = slot.borrow_mut();
        assert!(slot.is_none(), "lcache_bucket_lock: guard already held");
        *slot = Some((hind, guard));
    });
}
fn lcache_bucket_unlock(hind: usize) {
    let entry = LCACHE_LOCK_GUARD
        .with(|slot| slot.borrow_mut().take())
        .expect("lcache_bucket_unlock: no guard held on this thread");
    assert!(
        entry.0 == hind,
        "lcache_bucket_unlock: bucket index mismatch"
    );
    drop(entry.1);
}
// Caller must hold LCACHE_LOCK[hind]; wait releases and reacquires it,
// exactly like pthread_cond_wait(&plc->cond, lcache_lock+hind). C waits in
// a while (refresh_in_progress) predicate loop, so spurious wakeups are
// already handled there.
unsafe fn lcache_cond_wait(cond: *const std::sync::Condvar, hind: usize) {
    unsafe {
        let entry = LCACHE_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("lcache_cond_wait: no guard held on this thread");
        assert!(entry.0 == hind, "lcache_cond_wait: bucket index mismatch");
        // SAFETY: cond is &(*plc).cond inside the static lcache array — it
        // outlives every wait.
        let guard = (*cond).wait(entry.1).unwrap_or_else(|e| e.into_inner());
        LCACHE_LOCK_GUARD.with(|slot| {
            *slot.borrow_mut() = Some((entry.0, guard));
        });
    }
}
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
        lcache_bucket_lock(hind as usize);
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
                    lcache_cond_wait(&raw const (*plc).cond, hind as usize);
                }
                if (*plc).validts <= ts {
                    (*plc).refresh_in_progress = 1 as uint8_t;
                    lcache_bucket_unlock(hind as usize);
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
                    lcache_bucket_lock(hind as usize);
                    (*plc).refresh_in_progress = 0 as uint8_t;
                    (*plc).cond.notify_all();
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        lcache_bucket_unlock(hind as usize);
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
                lcache_bucket_unlock(hind as usize);
                return MFS_STATUS_OK as uint8_t;
            } else if (*plc).refresh_in_progress as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if minplc.is_null() || (*plc).validts < (*minplc).validts {
                    minplc = plc;
                }
            }
            qind = qind.wrapping_add(1);
        }
        if minplc.is_null() {
            lcache_bucket_unlock(hind as usize);
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
        lcache_bucket_unlock(hind as usize);
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
        lcache_bucket_lock(hind as usize);
        (*plc).refresh_in_progress = 0 as uint8_t;
        (*plc).cond.notify_all();
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            (*plc).validts = ts;
            lcache_bucket_unlock(hind as usize);
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
        lcache_bucket_unlock(hind as usize);
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
        lcache_bucket_lock(hind as usize);
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
        lcache_bucket_unlock(hind as usize);
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
            lcache_bucket_lock(hind as usize);
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
            lcache_bucket_unlock(hind as usize);
            hind = hind.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lcache_term() {
    // C: pthread_mutex_destroy/pthread_cond_destroy loops. std Mutex/Condvar
    // need no destroy; statics live for process lifetime. Nothing to do.
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
                // C: pthread_cond_init(&lcache[hind][qind].cond) — std
                // Condvar is const-initialized in the static, never fails.
                qind = qind.wrapping_add(1);
            }
            // C: pthread_mutex_init(lcache_lock+hind) — std Mutex is
            // const-initialized in LCACHE_LOCK, never fails.
            hind = hind.wrapping_add(1);
        }
        lcache_retention = lc_retention;
        return 0 as ::core::ffi::c_int;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // Reference tests against mfsioint_lookupcache.c semantics.
    fn normalize(path: &[u8]) -> (uint8_t, Vec<u8>) {
        let mut rpath = [0u8; MFS_PATH_MAX as usize];
        let mut rpleng: uint32_t = 0;
        let status = unsafe {
            lcache_path_normalize(
                path.len() as uint32_t,
                path.as_ptr(),
                &raw mut rpleng,
                rpath.as_mut_ptr(),
            )
        };
        (status, rpath[..rpleng as usize].to_vec())
    }

    #[test]
    fn normalize_plain() {
        assert_eq!(
            normalize(b"a/b/c\0"),
            (MFS_STATUS_OK as uint8_t, b"a/b/c".to_vec())
        );
    }

    #[test]
    fn normalize_dot() {
        // '.' component is dropped (only when followed by '/').
        assert_eq!(
            normalize(b"a/./b\0"),
            (MFS_STATUS_OK as uint8_t, b"a/b".to_vec())
        );
    }

    #[test]
    fn normalize_dotdot() {
        // "a/b/../c": rleng backs up past "b/" leaving "a/", then "c".
        assert_eq!(
            normalize(b"a/b/../c\0"),
            (MFS_STATUS_OK as uint8_t, b"a/c".to_vec())
        );
    }

    #[test]
    fn normalize_dotdot_above_root() {
        // '../' with rleng<3 escapes the root -> EINVAL.
        assert_eq!(normalize(b"../a\0").0, MFS_ERROR_EINVAL as uint8_t);
        // 'a/../../b': second '..' hits rleng=2 < 3 -> EINVAL.
        assert_eq!(normalize(b"a/../../b\0").0, MFS_ERROR_EINVAL as uint8_t);
    }

    #[test]
    fn normalize_name_too_long() {
        // Single component of 256 bytes exceeds MFS_NAME_MAX=255.
        let mut p = vec![b'a'; 256];
        p.push(0);
        assert_eq!(normalize(&p).0, MFS_ERROR_ENAMETOOLONG as uint8_t);
    }

    #[test]
    fn normalize_path_too_long() {
        // "ab/" * 400 = 1200 bytes exceeds MFS_PATH_MAX=1024.
        let mut p = b"ab/".repeat(400);
        p.push(0);
        assert_eq!(normalize(&p).0, MFS_ERROR_ENAMETOOLONG as uint8_t);
    }

    #[test]
    fn hash_reference() {
        // C: hash = base_inode; hash = hash*33 + path[i] (u32 wrap).
        let h = unsafe { lcache_hash(1, 3, b"abc".as_ptr() as *mut uint8_t) };
        assert_eq!(h, ((1u32.wrapping_mul(33) + 97) * 33 + 98) * 33 + 99);
        // Wrap parity: base_inode = u32::MAX.
        let h = unsafe { lcache_hash(u32::MAX, 1, b"\x01".as_ptr() as *mut uint8_t) };
        assert_eq!(h, u32::MAX.wrapping_mul(33).wrapping_add(1));
    }
}
