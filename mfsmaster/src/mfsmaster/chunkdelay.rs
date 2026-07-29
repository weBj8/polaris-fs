extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn monotonic_seconds() -> ::core::ffi::c_double;
    fn chunk_do_extra_job(chunkid: uint64_t);
    fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_msectime_register_fname(
        mseconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk_prot {
    pub chunkid: uint64_t,
    pub ts: ::core::ffi::c_double,
    pub next: *mut chunk_prot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunk_group {
    pub chunkidtab: [uint64_t; 128],
    pub count: uint32_t,
    pub next: *mut _chunk_group,
}
pub type chunk_group = _chunk_group;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const HASH_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut chunk_prot_hashtab: [*mut chunk_prot; 4096] =
    [::core::ptr::null_mut::<chunk_prot>(); 4096];
static mut ProtectionDelay: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn chunk_delay_protect(mut chunkid: uint64_t) {
    let mut hash: uint32_t = (chunkid ^ chunkid >> 16 as ::core::ffi::c_int)
        .wrapping_rem(HASH_SIZE as uint64_t) as uint32_t;
    let mut cp: *mut chunk_prot = ::core::ptr::null_mut::<chunk_prot>();
    cp = chunk_prot_hashtab[hash as usize];
    while !cp.is_null() {
        if (*cp).chunkid == chunkid {
            (*cp).ts = monotonic_seconds();
            return;
        }
        cp = (*cp).next as *mut chunk_prot;
    }
    cp = malloc(::core::mem::size_of::<chunk_prot>()) as *mut chunk_prot;
    (*cp).chunkid = chunkid;
    (*cp).ts = monotonic_seconds();
    (*cp).next = chunk_prot_hashtab[hash as usize] as *mut chunk_prot;
    chunk_prot_hashtab[hash as usize] = cp;
}
#[no_mangle]
pub unsafe extern "C" fn chunk_delay_is_protected(mut chunkid: uint64_t) -> uint8_t {
    let mut hash: uint32_t = (chunkid ^ chunkid >> 16 as ::core::ffi::c_int)
        .wrapping_rem(HASH_SIZE as uint64_t) as uint32_t;
    let mut cp: *mut chunk_prot = ::core::ptr::null_mut::<chunk_prot>();
    let mut cpp: *mut *mut chunk_prot = ::core::ptr::null_mut::<*mut chunk_prot>();
    let mut ts: ::core::ffi::c_double = 0.;
    ts = monotonic_seconds();
    cpp = (&raw mut chunk_prot_hashtab as *mut *mut chunk_prot).offset(hash as isize);
    loop {
        cp = *cpp;
        if cp.is_null() {
            break;
        }
        if (*cp).chunkid == chunkid {
            if ((*cp).ts + ProtectionDelay as ::core::ffi::c_double) < ts {
                *cpp = (*cp).next as *mut chunk_prot;
                free(cp as *mut ::core::ffi::c_void);
                return 0 as uint8_t;
            } else {
                return 1 as uint8_t;
            }
        }
        cpp = &raw mut (*cp).next as *mut *mut chunk_prot;
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn chunk_delay_remove_old() {
    static mut current_index: uint32_t = 0 as uint32_t;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut ts: ::core::ffi::c_double = 0.;
    let mut cghead: chunk_group = chunk_group {
        chunkidtab: [0; 128],
        count: 0,
        next: ::core::ptr::null_mut::<_chunk_group>(),
    };
    let mut cgcurr: *mut chunk_group = ::core::ptr::null_mut::<chunk_group>();
    let mut cgnext: *mut chunk_group = ::core::ptr::null_mut::<chunk_group>();
    let mut cp: *mut chunk_prot = ::core::ptr::null_mut::<chunk_prot>();
    let mut cpp: *mut *mut chunk_prot = ::core::ptr::null_mut::<*mut chunk_prot>();
    ts = monotonic_seconds();
    i = 0 as uint32_t;
    while i < 10 as uint32_t {
        cgcurr = &raw mut cghead;
        (*cgcurr).count = 0 as uint32_t;
        (*cgcurr).next = ::core::ptr::null_mut::<_chunk_group>();
        cpp = (&raw mut chunk_prot_hashtab as *mut *mut chunk_prot).offset(current_index as isize);
        loop {
            cp = *cpp;
            if cp.is_null() {
                break;
            }
            if ((*cp).ts + ProtectionDelay as ::core::ffi::c_double) < ts {
                if (*cgcurr).count >= 128 as uint32_t {
                    (*cgcurr).next =
                        malloc(::core::mem::size_of::<chunk_group>()) as *mut _chunk_group;
                    cgcurr = (*cgcurr).next as *mut chunk_group;
                    (*cgcurr).count = 0 as uint32_t;
                    (*cgcurr).next = ::core::ptr::null_mut::<_chunk_group>();
                }
                (*cgcurr).chunkidtab[(*cgcurr).count as usize] = (*cp).chunkid;
                (*cgcurr).count = (*cgcurr).count.wrapping_add(1);
                *cpp = (*cp).next as *mut chunk_prot;
                free(cp as *mut ::core::ffi::c_void);
            } else {
                cpp = &raw mut (*cp).next as *mut *mut chunk_prot;
            }
        }
        cgcurr = &raw mut cghead;
        while !cgcurr.is_null() {
            j = 0 as uint32_t;
            while j < (*cgcurr).count {
                chunk_do_extra_job((*cgcurr).chunkidtab[j as usize]);
                j = j.wrapping_add(1);
            }
            cgcurr = (*cgcurr).next as *mut chunk_group;
        }
        cgcurr = cghead.next as *mut chunk_group;
        while !cgcurr.is_null() {
            cgnext = (*cgcurr).next as *mut chunk_group;
            free(cgcurr as *mut ::core::ffi::c_void);
            cgcurr = cgnext;
        }
        cghead.next = ::core::ptr::null_mut::<_chunk_group>();
        current_index = current_index.wrapping_add(1);
        if current_index >= HASH_SIZE as uint32_t {
            current_index = 0 as uint32_t;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn chunk_delay_reload() {
    ProtectionDelay = cfg_getuint32(
        b"CHUNK_PROTECTION_SECONDS\0".as_ptr() as *const ::core::ffi::c_char,
        15 as uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn chunk_delay_init() {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < HASH_SIZE as uint32_t {
        chunk_prot_hashtab[i as usize] = ::core::ptr::null_mut::<chunk_prot>();
        i = i.wrapping_add(1);
    }
    chunk_delay_reload();
    main_msectime_register_fname(
        10 as uint32_t,
        0 as uint32_t,
        Some(chunk_delay_remove_old as unsafe extern "C" fn() -> ()),
        b"chunk_delay_remove_old\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_reload_register_fname(
        Some(chunk_delay_reload as unsafe extern "C" fn() -> ()),
        b"chunk_delay_reload\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
