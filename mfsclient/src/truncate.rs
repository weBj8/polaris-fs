unsafe extern "C" {
    unsafe fn fs_truncate(
        inode: uint32_t,
        flags: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        attrlength: uint64_t,
        attr: *mut uint8_t,
        prevlength: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
}
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_ERROR_EACCES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_ERROR_CHUNKLOST: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MFS_ERROR_LOCKED: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSPACE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const MFS_ERROR_EROFS: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const MFS_ERROR_QUOTA: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn portable_usleep(mut usec: uint64_t) {
    unsafe {
        let mut req: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut rem: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut s: ::core::ffi::c_int = 0;
        req.tv_sec = usec.wrapping_div(1000000 as uint64_t) as __time_t;
        req.tv_nsec = usec
            .wrapping_rem(1000000 as uint64_t)
            .wrapping_mul(1000 as uint64_t) as __syscall_slong_t;
        loop {
            s = nanosleep(&raw mut req, &raw mut rem);
            if s < 0 as ::core::ffi::c_int {
                req = rem;
            }
            if s >= 0 as ::core::ffi::c_int {
                break;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_truncate(
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut attrlength: uint64_t,
    mut attr: *mut uint8_t,
    mut prevlength: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut status: uint8_t = 0;
        let mut trycnt: uint32_t = 0;
        trycnt = 0 as uint32_t;
        loop {
            status = fs_truncate(inode, flags, uid, gids, gid, attrlength, attr, prevlength);
            if status as ::core::ffi::c_int == MFS_STATUS_OK
                || status as ::core::ffi::c_int == MFS_ERROR_EROFS
                || status as ::core::ffi::c_int == MFS_ERROR_EACCES
                || status as ::core::ffi::c_int == MFS_ERROR_EPERM
                || status as ::core::ffi::c_int == MFS_ERROR_ENOENT
                || status as ::core::ffi::c_int == MFS_ERROR_QUOTA
                || status as ::core::ffi::c_int == MFS_ERROR_NOSPACE
                || status as ::core::ffi::c_int == MFS_ERROR_CHUNKLOST
            {
                break;
            }
            if status as ::core::ffi::c_int != MFS_ERROR_LOCKED {
                trycnt = trycnt.wrapping_add(1);
                if trycnt >= 30 as uint32_t {
                    break;
                }
                portable_usleep((1000 as uint32_t).wrapping_add(
                    (if trycnt < 30 as uint32_t {
                        trycnt
                            .wrapping_sub(1 as uint32_t)
                            .wrapping_mul(300000 as uint32_t)
                    } else {
                        10000000 as uint32_t
                    }),
                ) as uint64_t);
            } else {
                portable_usleep(10000 as uint64_t);
            }
        }
        return status;
    }
}
