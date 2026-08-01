unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _busy_chunk {
    pub packet: *mut ::core::ffi::c_void,
    pub chunkid: uint64_t,
    pub next: *mut _busy_chunk,
    pub prev: *mut *mut _busy_chunk,
}
pub type busy_chunk = _busy_chunk;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const HASHSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut bchashmap: [*mut busy_chunk; 1024] = [::core::ptr::null_mut::<busy_chunk>(); 1024];
#[inline]
unsafe extern "C" fn busychunk_hashfn(mut chunkid: uint64_t) -> uint32_t {
    return chunkid.wrapping_rem(HASHSIZE as uint64_t) as uint32_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn busychunk_start(
    mut packet: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut bc: *mut busy_chunk = ::core::ptr::null_mut::<busy_chunk>();
        let mut hash: uint32_t = 0;
        chunkid &= 0xffffffffffffff as uint64_t;
        hash = busychunk_hashfn(chunkid);
        bc = malloc(::core::mem::size_of::<busy_chunk>()) as *mut busy_chunk;
        (*bc).packet = packet;
        (*bc).chunkid = chunkid;
        (*bc).next = bchashmap[hash as usize] as *mut _busy_chunk;
        (*bc).prev = (&raw mut bchashmap as *mut *mut busy_chunk).offset(hash as isize)
            as *mut *mut _busy_chunk;
        bchashmap[hash as usize] = bc;
        if !(*bc).next.is_null() {
            (*(*bc).next).prev = &raw mut (*bc).next;
        }
        return bc as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn busychunk_end(
    mut vbc: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut bc: *mut busy_chunk = vbc as *mut busy_chunk;
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        packet = (*bc).packet;
        *(*bc).prev = (*bc).next;
        if !(*bc).next.is_null() {
            (*(*bc).next).prev = (*bc).prev;
        }
        free(bc as *mut ::core::ffi::c_void);
        return packet;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn busychunk_isbusy(mut chunkid: uint64_t) -> uint8_t {
    unsafe {
        let mut bc: *mut busy_chunk = ::core::ptr::null_mut::<busy_chunk>();
        let mut hash: uint32_t = 0;
        chunkid &= 0xffffffffffffff as uint64_t;
        hash = busychunk_hashfn(chunkid);
        bc = bchashmap[hash as usize];
        while !bc.is_null() {
            if (*bc).chunkid == chunkid {
                return 1 as uint8_t;
            }
            bc = (*bc).next as *mut busy_chunk;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn busychunk_init() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < HASHSIZE as uint32_t {
            bchashmap[i as usize] = ::core::ptr::null_mut::<busy_chunk>();
            i = i.wrapping_add(1);
        }
    }
}
