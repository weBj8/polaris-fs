use ::c2rust_bitfields;
pub type uint32_t = u32;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _datacache_entry {
    pub inode: uint32_t,
    #[bitfield(name = "cacheok", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "sessionid", ty = "::core::ffi::c_uint", bits = "1..=31")]
    pub cacheok_sessionid: [u8; 4],
    pub iprev: uint32_t,
    pub inext: uint32_t,
    pub lruprev: uint32_t,
    pub lrunext: uint32_t,
}
pub type datacache_entry = _datacache_entry;
pub const DCM_TAB_LENG: ::core::ffi::c_int = 500000 as ::core::ffi::c_int;
pub const DCM_INODEHASH_LENG: ::core::ffi::c_int = DCM_TAB_LENG / 2 as ::core::ffi::c_int;
pub const DCM_NIL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
static mut dcm_tab: [datacache_entry; 500000] = [datacache_entry {
    inode: 0,
    cacheok_sessionid: [0; 4],
    iprev: 0,
    inext: 0,
    lruprev: 0,
    lrunext: 0,
}; 500000];
static mut dcm_inodehash: [uint32_t; 250000] = [0; 250000];
static mut dcm_lru_first: uint32_t = 0;
static mut dcm_lru_last: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn dcm_open(
    mut inode: uint32_t,
    mut sessionid: uint32_t,
) -> ::core::ffi::c_int {
    let mut ih: uint32_t = inode
        .wrapping_mul(0x4a4fecd1 as uint32_t)
        .wrapping_rem(DCM_INODEHASH_LENG as uint32_t);
    let mut p: uint32_t = 0;
    let mut pp: uint32_t = 0;
    let mut np: uint32_t = 0;
    p = dcm_inodehash[ih as usize];
    while p < DCM_TAB_LENG as uint32_t {
        if dcm_tab[p as usize].inode == inode
            && dcm_tab[p as usize].sessionid() as uint32_t == sessionid
        {
            if dcm_lru_last != p {
                pp = dcm_tab[p as usize].lruprev;
                np = dcm_tab[p as usize].lrunext;
                if pp < DCM_TAB_LENG as uint32_t {
                    dcm_tab[pp as usize].lrunext = np;
                } else {
                    dcm_lru_first = np;
                }
                if np < DCM_TAB_LENG as uint32_t {
                    dcm_tab[np as usize].lruprev = pp;
                } else {
                    dcm_lru_last = pp;
                }
                dcm_tab[p as usize].lruprev = dcm_lru_last;
                dcm_tab[p as usize].lrunext = DCM_NIL as uint32_t;
                dcm_tab[dcm_lru_last as usize].lrunext = p;
                dcm_lru_last = p;
            }
            return dcm_tab[p as usize].cacheok() as ::core::ffi::c_int;
        }
        p = dcm_tab[p as usize].inext;
    }
    p = dcm_lru_first;
    pp = dcm_tab[p as usize].lruprev;
    np = dcm_tab[p as usize].lrunext;
    if pp < DCM_TAB_LENG as uint32_t {
        dcm_tab[pp as usize].lrunext = np;
    } else {
        dcm_lru_first = np;
    }
    if np < DCM_TAB_LENG as uint32_t {
        dcm_tab[np as usize].lruprev = pp;
    } else {
        dcm_lru_last = pp;
    }
    dcm_tab[p as usize].lruprev = dcm_lru_last;
    dcm_tab[p as usize].lrunext = DCM_NIL as uint32_t;
    dcm_tab[dcm_lru_last as usize].lrunext = p;
    dcm_lru_last = p;
    if dcm_tab[p as usize].inode > 0 as uint32_t {
        ih = dcm_tab[p as usize]
            .inode
            .wrapping_mul(0x4a4fecd1 as uint32_t)
            .wrapping_rem(DCM_INODEHASH_LENG as uint32_t);
        pp = dcm_tab[p as usize].iprev;
        np = dcm_tab[p as usize].inext;
        if pp < DCM_TAB_LENG as uint32_t {
            dcm_tab[pp as usize].inext = np;
        } else {
            dcm_inodehash[ih as usize] = np;
        }
        if np < DCM_TAB_LENG as uint32_t {
            dcm_tab[np as usize].iprev = pp;
        }
    }
    dcm_tab[p as usize].inode = inode;
    dcm_tab[p as usize].set_cacheok(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    dcm_tab[p as usize].set_sessionid(sessionid as ::core::ffi::c_uint as ::core::ffi::c_uint);
    ih = inode
        .wrapping_mul(0x4a4fecd1 as uint32_t)
        .wrapping_rem(DCM_INODEHASH_LENG as uint32_t);
    np = dcm_inodehash[ih as usize];
    dcm_tab[p as usize].inext = np;
    dcm_tab[p as usize].iprev = DCM_NIL as uint32_t;
    dcm_inodehash[ih as usize] = p;
    if np < DCM_TAB_LENG as uint32_t {
        dcm_tab[np as usize].iprev = p;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dcm_access(mut inode: uint32_t, mut sessionid: uint32_t) {
    let mut ih: uint32_t = inode
        .wrapping_mul(0x4a4fecd1 as uint32_t)
        .wrapping_rem(DCM_INODEHASH_LENG as uint32_t);
    let mut p: uint32_t = 0;
    let mut pp: uint32_t = 0;
    let mut np: uint32_t = 0;
    p = dcm_inodehash[ih as usize];
    while p < DCM_TAB_LENG as uint32_t {
        if dcm_tab[p as usize].inode == inode
            && dcm_tab[p as usize].sessionid() as uint32_t == sessionid
        {
            if dcm_lru_last != p {
                pp = dcm_tab[p as usize].lruprev;
                np = dcm_tab[p as usize].lrunext;
                if pp < DCM_TAB_LENG as uint32_t {
                    dcm_tab[pp as usize].lrunext = np;
                } else {
                    dcm_lru_first = np;
                }
                if np < DCM_TAB_LENG as uint32_t {
                    dcm_tab[np as usize].lruprev = pp;
                } else {
                    dcm_lru_last = pp;
                }
                dcm_tab[p as usize].lruprev = dcm_lru_last;
                dcm_tab[p as usize].lrunext = DCM_NIL as uint32_t;
                dcm_tab[dcm_lru_last as usize].lrunext = p;
                dcm_lru_last = p;
            }
            dcm_tab[p as usize].set_cacheok(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            return;
        }
        p = dcm_tab[p as usize].inext;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dcm_modify(mut inode: uint32_t, mut sessionid: uint32_t) {
    let mut ih: uint32_t = inode
        .wrapping_mul(0x4a4fecd1 as uint32_t)
        .wrapping_rem(DCM_INODEHASH_LENG as uint32_t);
    let mut p: uint32_t = 0;
    let mut pp: uint32_t = 0;
    let mut np: uint32_t = 0;
    p = dcm_inodehash[ih as usize];
    while p < DCM_TAB_LENG as uint32_t {
        if dcm_tab[p as usize].inode == inode
            && dcm_tab[p as usize].sessionid() as uint32_t != sessionid
        {
            if p != dcm_lru_first {
                pp = dcm_tab[p as usize].lruprev;
                np = dcm_tab[p as usize].lrunext;
                if pp < DCM_TAB_LENG as uint32_t {
                    dcm_tab[pp as usize].lrunext = np;
                } else {
                    dcm_lru_first = np;
                }
                if np < DCM_TAB_LENG as uint32_t {
                    dcm_tab[np as usize].lruprev = pp;
                } else {
                    dcm_lru_last = pp;
                }
                dcm_tab[p as usize].lruprev = DCM_NIL as uint32_t;
                dcm_tab[p as usize].lrunext = dcm_lru_first;
                dcm_tab[dcm_lru_first as usize].lruprev = p;
                dcm_lru_first = p;
            }
            pp = dcm_tab[p as usize].iprev;
            np = dcm_tab[p as usize].inext;
            if pp < DCM_TAB_LENG as uint32_t {
                dcm_tab[pp as usize].inext = np;
            } else {
                dcm_inodehash[ih as usize] = np;
            }
            if np < DCM_TAB_LENG as uint32_t {
                dcm_tab[np as usize].iprev = pp;
            }
            dcm_tab[p as usize].inode = 0 as uint32_t;
            dcm_tab[p as usize].inext = DCM_NIL as uint32_t;
            dcm_tab[p as usize].iprev = DCM_NIL as uint32_t;
            p = np;
        } else {
            if dcm_tab[p as usize].inode == inode
                && dcm_tab[p as usize].sessionid() as uint32_t == sessionid
            {
                if dcm_lru_last != p {
                    pp = dcm_tab[p as usize].lruprev;
                    np = dcm_tab[p as usize].lrunext;
                    if pp < DCM_TAB_LENG as uint32_t {
                        dcm_tab[pp as usize].lrunext = np;
                    } else {
                        dcm_lru_first = np;
                    }
                    if np < DCM_TAB_LENG as uint32_t {
                        dcm_tab[np as usize].lruprev = pp;
                    } else {
                        dcm_lru_last = pp;
                    }
                    dcm_tab[p as usize].lruprev = dcm_lru_last;
                    dcm_tab[p as usize].lrunext = DCM_NIL as uint32_t;
                    dcm_tab[dcm_lru_last as usize].lrunext = p;
                    dcm_lru_last = p;
                }
                dcm_tab[p as usize].set_cacheok(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            p = dcm_tab[p as usize].inext;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dcm_init() -> ::core::ffi::c_int {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < DCM_INODEHASH_LENG as uint32_t {
        dcm_inodehash[i as usize] = DCM_NIL as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as uint32_t;
    while i < DCM_TAB_LENG as uint32_t {
        dcm_tab[i as usize].inode = 0 as uint32_t;
        dcm_tab[i as usize].set_sessionid(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        dcm_tab[i as usize].iprev = DCM_NIL as uint32_t;
        dcm_tab[i as usize].inext = DCM_NIL as uint32_t;
        dcm_tab[i as usize].lruprev = i.wrapping_sub(1 as uint32_t);
        dcm_tab[i as usize].lrunext = i.wrapping_add(1 as uint32_t);
        i = i.wrapping_add(1);
    }
    dcm_tab[0 as usize].lruprev = DCM_NIL as uint32_t;
    dcm_lru_first = 0 as uint32_t;
    dcm_tab[(DCM_TAB_LENG - 1 as ::core::ffi::c_int) as usize].lrunext = DCM_NIL as uint32_t;
    dcm_lru_last = (DCM_TAB_LENG - 1 as ::core::ffi::c_int) as uint32_t;
    return 0 as ::core::ffi::c_int;
}
