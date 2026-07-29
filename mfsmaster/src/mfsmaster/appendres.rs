unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _appendreservation {
    pub inode: uint32_t,
    pub vlength: uint64_t,
    pub next: *mut _appendreservation,
}
pub type appendreservation = _appendreservation;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const APPENDRES_HASHSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut appendreshash: [*mut appendreservation; 1024] =
    [::core::ptr::null_mut::<appendreservation>(); 1024];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn appendres_getvleng(mut inode: uint32_t) -> uint64_t {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut arptr: *mut appendreservation = ::core::ptr::null_mut::<appendreservation>();
        hash = inode.wrapping_rem(APPENDRES_HASHSIZE as uint32_t);
        arptr = appendreshash[hash as usize];
        while !arptr.is_null() {
            if (*arptr).inode == inode {
                return (*arptr).vlength;
            }
            arptr = (*arptr).next as *mut appendreservation;
        }
        return 0 as uint64_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn appendres_setvleng(mut inode: uint32_t, mut vlength: uint64_t) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut arptr: *mut appendreservation = ::core::ptr::null_mut::<appendreservation>();
        hash = inode.wrapping_rem(APPENDRES_HASHSIZE as uint32_t);
        arptr = appendreshash[hash as usize];
        while !arptr.is_null() {
            if (*arptr).inode == inode {
                (*arptr).vlength = vlength;
                return;
            }
            arptr = (*arptr).next as *mut appendreservation;
        }
        arptr = malloc(::core::mem::size_of::<appendreservation>()) as *mut appendreservation;
        (*arptr).inode = inode;
        (*arptr).vlength = vlength;
        (*arptr).next = appendreshash[hash as usize] as *mut _appendreservation;
        appendreshash[hash as usize] = arptr;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn appendres_setrleng(mut inode: uint32_t, mut rlength: uint64_t) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut arptr: *mut appendreservation = ::core::ptr::null_mut::<appendreservation>();
        let mut arpptr: *mut *mut appendreservation =
            ::core::ptr::null_mut::<*mut appendreservation>();
        hash = inode.wrapping_rem(APPENDRES_HASHSIZE as uint32_t);
        arpptr = (&raw mut appendreshash as *mut *mut appendreservation).offset(hash as isize);
        loop {
            arptr = *arpptr;
            if arptr.is_null() {
                break;
            }
            if (*arptr).inode == inode {
                if rlength >= (*arptr).vlength {
                    *arpptr = (*arptr).next as *mut appendreservation;
                    free(arptr as *mut ::core::ffi::c_void);
                }
                return;
            } else {
                arpptr = &raw mut (*arptr).next as *mut *mut appendreservation;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn appendres_clear(mut inode: uint32_t) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut arptr: *mut appendreservation = ::core::ptr::null_mut::<appendreservation>();
        let mut arpptr: *mut *mut appendreservation =
            ::core::ptr::null_mut::<*mut appendreservation>();
        hash = inode.wrapping_rem(APPENDRES_HASHSIZE as uint32_t);
        arpptr = (&raw mut appendreshash as *mut *mut appendreservation).offset(hash as isize);
        loop {
            arptr = *arpptr;
            if arptr.is_null() {
                break;
            }
            if (*arptr).inode == inode {
                *arpptr = (*arptr).next as *mut appendreservation;
                free(arptr as *mut ::core::ffi::c_void);
                return;
            } else {
                arpptr = &raw mut (*arptr).next as *mut *mut appendreservation;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn appendres_cleanall() {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut arptr: *mut appendreservation = ::core::ptr::null_mut::<appendreservation>();
        let mut arnptr: *mut appendreservation = ::core::ptr::null_mut::<appendreservation>();
        hash = 0 as uint32_t;
        while hash < APPENDRES_HASHSIZE as uint32_t {
            arptr = appendreshash[hash as usize];
            while !arptr.is_null() {
                arnptr = (*arptr).next as *mut appendreservation;
                free(arptr as *mut ::core::ffi::c_void);
                arptr = arnptr;
            }
            appendreshash[hash as usize] = ::core::ptr::null_mut::<appendreservation>();
            hash = hash.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn appendres_init() {
    unsafe {
        let mut hash: uint32_t = 0;
        hash = 0 as uint32_t;
        while hash < APPENDRES_HASHSIZE as uint32_t {
            appendreshash[hash as usize] = ::core::ptr::null_mut::<appendreservation>();
            hash = hash.wrapping_add(1);
        }
    }
}
