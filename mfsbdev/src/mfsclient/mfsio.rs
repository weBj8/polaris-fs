extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn getpid() -> __pid_t;
    fn geteuid() -> __uid_t;
    fn getegid() -> __gid_t;
    fn getgroups(__size: ::core::ffi::c_int, __list: *mut __gid_t) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn mfs_int_mknod(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        r#type: uint8_t,
        mode: uint16_t,
        dev: uint32_t,
    ) -> uint8_t;
    fn mfs_int_unlink(cr: *mut mfs_int_cred, path: *const ::core::ffi::c_char) -> uint8_t;
    fn mfs_int_mkdir(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        mode: uint16_t,
    ) -> uint8_t;
    fn mfs_int_rename(
        cr: *mut mfs_int_cred,
        src: *const ::core::ffi::c_char,
        dst: *const ::core::ffi::c_char,
    ) -> uint8_t;
    fn mfs_int_rmdir(cr: *mut mfs_int_cred, path: *const ::core::ffi::c_char) -> uint8_t;
    fn mfs_int_link(
        cr: *mut mfs_int_cred,
        src: *const ::core::ffi::c_char,
        dst: *const ::core::ffi::c_char,
    ) -> uint8_t;
    fn mfs_int_symlink(
        cr: *mut mfs_int_cred,
        nodepath: *const ::core::ffi::c_char,
        linkpath: *const ::core::ffi::c_char,
    ) -> uint8_t;
    fn mfs_int_readlink(
        cr: *mut mfs_int_cred,
        nodepath: *const ::core::ffi::c_char,
        linkpath: *mut ::core::ffi::c_char,
    ) -> uint8_t;
    fn mfs_int_chmod(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        mode: uint16_t,
    ) -> uint8_t;
    fn mfs_int_fchmod(cr: *mut mfs_int_cred, fildes: ::core::ffi::c_int, mode: uint16_t)
        -> uint8_t;
    fn mfs_int_chown(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        owner: uint32_t,
        group: uint32_t,
    ) -> uint8_t;
    fn mfs_int_fchown(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        owner: uint32_t,
        group: uint32_t,
    ) -> uint8_t;
    fn mfs_int_utimes(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        flags: uint8_t,
        atime: uint32_t,
        mtime: uint32_t,
    ) -> uint8_t;
    fn mfs_int_futimes(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        flags: uint8_t,
        atime: uint32_t,
        mtime: uint32_t,
    ) -> uint8_t;
    fn mfs_int_truncate(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        size: int64_t,
    ) -> uint8_t;
    fn mfs_int_ftruncate(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        size: int64_t,
    ) -> uint8_t;
    fn mfs_int_lseek(fildes: ::core::ffi::c_int, offset: *mut int64_t, whence: uint8_t) -> uint8_t;
    fn mfs_int_stat(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        buf: *mut mfs_int_statrec,
    ) -> uint8_t;
    fn mfs_int_fstat(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        buf: *mut mfs_int_statrec,
    ) -> uint8_t;
    fn mfs_int_getxattr(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        vbuff: *mut *const uint8_t,
        vleng: *mut uint32_t,
        mode: uint8_t,
    ) -> uint8_t;
    fn mfs_int_fgetxattr(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
        vbuff: *mut *const uint8_t,
        vleng: *mut uint32_t,
        mode: uint8_t,
    ) -> uint8_t;
    fn mfs_int_setxattr(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        value: *const uint8_t,
        vsize: uint32_t,
        mode: uint8_t,
    ) -> uint8_t;
    fn mfs_int_fsetxattr(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
        value: *const uint8_t,
        vsize: uint32_t,
        mode: uint8_t,
    ) -> uint8_t;
    fn mfs_int_removexattr(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
    ) -> uint8_t;
    fn mfs_int_fremovexattr(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
    ) -> uint8_t;
    fn mfs_int_listxattr(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        rsize: *mut int32_t,
        list: *mut ::core::ffi::c_char,
        size: uint32_t,
    ) -> uint8_t;
    fn mfs_int_flistxattr(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        rsize: *mut int32_t,
        list: *mut ::core::ffi::c_char,
        size: uint32_t,
    ) -> uint8_t;
    fn mfs_int_getfacl(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        acltype: uint8_t,
        userperm: *mut uint16_t,
        groupperm: *mut uint16_t,
        otherperm: *mut uint16_t,
        maskperm: *mut uint16_t,
        namedusers: *mut uint16_t,
        namedgroups: *mut uint16_t,
        namedacls: *mut *const uint8_t,
        namedaclssize: *mut uint32_t,
    ) -> uint8_t;
    fn mfs_int_fgetfacl(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        acltype: uint8_t,
        userperm: *mut uint16_t,
        groupperm: *mut uint16_t,
        otherperm: *mut uint16_t,
        maskperm: *mut uint16_t,
        namedusers: *mut uint16_t,
        namedgroups: *mut uint16_t,
        namedacls: *mut *const uint8_t,
        namedaclssize: *mut uint32_t,
    ) -> uint8_t;
    fn mfs_int_setfacl(
        cr: *mut mfs_int_cred,
        path: *const ::core::ffi::c_char,
        acltype: uint8_t,
        userperm: uint16_t,
        groupperm: uint16_t,
        otherperm: uint16_t,
        maskperm: uint16_t,
        namedusers: uint16_t,
        namedgroups: uint16_t,
        namedacls: *mut uint8_t,
        namedaclssize: uint32_t,
    ) -> uint8_t;
    fn mfs_int_fsetfacl(
        cr: *mut mfs_int_cred,
        fildes: ::core::ffi::c_int,
        acltype: uint8_t,
        userperm: uint16_t,
        groupperm: uint16_t,
        otherperm: uint16_t,
        maskperm: uint16_t,
        namedusers: uint16_t,
        namedgroups: uint16_t,
        namedacls: *mut uint8_t,
        namedaclssize: uint32_t,
    ) -> uint8_t;
    fn mfs_int_statfs(buf: *mut mfs_int_statfsrec) -> uint8_t;
    fn mfs_int_open(
        cr: *mut mfs_int_cred,
        fildes: *mut ::core::ffi::c_int,
        path: *const ::core::ffi::c_char,
        oflag: ::core::ffi::c_int,
        mode: ::core::ffi::c_int,
    ) -> uint8_t;
    fn mfs_int_pread(
        fildes: ::core::ffi::c_int,
        rsize: *mut int64_t,
        buf: *mut uint8_t,
        nbyte: uint64_t,
        offset: uint64_t,
    ) -> uint8_t;
    fn mfs_int_read(
        fildes: ::core::ffi::c_int,
        rsize: *mut int64_t,
        buf: *mut uint8_t,
        nbyte: uint64_t,
    ) -> uint8_t;
    fn mfs_int_pwrite(
        fildes: ::core::ffi::c_int,
        rsize: *mut int64_t,
        buf: *const uint8_t,
        nbyte: uint64_t,
        offset: uint64_t,
    ) -> uint8_t;
    fn mfs_int_write(
        fildes: ::core::ffi::c_int,
        rsize: *mut int64_t,
        buf: *const uint8_t,
        nbyte: uint64_t,
    ) -> uint8_t;
    fn mfs_int_fsync(fildes: ::core::ffi::c_int) -> uint8_t;
    fn mfs_int_close(fildes: ::core::ffi::c_int) -> uint8_t;
    fn mfs_int_flock(fildes: ::core::ffi::c_int, op: uint8_t) -> uint8_t;
    fn mfs_int_lockf(
        fildes: ::core::ffi::c_int,
        pid: uint32_t,
        function: uint8_t,
        size: int64_t,
    ) -> uint8_t;
    fn mfs_int_fcntl_locks(
        fildes: ::core::ffi::c_int,
        pid: uint32_t,
        function: uint8_t,
        fl: *mut mfs_int_flockrec,
    ) -> uint8_t;
    fn mfs_int_init(mcfg: *mut mfs_int_cfg, stage: uint8_t) -> ::core::ffi::c_int;
    fn mfs_int_term();
    fn umask(__mask: __mode_t) -> __mode_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __fsblkcnt64_t = ::core::ffi::c_ulong;
pub type __fsfilcnt64_t = ::core::ffi::c_ulong;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type va_list = __gnuc_va_list;
pub type off_t = __off64_t;
pub type ssize_t = isize;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type int32_t = i32;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfs_int_cfg {
    pub masterhost: *mut ::core::ffi::c_char,
    pub masterport: *mut ::core::ffi::c_char,
    pub masterbind: *mut ::core::ffi::c_char,
    pub masterpassword: *mut ::core::ffi::c_char,
    pub mastermd5pass: *mut ::core::ffi::c_char,
    pub mountpoint: *mut ::core::ffi::c_char,
    pub masterpath: *mut ::core::ffi::c_char,
    pub preferedlabels: *mut ::core::ffi::c_char,
    pub read_cache_mb: ::core::ffi::c_int,
    pub write_cache_mb: ::core::ffi::c_int,
    pub io_try_cnt: ::core::ffi::c_int,
    pub io_timeout: ::core::ffi::c_int,
    pub min_log_entry: ::core::ffi::c_int,
    pub readahead_leng: ::core::ffi::c_int,
    pub readahead_trigger: ::core::ffi::c_int,
    pub error_on_lost_chunk: ::core::ffi::c_int,
    pub error_on_no_space: ::core::ffi::c_int,
    pub sugid_clear_mode: ::core::ffi::c_int,
    pub mkdir_copy_sgid: ::core::ffi::c_int,
    pub lcache_retention: ::core::ffi::c_double,
    pub logident: *mut ::core::ffi::c_char,
    pub logdaemon: ::core::ffi::c_int,
    pub logminlevel: ::core::ffi::c_int,
    pub logelevateto: ::core::ffi::c_int,
    pub master_min_version_maj: uint16_t,
    pub master_min_version_mid: uint16_t,
}
pub type mfs_int_cfg = _mfs_int_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfs_int_cred {
    pub umask: uint16_t,
    pub uid: uint32_t,
    pub gidcnt: uint32_t,
    pub gidtab: [uint32_t; 256],
}
pub type mfs_int_cred = _mfs_int_cred;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfs_int_statfsrec {
    pub totalspace: uint64_t,
    pub availspace: uint64_t,
    pub freespace: uint64_t,
    pub trashspace: uint64_t,
    pub sustainedspace: uint64_t,
    pub inodes: uint32_t,
    pub masterip: uint32_t,
    pub masterport: uint16_t,
    pub sessionid: uint32_t,
    pub masterprocessid: uint64_t,
    pub masterversion: uint32_t,
}
pub type mfs_int_statfsrec = _mfs_int_statfsrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfs_int_statrec {
    pub inode: uint32_t,
    pub r#type: uint8_t,
    pub winattr: uint8_t,
    pub mode: uint16_t,
    pub uid: uint32_t,
    pub gid: uint32_t,
    pub atime: uint32_t,
    pub mtime: uint32_t,
    pub ctime: uint32_t,
    pub nlink: uint32_t,
    pub dev: uint32_t,
    pub length: uint64_t,
}
pub type mfs_int_statrec = _mfs_int_statrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfs_int_flockrec {
    pub r#type: uint8_t,
    pub whence: uint8_t,
    pub start: int64_t,
    pub len: int64_t,
    pub pid: uint32_t,
}
pub type mfs_int_flockrec = _mfs_int_flockrec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: ::core::ffi::c_ulong,
    pub f_frsize: ::core::ffi::c_ulong,
    pub f_blocks: __fsblkcnt64_t,
    pub f_bfree: __fsblkcnt64_t,
    pub f_bavail: __fsblkcnt64_t,
    pub f_files: __fsfilcnt64_t,
    pub f_ffree: __fsfilcnt64_t,
    pub f_favail: __fsfilcnt64_t,
    pub f_fsid: ::core::ffi::c_ulong,
    pub f_flag: ::core::ffi::c_ulong,
    pub f_namemax: ::core::ffi::c_ulong,
    pub f_type: ::core::ffi::c_uint,
    pub __f_spare: [::core::ffi::c_int; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfscfg {
    pub masterhost: *mut ::core::ffi::c_char,
    pub masterport: *mut ::core::ffi::c_char,
    pub masterbind: *mut ::core::ffi::c_char,
    pub masterpassword: *mut ::core::ffi::c_char,
    pub mastermd5pass: *mut ::core::ffi::c_char,
    pub mountpoint: *mut ::core::ffi::c_char,
    pub masterpath: *mut ::core::ffi::c_char,
    pub preferedlabels: *mut ::core::ffi::c_char,
    pub read_cache_mb: ::core::ffi::c_int,
    pub write_cache_mb: ::core::ffi::c_int,
    pub io_try_cnt: ::core::ffi::c_int,
    pub io_timeout: ::core::ffi::c_int,
    pub min_log_entry: ::core::ffi::c_int,
    pub readahead_leng: ::core::ffi::c_int,
    pub readahead_trigger: ::core::ffi::c_int,
    pub error_on_lost_chunk: ::core::ffi::c_int,
    pub error_on_no_space: ::core::ffi::c_int,
    pub sugid_clear_mode: ::core::ffi::c_int,
    pub mkdir_copy_sgid: ::core::ffi::c_int,
    pub lcache_retention: ::core::ffi::c_double,
    pub logident: *mut ::core::ffi::c_char,
    pub logdaemon: ::core::ffi::c_int,
    pub logminlevel: ::core::ffi::c_int,
    pub logelevateto: ::core::ffi::c_int,
    pub master_min_version_maj: uint16_t,
    pub master_min_version_mid: uint16_t,
}
pub type mfscfg = _mfscfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfsaclid {
    pub id: uint32_t,
    pub perm: uint16_t,
}
pub type mfsaclid = _mfsaclid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfsacl {
    pub userperm: uint16_t,
    pub groupperm: uint16_t,
    pub otherperm: uint16_t,
    pub maskperm: uint16_t,
    pub nuserscnt: uint16_t,
    pub ngroupscnt: uint16_t,
    pub namedacls: [mfsaclid; 1],
}
pub type mfsacl = _mfsacl;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const F_ULOCK: ::core::ffi::c_int = 0;
pub const F_LOCK: ::core::ffi::c_int = 1;
pub const F_TLOCK: ::core::ffi::c_int = 2;
pub const F_TEST: ::core::ffi::c_int = 3;
pub const F_GETLK64: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const F_SETLK64: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const F_SETLKW64: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const O_ACCMODE: ::core::ffi::c_int = 0o3 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0;
pub const O_WRONLY: ::core::ffi::c_int = 1;
pub const O_RDWR: ::core::ffi::c_int = 2;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const F_GETLK: ::core::ffi::c_int = F_GETLK64;
pub const F_SETLK: ::core::ffi::c_int = F_SETLK64;
pub const F_SETLKW: ::core::ffi::c_int = F_SETLKW64;
pub const F_RDLCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_WRLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_UNLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFDIR: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const __S_IFCHR: ::core::ffi::c_int = 0o20000 as ::core::ffi::c_int;
pub const __S_IFBLK: ::core::ffi::c_int = 0o60000 as ::core::ffi::c_int;
pub const __S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const __S_IFIFO: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const __S_IFLNK: ::core::ffi::c_int = 0o120000 as ::core::ffi::c_int;
pub const __S_IFSOCK: ::core::ffi::c_int = 0o140000 as ::core::ffi::c_int;
pub const UTIME_NOW: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_long) << 30 as ::core::ffi::c_int) - 1 as ::core::ffi::c_long;
pub const UTIME_OMIT: ::core::ffi::c_long =
    ((1 as ::core::ffi::c_long) << 30 as ::core::ffi::c_int) - 2 as ::core::ffi::c_long;
pub const SEEK_SET: ::core::ffi::c_int = 0;
pub const SEEK_CUR: ::core::ffi::c_int = 1;
pub const SEEK_END: ::core::ffi::c_int = 2;
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const MFSBLOCKSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const MFS_NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const MFS_SYMLINK_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1;
pub const MFS_ERROR_ENOTDIR: ::core::ffi::c_int = 2;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3;
pub const MFS_ERROR_EACCES: ::core::ffi::c_int = 4;
pub const MFS_ERROR_EEXIST: ::core::ffi::c_int = 5;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6;
pub const MFS_ERROR_ENOTEMPTY: ::core::ffi::c_int = 7;
pub const MFS_ERROR_CHUNKLOST: ::core::ffi::c_int = 8;
pub const MFS_ERROR_NOCHUNKSERVERS: ::core::ffi::c_int = 12;
pub const MFS_ERROR_NOTOPENED: ::core::ffi::c_int = 17;
pub const MFS_ERROR_NOSPACE: ::core::ffi::c_int = 21;
pub const MFS_ERROR_IO: ::core::ffi::c_int = 22;
pub const MFS_ERROR_EROFS: ::core::ffi::c_int = 33;
pub const MFS_ERROR_QUOTA: ::core::ffi::c_int = 34;
pub const MFS_ERROR_ENOATTR: ::core::ffi::c_int = 38;
pub const MFS_ERROR_ENOTSUP: ::core::ffi::c_int = 39;
pub const MFS_ERROR_ERANGE: ::core::ffi::c_int = 40;
pub const MFS_ERROR_CSNOTPRESENT: ::core::ffi::c_int = 43;
pub const MFS_ERROR_EAGAIN: ::core::ffi::c_int = 45;
pub const MFS_ERROR_EINTR: ::core::ffi::c_int = 46;
pub const MFS_ERROR_ECANCELED: ::core::ffi::c_int = 47;
pub const MFS_ERROR_ENAMETOOLONG: ::core::ffi::c_int = 58;
pub const MFS_ERROR_EMLINK: ::core::ffi::c_int = 59;
pub const MFS_ERROR_EBADF: ::core::ffi::c_int = 61;
pub const MFS_ERROR_EFBIG: ::core::ffi::c_int = 62;
pub const MFS_ERROR_EISDIR: ::core::ffi::c_int = 63;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TYPE_FILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TYPE_DIRECTORY: ::core::ffi::c_int = 2;
pub const TYPE_SYMLINK: ::core::ffi::c_int = 3;
pub const TYPE_FIFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TYPE_BLOCKDEV: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TYPE_CHARDEV: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TYPE_SOCKET: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const MFS_XATTR_CREATE_OR_REPLACE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_XATTR_CREATE_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_XATTR_REPLACE_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_XATTR_GETA_DATA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_XATTR_LENGTH_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_XATTR_SIZE_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ENXIO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EFBIG: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const ENOSPC: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const EROFS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const EMLINK: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const ENAMETOOLONG: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const ENOTEMPTY: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const ENODATA: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const EDQUOT: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const ECANCELED: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
pub const LOCK_SH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOCK_EX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOCK_UN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LOCK_NB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DEFAULT_MASTERNAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"mfsmaster\0") };
pub const DEFAULT_MASTER_CLIENT_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9421\0") };
pub const RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VERSMAJ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VERSMID: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const VERSMIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    val = val.swap_bytes() as uint32_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn put16bit(mut ptr: *mut *mut uint8_t, mut val: uint16_t) {
    val = val.swap_bytes() as uint16_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        2 as size_t,
    );
    *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    let mut t32: uint32_t = 0;
    memcpy(
        &raw mut t32 as *mut ::core::ffi::c_void,
        *ptr as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
    return t32.swap_bytes();
}
#[inline]
unsafe extern "C" fn get16bit(mut ptr: *mut *const uint8_t) -> uint16_t {
    let mut t16: uint16_t = 0;
    memcpy(
        &raw mut t16 as *mut ::core::ffi::c_void,
        *ptr as *const ::core::ffi::c_void,
        2 as size_t,
    );
    *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
    return t16.swap_bytes();
}
#[no_mangle]
pub static mut id: [::core::ffi::c_char; 72] = unsafe {
    ::core::mem::transmute::<[u8; 72], [::core::ffi::c_char; 72]>(
        *b"@(#) version: 4.59.2-1, build: 2106, written by Jakub Kruszona-Zawadzki\0",
    )
};
pub const MFS_NGROUPS_MAX: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MFS_TIMES_ATIME_NOW: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_TIMES_MTIME_NOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_TIMES_ATIME_OMIT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_TIMES_MTIME_OMIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MFS_SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_O_WRONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_O_RDWR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_O_ACCMODE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_O_CREAT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_O_TRUNC: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MFS_O_EXCL: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFS_O_APPEND: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_LOCK_SH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_LOCK_EX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_LOCK_NB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_LOCK_UN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MFS_F_ULOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_F_LOCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_F_TLOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_F_TEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_F_GETLK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_F_SETLK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_F_SETLKW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_F_RDLCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_F_WRLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_F_UNLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const S_IFDIR: ::core::ffi::c_int = __S_IFDIR;
pub const S_IFCHR: ::core::ffi::c_int = __S_IFCHR;
pub const S_IFBLK: ::core::ffi::c_int = __S_IFBLK;
pub const S_IFREG: ::core::ffi::c_int = __S_IFREG;
pub const S_IFIFO: ::core::ffi::c_int = __S_IFIFO;
pub const S_IFLNK: ::core::ffi::c_int = __S_IFLNK;
pub const S_IFSOCK: ::core::ffi::c_int = __S_IFSOCK;
pub const XATTR_CREATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XATTR_REPLACE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENOATTR: ::core::ffi::c_int = ENODATA;
unsafe extern "C" fn mfs_errorconv(mut status: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    match status {
        MFS_STATUS_OK => {
            ret = 0 as ::core::ffi::c_int;
        }
        MFS_ERROR_EPERM => {
            ret = EPERM;
        }
        MFS_ERROR_ENOTDIR => {
            ret = ENOTDIR;
        }
        MFS_ERROR_ENOENT => {
            ret = ENOENT;
        }
        MFS_ERROR_EACCES => {
            ret = EACCES;
        }
        MFS_ERROR_EEXIST => {
            ret = EEXIST;
        }
        MFS_ERROR_EINVAL => {
            ret = EINVAL;
        }
        MFS_ERROR_ENOTEMPTY => {
            ret = ENOTEMPTY;
        }
        MFS_ERROR_IO => {
            ret = EIO;
        }
        MFS_ERROR_EROFS => {
            ret = EROFS;
        }
        MFS_ERROR_EINTR => {
            ret = EINTR;
        }
        MFS_ERROR_EAGAIN => {
            ret = EAGAIN;
        }
        MFS_ERROR_ECANCELED => {
            ret = ECANCELED;
        }
        MFS_ERROR_QUOTA => {
            ret = EDQUOT;
        }
        MFS_ERROR_ENOATTR => {
            ret = ENOATTR;
        }
        MFS_ERROR_ENOTSUP => {
            ret = ENOTSUP;
        }
        MFS_ERROR_ERANGE => {
            ret = ERANGE;
        }
        MFS_ERROR_NOSPACE => {
            ret = ENOSPC;
        }
        MFS_ERROR_CHUNKLOST => {
            ret = ENXIO;
        }
        MFS_ERROR_NOCHUNKSERVERS => {
            ret = ENOSPC;
        }
        MFS_ERROR_CSNOTPRESENT => {
            ret = ENXIO;
        }
        MFS_ERROR_NOTOPENED => {
            ret = EBADF;
        }
        MFS_ERROR_ENAMETOOLONG => {
            ret = ENAMETOOLONG;
        }
        MFS_ERROR_EMLINK => {
            ret = EMLINK;
        }
        MFS_ERROR_EBADF => {
            ret = EBADF;
        }
        MFS_ERROR_EFBIG => {
            ret = EFBIG;
        }
        MFS_ERROR_EISDIR => {
            ret = EISDIR;
        }
        _ => {
            ret = EINVAL;
        }
    }
    return ret;
}
pub const PKGVERSION: ::core::ffi::c_int = VERSMAJ * 1000000 as ::core::ffi::c_int
    + VERSMID * 10000 as ::core::ffi::c_int
    + (VERSMIN >> 1 as ::core::ffi::c_int) * 100 as ::core::ffi::c_int
    + RELEASE;
unsafe extern "C" fn mfsstatfs_to_statvfs(
    mut mfsstatfs: *mut mfs_int_statfsrec,
    mut stvfsbuf: *mut statvfs,
) {
    let bsize: uint32_t = 0x10000 as uint32_t;
    (*stvfsbuf).f_bsize = bsize as ::core::ffi::c_ulong;
    (*stvfsbuf).f_frsize = bsize as ::core::ffi::c_ulong;
    (*stvfsbuf).f_blocks =
        (*mfsstatfs).totalspace.wrapping_div(bsize as uint64_t) as __fsblkcnt64_t;
    (*stvfsbuf).f_bfree = (*mfsstatfs).freespace.wrapping_div(bsize as uint64_t) as __fsblkcnt64_t;
    (*stvfsbuf).f_bavail =
        (*mfsstatfs).availspace.wrapping_div(bsize as uint64_t) as __fsblkcnt64_t;
    (*stvfsbuf).f_files = ((1100000000 as ::core::ffi::c_int + PKGVERSION) as uint32_t)
        .wrapping_add((*mfsstatfs).inodes) as __fsfilcnt64_t;
    (*stvfsbuf).f_ffree = (1100000000 as ::core::ffi::c_int + PKGVERSION) as __fsfilcnt64_t;
    (*stvfsbuf).f_favail = (1100000000 as ::core::ffi::c_int + PKGVERSION) as __fsfilcnt64_t;
    (*stvfsbuf).f_namemax = MFS_NAME_MAX as ::core::ffi::c_ulong;
    (*stvfsbuf).f_fsid = (*mfsstatfs).sessionid as ::core::ffi::c_ulong;
}
unsafe extern "C" fn mfsstat_to_stat(mut mfsstat: *mut mfs_int_statrec, mut stbuf: *mut stat) {
    (*stbuf).st_ino = (*mfsstat).inode as __ino_t;
    (*stbuf).st_blksize = MFSBLOCKSIZE as __blksize_t;
    match (*mfsstat).r#type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int {
        TYPE_DIRECTORY => {
            (*stbuf).st_mode = (S_IFDIR | (*mfsstat).mode as ::core::ffi::c_int) as __mode_t;
        }
        TYPE_SYMLINK => {
            (*stbuf).st_mode = (S_IFLNK | (*mfsstat).mode as ::core::ffi::c_int) as __mode_t;
        }
        TYPE_FILE => {
            (*stbuf).st_mode = (S_IFREG | (*mfsstat).mode as ::core::ffi::c_int) as __mode_t;
        }
        TYPE_FIFO => {
            (*stbuf).st_mode = (S_IFIFO | (*mfsstat).mode as ::core::ffi::c_int) as __mode_t;
        }
        TYPE_SOCKET => {
            (*stbuf).st_mode = (S_IFSOCK | (*mfsstat).mode as ::core::ffi::c_int) as __mode_t;
        }
        TYPE_BLOCKDEV => {
            (*stbuf).st_mode = (S_IFBLK | (*mfsstat).mode as ::core::ffi::c_int) as __mode_t;
        }
        TYPE_CHARDEV => {
            (*stbuf).st_mode = (S_IFCHR | (*mfsstat).mode as ::core::ffi::c_int) as __mode_t;
        }
        _ => {
            (*stbuf).st_mode = 0 as __mode_t;
        }
    }
    (*stbuf).st_uid = (*mfsstat).uid as __uid_t;
    (*stbuf).st_gid = (*mfsstat).gid as __gid_t;
    (*stbuf).st_atim.tv_sec = (*mfsstat).atime as __time_t;
    (*stbuf).st_mtim.tv_sec = (*mfsstat).mtime as __time_t;
    (*stbuf).st_ctim.tv_sec = (*mfsstat).ctime as __time_t;
    (*stbuf).st_nlink = (*mfsstat).nlink as __nlink_t;
    (*stbuf).st_size = (*mfsstat).length as __off_t;
    (*stbuf).st_blocks = (*mfsstat)
        .length
        .wrapping_add(511 as uint64_t)
        .wrapping_div(512 as uint64_t) as __blkcnt_t;
    (*stbuf).st_rdev = (*mfsstat).dev as __dev_t;
}
pub const CRED_BASIC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CRED_UMASK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn mfs_get_credentials(mut ctx: *mut mfs_int_cred, mut mode: uint8_t) {
    static mut last_umask: mode_t = 0 as mode_t;
    let mut gids: [gid_t; 256] = [0; 256];
    let mut gid: gid_t = 0;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    (*ctx).uid = geteuid() as uint32_t;
    (*ctx).gidcnt = getgroups(MFS_NGROUPS_MAX, &raw mut gids as *mut __gid_t) as uint32_t;
    gid = getegid() as gid_t;
    (*ctx).gidtab[0 as usize] = gid as uint32_t;
    i = 0 as uint32_t;
    j = 1 as uint32_t;
    while i < (*ctx).gidcnt {
        if gids[i as usize] != gid {
            let c2rust_fresh0 = j;
            j = j.wrapping_add(1);
            (*ctx).gidtab[c2rust_fresh0 as usize] = gids[i as usize] as uint32_t;
        }
        i = i.wrapping_add(1);
    }
    (*ctx).gidcnt = j;
    if mode as ::core::ffi::c_int == CRED_UMASK {
        last_umask = umask(last_umask as __mode_t) as mode_t;
        umask(last_umask as __mode_t);
        (*ctx).umask = last_umask as uint16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mfs_mknod(
    mut path: *const ::core::ffi::c_char,
    mut mode: mode_t,
    mut dev: dev_t,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut r#type: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_UMASK as uint8_t);
    if mode & __S_IFMT as mode_t == 0o10000 as mode_t {
        r#type = TYPE_FIFO as uint8_t;
    } else if mode & __S_IFMT as mode_t == 0o20000 as mode_t {
        r#type = TYPE_CHARDEV as uint8_t;
    } else if mode & __S_IFMT as mode_t == 0o60000 as mode_t {
        r#type = TYPE_BLOCKDEV as uint8_t;
    } else if mode & __S_IFMT as mode_t == 0o140000 as mode_t {
        r#type = TYPE_SOCKET as uint8_t;
    } else if mode & __S_IFMT as mode_t == 0o100000 as mode_t
        || mode & 0o170000 as mode_t == 0 as mode_t
    {
        r#type = TYPE_FILE as uint8_t;
    } else {
        *__errno_location() = EPERM;
        return -1 as ::core::ffi::c_int;
    }
    status = mfs_int_mknod(&raw mut cr, path, r#type, mode as uint16_t, dev as uint32_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_unlink(mut path: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_unlink(&raw mut cr, path);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_mkdir(
    mut path: *const ::core::ffi::c_char,
    mut mode: mode_t,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_UMASK as uint8_t);
    status = mfs_int_mkdir(&raw mut cr, path, mode as uint16_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_rmdir(mut path: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_rmdir(&raw mut cr, path);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_rename(
    mut src: *const ::core::ffi::c_char,
    mut dst: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_rename(&raw mut cr, src, dst);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_link(
    mut src: *const ::core::ffi::c_char,
    mut dst: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_link(&raw mut cr, src, dst);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_symlink(
    mut path1: *const ::core::ffi::c_char,
    mut path2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_symlink(&raw mut cr, path1, path2);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_readlink(
    mut path: *const ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut lnkbuff: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut leng: ssize_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_readlink(
        &raw mut cr,
        path,
        &raw mut lnkbuff as *mut ::core::ffi::c_char,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    lnkbuff[(MFS_SYMLINK_MAX - 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
    leng = strlen(&raw mut lnkbuff as *mut ::core::ffi::c_char) as ssize_t;
    if leng as size_t > bufsize {
        leng = bufsize as ssize_t;
    }
    memcpy(
        buf as *mut ::core::ffi::c_void,
        &raw mut lnkbuff as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        leng as size_t,
    );
    return leng;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_chmod(
    mut path: *const ::core::ffi::c_char,
    mut mode: mode_t,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_chmod(&raw mut cr, path, mode as uint16_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fchmod(
    mut fildes: ::core::ffi::c_int,
    mut mode: mode_t,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_fchmod(&raw mut cr, fildes, mode as uint16_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_chown(
    mut path: *const ::core::ffi::c_char,
    mut owner: uid_t,
    mut group: gid_t,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_chown(
        &raw mut cr,
        path,
        if owner != -1 as ::core::ffi::c_int as uid_t {
            owner as uint32_t
        } else {
            0xffffffff as uint32_t
        },
        if group != -1 as ::core::ffi::c_int as gid_t {
            group as uint32_t
        } else {
            0xffffffff as uint32_t
        },
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fchown(
    mut fildes: ::core::ffi::c_int,
    mut owner: uid_t,
    mut group: gid_t,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_fchown(
        &raw mut cr,
        fildes,
        if owner != -1 as ::core::ffi::c_int as uid_t {
            owner as uint32_t
        } else {
            0xffffffff as uint32_t
        },
        if group != -1 as ::core::ffi::c_int as gid_t {
            group as uint32_t
        } else {
            0xffffffff as uint32_t
        },
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_utimes(
    mut path: *const ::core::ffi::c_char,
    mut times: *const timeval,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut flags: uint8_t = 0;
    let mut atime: uint32_t = 0;
    let mut mtime: uint32_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    if times.is_null() {
        flags = (MFS_TIMES_ATIME_NOW | MFS_TIMES_MTIME_NOW) as uint8_t;
        atime = 0 as uint32_t;
        mtime = 0 as uint32_t;
    } else {
        flags = 0 as uint8_t;
        atime = (*times.offset(0 as isize)).tv_sec as uint32_t;
        mtime = (*times.offset(1 as isize)).tv_sec as uint32_t;
    }
    status = mfs_int_utimes(&raw mut cr, path, flags, atime, mtime);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_futimes(
    mut fildes: ::core::ffi::c_int,
    mut times: *const timeval,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut flags: uint8_t = 0;
    let mut atime: uint32_t = 0;
    let mut mtime: uint32_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    if times.is_null() {
        flags = (MFS_TIMES_ATIME_NOW | MFS_TIMES_MTIME_NOW) as uint8_t;
        atime = 0 as uint32_t;
        mtime = 0 as uint32_t;
    } else {
        flags = 0 as uint8_t;
        atime = (*times.offset(0 as isize)).tv_sec as uint32_t;
        mtime = (*times.offset(1 as isize)).tv_sec as uint32_t;
    }
    status = mfs_int_futimes(&raw mut cr, fildes, flags, atime, mtime);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_futimens(
    mut fildes: ::core::ffi::c_int,
    mut times: *const timespec,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut flags: uint8_t = 0;
    let mut atime: uint32_t = 0;
    let mut mtime: uint32_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    atime = 0 as uint32_t;
    mtime = 0 as uint32_t;
    flags = 0 as uint8_t;
    if times.is_null() {
        flags = (MFS_TIMES_ATIME_NOW | MFS_TIMES_MTIME_NOW) as uint8_t;
    } else {
        if (*times.offset(0 as isize)).tv_nsec == UTIME_NOW {
            flags = (flags as ::core::ffi::c_int | MFS_TIMES_ATIME_NOW) as uint8_t;
        } else if (*times.offset(0 as isize)).tv_nsec == UTIME_OMIT {
            flags = (flags as ::core::ffi::c_int | MFS_TIMES_ATIME_OMIT) as uint8_t;
        } else {
            atime = (*times.offset(0 as isize)).tv_sec as uint32_t;
        }
        if (*times.offset(1 as isize)).tv_nsec == UTIME_NOW {
            flags = (flags as ::core::ffi::c_int | MFS_TIMES_MTIME_NOW) as uint8_t;
        } else if (*times.offset(1 as isize)).tv_nsec == UTIME_OMIT {
            flags = (flags as ::core::ffi::c_int | MFS_TIMES_MTIME_OMIT) as uint8_t;
        } else {
            mtime = (*times.offset(1 as isize)).tv_sec as uint32_t;
        }
    }
    status = mfs_int_futimes(&raw mut cr, fildes, flags, atime, mtime);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_truncate(
    mut path: *const ::core::ffi::c_char,
    mut size: off_t,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_truncate(&raw mut cr, path, size as int64_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_ftruncate(
    mut fildes: ::core::ffi::c_int,
    mut size: off_t,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_ftruncate(&raw mut cr, fildes, size as int64_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_lseek(
    mut fildes: ::core::ffi::c_int,
    mut offset: off_t,
    mut whence: ::core::ffi::c_int,
) -> off_t {
    let mut ioffset: int64_t = 0;
    let mut iwhence: uint8_t = 0;
    let mut status: uint8_t = 0;
    ioffset = offset as int64_t;
    match whence {
        SEEK_SET => {
            iwhence = MFS_SEEK_SET as uint8_t;
        }
        SEEK_CUR => {
            iwhence = MFS_SEEK_CUR as uint8_t;
        }
        SEEK_END => {
            iwhence = MFS_SEEK_END as uint8_t;
        }
        _ => {
            *__errno_location() = EINVAL;
            return -1 as off_t;
        }
    }
    status = mfs_int_lseek(fildes, &raw mut ioffset, iwhence);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as off_t;
    }
    return ioffset as off_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_statvfs(
    mut path: *const ::core::ffi::c_char,
    mut buf: *mut statvfs,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut stvfs: mfs_int_statfsrec = mfs_int_statfsrec {
        totalspace: 0,
        availspace: 0,
        freespace: 0,
        trashspace: 0,
        sustainedspace: 0,
        inodes: 0,
        masterip: 0,
        masterport: 0,
        sessionid: 0,
        masterprocessid: 0,
        masterversion: 0,
    };
    status = mfs_int_statfs(&raw mut stvfs);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    memset(
        buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<statvfs>(),
    );
    mfsstatfs_to_statvfs(&raw mut stvfs, buf);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fstatvfs(
    mut fildes: ::core::ffi::c_int,
    mut buf: *mut statvfs,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut stvfs: mfs_int_statfsrec = mfs_int_statfsrec {
        totalspace: 0,
        availspace: 0,
        freespace: 0,
        trashspace: 0,
        sustainedspace: 0,
        inodes: 0,
        masterip: 0,
        masterport: 0,
        sessionid: 0,
        masterprocessid: 0,
        masterversion: 0,
    };
    status = mfs_int_statfs(&raw mut stvfs);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    memset(
        buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<statvfs>(),
    );
    mfsstatfs_to_statvfs(&raw mut stvfs, buf);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_stat(
    mut path: *const ::core::ffi::c_char,
    mut buf: *mut stat,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    let mut st: mfs_int_statrec = mfs_int_statrec {
        inode: 0,
        r#type: 0,
        winattr: 0,
        mode: 0,
        uid: 0,
        gid: 0,
        atime: 0,
        mtime: 0,
        ctime: 0,
        nlink: 0,
        dev: 0,
        length: 0,
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_stat(&raw mut cr, path, &raw mut st);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    memset(
        buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stat>(),
    );
    mfsstat_to_stat(&raw mut st, buf);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fstat(
    mut fildes: ::core::ffi::c_int,
    mut buf: *mut stat,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    let mut st: mfs_int_statrec = mfs_int_statrec {
        inode: 0,
        r#type: 0,
        winattr: 0,
        mode: 0,
        uid: 0,
        gid: 0,
        atime: 0,
        mtime: 0,
        ctime: 0,
        nlink: 0,
        dev: 0,
        length: 0,
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_fstat(&raw mut cr, fildes, &raw mut st);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    memset(
        buf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stat>(),
    );
    mfsstat_to_stat(&raw mut st, buf);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_open(
    mut path: *const ::core::ffi::c_char,
    mut oflag: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    let mut ap: ::core::ffi::VaListImpl;
    let mut mfsoflag: ::core::ffi::c_int = 0;
    let mut mode: ::core::ffi::c_int = 0;
    let mut fildes: ::core::ffi::c_int = 0;
    if oflag & O_CREAT != 0 {
        ap = c2rust_args.clone();
        mode = ap.arg::<::core::ffi::c_int>();
        mfs_get_credentials(&raw mut cr, CRED_UMASK as uint8_t);
    } else {
        mode = 0 as ::core::ffi::c_int;
        mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    }
    mfsoflag = MFS_O_ACCMODE;
    match oflag & O_ACCMODE {
        O_RDONLY => {
            mfsoflag = MFS_O_RDONLY;
        }
        O_WRONLY => {
            mfsoflag = MFS_O_WRONLY;
        }
        O_RDWR => {
            mfsoflag = MFS_O_RDWR;
        }
        _ => {}
    }
    if oflag & O_CREAT != 0 {
        mfsoflag |= MFS_O_CREAT;
    }
    if oflag & O_TRUNC != 0 {
        mfsoflag |= MFS_O_TRUNC;
    }
    if oflag & O_EXCL != 0 {
        mfsoflag |= MFS_O_EXCL;
    }
    if oflag & O_APPEND != 0 {
        mfsoflag |= MFS_O_APPEND;
    }
    status = mfs_int_open(&raw mut cr, &raw mut fildes, path, mfsoflag, mode);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return fildes;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_pread(
    mut fildes: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_void,
    mut nbyte: size_t,
    mut offset: off_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut rsize: int64_t = 0;
    status = mfs_int_pread(
        fildes,
        &raw mut rsize,
        buf as *mut uint8_t,
        nbyte as uint64_t,
        offset as uint64_t,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    return rsize as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_read(
    mut fildes: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_void,
    mut nbyte: size_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut rsize: int64_t = 0;
    status = mfs_int_read(
        fildes,
        &raw mut rsize,
        buf as *mut uint8_t,
        nbyte as uint64_t,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    return rsize as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_pwrite(
    mut fildes: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_void,
    mut nbyte: size_t,
    mut offset: off_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut rsize: int64_t = 0;
    status = mfs_int_pwrite(
        fildes,
        &raw mut rsize,
        buf as *const uint8_t,
        nbyte as uint64_t,
        offset as uint64_t,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    return rsize as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_write(
    mut fildes: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_void,
    mut nbyte: size_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut rsize: int64_t = 0;
    status = mfs_int_write(
        fildes,
        &raw mut rsize,
        buf as *const uint8_t,
        nbyte as uint64_t,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    return rsize as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fsync(mut fildes: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    status = mfs_int_fsync(fildes);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_close(mut fildes: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    status = mfs_int_close(fildes);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_flock(
    mut fildes: ::core::ffi::c_int,
    mut op: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut mfsop: uint8_t = 0;
    mfsop = 0 as uint8_t;
    if op & LOCK_SH != 0 {
        mfsop = (mfsop as ::core::ffi::c_int | MFS_LOCK_SH) as uint8_t;
    }
    if op & LOCK_EX != 0 {
        mfsop = (mfsop as ::core::ffi::c_int | MFS_LOCK_EX) as uint8_t;
    }
    if op & LOCK_NB != 0 {
        mfsop = (mfsop as ::core::ffi::c_int | MFS_LOCK_NB) as uint8_t;
    }
    if op & LOCK_UN != 0 {
        mfsop = (mfsop as ::core::ffi::c_int | MFS_LOCK_UN) as uint8_t;
    }
    status = mfs_int_flock(fildes, mfsop);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_lockf(
    mut fildes: ::core::ffi::c_int,
    mut function: ::core::ffi::c_int,
    mut size: off_t,
) -> ::core::ffi::c_int {
    let mut mfsfunction: uint8_t = 0;
    let mut status: uint8_t = 0;
    match function {
        F_ULOCK => {
            mfsfunction = MFS_F_ULOCK as uint8_t;
        }
        F_LOCK => {
            mfsfunction = MFS_F_LOCK as uint8_t;
        }
        F_TLOCK => {
            mfsfunction = MFS_F_TLOCK as uint8_t;
        }
        F_TEST => {
            mfsfunction = MFS_F_TEST as uint8_t;
        }
        _ => {
            *__errno_location() = EINVAL;
            return -1 as ::core::ffi::c_int;
        }
    }
    status = mfs_int_lockf(fildes, getpid() as uint32_t, mfsfunction, size as int64_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fcntl_locks(
    mut fildes: ::core::ffi::c_int,
    mut function: ::core::ffi::c_int,
    mut fl: *mut flock,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut mfsfl: mfs_int_flockrec = mfs_int_flockrec {
        r#type: 0,
        whence: 0,
        start: 0,
        len: 0,
        pid: 0,
    };
    let mut mfsfunction: uint8_t = 0;
    memset(
        &raw mut mfsfl as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mfs_int_flockrec>(),
    );
    if (*fl).l_whence as ::core::ffi::c_int == SEEK_CUR {
        mfsfl.whence = MFS_SEEK_CUR as uint8_t;
    } else if (*fl).l_whence as ::core::ffi::c_int == SEEK_SET {
        mfsfl.whence = MFS_SEEK_SET as uint8_t;
    } else if (*fl).l_whence as ::core::ffi::c_int == SEEK_END {
        mfsfl.whence = MFS_SEEK_END as uint8_t;
    } else {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    mfsfl.start = (*fl).l_start as int64_t;
    mfsfl.len = (*fl).l_len as int64_t;
    if (*fl).l_type as ::core::ffi::c_int == F_UNLCK {
        mfsfl.r#type = MFS_F_UNLCK as uint8_t;
    } else if (*fl).l_type as ::core::ffi::c_int == F_RDLCK {
        mfsfl.r#type = MFS_F_RDLCK as uint8_t;
    } else if (*fl).l_type as ::core::ffi::c_int == F_WRLCK {
        mfsfl.r#type = MFS_F_WRLCK as uint8_t;
    } else {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    if function == F_GETLK {
        mfsfunction = MFS_F_GETLK as uint8_t;
    } else if function == F_SETLK {
        mfsfunction = MFS_F_SETLK as uint8_t;
    } else if function == F_SETLKW {
        mfsfunction = MFS_F_SETLKW as uint8_t;
    } else {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    status = mfs_int_fcntl_locks(fildes, getpid() as uint32_t, mfsfunction, &raw mut mfsfl);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    if function == F_GETLK {
        memset(
            fl as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<flock>(),
        );
        if mfsfl.r#type as ::core::ffi::c_int == MFS_F_RDLCK {
            (*fl).l_type = F_RDLCK as ::core::ffi::c_short;
        } else if mfsfl.r#type as ::core::ffi::c_int == MFS_F_WRLCK {
            (*fl).l_type = F_WRLCK as ::core::ffi::c_short;
        } else {
            (*fl).l_type = F_UNLCK as ::core::ffi::c_short;
        }
        (*fl).l_whence = SEEK_SET as ::core::ffi::c_short;
        (*fl).l_start = mfsfl.start as __off64_t;
        (*fl).l_len = mfsfl.len as __off64_t;
        (*fl).l_pid = mfsfl.pid as __pid_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_getxattr(
    mut path: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut mode: uint8_t = 0;
    let mut vbuff: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut vleng: uint32_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mode = (if size == 0 as size_t {
        MFS_XATTR_LENGTH_ONLY
    } else {
        MFS_XATTR_GETA_DATA
    }) as uint8_t;
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_getxattr(
        &raw mut cr,
        path,
        name,
        &raw mut vbuff,
        &raw mut vleng,
        mode,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    if size > 0 as size_t {
        if vleng as size_t > size {
            *__errno_location() = ERANGE;
            return -1 as ssize_t;
        }
        if vleng > 0 as uint32_t {
            memcpy(value, vbuff as *const ::core::ffi::c_void, vleng as size_t);
        }
    }
    return vleng as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fgetxattr(
    mut fildes: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut mode: uint8_t = 0;
    let mut vbuff: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut vleng: uint32_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mode = (if size == 0 as size_t {
        MFS_XATTR_LENGTH_ONLY
    } else {
        MFS_XATTR_GETA_DATA
    }) as uint8_t;
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_fgetxattr(
        &raw mut cr,
        fildes,
        name,
        &raw mut vbuff,
        &raw mut vleng,
        mode,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    if size > 0 as size_t {
        if vleng as size_t > size {
            *__errno_location() = ERANGE;
            return -1 as ssize_t;
        }
        if vleng > 0 as uint32_t {
            memcpy(value, vbuff as *const ::core::ffi::c_void, vleng as size_t);
        }
    }
    return vleng as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_setxattr(
    mut path: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_void,
    mut size: size_t,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut mode: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    if size > MFS_XATTR_SIZE_MAX as size_t {
        *__errno_location() = ERANGE;
        return -1 as ::core::ffi::c_int;
    }
    mode = (if flags == XATTR_CREATE {
        MFS_XATTR_CREATE_ONLY
    } else if flags == XATTR_REPLACE {
        MFS_XATTR_REPLACE_ONLY
    } else {
        MFS_XATTR_CREATE_OR_REPLACE
    }) as uint8_t;
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_setxattr(
        &raw mut cr,
        path,
        name,
        value as *const uint8_t,
        size as uint32_t,
        mode,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fsetxattr(
    mut fildes: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_void,
    mut size: size_t,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut mode: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    if size > MFS_XATTR_SIZE_MAX as size_t {
        *__errno_location() = ERANGE;
        return -1 as ::core::ffi::c_int;
    }
    mode = (if flags == XATTR_CREATE {
        MFS_XATTR_CREATE_ONLY
    } else if flags == XATTR_REPLACE {
        MFS_XATTR_REPLACE_ONLY
    } else {
        MFS_XATTR_CREATE_OR_REPLACE
    }) as uint8_t;
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_fsetxattr(
        &raw mut cr,
        fildes,
        name,
        value as *const uint8_t,
        size as uint32_t,
        mode,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_listxattr(
    mut path: *const ::core::ffi::c_char,
    mut list: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut rsize: int32_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_listxattr(&raw mut cr, path, &raw mut rsize, list, size as uint32_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    return rsize as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_flistxattr(
    mut fildes: ::core::ffi::c_int,
    mut list: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ssize_t {
    let mut status: uint8_t = 0;
    let mut rsize: int32_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_flistxattr(&raw mut cr, fildes, &raw mut rsize, list, size as uint32_t);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ssize_t;
    }
    return rsize as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_removexattr(
    mut path: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_removexattr(&raw mut cr, path, name);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fremovexattr(
    mut fildes: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_fremovexattr(&raw mut cr, fildes, name);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_acl_alloc(mut namedaclscnt: uint32_t) -> *mut mfsacl {
    if namedaclscnt == 0 as uint32_t {
        return malloc(::core::mem::size_of::<mfsacl>()) as *mut mfsacl;
    } else {
        return malloc(
            ::core::mem::size_of::<mfsacl>().wrapping_add(
                ::core::mem::size_of::<mfsaclid>()
                    .wrapping_mul(namedaclscnt.wrapping_sub(1 as uint32_t) as size_t),
            ),
        ) as *mut mfsacl;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mfs_acl_free(mut aclrec: *mut mfsacl) {
    free(aclrec as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mfs_getfacl(
    mut path: *const ::core::ffi::c_char,
    mut acltype: uint8_t,
    mut aclrec: *mut *mut mfsacl,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    let mut namedacls: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut namedaclsize: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut namedaclscnt: uint32_t = 0;
    let mut userperm: uint16_t = 0;
    let mut groupperm: uint16_t = 0;
    let mut otherperm: uint16_t = 0;
    let mut maskperm: uint16_t = 0;
    let mut nuserscnt: uint16_t = 0;
    let mut ngroupscnt: uint16_t = 0;
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_getfacl(
        &raw mut cr,
        path,
        acltype,
        &raw mut userperm,
        &raw mut groupperm,
        &raw mut otherperm,
        &raw mut maskperm,
        &raw mut nuserscnt,
        &raw mut ngroupscnt,
        &raw mut namedacls,
        &raw mut namedaclsize,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    namedaclscnt = (nuserscnt as ::core::ffi::c_int + ngroupscnt as ::core::ffi::c_int) as uint32_t;
    if namedaclscnt.wrapping_mul(6 as uint32_t) != namedaclsize {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    *aclrec = mfs_acl_alloc(namedaclscnt);
    if (*aclrec).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (**aclrec).userperm = userperm;
    (**aclrec).groupperm = groupperm;
    (**aclrec).otherperm = otherperm;
    (**aclrec).maskperm = maskperm;
    (**aclrec).nuserscnt = nuserscnt;
    (**aclrec).ngroupscnt = ngroupscnt;
    i = 0 as uint32_t;
    while i < namedaclscnt {
        (*(&raw mut (**aclrec).namedacls as *mut mfsaclid).offset(i as isize)).id =
            get32bit(&raw mut namedacls);
        (*(&raw mut (**aclrec).namedacls as *mut mfsaclid).offset(i as isize)).perm =
            get16bit(&raw mut namedacls);
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fgetfacl(
    mut filedes: ::core::ffi::c_int,
    mut acltype: uint8_t,
    mut aclrec: *mut *mut mfsacl,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    let mut namedacls: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut namedaclsize: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut namedaclscnt: uint32_t = 0;
    let mut userperm: uint16_t = 0;
    let mut groupperm: uint16_t = 0;
    let mut otherperm: uint16_t = 0;
    let mut maskperm: uint16_t = 0;
    let mut nuserscnt: uint16_t = 0;
    let mut ngroupscnt: uint16_t = 0;
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    status = mfs_int_fgetfacl(
        &raw mut cr,
        filedes,
        acltype,
        &raw mut userperm,
        &raw mut groupperm,
        &raw mut otherperm,
        &raw mut maskperm,
        &raw mut nuserscnt,
        &raw mut ngroupscnt,
        &raw mut namedacls,
        &raw mut namedaclsize,
    );
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    namedaclscnt = (nuserscnt as ::core::ffi::c_int + ngroupscnt as ::core::ffi::c_int) as uint32_t;
    if namedaclscnt.wrapping_mul(6 as uint32_t) != namedaclsize {
        *__errno_location() = EINVAL;
        return -1 as ::core::ffi::c_int;
    }
    *aclrec = mfs_acl_alloc(namedaclscnt);
    if (*aclrec).is_null() {
        return -1 as ::core::ffi::c_int;
    }
    (**aclrec).userperm = userperm;
    (**aclrec).groupperm = groupperm;
    (**aclrec).otherperm = otherperm;
    (**aclrec).maskperm = maskperm;
    (**aclrec).nuserscnt = nuserscnt;
    (**aclrec).ngroupscnt = ngroupscnt;
    i = 0 as uint32_t;
    while i < namedaclscnt {
        (*(&raw mut (**aclrec).namedacls as *mut mfsaclid).offset(i as isize)).id =
            get32bit(&raw mut namedacls);
        (*(&raw mut (**aclrec).namedacls as *mut mfsaclid).offset(i as isize)).perm =
            get16bit(&raw mut namedacls);
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_setfacl(
    mut path: *const ::core::ffi::c_char,
    mut acltype: uint8_t,
    mut aclrec: *mut mfsacl,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    let mut namedacls: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut namedaclsize: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut namedaclscnt: uint32_t = 0;
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    namedaclscnt = ((*aclrec).nuserscnt as ::core::ffi::c_int
        + (*aclrec).ngroupscnt as ::core::ffi::c_int) as uint32_t;
    namedaclsize = (6 as uint32_t).wrapping_mul(namedaclscnt);
    namedacls = malloc(namedaclsize as size_t) as *mut uint8_t;
    wptr = namedacls;
    i = 0 as uint32_t;
    while i < namedaclscnt {
        put32bit(
            &raw mut wptr,
            (*(&raw mut (*aclrec).namedacls as *mut mfsaclid).offset(i as isize)).id,
        );
        put16bit(
            &raw mut wptr,
            (*(&raw mut (*aclrec).namedacls as *mut mfsaclid).offset(i as isize)).perm,
        );
        i = i.wrapping_add(1);
    }
    status = mfs_int_setfacl(
        &raw mut cr,
        path,
        acltype,
        (*aclrec).userperm,
        (*aclrec).groupperm,
        (*aclrec).otherperm,
        (*aclrec).maskperm,
        (*aclrec).nuserscnt,
        (*aclrec).ngroupscnt,
        namedacls,
        namedaclsize,
    );
    free(namedacls as *mut ::core::ffi::c_void);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_fsetfacl(
    mut filedes: ::core::ffi::c_int,
    mut acltype: uint8_t,
    mut aclrec: *mut mfsacl,
) -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    let mut cr: mfs_int_cred = mfs_int_cred {
        umask: 0,
        uid: 0,
        gidcnt: 0,
        gidtab: [0; 256],
    };
    let mut namedacls: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut namedaclsize: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut namedaclscnt: uint32_t = 0;
    mfs_get_credentials(&raw mut cr, CRED_BASIC as uint8_t);
    namedaclscnt = ((*aclrec).nuserscnt as ::core::ffi::c_int
        + (*aclrec).ngroupscnt as ::core::ffi::c_int) as uint32_t;
    namedaclsize = (6 as uint32_t).wrapping_mul(namedaclscnt);
    namedacls = malloc(namedaclsize as size_t) as *mut uint8_t;
    wptr = namedacls;
    i = 0 as uint32_t;
    while i < namedaclscnt {
        put32bit(
            &raw mut wptr,
            (*(&raw mut (*aclrec).namedacls as *mut mfsaclid).offset(i as isize)).id,
        );
        put16bit(
            &raw mut wptr,
            (*(&raw mut (*aclrec).namedacls as *mut mfsaclid).offset(i as isize)).perm,
        );
        i = i.wrapping_add(1);
    }
    status = mfs_int_fsetfacl(
        &raw mut cr,
        filedes,
        acltype,
        (*aclrec).userperm,
        (*aclrec).groupperm,
        (*aclrec).otherperm,
        (*aclrec).maskperm,
        (*aclrec).nuserscnt,
        (*aclrec).ngroupscnt,
        namedacls,
        namedaclsize,
    );
    free(namedacls as *mut ::core::ffi::c_void);
    if status as ::core::ffi::c_int != MFS_STATUS_OK {
        *__errno_location() = mfs_errorconv(status as ::core::ffi::c_int);
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_set_defaults(mut mcfg: *mut mfscfg) {
    memset(
        mcfg as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<mfscfg>(),
    );
    (*mcfg).masterhost = strdup(DEFAULT_MASTERNAME.as_ptr());
    (*mcfg).masterport = strdup(DEFAULT_MASTER_CLIENT_PORT.as_ptr());
    (*mcfg).masterpath = strdup(b"/\0".as_ptr() as *const ::core::ffi::c_char);
    (*mcfg).masterbind = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mcfg).masterpassword = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mcfg).mastermd5pass = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mcfg).mountpoint = strdup(b"[MFSIO]\0".as_ptr() as *const ::core::ffi::c_char);
    (*mcfg).preferedlabels = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mcfg).read_cache_mb = 128 as ::core::ffi::c_int;
    (*mcfg).write_cache_mb = 128 as ::core::ffi::c_int;
    (*mcfg).io_try_cnt = 30 as ::core::ffi::c_int;
    (*mcfg).io_timeout = 0 as ::core::ffi::c_int;
    (*mcfg).min_log_entry = 5 as ::core::ffi::c_int;
    (*mcfg).readahead_leng = 0x200000 as ::core::ffi::c_int;
    (*mcfg).readahead_trigger = 10 as ::core::ffi::c_int * 0x200000 as ::core::ffi::c_int;
    (*mcfg).lcache_retention = 1.0f64;
    (*mcfg).logident = strdup(b"libmfsio\0".as_ptr() as *const ::core::ffi::c_char);
    (*mcfg).logdaemon = 0 as ::core::ffi::c_int;
    (*mcfg).logminlevel = MFSLOG_INFO;
    (*mcfg).logelevateto = MFSLOG_NOTICE;
    (*mcfg).master_min_version_maj = 0 as uint16_t;
    (*mcfg).master_min_version_mid = 0 as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_init(mut mcfg: *mut mfscfg, mut stage: uint8_t) -> ::core::ffi::c_int {
    let mut mcfgi: mfs_int_cfg = mfs_int_cfg {
        masterhost: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        masterport: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        masterbind: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        masterpassword: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        mastermd5pass: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        mountpoint: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        masterpath: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        preferedlabels: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        read_cache_mb: 0,
        write_cache_mb: 0,
        io_try_cnt: 0,
        io_timeout: 0,
        min_log_entry: 0,
        readahead_leng: 0,
        readahead_trigger: 0,
        error_on_lost_chunk: 0,
        error_on_no_space: 0,
        sugid_clear_mode: 0,
        mkdir_copy_sgid: 0,
        lcache_retention: 0.,
        logident: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        logdaemon: 0,
        logminlevel: 0,
        logelevateto: 0,
        master_min_version_maj: 0,
        master_min_version_mid: 0,
    };
    mcfgi.masterhost = (*mcfg).masterhost;
    mcfgi.masterport = (*mcfg).masterport;
    mcfgi.masterbind = (*mcfg).masterbind;
    mcfgi.masterpassword = (*mcfg).masterpassword;
    mcfgi.mastermd5pass = (*mcfg).mastermd5pass;
    mcfgi.mountpoint = (*mcfg).mountpoint;
    mcfgi.masterpath = (*mcfg).masterpath;
    mcfgi.preferedlabels = (*mcfg).preferedlabels;
    mcfgi.read_cache_mb = (*mcfg).read_cache_mb;
    mcfgi.write_cache_mb = (*mcfg).write_cache_mb;
    mcfgi.io_try_cnt = (*mcfg).io_try_cnt;
    mcfgi.io_timeout = (*mcfg).io_timeout;
    mcfgi.min_log_entry = (*mcfg).min_log_entry;
    mcfgi.readahead_leng = (*mcfg).readahead_leng;
    mcfgi.readahead_trigger = (*mcfg).readahead_trigger;
    mcfgi.error_on_lost_chunk = (*mcfg).error_on_lost_chunk;
    mcfgi.error_on_no_space = (*mcfg).error_on_no_space;
    mcfgi.sugid_clear_mode = (*mcfg).sugid_clear_mode;
    mcfgi.mkdir_copy_sgid = (*mcfg).mkdir_copy_sgid;
    mcfgi.lcache_retention = (*mcfg).lcache_retention;
    mcfgi.logident = (*mcfg).logident;
    mcfgi.logdaemon = (*mcfg).logdaemon;
    mcfgi.logminlevel = (*mcfg).logminlevel;
    mcfgi.logelevateto = (*mcfg).logelevateto;
    mcfgi.master_min_version_maj = (*mcfg).master_min_version_maj;
    mcfgi.master_min_version_mid = (*mcfg).master_min_version_mid;
    return mfs_int_init(&raw mut mcfgi, stage);
}
#[no_mangle]
pub unsafe extern "C" fn mfs_term() {
    mfs_int_term();
}
