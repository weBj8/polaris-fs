pub enum __dirstream {}
unsafe extern "C" {
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn sparents_get(inode: uint32_t) -> uint32_t;
    unsafe fn fs_add_entry(inode: uint32_t);
    unsafe fn fs_forget_entry(inode: uint32_t);
    unsafe fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
pub type pid_t = __pid_t;
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sinodes_ino {
    pub inode: uint32_t,
    pub parent: uint32_t,
    pub next: *mut _sinodes_ino,
}
pub type sinodes_ino = _sinodes_ino;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const RINODES_CHECK_INTERVAL_100MS: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
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
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
static mut clthread: pthread_t = 0;
static mut term: uint8_t = 0;
static mut lastlist: *mut sinodes_ino = ::core::ptr::null_mut::<sinodes_ino>();
static mut currentlist: *mut sinodes_ino = ::core::ptr::null_mut::<sinodes_ino>();
#[inline]
unsafe extern "C" fn sinodes_close(mut inode: uint32_t) {
    unsafe {
        fs_forget_entry(inode);
    }
}
#[inline]
unsafe extern "C" fn sinodes_open(mut inode: uint32_t) {
    unsafe {
        fs_add_entry(inode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_process_inode(mut inode: uint32_t) {
    unsafe {
        let mut ril: *mut sinodes_ino = ::core::ptr::null_mut::<sinodes_ino>();
        let mut rilp: *mut *mut sinodes_ino = ::core::ptr::null_mut::<*mut sinodes_ino>();
        let mut parent: uint32_t = 0;
        rilp = &raw mut currentlist;
        loop {
            ril = *rilp;
            if ril.is_null() {
                break;
            }
            if inode > (*ril).inode {
                rilp = &raw mut (*ril).next as *mut *mut sinodes_ino;
            } else {
                if inode != (*ril).inode {
                    break;
                }
                parent = sparents_get(inode);
                if parent != 0 as uint32_t {
                    (*ril).parent = parent;
                }
                return;
            }
        }
        ril = malloc(::core::mem::size_of::<sinodes_ino>()) as *mut sinodes_ino;
        (*ril).inode = inode;
        (*ril).parent = sparents_get(inode);
        (*ril).next = *rilp as *mut _sinodes_ino;
        *rilp = ril;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_end() {
    unsafe {
        let mut rill: *mut sinodes_ino = ::core::ptr::null_mut::<sinodes_ino>();
        let mut ricl: *mut sinodes_ino = ::core::ptr::null_mut::<sinodes_ino>();
        rill = lastlist;
        ricl = currentlist;
        while !rill.is_null() || !ricl.is_null() {
            if ricl.is_null() || !rill.is_null() && (*rill).inode < (*ricl).inode {
                if (*rill).parent != 0 as uint32_t {
                    sinodes_close((*rill).parent);
                }
                sinodes_close((*rill).inode);
                rill = (*rill).next as *mut sinodes_ino;
            } else if rill.is_null() || (*rill).inode > (*ricl).inode {
                sinodes_open((*ricl).inode);
                if (*ricl).parent != 0 as uint32_t {
                    sinodes_open((*ricl).parent);
                }
                ricl = (*ricl).next as *mut sinodes_ino;
            } else {
                if (*rill).parent != (*ricl).parent {
                    if (*rill).parent != 0 as uint32_t {
                        if (*ricl).parent == 0 as uint32_t {
                            (*ricl).parent = (*rill).parent;
                        } else {
                            sinodes_close((*rill).parent);
                        }
                    }
                    if (*ricl).parent != 0 as uint32_t {
                        sinodes_open((*ricl).parent);
                    }
                }
                rill = (*rill).next as *mut sinodes_ino;
                ricl = (*ricl).next as *mut sinodes_ino;
            }
        }
        rill = lastlist;
        while !rill.is_null() {
            ricl = (*rill).next as *mut sinodes_ino;
            free(rill as *mut ::core::ffi::c_void);
            rill = ricl;
        }
        lastlist = currentlist;
        currentlist = ::core::ptr::null_mut::<sinodes_ino>();
    }
}
static mut mydevid: uint32_t = 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_pid_inodes(mut pid: pid_t) {
    unsafe {
        let mut devid: uint32_t = 0;
        let mut inode: uint64_t = 0;
        let mut path: [::core::ffi::c_char; 100] = [0; 100];
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        snprintf(
            &raw mut path as *mut ::core::ffi::c_char,
            100 as size_t,
            b"/proc/%lld/cwd\0".as_ptr() as *const ::core::ffi::c_char,
            pid as ::core::ffi::c_longlong,
        );
        if stat(&raw mut path as *mut ::core::ffi::c_char, &raw mut st) >= 0 as ::core::ffi::c_int {
            devid = st.st_dev as uint32_t;
            inode = st.st_ino as uint64_t;
            if devid == mydevid {
                sinodes_process_inode(inode as uint32_t);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_all_pids() {
    unsafe {
        let mut dd: *mut DIR = ::core::ptr::null_mut::<DIR>();
        let mut de: *mut dirent = ::core::ptr::null_mut::<dirent>();
        let mut np: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut pid: ::core::ffi::c_int = 0;
        dd = opendir(b"/proc\0".as_ptr() as *const ::core::ffi::c_char);
        if dd.is_null() {
            return;
        }
        loop {
            de = readdir(dd);
            if de.is_null() {
                break;
            }
            pid = 0 as ::core::ffi::c_int;
            np = &raw mut (*de).d_name as *mut ::core::ffi::c_char;
            while *np != 0 {
                if *np as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *np as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    pid *= 10 as ::core::ffi::c_int;
                    pid += *np as ::core::ffi::c_int - '0' as ::core::ffi::c_int;
                    np = np.offset(1);
                } else {
                    pid = 0 as ::core::ffi::c_int;
                    break;
                }
            }
            if pid > 0 as ::core::ffi::c_int {
                sinodes_pid_inodes(pid as pid_t);
            }
        }
        closedir(dd);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_scanthread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut i: uint32_t = 0;
        let mut mountpoint: *mut ::core::ffi::c_char = arg as *mut ::core::ffi::c_char;
        st.st_ino = 1 as __ino_t;
        while stat(mountpoint, &raw mut st) < 0 as ::core::ffi::c_int || st.st_ino != 1 as __ino_t {
            if st.st_ino == 1 as __ino_t {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't stat my mountpoint (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                    mountpoint,
                );
            } else {
                st.st_ino = 1 as __ino_t;
            }
            sleep(1 as ::core::ffi::c_uint);
            if ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut term,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                free(arg);
                return NULL;
            }
        }
        free(mountpoint as *mut ::core::ffi::c_void);
        mountpoint = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mydevid = st.st_dev as uint32_t;
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"my st_dev: %u\0".as_ptr() as *const ::core::ffi::c_char,
            mydevid,
        );
        i = 0 as uint32_t;
        loop {
            if i > RINODES_CHECK_INTERVAL_100MS as uint32_t {
                sinodes_all_pids();
                sinodes_end();
                i = 0 as uint32_t;
            } else {
                i = i.wrapping_add(1);
            }
            portable_usleep(100000 as uint64_t);
            if ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut term,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                return NULL;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_term() {
    unsafe {
        ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut term,
            1 as uint8_t,
        );
        pthread_join(
            clthread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        sinodes_end();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_init(mut mp: *const ::core::ffi::c_char) {
    unsafe {
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut term,
            0 as uint8_t,
        );
        lwt_minthread_create(
            &raw mut clthread,
            0 as uint8_t,
            Some(
                sinodes_scanthread
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            strdup(mp) as *mut ::core::ffi::c_void,
        );
    }
}
