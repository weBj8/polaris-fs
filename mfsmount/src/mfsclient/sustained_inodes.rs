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
// ---------------------------------------------------------------------------
// Safe core (P4 rewrite): sorted inode table + merge-diff. The C kept two
// malloc'd sorted linked lists (current/last) and diffed them in sinodes_end;
// here they are sorted Vecs with identical merge semantics.
// ---------------------------------------------------------------------------

#[deny(unsafe_code)]
pub mod imp {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct InodeEntry {
        pub inode: u32,
        pub parent: u32,
    }

    #[derive(Default)]
    pub struct Sinodes {
        last: Vec<InodeEntry>,
        current: Vec<InodeEntry>,
    }

    impl Sinodes {
        /// Record `inode` in the current scan (sorted insert/update).
        /// `parent_of` resolves an inode's parent (sparents_get at the
        /// boundary); parent 0 = unknown.
        pub fn process_inode(&mut self, inode: u32, parent_of: &dyn Fn(u32) -> u32) {
            match self.current.binary_search_by_key(&inode, |e| e.inode) {
                Ok(i) => {
                    let parent = parent_of(inode);
                    if parent != 0 {
                        self.current[i].parent = parent;
                    }
                }
                Err(i) => {
                    self.current.insert(
                        i,
                        InodeEntry {
                            inode,
                            parent: parent_of(inode),
                        },
                    );
                }
            }
        }

        /// Diff current scan against the previous one; `open`/`close` are
        /// fs_add_entry/fs_forget_entry at the boundary. Ends with
        /// last = current, current cleared (exact C merge order).
        pub fn end(
            &mut self,
            open: &mut dyn FnMut(u32),
            close: &mut dyn FnMut(u32),
        ) {
            let mut l = 0usize; // cursor into last
            let mut c = 0usize; // cursor into current
            while l < self.last.len() || c < self.current.len() {
                if c >= self.current.len()
                    || (l < self.last.len() && self.last[l].inode < self.current[c].inode)
                {
                    let e = self.last[l];
                    if e.parent != 0 {
                        close(e.parent);
                    }
                    close(e.inode);
                    l += 1;
                } else if l >= self.last.len() || self.last[l].inode > self.current[c].inode {
                    let e = self.current[c];
                    open(e.inode);
                    if e.parent != 0 {
                        open(e.parent);
                    }
                    c += 1;
                } else {
                    let lp = self.last[l].parent;
                    let cp = self.current[c].parent;
                    if lp != cp {
                        if lp != 0 {
                            if cp == 0 {
                                self.current[c].parent = lp;
                            } else {
                                close(lp);
                            }
                        }
                        if self.current[c].parent != 0 {
                            open(self.current[c].parent);
                        }
                    }
                    l += 1;
                    c += 1;
                }
            }
            ::core::mem::swap(&mut self.last, &mut self.current);
            self.current.clear();
        }
    }
}

// SAFETY: module is single-init; scan thread + FUSE threads call these
// functions under the mount's own serialization assumptions (same as C —
// the original had no locking here either). ponytail: global state mirrors
// the C design; per-instance state if a second consumer ever appears.
static mut SINODES: Option<imp::Sinodes> = None;

/// SAFETY: same serialization assumptions as the C original.
unsafe fn sinodes() -> &'static mut imp::Sinodes {
    unsafe {
        let p = &mut *(&raw mut SINODES);
        if p.is_none() {
            *p = Some(imp::Sinodes::default());
        }
        p.as_mut().unwrap_unchecked()
    }
}

/// SAFETY: boundary wrapper for sparents_get.
unsafe fn parent_of(inode: uint32_t) -> uint32_t {
    unsafe { sparents_get(inode) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_process_inode(inode: uint32_t) {
    unsafe {
        sinodes().process_inode(inode, &|i| parent_of(i));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_end() {
    unsafe {
        let mut open = |i: uint32_t| fs_add_entry(i);
        let mut close = |i: uint32_t| fs_forget_entry(i);
        sinodes().end(&mut open, &mut close);
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

// term flag as a real atomic (replaces core::intrinsics::atomic_or reads)
static TERM: ::core::sync::atomic::AtomicU8 = ::core::sync::atomic::AtomicU8::new(0);

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
            if TERM.load(::core::sync::atomic::Ordering::SeqCst) == 1 {
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
            if TERM.load(::core::sync::atomic::Ordering::SeqCst) == 1 {
                return NULL;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sinodes_term() {
    unsafe {
        TERM.store(1, ::core::sync::atomic::Ordering::SeqCst);
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
        TERM.store(0, ::core::sync::atomic::Ordering::SeqCst);
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

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::collections::HashMap;
    use std::vec::Vec;

    struct Mock {
        parents: HashMap<u32, u32>,
        log: std::cell::RefCell<Vec<(bool, u32)>>, // (open?, inode)
    }
    impl Mock {
        fn parent_of(&self, i: u32) -> u32 {
            *self.parents.get(&i).unwrap_or(&0)
        }
        fn open(&self, i: u32) {
            self.log.borrow_mut().push((true, i));
        }
        fn close(&self, i: u32) {
            self.log.borrow_mut().push((false, i));
        }
    }

    #[test]
    fn first_scan_opens_all() {
        let mut s = Sinodes::default();
        let mut m = Mock {
            parents: HashMap::from([(1, 100), (2, 100), (3, 0)]),
            log: std::cell::RefCell::new(Vec::new()),
        };
        for i in [3u32, 1, 2] {
            // unsorted insertion order on purpose
            let mo = &m;
            s.process_inode(i, &|x| mo.parent_of(x));
        }
        let mut open = |i: u32| (&m).open(i);
        let mut close = |i: u32| (&m).close(i);
        s.end(&mut open, &mut close);
        // sorted order: inode 1 (open, parent 100), 2 (open, parent 100), 3
        assert_eq!(
            *m.log.borrow(),
            std::vec![
                (true, 1),
                (true, 100),
                (true, 2),
                (true, 100),
                (true, 3)
            ]
        );
    }

    #[test]
    fn diff_closes_vanished_and_opens_new() {
        let mut s = Sinodes::default();
        let mut m = Mock {
            parents: HashMap::from([(1, 100), (2, 100), (4, 200)]),
            log: std::cell::RefCell::new(Vec::new()),
        };
        for i in [1u32, 2, 3] {
            let mo = &m;
            s.process_inode(i, &|x| mo.parent_of(x));
        }
        {
            let mut open = |i: u32| (&m).open(i);
            let mut close = |i: u32| (&m).close(i);
            s.end(&mut open, &mut close);
        }
        m.log.borrow_mut().clear();
        // second scan: 1 stays, 2 vanishes, 4 appears
        for i in [1u32, 4] {
            let mo = &m;
            s.process_inode(i, &|x| mo.parent_of(x));
        }
        {
            let mut open = |i: u32| (&m).open(i);
            let mut close = |i: u32| (&m).close(i);
            s.end(&mut open, &mut close);
        }
        // C merge order: 1 equal (parent 100 == 100, nothing), 2 closed
        // (parent 100 closed first), 3 closed, 4 opened (+parent 200)
        assert_eq!(
            *m.log.borrow(),
            std::vec![
                (false, 100),
                (false, 2),
                (false, 3),
                (true, 4),
                (true, 200)
            ]
        );
    }

    #[test]
    fn parent_transition_semantics() {
        let mut s = Sinodes::default();
        let mut m = Mock {
            parents: HashMap::from([(1, 100)]),
            log: std::cell::RefCell::new(Vec::new()),
        };
        s.process_inode(1, &|x| m.parent_of(x));
        {
            let mut open = |i: u32| (&m).open(i);
            let mut close = |i: u32| (&m).close(i);
            s.end(&mut open, &mut close);
        }
        m.log.borrow_mut().clear();
        // parent changes 100 -> 200
        m.parents.insert(1, 200);
        {
            let mo = &m;
            s.process_inode(1, &|x| mo.parent_of(x));
        }
        {
            let mut open = |i: u32| (&m).open(i);
            let mut close = |i: u32| (&m).close(i);
            s.end(&mut open, &mut close);
        }
        assert_eq!(*m.log.borrow(), std::vec![(false, 100), (true, 200)]);
        // parent becomes unknown (0): new entry inserts with parent 0,
        // merge inherits the old parent 200 and re-opens it (C quirk)
        m.log.borrow_mut().clear();
        m.parents.insert(1, 0);
        {
            let mo = &m;
            s.process_inode(1, &|x| mo.parent_of(x));
        }
        {
            let mut open = |i: u32| (&m).open(i);
            let mut close = |i: u32| (&m).close(i);
            s.end(&mut open, &mut close);
        }
        assert_eq!(*m.log.borrow(), std::vec![(true, 200)]);
    }
}
