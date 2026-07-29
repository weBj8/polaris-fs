pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
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
pub type uint32_t = u32;
pub type itnode = _itnode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _itnode {
    pub from: uint32_t,
    pub to: uint32_t,
    pub id: uint32_t,
    pub left: *mut _itnode,
    pub right: *mut _itnode,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn itree_free(mut n: *mut itnode) {
    unsafe {
        if !n.is_null() {
            itree_free((*n).left as *mut itnode);
            itree_free((*n).right as *mut itnode);
            free(n as *mut ::core::ffi::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn itree_remove(mut p: *mut *mut itnode) {
    unsafe {
        let mut n: *mut itnode = *p;
        let mut nit: *mut itnode = ::core::ptr::null_mut::<itnode>();
        let mut nptr: *mut *mut itnode = ::core::ptr::null_mut::<*mut itnode>();
        let mut l: uint32_t = 0;
        let mut r: uint32_t = 0;
        if (*n).left.is_null() {
            *p = (*n).right as *mut itnode;
            free(n as *mut ::core::ffi::c_void);
        } else if (*n).right.is_null() {
            *p = (*n).left as *mut itnode;
            free(n as *mut ::core::ffi::c_void);
        } else {
            r = 0 as uint32_t;
            l = r;
            nit = (*(*n).left).right as *mut itnode;
            while !nit.is_null() {
                l = l.wrapping_add(1);
                nit = (*nit).right as *mut itnode;
            }
            nit = (*(*n).right).left as *mut itnode;
            while !nit.is_null() {
                r = r.wrapping_add(1);
                nit = (*nit).left as *mut itnode;
            }
            if l == r {
                l = l.wrapping_add(((*n).from ^ (*n).to) & 1 as uint32_t);
            }
            if r > l {
                nptr = &raw mut (*n).right as *mut *mut itnode;
                loop {
                    nit = *nptr;
                    if !(!nit.is_null() && !(*nit).left.is_null()) {
                        break;
                    }
                    nptr = &raw mut (*nit).left as *mut *mut itnode;
                }
                *nptr = (*nit).right as *mut itnode;
            } else {
                nptr = &raw mut (*n).left as *mut *mut itnode;
                loop {
                    nit = *nptr;
                    if !(!nit.is_null() && !(*nit).right.is_null()) {
                        break;
                    }
                    nptr = &raw mut (*nit).right as *mut *mut itnode;
                }
                *nptr = (*nit).left as *mut itnode;
            }
            (*nit).left = (*n).left;
            (*nit).right = (*n).right;
            *p = nit;
        };
    }
}
#[inline]
unsafe extern "C" fn itree_delete(mut p: *mut *mut itnode, mut f: uint32_t, mut t: uint32_t) {
    unsafe {
        let mut n: *mut itnode = *p;
        if !n.is_null() {
            if t < (*n).from {
                itree_delete(&raw mut (*n).left, f, t);
            } else if f > (*n).to {
                itree_delete(&raw mut (*n).right, f, t);
            } else if f <= (*n).from && t >= (*n).to {
                if f < (*n).from {
                    itree_delete(&raw mut (*n).left, f, (*n).from.wrapping_sub(1 as uint32_t));
                }
                if t > (*n).to {
                    itree_delete(&raw mut (*n).right, (*n).to.wrapping_add(1 as uint32_t), t);
                }
                itree_remove(p);
            } else if f >= (*n).from && t <= (*n).to {
                if f == (*n).from {
                    (*n).from = t.wrapping_add(1 as uint32_t);
                } else if t == (*n).to {
                    (*n).to = f.wrapping_sub(1 as uint32_t);
                } else if (t ^ f) & 1 as uint32_t != 0 {
                    itree_add(
                        &raw mut (*n).right,
                        t.wrapping_add(1 as uint32_t),
                        (*n).to,
                        (*n).id,
                    );
                    (*n).to = f.wrapping_sub(1 as uint32_t);
                } else {
                    itree_add(
                        &raw mut (*n).left,
                        (*n).from,
                        f.wrapping_sub(1 as uint32_t),
                        (*n).id,
                    );
                    (*n).from = t.wrapping_add(1 as uint32_t);
                }
            } else if f < (*n).from {
                (*n).from = t.wrapping_add(1 as uint32_t);
                itree_delete(&raw mut (*n).left, f, t);
            } else if t > (*n).to {
                (*n).to = f.wrapping_sub(1 as uint32_t);
                itree_delete(&raw mut (*n).right, f, t);
            }
        }
    }
}
#[inline]
unsafe extern "C" fn itree_add(
    mut p: *mut *mut itnode,
    mut f: uint32_t,
    mut t: uint32_t,
    mut id: uint32_t,
) {
    unsafe {
        let mut n: *mut itnode = *p;
        if !n.is_null() {
            if t < (*n).from {
                itree_add(&raw mut (*n).left, f, t, id);
            } else if f > (*n).to {
                itree_add(&raw mut (*n).right, f, t, id);
            } else if f <= (*n).from && t >= (*n).to {
                if f < (*n).from {
                    itree_delete(&raw mut (*n).left, f, (*n).from.wrapping_sub(1 as uint32_t));
                }
                if t > (*n).to {
                    itree_delete(&raw mut (*n).right, (*n).to.wrapping_add(1 as uint32_t), t);
                }
                (*n).from = f;
                (*n).to = t;
                (*n).id = id;
            } else if f >= (*n).from && t <= (*n).to {
                if f > (*n).from {
                    itree_add(
                        &raw mut (*n).left,
                        (*n).from,
                        f.wrapping_sub(1 as uint32_t),
                        (*n).id,
                    );
                }
                if t < (*n).to {
                    itree_add(
                        &raw mut (*n).right,
                        t.wrapping_add(1 as uint32_t),
                        (*n).to,
                        (*n).id,
                    );
                }
                (*n).from = f;
                (*n).to = t;
                (*n).id = id;
            } else if f < (*n).from {
                (*n).from = t.wrapping_add(1 as uint32_t);
                itree_add(&raw mut (*n).left, f, t, id);
            } else if t > (*n).to {
                (*n).to = f.wrapping_sub(1 as uint32_t);
                itree_add(&raw mut (*n).right, f, t, id);
            }
        } else {
            n = malloc(::core::mem::size_of::<itnode>()) as *mut itnode;
            *p = n;
            if n.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/itree.c\0".as_ptr() as *const ::core::ffi::c_char,
                    161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/itree.c\0".as_ptr() as *const ::core::ffi::c_char,
                    161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if n
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut itnode
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/itree.c\0".as_ptr() as *const ::core::ffi::c_char,
                    161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"n\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/itree.c\0".as_ptr() as *const ::core::ffi::c_char,
                    161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"n\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*n).from = f;
            (*n).to = t;
            (*n).id = id;
            (*n).left = ::core::ptr::null_mut::<_itnode>();
            (*n).right = ::core::ptr::null_mut::<_itnode>();
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_tolist(
    mut n: *mut itnode,
    mut tail: *mut *mut itnode,
) -> *mut *mut itnode {
    unsafe {
        if !n.is_null() {
            tail = itree_tolist((*n).left as *mut itnode, tail);
            (*n).left = ::core::ptr::null_mut::<_itnode>();
            *tail = n;
            tail = itree_tolist((*n).right as *mut itnode, &raw mut (*n).left);
            (*n).right = ::core::ptr::null_mut::<_itnode>();
        }
        return tail;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_simplify(mut n: *mut itnode) {
    unsafe {
        let mut f: *mut itnode = ::core::ptr::null_mut::<itnode>();
        while !n.is_null() && !(*n).left.is_null() {
            if (*n).id == (*(*n).left).id
                && (*n).to.wrapping_add(1 as uint32_t) == (*(*n).left).from
            {
                (*n).to = (*(*n).left).to;
                f = (*n).left as *mut itnode;
                (*n).left = (*(*n).left).left;
                free(f as *mut ::core::ffi::c_void);
            } else {
                n = (*n).left as *mut itnode;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_totree(mut l: *mut itnode, mut p: *mut *mut itnode) {
    unsafe {
        let mut m: *mut *mut itnode = ::core::ptr::null_mut::<*mut itnode>();
        let mut i: *mut itnode = ::core::ptr::null_mut::<itnode>();
        if !l.is_null() {
            i = l;
            m = &raw mut l;
            while !i.is_null() && !(*i).left.is_null() {
                m = &raw mut (**m).left as *mut *mut itnode;
                i = (*(*i).left).left as *mut itnode;
            }
            i = *m;
            *p = i;
            *m = ::core::ptr::null_mut::<itnode>();
            itree_totree((*i).left as *mut itnode, &raw mut (*i).right);
            itree_totree(l, &raw mut (*i).left);
        } else {
            *p = ::core::ptr::null_mut::<itnode>();
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_rebalance(
    mut o: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut head: *mut itnode = ::core::ptr::null_mut::<itnode>();
        let mut root: *mut itnode = o as *mut itnode;
        head = ::core::ptr::null_mut::<itnode>();
        itree_tolist(root, &raw mut head);
        itree_simplify(head);
        root = ::core::ptr::null_mut::<itnode>();
        itree_totree(head, &raw mut root);
        return root as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_add_interval(
    mut o: *mut ::core::ffi::c_void,
    mut f: uint32_t,
    mut t: uint32_t,
    mut id: uint32_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut root: *mut itnode = o as *mut itnode;
        if id == 0 as uint32_t {
            if t < f {
                itree_delete(&raw mut root, t, f);
            } else {
                itree_delete(&raw mut root, f, t);
            }
        } else if t < f {
            itree_add(&raw mut root, t, f, id);
        } else {
            itree_add(&raw mut root, f, t, id);
        }
        return root as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_find(mut o: *mut ::core::ffi::c_void, mut v: uint32_t) -> uint32_t {
    unsafe {
        let mut n: *mut itnode = ::core::ptr::null_mut::<itnode>();
        n = o as *mut itnode;
        while !n.is_null() {
            if v >= (*n).from && v <= (*n).to {
                return (*n).id;
            }
            n = (if v < (*n).from { (*n).left } else { (*n).right }) as *mut itnode;
        }
        return 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn itree_freeall(mut o: *mut ::core::ffi::c_void) {
    unsafe {
        itree_free(o as *mut itnode);
    }
}
