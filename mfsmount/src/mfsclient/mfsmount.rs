#![feature(core_intrinsics)]
#![allow(
    clippy::missing_safety_doc,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum fuse_session {}
pub enum fuse_pollhandle {}
pub enum fuse_req {}
pub enum __dirstream {}
#[macro_use]
extern crate c2rust_bitfields;
#[allow(unused_imports)]
use ::mfsmount;
unsafe extern "C" {
    unsafe fn mlockall(__flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> ::core::ffi::c_int;
    unsafe fn setpriority(
        __which: __priority_which_t,
        __who: id_t,
        __prio: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    unsafe fn mallopt(__param: ::core::ffi::c_int, __val: ::core::ffi::c_int)
    -> ::core::ffi::c_int;
    unsafe fn uname(__name: *mut utsname) -> ::core::ffi::c_int;
    unsafe fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut ::core::ffi::c_void,
        opts: *const fuse_opt,
        proc: fuse_opt_proc_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_opt_add_arg(
        args: *mut fuse_args,
        arg: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_opt_insert_arg(
        args: *mut fuse_args,
        pos: ::core::ffi::c_int,
        arg: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_opt_free_args(args: *mut fuse_args);
    unsafe fn fuse_version() -> ::core::ffi::c_int;
    unsafe fn fuse_set_signal_handlers(se: *mut fuse_session) -> ::core::ffi::c_int;
    unsafe fn fuse_remove_signal_handlers(se: *mut fuse_session);
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fuse_lowlevel_version();
    unsafe fn fuse_lowlevel_help();
    unsafe fn fuse_cmdline_help();
    unsafe fn fuse_parse_cmdline(
        args: *mut fuse_args,
        opts: *mut fuse_cmdline_opts,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_session_new_versioned(
        args: *mut fuse_args,
        op: *const fuse_lowlevel_ops,
        op_size: size_t,
        version: *mut libfuse_version,
        userdata: *mut ::core::ffi::c_void,
    ) -> *mut fuse_session;
    unsafe fn fuse_session_mount(
        se: *mut fuse_session,
        mountpoint: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_session_loop(se: *mut fuse_session) -> ::core::ffi::c_int;
    unsafe fn fuse_session_loop_mt_32(
        se: *mut fuse_session,
        config: *mut fuse_loop_config,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_session_unmount(se: *mut fuse_session);
    unsafe fn fuse_session_destroy(se: *mut fuse_session);
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    unsafe fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    unsafe fn readdir(__dirp: *mut DIR) -> *mut dirent;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
    unsafe fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::core::ffi::c_int;
    unsafe fn setsid() -> __pid_t;
    unsafe fn fork() -> __pid_t;
    unsafe fn getpass(__prompt: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn exit(__status: ::core::ffi::c_int) -> !;
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn mfs_statfs(req: fuse_req_t, ino: fuse_ino_t);
    unsafe fn mfs_access(req: fuse_req_t, ino: fuse_ino_t, mask: ::core::ffi::c_int);
    unsafe fn mfs_lookup(req: fuse_req_t, parent: fuse_ino_t, name: *const ::core::ffi::c_char);
    unsafe fn mfs_getattr(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_setattr(
        req: fuse_req_t,
        ino: fuse_ino_t,
        stbuf: *mut stat,
        to_set: ::core::ffi::c_int,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_mknod(
        req: fuse_req_t,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
        mode: mode_t,
        rdev: dev_t,
    );
    unsafe fn mfs_unlink(req: fuse_req_t, parent: fuse_ino_t, name: *const ::core::ffi::c_char);
    unsafe fn mfs_mkdir(
        req: fuse_req_t,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
        mode: mode_t,
    );
    unsafe fn mfs_rmdir(req: fuse_req_t, parent: fuse_ino_t, name: *const ::core::ffi::c_char);
    unsafe fn mfs_symlink(
        req: fuse_req_t,
        path: *const ::core::ffi::c_char,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
    );
    unsafe fn mfs_readlink(req: fuse_req_t, ino: fuse_ino_t);
    unsafe fn mfs_rename(
        req: fuse_req_t,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
        newparent: fuse_ino_t,
        newname: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    );
    unsafe fn mfs_link(
        req: fuse_req_t,
        ino: fuse_ino_t,
        newparent: fuse_ino_t,
        newname: *const ::core::ffi::c_char,
    );
    unsafe fn mfs_opendir(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_readdir(
        req: fuse_req_t,
        ino: fuse_ino_t,
        size: size_t,
        off: off_t,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_releasedir(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_create(
        req: fuse_req_t,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
        mode: mode_t,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_open(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_release(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_read(
        req: fuse_req_t,
        ino: fuse_ino_t,
        size: size_t,
        off: off_t,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_write(
        req: fuse_req_t,
        ino: fuse_ino_t,
        buf: *const ::core::ffi::c_char,
        size: size_t,
        off: off_t,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_flush(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_fsync(
        req: fuse_req_t,
        ino: fuse_ino_t,
        datasync: ::core::ffi::c_int,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_setxattr(
        req: fuse_req_t,
        ino: fuse_ino_t,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        size: size_t,
        flags: ::core::ffi::c_int,
    );
    unsafe fn mfs_getxattr(
        req: fuse_req_t,
        ino: fuse_ino_t,
        name: *const ::core::ffi::c_char,
        size: size_t,
    );
    unsafe fn mfs_listxattr(req: fuse_req_t, ino: fuse_ino_t, size: size_t);
    unsafe fn mfs_removexattr(req: fuse_req_t, ino: fuse_ino_t, name: *const ::core::ffi::c_char);
    unsafe fn mfs_getlk(
        req: fuse_req_t,
        ino: fuse_ino_t,
        fi: *mut fuse_file_info,
        lock: *mut flock,
    );
    unsafe fn mfs_setlk(
        req: fuse_req_t,
        ino: fuse_ino_t,
        fi: *mut fuse_file_info,
        lock: *mut flock,
        sl: ::core::ffi::c_int,
    );
    unsafe fn mfs_flock(
        req: fuse_req_t,
        ino: fuse_ino_t,
        fi: *mut fuse_file_info,
        op: ::core::ffi::c_int,
    );
    unsafe fn mfs_readdirplus(
        req: fuse_req_t,
        ino: fuse_ino_t,
        size: size_t,
        off: off_t,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_term();
    unsafe fn mfs_init(
        debug_mode_in: ::core::ffi::c_int,
        keep_cache_in: ::core::ffi::c_int,
        readdirplus_cache_min_timeout_in: ::core::ffi::c_double,
        direntry_cache_timeout_in: ::core::ffi::c_double,
        entry_cache_timeout_in: ::core::ffi::c_double,
        attr_cache_timeout_in: ::core::ffi::c_double,
        xattr_cache_timeout_in: ::core::ffi::c_double,
        groups_cache_timeout: ::core::ffi::c_double,
        mkdir_copy_sgid_in: ::core::ffi::c_int,
        sugid_clear_mode_in: ::core::ffi::c_int,
        xattr_acl_support_in: ::core::ffi::c_int,
        fsync_before_close_min_time_in: ::core::ffi::c_double,
        no_xattrs_in: ::core::ffi::c_int,
        no_posix_locks_in: ::core::ffi::c_int,
        no_bsd_locks_in: ::core::ffi::c_int,
    );
    unsafe fn mfs_setsession(se: *mut fuse_session);
    unsafe fn mfs_meta_statfs(req: fuse_req_t, ino: fuse_ino_t);
    unsafe fn mfs_meta_lookup(
        req: fuse_req_t,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
    );
    unsafe fn mfs_meta_getattr(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_meta_setattr(
        req: fuse_req_t,
        ino: fuse_ino_t,
        stbuf: *mut stat,
        to_set: ::core::ffi::c_int,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_meta_unlink(
        req: fuse_req_t,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
    );
    unsafe fn mfs_meta_rename(
        req: fuse_req_t,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
        newparent: fuse_ino_t,
        newname: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    );
    unsafe fn mfs_meta_opendir(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_meta_readdir(
        req: fuse_req_t,
        ino: fuse_ino_t,
        size: size_t,
        off: off_t,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_meta_releasedir(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_meta_open(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_meta_release(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info);
    unsafe fn mfs_meta_read(
        req: fuse_req_t,
        ino: fuse_ino_t,
        size: size_t,
        off: off_t,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_meta_write(
        req: fuse_req_t,
        ino: fuse_ino_t,
        buf: *const ::core::ffi::c_char,
        size: size_t,
        off: off_t,
        fi: *mut fuse_file_info,
    );
    unsafe fn mfs_meta_init(
        debug_mode_in: ::core::ffi::c_int,
        entry_cache_timeout_in: ::core::ffi::c_double,
        attr_cache_timeout_in: ::core::ffi::c_double,
        flat_trash_in: ::core::ffi::c_int,
    );
    unsafe fn monotonic_method() -> *const ::core::ffi::c_char;
    unsafe fn monotonic_speed() -> uint32_t;
    unsafe fn mfs_log_str_to_pri(pristr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn mfs_log_set_min_level(minlevel: ::core::ffi::c_int);
    unsafe fn mfs_log_set_elevate_to(elevateto: ::core::ffi::c_int);
    unsafe fn mfs_log_term();
    unsafe fn mfs_log_init(
        ident: *const ::core::ffi::c_char,
        daemon: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn strerr_init();
    unsafe fn strerr_term();
    unsafe fn md5_init(ctx: *mut md5ctx);
    unsafe fn md5_update(ctx: *mut md5ctx, buff: *const uint8_t, leng: uint32_t);
    unsafe fn md5_final(digest: *mut uint8_t, ctx: *mut md5ctx);
    unsafe fn fs_set_working_flags(sflags: uint8_t);
    unsafe fn master_version() -> uint32_t;
    unsafe fn fs_get_current_srcstrip() -> *const ::core::ffi::c_char;
    unsafe fn fs_get_current_masterstrip() -> *const ::core::ffi::c_char;
    unsafe fn fs_get_current_masterport() -> uint16_t;
    unsafe fn fs_init_master_connection(
        bindhostname: *const ::core::ffi::c_char,
        masterhostname: *const ::core::ffi::c_char,
        masterportname: *const ::core::ffi::c_char,
        meta: uint8_t,
        info: *const ::core::ffi::c_char,
        subfolder: *const ::core::ffi::c_char,
        passworddigest: *const uint8_t,
        donotrememberpassword: uint8_t,
        bgregister: uint8_t,
        minversion: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fs_init_threads(retries: uint32_t, to: uint32_t);
    unsafe fn fs_term();
    unsafe fn masterproxy_term();
    unsafe fn masterproxy_init(masterproxyip: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn csorder_init(labelexpr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn sparents_term();
    unsafe fn sparents_init();
    unsafe fn sinodes_term();
    unsafe fn sinodes_init(mountpoint: *const ::core::ffi::c_char);
    unsafe fn sstats_term();
    unsafe fn sstats_init() -> ::core::ffi::c_int;
    unsafe fn symlink_cache_init(to: ::core::ffi::c_double);
    unsafe fn symlink_cache_term();
    unsafe fn negentry_cache_init(to: ::core::ffi::c_double);
    unsafe fn negentry_cache_term();
    unsafe fn chunksdatacache_term();
    unsafe fn chunksdatacache_init();
    unsafe fn inoleng_term();
    unsafe fn inoleng_init();
    unsafe fn conncache_term();
    unsafe fn conncache_init(capacity: uint32_t) -> ::core::ffi::c_int;
    unsafe fn chunkrwlock_init();
    unsafe fn chunkrwlock_term();
    unsafe fn read_data_init(
        readaheadsize: uint64_t,
        readaheadleng: uint32_t,
        readaheadtrigger: uint32_t,
        retries: uint32_t,
        timeout: uint32_t,
        minlogretry: uint32_t,
        erronlostchunk: uint8_t,
        erronnospace: uint8_t,
    );
    unsafe fn read_data_term();
    unsafe fn read_init();
    unsafe fn read_term();
    unsafe fn write_data_init(
        cachesize: uint32_t,
        retries: uint32_t,
        timeout: uint32_t,
        minlogretry: uint32_t,
        erronlostchunk: uint8_t,
        erronnospace: uint8_t,
    );
    unsafe fn write_data_term();
    unsafe fn write_init();
    unsafe fn write_term();
    unsafe fn delay_term();
    unsafe fn delay_init();
    unsafe fn csdb_init();
    unsafe fn csdb_term();
    unsafe fn stats_term();
    unsafe fn mycrc32_init();
    unsafe fn processname_init(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char);
    unsafe fn processname_set(name: *mut ::core::ffi::c_char);
}
pub type __uint64_t = u64;
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
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __id_t = ::core::ffi::c_uint;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type size_t = usize;
pub type off_t = __off64_t;
pub type mode_t = __mode_t;
pub type __rlimit_resource = ::core::ffi::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __priority_which = ::core::ffi::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type id_t = __id_t;
pub type __rlimit_resource_t = __rlimit_resource;
pub type __priority_which_t = __priority_which;
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
pub type ssize_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [::core::ffi::c_char; 65],
    pub nodename: [::core::ffi::c_char; 65],
    pub release: [::core::ffi::c_char; 65],
    pub version: [::core::ffi::c_char; 65],
    pub machine: [::core::ffi::c_char; 65],
    pub domainname: [::core::ffi::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_opt {
    pub templ: *const ::core::ffi::c_char,
    pub offset: ::core::ffi::c_ulong,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_args {
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
    pub allocated: ::core::ffi::c_int,
}
pub type fuse_opt_proc_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        *mut fuse_args,
    ) -> ::core::ffi::c_int,
>;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type dev_t = __dev_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct fuse_file_info {
    pub flags: int32_t,
    #[bitfield(name = "writepage", ty = "uint32_t", bits = "0..=0")]
    #[bitfield(name = "direct_io", ty = "uint32_t", bits = "1..=1")]
    #[bitfield(name = "keep_cache", ty = "uint32_t", bits = "2..=2")]
    #[bitfield(name = "flush", ty = "uint32_t", bits = "3..=3")]
    #[bitfield(name = "nonseekable", ty = "uint32_t", bits = "4..=4")]
    #[bitfield(name = "flock_release", ty = "uint32_t", bits = "5..=5")]
    #[bitfield(name = "cache_readdir", ty = "uint32_t", bits = "6..=6")]
    #[bitfield(name = "noflush", ty = "uint32_t", bits = "7..=7")]
    #[bitfield(name = "parallel_direct_writes", ty = "uint32_t", bits = "8..=8")]
    #[bitfield(name = "padding", ty = "uint32_t", bits = "9..=31")]
    #[bitfield(name = "padding2", ty = "uint32_t", bits = "32..=63")]
    #[bitfield(name = "padding3", ty = "uint32_t", bits = "64..=95")]
    pub writepage_direct_io_keep_cache_flush_nonseekable_flock_release_cache_readdir_noflush_parallel_direct_writes_padding_padding2_padding3:
        [u8; 12],
    pub fh: uint64_t,
    pub lock_owner: uint64_t,
    pub poll_events: uint32_t,
    pub backing_id: int32_t,
    pub compat_flags: uint64_t,
    pub reserved: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_loop_config {
    pub clone_fd: ::core::ffi::c_int,
    pub max_idle_threads: ::core::ffi::c_uint,
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct fuse_conn_info {
    pub proto_major: uint32_t,
    pub proto_minor: uint32_t,
    pub max_write: uint32_t,
    pub max_read: uint32_t,
    pub max_readahead: uint32_t,
    pub capable: uint32_t,
    pub want: uint32_t,
    pub max_background: uint32_t,
    pub congestion_threshold: uint32_t,
    pub time_gran: uint32_t,
    pub max_backing_stack_depth: uint32_t,
    #[bitfield(name = "no_interrupt", ty = "uint32_t", bits = "0..=0")]
    #[bitfield(name = "padding", ty = "uint32_t", bits = "1..=31")]
    pub no_interrupt_padding: [u8; 4],
    pub capable_ext: uint64_t,
    pub want_ext: uint64_t,
    pub request_timeout: uint16_t,
    pub reserved: [uint16_t; 31],
}
pub type fuse_buf_flags = ::core::ffi::c_uint;
pub const FUSE_BUF_FD_RETRY: fuse_buf_flags = 8;
pub const FUSE_BUF_FD_SEEK: fuse_buf_flags = 4;
pub const FUSE_BUF_IS_FD: fuse_buf_flags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_buf {
    pub size: size_t,
    pub flags: fuse_buf_flags,
    pub mem: *mut ::core::ffi::c_void,
    pub fd: ::core::ffi::c_int,
    pub pos: off_t,
    pub mem_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_bufvec {
    pub count: size_t,
    pub idx: size_t,
    pub off: size_t,
    pub buf: [fuse_buf; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libfuse_version {
    pub major: uint32_t,
    pub minor: uint32_t,
    pub hotfix: uint32_t,
    pub padding: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: ::core::ffi::c_short,
    pub l_whence: ::core::ffi::c_short,
    pub l_start: __off64_t,
    pub l_len: __off64_t,
    pub l_pid: __pid_t,
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
pub type fuse_ino_t = uint64_t;
pub type fuse_req_t = *mut fuse_req;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_forget_data {
    pub ino: fuse_ino_t,
    pub nlookup: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_lowlevel_ops {
    pub init: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut fuse_conn_info) -> ()>,
    pub destroy: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub lookup:
        Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> ()>,
    pub forget: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, uint64_t) -> ()>,
    pub getattr: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> ()>,
    pub setattr: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *mut stat,
            ::core::ffi::c_int,
            *mut fuse_file_info,
        ) -> (),
    >,
    pub readlink: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t) -> ()>,
    pub mknod: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *const ::core::ffi::c_char,
            mode_t,
            dev_t,
        ) -> (),
    >,
    pub mkdir: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char, mode_t) -> (),
    >,
    pub unlink:
        Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> ()>,
    pub rmdir:
        Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> ()>,
    pub symlink: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            *const ::core::ffi::c_char,
            fuse_ino_t,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub rename: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *const ::core::ffi::c_char,
            fuse_ino_t,
            *const ::core::ffi::c_char,
            ::core::ffi::c_uint,
        ) -> (),
    >,
    pub link: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, fuse_ino_t, *const ::core::ffi::c_char) -> (),
    >,
    pub open: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> ()>,
    pub read: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, size_t, off_t, *mut fuse_file_info) -> (),
    >,
    pub write: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *const ::core::ffi::c_char,
            size_t,
            off_t,
            *mut fuse_file_info,
        ) -> (),
    >,
    pub flush: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> ()>,
    pub release: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> ()>,
    pub fsync: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, ::core::ffi::c_int, *mut fuse_file_info) -> (),
    >,
    pub opendir: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> ()>,
    pub readdir: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, size_t, off_t, *mut fuse_file_info) -> (),
    >,
    pub releasedir: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> ()>,
    pub fsyncdir: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, ::core::ffi::c_int, *mut fuse_file_info) -> (),
    >,
    pub statfs: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t) -> ()>,
    pub setxattr: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            size_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub getxattr: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char, size_t) -> (),
    >,
    pub listxattr: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, size_t) -> ()>,
    pub removexattr:
        Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> ()>,
    pub access: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, ::core::ffi::c_int) -> ()>,
    pub create: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *const ::core::ffi::c_char,
            mode_t,
            *mut fuse_file_info,
        ) -> (),
    >,
    pub getlk:
        Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info, *mut flock) -> ()>,
    pub setlk: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *mut fuse_file_info,
            *mut flock,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub bmap: Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, size_t, uint64_t) -> ()>,
    pub ioctl: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            *mut fuse_file_info,
            ::core::ffi::c_uint,
            *const ::core::ffi::c_void,
            size_t,
            size_t,
        ) -> (),
    >,
    pub poll: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *mut fuse_file_info,
            *mut fuse_pollhandle,
        ) -> (),
    >,
    pub write_buf: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            *mut fuse_bufvec,
            off_t,
            *mut fuse_file_info,
        ) -> (),
    >,
    pub retrieve_reply: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            *mut ::core::ffi::c_void,
            fuse_ino_t,
            off_t,
            *mut fuse_bufvec,
        ) -> (),
    >,
    pub forget_multi: Option<unsafe extern "C" fn(fuse_req_t, size_t, *mut fuse_forget_data) -> ()>,
    pub flock: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info, ::core::ffi::c_int) -> (),
    >,
    pub fallocate: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            ::core::ffi::c_int,
            off_t,
            off_t,
            *mut fuse_file_info,
        ) -> (),
    >,
    pub readdirplus: Option<
        unsafe extern "C" fn(fuse_req_t, fuse_ino_t, size_t, off_t, *mut fuse_file_info) -> (),
    >,
    pub copy_file_range: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            off_t,
            *mut fuse_file_info,
            fuse_ino_t,
            off_t,
            *mut fuse_file_info,
            size_t,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub lseek: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            off_t,
            ::core::ffi::c_int,
            *mut fuse_file_info,
        ) -> (),
    >,
    pub tmpfile:
        Option<unsafe extern "C" fn(fuse_req_t, fuse_ino_t, mode_t, *mut fuse_file_info) -> ()>,
    pub statx: Option<
        unsafe extern "C" fn(
            fuse_req_t,
            fuse_ino_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut fuse_file_info,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_cmdline_opts {
    pub singlethread: ::core::ffi::c_int,
    pub foreground: ::core::ffi::c_int,
    pub debug: ::core::ffi::c_int,
    pub nodefault_subtype: ::core::ffi::c_int,
    pub mountpoint: *mut ::core::ffi::c_char,
    pub show_version: ::core::ffi::c_int,
    pub show_help: ::core::ffi::c_int,
    pub clone_fd: ::core::ffi::c_int,
    pub max_idle_threads: ::core::ffi::c_uint,
    pub max_threads: ::core::ffi::c_uint,
}
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
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _md5ctx {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub type md5ctx = _md5ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mfsopts {
    pub masterhost: *mut ::core::ffi::c_char,
    pub masterport: *mut ::core::ffi::c_char,
    pub bindhost: *mut ::core::ffi::c_char,
    pub proxyhost: *mut ::core::ffi::c_char,
    pub subfolder: *mut ::core::ffi::c_char,
    pub password: *mut ::core::ffi::c_char,
    pub passfile: *mut ::core::ffi::c_char,
    pub md5pass: *mut ::core::ffi::c_char,
    pub preferedlabels: *mut ::core::ffi::c_char,
    pub nofile: ::core::ffi::c_uint,
    pub nice: ::core::ffi::c_int,
    pub mfssuid: ::core::ffi::c_int,
    pub mfsdev: ::core::ffi::c_int,
    pub memlock: ::core::ffi::c_int,
    pub limitarenas: ::core::ffi::c_int,
    pub allowoomkiller: ::core::ffi::c_int,
    pub nonempty: ::core::ffi::c_int,
    pub logminlevelstr: *mut ::core::ffi::c_char,
    pub logminlevel: ::core::ffi::c_int,
    pub logelevatetostr: *mut ::core::ffi::c_char,
    pub logelevateto: ::core::ffi::c_int,
    pub nostdmountoptions: ::core::ffi::c_int,
    pub meta: ::core::ffi::c_int,
    pub debug: ::core::ffi::c_int,
    pub flattrash: ::core::ffi::c_int,
    pub delayedinit: ::core::ffi::c_int,
    pub mkdircopysgid: ::core::ffi::c_uint,
    pub sugidclearmodestr: *mut ::core::ffi::c_char,
    pub sugidclearmode: ::core::ffi::c_int,
    pub cachemode: *mut ::core::ffi::c_char,
    pub cachefiles: ::core::ffi::c_int,
    pub keepcache: ::core::ffi::c_int,
    pub passwordask: ::core::ffi::c_int,
    pub noxattrs: ::core::ffi::c_int,
    pub noposixlocks: ::core::ffi::c_int,
    pub nobsdlocks: ::core::ffi::c_int,
    pub donotrememberpassword: ::core::ffi::c_int,
    pub writecachesize: ::core::ffi::c_uint,
    pub readaheadsize: ::core::ffi::c_uint,
    pub readaheadleng: ::core::ffi::c_uint,
    pub readaheadtrigger: ::core::ffi::c_uint,
    pub erroronlostchunk: ::core::ffi::c_int,
    pub erroronnospace: ::core::ffi::c_int,
    pub ioretries: ::core::ffi::c_uint,
    pub timeout: ::core::ffi::c_uint,
    pub logretry: ::core::ffi::c_uint,
    pub readdirplusminto: ::core::ffi::c_double,
    pub attrcacheto: ::core::ffi::c_double,
    pub xattrcacheto: ::core::ffi::c_double,
    pub entrycacheto: ::core::ffi::c_double,
    pub direntrycacheto: ::core::ffi::c_double,
    pub negentrycacheto: ::core::ffi::c_double,
    pub symlinkcacheto: ::core::ffi::c_double,
    pub groupscacheto: ::core::ffi::c_double,
    pub fsyncmintime: ::core::ffi::c_double,
    pub fsyncbeforeclose: ::core::ffi::c_int,
    pub netdev: ::core::ffi::c_int,
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const KEY_VERSION: C2Rust_Unnamed = 10;
pub const KEY_HELP: C2Rust_Unnamed = 9;
pub const KEY_NOSTDMOUNTOPTIONS: C2Rust_Unnamed = 8;
pub const KEY_PASSWORDASK: C2Rust_Unnamed = 7;
pub const KEY_PATH: C2Rust_Unnamed = 6;
pub const KEY_PROXY: C2Rust_Unnamed = 5;
pub const KEY_BIND: C2Rust_Unnamed = 4;
pub const KEY_PORT: C2Rust_Unnamed = 3;
pub const KEY_HOST: C2Rust_Unnamed = 2;
pub const KEY_META: C2Rust_Unnamed = 1;
pub const KEY_CFGFILE: C2Rust_Unnamed = 0;
pub const MCL_CURRENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MCL_FUTURE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const RLIM_INFINITY: ::core::ffi::c_ulonglong = 0xffffffffffffffff as ::core::ffi::c_ulonglong;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    unsafe {
        return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
    }
}
pub const M_ARENA_TEST: ::core::ffi::c_int = -7 as ::core::ffi::c_int;
pub const M_ARENA_MAX: ::core::ffi::c_int = -8 as ::core::ffi::c_int;
pub const OOM_SCORE_ADJ_MIN: ::core::ffi::c_int = -1000 as ::core::ffi::c_int;
pub const OOM_DISABLE: ::core::ffi::c_int = -17 as ::core::ffi::c_int;
pub const FUSE_HOTFIX_VERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FUSE_MAJOR_VERSION: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FUSE_MINOR_VERSION: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const FUSE_OPT_END: fuse_opt = fuse_opt {
    templ: ::core::ptr::null::<::core::ffi::c_char>(),
    offset: 0 as ::core::ffi::c_ulong,
    value: 0 as ::core::ffi::c_int,
};
pub const FUSE_OPT_KEY_OPT: ::core::ffi::c_int = -1;
pub const FUSE_OPT_KEY_NONOPT: ::core::ffi::c_int = -2;
pub const FUSE_CAP_ASYNC_READ: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 0 as ::core::ffi::c_int;
pub const FUSE_CAP_POSIX_LOCKS: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 1 as ::core::ffi::c_int;
pub const FUSE_CAP_ATOMIC_O_TRUNC: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 3 as ::core::ffi::c_int;
pub const FUSE_CAP_EXPORT_SUPPORT: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 4 as ::core::ffi::c_int;
pub const FUSE_CAP_DONT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 6 as ::core::ffi::c_int;
pub const FUSE_CAP_SPLICE_WRITE: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 7 as ::core::ffi::c_int;
pub const FUSE_CAP_SPLICE_MOVE: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int;
pub const FUSE_CAP_SPLICE_READ: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 9 as ::core::ffi::c_int;
pub const FUSE_CAP_FLOCK_LOCKS: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 10 as ::core::ffi::c_int;
pub const FUSE_CAP_AUTO_INVAL_DATA: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 12 as ::core::ffi::c_int;
pub const FUSE_CAP_READDIRPLUS: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 13 as ::core::ffi::c_int;
pub const FUSE_CAP_READDIRPLUS_AUTO: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 14 as ::core::ffi::c_int;
pub const FUSE_CAP_ASYNC_DIO: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 15 as ::core::ffi::c_int;
pub const FUSE_CAP_WRITEBACK_CACHE: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int;
pub const FUSE_CAP_PARALLEL_DIROPS: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 18 as ::core::ffi::c_int;
pub const FUSE_CAP_POSIX_ACL: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 19 as ::core::ffi::c_int;
pub const FUSE_CAP_HANDLE_KILLPRIV: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 20 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFDIR: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const S_IFDIR: ::core::ffi::c_int = __S_IFDIR;
#[inline]
unsafe extern "C" fn fuse_session_new_fn(
    mut args: *mut fuse_args,
    mut op: *const fuse_lowlevel_ops,
    mut op_size: size_t,
    mut userdata: *mut ::core::ffi::c_void,
) -> *mut fuse_session {
    unsafe {
        let mut version: libfuse_version = libfuse_version {
            major: FUSE_MAJOR_VERSION as uint32_t,
            minor: FUSE_MINOR_VERSION as uint32_t,
            hotfix: FUSE_HOTFIX_VERSION as uint32_t,
            padding: 0 as uint32_t,
        };
        return fuse_session_new_versioned(args, op, op_size, &raw mut version, userdata);
    }
}
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_NEVER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_ALWAYS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_OSX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_BSD: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_EXT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_XFS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const WFLAG_INVALIDATE_CACHE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DEFAULT_MASTERNAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"mfsmaster\0") };
pub const DEFAULT_MASTER_CLIENT_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9421\0") };
pub const VERSSTR: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"4.59.2-1\0") };
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub static mut id: [::core::ffi::c_char; 72] = unsafe {
    ::core::mem::transmute::<[u8; 72], [::core::ffi::c_char; 72]>(
        *b"@(#) version: 4.59.2-1, build: 2106, written by Jakub Kruszona-Zawadzki\0",
    )
};
static mut mfs_meta_oper: fuse_lowlevel_ops = fuse_lowlevel_ops {
    init: Some(
        mfs_fsinit as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut fuse_conn_info) -> (),
    ),
    destroy: None,
    lookup: Some(
        mfs_meta_lookup
            as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> (),
    ),
    forget: None,
    getattr: Some(
        mfs_meta_getattr as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    setattr: Some(
        mfs_meta_setattr
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *mut stat,
                ::core::ffi::c_int,
                *mut fuse_file_info,
            ) -> (),
    ),
    readlink: None,
    mknod: None,
    mkdir: None,
    unlink: Some(
        mfs_meta_unlink
            as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> (),
    ),
    rmdir: None,
    symlink: None,
    rename: Some(
        mfs_meta_rename
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                ::core::ffi::c_uint,
            ) -> (),
    ),
    link: None,
    open: Some(
        mfs_meta_open as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    read: Some(
        mfs_meta_read
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                size_t,
                off_t,
                *mut fuse_file_info,
            ) -> (),
    ),
    write: Some(
        mfs_meta_write
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                size_t,
                off_t,
                *mut fuse_file_info,
            ) -> (),
    ),
    flush: None,
    release: Some(
        mfs_meta_release as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    fsync: None,
    opendir: Some(
        mfs_meta_opendir as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    readdir: Some(
        mfs_meta_readdir
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                size_t,
                off_t,
                *mut fuse_file_info,
            ) -> (),
    ),
    releasedir: Some(
        mfs_meta_releasedir
            as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    fsyncdir: None,
    statfs: Some(mfs_meta_statfs as unsafe extern "C" fn(fuse_req_t, fuse_ino_t) -> ()),
    setxattr: None,
    getxattr: None,
    listxattr: None,
    removexattr: None,
    access: None,
    create: None,
    getlk: None,
    setlk: None,
    bmap: None,
    ioctl: None,
    poll: None,
    write_buf: None,
    retrieve_reply: None,
    forget_multi: None,
    flock: None,
    fallocate: None,
    readdirplus: None,
    copy_file_range: None,
    lseek: None,
    tmpfile: None,
    statx: None,
};
static mut mfs_oper: fuse_lowlevel_ops = fuse_lowlevel_ops {
    init: Some(
        mfs_fsinit as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut fuse_conn_info) -> (),
    ),
    destroy: None,
    lookup: Some(
        mfs_lookup
            as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> (),
    ),
    forget: None,
    getattr: Some(
        mfs_getattr as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    setattr: Some(
        mfs_setattr
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *mut stat,
                ::core::ffi::c_int,
                *mut fuse_file_info,
            ) -> (),
    ),
    readlink: Some(mfs_readlink as unsafe extern "C" fn(fuse_req_t, fuse_ino_t) -> ()),
    mknod: Some(
        mfs_mknod
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                mode_t,
                dev_t,
            ) -> (),
    ),
    mkdir: Some(
        mfs_mkdir
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                mode_t,
            ) -> (),
    ),
    unlink: Some(
        mfs_unlink
            as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> (),
    ),
    rmdir: Some(
        mfs_rmdir as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> (),
    ),
    symlink: Some(
        mfs_symlink
            as unsafe extern "C" fn(
                fuse_req_t,
                *const ::core::ffi::c_char,
                fuse_ino_t,
                *const ::core::ffi::c_char,
            ) -> (),
    ),
    rename: Some(
        mfs_rename
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                ::core::ffi::c_uint,
            ) -> (),
    ),
    link: Some(
        mfs_link
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
            ) -> (),
    ),
    open: Some(mfs_open as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> ()),
    read: Some(
        mfs_read
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                size_t,
                off_t,
                *mut fuse_file_info,
            ) -> (),
    ),
    write: Some(
        mfs_write
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                size_t,
                off_t,
                *mut fuse_file_info,
            ) -> (),
    ),
    flush: Some(
        mfs_flush as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    release: Some(
        mfs_release as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    fsync: Some(
        mfs_fsync
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                ::core::ffi::c_int,
                *mut fuse_file_info,
            ) -> (),
    ),
    opendir: Some(
        mfs_opendir as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    readdir: Some(
        mfs_readdir
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                size_t,
                off_t,
                *mut fuse_file_info,
            ) -> (),
    ),
    releasedir: Some(
        mfs_releasedir as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info) -> (),
    ),
    fsyncdir: None,
    statfs: Some(mfs_statfs as unsafe extern "C" fn(fuse_req_t, fuse_ino_t) -> ()),
    setxattr: Some(
        mfs_setxattr
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                size_t,
                ::core::ffi::c_int,
            ) -> (),
    ),
    getxattr: Some(
        mfs_getxattr
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                size_t,
            ) -> (),
    ),
    listxattr: Some(mfs_listxattr as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, size_t) -> ()),
    removexattr: Some(
        mfs_removexattr
            as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *const ::core::ffi::c_char) -> (),
    ),
    access: Some(
        mfs_access as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, ::core::ffi::c_int) -> (),
    ),
    create: Some(
        mfs_create
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *const ::core::ffi::c_char,
                mode_t,
                *mut fuse_file_info,
            ) -> (),
    ),
    getlk: Some(
        mfs_getlk
            as unsafe extern "C" fn(fuse_req_t, fuse_ino_t, *mut fuse_file_info, *mut flock) -> (),
    ),
    setlk: Some(
        mfs_setlk
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *mut fuse_file_info,
                *mut flock,
                ::core::ffi::c_int,
            ) -> (),
    ),
    bmap: None,
    ioctl: None,
    poll: None,
    write_buf: None,
    retrieve_reply: None,
    forget_multi: None,
    flock: Some(
        mfs_flock
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                *mut fuse_file_info,
                ::core::ffi::c_int,
            ) -> (),
    ),
    fallocate: None,
    readdirplus: Some(
        mfs_readdirplus
            as unsafe extern "C" fn(
                fuse_req_t,
                fuse_ino_t,
                size_t,
                off_t,
                *mut fuse_file_info,
            ) -> (),
    ),
    copy_file_range: None,
    lseek: None,
    tmpfile: None,
    statx: None,
};
static mut mfsopts: mfsopts = mfsopts {
    masterhost: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    masterport: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    bindhost: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    proxyhost: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    subfolder: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    password: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    passfile: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    md5pass: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    preferedlabels: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    nofile: 0,
    nice: 0,
    mfssuid: 0,
    mfsdev: 0,
    memlock: 0,
    limitarenas: 0,
    allowoomkiller: 0,
    nonempty: 0,
    logminlevelstr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    logminlevel: 0,
    logelevatetostr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    logelevateto: 0,
    nostdmountoptions: 0,
    meta: 0,
    debug: 0,
    flattrash: 0,
    delayedinit: 0,
    mkdircopysgid: 0,
    sugidclearmodestr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    sugidclearmode: 0,
    cachemode: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    cachefiles: 0,
    keepcache: 0,
    passwordask: 0,
    noxattrs: 0,
    noposixlocks: 0,
    nobsdlocks: 0,
    donotrememberpassword: 0,
    writecachesize: 0,
    readaheadsize: 0,
    readaheadleng: 0,
    readaheadtrigger: 0,
    erroronlostchunk: 0,
    erroronnospace: 0,
    ioretries: 0,
    timeout: 0,
    logretry: 0,
    readdirplusminto: 0.,
    attrcacheto: 0.,
    xattrcacheto: 0.,
    entrycacheto: 0.,
    direntrycacheto: 0.,
    negentrycacheto: 0.,
    symlinkcacheto: 0.,
    groupscacheto: 0.,
    fsyncmintime: 0.,
    fsyncbeforeclose: 0,
    netdev: 0,
};
static mut defaultmountpoint: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut custom_cfg: ::core::ffi::c_int = 0;
static mut mfs_opts_stage1: [fuse_opt; 3] = [fuse_opt {
    templ: ::core::ptr::null::<::core::ffi::c_char>(),
    offset: 0,
    value: 0,
}; 3];
static mut mfs_opts_stage2: [fuse_opt; 70] = [fuse_opt {
    templ: ::core::ptr::null::<::core::ffi::c_char>(),
    offset: 0,
    value: 0,
}; 70];
unsafe extern "C" fn usage(mut progname: *const ::core::ffi::c_char) {
    unsafe {
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        fd = stdout;
        fprintf(
            fd,
            b"usage: %s [HOST[:PORT]:[PATH]] [options] mountpoint\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            progname,
        );
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"general options:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o opt,[opt...]         mount options\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fuse_cmdline_help();
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"MFS options:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -c CFGFILE                  equivalent to '-o mfscfgfile=CFGFILE'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -m   --meta                 equivalent to '-o mfsmeta'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -H HOST                     equivalent to '-o mfsmaster=HOST'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -P PORT                     equivalent to '-o mfsport=PORT'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -B IP                       equivalent to '-o mfsbind=IP'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -L IP                       equivalent to '-o mfsproxy=IP'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -S PATH                     equivalent to '-o mfssubfolder=PATH'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -p   --password             similar to '-o mfspassword=PASSWORD', but show prompt and ask user for password\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -n   --nostdopts            do not add standard MFS mount options: '-o allow_other,fsname=MFS'\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o nonempty                 allow to mount MFS in nonempty directory\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfscfgfile=CFGFILE       load some mount options from external file (if not specified then use default file: /home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfs/mfsmount.cfg or /home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfsmount.cfg)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsdebug                 print some debugging information\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsmeta                  mount meta filesystem (trash etc.)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsflattrash             use flat trash structure in meta\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsdelayedinit           connection with master is done in background - with this option mount can be run without network (good for being run from fstab / init scripts etc.)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsmkdircopysgid=N       sgid bit should be copied during mkdir operation (default: 1)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfssugidclearmode=SMODE  set sugid clear mode (see below ; default: EXT)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfscachemode=CMODE       set cache mode (see below ; default: AUTO)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfscachefiles            (deprecated) equivalent to '-o mfscachemode=YES'\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsreaddirplusminto=SEC  set minimal entry/attributes cache timeout for readdirplus in seconds (default: 0.0001)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsattrcacheto=SEC       set attributes cache timeout in seconds (default: 1.0)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsxattrcacheto=SEC      set extended attributes (xattr) cache timeout in seconds (default: 30.0)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsentrycacheto=SEC      set file entry cache timeout in seconds (default: 0.0)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsdirentrycacheto=SEC   set directory entry cache timeout in seconds (default: 1.0)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsnegentrycacheto=SEC   set negative entry cache timeout in seconds (default: 0.0)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfssymlinkcacheto=SEC    set symbolic link cache timeout in seconds (default: 300.0)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsgroupscacheto=SEC     set supplementary groups cache timeout in seconds (default: 300.0)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsrlimitnofile=N        on startup mfsmount tries to change number of descriptors it can simultaneously open (default: 100000)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsnice=N                on startup mfsmount tries to change his 'nice' value (default: -19)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsmemlock               try to lock memory\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfslimitarenas=N         if N>0 then limit glibc malloc arenas (default: 4)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsallowoomkiller        do not disable out of memory killer\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfslogminlevel=LEVEL     minimal message level to log ([D]EBUG,[I]NFO,[N]OTICE,[W]ARNING or [E]RROR - default is INFO)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfslogelevateto=LEVEL    send messages with log level lower than LEVEL to syslog as LEVEL (levels as above - default in NOTICE)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsfsyncmintime=SEC      force fsync before last file close when file was opened/created at least SEC seconds earlier (default: 0.0 - always do fsync before close)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfswritecachesize=N      define size of write cache in MiB (default: 256)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsreadaheadsize=N       define size of all read ahead buffers in MiB (default: 256)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsreadaheadleng=N       define amount of bytes to be additionally read (default: 1048576)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsreadaheadtrigger=N    define amount of bytes read sequentially that turns on read ahead (default: 10 * mfsreadaheadleng)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfserroronlostchunk      when all known chunkservers are connected to the master and the required chunk is missing then immediately finish I/O and return an error\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfserroronnospace        when all known chunkservers are connected to the master and there is no free space then immediately finish I/O and return an error\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsioretries=N           define number of retries before I/O error is returned (default: 30)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfstimeout=N             define maximum timeout in seconds before I/O error is returned (default: 0 - which means no timeout)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfslogretry=N            define minimal retry counter on which system will start log I/O messages (default: 5)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsmaster=HOST           define mfsmaster location (default: mfsmaster)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsport=PORT             define mfsmaster port number (default: 9421)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsbind=IP               define source ip address for connections (default: NOT DEFINED - chosen automatically by OS)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsproxy=IP              define listen ip address of local master proxy for communication with tools (default: 127.0.0.1)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfssubfolder=PATH        define subfolder to mount as root (default: /)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfspassword=PASSWORD     authenticate to mfsmaster with given password\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfspassfile=FILENAME     authenticate to mfsmaster with password from given file\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsmd5pass=MD5           authenticate to mfsmaster using directly given md5 (only if mfspassword is not defined)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsdonotrememberpassword do not remember password in memory - more secure, but when session is lost then new session is created without password\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfspreflabels=LABELEXPR  specify preferred labels for choosing chunkservers during I/O\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsnoxattrs              turn off xattr support\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsnoposixlocks          turn off support for global posix locks (lockf + ioctl) - locks will work locally\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    -o mfsnobsdlocks            turn off support for global BSD locks (flock) - locks will work locally\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"CMODE can be set to:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    DIRECT                      forces direct io (bypasses cache)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    NO,NONE or NEVER            never allow files data to be kept in cache (safest but can reduce efficiency)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    YES or ALWAYS               always allow files data to be kept in cache (dangerous)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    AUTO                        file cache is managed by mfsmaster automatically (should be very safe and efficient)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"SMODE can be set to:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    NEVER                       MFS will not change suid and sgid bit on chown\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    ALWAYS                      clear suid and sgid on every chown - safest operation\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    OSX                         standard behavior in OS X and Solaris (chown made by unprivileged user clear suid and sgid)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    BSD                         standard behavior in *BSD systems (like in OSX, but only when something is really changed)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    EXT                         standard behavior in most file systems on Linux (directories not changed, others: suid cleared always, sgid only when group exec bit is set)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    XFS                         standard behavior in XFS on Linux (like EXT but directories are changed by unprivileged users)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"SMODE extra info:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    btrfs,ext2,ext3,ext4,hfs[+],jfs,ntfs and reiserfs on Linux work as 'EXT'.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    Only xfs on Linux works a little different. Beware that there is a strange\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    operation - chown(-1,-1) which is usually converted by a kernel into something\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    like 'chmod ug-s', and therefore can't be controlled by MFS as 'chown'\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"LABELEXPR grammar:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    LABELEXPR -> S ';' LABELEXPR | S\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    S -> S '+' M | M\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    M -> M L | L\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    L -> 'a' .. 'z' | 'A' .. 'Z' | '(' S ')' | '[' S ']'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"    Subexpressions should be placed in priority order.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"    Up to nine subexpressions (priorities) can be specified.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn mfs_opt_parse_cfg_file(
    mut filename: *const ::core::ffi::c_char,
    mut optional: ::core::ffi::c_int,
    mut outargs: *mut fuse_args,
) {
    unsafe {
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut lbuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lbsize: size_t = 0;
        fd = fopen(filename, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        if fd.is_null() {
            if optional == 0 as ::core::ffi::c_int {
                if *__errno_location() == ENOENT {
                    fprintf(
                        stderr,
                        b"cfg file (%s) doesn't exist\n\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"can't open cfg file (%s), error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        filename,
                        strerr(*__errno_location()),
                    );
                }
                abort();
            }
            return;
        }
        lbsize = 1000 as size_t;
        lbuff = malloc(lbsize) as *mut ::core::ffi::c_char;
        custom_cfg = 1 as ::core::ffi::c_int;
        while getline(&raw mut lbuff, &raw mut lbsize, fd) != -1 as __ssize_t {
            if *lbuff.offset(0 as isize) as ::core::ffi::c_int != '#' as ::core::ffi::c_int
                && *lbuff.offset(0 as isize) as ::core::ffi::c_int != ';' as ::core::ffi::c_int
            {
                *lbuff.offset(999 as isize) = 0 as ::core::ffi::c_char;
                p = lbuff;
                while *p != 0 {
                    if *p as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                        || *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                    {
                        *p = 0 as ::core::ffi::c_char;
                        break;
                    } else {
                        p = p.offset(1);
                    }
                }
                p = p.offset(-1);
                while p >= lbuff
                    && (*p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
                {
                    *p = 0 as ::core::ffi::c_char;
                    p = p.offset(-1);
                }
                p = lbuff;
                while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    p = p.offset(1);
                }
                if *p != 0 {
                    if *p as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
                        fuse_opt_add_arg(outargs, p);
                    } else if *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                        if !defaultmountpoint.is_null() {
                            free(defaultmountpoint as *mut ::core::ffi::c_void);
                        }
                        defaultmountpoint = strdup(p);
                    } else {
                        fuse_opt_add_arg(outargs, b"-o\0".as_ptr() as *const ::core::ffi::c_char);
                        fuse_opt_add_arg(outargs, p);
                    }
                }
            }
        }
        free(lbuff as *mut ::core::ffi::c_void);
        fclose(fd);
    }
}
unsafe extern "C" fn mfs_opt_proc_stage1(
    mut data: *mut ::core::ffi::c_void,
    mut arg: *const ::core::ffi::c_char,
    mut key: ::core::ffi::c_int,
    mut outargs: *mut fuse_args,
) -> ::core::ffi::c_int {
    unsafe {
        let mut defargs: *mut fuse_args = data as *mut fuse_args;
        if key == KEY_CFGFILE as ::core::ffi::c_int {
            if memcmp(
                arg as *const ::core::ffi::c_void,
                b"mfscfgfile=\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                11 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                mfs_opt_parse_cfg_file(
                    arg.offset(11 as ::core::ffi::c_int as isize),
                    0 as ::core::ffi::c_int,
                    defargs,
                );
            } else if *arg.offset(0 as isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
                && *arg.offset(1 as isize) as ::core::ffi::c_int == 'c' as ::core::ffi::c_int
            {
                mfs_opt_parse_cfg_file(
                    arg.offset(2 as ::core::ffi::c_int as isize),
                    0 as ::core::ffi::c_int,
                    defargs,
                );
            }
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn mfs_opt_proc_stage2(
    mut data: *mut ::core::ffi::c_void,
    mut arg: *const ::core::ffi::c_char,
    mut key: ::core::ffi::c_int,
    mut outargs: *mut fuse_args,
) -> ::core::ffi::c_int {
    unsafe {
        match key {
            FUSE_OPT_KEY_OPT => return 1 as ::core::ffi::c_int,
            FUSE_OPT_KEY_NONOPT => return 1 as ::core::ffi::c_int,
            2 => {
                if !mfsopts.masterhost.is_null() {
                    free(mfsopts.masterhost as *mut ::core::ffi::c_void);
                }
                mfsopts.masterhost = strdup(arg.offset(2 as ::core::ffi::c_int as isize));
                return 0 as ::core::ffi::c_int;
            }
            3 => {
                if !mfsopts.masterport.is_null() {
                    free(mfsopts.masterport as *mut ::core::ffi::c_void);
                }
                mfsopts.masterport = strdup(arg.offset(2 as ::core::ffi::c_int as isize));
                return 0 as ::core::ffi::c_int;
            }
            4 => {
                if !mfsopts.bindhost.is_null() {
                    free(mfsopts.bindhost as *mut ::core::ffi::c_void);
                }
                mfsopts.bindhost = strdup(arg.offset(2 as ::core::ffi::c_int as isize));
                return 0 as ::core::ffi::c_int;
            }
            5 => {
                if !mfsopts.proxyhost.is_null() {
                    free(mfsopts.proxyhost as *mut ::core::ffi::c_void);
                }
                mfsopts.proxyhost = strdup(arg.offset(2 as ::core::ffi::c_int as isize));
                return 0 as ::core::ffi::c_int;
            }
            6 => {
                if !mfsopts.subfolder.is_null() {
                    free(mfsopts.subfolder as *mut ::core::ffi::c_void);
                }
                mfsopts.subfolder = strdup(arg.offset(2 as ::core::ffi::c_int as isize));
                return 0 as ::core::ffi::c_int;
            }
            7 => {
                mfsopts.passwordask = 1 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
            1 => {
                mfsopts.meta = 1 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
            8 => {
                mfsopts.nostdmountoptions = 1 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
            10 => {
                fprintf(
                    stderr,
                    b"MFS version: %s ; build: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    VERSSTR.as_ptr(),
                    b"2106\0".as_ptr() as *const ::core::ffi::c_char,
                );
                fuse_lowlevel_version();
                exit(0 as ::core::ffi::c_int);
            }
            9 => {
                usage(*(*outargs).argv.offset(0 as isize));
                printf(b"fuse lowlevel options:\n\0".as_ptr() as *const ::core::ffi::c_char);
                fuse_lowlevel_help();
                exit(1 as ::core::ffi::c_int);
            }
            _ => {
                fprintf(
                    stderr,
                    b"internal error\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            }
        };
    }
}
static mut fuse_init_set: uint8_t = 0 as uint8_t;
static mut fuse_proto_major: uint32_t = 0 as uint32_t;
static mut fuse_proto_minor: uint32_t = 0 as uint32_t;
static mut fuse_capable: uint32_t = 0 as uint32_t;
static mut fuse_defaults: uint32_t = 0 as uint32_t;
static mut fuse_want: uint32_t = 0 as uint32_t;
unsafe extern "C" fn mfs_fsinit(
    mut userdata: *mut ::core::ffi::c_void,
    mut conn: *mut fuse_conn_info,
) {
    unsafe {
        let mut piped: *mut ::core::ffi::c_int = userdata as *mut ::core::ffi::c_int;
        let mut s: ::core::ffi::c_char = 0;
        fuse_defaults = (*conn).want;
        (*conn).max_read = 0 as uint32_t;
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_ASYNC_READ) as uint32_t;
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_ATOMIC_O_TRUNC) as uint32_t;
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_EXPORT_SUPPORT) as uint32_t;
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_DONT_MASK) as uint32_t;
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_SPLICE_WRITE) as uint32_t;
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_SPLICE_MOVE) as uint32_t;
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_SPLICE_READ) as uint32_t;
        (*conn).want =
            ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_AUTO_INVAL_DATA) as uint32_t;
        if mfsopts.meta != 0 {
            (*conn).want =
                ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_READDIRPLUS) as uint32_t;
            (*conn).want =
                ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_READDIRPLUS_AUTO) as uint32_t;
        } else {
            (*conn).want =
                ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_READDIRPLUS) as uint32_t;
            (*conn).want =
                ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_READDIRPLUS_AUTO) as uint32_t;
        }
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_ASYNC_DIO) as uint32_t;
        (*conn).want =
            ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_WRITEBACK_CACHE) as uint32_t;
        (*conn).want =
            ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_PARALLEL_DIROPS) as uint32_t;
        (*conn).want = ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_POSIX_ACL) as uint32_t;
        (*conn).want =
            ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_HANDLE_KILLPRIV) as uint32_t;
        if mfsopts.nobsdlocks == 0 as ::core::ffi::c_int {
            (*conn).want =
                ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_FLOCK_LOCKS) as uint32_t;
        } else {
            (*conn).want =
                ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_FLOCK_LOCKS) as uint32_t;
        }
        if mfsopts.noposixlocks == 0 as ::core::ffi::c_int {
            (*conn).want =
                ((*conn).want as ::core::ffi::c_ulong | FUSE_CAP_POSIX_LOCKS) as uint32_t;
        } else {
            (*conn).want =
                ((*conn).want as ::core::ffi::c_ulong & !FUSE_CAP_POSIX_LOCKS) as uint32_t;
        }
        (*conn).want &= (*conn).capable;
        (*conn).time_gran = 1000000000 as uint32_t;
        fuse_proto_major = (*conn).proto_major;
        fuse_proto_minor = (*conn).proto_minor;
        fuse_capable = (*conn).capable;
        fuse_want = (*conn).want;
        fuse_init_set = 1 as uint8_t;
        fs_set_working_flags(WFLAG_INVALIDATE_CACHE as uint8_t);
        if *piped.offset(1 as isize) >= 0 as ::core::ffi::c_int {
            s = 0 as ::core::ffi::c_char;
            if write(
                *piped.offset(1 as isize),
                &raw mut s as *const ::core::ffi::c_void,
                1 as size_t,
            ) != 1 as ssize_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"pipe write error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
            }
            close(*piped.offset(1 as isize));
        }
    }
}
static mut params_sesflags: uint8_t = 0 as uint8_t;
static mut params_umaskval: uint16_t = 0 as uint16_t;
static mut params_maprootuid: uint32_t = 0 as uint32_t;
static mut params_maprootgid: uint32_t = 0 as uint32_t;
static mut params_mapalluid: uint32_t = 0 as uint32_t;
static mut params_mapallgid: uint32_t = 0 as uint32_t;
static mut params_sclassgroups: int32_t = -1 as int32_t;
static mut params_mingoal: uint8_t = 0 as uint8_t;
static mut params_maxgoal: uint8_t = 0 as uint8_t;
static mut params_mintrashretention: uint32_t = 0 as uint32_t;
static mut params_maxtrashretention: uint32_t = 0 as uint32_t;
static mut params_disables: uint32_t = 0 as uint32_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_setparams(
    mut sesflags: uint8_t,
    mut umaskval: uint16_t,
    mut maprootuid: uint32_t,
    mut maprootgid: uint32_t,
    mut mapalluid: uint32_t,
    mut mapallgid: uint32_t,
    mut sclassgroups: int32_t,
    mut mingoal: uint8_t,
    mut maxgoal: uint8_t,
    mut mintrashretention: uint32_t,
    mut maxtrashretention: uint32_t,
    mut disables: uint32_t,
) {
    unsafe {
        params_sesflags = sesflags;
        params_umaskval = umaskval;
        params_maprootuid = maprootuid;
        params_maprootgid = maprootgid;
        params_mapalluid = mapalluid;
        params_mapallgid = mapallgid;
        params_sclassgroups = sclassgroups;
        params_mingoal = mingoal;
        params_maxgoal = maxgoal;
        params_mintrashretention = mintrashretention;
        params_maxtrashretention = maxtrashretention;
        params_disables = disables;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_snprint_parameters(
    mut buff: *mut ::core::ffi::c_char,
    mut size: uint32_t,
) -> uint32_t {
    unsafe {
        let mut leng: uint32_t = 0 as uint32_t;
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsmaster: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.masterhost.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.masterhost as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"working_masterip: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                fs_get_current_masterstrip(),
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsport: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.masterport.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.masterport as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"working_masterport: %hu\n\0".as_ptr() as *const ::core::ffi::c_char,
                fs_get_current_masterport() as ::core::ffi::c_int,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsbind: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.bindhost.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.bindhost as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"working_bindip: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                fs_get_current_srcstrip(),
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsproxy: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.proxyhost.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.proxyhost as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfssubfolder: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.subfolder.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.subfolder as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfspassword: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.password.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.password as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfspassfile: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.passfile.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.passfile as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsmd5pass: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.md5pass.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.md5pass as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfspreflabels: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.preferedlabels.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.preferedlabels as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsrlimitnofile: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.nofile,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsnice: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.nice,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfssuid: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.mfssuid != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsdev: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.mfsdev != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsmemlock: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.memlock != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfslimitarenas: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.limitarenas,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsallowoomkiller: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.allowoomkiller != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfslogminlevel: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.logminlevel == 0 as ::core::ffi::c_int {
                    b"DEBUG\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.logminlevel == 1 as ::core::ffi::c_int {
                    b"INFO\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.logminlevel == 2 as ::core::ffi::c_int {
                    b"NOTICE\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.logminlevel == 3 as ::core::ffi::c_int {
                    b"WARNING\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.logminlevel == 4 as ::core::ffi::c_int {
                    b"ERROR\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"(unknown value)\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfslogelevateto: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.logelevateto == 0 as ::core::ffi::c_int {
                    b"DEBUG\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.logelevateto == 1 as ::core::ffi::c_int {
                    b"INFO\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.logelevateto == 2 as ::core::ffi::c_int {
                    b"NOTICE\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.logelevateto == 3 as ::core::ffi::c_int {
                    b"WARNING\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.logelevateto == 4 as ::core::ffi::c_int {
                    b"ERROR\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"(unknown value)\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"nonempty: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.nonempty != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfswritecachesize: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.writecachesize,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsreadaheadsize: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.readaheadsize,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsreadaheadleng: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.readaheadleng,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsreadaheadtrigger: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.readaheadtrigger,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfserroronlostchunk: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.erroronlostchunk != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfserroronnospace: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.erroronnospace != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsioretries: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.ioretries,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfstimeout: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.timeout,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfslogretry: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.logretry,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsdebug: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.debug != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsmeta: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.meta != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsflattrash: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.flattrash != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsdelayedinit: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.delayedinit != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsdonotrememberpassword: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.donotrememberpassword != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfscachefiles: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.cachefiles != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsnoxattrs: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.noxattrs != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsnoposixlocks: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.noposixlocks != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsnobsdlocks: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.nobsdlocks != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsmkdircopysgid: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.mkdircopysgid,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsreaddirplusminto: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.readdirplusminto,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsattrcacheto: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.attrcacheto,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsxattrcacheto: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.xattrcacheto,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsentrycacheto: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.entrycacheto,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsdirentrycacheto: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.direntrycacheto,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsnegentrycacheto: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.negentrycacheto,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfssymlinkcacheto: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.symlinkcacheto,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsgroupscacheto: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.groupscacheto,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsfsyncmintime: %.3lf\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.fsyncmintime,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfsfsyncbeforeclose: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.fsyncbeforeclose != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfscachemode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.cachemode.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.cachemode as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"working_keep_cache_mode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.keepcache == 0 as ::core::ffi::c_int {
                    b"AUTO\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.keepcache == 1 as ::core::ffi::c_int {
                    b"YES\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.keepcache == 2 as ::core::ffi::c_int {
                    b"NO\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.keepcache == 3 as ::core::ffi::c_int {
                    b"DIRECT\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.keepcache == 4 as ::core::ffi::c_int {
                    b"FBSDAUTO\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"(unknown value)\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"mfssugidclearmode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.sugidclearmodestr.is_null() {
                    b"(not defined)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    mfsopts.sugidclearmodestr as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"working_sugid_clear_mode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.sugidclearmode == 0 as ::core::ffi::c_int {
                    b"NEVER\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.sugidclearmode == 1 as ::core::ffi::c_int {
                    b"ALWAYS\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.sugidclearmode == 2 as ::core::ffi::c_int {
                    b"OSX\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.sugidclearmode == 3 as ::core::ffi::c_int {
                    b"BSD\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.sugidclearmode == 4 as ::core::ffi::c_int {
                    b"EXT\0".as_ptr() as *const ::core::ffi::c_char
                } else if mfsopts.sugidclearmode == 5 as ::core::ffi::c_int {
                    b"XFS\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"(unknown value)\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"no_std_mount_options: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.nostdmountoptions != 0 {
                    b"TRUE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"FALSE\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"master_sesflags: %hhu\n\0".as_ptr() as *const ::core::ffi::c_char,
                params_sesflags as ::core::ffi::c_int,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"master_umaskval: 0%03ho\n\0".as_ptr() as *const ::core::ffi::c_char,
                params_umaskval as ::core::ffi::c_int,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"master_maproot: %u:%u\n\0".as_ptr() as *const ::core::ffi::c_char,
                params_maprootuid,
                params_maprootgid,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"master_mapall: %u:%u\n\0".as_ptr() as *const ::core::ffi::c_char,
                params_mapalluid,
                params_mapallgid,
            ) as uint32_t);
        }
        if params_sclassgroups >= 0 as int32_t {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"master_sclassgroups: 0x04%hX\n\0".as_ptr() as *const ::core::ffi::c_char,
                    params_sclassgroups as uint16_t as ::core::ffi::c_int,
                ) as uint32_t);
            }
        } else if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"master_goallimit: %hhu:%hhu\n\0".as_ptr() as *const ::core::ffi::c_char,
                params_mingoal as ::core::ffi::c_int,
                params_maxgoal as ::core::ffi::c_int,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"master_trashlimit: %u:%u\n\0".as_ptr() as *const ::core::ffi::c_char,
                params_mintrashretention,
                params_maxtrashretention,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"master_disables: 0x08%X\n\0".as_ptr() as *const ::core::ffi::c_char,
                params_disables,
            ) as uint32_t);
        }
        let mut mver: uint32_t = master_version();
        if mver > 0 as uint32_t {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"moosefs_master_version: %u.%u.%u%s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mver >> 16 as ::core::ffi::c_int,
                    mver >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                    mver >> 1 as ::core::ffi::c_int & 0x7f as uint32_t,
                    if mver & 1 as uint32_t != 0 {
                        b"-pro\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                ) as uint32_t);
            }
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"moosefs_mount_version: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"4.59.2-1\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"moosefs_mount_build: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                2106 as ::core::ffi::c_int,
            ) as uint32_t);
        }
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"compiled_with_fuse: %u.%u\n\0".as_ptr() as *const ::core::ffi::c_char,
                (3 as ::core::ffi::c_int * 100 as ::core::ffi::c_int + 18 as ::core::ffi::c_int)
                    / 10 as ::core::ffi::c_int,
                (3 as ::core::ffi::c_int * 100 as ::core::ffi::c_int + 18 as ::core::ffi::c_int)
                    % 10 as ::core::ffi::c_int,
            ) as uint32_t);
        }
        let mut libver: ::core::ffi::c_int = fuse_version();
        if leng < size {
            leng = leng.wrapping_add(snprintf(
                buff.offset(leng as isize),
                size.wrapping_sub(leng) as size_t,
                b"fuse_library_version: %d.%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                libver / 10 as ::core::ffi::c_int,
                libver % 10 as ::core::ffi::c_int,
            ) as uint32_t);
        }
        if fuse_init_set != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"kernel_fuse_protocol: %u.%u\n\0".as_ptr() as *const ::core::ffi::c_char,
                    fuse_proto_major,
                    fuse_proto_minor,
                ) as uint32_t);
            }
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"kernel_capability_mask: 0x%X\n\0".as_ptr() as *const ::core::ffi::c_char,
                    fuse_capable,
                ) as uint32_t);
            }
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"kernel_defaults_mask: 0x%X\n\0".as_ptr() as *const ::core::ffi::c_char,
                    fuse_defaults,
                ) as uint32_t);
            }
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"kernel_working_mask: 0x%X\n\0".as_ptr() as *const ::core::ffi::c_char,
                    fuse_want,
                ) as uint32_t);
            }
        }
        return leng;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_kernelversion() -> uint32_t {
    unsafe {
        let mut maj: uint32_t = 0;
        let mut min: uint32_t = 0;
        let mut r: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut utsn: utsname = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };
        maj = 0 as uint32_t;
        min = 0 as uint32_t;
        if uname(&raw mut utsn) < 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"uname error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                strerr(*__errno_location()),
            );
            return 0 as uint32_t;
        }
        r = &raw mut utsn.release as *mut ::core::ffi::c_char;
        if r.is_null() {
            fprintf(
                stderr,
                b"uname error: (release is NULL)\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as uint32_t;
        }
        while *r as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *r as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            maj = maj.wrapping_mul(10 as uint32_t);
            maj = maj
                .wrapping_add((*r as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t);
            r = r.offset(1);
        }
        if *r as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            r = r.offset(1);
            while *r as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *r as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                min = min.wrapping_mul(10 as uint32_t);
                min = min.wrapping_add(
                    (*r as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                );
                r = r.offset(1);
            }
        }
        if maj > 0xffff as uint32_t {
            maj = 0xffff as uint32_t;
        }
        if min > 0xffff as uint32_t {
            min = 0xffff as uint32_t;
        }
        return maj.wrapping_mul(0x10000 as uint32_t).wrapping_add(min);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop(
    mut args: *mut fuse_args,
    mut cmdopts: *mut fuse_cmdline_opts,
) -> ::core::ffi::c_int {
    unsafe {
        let mut se: *mut fuse_session = ::core::ptr::null_mut::<fuse_session>();
        let mut rls: rlimit = rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        let mut piped: [::core::ffi::c_int; 2] = [0; 2];
        let mut s: ::core::ffi::c_char = 0;
        let mut err: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut ctx: md5ctx = md5ctx {
            state: [0; 4],
            count: [0; 2],
            buffer: [0; 64],
        };
        let mut md5pass: [uint8_t; 16] = [0; 16];
        if mfsopts.passwordask != 0 && mfsopts.password.is_null() && mfsopts.md5pass.is_null() {
            mfsopts.password = getpass(b"MFS Password:\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if !mfsopts.password.is_null() {
            md5_init(&raw mut ctx);
            md5_update(
                &raw mut ctx,
                mfsopts.password as *mut uint8_t,
                strlen(mfsopts.password) as uint32_t,
            );
            md5_final(&raw mut md5pass as *mut uint8_t, &raw mut ctx);
            memset(
                mfsopts.password as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                strlen(mfsopts.password),
            );
        } else if !mfsopts.md5pass.is_null() {
            let mut p: *mut uint8_t = mfsopts.md5pass as *mut uint8_t;
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                if *p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    md5pass[i as usize] = ((*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        << 4 as ::core::ffi::c_int)
                        as uint8_t;
                } else if *p as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
                {
                    md5pass[i as usize] = ((*p as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int)
                        << 4 as ::core::ffi::c_int)
                        as uint8_t;
                } else if *p as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    md5pass[i as usize] = ((*p as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int)
                        << 4 as ::core::ffi::c_int)
                        as uint8_t;
                } else {
                    fprintf(
                        stderr,
                        b"bad md5 definition (md5 should be given as 32 hex digits)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return 1 as ::core::ffi::c_int;
                }
                p = p.offset(1);
                if *p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    md5pass[i as usize] = (md5pass[i as usize] as ::core::ffi::c_int
                        + (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
                        as uint8_t;
                } else if *p as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
                {
                    md5pass[i as usize] = (md5pass[i as usize] as ::core::ffi::c_int
                        + (*p as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int))
                        as uint8_t;
                } else if *p as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    md5pass[i as usize] = (md5pass[i as usize] as ::core::ffi::c_int
                        + (*p as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int))
                        as uint8_t;
                } else {
                    fprintf(
                        stderr,
                        b"bad md5 definition (md5 should be given as 32 hex digits)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return 1 as ::core::ffi::c_int;
                }
                p = p.offset(1);
                i += 1;
            }
            if *p != 0 {
                fprintf(
                    stderr,
                    b"bad md5 definition (md5 should be given as 32 hex digits)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return 1 as ::core::ffi::c_int;
            }
            memset(
                mfsopts.md5pass as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                strlen(mfsopts.md5pass),
            );
        }
        if mfsopts.delayedinit != 0 {
            fs_init_master_connection(
                mfsopts.bindhost,
                mfsopts.masterhost,
                mfsopts.masterport,
                mfsopts.meta as uint8_t,
                (*cmdopts).mountpoint,
                mfsopts.subfolder,
                (if !mfsopts.password.is_null() || !mfsopts.md5pass.is_null() {
                    &raw mut md5pass as *mut uint8_t
                } else {
                    ::core::ptr::null_mut::<uint8_t>()
                }) as *const uint8_t,
                mfsopts.donotrememberpassword as uint8_t,
                1 as uint8_t,
                0 as uint32_t,
            );
        } else if fs_init_master_connection(
            mfsopts.bindhost,
            mfsopts.masterhost,
            mfsopts.masterport,
            mfsopts.meta as uint8_t,
            (*cmdopts).mountpoint,
            mfsopts.subfolder,
            (if !mfsopts.password.is_null() || !mfsopts.md5pass.is_null() {
                &raw mut md5pass as *mut uint8_t
            } else {
                ::core::ptr::null_mut::<uint8_t>()
            }) as *const uint8_t,
            mfsopts.donotrememberpassword as uint8_t,
            0 as uint8_t,
            0 as uint32_t,
        ) < 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        memset(
            &raw mut md5pass as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
        mfs_log_init(
            b"mfsmount\0".as_ptr() as *const ::core::ffi::c_char,
            if (*cmdopts).foreground == 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
        mfs_log_set_min_level(mfsopts.logminlevel);
        mfs_log_set_elevate_to(mfsopts.logelevateto);
        i = mfsopts.nofile as ::core::ffi::c_int;
        loop {
            rls.rlim_cur = i as rlim_t;
            rls.rlim_max = i as rlim_t;
            if setrlimit(RLIMIT_NOFILE, &raw mut rls) >= 0 as ::core::ffi::c_int {
                break;
            }
            i /= 2 as ::core::ffi::c_int;
            if i < 1000 as ::core::ffi::c_int {
                break;
            }
        }
        if i != mfsopts.nofile as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"can't set open file limit to %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                mfsopts.nofile,
            );
            if i >= 1000 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"open file limit set to: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                    i,
                );
            }
        }
        setpriority(PRIO_PROCESS, getpid() as id_t, mfsopts.nice);
        if mfsopts.memlock != 0 {
            rls.rlim_cur = RLIM_INFINITY as rlim_t;
            rls.rlim_max = RLIM_INFINITY as rlim_t;
            if setrlimit(__RLIMIT_MEMLOCK, &raw mut rls) < 0 as ::core::ffi::c_int {
                mfsopts.memlock = 0 as ::core::ffi::c_int;
            }
        }
        piped[1 as usize] = -1 as ::core::ffi::c_int;
        piped[0 as usize] = piped[1 as usize];
        if (*cmdopts).foreground == 0 as ::core::ffi::c_int {
            if pipe(&raw mut piped as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"pipe error\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return 1 as ::core::ffi::c_int;
            }
            err = fork() as ::core::ffi::c_int;
            if err < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"fork error\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return 1 as ::core::ffi::c_int;
            } else if err > 0 as ::core::ffi::c_int {
                close(piped[1 as usize]);
                err = read(
                    piped[0 as usize],
                    &raw mut s as *mut ::core::ffi::c_void,
                    1 as size_t,
                ) as ::core::ffi::c_int;
                if err == 0 as ::core::ffi::c_int {
                    s = 1 as ::core::ffi::c_char;
                }
                return s as ::core::ffi::c_int;
            }
            close(piped[0 as usize]);
            s = 1 as ::core::ffi::c_char;
            if chdir(b"/\0".as_ptr() as *const ::core::ffi::c_char) < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"chdir error\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
        if mfsopts.memlock != 0 {
            if mlockall(MCL_CURRENT | MCL_FUTURE) == 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"process memory was successfully locked in RAM\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if mfsopts.limitarenas != 0 {
            if getenv(b"MALLOC_ARENA_MAX\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"setting glibc malloc arena max to %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mfsopts.limitarenas,
                );
                mallopt(M_ARENA_MAX, mfsopts.limitarenas);
            }
            if getenv(b"MALLOC_ARENA_TEST\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"setting glibc malloc arena test to %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mfsopts.limitarenas,
                );
                mallopt(M_ARENA_TEST, mfsopts.limitarenas);
            }
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"setting glibc malloc arenas turned off\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if mfsopts.allowoomkiller == 0 as ::core::ffi::c_int {
            let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
            let mut dis: ::core::ffi::c_int = 0;
            dis = 0 as ::core::ffi::c_int;
            fd = fopen(
                b"/proc/self/oom_score_adj\0".as_ptr() as *const ::core::ffi::c_char,
                b"w\0".as_ptr() as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if !fd.is_null() {
                fprintf(
                    fd,
                    b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                    OOM_SCORE_ADJ_MIN,
                );
                fclose(fd);
                dis = 1 as ::core::ffi::c_int;
            } else {
                fd = fopen(
                    b"/proc/self/oom_adj\0".as_ptr() as *const ::core::ffi::c_char,
                    b"w\0".as_ptr() as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if !fd.is_null() {
                    fprintf(
                        fd,
                        b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        OOM_DISABLE,
                    );
                    fclose(fd);
                    dis = 1 as ::core::ffi::c_int;
                }
            }
            if dis != 0 {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"out of memory killer disabled\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't disable out of memory killer\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"monotonic clock function: %s\0".as_ptr() as *const ::core::ffi::c_char,
            monotonic_method(),
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"monotonic clock speed: %u ops / 10 mili seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            monotonic_speed(),
        );
        inoleng_init();
        conncache_init(200 as uint32_t);
        chunkrwlock_init();
        chunksdatacache_init();
        symlink_cache_init(mfsopts.symlinkcacheto);
        negentry_cache_init(mfsopts.negentrycacheto);
        read_init();
        write_init();
        fs_init_threads(mfsopts.ioretries as uint32_t, mfsopts.timeout as uint32_t);
        if masterproxy_init(mfsopts.proxyhost) < 0 as ::core::ffi::c_int {
            err = 1 as ::core::ffi::c_int;
        } else {
            if mfsopts.meta != 0 {
                mfs_meta_init(
                    mfsopts.debug,
                    mfsopts.entrycacheto,
                    mfsopts.attrcacheto,
                    mfsopts.flattrash,
                );
                se = fuse_session_new_fn(
                    args,
                    &raw mut mfs_meta_oper,
                    ::core::mem::size_of::<fuse_lowlevel_ops>(),
                    &raw mut piped as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
                );
            } else {
                csdb_init();
                delay_init();
                read_data_init(
                    mfsopts
                        .readaheadsize
                        .wrapping_mul(1024 as ::core::ffi::c_uint)
                        .wrapping_mul(1024 as ::core::ffi::c_uint) as uint64_t,
                    mfsopts.readaheadleng as uint32_t,
                    mfsopts.readaheadtrigger as uint32_t,
                    mfsopts.ioretries as uint32_t,
                    mfsopts.timeout as uint32_t,
                    mfsopts.logretry as uint32_t,
                    mfsopts.erroronlostchunk as uint8_t,
                    mfsopts.erroronnospace as uint8_t,
                );
                write_data_init(
                    (mfsopts.writecachesize as uint32_t)
                        .wrapping_mul(1024 as uint32_t)
                        .wrapping_mul(1024 as uint32_t),
                    mfsopts.ioretries as uint32_t,
                    mfsopts.timeout as uint32_t,
                    mfsopts.logretry as uint32_t,
                    mfsopts.erroronlostchunk as uint8_t,
                    mfsopts.erroronnospace as uint8_t,
                );
                mfs_init(
                    mfsopts.debug,
                    mfsopts.keepcache,
                    mfsopts.readdirplusminto,
                    mfsopts.direntrycacheto,
                    mfsopts.entrycacheto,
                    mfsopts.attrcacheto,
                    mfsopts.xattrcacheto,
                    mfsopts.groupscacheto,
                    mfsopts.mkdircopysgid as ::core::ffi::c_int,
                    mfsopts.sugidclearmode,
                    1 as ::core::ffi::c_int,
                    mfsopts.fsyncmintime,
                    mfsopts.noxattrs,
                    mfsopts.noposixlocks,
                    mfsopts.nobsdlocks,
                );
                se = fuse_session_new_fn(
                    args,
                    &raw mut mfs_oper,
                    ::core::mem::size_of::<fuse_lowlevel_ops>(),
                    &raw mut piped as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
                );
                mfs_setsession(se);
            }
            if se.is_null() {
                fprintf(
                    stderr,
                    b"error in fuse_session_new\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                portable_usleep(100000 as uint64_t);
                if piped[1 as usize] >= 0 as ::core::ffi::c_int {
                    if write(
                        piped[1 as usize],
                        &raw mut s as *const ::core::ffi::c_void,
                        1 as size_t,
                    ) != 1 as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"pipe write error\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    close(piped[1 as usize]);
                }
                err = 1 as ::core::ffi::c_int;
            } else if fuse_set_signal_handlers(se) < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"error in fuse_set_signal_handlers\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                fuse_session_destroy(se);
                if piped[1 as usize] >= 0 as ::core::ffi::c_int {
                    if write(
                        piped[1 as usize],
                        &raw mut s as *const ::core::ffi::c_void,
                        1 as size_t,
                    ) != 1 as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"pipe write error\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    close(piped[1 as usize]);
                }
                err = 1 as ::core::ffi::c_int;
            } else if fuse_session_mount(se, (*cmdopts).mountpoint) < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"error in fuse_session_mount\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                fuse_session_destroy(se);
                if piped[1 as usize] >= 0 as ::core::ffi::c_int {
                    if write(
                        piped[1 as usize],
                        &raw mut s as *const ::core::ffi::c_void,
                        1 as size_t,
                    ) != 1 as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"pipe write error\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    close(piped[1 as usize]);
                }
                err = 1 as ::core::ffi::c_int;
            } else {
                if mfsopts.debug == 0 as ::core::ffi::c_int
                    && (*cmdopts).foreground == 0 as ::core::ffi::c_int
                {
                    setsid();
                    setpgid(0 as __pid_t, getpid());
                    i = open(
                        b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
                        O_RDWR,
                        0 as ::core::ffi::c_int,
                    );
                    if i != -1 as ::core::ffi::c_int {
                        dup2(i, STDIN_FILENO);
                        dup2(i, STDOUT_FILENO);
                        dup2(i, STDERR_FILENO);
                        if i > 2 as ::core::ffi::c_int {
                            close(i);
                        }
                    }
                }
                sparents_init();
                sinodes_init((*cmdopts).mountpoint);
                sstats_init();
                let mut pname: [::core::ffi::c_char; 256] = [0; 256];
                snprintf(
                    &raw mut pname as *mut ::core::ffi::c_char,
                    256 as size_t,
                    b"mfsmount (mounted on: %s)\0".as_ptr() as *const ::core::ffi::c_char,
                    (*cmdopts).mountpoint,
                );
                pname[255 as usize] = 0 as ::core::ffi::c_char;
                processname_set(&raw mut pname as *mut ::core::ffi::c_char);
                if (*cmdopts).singlethread == 0 as ::core::ffi::c_int {
                    let mut lopts: fuse_loop_config = fuse_loop_config {
                        clone_fd: 0,
                        max_idle_threads: 0,
                    };
                    memset(
                        &raw mut lopts as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<fuse_loop_config>(),
                    );
                    lopts.clone_fd = (*cmdopts).clone_fd;
                    lopts.max_idle_threads = (*cmdopts).max_idle_threads;
                    err = fuse_session_loop_mt_32(se, &raw mut lopts);
                } else {
                    err = fuse_session_loop(se);
                }
                if err != 0 {
                    if piped[1 as usize] >= 0 as ::core::ffi::c_int {
                        if write(
                            piped[1 as usize],
                            &raw mut s as *const ::core::ffi::c_void,
                            1 as size_t,
                        ) != 1 as ssize_t
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"pipe write error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                strerr(*__errno_location()),
                            );
                        }
                        close(piped[1 as usize]);
                    }
                }
                sstats_term();
                sinodes_term();
                sparents_term();
                fuse_remove_signal_handlers(se);
                fuse_session_unmount(se);
                fuse_session_destroy(se);
            }
            if mfsopts.meta == 0 as ::core::ffi::c_int {
                mfs_term();
                write_data_term();
                read_data_term();
                delay_term();
                csdb_term();
            }
            masterproxy_term();
        }
        fs_term();
        write_term();
        read_term();
        negentry_cache_term();
        symlink_cache_term();
        chunksdatacache_term();
        chunkrwlock_term();
        conncache_term();
        inoleng_term();
        mfs_log_term();
        return if err != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
unsafe extern "C" fn strncpy_remove_commas(
    mut dstbuff: *mut ::core::ffi::c_char,
    mut dstsize: ::core::ffi::c_uint,
    mut src: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        let mut l: ::core::ffi::c_uint = 0;
        l = 0 as ::core::ffi::c_uint;
        loop {
            let c2rust_fresh0 = src;
            src = src.offset(1);
            c = *c2rust_fresh0;
            if !(c as ::core::ffi::c_int != 0 && l.wrapping_add(1 as ::core::ffi::c_uint) < dstsize)
            {
                break;
            }
            if c as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                let c2rust_fresh1 = dstbuff;
                dstbuff = dstbuff.offset(1);
                *c2rust_fresh1 = c;
                l = l.wrapping_add(1);
            }
        }
        *dstbuff = 0 as ::core::ffi::c_char;
        return l;
    }
}
unsafe extern "C" fn strncpy_escape_commas(
    mut dstbuff: *mut ::core::ffi::c_char,
    mut dstsize: ::core::ffi::c_uint,
    mut src: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        let mut l: ::core::ffi::c_uint = 0;
        l = 0 as ::core::ffi::c_uint;
        loop {
            let c2rust_fresh2 = src;
            src = src.offset(1);
            c = *c2rust_fresh2;
            if !(c as ::core::ffi::c_int != 0 && l.wrapping_add(1 as ::core::ffi::c_uint) < dstsize)
            {
                break;
            }
            if c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && c as ::core::ffi::c_int != '\\' as ::core::ffi::c_int
            {
                let c2rust_fresh3 = dstbuff;
                dstbuff = dstbuff.offset(1);
                *c2rust_fresh3 = c;
                l = l.wrapping_add(1);
            } else if l.wrapping_add(2 as ::core::ffi::c_uint) < dstsize {
                let c2rust_fresh4 = dstbuff;
                dstbuff = dstbuff.offset(1);
                *c2rust_fresh4 = '\\' as ::core::ffi::c_char;
                let c2rust_fresh5 = dstbuff;
                dstbuff = dstbuff.offset(1);
                *c2rust_fresh5 = c;
                l = l.wrapping_add(2 as ::core::ffi::c_uint);
            } else {
                *dstbuff = 0 as ::core::ffi::c_char;
                return l;
            }
        }
        *dstbuff = 0 as ::core::ffi::c_char;
        return l;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove_mfsmount_magic(mut args: *mut fuse_args) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 1 as ::core::ffi::c_int;
        while i < (*args).argc {
            if strcmp(
                *(*args).argv.offset(i as isize),
                b"mfsmount_magic\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if (i + 1 as ::core::ffi::c_int) < (*args).argc {
                    memmove(
                        (*args).argv.offset(i as isize) as *mut ::core::ffi::c_void,
                        (*args).argv.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<*mut ::core::ffi::c_char>()
                            .wrapping_mul(((*args).argc - i - 1 as ::core::ffi::c_int) as size_t),
                    );
                }
                (*args).argc -= 1;
                return;
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_fsname(mut args: *mut fuse_args) {
    unsafe {
        let mut fsnamearg: [::core::ffi::c_char; 256] = [0; 256];
        let mut l: ::core::ffi::c_uint = 0;
        let mut libver: ::core::ffi::c_int = 0;
        libver = fuse_version();
        if libver >= 27 as ::core::ffi::c_int {
            l = snprintf(
                &raw mut fsnamearg as *mut ::core::ffi::c_char,
                256 as size_t,
                b"-osubtype=mfs%s,fsname=\0".as_ptr() as *const ::core::ffi::c_char,
                if mfsopts.meta != 0 {
                    b"meta\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as ::core::ffi::c_uint;
            if libver >= 28 as ::core::ffi::c_int {
                l = l.wrapping_add(strncpy_escape_commas(
                    (&raw mut fsnamearg as *mut ::core::ffi::c_char).offset(l as isize),
                    (256 as ::core::ffi::c_uint).wrapping_sub(l),
                    mfsopts.masterhost,
                ));
                if l < 255 as ::core::ffi::c_uint {
                    let c2rust_fresh6 = l;
                    l = l.wrapping_add(1);
                    fsnamearg[c2rust_fresh6 as usize] = ':' as ::core::ffi::c_char;
                }
                l = l.wrapping_add(strncpy_escape_commas(
                    (&raw mut fsnamearg as *mut ::core::ffi::c_char).offset(l as isize),
                    (256 as ::core::ffi::c_uint).wrapping_sub(l),
                    mfsopts.masterport,
                ));
                if *mfsopts.subfolder.offset(0 as isize) as ::core::ffi::c_int
                    != '/' as ::core::ffi::c_int
                {
                    if l < 255 as ::core::ffi::c_uint {
                        let c2rust_fresh7 = l;
                        l = l.wrapping_add(1);
                        fsnamearg[c2rust_fresh7 as usize] = '/' as ::core::ffi::c_char;
                    }
                }
                if *mfsopts.subfolder.offset(0 as isize) as ::core::ffi::c_int
                    != '/' as ::core::ffi::c_int
                    || *mfsopts.subfolder.offset(1 as isize) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                {
                    l = l.wrapping_add(strncpy_escape_commas(
                        (&raw mut fsnamearg as *mut ::core::ffi::c_char).offset(l as isize),
                        (256 as ::core::ffi::c_uint).wrapping_sub(l),
                        mfsopts.subfolder,
                    ));
                }
                if l > 255 as ::core::ffi::c_uint {
                    l = 255 as ::core::ffi::c_uint;
                }
                fsnamearg[l as usize] = 0 as ::core::ffi::c_char;
            } else {
                l = l.wrapping_add(strncpy_remove_commas(
                    (&raw mut fsnamearg as *mut ::core::ffi::c_char).offset(l as isize),
                    (256 as ::core::ffi::c_uint).wrapping_sub(l),
                    mfsopts.masterhost,
                ));
                if l < 255 as ::core::ffi::c_uint {
                    let c2rust_fresh8 = l;
                    l = l.wrapping_add(1);
                    fsnamearg[c2rust_fresh8 as usize] = ':' as ::core::ffi::c_char;
                }
                l = l.wrapping_add(strncpy_remove_commas(
                    (&raw mut fsnamearg as *mut ::core::ffi::c_char).offset(l as isize),
                    (256 as ::core::ffi::c_uint).wrapping_sub(l),
                    mfsopts.masterport,
                ));
                if *mfsopts.subfolder.offset(0 as isize) as ::core::ffi::c_int
                    != '/' as ::core::ffi::c_int
                {
                    if l < 255 as ::core::ffi::c_uint {
                        let c2rust_fresh9 = l;
                        l = l.wrapping_add(1);
                        fsnamearg[c2rust_fresh9 as usize] = '/' as ::core::ffi::c_char;
                    }
                }
                if *mfsopts.subfolder.offset(0 as isize) as ::core::ffi::c_int
                    != '/' as ::core::ffi::c_int
                    || *mfsopts.subfolder.offset(1 as isize) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                {
                    l = l.wrapping_add(strncpy_remove_commas(
                        (&raw mut fsnamearg as *mut ::core::ffi::c_char).offset(l as isize),
                        (256 as ::core::ffi::c_uint).wrapping_sub(l),
                        mfsopts.subfolder,
                    ));
                }
                if l > 255 as ::core::ffi::c_uint {
                    l = 255 as ::core::ffi::c_uint;
                }
                fsnamearg[l as usize] = 0 as ::core::ffi::c_char;
            }
        }
        fuse_opt_insert_arg(
            args,
            1 as ::core::ffi::c_int,
            &raw mut fsnamearg as *mut ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn password_read(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut passwordbuff: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pbsize: size_t = 0;
        let mut i: ::core::ffi::c_int = 0;
        fd = fopen(filename, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        if fd.is_null() {
            fprintf(
                stderr,
                b"error opening password file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        pbsize = 0 as size_t;
        passwordbuff = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if getline(&raw mut passwordbuff, &raw mut pbsize, fd) == -1 as __ssize_t {
            fprintf(
                stderr,
                b"password file (%s) is empty\n\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
            );
            if !passwordbuff.is_null() {
                free(passwordbuff as *mut ::core::ffi::c_void);
            }
            fclose(fd);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        fclose(fd);
        i = strlen(passwordbuff) as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            i -= 1;
            if !(*passwordbuff.offset(i as isize) as ::core::ffi::c_int
                == '\n' as ::core::ffi::c_int
                || *passwordbuff.offset(i as isize) as ::core::ffi::c_int
                    == '\r' as ::core::ffi::c_int)
            {
                break;
            }
            *passwordbuff.offset(i as isize) = 0 as ::core::ffi::c_char;
        }
        if i == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"first line in password file (%s) is empty\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                filename,
            );
            free(passwordbuff as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return passwordbuff;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_if_dir_is_empty(
    mut mp: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dd: *mut DIR = ::core::ptr::null_mut::<DIR>();
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
        let mut de: *mut dirent = ::core::ptr::null_mut::<dirent>();
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        '_err: {
            if stat(mp, &raw mut st) < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"stat mountpoint '%s': %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    mp,
                    strerr(*__errno_location()),
                );
            } else if st.st_mode & S_IFMT as __mode_t != S_IFDIR as __mode_t {
                fprintf(
                    stderr,
                    b"given mountpoint '%s' is not a directory\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mp,
                );
            } else {
                dd = opendir(mp);
                if dd.is_null() {
                    fprintf(
                        stderr,
                        b"error opening '%s': %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        mp,
                        strerr(*__errno_location()),
                    );
                } else {
                    loop {
                        de = readdir(dd);
                        if de.is_null() {
                            break;
                        }
                        if (*de).d_name[0 as usize] as ::core::ffi::c_int
                            == '.' as ::core::ffi::c_int
                        {
                            if (*de).d_name[1 as usize] as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                continue;
                            }
                            if (*de).d_name[1 as usize] as ::core::ffi::c_int
                                == '.' as ::core::ffi::c_int
                                && (*de).d_name[2 as usize] as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                            {
                                continue;
                            }
                        }
                        fprintf(
                            stderr,
                            b"mountpoint '%s' is not empty\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            mp,
                        );
                        break '_err;
                    }
                    res = 1 as ::core::ffi::c_int;
                }
            }
        }
        if !dd.is_null() {
            closedir(dd);
        }
        return res;
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut res: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut cmdopts: fuse_cmdline_opts = fuse_cmdline_opts {
            singlethread: 0,
            foreground: 0,
            debug: 0,
            nodefault_subtype: 0,
            mountpoint: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            show_version: 0,
            show_help: 0,
            clone_fd: 0,
            max_idle_threads: 0,
            max_threads: 0,
        };
        let mut args: fuse_args = fuse_args {
            argc: argc,
            argv: argv as *mut *mut ::core::ffi::c_char,
            allocated: 0 as ::core::ffi::c_int,
        };
        let mut defaultargs: fuse_args = fuse_args {
            argc: 0 as ::core::ffi::c_int,
            argv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            allocated: 0 as ::core::ffi::c_int,
        };
        processname_init(argc, argv);
        signal(
            SIGPIPE,
            ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            ),
        );
        strerr_init();
        mycrc32_init();
        setenv(
            b"FUSE_THREAD_STACK\0".as_ptr() as *const ::core::ffi::c_char,
            b"524288\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        memset(
            &raw mut mfsopts as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mfsopts>(),
        );
        mfsopts.masterhost = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.masterport = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.bindhost = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.proxyhost = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.subfolder = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.password = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.passfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.md5pass = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.preferedlabels = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.nofile = 0 as ::core::ffi::c_uint;
        mfsopts.nice = -19 as ::core::ffi::c_int;
        mfsopts.mfssuid = 0 as ::core::ffi::c_int;
        mfsopts.mfsdev = 0 as ::core::ffi::c_int;
        mfsopts.memlock = 0 as ::core::ffi::c_int;
        mfsopts.limitarenas = 4 as ::core::ffi::c_int;
        mfsopts.allowoomkiller = 0 as ::core::ffi::c_int;
        mfsopts.nostdmountoptions = 0 as ::core::ffi::c_int;
        mfsopts.meta = 0 as ::core::ffi::c_int;
        mfsopts.flattrash = 0 as ::core::ffi::c_int;
        mfsopts.debug = 0 as ::core::ffi::c_int;
        mfsopts.delayedinit = 0 as ::core::ffi::c_int;
        mfsopts.mkdircopysgid = 1 as ::core::ffi::c_uint;
        mfsopts.sugidclearmodestr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.donotrememberpassword = 0 as ::core::ffi::c_int;
        mfsopts.cachefiles = 0 as ::core::ffi::c_int;
        mfsopts.noxattrs = 0 as ::core::ffi::c_int;
        mfsopts.noposixlocks = 0 as ::core::ffi::c_int;
        mfsopts.nobsdlocks = 0 as ::core::ffi::c_int;
        mfsopts.cachemode = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfsopts.writecachesize = 0 as ::core::ffi::c_uint;
        mfsopts.readaheadsize = 0 as ::core::ffi::c_uint;
        mfsopts.readaheadleng = 0 as ::core::ffi::c_uint;
        mfsopts.readaheadtrigger = 0 as ::core::ffi::c_uint;
        mfsopts.erroronlostchunk = 0 as ::core::ffi::c_int;
        mfsopts.erroronnospace = 0 as ::core::ffi::c_int;
        mfsopts.ioretries = 30 as ::core::ffi::c_uint;
        mfsopts.timeout = 0 as ::core::ffi::c_uint;
        mfsopts.logretry = 5 as ::core::ffi::c_uint;
        mfsopts.passwordask = 0 as ::core::ffi::c_int;
        mfsopts.readdirplusminto = 0.0001f64;
        mfsopts.attrcacheto = 1.0f64;
        mfsopts.xattrcacheto = 30.0f64;
        mfsopts.entrycacheto = 0.0f64;
        mfsopts.direntrycacheto = 1.0f64;
        mfsopts.negentrycacheto = 0.0f64;
        mfsopts.symlinkcacheto = 300.0f64;
        mfsopts.groupscacheto = 300.0f64;
        mfsopts.fsyncbeforeclose = 0 as ::core::ffi::c_int;
        mfsopts.fsyncmintime = 0.0f64;
        custom_cfg = 0 as ::core::ffi::c_int;
        if args.argc > 1 as ::core::ffi::c_int {
            let mut hostlen: uint32_t = 0;
            let mut portlen: uint32_t = 0;
            let mut colons: uint32_t = 0;
            let mut c: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut portbegin: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut optpos: ::core::ffi::c_int = 0;
            optpos = 1 as ::core::ffi::c_int;
            while optpos < args.argc {
                c = *args.argv.offset(optpos as isize);
                if !(*c.offset(0 as isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
                    && *c.offset(1 as isize) as ::core::ffi::c_int == 'o' as ::core::ffi::c_int)
                {
                    break;
                }
                if *c.offset(2 as isize) != 0 {
                    optpos += 1;
                } else {
                    optpos += 2 as ::core::ffi::c_int;
                }
            }
            if optpos < args.argc {
                c = *args.argv.offset(optpos as isize);
                colons = 0 as uint32_t;
                i = 0 as ::core::ffi::c_int;
                while *c.offset(i as isize) != 0 {
                    if *c.offset(i as isize) as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                        colons = colons.wrapping_add(1);
                    }
                    i += 1;
                }
                if colons > 0 as uint32_t {
                    hostlen = 0 as uint32_t;
                    portlen = 0 as uint32_t;
                    portbegin = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    while *c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                        && *c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
                        || *c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && *c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
                        || *c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && *c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        || *c as ::core::ffi::c_int == '-' as ::core::ffi::c_int
                        || *c as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                    {
                        c = c.offset(1);
                        hostlen = hostlen.wrapping_add(1);
                    }
                    if hostlen > 0 as uint32_t {
                        if *c as ::core::ffi::c_int == ':' as ::core::ffi::c_int
                            && colons > 1 as uint32_t
                        {
                            c = c.offset(1);
                            portbegin = c;
                            while *c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                                && *c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                            {
                                c = c.offset(1);
                                portlen = portlen.wrapping_add(1);
                            }
                        }
                        if *c as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                            c = c.offset(1);
                            if *c != 0 {
                                mfsopts.subfolder = strdup(c);
                            }
                            mfsopts.masterhost =
                                malloc(hostlen.wrapping_add(1 as uint32_t) as size_t)
                                    as *mut ::core::ffi::c_char;
                            memcpy(
                                mfsopts.masterhost as *mut ::core::ffi::c_void,
                                *args.argv.offset(optpos as isize) as *const ::core::ffi::c_void,
                                hostlen as size_t,
                            );
                            *mfsopts.masterhost.offset(hostlen as isize) = 0 as ::core::ffi::c_char;
                            if !portbegin.is_null() && portlen > 0 as uint32_t {
                                mfsopts.masterport =
                                    malloc(portlen.wrapping_add(1 as uint32_t) as size_t)
                                        as *mut ::core::ffi::c_char;
                                memcpy(
                                    mfsopts.masterport as *mut ::core::ffi::c_void,
                                    portbegin as *const ::core::ffi::c_void,
                                    portlen as size_t,
                                );
                                *mfsopts.masterport.offset(portlen as isize) =
                                    0 as ::core::ffi::c_char;
                            }
                            i = optpos + 1 as ::core::ffi::c_int;
                            while i < args.argc {
                                *args.argv.offset((i - 1 as ::core::ffi::c_int) as isize) =
                                    *args.argv.offset(i as isize);
                                i += 1;
                            }
                            args.argc -= 1;
                        }
                    }
                }
            }
        }
        if fuse_opt_parse(
            &raw mut args,
            &raw mut defaultargs as *mut ::core::ffi::c_void,
            &raw mut mfs_opts_stage1 as *mut fuse_opt as *const fuse_opt,
            Some(
                mfs_opt_proc_stage1
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut fuse_args,
                    ) -> ::core::ffi::c_int,
            ),
        ) < 0 as ::core::ffi::c_int
        {
            exit(1 as ::core::ffi::c_int);
        }
        if custom_cfg == 0 as ::core::ffi::c_int {
            let mut cfgfd: ::core::ffi::c_int = 0;
            let mut cfgfile: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            cfgfile = strdup(
                b"/home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfs/mfsmount.cfg\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            cfgfd = open(cfgfile, O_RDONLY);
            if cfgfd < 0 as ::core::ffi::c_int && *__errno_location() == ENOENT {
                free(cfgfile as *mut ::core::ffi::c_void);
                cfgfile = strdup(
                    b"/home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfsmount.cfg\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                cfgfd = open(cfgfile, O_RDONLY);
                if cfgfd >= 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"default sysconf path has changed - please move mfsmount.cfg from /home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/ to /home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfs/\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            if cfgfd >= 0 as ::core::ffi::c_int {
                close(cfgfd);
            }
            mfs_opt_parse_cfg_file(cfgfile, 1 as ::core::ffi::c_int, &raw mut defaultargs);
            free(cfgfile as *mut ::core::ffi::c_void);
        }
        i = defaultargs.argc;
        while i > 0 as ::core::ffi::c_int {
            fuse_opt_insert_arg(
                &raw mut args,
                1 as ::core::ffi::c_int,
                *defaultargs
                    .argv
                    .offset((i - 1 as ::core::ffi::c_int) as isize),
            );
            i -= 1;
        }
        if fuse_opt_parse(
            &raw mut args,
            &raw mut mfsopts as *mut ::core::ffi::c_void,
            &raw mut mfs_opts_stage2 as *mut fuse_opt as *const fuse_opt,
            Some(
                mfs_opt_proc_stage2
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut fuse_args,
                    ) -> ::core::ffi::c_int,
            ),
        ) < 0 as ::core::ffi::c_int
        {
            exit(1 as ::core::ffi::c_int);
        }
        if !mfsopts.cachemode.is_null() && mfsopts.cachefiles != 0 {
            fprintf(
                stderr,
                b"mfscachemode and mfscachefiles options are exclusive - use only mfscachemode\nsee: %s -h for help\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as isize),
            );
            return 1 as ::core::ffi::c_int;
        }
        if mfsopts.cachemode.is_null() {
            mfsopts.keepcache = if mfsopts.cachefiles != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        } else if strcasecmp(
            mfsopts.cachemode,
            b"AUTO\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mfsopts.keepcache = 0 as ::core::ffi::c_int;
        } else if strcasecmp(
            mfsopts.cachemode,
            b"YES\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                mfsopts.cachemode,
                b"ALWAYS\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            mfsopts.keepcache = 1 as ::core::ffi::c_int;
        } else if strcasecmp(
            mfsopts.cachemode,
            b"NO\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                mfsopts.cachemode,
                b"NONE\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcasecmp(
                mfsopts.cachemode,
                b"NEVER\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            mfsopts.keepcache = 2 as ::core::ffi::c_int;
        } else if strcasecmp(
            mfsopts.cachemode,
            b"DIRECT\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mfsopts.keepcache = 3 as ::core::ffi::c_int;
        } else {
            fprintf(
                stderr,
                b"unrecognized cachemode option\nsee: %s -h for help\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *argv.offset(0 as isize),
            );
            return 1 as ::core::ffi::c_int;
        }
        if mfsopts.sugidclearmodestr.is_null() {
            mfsopts.sugidclearmode = SUGID_CLEAR_MODE_EXT;
        } else if strcasecmp(
            mfsopts.sugidclearmodestr,
            b"NEVER\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mfsopts.sugidclearmode = SUGID_CLEAR_MODE_NEVER;
        } else if strcasecmp(
            mfsopts.sugidclearmodestr,
            b"ALWAYS\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mfsopts.sugidclearmode = SUGID_CLEAR_MODE_ALWAYS;
        } else if strcasecmp(
            mfsopts.sugidclearmodestr,
            b"OSX\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mfsopts.sugidclearmode = SUGID_CLEAR_MODE_OSX;
        } else if strcasecmp(
            mfsopts.sugidclearmodestr,
            b"BSD\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mfsopts.sugidclearmode = SUGID_CLEAR_MODE_BSD;
        } else if strcasecmp(
            mfsopts.sugidclearmodestr,
            b"EXT\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mfsopts.sugidclearmode = SUGID_CLEAR_MODE_EXT;
        } else if strcasecmp(
            mfsopts.sugidclearmodestr,
            b"XFS\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mfsopts.sugidclearmode = SUGID_CLEAR_MODE_XFS;
        } else {
            fprintf(
                stderr,
                b"unrecognized sugidclearmode option\nsee: %s -h for help\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *argv.offset(0 as isize),
            );
            return 1 as ::core::ffi::c_int;
        }
        if mfsopts.logminlevelstr.is_null() {
            mfsopts.logminlevel = MFSLOG_INFO;
        } else {
            mfsopts.logminlevel = mfs_log_str_to_pri(mfsopts.logminlevelstr);
            if mfsopts.logminlevel < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"mfslogminlevel: unrecognized log level, valid levels: [D]EBUG,[I]NFO,[N]OTICE,[W]ARNING,[E]RROR\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                return 1 as ::core::ffi::c_int;
            }
        }
        if mfsopts.logelevatetostr.is_null() {
            mfsopts.logelevateto = MFSLOG_NOTICE;
        } else {
            mfsopts.logelevateto = mfs_log_str_to_pri(mfsopts.logelevatetostr);
            if mfsopts.logelevateto < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"mfslogelevateto: unrecognized log level, valid levels: [D]EBUG,[I]NFO,[N]OTICE,[W]ARNING,[E]RROR\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                return 1 as ::core::ffi::c_int;
            }
        }
        if mfsopts.masterhost.is_null() {
            mfsopts.masterhost = strdup(DEFAULT_MASTERNAME.as_ptr());
        }
        if mfsopts.masterport.is_null() {
            mfsopts.masterport = strdup(DEFAULT_MASTER_CLIENT_PORT.as_ptr());
        }
        if mfsopts.proxyhost.is_null() {
            mfsopts.proxyhost = strdup(b"127.0.0.1\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if mfsopts.subfolder.is_null() {
            mfsopts.subfolder = strdup(b"/\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if !mfsopts.passfile.is_null() {
            if !mfsopts.password.is_null() || !mfsopts.md5pass.is_null() {
                fprintf(
                    stderr,
                    b"mfspassfile option is mutually exclusive with mfspassword and mfsmd5pass\nsee: %s -h for help\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    *argv.offset(0 as isize),
                );
                return 1 as ::core::ffi::c_int;
            }
            mfsopts.password = password_read(mfsopts.passfile);
            if mfsopts.password.is_null() {
                return 1 as ::core::ffi::c_int;
            }
        }
        if mfsopts.nofile == 0 as ::core::ffi::c_uint {
            mfsopts.nofile = 100000 as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
        if mfsopts.writecachesize == 0 as ::core::ffi::c_uint {
            mfsopts.writecachesize = 256 as ::core::ffi::c_uint;
        }
        if mfsopts.writecachesize < 16 as ::core::ffi::c_uint {
            fprintf(
                stderr,
                b"write cache size too low (%u MiB) - increased to 16 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.writecachesize,
            );
            mfsopts.writecachesize = 16 as ::core::ffi::c_uint;
        }
        if mfsopts.writecachesize > 2048 as ::core::ffi::c_uint {
            fprintf(
                stderr,
                b"write cache size too big (%u MiB) - decresed to 2048 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.writecachesize,
            );
            mfsopts.writecachesize = 2048 as ::core::ffi::c_uint;
        }
        if mfsopts.readaheadsize == 0 as ::core::ffi::c_uint {
            mfsopts.readaheadsize = 256 as ::core::ffi::c_uint;
        }
        if mfsopts.readaheadsize < 16 as ::core::ffi::c_uint {
            fprintf(
                stderr,
                b"read ahead size too low (%u MiB) - increased to 16 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.readaheadsize,
            );
            mfsopts.readaheadsize = 16 as ::core::ffi::c_uint;
        }
        if mfsopts.readaheadsize > 2048 as ::core::ffi::c_uint {
            fprintf(
                stderr,
                b"read ahead size too big (%u MiB) - decresed to 2048 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.readaheadsize,
            );
            mfsopts.readaheadsize = 2048 as ::core::ffi::c_uint;
        }
        if mfsopts.readaheadleng == 0 as ::core::ffi::c_uint {
            mfsopts.readaheadleng = 0x100000 as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
        if mfsopts.readaheadleng < 0x20000 as ::core::ffi::c_int as ::core::ffi::c_uint {
            fprintf(
                stderr,
                b"read ahead length too low (%u B) - increased to 128 KiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.readaheadleng,
            );
            mfsopts.readaheadleng = 0x20000 as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
        if mfsopts.readaheadleng > 0x200000 as ::core::ffi::c_int as ::core::ffi::c_uint {
            fprintf(
                stderr,
                b"read ahead length too big (%u B) - decresed to 2 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.readaheadleng,
            );
            mfsopts.readaheadleng = 0x200000 as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
        if mfsopts.readaheadtrigger == 0 as ::core::ffi::c_uint {
            mfsopts.readaheadtrigger = mfsopts
                .readaheadleng
                .wrapping_mul(10 as ::core::ffi::c_uint);
        }
        if mfsopts.nostdmountoptions == 0 as ::core::ffi::c_int {
            fuse_opt_add_arg(
                &raw mut args,
                b"-oallow_other\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if mfsopts.fsyncbeforeclose != 0 {
            mfsopts.fsyncmintime = 0.0f64;
        }
        if mfsopts.attrcacheto > 86400.0f64 {
            fprintf(
                stderr,
                b"attribute cache timeout too big (%.2lf) - decreased to %.2lf\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.attrcacheto,
                86400.0f64,
            );
            mfsopts.attrcacheto = 86400.0f64;
        }
        if mfsopts.attrcacheto < 0.0f64 {
            fprintf(
                stderr,
                b"negative value of attribute cache timeout - set to 0\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfsopts.attrcacheto = 0.0f64;
        }
        if mfsopts.xattrcacheto > 86400.0f64 {
            fprintf(
                stderr,
                b"xattr cache timeout too big (%.2lf) - decreased to %.2lf\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.xattrcacheto,
                86400.0f64,
            );
            mfsopts.xattrcacheto = 86400.0f64;
        }
        if mfsopts.xattrcacheto < 0.0f64 {
            fprintf(
                stderr,
                b"negative value of xattr cache timeout - set to 0\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfsopts.xattrcacheto = 0.0f64;
        }
        if mfsopts.entrycacheto > 86400.0f64 {
            fprintf(
                stderr,
                b"entry cache timeout too big (%.2lf) - decreased to %.2lf\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.entrycacheto,
                86400.0f64,
            );
            mfsopts.entrycacheto = 86400.0f64;
        }
        if mfsopts.entrycacheto < 0.0f64 {
            fprintf(
                stderr,
                b"negative value of entry cache timeout - set to 0\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfsopts.entrycacheto = 0.0f64;
        }
        if mfsopts.readdirplusminto > 60.0f64 {
            fprintf(
                stderr,
                b"readdir plus cache timeout too big (%.2lf) - decreased to %.2lf\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.readdirplusminto,
                60.0f64,
            );
            mfsopts.readdirplusminto = 60.0f64;
        }
        if mfsopts.readdirplusminto < 0.0f64 {
            fprintf(
                stderr,
                b"negative value of readdir plus cache timeout - set to 0\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfsopts.readdirplusminto = 0.0f64;
        }
        if mfsopts.direntrycacheto > 86400.0f64 {
            fprintf(
                stderr,
                b"directory entry cache timeout too big (%.2lf) - decreased to %.2lf\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.direntrycacheto,
                86400.0f64,
            );
            mfsopts.direntrycacheto = 86400.0f64;
        }
        if mfsopts.direntrycacheto < 0.0f64 {
            fprintf(
                stderr,
                b"negative value of directory entry cache timeout - set to 0\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfsopts.direntrycacheto = 0.0f64;
        }
        if mfsopts.negentrycacheto > 86400.0f64 {
            fprintf(
                stderr,
                b"non existing entry cache timeout too big (%.2lf) - decreased to %.2lf\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                mfsopts.negentrycacheto,
                86400.0f64,
            );
            mfsopts.negentrycacheto = 86400.0f64;
        }
        if mfsopts.negentrycacheto < 0.0f64 {
            fprintf(
                stderr,
                b"negative value of non existing entry cache timeout - set to 0\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfsopts.negentrycacheto = 0.0f64;
        }
        if mfsopts.symlinkcacheto > 86400.0f64 {
            fprintf(
                stderr,
                b"symbolic link cache timeout too big (%.2lf) - decreased to %.2lf\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.symlinkcacheto,
                86400.0f64,
            );
            mfsopts.symlinkcacheto = 86400.0f64;
        }
        if mfsopts.symlinkcacheto < 0.0f64 {
            fprintf(
                stderr,
                b"negative value of symbolic link cache timeout - set to 0\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfsopts.symlinkcacheto = 0.0f64;
        }
        if mfsopts.groupscacheto > 86400.0f64 {
            fprintf(
                stderr,
                b"auxiliary groups cache timeout too big (%.2lf) - decreased to %.2lf\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mfsopts.groupscacheto,
                86400.0f64,
            );
            mfsopts.groupscacheto = 86400.0f64;
        }
        if mfsopts.groupscacheto < 0.0f64 {
            fprintf(
                stderr,
                b"negative value of auxiliary groups cache timeout - set to 0\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfsopts.groupscacheto = 0.0f64;
        }
        if csorder_init(mfsopts.preferedlabels) < 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"error parsing preferred labels expression\nsee: %s -h for help\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *argv.offset(0 as isize),
            );
            return 1 as ::core::ffi::c_int;
        }
        make_fsname(&raw mut args);
        if mfsopts.mfssuid != 0 {
            fuse_opt_insert_arg(
                &raw mut args,
                1 as ::core::ffi::c_int,
                b"-osuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if mfsopts.mfsdev != 0 {
            fuse_opt_insert_arg(
                &raw mut args,
                1 as ::core::ffi::c_int,
                b"-odev\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        remove_mfsmount_magic(&raw mut args);
        if fuse_parse_cmdline(&raw mut args, &raw mut cmdopts) < 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"see: %s -h for help\n\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0 as isize),
            );
            return 1 as ::core::ffi::c_int;
        }
        if cmdopts.mountpoint.is_null() {
            if !defaultmountpoint.is_null() {
                cmdopts.mountpoint = defaultmountpoint;
            } else {
                fprintf(
                    stderr,
                    b"no mount point\nsee: %s -h for help\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as isize),
                );
                return 1 as ::core::ffi::c_int;
            }
        }
        if mfsopts.nonempty == 0 as ::core::ffi::c_int {
            if check_if_dir_is_empty(cmdopts.mountpoint) == 0 as ::core::ffi::c_int {
                return 1 as ::core::ffi::c_int;
            }
        }
        res = mainloop(&raw mut args, &raw mut cmdopts);
        fuse_opt_free_args(&raw mut defaultargs);
        fuse_opt_free_args(&raw mut args);
        free(mfsopts.masterhost as *mut ::core::ffi::c_void);
        free(mfsopts.masterport as *mut ::core::ffi::c_void);
        if !mfsopts.bindhost.is_null() {
            free(mfsopts.bindhost as *mut ::core::ffi::c_void);
        }
        if !mfsopts.proxyhost.is_null() {
            free(mfsopts.proxyhost as *mut ::core::ffi::c_void);
        }
        free(mfsopts.subfolder as *mut ::core::ffi::c_void);
        if !defaultmountpoint.is_null() {
            free(defaultmountpoint as *mut ::core::ffi::c_void);
        }
        stats_term();
        strerr_term();
        return res;
    }
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        mfs_opts_stage1 = [
            fuse_opt {
                templ: b"mfscfgfile=\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_CFGFILE as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-c \0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_CFGFILE as ::core::ffi::c_int,
            },
            FUSE_OPT_END,
        ];
        mfs_opts_stage2 = [
            fuse_opt {
                templ: b"mfsmaster=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 0 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsport=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 8 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsbind=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 16 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsproxy=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 24 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfssubfolder=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 32 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfspassword=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 40 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfspassfile=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 48 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsmd5pass=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 56 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfspreflabels=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 64 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsrlimitnofile=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 72 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsnice=%d\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 76 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfssuid\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 80 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsdev\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 84 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsmemlock\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 88 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfslimitarenas=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 92 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsallowoomkiller\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 96 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfslogminlevel=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 104 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfslogelevateto=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 120 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"nonempty\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 100 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfswritecachesize=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 212 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsreadaheadsize=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 216 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsreadaheadleng=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 220 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsreadaheadtrigger=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 224 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfserroronlostchunk\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 228 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfserroronnospace\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 232 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsioretries=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 236 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfstimeout=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 240 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfslogretry=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 244 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsdebug\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 140 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsmeta\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 136 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsflattrash\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 144 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsdelayedinit\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 148 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsdonotrememberpassword\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 208 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfscachefiles\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 184 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsnoxattr\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 196 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsnoxattrs\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 196 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsnoposixlock\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 200 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsnoposixlocks\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 200 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsnobsdlock\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 204 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsnobsdlocks\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 204 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfscachemode=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 176 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsmkdircopysgid=%u\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 152 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfssugidclearmode=%s\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 160 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsreaddirplusminto=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 248 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsattrcacheto=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 256 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsxattrcacheto=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 264 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsentrycacheto=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 272 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsdirentrycacheto=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 280 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsnegentrycacheto=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 288 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfssymlinkcacheto=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 296 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsgroupscacheto=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 304 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsfsyncmintime=%lf\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 312 as ::core::ffi::c_ulong,
                value: 0 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"mfsfsyncbeforeclose\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 320 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"_netdev\0".as_ptr() as *const ::core::ffi::c_char,
                offset: 324 as ::core::ffi::c_ulong,
                value: 1 as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-m\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_META as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"--meta\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_META as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-H \0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_HOST as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-P \0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_PORT as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-B \0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_BIND as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-L \0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_PROXY as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-S \0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_PATH as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-p\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_PASSWORDASK as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"--password\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_PASSWORDASK as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-n\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_NOSTDMOUNTOPTIONS as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"--nostdopts\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_NOSTDMOUNTOPTIONS as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-V\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_VERSION as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"--version\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_VERSION as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"-h\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_HELP as ::core::ffi::c_int,
            },
            fuse_opt {
                templ: b"--help\0".as_ptr() as *const ::core::ffi::c_char,
                offset: (1 as ::core::ffi::c_uint).wrapping_neg() as ::core::ffi::c_ulong,
                value: KEY_HELP as ::core::ffi::c_int,
            },
            FUSE_OPT_END,
        ];
    }
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XIB"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
