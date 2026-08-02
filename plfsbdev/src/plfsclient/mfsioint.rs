pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn memcpy(
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
    unsafe fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn fs_getmasterparams(
        mip: *mut uint32_t,
        mport: *mut uint16_t,
        sid: *mut uint32_t,
        mver: *mut uint32_t,
        mprocid: *mut uint64_t,
    );
    unsafe fn fs_atime(inode: uint32_t);
    unsafe fn fs_mtime(inode: uint32_t);
    unsafe fn fs_fix_amtime(inode: uint32_t, atime: *mut uint32_t, mtime: *mut uint32_t);
    unsafe fn fs_inc_acnt(inode: uint32_t);
    unsafe fn fs_dec_acnt(inode: uint32_t);
    unsafe fn fs_read_notify(bytes: uint64_t);
    unsafe fn fs_write_notify(bytes: uint64_t);
    unsafe fn fs_fsync_notify();
    unsafe fn fs_get_cfg(
        opt_name: *const ::core::ffi::c_char,
        oleng: *mut uint8_t,
        odata: *mut *const uint8_t,
    ) -> uint8_t;
    unsafe fn fs_get_cfg_file(
        opt_name: *const ::core::ffi::c_char,
        oleng: *mut uint16_t,
        odata: *mut *const uint8_t,
    ) -> uint8_t;
    unsafe fn fs_statfs(
        totalspace: *mut uint64_t,
        availspace: *mut uint64_t,
        freespace: *mut uint64_t,
        trashspace: *mut uint64_t,
        sustainedspace: *mut uint64_t,
        inodes: *mut uint32_t,
    );
    unsafe fn fs_getattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gid: uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_setattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        setmask: uint8_t,
        attrmode: uint16_t,
        attruid: uint32_t,
        attrgid: uint32_t,
        attratime: uint32_t,
        attrmtime: uint32_t,
        winattr: uint8_t,
        sugidclearmode: uint8_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_readlink(inode: uint32_t, path: *mut *const uint8_t) -> uint8_t;
    unsafe fn fs_symlink(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        path: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mknod(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        r#type: uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        rdev: uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mkdir(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        copysgid: uint8_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_unlink(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_rmdir(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_rename(
        parent_src: uint32_t,
        nleng_src: uint8_t,
        name_src: *const uint8_t,
        parent_dst: uint32_t,
        nleng: uint8_t,
        name_dst: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        mfsflags: uint8_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_link(
        inode_src: uint32_t,
        parent_dst: uint32_t,
        nleng_dst: uint8_t,
        name_dst: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_readdir(
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        edgeid: *mut uint64_t,
        edgelimit: uint32_t,
        wantattr: uint8_t,
        addtocache: uint8_t,
        dbuff: *mut *const uint8_t,
        dbuffsize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_create(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
        oflags: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_opencheck(
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        flags: uint8_t,
        attr: *mut uint8_t,
        oflags: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_flock(inode: uint32_t, reqid: uint32_t, owner: uint64_t, cmd: uint8_t) -> uint8_t;
    unsafe fn fs_posixlock(
        inode: uint32_t,
        reqid: uint32_t,
        owner: uint64_t,
        cmd: uint8_t,
        r#type: uint8_t,
        start: uint64_t,
        end: uint64_t,
        pid: uint32_t,
        rtype: *mut uint8_t,
        rstart: *mut uint64_t,
        rend: *mut uint64_t,
        rpid: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_getfacl(
        inode: uint32_t,
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
    unsafe fn fs_setfacl(
        inode: uint32_t,
        uid: uint32_t,
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
    unsafe fn fs_getxattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        mode: uint8_t,
        vbuff: *mut *const uint8_t,
        vleng: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_listxattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        mode: uint8_t,
        dbuff: *mut *const uint8_t,
        dleng: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_setxattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        vleng: uint32_t,
        value: *const uint8_t,
        mode: uint8_t,
    ) -> uint8_t;
    unsafe fn fs_removexattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
    ) -> uint8_t;
    unsafe fn master_version() -> uint32_t;
    unsafe fn master_attrsize() -> uint8_t;
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
    unsafe fn csorder_init(labelexpr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    unsafe fn read_data(
        vid: *mut ::core::ffi::c_void,
        offset: uint64_t,
        size: *mut uint32_t,
        rhead: *mut *mut ::core::ffi::c_void,
        iov: *mut *mut iovec,
        iovcnt: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn read_data_free_buff(
        vid: *mut ::core::ffi::c_void,
        vrhead: *mut ::core::ffi::c_void,
        iov: *mut iovec,
        iovcnt: uint32_t,
    );
    unsafe fn read_inode_clear_cache(inode: uint32_t, offset: uint64_t, leng: uint64_t);
    unsafe fn read_inode_set_length_active(inode: uint32_t, newlength: uint64_t);
    unsafe fn read_inode_set_length_passive(inode: uint32_t, newlength: uint64_t);
    unsafe fn read_data_new(inode: uint32_t, fleng: uint64_t) -> *mut ::core::ffi::c_void;
    unsafe fn read_data_end(vid: *mut ::core::ffi::c_void);
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
    unsafe fn write_data_new(inode: uint32_t, fleng: uint64_t) -> *mut ::core::ffi::c_void;
    unsafe fn write_data_end(vid: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    unsafe fn write_data_flush(vid: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    unsafe fn write_data_inode_setmaxfleng(inode: uint32_t, maxfleng: uint64_t);
    unsafe fn write_data_inode_getmaxfleng(inode: uint32_t) -> uint64_t;
    unsafe fn write_data_flush_inode(inode: uint32_t) -> ::core::ffi::c_int;
    unsafe fn write_data(
        vid: *mut ::core::ffi::c_void,
        offset: uint64_t,
        size: uint32_t,
        buff: *const uint8_t,
        superuser: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn write_init();
    unsafe fn write_term();
    unsafe fn do_truncate(
        inode: uint32_t,
        flags: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        attrlength: uint64_t,
        attr: *mut uint8_t,
        prevlength: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn delay_term();
    unsafe fn delay_init();
    unsafe fn conncache_term();
    unsafe fn conncache_init(capacity: uint32_t) -> ::core::ffi::c_int;
    unsafe fn mycrc32_init();
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn strerr_init();
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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn md5_init(ctx: *mut md5ctx);
    unsafe fn md5_update(ctx: *mut md5ctx, buff: *const uint8_t, leng: uint32_t);
    unsafe fn md5_final(digest: *mut uint8_t, ctx: *mut md5ctx);
    unsafe fn lcache_path_lookup(
        base_inode: uint32_t,
        pleng: uint32_t,
        path: *const uint8_t,
        uid: uint32_t,
        gidcnt: uint32_t,
        gidtab: *mut uint32_t,
        parent_inode: *mut uint32_t,
        last_inode: *mut uint32_t,
        nleng: *mut uint8_t,
        name: *mut uint8_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn lcache_path_invalidate(base_inode: uint32_t, pleng: uint32_t, path: *const uint8_t);
    unsafe fn lcache_inode_invalidate(inode: uint32_t);
    unsafe fn lcache_term();
    unsafe fn lcache_init(lc_retention: ::core::ffi::c_double) -> ::core::ffi::c_int;
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
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
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
pub struct _mfs_int_direntry {
    pub inode: uint32_t,
    pub r#type: uint8_t,
    pub name: [uint8_t; 256],
}
pub type mfs_int_direntry = _mfs_int_direntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfs_int_direntryplus {
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
    pub name: [uint8_t; 256],
}
pub type mfs_int_direntryplus = _mfs_int_direntryplus;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _data_buff {
    pub leng: uint32_t,
    pub data: [uint8_t; 1],
}
pub type data_buff = _data_buff;
#[repr(C)]
pub struct file_info {
    pub flengptr: Option<plfsclient::inoleng::Handle>,
    pub inode: uint32_t,
    pub mode: uint8_t,
    pub writing: uint8_t,
    pub reading: uint8_t,
    pub privuser: uint8_t,
    pub wasread: uint8_t,
    pub dataformat: uint8_t,
    pub offset: uint64_t,
    pub readers_cnt: uint32_t,
    pub writers_cnt: uint32_t,
    pub rdata: *mut ::core::ffi::c_void,
    pub wdata: *mut ::core::ffi::c_void,
    pub dbuff: *mut uint8_t,
    pub dbuffsize: uint64_t,
    pub lock: std::sync::Mutex<()>,
    pub rwcond: std::sync::Condvar,
}
pub const MFS_IO_FORBIDDEN: C2Rust_Unnamed_0 = 6;
pub const MFS_IO_DIRECTORY: C2Rust_Unnamed_0 = 7;
pub const MFS_IO_ATTRONLY: C2Rust_Unnamed_0 = 3;
pub const MFS_IO_READONLY: C2Rust_Unnamed_0 = 1;
pub const MFS_IO_READWRITE: C2Rust_Unnamed_0 = 0;
pub const MFS_IO_READAPPEND: C2Rust_Unnamed_0 = 4;
pub const MFS_IO_WRITEONLY: C2Rust_Unnamed_0 = 2;
pub const MFS_IO_APPENDONLY: C2Rust_Unnamed_0 = 5;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const MFS_ROOT_ID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_SYMLINK_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MFS_MAX_FILE_SIZE: uint64_t = (MFSCHUNKSIZE as uint64_t) << 31 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOTDIR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_ERROR_EACCES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_ERROR_EEXIST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_CHUNKLOST: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MFS_ERROR_OUTOFMEMORY: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSPACE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const MFS_ERROR_IO: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const MFS_ERROR_QUOTA: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const MFS_ERROR_ERANGE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTFOUND: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const MFS_ERROR_EBADF: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
pub const MFS_ERROR_EFBIG: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
pub const MFS_ERROR_EISDIR: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DISP_TYPE_FILE: ::core::ffi::c_int = 102;
pub const DISP_TYPE_DIRECTORY: ::core::ffi::c_int = 100;
pub const DISP_TYPE_SYMLINK: ::core::ffi::c_int = 108;
pub const DISP_TYPE_FIFO: ::core::ffi::c_int = 113;
pub const DISP_TYPE_BLOCKDEV: ::core::ffi::c_int = 98;
pub const DISP_TYPE_CHARDEV: ::core::ffi::c_int = 99;
pub const DISP_TYPE_SOCKET: ::core::ffi::c_int = 115;
pub const DISP_TYPE_TRASH: ::core::ffi::c_int = 116;
pub const DISP_TYPE_SUSTAINED: ::core::ffi::c_int = 114;
pub const TYPE_FILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TYPE_DIRECTORY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TYPE_SYMLINK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TYPE_FIFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TYPE_BLOCKDEV: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TYPE_CHARDEV: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TYPE_SOCKET: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const TYPE_TRASH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TYPE_SUSTAINED: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SET_WINATTR_FLAG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SET_MODE_FLAG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SET_UID_FLAG: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SET_GID_FLAG: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SET_MTIME_NOW_FLAG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SET_MTIME_FLAG: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SET_ATIME_FLAG: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SET_ATIME_NOW_FLAG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const FLOCK_UNLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLOCK_TRY_SHARED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLOCK_LOCK_SHARED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FLOCK_TRY_EXCLUSIVE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FLOCK_LOCK_EXCLUSIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_GET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_SET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_TRY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const POSIX_LOCK_UNLCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POSIX_LOCK_RDLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_LOCK_WRLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_OPENED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_UPDATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_RESERVE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_EXT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OPEN_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPEN_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPEN_TRUNCATE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPEN_APPENDONLY: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MFS_XATTR_GETA_DATA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_XATTR_LENGTH_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ATTR_RECORD_SIZE: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const DEFAULT_MASTERNAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"mfsmaster\0") };
pub const DEFAULT_MASTER_CLIENT_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9421\0") };
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const EDQUOT: ::core::ffi::c_int = 122;
#[inline]
unsafe extern "C" fn get64bit(mut ptr: *mut *const uint8_t) -> uint64_t {
    unsafe {
        let mut t64: uint64_t = 0;
        memcpy(
            &raw mut t64 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            8 as size_t,
        );
        *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
        return t64.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    unsafe {
        let mut t32: uint32_t = 0;
        memcpy(
            &raw mut t32 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
        return t32.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get16bit(mut ptr: *mut *const uint8_t) -> uint16_t {
    unsafe {
        let mut t16: uint16_t = 0;
        memcpy(
            &raw mut t16 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
        return t16.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get8bit(mut ptr: *mut *const uint8_t) -> uint8_t {
    unsafe {
        let mut t8: uint8_t = 0;
        t8 = *(*ptr).offset(0 as isize);
        *ptr = (*ptr).offset(1);
        return t8;
    }
}
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5;
pub const ENXIO: ::core::ffi::c_int = 6;
pub const EBADF: ::core::ffi::c_int = 9;
pub const EINVAL: ::core::ffi::c_int = 22;
pub const EFBIG: ::core::ffi::c_int = 27;
pub const ENOSPC: ::core::ffi::c_int = 28;
pub const MFS_TIMES_ATIME_NOW: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_TIMES_MTIME_NOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_TIMES_ATIME_OMIT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_TIMES_MTIME_OMIT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MFS_SEEK_SET: ::core::ffi::c_int = 0;
pub const MFS_SEEK_CUR: ::core::ffi::c_int = 1;
pub const MFS_SEEK_END: ::core::ffi::c_int = 2;
pub const MFS_O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_O_WRONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_O_RDWR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_O_ATTRONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
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
pub const MAX_FILE_SIZE: int64_t = MFS_MAX_FILE_SIZE as int64_t;
pub const PATH_TO_INODES_EXPECT_NOENTRY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PATH_TO_INODES_EXPECT_OBJECT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PATH_TO_INODES_SKIP_LAST: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PATH_TO_INODES_CHECK_LAST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mfs_type_convert(mut r#type: uint8_t) -> uint8_t {
    match r#type as ::core::ffi::c_int {
        DISP_TYPE_FILE => return TYPE_FILE as uint8_t,
        DISP_TYPE_DIRECTORY => return TYPE_DIRECTORY as uint8_t,
        DISP_TYPE_SYMLINK => return TYPE_SYMLINK as uint8_t,
        DISP_TYPE_FIFO => return TYPE_FIFO as uint8_t,
        DISP_TYPE_BLOCKDEV => return TYPE_BLOCKDEV as uint8_t,
        DISP_TYPE_CHARDEV => return TYPE_CHARDEV as uint8_t,
        DISP_TYPE_SOCKET => return TYPE_SOCKET as uint8_t,
        DISP_TYPE_TRASH => return TYPE_TRASH as uint8_t,
        DISP_TYPE_SUSTAINED => return TYPE_SUSTAINED as uint8_t,
        _ => {}
    }
    return 0 as uint8_t;
}
#[inline]
unsafe extern "C" fn fsnodes_type_convert(mut r#type: uint8_t) -> uint8_t {
    match r#type as ::core::ffi::c_int {
        DISP_TYPE_FILE => return TYPE_FILE as uint8_t,
        DISP_TYPE_DIRECTORY => return TYPE_DIRECTORY as uint8_t,
        DISP_TYPE_SYMLINK => return TYPE_SYMLINK as uint8_t,
        DISP_TYPE_FIFO => return TYPE_FIFO as uint8_t,
        DISP_TYPE_BLOCKDEV => return TYPE_BLOCKDEV as uint8_t,
        DISP_TYPE_CHARDEV => return TYPE_CHARDEV as uint8_t,
        DISP_TYPE_SOCKET => return TYPE_SOCKET as uint8_t,
        DISP_TYPE_TRASH => return TYPE_TRASH as uint8_t,
        DISP_TYPE_SUSTAINED => return TYPE_SUSTAINED as uint8_t,
        _ => {}
    }
    return 0 as uint8_t;
}
#[inline]
unsafe extern "C" fn mfs_attr_get_type(mut attr: *const uint8_t) -> uint8_t {
    unsafe {
        if (*attr.offset(0 as isize) as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            return (*attr.offset(1 as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                as uint8_t;
        } else {
            return fsnodes_type_convert(
                (*attr.offset(0 as isize) as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                    as uint8_t,
            );
        };
    }
}
unsafe extern "C" fn mfs_attr_to_mfsstat(
    mut inode: uint32_t,
    mut attr: *const uint8_t,
    mut stbuf: *mut mfs_int_statrec,
) {
    unsafe {
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        ptr = attr as *const uint8_t;
        (*stbuf).inode = inode;
        if (*attr.offset(0 as isize) as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            ptr = ptr.offset(1);
            (*stbuf).mode = get16bit(&raw mut ptr);
            (*stbuf).r#type =
                ((*stbuf).mode as ::core::ffi::c_int >> 12 as ::core::ffi::c_int) as uint8_t;
        } else {
            (*stbuf).r#type = get8bit(&raw mut ptr);
            (*stbuf).r#type = mfs_type_convert(
                ((*stbuf).r#type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as uint8_t,
            );
            (*stbuf).mode = get16bit(&raw mut ptr);
        }
        (*stbuf).mode =
            ((*stbuf).mode as ::core::ffi::c_int & 0xfff as ::core::ffi::c_int) as uint16_t;
        (*stbuf).uid = get32bit(&raw mut ptr);
        (*stbuf).gid = get32bit(&raw mut ptr);
        (*stbuf).atime = get32bit(&raw mut ptr);
        (*stbuf).mtime = get32bit(&raw mut ptr);
        (*stbuf).ctime = get32bit(&raw mut ptr);
        (*stbuf).nlink = get32bit(&raw mut ptr);
        if (*stbuf).r#type as ::core::ffi::c_int == TYPE_DIRECTORY
            || (*stbuf).r#type as ::core::ffi::c_int == TYPE_SYMLINK
            || (*stbuf).r#type as ::core::ffi::c_int == TYPE_FILE
        {
            (*stbuf).length = get64bit(&raw mut ptr);
        } else if (*stbuf).r#type as ::core::ffi::c_int == TYPE_BLOCKDEV
            || (*stbuf).r#type as ::core::ffi::c_int == TYPE_CHARDEV
        {
            (*stbuf).dev = get32bit(&raw mut ptr);
            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        } else {
            ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
        }
        (*stbuf).winattr = get8bit(&raw mut ptr);
    }
}
unsafe extern "C" fn mfs_attr_to_direntry(
    mut inode: uint32_t,
    mut attr: *const uint8_t,
    mut stbuf: *mut mfs_int_direntryplus,
) {
    unsafe {
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        ptr = attr as *const uint8_t;
        (*stbuf).inode = inode;
        if (*attr.offset(0 as isize) as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            ptr = ptr.offset(1);
            (*stbuf).mode = get16bit(&raw mut ptr);
            (*stbuf).r#type =
                ((*stbuf).mode as ::core::ffi::c_int >> 12 as ::core::ffi::c_int) as uint8_t;
        } else {
            (*stbuf).r#type = get8bit(&raw mut ptr);
            (*stbuf).r#type = mfs_type_convert(
                ((*stbuf).r#type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as uint8_t,
            );
            (*stbuf).mode = get16bit(&raw mut ptr);
        }
        (*stbuf).mode =
            ((*stbuf).mode as ::core::ffi::c_int & 0xfff as ::core::ffi::c_int) as uint16_t;
        (*stbuf).uid = get32bit(&raw mut ptr);
        (*stbuf).gid = get32bit(&raw mut ptr);
        (*stbuf).atime = get32bit(&raw mut ptr);
        (*stbuf).mtime = get32bit(&raw mut ptr);
        (*stbuf).ctime = get32bit(&raw mut ptr);
        (*stbuf).nlink = get32bit(&raw mut ptr);
        if (*stbuf).r#type as ::core::ffi::c_int == TYPE_DIRECTORY
            || (*stbuf).r#type as ::core::ffi::c_int == TYPE_SYMLINK
            || (*stbuf).r#type as ::core::ffi::c_int == TYPE_FILE
        {
            (*stbuf).length = get64bit(&raw mut ptr);
        } else if (*stbuf).r#type as ::core::ffi::c_int == TYPE_BLOCKDEV
            || (*stbuf).r#type as ::core::ffi::c_int == TYPE_CHARDEV
        {
            (*stbuf).dev = get32bit(&raw mut ptr);
            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        } else {
            ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
        }
        (*stbuf).winattr = get8bit(&raw mut ptr);
    }
}
unsafe extern "C" fn mfs_path_to_inodes(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut parent: *mut uint32_t,
    mut inode: *mut uint32_t,
    mut name: *mut uint8_t,
    mut nleng: *mut uint8_t,
    mut existflag: uint8_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut parent_inode: uint32_t = 0;
        let mut last_inode: uint32_t = 0;
        let mut status: uint8_t = 0;
        if !inode.is_null() {
            *inode = 0 as uint32_t;
        }
        memset(
            attr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ATTR_RECORD_SIZE as size_t,
        );
        status = lcache_path_lookup(
            MFS_ROOT_ID as uint32_t,
            strlen(path) as uint32_t,
            path as *const uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            &raw mut parent_inode,
            &raw mut last_inode,
            nleng,
            name,
            attr,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if !parent.is_null() {
            *parent = parent_inode;
        }
        if !inode.is_null() {
            *inode = last_inode;
        }
        if existflag as ::core::ffi::c_int == PATH_TO_INODES_EXPECT_NOENTRY
            && last_inode != 0 as uint32_t
        {
            return MFS_ERROR_EEXIST as uint8_t;
        }
        if existflag as ::core::ffi::c_int == PATH_TO_INODES_EXPECT_OBJECT
            && last_inode == 0 as uint32_t
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
unsafe extern "C" fn mfs_path_removed(mut path: *const ::core::ffi::c_char) {
    unsafe {
        lcache_path_invalidate(
            MFS_ROOT_ID as uint32_t,
            strlen(path) as uint32_t,
            path as *const uint8_t,
        );
    }
}
unsafe extern "C" fn mfs_path_created(mut path: *const ::core::ffi::c_char) {
    unsafe {
        lcache_path_invalidate(
            MFS_ROOT_ID as uint32_t,
            strlen(path) as uint32_t,
            path as *const uint8_t,
        );
    }
}
unsafe extern "C" fn mfs_inode_invalidate(mut inode: uint32_t) {
    unsafe {
        lcache_inode_invalidate(inode);
    }
}
unsafe extern "C" fn mfs_attr_to_type(mut attr: *const uint8_t) -> uint8_t {
    unsafe {
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        ptr = attr as *const uint8_t;
        if (*ptr.offset(0 as isize) as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            return (*attr.offset(1 as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                as uint8_t;
        } else {
            return mfs_type_convert(
                (*attr.offset(0 as isize) as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                    as uint8_t,
            );
        };
    }
}
unsafe extern "C" fn mfs_attr_to_size(mut attr: *const uint8_t) -> uint64_t {
    unsafe {
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        ptr = attr.offset(27 as ::core::ffi::c_int as isize) as *const uint8_t;
        return get64bit(&raw mut ptr);
    }
}
static mut fdtab: *mut file_info = ::core::ptr::null_mut::<file_info>();
static mut fdtabsize: uint32_t = 0;
static mut fdtabusemask: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
// Global fdtab lock: C fdtablock (pthread_mutex_t, PTHREAD_MUTEX_INITIALIZER).
// std::sync::Mutex is RAII-only, so emulate pthread-style manual lock/unlock
// by stashing the guard in a thread_local slot: fdtab_lock() parks the guard,
// fdtab_unlock() drops it. Poisoning is ignored (into_inner) for pthread
// parity. C pthread_mutex_init in mfs_int_init and pthread_mutex_destroy in
// mfs_int_term are gone: const init / no destroy needed.
// INVARIANT (audited vs mfsioint.c): fdtablock is never held while a
// fileinfo->lock is held - mfs_next_fd/mfs_free_fd/mfs_get_fi take and
// release fdtablock before callers touch fileinfo->lock.
static FDTAB_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
thread_local! {
    static FDTAB_LOCK_GUARD: std::cell::RefCell<Option<std::sync::MutexGuard<'static, ()>>> =
        const { std::cell::RefCell::new(None) };
}
fn fdtab_lock() {
    let guard = FDTAB_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    FDTAB_LOCK_GUARD.with(|slot| {
        let mut slot = slot.borrow_mut();
        assert!(slot.is_none(), "fdtab_lock: guard already held");
        *slot = Some(guard);
    });
}
fn fdtab_unlock() {
    let guard = FDTAB_LOCK_GUARD
        .with(|slot| slot.borrow_mut().take())
        .expect("fdtab_unlock: no guard held on this thread");
    drop(guard);
}
// Per-file lock: C fileinfo->lock (pthread_mutex_t) with fileinfo->rwcond
// waiting on it. Same guard-slot emulation as FDTAB_LOCK, but the slot also
// carries the mutex address so unlock/wait with a mismatched fileinfo panics
// (pthread would UB).
// MULTI-HOLD AUDIT (all 35 lock sites vs mfsioint.c): no thread ever holds
// two different file_info locks at once. Every public entry point resolves a
// single fildes via mfs_get_fi and locks only that fileinfo; mfs_int_close
// re-locks the SAME fileinfo inside mfs_int_fsync_common only after
// releasing it. Single slot suffices.
// FDTAB-GROWTH INTERLEAVING: mfs_resize_fd realloc-moves live file_info
// entries (memcpy) under fdtablock while other threads may hold
// fileinfo->lock or wait on rwcond (no lock ordering prevents it: holders
// never touch fdtablock). On Linux both pthread and std Mutex/Condvar are
// futex-based and survive a move only when unlocked with no parked waiters;
// a waiter parked in cond_wait during a move is broken in C identically
// (kernel futex queue keys on the old address), so this matches C behavior.
// SAFETY (guard lifetime): the guard borrows (*fileinfo).lock inside the
// realloc'd fdtab; transmuted to 'static. Sound because the guard is dropped
// only via fi_unlock/fi_cond_wait on the same thread, and an in-use fd's
// entry is never freed or reinitialized until mfs_int_close drops the use
// bit (and mfs_int_term closes every fd first).
thread_local! {
    static FI_LOCK_GUARD: std::cell::RefCell<
        Option<(*const std::sync::Mutex<()>, std::sync::MutexGuard<'static, ()>)>,
    > = const { std::cell::RefCell::new(None) };
}
unsafe fn fi_lock(fileinfo: *mut file_info) {
    unsafe {
        let guard = (*fileinfo).lock.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: see FI_LOCK_GUARD invariant above.
        let guard: std::sync::MutexGuard<'static, ()> = std::mem::transmute(guard);
        FI_LOCK_GUARD.with(|slot| {
            let mut slot = slot.borrow_mut();
            assert!(slot.is_none(), "fi_lock: guard already held");
            *slot = Some((&raw const (*fileinfo).lock, guard));
        });
    }
}
unsafe fn fi_unlock(fileinfo: *mut file_info) {
    unsafe {
        let entry = FI_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("fi_unlock: no guard held on this thread");
        assert!(
            entry.0 == &raw const (*fileinfo).lock,
            "fi_unlock: fileinfo pointer mismatch"
        );
        drop(entry.1);
    }
}
// Caller must hold fileinfo->lock; wait releases and reacquires it, exactly
// like pthread_cond_wait(&fileinfo->rwcond, &fileinfo->lock). C uses
// while-predicate loops at every site, so spurious wakeups are already
// handled there.
unsafe fn fi_cond_wait(fileinfo: *mut file_info) {
    unsafe {
        let entry = FI_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("fi_cond_wait: no guard held on this thread");
        assert!(
            entry.0 == &raw const (*fileinfo).lock,
            "fi_cond_wait: fileinfo pointer mismatch"
        );
        let guard = (*fileinfo)
            .rwcond
            .wait(entry.1)
            .unwrap_or_else(|e| e.into_inner());
        FI_LOCK_GUARD.with(|slot| {
            *slot.borrow_mut() = Some((entry.0, guard));
        });
    }
}
pub const FDTABSIZE_INIT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C" fn mfs_fi_init(mut fileinfo: *mut file_info) {
    unsafe {
        memset(
            fileinfo as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<file_info>(),
        );
        std::ptr::write(&raw mut (*fileinfo).flengptr, None);
        (*fileinfo).mode = MFS_IO_FORBIDDEN as ::core::ffi::c_int as uint8_t;
        std::ptr::write(&raw mut (*fileinfo).lock, std::sync::Mutex::new(()));
        std::ptr::write(&raw mut (*fileinfo).rwcond, std::sync::Condvar::new());
    }
}
unsafe extern "C" fn mfs_fi_term(mut fileinfo: *mut file_info) {
    unsafe {
        fi_lock(fileinfo);
        fi_unlock(fileinfo);
        // C pthread_mutex_destroy: std Mutex needs no destroy; futex state is freed with fdtab.
        // C pthread_cond_destroy: std Condvar needs no destroy.
    }
}
unsafe extern "C" fn mfs_resize_fd() {
    unsafe {
        let mut newfdtab: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut newfdtabusemask: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut newfdtabsize: uint32_t = 0;
        let mut i: uint32_t = 0;
        newfdtabsize = fdtabsize.wrapping_mul(2 as uint32_t);
        newfdtab = realloc(
            fdtab as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<file_info>().wrapping_mul(newfdtabsize as size_t),
        ) as *mut file_info;
        if newfdtab.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr() as *const ::core::ffi::c_char,
                423 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newfdtab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr() as *const ::core::ffi::c_char,
                423 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newfdtab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if newfdtab
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut file_info
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr() as *const ::core::ffi::c_char,
                423 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newfdtab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr() as *const ::core::ffi::c_char,
                423 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newfdtab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        newfdtabusemask = realloc(
            fdtabusemask as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<uint32_t>().wrapping_mul(
                newfdtabsize
                    .wrapping_add(31 as uint32_t)
                    .wrapping_div(32 as uint32_t) as size_t,
            ),
        ) as *mut uint32_t;
        if newfdtabusemask.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr() as *const ::core::ffi::c_char,
                425 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newfdtabusemask\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr() as *const ::core::ffi::c_char,
                425 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newfdtabusemask\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if newfdtabusemask
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint32_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr() as *const ::core::ffi::c_char,
                425 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newfdtabusemask\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr() as *const ::core::ffi::c_char,
                425 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newfdtabusemask\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        fdtab = newfdtab;
        fdtabusemask = newfdtabusemask;
        i = fdtabsize;
        while i < newfdtabsize {
            mfs_fi_init(fdtab.offset(i as isize));
            i = i.wrapping_add(1);
        }
        i = fdtabsize
            .wrapping_add(31 as uint32_t)
            .wrapping_div(32 as uint32_t);
        memset(
            fdtabusemask.offset(i as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<uint32_t>().wrapping_mul(
                newfdtabsize
                    .wrapping_add(31 as uint32_t)
                    .wrapping_div(32 as uint32_t)
                    .wrapping_sub(i) as size_t,
            ),
        );
        if fdtabsize & 0x1f as uint32_t != 0 as uint32_t {
            *fdtabusemask.offset(i.wrapping_sub(1 as uint32_t) as isize) = (*fdtabusemask
                .offset(i.wrapping_sub(1 as uint32_t) as isize)
                as ::core::ffi::c_uint
                & 0xffffffff as ::core::ffi::c_uint
                    >> (0x20 as uint32_t).wrapping_sub(fdtabsize & 0x1f as uint32_t))
                as uint32_t;
        }
        fdtabsize = newfdtabsize;
    }
}
unsafe extern "C" fn mfs_next_fd() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut m: uint32_t = 0;
        let mut fd: ::core::ffi::c_int = 0;
        fdtab_lock();
        i = 0 as uint32_t;
        while i < fdtabsize
            .wrapping_add(31 as uint32_t)
            .wrapping_div(32 as uint32_t)
        {
            if *fdtabusemask.offset(i as isize) != 0xffffffff as uint32_t {
                fd = i.wrapping_mul(32 as uint32_t) as ::core::ffi::c_int;
                m = *fdtabusemask.offset(i as isize);
                while m & 1 as uint32_t != 0 {
                    fd += 1;
                    m >>= 1 as ::core::ffi::c_int;
                }
                while (fd as uint32_t) >= fdtabsize {
                    mfs_resize_fd();
                }
                *fdtabusemask.offset((fd >> 5 as ::core::ffi::c_int) as isize) |=
                    ((1 as ::core::ffi::c_int) << (fd & 0x1f as ::core::ffi::c_int)) as uint32_t;
                fdtab_unlock();
                return fd;
            }
            i = i.wrapping_add(1);
        }
        fd = fdtabsize as ::core::ffi::c_int;
        mfs_resize_fd();
        *fdtabusemask.offset((fd >> 5 as ::core::ffi::c_int) as isize) |=
            ((1 as ::core::ffi::c_int) << (fd & 0x1f as ::core::ffi::c_int)) as uint32_t;
        fdtab_unlock();
        return fd;
    }
}
unsafe extern "C" fn mfs_free_fd(mut fd: ::core::ffi::c_int) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut m: uint32_t = 0;
        fdtab_lock();
        if fd >= 0 as ::core::ffi::c_int && (fd as uint32_t) < fdtabsize {
            i = (fd >> 5 as ::core::ffi::c_int) as uint32_t;
            m = ((1 as ::core::ffi::c_int) << (fd & 0x1f as ::core::ffi::c_int)) as uint32_t;
            *fdtabusemask.offset(i as isize) &= !m;
        }
        fdtab_unlock();
    }
}
unsafe extern "C" fn mfs_get_fi(mut fd: ::core::ffi::c_int) -> *mut file_info {
    unsafe {
        let mut i: uint32_t = 0;
        let mut m: uint32_t = 0;
        fdtab_lock();
        if fd >= 0 as ::core::ffi::c_int && (fd as uint32_t) < fdtabsize {
            i = (fd >> 5 as ::core::ffi::c_int) as uint32_t;
            m = ((1 as ::core::ffi::c_int) << (fd & 0x1f as ::core::ffi::c_int)) as uint32_t;
            if *fdtabusemask.offset(i as isize) & m != 0 {
                fdtab_unlock();
                return fdtab.offset(fd as isize);
            }
        }
        fdtab_unlock();
        return ::core::ptr::null_mut::<file_info>();
    }
}
unsafe extern "C" fn finfo_change_fleng(mut inode: uint32_t, mut fleng: uint64_t) {
    unsafe {
        plfsclient::inoleng::update_length(inode, fleng);
    }
}
static mut sugid_clear_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut mkdir_copy_sgid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_mknod(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut r#type: uint8_t,
    mut mode: uint16_t,
    mut dev: uint32_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            ::core::ptr::null_mut::<uint32_t>(),
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_SKIP_LAST as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if parent == 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = fs_mknod(
            parent,
            nleng,
            &raw mut name as *mut uint8_t as *const uint8_t,
            r#type,
            (mode as ::core::ffi::c_int & 0o7777 as ::core::ffi::c_int) as uint16_t,
            (*cr).umask,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            dev,
            &raw mut inode,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            mfs_path_created(path);
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_unlink(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if parent == 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = fs_unlink(
            parent,
            nleng,
            &raw mut name as *mut uint8_t as *const uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            &raw mut inode,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            mfs_path_removed(path);
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_mkdir(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut mode: uint16_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            ::core::ptr::null_mut::<uint32_t>(),
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_SKIP_LAST as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if parent == 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = fs_mkdir(
            parent,
            nleng,
            &raw mut name as *mut uint8_t as *const uint8_t,
            mode,
            (*cr).umask,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            mkdir_copy_sgid as uint8_t,
            &raw mut inode,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            mfs_path_created(path);
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_rmdir(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if parent == 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = fs_rmdir(
            parent,
            nleng,
            &raw mut name as *mut uint8_t as *const uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            &raw mut inode,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            mfs_path_removed(path);
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_rename(
    mut cr: *mut mfs_int_cred,
    mut src: *const ::core::ffi::c_char,
    mut dst: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut src_parent: uint32_t = 0;
        let mut src_name: [uint8_t; 256] = [0; 256];
        let mut src_nleng: uint8_t = 0;
        let mut dst_parent: uint32_t = 0;
        let mut dst_name: [uint8_t; 256] = [0; 256];
        let mut dst_nleng: uint8_t = 0;
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            src,
            &raw mut src_parent,
            ::core::ptr::null_mut::<uint32_t>(),
            &raw mut src_name as *mut uint8_t,
            &raw mut src_nleng,
            PATH_TO_INODES_SKIP_LAST as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if src_parent == 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = mfs_path_to_inodes(
            cr,
            dst,
            &raw mut dst_parent,
            ::core::ptr::null_mut::<uint32_t>(),
            &raw mut dst_name as *mut uint8_t,
            &raw mut dst_nleng,
            PATH_TO_INODES_SKIP_LAST as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if dst_parent == 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = fs_rename(
            src_parent,
            src_nleng,
            &raw mut src_name as *mut uint8_t as *const uint8_t,
            dst_parent,
            dst_nleng,
            &raw mut dst_name as *mut uint8_t as *const uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            0 as uint8_t,
            &raw mut inode,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            mfs_path_removed(src);
            mfs_path_created(dst);
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_link(
    mut cr: *mut mfs_int_cred,
    mut src: *const ::core::ffi::c_char,
    mut dst: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut src_inode: uint32_t = 0;
        let mut src_parent: uint32_t = 0;
        let mut src_name: [uint8_t; 256] = [0; 256];
        let mut src_nleng: uint8_t = 0;
        let mut dst_parent: uint32_t = 0;
        let mut dst_name: [uint8_t; 256] = [0; 256];
        let mut dst_nleng: uint8_t = 0;
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            src,
            &raw mut src_parent,
            &raw mut src_inode,
            &raw mut src_name as *mut uint8_t,
            &raw mut src_nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = mfs_path_to_inodes(
            cr,
            dst,
            &raw mut dst_parent,
            ::core::ptr::null_mut::<uint32_t>(),
            &raw mut dst_name as *mut uint8_t,
            &raw mut dst_nleng,
            PATH_TO_INODES_SKIP_LAST as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if dst_parent == 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = fs_link(
            src_inode,
            dst_parent,
            dst_nleng,
            &raw mut dst_name as *mut uint8_t as *const uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            &raw mut inode,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            mfs_path_created(dst);
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_symlink(
    mut cr: *mut mfs_int_cred,
    mut nodepath: *const ::core::ffi::c_char,
    mut linkpath: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            nodepath,
            &raw mut parent,
            ::core::ptr::null_mut::<uint32_t>(),
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_SKIP_LAST as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if parent == 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = fs_symlink(
            parent,
            nleng,
            &raw mut name as *mut uint8_t as *const uint8_t,
            linkpath as *const uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            &raw mut inode,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            mfs_path_created(nodepath);
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_readlink(
    mut cr: *mut mfs_int_cred,
    mut nodepath: *const ::core::ffi::c_char,
    mut linkpath: *mut ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        let mut cpath: *const uint8_t = ::core::ptr::null::<uint8_t>();
        status = mfs_path_to_inodes(
            cr,
            nodepath,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_readlink(inode, &raw mut cpath);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        strncpy(
            linkpath as *mut ::core::ffi::c_char,
            cpath as *const ::core::ffi::c_char,
            MFS_SYMLINK_MAX as size_t,
        );
        *linkpath.offset((MFS_SYMLINK_MAX - 1 as ::core::ffi::c_int) as isize) =
            '\0' as ::core::ffi::c_char;
        return MFS_STATUS_OK as uint8_t;
    }
}
unsafe extern "C" fn mfs_int_setattr(
    mut cr: *mut mfs_int_cred,
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut setmask: uint8_t,
    mut mode: uint16_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut atime: uint32_t,
    mut mtime: uint32_t,
    mut winattr: uint8_t,
) -> uint8_t {
    unsafe {
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = fs_setattr(
            inode,
            opened,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            setmask,
            (mode as ::core::ffi::c_int & 0o7777 as ::core::ffi::c_int) as uint16_t,
            uid,
            gid,
            atime,
            mtime,
            winattr,
            sugid_clear_mode as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            mfs_inode_invalidate(inode);
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_setwinattr(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut winattr: uint8_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        return mfs_int_setattr(
            cr,
            inode,
            0 as uint8_t,
            SET_WINATTR_FLAG as uint8_t,
            0 as uint16_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            winattr,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fsetwinattr(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut winattr: uint8_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        return mfs_int_setattr(
            cr,
            (*fileinfo).inode,
            1 as uint8_t,
            SET_WINATTR_FLAG as uint8_t,
            0 as uint16_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            winattr,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_chmod(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut mode: uint16_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        return mfs_int_setattr(
            cr,
            inode,
            0 as uint8_t,
            SET_MODE_FLAG as uint8_t,
            mode,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fchmod(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut mode: uint16_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        return mfs_int_setattr(
            cr,
            (*fileinfo).inode,
            1 as uint8_t,
            SET_MODE_FLAG as uint8_t,
            mode,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_chown(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut owner: uint32_t,
    mut group: uint32_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut setmask: uint8_t = 0;
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        setmask = 0 as uint8_t;
        if owner != 0xffffffff as uint32_t {
            setmask = (setmask as ::core::ffi::c_int | SET_UID_FLAG) as uint8_t;
        }
        if group != 0xffffffff as uint32_t {
            setmask = (setmask as ::core::ffi::c_int | SET_GID_FLAG) as uint8_t;
        }
        return mfs_int_setattr(
            cr,
            inode,
            0 as uint8_t,
            setmask,
            0 as uint16_t,
            owner,
            group,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fchown(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut owner: uint32_t,
    mut group: uint32_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut setmask: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        setmask = 0 as uint8_t;
        if owner != 0xffffffff as uint32_t {
            setmask = (setmask as ::core::ffi::c_int | SET_UID_FLAG) as uint8_t;
        }
        if group != 0xffffffff as uint32_t {
            setmask = (setmask as ::core::ffi::c_int | SET_GID_FLAG) as uint8_t;
        }
        return mfs_int_setattr(
            cr,
            (*fileinfo).inode,
            1 as uint8_t,
            setmask,
            0 as uint16_t,
            owner,
            group,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_utimes(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut flags: uint8_t,
    mut atime: uint32_t,
    mut mtime: uint32_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut setmask: uint8_t = 0;
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        setmask = 0 as uint8_t;
        if flags as ::core::ffi::c_int & MFS_TIMES_ATIME_NOW != 0 {
            setmask = (setmask as ::core::ffi::c_int | SET_ATIME_NOW_FLAG) as uint8_t;
        } else if flags as ::core::ffi::c_int & MFS_TIMES_ATIME_OMIT == 0 as ::core::ffi::c_int {
            setmask = (setmask as ::core::ffi::c_int | SET_ATIME_FLAG) as uint8_t;
        }
        if flags as ::core::ffi::c_int & MFS_TIMES_MTIME_NOW != 0 {
            setmask = (setmask as ::core::ffi::c_int | SET_MTIME_NOW_FLAG) as uint8_t;
        } else if flags as ::core::ffi::c_int & MFS_TIMES_MTIME_OMIT == 0 as ::core::ffi::c_int {
            setmask = (setmask as ::core::ffi::c_int | SET_MTIME_FLAG) as uint8_t;
        }
        return mfs_int_setattr(
            cr,
            inode,
            0 as uint8_t,
            setmask,
            0 as uint16_t,
            0 as uint32_t,
            0 as uint32_t,
            atime,
            mtime,
            0 as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_futimes(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut flags: uint8_t,
    mut atime: uint32_t,
    mut mtime: uint32_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut setmask: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        setmask = 0 as uint8_t;
        if flags as ::core::ffi::c_int & MFS_TIMES_ATIME_NOW != 0 {
            setmask = (setmask as ::core::ffi::c_int | SET_ATIME_NOW_FLAG) as uint8_t;
        } else if flags as ::core::ffi::c_int & MFS_TIMES_ATIME_OMIT == 0 as ::core::ffi::c_int {
            setmask = (setmask as ::core::ffi::c_int | SET_ATIME_FLAG) as uint8_t;
        }
        if flags as ::core::ffi::c_int & MFS_TIMES_MTIME_NOW != 0 {
            setmask = (setmask as ::core::ffi::c_int | SET_MTIME_NOW_FLAG) as uint8_t;
        } else if flags as ::core::ffi::c_int & MFS_TIMES_MTIME_OMIT == 0 as ::core::ffi::c_int {
            setmask = (setmask as ::core::ffi::c_int | SET_MTIME_FLAG) as uint8_t;
        }
        return mfs_int_setattr(
            cr,
            (*fileinfo).inode,
            1 as uint8_t,
            setmask,
            0 as uint16_t,
            0 as uint32_t,
            0 as uint32_t,
            atime,
            mtime,
            0 as uint8_t,
        );
    }
}
unsafe extern "C" fn mfs_int_truncate_common(
    mut cr: *mut mfs_int_cred,
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut size: int64_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut status: uint8_t = 0;
        if size < 0 as int64_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if size >= MAX_FILE_SIZE {
            return MFS_ERROR_EFBIG as uint8_t;
        }
        write_data_flush_inode(inode);
        status = do_truncate(
            inode,
            (if opened as ::core::ffi::c_int != 0 {
                TRUNCATE_FLAG_OPENED
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            size as uint64_t,
            attr,
            ::core::ptr::null_mut::<uint64_t>(),
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        plfsclient::chunksdatacache::clear_inode(
            inode,
            (size / MFSCHUNKSIZE as int64_t) as uint32_t,
        );
        finfo_change_fleng(inode, size as uint64_t);
        write_data_inode_setmaxfleng(inode, size as uint64_t);
        read_inode_set_length_active(inode, size as uint64_t);
        mfs_inode_invalidate(inode);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_truncate(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut size: int64_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return -1 as ::core::ffi::c_int as uint8_t;
        }
        return mfs_int_truncate_common(
            cr,
            inode,
            0 as uint8_t,
            size,
            &raw mut attr as *mut uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_ftruncate(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut size: int64_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut attr: [uint8_t; 36] = [0; 36];
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_READONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_ATTRONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_DIRECTORY as ::core::ffi::c_int
        {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        return mfs_int_truncate_common(
            cr,
            (*fileinfo).inode,
            1 as uint8_t,
            size,
            &raw mut attr as *mut uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_lseek(
    mut fildes: ::core::ffi::c_int,
    mut offset: *mut int64_t,
    mut whence: uint8_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut noffset: int64_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_DIRECTORY as ::core::ffi::c_int
        {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        match whence as ::core::ffi::c_int {
            MFS_SEEK_SET => {
                noffset = *offset;
            }
            MFS_SEEK_CUR => {
                noffset = (*fileinfo).offset as int64_t + *offset;
            }
            MFS_SEEK_END => {
                noffset = plfsclient::inoleng::get_length((*fileinfo).flengptr.as_ref().unwrap())
                    as int64_t
                    + *offset;
            }
            _ => {
                fi_unlock(fileinfo);
                return MFS_ERROR_EINVAL as uint8_t;
            }
        }
        if noffset < 0 as int64_t {
            noffset = 0 as int64_t;
        }
        (*fileinfo).offset = noffset as uint64_t;
        *offset = (*fileinfo).offset as int64_t;
        fi_unlock(fileinfo);
        return MFS_STATUS_OK as uint8_t;
    }
}
unsafe extern "C" fn mfs_fix_attr(
    mut r#type: uint8_t,
    mut inode: uint32_t,
    mut buf: *mut mfs_int_statrec,
) {
    unsafe {
        if r#type as ::core::ffi::c_int == TYPE_FILE {
            let mut maxfleng: uint64_t = write_data_inode_getmaxfleng(inode);
            if maxfleng > (*buf).length {
                (*buf).length = maxfleng;
            }
            read_inode_set_length_passive(inode, (*buf).length);
            finfo_change_fleng(inode, (*buf).length);
        }
        fs_fix_amtime(inode, &raw mut (*buf).atime, &raw mut (*buf).mtime);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_stat(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut buf: *mut mfs_int_statrec,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut r#type: uint8_t = 0;
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        memset(
            buf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mfs_int_statrec>(),
        );
        mfs_attr_to_mfsstat(inode, &raw mut attr as *mut uint8_t as *const uint8_t, buf);
        r#type = mfs_attr_get_type(&raw mut attr as *mut uint8_t as *const uint8_t);
        mfs_fix_attr(r#type, inode, buf);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fstat(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut buf: *mut mfs_int_statrec,
) -> uint8_t {
    unsafe {
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut status: uint8_t = 0;
        let mut r#type: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        status = fs_getattr(
            (*fileinfo).inode,
            1 as uint8_t,
            (*cr).uid,
            (*cr).gidtab[0 as usize],
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        memset(
            buf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mfs_int_statrec>(),
        );
        mfs_attr_to_mfsstat(
            (*fileinfo).inode,
            &raw mut attr as *mut uint8_t as *const uint8_t,
            buf,
        );
        r#type = mfs_attr_get_type(&raw mut attr as *mut uint8_t as *const uint8_t);
        mfs_fix_attr(r#type, (*fileinfo).inode, buf);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_check_attrname(
    mut nleng: *mut uint32_t,
    mut name: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        *nleng = strlen(name) as uint32_t;
        if *nleng > 255 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if *nleng < 6 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if memcmp(
            name as *const ::core::ffi::c_void,
            b"user.\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            5 as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_getxattr(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut vbuff: *mut *const uint8_t,
    mut vleng: *mut uint32_t,
    mut mode: uint8_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut fname: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut xattrnameleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut fname as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = mfs_int_check_attrname(&raw mut xattrnameleng, name);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_getxattr(
            inode,
            0 as uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            xattrnameleng as uint8_t,
            name as *const uint8_t,
            mode,
            vbuff,
            vleng,
        );
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fgetxattr(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut vbuff: *mut *const uint8_t,
    mut vleng: *mut uint32_t,
    mut mode: uint8_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut xattrnameleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        status = mfs_int_check_attrname(&raw mut xattrnameleng, name);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_getxattr(
            (*fileinfo).inode,
            1 as uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            xattrnameleng as uint8_t,
            name as *const uint8_t,
            mode,
            vbuff,
            vleng,
        );
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_setxattr(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut value: *const uint8_t,
    mut vsize: uint32_t,
    mut mode: uint8_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut fname: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut xattrnameleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut fname as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = mfs_int_check_attrname(&raw mut xattrnameleng, name);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_setxattr(
            inode,
            0 as uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            xattrnameleng as uint8_t,
            name as *const uint8_t,
            vsize,
            value,
            mode,
        );
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fsetxattr(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut value: *const uint8_t,
    mut vsize: uint32_t,
    mut mode: uint8_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut xattrnameleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        status = mfs_int_check_attrname(&raw mut xattrnameleng, name);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_setxattr(
            (*fileinfo).inode,
            1 as uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            xattrnameleng as uint8_t,
            name as *const uint8_t,
            vsize,
            value,
            mode,
        );
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_removexattr(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut fname: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut xattrnameleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut fname as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = mfs_int_check_attrname(&raw mut xattrnameleng, name);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_removexattr(
            inode,
            0 as uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            xattrnameleng as uint8_t,
            name as *const uint8_t,
        );
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fremovexattr(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut xattrnameleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        status = mfs_int_check_attrname(&raw mut xattrnameleng, name);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_removexattr(
            (*fileinfo).inode,
            1 as uint8_t,
            (*cr).uid,
            (*cr).gidcnt,
            &raw mut (*cr).gidtab as *mut uint32_t,
            xattrnameleng as uint8_t,
            name as *const uint8_t,
        );
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_listxattr(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut rsize: *mut int32_t,
    mut list: *mut ::core::ffi::c_char,
    mut size: uint32_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut fname: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut vbuff: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut vleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut fname as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if size == 0 as uint32_t {
            status = fs_listxattr(
                inode,
                0 as uint8_t,
                (*cr).uid,
                (*cr).gidcnt,
                &raw mut (*cr).gidtab as *mut uint32_t,
                MFS_XATTR_LENGTH_ONLY as uint8_t,
                &raw mut vbuff,
                &raw mut vleng,
            );
        } else {
            status = fs_listxattr(
                inode,
                0 as uint8_t,
                (*cr).uid,
                (*cr).gidcnt,
                &raw mut (*cr).gidtab as *mut uint32_t,
                MFS_XATTR_GETA_DATA as uint8_t,
                &raw mut vbuff,
                &raw mut vleng,
            );
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        *rsize = vleng as int32_t;
        if size > 0 as uint32_t {
            if vleng <= size {
                memcpy(
                    list as *mut ::core::ffi::c_void,
                    vbuff as *const ::core::ffi::c_void,
                    vleng as size_t,
                );
                *rsize = vleng as int32_t;
            } else {
                return MFS_ERROR_ERANGE as uint8_t;
            }
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_flistxattr(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut rsize: *mut int32_t,
    mut list: *mut ::core::ffi::c_char,
    mut size: uint32_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut vbuff: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut vleng: uint32_t = 0;
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        if size == 0 as uint32_t {
            status = fs_listxattr(
                (*fileinfo).inode,
                1 as uint8_t,
                (*cr).uid,
                (*cr).gidcnt,
                &raw mut (*cr).gidtab as *mut uint32_t,
                MFS_XATTR_LENGTH_ONLY as uint8_t,
                &raw mut vbuff,
                &raw mut vleng,
            );
        } else {
            status = fs_listxattr(
                (*fileinfo).inode,
                1 as uint8_t,
                (*cr).uid,
                (*cr).gidcnt,
                &raw mut (*cr).gidtab as *mut uint32_t,
                MFS_XATTR_GETA_DATA as uint8_t,
                &raw mut vbuff,
                &raw mut vleng,
            );
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        *rsize = vleng as int32_t;
        if size > 0 as uint32_t {
            if vleng <= size {
                memcpy(
                    list as *mut ::core::ffi::c_void,
                    vbuff as *const ::core::ffi::c_void,
                    vleng as size_t,
                );
                *rsize = vleng as int32_t;
            } else {
                return MFS_ERROR_ERANGE as uint8_t;
            }
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_getfacl(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut acltype: uint8_t,
    mut userperm: *mut uint16_t,
    mut groupperm: *mut uint16_t,
    mut otherperm: *mut uint16_t,
    mut maskperm: *mut uint16_t,
    mut namedusers: *mut uint16_t,
    mut namedgroups: *mut uint16_t,
    mut namedacls: *mut *const uint8_t,
    mut namedaclssize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut fname: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut fname as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_getfacl(
            inode,
            acltype,
            userperm,
            groupperm,
            otherperm,
            maskperm,
            namedusers,
            namedgroups,
            namedacls,
            namedaclssize,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fgetfacl(
    _cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut acltype: uint8_t,
    mut userperm: *mut uint16_t,
    mut groupperm: *mut uint16_t,
    mut otherperm: *mut uint16_t,
    mut maskperm: *mut uint16_t,
    mut namedusers: *mut uint16_t,
    mut namedgroups: *mut uint16_t,
    mut namedacls: *mut *const uint8_t,
    mut namedaclssize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        status = fs_getfacl(
            (*fileinfo).inode,
            acltype,
            userperm,
            groupperm,
            otherperm,
            maskperm,
            namedusers,
            namedgroups,
            namedacls,
            namedaclssize,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_setfacl(
    mut cr: *mut mfs_int_cred,
    mut path: *const ::core::ffi::c_char,
    mut acltype: uint8_t,
    mut userperm: uint16_t,
    mut groupperm: uint16_t,
    mut otherperm: uint16_t,
    mut maskperm: uint16_t,
    mut namedusers: uint16_t,
    mut namedgroups: uint16_t,
    mut namedacls: *mut uint8_t,
    mut namedaclssize: uint32_t,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut fname: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut fname as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        status = fs_setfacl(
            inode,
            (*cr).uid,
            acltype,
            userperm,
            groupperm,
            otherperm,
            maskperm,
            namedusers,
            namedgroups,
            namedacls,
            namedaclssize,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fsetfacl(
    mut cr: *mut mfs_int_cred,
    mut fildes: ::core::ffi::c_int,
    mut acltype: uint8_t,
    mut userperm: uint16_t,
    mut groupperm: uint16_t,
    mut otherperm: uint16_t,
    mut maskperm: uint16_t,
    mut namedusers: uint16_t,
    mut namedgroups: uint16_t,
    mut namedacls: *mut uint8_t,
    mut namedaclssize: uint32_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        fi_unlock(fileinfo);
        status = fs_setfacl(
            (*fileinfo).inode,
            (*cr).uid,
            acltype,
            userperm,
            groupperm,
            otherperm,
            maskperm,
            namedusers,
            namedgroups,
            namedacls,
            namedaclssize,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_statfs(mut buf: *mut mfs_int_statfsrec) -> uint8_t {
    unsafe {
        memset(
            buf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mfs_int_statfsrec>(),
        );
        fs_statfs(
            &raw mut (*buf).totalspace,
            &raw mut (*buf).availspace,
            &raw mut (*buf).freespace,
            &raw mut (*buf).trashspace,
            &raw mut (*buf).sustainedspace,
            &raw mut (*buf).inodes,
        );
        fs_getmasterparams(
            &raw mut (*buf).masterip,
            &raw mut (*buf).masterport,
            &raw mut (*buf).sessionid,
            &raw mut (*buf).masterversion,
            &raw mut (*buf).masterprocessid,
        );
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_open(
    mut cr: *mut mfs_int_cred,
    mut fildes: *mut ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
    mut oflag: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> uint8_t {
    unsafe {
        let mut fsize: uint64_t = 0;
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut noatomictrunc: uint8_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        let mut mfsoflag: uint8_t = 0;
        let mut oflags: uint8_t = 0;
        let mut needopen: ::core::ffi::c_int = 0;
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        mfsoflag = 0 as uint8_t;
        match oflag & MFS_O_ACCMODE {
            MFS_O_RDONLY => {
                mfsoflag = (mfsoflag as ::core::ffi::c_int | OPEN_READ) as uint8_t;
            }
            MFS_O_WRONLY => {
                mfsoflag = (mfsoflag as ::core::ffi::c_int | OPEN_WRITE) as uint8_t;
            }
            MFS_O_RDWR => {
                mfsoflag = (mfsoflag as ::core::ffi::c_int | (OPEN_READ | OPEN_WRITE)) as uint8_t;
            }
            MFS_O_ATTRONLY => {}
            _ => return MFS_ERROR_EINVAL as uint8_t,
        }
        if oflag & MFS_O_TRUNC != 0 {
            let mut mver: uint32_t = 0;
            mver = master_version();
            noatomictrunc = (if mver
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
                && mver
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                || mver
                    < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            113 as ::core::ffi::c_int
                        })) as uint32_t
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            mfsoflag = (mfsoflag as ::core::ffi::c_int | OPEN_TRUNCATE) as uint8_t;
        } else {
            noatomictrunc = 0 as uint8_t;
        }
        oflags = 0 as uint8_t;
        needopen = 1 as ::core::ffi::c_int;
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_CHECK_LAST as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if oflag & MFS_O_CREAT != 0 {
            if oflag & MFS_O_EXCL != 0 {
                if inode != 0 as uint32_t {
                    return MFS_ERROR_EEXIST as uint8_t;
                }
            } else if inode == 0 as uint32_t {
                status = fs_create(
                    parent,
                    nleng,
                    &raw mut name as *mut uint8_t as *const uint8_t,
                    mode as uint16_t,
                    (*cr).umask,
                    (*cr).uid,
                    (*cr).gidcnt,
                    &raw mut (*cr).gidtab as *mut uint32_t,
                    &raw mut inode,
                    &raw mut attr as *mut uint8_t,
                    &raw mut oflags,
                );
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    return status;
                }
                mfs_path_created(path);
                needopen = 0 as ::core::ffi::c_int;
            }
        } else if inode == 0 as uint32_t {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if needopen != 0 {
            if mfs_attr_to_type(&raw mut attr as *mut uint8_t as *const uint8_t)
                as ::core::ffi::c_int
                != TYPE_FILE
            {
                return MFS_ERROR_EISDIR as uint8_t;
            }
            status = fs_opencheck(
                inode,
                (*cr).uid,
                (*cr).gidcnt,
                &raw mut (*cr).gidtab as *mut uint32_t,
                mfsoflag,
                &raw mut attr as *mut uint8_t,
                &raw mut oflags,
            );
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                return status;
            }
            if mfsoflag as ::core::ffi::c_int & OPEN_TRUNCATE != 0
                && noatomictrunc as ::core::ffi::c_int != 0
            {
                status = mfs_int_truncate_common(
                    cr,
                    inode,
                    1 as uint8_t,
                    0 as int64_t,
                    &raw mut attr as *mut uint8_t,
                );
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    return status;
                }
            }
        }
        if oflags as ::core::ffi::c_int & OPEN_APPENDONLY != 0 {
            if oflag & MFS_O_APPEND == 0 as ::core::ffi::c_int {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        fs_inc_acnt(inode);
        fsize = mfs_attr_to_size(&raw mut attr as *mut uint8_t as *const uint8_t);
        *fildes = mfs_next_fd();
        fileinfo = mfs_get_fi(*fildes);
        fi_lock(fileinfo);
        (*fileinfo).flengptr = Some(plfsclient::inoleng::acquire(inode));
        (*fileinfo).inode = inode;
        (*fileinfo).mode = MFS_IO_FORBIDDEN as ::core::ffi::c_int as uint8_t;
        (*fileinfo).offset = 0 as uint64_t;
        (*fileinfo).rdata = NULL;
        (*fileinfo).wdata = NULL;
        (*fileinfo).readers_cnt = 0 as uint32_t;
        (*fileinfo).writers_cnt = 0 as uint32_t;
        (*fileinfo).writing = 0 as uint8_t;
        (*fileinfo).privuser = (if (*cr).uid == 0 as uint32_t {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        (*fileinfo).reading = 0 as uint8_t;
        (*fileinfo).wasread = 0 as uint8_t;
        (*fileinfo).dataformat = 0 as uint8_t;
        (*fileinfo).dbuff = ::core::ptr::null_mut::<uint8_t>();
        (*fileinfo).dbuffsize = 0 as uint64_t;
        plfsclient::inoleng::set_length((*fileinfo).flengptr.as_ref().unwrap(), fsize);
        if oflag & MFS_O_ACCMODE == MFS_O_RDONLY {
            (*fileinfo).mode = MFS_IO_READONLY as ::core::ffi::c_int as uint8_t;
            (*fileinfo).rdata = read_data_new(inode, fsize);
        } else if oflag & MFS_O_ACCMODE == MFS_O_WRONLY {
            if oflag & MFS_O_APPEND != 0 {
                (*fileinfo).mode = MFS_IO_APPENDONLY as ::core::ffi::c_int as uint8_t;
            } else {
                (*fileinfo).mode = MFS_IO_WRITEONLY as ::core::ffi::c_int as uint8_t;
            }
            (*fileinfo).wdata = write_data_new(inode, fsize);
        } else if oflag & MFS_O_ACCMODE == MFS_O_RDWR {
            if oflag & MFS_O_APPEND != 0 {
                (*fileinfo).mode = MFS_IO_READAPPEND as ::core::ffi::c_int as uint8_t;
            } else {
                (*fileinfo).mode = MFS_IO_READWRITE as ::core::ffi::c_int as uint8_t;
            }
            (*fileinfo).rdata = read_data_new(inode, fsize);
            (*fileinfo).wdata = write_data_new(inode, fsize);
        } else {
            (*fileinfo).mode = MFS_O_ATTRONLY as uint8_t;
        }
        if oflag & MFS_O_APPEND != 0 {
            (*fileinfo).offset = fsize;
        }
        fi_unlock(fileinfo);
        return MFS_STATUS_OK as uint8_t;
    }
}
unsafe extern "C" fn mfs_int_error_conv(mut err: ::core::ffi::c_int) -> uint8_t {
    match err {
        0 => return MFS_STATUS_OK as uint8_t,
        EBADF => return MFS_ERROR_EBADF as uint8_t,
        EINVAL => return MFS_ERROR_EINVAL as uint8_t,
        EDQUOT => return MFS_ERROR_QUOTA as uint8_t,
        ENOSPC => return MFS_ERROR_NOSPACE as uint8_t,
        EFBIG => return MFS_ERROR_EFBIG as uint8_t,
        ENXIO => return MFS_ERROR_CHUNKLOST as uint8_t,
        EIO => return MFS_ERROR_IO as uint8_t,
        _ => {}
    }
    return MFS_ERROR_IO as uint8_t;
}
unsafe extern "C" fn mfs_int_pread_common(
    mut fileinfo: *mut file_info,
    mut rsize: *mut int32_t,
    mut buf: *mut uint8_t,
    mut nbyte: uint32_t,
    mut offset: uint64_t,
) -> uint8_t {
    unsafe {
        let mut ssize: uint32_t = 0;
        let mut iov: *mut iovec = ::core::ptr::null_mut::<iovec>();
        let mut iovcnt: uint32_t = 0;
        let mut pos: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut buffptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut status: uint8_t = 0;
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        if offset >= MAX_FILE_SIZE as uint64_t
            || offset.wrapping_add(nbyte as uint64_t) >= MAX_FILE_SIZE as uint64_t
        {
            return MFS_ERROR_EFBIG as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_WRITEONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_APPENDONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_ATTRONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_DIRECTORY as ::core::ffi::c_int
        {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        while (*fileinfo).writing as uint32_t | (*fileinfo).writers_cnt != 0 {
            fi_cond_wait(fileinfo);
        }
        (*fileinfo).readers_cnt = (*fileinfo).readers_cnt.wrapping_add(1);
        fi_unlock(fileinfo);
        write_data_flush_inode((*fileinfo).inode);
        ssize = nbyte;
        fs_atime((*fileinfo).inode);
        status = mfs_int_error_conv(read_data(
            (*fileinfo).rdata,
            offset,
            &raw mut ssize,
            &raw mut buffptr,
            &raw mut iov,
            &raw mut iovcnt,
        ));
        fs_atime((*fileinfo).inode);
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            pos = 0 as uint32_t;
            i = 0 as uint32_t;
            while i < iovcnt {
                memcpy(
                    buf.offset(pos as isize) as *mut ::core::ffi::c_void,
                    (*iov.offset(i as isize)).iov_base,
                    (*iov.offset(i as isize)).iov_len,
                );
                pos = (pos as size_t).wrapping_add((*iov.offset(i as isize)).iov_len) as uint32_t;
                i = i.wrapping_add(1);
            }
        }
        read_data_free_buff((*fileinfo).rdata, buffptr, iov, iovcnt);
        fi_lock(fileinfo);
        (*fileinfo).readers_cnt = (*fileinfo).readers_cnt.wrapping_sub(1);
        if (*fileinfo).readers_cnt == 0 as uint32_t {
            (*fileinfo).rwcond.notify_all();
        }
        fi_unlock(fileinfo);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            fs_read_notify(0 as uint64_t);
            return status;
        }
        fs_read_notify(ssize as uint64_t);
        *rsize = ssize as int32_t;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_pread(
    mut fildes: ::core::ffi::c_int,
    mut rsize: *mut int64_t,
    mut buf: *mut uint8_t,
    mut nbyte: uint64_t,
    mut offset: uint64_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut rsize_part: int32_t = 0;
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        *rsize = 0 as int64_t;
        while nbyte > 0x1000000 as uint64_t {
            status = mfs_int_pread_common(
                fileinfo,
                &raw mut rsize_part,
                buf,
                0x1000000 as uint32_t,
                offset,
            );
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                return status;
            }
            offset = offset.wrapping_add(rsize_part as uint64_t);
            buf = buf.offset(rsize_part as isize);
            nbyte = nbyte.wrapping_sub(rsize_part as uint64_t);
            *rsize += rsize_part as int64_t;
            if rsize_part < 0x1000000 as int32_t {
                return MFS_STATUS_OK as uint8_t;
            }
        }
        status = mfs_int_pread_common(
            fileinfo,
            &raw mut rsize_part,
            buf,
            nbyte as uint32_t,
            offset,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        *rsize += rsize_part as int64_t;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_read(
    mut fildes: ::core::ffi::c_int,
    mut rsize: *mut int64_t,
    mut buf: *mut uint8_t,
    mut nbyte: uint64_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut rsize_part: int32_t = 0;
        let mut offset: uint64_t = 0;
        let mut status: uint8_t = 0;
        *rsize = 0 as int64_t;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        offset = (*fileinfo).offset;
        fi_unlock(fileinfo);
        while nbyte > 0x1000000 as uint64_t {
            status = mfs_int_pread_common(
                fileinfo,
                &raw mut rsize_part,
                buf,
                0x1000000 as uint32_t,
                offset,
            );
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                return status;
            }
            offset = offset.wrapping_add(rsize_part as uint64_t);
            buf = buf.offset(rsize_part as isize);
            nbyte = nbyte.wrapping_sub(rsize_part as uint64_t);
            *rsize += rsize_part as int64_t;
            if rsize_part < 0x1000000 as int32_t {
                fi_lock(fileinfo);
                (*fileinfo).offset = offset;
                fi_unlock(fileinfo);
                return MFS_STATUS_OK as uint8_t;
            }
        }
        status = mfs_int_pread_common(
            fileinfo,
            &raw mut rsize_part,
            buf,
            nbyte as uint32_t,
            offset,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        offset = offset.wrapping_add(rsize_part as uint64_t);
        *rsize += rsize_part as int64_t;
        fi_lock(fileinfo);
        (*fileinfo).offset = offset;
        fi_unlock(fileinfo);
        return MFS_STATUS_OK as uint8_t;
    }
}
unsafe extern "C" fn mfs_int_pwrite_common(
    mut fileinfo: *mut file_info,
    mut rsize: *mut int32_t,
    mut buf: *const uint8_t,
    mut nbyte: uint32_t,
    mut offset: uint64_t,
) -> uint8_t {
    unsafe {
        let mut newfleng: uint64_t = 0;
        let mut appendonly: uint8_t = 0;
        let mut status: uint8_t = 0;
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        if offset >= MAX_FILE_SIZE as uint64_t
            || offset.wrapping_add(nbyte as uint64_t) >= MAX_FILE_SIZE as uint64_t
        {
            return MFS_ERROR_EFBIG as uint8_t;
        }
        fi_lock(fileinfo);
        appendonly = (if (*fileinfo).mode as ::core::ffi::c_int
            == MFS_IO_APPENDONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_READAPPEND as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_READONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_ATTRONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_DIRECTORY as ::core::ffi::c_int
        {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        (*fileinfo).writers_cnt = (*fileinfo).writers_cnt.wrapping_add(1);
        while (*fileinfo).readers_cnt | (*fileinfo).writing as uint32_t != 0 {
            fi_cond_wait(fileinfo);
        }
        (*fileinfo).writers_cnt = (*fileinfo).writers_cnt.wrapping_sub(1);
        (*fileinfo).writing = 1 as uint8_t;
        status = MFS_STATUS_OK as uint8_t;
        if appendonly != 0 {
            if master_version()
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        113 as ::core::ffi::c_int
                    })) as uint32_t
            {
                let mut prevleng: uint64_t = 0;
                let mut gid: uint32_t = 0 as uint32_t;
                let mut inode: uint32_t = (*fileinfo).inode;
                fi_unlock(fileinfo);
                status = do_truncate(
                    inode,
                    (TRUNCATE_FLAG_OPENED | TRUNCATE_FLAG_UPDATE | TRUNCATE_FLAG_RESERVE)
                        as uint8_t,
                    0 as uint32_t,
                    1 as uint32_t,
                    &raw mut gid,
                    nbyte as uint64_t,
                    ::core::ptr::null_mut::<uint8_t>(),
                    &raw mut prevleng,
                );
                fi_lock(fileinfo);
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    offset = prevleng;
                }
            } else {
                offset = plfsclient::inoleng::get_length((*fileinfo).flengptr.as_ref().unwrap());
                if offset.wrapping_add(nbyte as uint64_t) >= MAX_FILE_SIZE as uint64_t {
                    status = MFS_ERROR_EFBIG as uint8_t;
                }
            }
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            fi_unlock(fileinfo);
            fs_mtime((*fileinfo).inode);
            status = mfs_int_error_conv(write_data(
                (*fileinfo).wdata,
                offset,
                nbyte,
                buf,
                (*fileinfo).privuser,
            ));
            fs_mtime((*fileinfo).inode);
            fi_lock(fileinfo);
        }
        (*fileinfo).writing = 0 as uint8_t;
        (*fileinfo).rwcond.notify_all();
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            fi_unlock(fileinfo);
            fs_write_notify(0 as uint64_t);
            return status;
        }
        if offset.wrapping_add(nbyte as uint64_t)
            > plfsclient::inoleng::get_length((*fileinfo).flengptr.as_ref().unwrap())
        {
            plfsclient::inoleng::set_length(
                (*fileinfo).flengptr.as_ref().unwrap(),
                offset.wrapping_add(nbyte as uint64_t),
            );
            newfleng = offset.wrapping_add(nbyte as uint64_t);
        } else {
            newfleng = 0 as uint64_t;
        }
        fi_unlock(fileinfo);
        if newfleng > 0 as uint64_t {
            read_inode_set_length_passive((*fileinfo).inode, newfleng);
            write_data_inode_setmaxfleng((*fileinfo).inode, newfleng);
            finfo_change_fleng((*fileinfo).inode, newfleng);
        }
        read_inode_clear_cache((*fileinfo).inode, offset, nbyte as uint64_t);
        fs_write_notify(nbyte as uint64_t);
        mfs_inode_invalidate((*fileinfo).inode);
        *rsize = nbyte as int32_t;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_pwrite(
    mut fildes: ::core::ffi::c_int,
    mut rsize: *mut int64_t,
    mut buf: *const uint8_t,
    mut nbyte: uint64_t,
    mut offset: uint64_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut rsize_part: int32_t = 0;
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        *rsize = 0 as int64_t;
        while nbyte > 0x1000000 as uint64_t {
            status = mfs_int_pwrite_common(
                fileinfo,
                &raw mut rsize_part,
                buf,
                0x1000000 as uint32_t,
                offset,
            );
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                return status;
            }
            offset = offset.wrapping_add(rsize_part as uint64_t);
            buf = buf.offset(rsize_part as isize);
            nbyte = nbyte.wrapping_sub(rsize_part as uint64_t);
            *rsize += rsize_part as int64_t;
            if rsize_part < 0x1000000 as int32_t {
                return MFS_ERROR_IO as uint8_t;
            }
        }
        status = mfs_int_pwrite_common(
            fileinfo,
            &raw mut rsize_part,
            buf,
            nbyte as uint32_t,
            offset,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        *rsize += rsize_part as int64_t;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_write(
    mut fildes: ::core::ffi::c_int,
    mut rsize: *mut int64_t,
    mut buf: *const uint8_t,
    mut nbyte: uint64_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut rsize_part: int32_t = 0;
        let mut offset: uint64_t = 0;
        let mut status: uint8_t = 0;
        *rsize = 0 as int64_t;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        offset = (*fileinfo).offset;
        fi_unlock(fileinfo);
        while nbyte > 0x1000000 as uint64_t {
            status = mfs_int_pwrite_common(
                fileinfo,
                &raw mut rsize_part,
                buf,
                0x1000000 as uint32_t,
                offset,
            );
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                return status;
            }
            offset = offset.wrapping_add(rsize_part as uint64_t);
            buf = buf.offset(rsize_part as isize);
            nbyte = nbyte.wrapping_sub(rsize_part as uint64_t);
            *rsize += rsize_part as int64_t;
            if rsize_part < 0x1000000 as int32_t {
                return MFS_ERROR_IO as uint8_t;
            }
        }
        status = mfs_int_pwrite_common(
            fileinfo,
            &raw mut rsize_part,
            buf,
            nbyte as uint32_t,
            offset,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        offset = offset.wrapping_add(rsize_part as uint64_t);
        *rsize += rsize_part as int64_t;
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_APPENDONLY as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_READAPPEND as ::core::ffi::c_int
        {
            (*fileinfo).offset =
                plfsclient::inoleng::get_length((*fileinfo).flengptr.as_ref().unwrap());
        } else {
            (*fileinfo).offset = offset;
        }
        fi_unlock(fileinfo);
        return MFS_STATUS_OK as uint8_t;
    }
}
unsafe extern "C" fn mfs_int_fsync_common(mut fileinfo: *mut file_info) -> uint8_t {
    unsafe {
        let mut status: uint8_t = 0;
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if !(*fileinfo).wdata.is_null()
            && ((*fileinfo).mode as ::core::ffi::c_int != MFS_IO_READONLY as ::core::ffi::c_int
                && (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_ATTRONLY as ::core::ffi::c_int
                && (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_FORBIDDEN as ::core::ffi::c_int
                && (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_DIRECTORY as ::core::ffi::c_int)
        {
            (*fileinfo).writers_cnt = (*fileinfo).writers_cnt.wrapping_add(1);
            while (*fileinfo).readers_cnt | (*fileinfo).writing as uint32_t != 0 {
                fi_cond_wait(fileinfo);
            }
            (*fileinfo).writers_cnt = (*fileinfo).writers_cnt.wrapping_sub(1);
            (*fileinfo).writing = 1 as uint8_t;
            fi_unlock(fileinfo);
            status = mfs_int_error_conv(write_data_flush((*fileinfo).wdata));
            fi_lock(fileinfo);
            (*fileinfo).writing = 0 as uint8_t;
            (*fileinfo).rwcond.notify_all();
        } else {
            status = MFS_STATUS_OK as uint8_t;
        }
        fi_unlock(fileinfo);
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fsync(mut fildes: ::core::ffi::c_int) -> uint8_t {
    unsafe {
        fs_fsync_notify();
        return mfs_int_fsync_common(mfs_get_fi(fildes));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_close(mut fildes: ::core::ffi::c_int) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut decacnt: uint8_t = 0;
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_FORBIDDEN as ::core::ffi::c_int
            || (*fileinfo).mode as ::core::ffi::c_int == MFS_IO_DIRECTORY as ::core::ffi::c_int
        {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        while (*fileinfo).writing as uint32_t | (*fileinfo).writers_cnt | (*fileinfo).readers_cnt
            != 0
        {
            fi_cond_wait(fileinfo);
        }
        if (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_FORBIDDEN as ::core::ffi::c_int {
            decacnt = 1 as uint8_t;
            (*fileinfo).mode = MFS_IO_FORBIDDEN as ::core::ffi::c_int as uint8_t;
        } else {
            decacnt = 0 as uint8_t;
        }
        fi_unlock(fileinfo);
        status = mfs_int_fsync_common(fileinfo);
        if !(*fileinfo).rdata.is_null() {
            read_data_end((*fileinfo).rdata);
            (*fileinfo).rdata = NULL;
        }
        if !(*fileinfo).wdata.is_null() {
            write_data_end((*fileinfo).wdata);
            (*fileinfo).wdata = NULL;
        }
        drop((*fileinfo).flengptr.take());
        if decacnt != 0 {
            fs_dec_acnt((*fileinfo).inode);
        }
        mfs_free_fd(fildes);
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_flock(mut fildes: ::core::ffi::c_int, mut op: uint8_t) -> uint8_t {
    unsafe {
        let mut lock_mode: uint8_t = 0;
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        if op as ::core::ffi::c_int & MFS_LOCK_UN != 0 {
            lock_mode = FLOCK_UNLOCK as uint8_t;
        } else if op as ::core::ffi::c_int & MFS_LOCK_SH != 0 {
            if op as ::core::ffi::c_int & MFS_LOCK_NB != 0 {
                lock_mode = FLOCK_TRY_SHARED as uint8_t;
            } else {
                lock_mode = FLOCK_LOCK_SHARED as uint8_t;
            }
        } else if op as ::core::ffi::c_int & MFS_LOCK_EX != 0 {
            if op as ::core::ffi::c_int & MFS_LOCK_NB != 0 {
                lock_mode = FLOCK_TRY_EXCLUSIVE as uint8_t;
            } else {
                lock_mode = FLOCK_LOCK_EXCLUSIVE as uint8_t;
            }
        } else {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if lock_mode as ::core::ffi::c_int == FLOCK_UNLOCK {
            mfs_int_fsync_common(fileinfo);
        }
        return fs_flock(
            (*fileinfo).inode,
            0 as uint32_t,
            fildes as uint64_t,
            lock_mode,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_lockf(
    mut fildes: ::core::ffi::c_int,
    mut pid: uint32_t,
    mut function: uint8_t,
    mut size: int64_t,
) -> uint8_t {
    unsafe {
        let mut start: uint64_t = 0;
        let mut end: uint64_t = 0;
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        if size > 0 as int64_t {
            start = (*fileinfo).offset;
            end = start.wrapping_add(size as uint64_t);
            if end < start {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        } else if size < 0 as int64_t {
            end = (*fileinfo).offset;
            start = end.wrapping_add(size as uint64_t);
            if end < start {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        } else {
            start = (*fileinfo).offset;
            end = UINT64_MAX as uint64_t;
        }
        if function as ::core::ffi::c_int == MFS_F_ULOCK {
            mfs_int_fsync_common(fileinfo);
            status = fs_posixlock(
                (*fileinfo).inode,
                0 as uint32_t,
                fildes as uint64_t,
                POSIX_LOCK_CMD_SET as uint8_t,
                POSIX_LOCK_UNLCK as uint8_t,
                start,
                end,
                pid,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            );
        } else if function as ::core::ffi::c_int == MFS_F_LOCK {
            status = fs_posixlock(
                (*fileinfo).inode,
                0 as uint32_t,
                fildes as uint64_t,
                POSIX_LOCK_CMD_SET as uint8_t,
                POSIX_LOCK_WRLCK as uint8_t,
                start,
                end,
                pid,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            );
        } else if function as ::core::ffi::c_int == MFS_F_TLOCK {
            status = fs_posixlock(
                (*fileinfo).inode,
                0 as uint32_t,
                fildes as uint64_t,
                POSIX_LOCK_CMD_TRY as uint8_t,
                POSIX_LOCK_WRLCK as uint8_t,
                start,
                end,
                pid,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            );
        } else if function as ::core::ffi::c_int == MFS_F_TEST {
            status = fs_posixlock(
                (*fileinfo).inode,
                0 as uint32_t,
                fildes as uint64_t,
                POSIX_LOCK_CMD_GET as uint8_t,
                POSIX_LOCK_WRLCK as uint8_t,
                start,
                end,
                pid,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            );
        } else {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_fcntl_locks(
    mut fildes: ::core::ffi::c_int,
    mut pid: uint32_t,
    mut function: uint8_t,
    mut fl: *mut mfs_int_flockrec,
) -> uint8_t {
    unsafe {
        let mut start: uint64_t = 0;
        let mut end: uint64_t = 0;
        let mut rstart: uint64_t = 0;
        let mut rend: uint64_t = 0;
        let mut rpid: uint32_t = 0;
        let mut r#type: uint8_t = 0;
        let mut rtype: uint8_t = 0;
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut status: uint8_t = 0;
        fileinfo = mfs_get_fi(fildes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        if (*fl).whence as ::core::ffi::c_int == MFS_SEEK_CUR {
            if (*fl).start > (*fileinfo).offset as int64_t {
                start = 0 as uint64_t;
            } else {
                start = (*fileinfo).offset.wrapping_add((*fl).start as uint64_t);
            }
        } else if (*fl).whence as ::core::ffi::c_int == MFS_SEEK_SET {
            if (*fl).start < 0 as int64_t {
                start = 0 as uint64_t;
            } else {
                start = (*fl).start as uint64_t;
            }
        } else if (*fl).whence as ::core::ffi::c_int == MFS_SEEK_END {
            if (*fl).start
                > plfsclient::inoleng::get_length((*fileinfo).flengptr.as_ref().unwrap()) as int64_t
            {
                start = 0 as uint64_t;
            } else {
                start = plfsclient::inoleng::get_length((*fileinfo).flengptr.as_ref().unwrap())
                    .wrapping_add((*fl).start as uint64_t);
            }
        } else {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (*fl).len <= 0 as int64_t {
            end = UINT64_MAX as uint64_t;
        } else {
            end = start.wrapping_add((*fl).len as uint64_t);
            if end < start {
                end = UINT64_MAX as uint64_t;
            }
        }
        if (*fl).r#type as ::core::ffi::c_int == MFS_F_UNLCK {
            r#type = POSIX_LOCK_UNLCK as uint8_t;
        } else if (*fl).r#type as ::core::ffi::c_int == MFS_F_RDLCK {
            r#type = POSIX_LOCK_RDLCK as uint8_t;
        } else if (*fl).r#type as ::core::ffi::c_int == MFS_F_WRLCK {
            r#type = POSIX_LOCK_WRLCK as uint8_t;
        } else {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if r#type as ::core::ffi::c_int == POSIX_LOCK_UNLCK {
            mfs_int_fsync_common(fileinfo);
        }
        if function as ::core::ffi::c_int == MFS_F_GETLK {
            status = fs_posixlock(
                (*fileinfo).inode,
                0 as uint32_t,
                fildes as uint64_t,
                POSIX_LOCK_CMD_GET as uint8_t,
                r#type,
                start,
                end,
                pid,
                &raw mut rtype,
                &raw mut rstart,
                &raw mut rend,
                &raw mut rpid,
            );
        } else if function as ::core::ffi::c_int == MFS_F_SETLKW {
            status = fs_posixlock(
                (*fileinfo).inode,
                0 as uint32_t,
                fildes as uint64_t,
                POSIX_LOCK_CMD_SET as uint8_t,
                r#type,
                start,
                end,
                pid,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            );
        } else if function as ::core::ffi::c_int == MFS_F_SETLK {
            status = fs_posixlock(
                (*fileinfo).inode,
                0 as uint32_t,
                fildes as uint64_t,
                POSIX_LOCK_CMD_TRY as uint8_t,
                r#type,
                start,
                end,
                pid,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            );
        } else {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if function as ::core::ffi::c_int == MFS_F_GETLK {
            memset(
                fl as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<mfs_int_flockrec>(),
            );
            if rtype as ::core::ffi::c_int == POSIX_LOCK_RDLCK {
                (*fl).r#type = MFS_F_RDLCK as uint8_t;
            } else if rtype as ::core::ffi::c_int == POSIX_LOCK_WRLCK {
                (*fl).r#type = MFS_F_WRLCK as uint8_t;
            } else {
                (*fl).r#type = MFS_F_UNLCK as uint8_t;
            }
            (*fl).whence = MFS_SEEK_SET as uint8_t;
            (*fl).start = rstart as int64_t;
            if rend.wrapping_sub(rstart) > INT64_MAX as uint64_t {
                (*fl).len = 0 as int64_t;
            } else {
                (*fl).len = rend.wrapping_sub(rstart) as int64_t;
            }
            (*fl).pid = rpid;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_opendir(
    mut cr: *mut mfs_int_cred,
    mut dirdes: *mut ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
) -> uint8_t {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: uint8_t = 0;
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        status = mfs_path_to_inodes(
            cr,
            path,
            &raw mut parent,
            &raw mut inode,
            &raw mut name as *mut uint8_t,
            &raw mut nleng,
            PATH_TO_INODES_EXPECT_OBJECT as uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if mfs_attr_to_type(&raw mut attr as *mut uint8_t as *const uint8_t) as ::core::ffi::c_int
            != TYPE_DIRECTORY
        {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        *dirdes = mfs_next_fd();
        fileinfo = mfs_get_fi(*dirdes);
        (*fileinfo).flengptr = None;
        (*fileinfo).inode = inode;
        (*fileinfo).mode = MFS_IO_DIRECTORY as ::core::ffi::c_int as uint8_t;
        (*fileinfo).offset = 0 as uint64_t;
        (*fileinfo).rdata = NULL;
        (*fileinfo).wdata = NULL;
        (*fileinfo).readers_cnt = 0 as uint32_t;
        (*fileinfo).writers_cnt = 0 as uint32_t;
        (*fileinfo).writing = 0 as uint8_t;
        (*fileinfo).privuser = 0 as uint8_t;
        (*fileinfo).reading = 0 as uint8_t;
        (*fileinfo).wasread = 0 as uint8_t;
        (*fileinfo).dataformat = 0 as uint8_t;
        (*fileinfo).dbuff = ::core::ptr::null_mut::<uint8_t>();
        (*fileinfo).dbuffsize = 0 as uint64_t;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_readdir(
    mut cr: *mut mfs_int_cred,
    mut dirdes: ::core::ffi::c_int,
    mut de: *mut mfs_int_direntry,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut tmpdbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut dbuff: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut eptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut nleng: uint8_t = 0;
        let mut newsize: uint64_t = 0;
        let mut edgeid: uint64_t = 0;
        let mut dsize: uint32_t = 0;
        let mut rsize: uint32_t = 0;
        let mut status: uint8_t = 0;
        memset(
            de as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mfs_int_direntry>(),
        );
        fileinfo = mfs_get_fi(dirdes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_DIRECTORY as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        while (*fileinfo).reading != 0 {
            fi_cond_wait(fileinfo);
        }
        if (*fileinfo).wasread as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*fileinfo).offset == 0 as uint64_t
        {
            (*fileinfo).reading = 1 as uint8_t;
            fi_unlock(fileinfo);
            if !(*fileinfo).dbuff.is_null() {
                free((*fileinfo).dbuff as *mut ::core::ffi::c_void);
            }
            (*fileinfo).dbuff = ::core::ptr::null_mut::<uint8_t>();
            (*fileinfo).dbuffsize = 0 as uint64_t;
            edgeid = 0 as uint64_t;
            loop {
                status = fs_readdir(
                    (*fileinfo).inode,
                    (*cr).uid,
                    (*cr).gidcnt,
                    &raw mut (*cr).gidtab as *mut uint32_t,
                    &raw mut edgeid,
                    10000 as uint32_t,
                    0 as uint8_t,
                    0 as uint8_t,
                    &raw mut dbuff,
                    &raw mut dsize,
                );
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    newsize = (*fileinfo).dbuffsize.wrapping_add(dsize as uint64_t);
                    if (*fileinfo).dbuff.is_null() {
                        (*fileinfo).dbuff = malloc(dsize as size_t) as *mut uint8_t;
                    } else {
                        tmpdbuff = (*fileinfo).dbuff;
                        (*fileinfo).dbuff = realloc(
                            (*fileinfo).dbuff as *mut ::core::ffi::c_void,
                            newsize as size_t,
                        ) as *mut uint8_t;
                        if (*fileinfo).dbuff.is_null() {
                            free(tmpdbuff as *mut ::core::ffi::c_void);
                        }
                    }
                    if (*fileinfo).dbuff.is_null() {
                        (*fileinfo).dbuffsize = 0 as uint64_t;
                        status = MFS_ERROR_OUTOFMEMORY as uint8_t;
                        edgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                    } else {
                        memcpy(
                            (*fileinfo).dbuff.offset((*fileinfo).dbuffsize as isize)
                                as *mut ::core::ffi::c_void,
                            dbuff as *const ::core::ffi::c_void,
                            dsize as size_t,
                        );
                        (*fileinfo).dbuffsize = newsize;
                    }
                } else {
                    if !(*fileinfo).dbuff.is_null() {
                        free((*fileinfo).dbuff as *mut ::core::ffi::c_void);
                    }
                    (*fileinfo).dbuff = ::core::ptr::null_mut::<uint8_t>();
                    (*fileinfo).dbuffsize = 0 as uint64_t;
                    edgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                }
                if edgeid == 0x7fffffffffffffff as uint64_t {
                    break;
                }
            }
            fi_lock(fileinfo);
            (*fileinfo).dataformat = 0 as uint8_t;
            (*fileinfo).reading = 0 as uint8_t;
            (*fileinfo).wasread = 1 as uint8_t;
            (*fileinfo).rwcond.notify_all();
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                fi_unlock(fileinfo);
                return status;
            }
        }
        ptr = (*fileinfo).dbuff.offset((*fileinfo).offset as isize);
        eptr = (*fileinfo).dbuff.offset((*fileinfo).dbuffsize as isize);
        if ptr < eptr {
            nleng = *ptr.offset(0 as isize);
            rsize = (nleng as ::core::ffi::c_int
                + (if (*fileinfo).dataformat as ::core::ffi::c_int != 0 {
                    master_attrsize() as ::core::ffi::c_int + 5 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                })) as uint32_t;
        } else {
            nleng = 0 as uint8_t;
            rsize = 0 as uint32_t;
        }
        if rsize > 0 as uint32_t && ptr.offset(rsize as isize) <= eptr {
            (*fileinfo).offset = (*fileinfo).offset.wrapping_add(rsize as uint64_t);
            ptr = ptr.offset(1);
            memcpy(
                &raw mut (*de).name as *mut uint8_t as *mut ::core::ffi::c_void,
                ptr as *const ::core::ffi::c_void,
                nleng as size_t,
            );
            (*de).name[nleng as usize] = 0 as uint8_t;
            ptr = ptr.offset(nleng as ::core::ffi::c_int as isize);
            (*de).inode = get32bit(&raw mut ptr);
            if (*fileinfo).dataformat != 0 {
                (*de).r#type = mfs_attr_get_type(ptr as *const uint8_t);
            } else {
                (*de).r#type = get8bit(&raw mut ptr);
            }
            status = MFS_STATUS_OK as uint8_t;
        } else {
            (*fileinfo).offset = (*fileinfo).dbuffsize;
            status = MFS_ERROR_NOTFOUND as uint8_t;
        }
        fi_unlock(fileinfo);
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_readdirplus(
    mut cr: *mut mfs_int_cred,
    mut dirdes: ::core::ffi::c_int,
    mut de: *mut mfs_int_direntryplus,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        let mut tmpdbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut dbuff: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut eptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut nleng: uint8_t = 0;
        let mut inode: uint32_t = 0;
        let mut newsize: uint64_t = 0;
        let mut edgeid: uint64_t = 0;
        let mut dsize: uint32_t = 0;
        let mut rsize: uint32_t = 0;
        let mut status: uint8_t = 0;
        memset(
            de as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mfs_int_direntry>(),
        );
        fileinfo = mfs_get_fi(dirdes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_DIRECTORY as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        while (*fileinfo).reading != 0 {
            fi_cond_wait(fileinfo);
        }
        if (*fileinfo).wasread as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*fileinfo).offset == 0 as uint64_t
            || (*fileinfo).dataformat as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*fileinfo).reading = 1 as uint8_t;
            fi_unlock(fileinfo);
            if !(*fileinfo).dbuff.is_null() {
                free((*fileinfo).dbuff as *mut ::core::ffi::c_void);
            }
            (*fileinfo).dbuff = ::core::ptr::null_mut::<uint8_t>();
            (*fileinfo).dbuffsize = 0 as uint64_t;
            edgeid = 0 as uint64_t;
            loop {
                status = fs_readdir(
                    (*fileinfo).inode,
                    (*cr).uid,
                    (*cr).gidcnt,
                    &raw mut (*cr).gidtab as *mut uint32_t,
                    &raw mut edgeid,
                    10000 as uint32_t,
                    1 as uint8_t,
                    0 as uint8_t,
                    &raw mut dbuff,
                    &raw mut dsize,
                );
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    newsize = (*fileinfo).dbuffsize.wrapping_add(dsize as uint64_t);
                    if (*fileinfo).dbuff.is_null() {
                        (*fileinfo).dbuff = malloc(dsize as size_t) as *mut uint8_t;
                    } else {
                        tmpdbuff = (*fileinfo).dbuff;
                        (*fileinfo).dbuff = realloc(
                            (*fileinfo).dbuff as *mut ::core::ffi::c_void,
                            newsize as size_t,
                        ) as *mut uint8_t;
                        if (*fileinfo).dbuff.is_null() {
                            free(tmpdbuff as *mut ::core::ffi::c_void);
                        }
                    }
                    if (*fileinfo).dbuff.is_null() {
                        (*fileinfo).dbuffsize = 0 as uint64_t;
                        status = MFS_ERROR_OUTOFMEMORY as uint8_t;
                        edgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                    } else {
                        memcpy(
                            (*fileinfo).dbuff.offset((*fileinfo).dbuffsize as isize)
                                as *mut ::core::ffi::c_void,
                            dbuff as *const ::core::ffi::c_void,
                            dsize as size_t,
                        );
                        (*fileinfo).dbuffsize = newsize;
                    }
                } else {
                    if !(*fileinfo).dbuff.is_null() {
                        free((*fileinfo).dbuff as *mut ::core::ffi::c_void);
                    }
                    (*fileinfo).dbuff = ::core::ptr::null_mut::<uint8_t>();
                    (*fileinfo).dbuffsize = 0 as uint64_t;
                    edgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                }
                if edgeid == 0x7fffffffffffffff as uint64_t {
                    break;
                }
            }
            fi_lock(fileinfo);
            (*fileinfo).dataformat = 1 as uint8_t;
            (*fileinfo).reading = 0 as uint8_t;
            (*fileinfo).wasread = 1 as uint8_t;
            (*fileinfo).rwcond.notify_all();
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                fi_unlock(fileinfo);
                return status;
            }
        }
        ptr = (*fileinfo).dbuff.offset((*fileinfo).offset as isize);
        eptr = (*fileinfo).dbuff.offset((*fileinfo).dbuffsize as isize);
        if ptr < eptr {
            nleng = *ptr.offset(0 as isize);
            rsize = (nleng as ::core::ffi::c_int
                + (if (*fileinfo).dataformat as ::core::ffi::c_int != 0 {
                    master_attrsize() as ::core::ffi::c_int + 5 as ::core::ffi::c_int
                } else {
                    6 as ::core::ffi::c_int
                })) as uint32_t;
        } else {
            nleng = 0 as uint8_t;
            rsize = 0 as uint32_t;
        }
        if rsize > 0 as uint32_t && ptr.offset(rsize as isize) <= eptr {
            (*fileinfo).offset = (*fileinfo).offset.wrapping_add(rsize as uint64_t);
            ptr = ptr.offset(1);
            memcpy(
                &raw mut (*de).name as *mut uint8_t as *mut ::core::ffi::c_void,
                ptr as *const ::core::ffi::c_void,
                nleng as size_t,
            );
            (*de).name[nleng as usize] = 0 as uint8_t;
            ptr = ptr.offset(nleng as ::core::ffi::c_int as isize);
            inode = get32bit(&raw mut ptr);
            mfs_attr_to_direntry(inode, ptr as *const uint8_t, de);
            status = MFS_STATUS_OK as uint8_t;
        } else {
            (*fileinfo).offset = (*fileinfo).dbuffsize;
            status = MFS_ERROR_NOTFOUND as uint8_t;
        }
        fi_unlock(fileinfo);
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_telldir(
    mut dirdes: ::core::ffi::c_int,
    mut offset: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        fileinfo = mfs_get_fi(dirdes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_DIRECTORY as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        *offset = (*fileinfo).offset;
        fi_unlock(fileinfo);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_seekdir(
    mut dirdes: ::core::ffi::c_int,
    mut offset: uint64_t,
) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        fileinfo = mfs_get_fi(dirdes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_DIRECTORY as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        if offset > (*fileinfo).dbuffsize {
            offset = 0 as uint64_t;
        }
        (*fileinfo).offset = offset;
        fi_unlock(fileinfo);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_rewinddir(mut dirdes: ::core::ffi::c_int) -> uint8_t {
    unsafe {
        return mfs_int_seekdir(dirdes, 0 as uint64_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_closedir(mut dirdes: ::core::ffi::c_int) -> uint8_t {
    unsafe {
        let mut fileinfo: *mut file_info = ::core::ptr::null_mut::<file_info>();
        fileinfo = mfs_get_fi(dirdes);
        if fileinfo.is_null() {
            return MFS_ERROR_EBADF as uint8_t;
        }
        fi_lock(fileinfo);
        if (*fileinfo).mode as ::core::ffi::c_int != MFS_IO_DIRECTORY as ::core::ffi::c_int {
            fi_unlock(fileinfo);
            return MFS_ERROR_EACCES as uint8_t;
        }
        (*fileinfo).mode = MFS_IO_FORBIDDEN as ::core::ffi::c_int as uint8_t;
        fi_unlock(fileinfo);
        if !(*fileinfo).dbuff.is_null() {
            free((*fileinfo).dbuff as *mut ::core::ffi::c_void);
        }
        mfs_free_fd(dirdes);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_get_config_str(
    mut option_name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut vleng: uint8_t = 0;
        let mut vdata: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut status: uint8_t = 0;
        status = fs_get_cfg(option_name, &raw mut vleng, &raw mut vdata);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        ret = malloc((1 as ::core::ffi::c_int + vleng as ::core::ffi::c_int) as size_t)
            as *mut ::core::ffi::c_char;
        memcpy(
            ret as *mut ::core::ffi::c_void,
            vdata as *const ::core::ffi::c_void,
            vleng as size_t,
        );
        *ret.offset(vleng as isize) = 0 as ::core::ffi::c_char;
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_get_config_file(
    mut option_name: *const ::core::ffi::c_char,
) -> *mut data_buff {
    unsafe {
        let mut vleng: uint16_t = 0;
        let mut vdata: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ret: *mut data_buff = ::core::ptr::null_mut::<data_buff>();
        let mut status: uint8_t = 0;
        status = fs_get_cfg_file(option_name, &raw mut vleng, &raw mut vdata);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return ::core::ptr::null_mut::<data_buff>();
        }
        ret = malloc((4 as size_t).wrapping_add(vleng as size_t)) as *mut data_buff;
        (*ret).leng = vleng as uint32_t;
        memcpy(
            &raw mut (*ret).data as *mut uint8_t as *mut ::core::ffi::c_void,
            vdata as *const ::core::ffi::c_void,
            vleng as size_t,
        );
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_set_defaults(mut mcfg: *mut mfs_int_cfg) {
    unsafe {
        memset(
            mcfg as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mfs_int_cfg>(),
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
        (*mcfg).logident = strdup(b"libmfsio_int\0".as_ptr() as *const ::core::ffi::c_char);
        (*mcfg).logdaemon = 0 as ::core::ffi::c_int;
        (*mcfg).logminlevel = MFSLOG_INFO;
        (*mcfg).logelevateto = MFSLOG_NOTICE;
        (*mcfg).master_min_version_maj = 0 as uint16_t;
        (*mcfg).master_min_version_mid = 0 as uint16_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_init(
    mut mcfg: *mut mfs_int_cfg,
    mut stage: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut ctx: md5ctx = md5ctx {
            state: [0; 4],
            count: [0; 2],
            buffer: [0; 64],
        };
        let mut md5pass: [uint8_t; 16] = [0; 16];
        if stage as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || stage as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        {
            mfs_log_init((*mcfg).logident, (*mcfg).logdaemon);
            mfs_log_set_min_level((*mcfg).logminlevel);
            mfs_log_set_elevate_to((*mcfg).logelevateto);
            if lcache_init((*mcfg).lcache_retention) < 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if csorder_init((*mcfg).preferedlabels) < 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if !(*mcfg).masterpassword.is_null() {
                md5_init(&raw mut ctx);
                md5_update(
                    &raw mut ctx,
                    (*mcfg).masterpassword as *mut uint8_t,
                    strlen((*mcfg).masterpassword) as uint32_t,
                );
                md5_final(&raw mut md5pass as *mut uint8_t, &raw mut ctx);
                memset(
                    (*mcfg).masterpassword as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    strlen((*mcfg).masterpassword),
                );
            } else if !(*mcfg).mastermd5pass.is_null() {
                let mut p: *mut uint8_t = (*mcfg).mastermd5pass as *mut uint8_t;
                i = 0 as uint32_t;
                while i < 16 as uint32_t {
                    if *p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && *p as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        md5pass[i as usize] =
                            ((*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                                << 4 as ::core::ffi::c_int) as uint8_t;
                    } else if *p as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                        && *p as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
                    {
                        md5pass[i as usize] =
                            ((*p as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int)
                                << 4 as ::core::ffi::c_int) as uint8_t;
                    } else if *p as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                        && *p as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                    {
                        md5pass[i as usize] =
                            ((*p as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                                + 10 as ::core::ffi::c_int)
                                << 4 as ::core::ffi::c_int) as uint8_t;
                    } else {
                        return -1 as ::core::ffi::c_int;
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
                        return -1 as ::core::ffi::c_int;
                    }
                    p = p.offset(1);
                    i = i.wrapping_add(1);
                }
                if *p != 0 {
                    return -1 as ::core::ffi::c_int;
                }
                memset(
                    (*mcfg).mastermd5pass as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    strlen((*mcfg).mastermd5pass),
                );
            }
            strerr_init();
            mycrc32_init();
            if fs_init_master_connection(
                (*mcfg).masterbind,
                (*mcfg).masterhost,
                (*mcfg).masterport,
                0 as uint8_t,
                (*mcfg).mountpoint,
                (*mcfg).masterpath,
                (if !(*mcfg).masterpassword.is_null() || !(*mcfg).mastermd5pass.is_null() {
                    &raw mut md5pass as *mut uint8_t
                } else {
                    ::core::ptr::null_mut::<uint8_t>()
                }) as *const uint8_t,
                1 as uint8_t,
                0 as uint8_t,
                ((*mcfg).master_min_version_maj as ::core::ffi::c_int
                    * 0x10000 as ::core::ffi::c_int
                    + (*mcfg).master_min_version_mid as ::core::ffi::c_int
                        * 0x100 as ::core::ffi::c_int
                    + (if (*mcfg).master_min_version_maj as ::core::ffi::c_int
                        > 1 as ::core::ffi::c_int
                    {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t,
            ) < 0 as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            memset(
                &raw mut md5pass as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                16 as size_t,
            );
        }
        if stage as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || stage as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        {
            plfsclient::inoleng::init();
            conncache_init(200 as uint32_t);
            plfsclient::chunkrwlock::init();
            plfsclient::chunksdatacache::init();
            read_init();
            write_init();
            fs_init_threads(
                (*mcfg).io_try_cnt as uint32_t,
                (*mcfg).io_timeout as uint32_t,
            );
            plfsclient::csdb::init();
            delay_init();
            read_data_init(
                ((*mcfg).read_cache_mb * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int)
                    as uint64_t,
                (*mcfg).readahead_leng as uint32_t,
                (*mcfg).readahead_trigger as uint32_t,
                (*mcfg).io_try_cnt as uint32_t,
                (*mcfg).io_timeout as uint32_t,
                (*mcfg).min_log_entry as uint32_t,
                (*mcfg).error_on_lost_chunk as uint8_t,
                (*mcfg).error_on_no_space as uint8_t,
            );
            write_data_init(
                ((*mcfg).write_cache_mb * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int)
                    as uint32_t,
                (*mcfg).io_try_cnt as uint32_t,
                (*mcfg).io_timeout as uint32_t,
                (*mcfg).min_log_entry as uint32_t,
                (*mcfg).error_on_lost_chunk as uint8_t,
                (*mcfg).error_on_no_space as uint8_t,
            );
            fdtab =
                malloc(::core::mem::size_of::<file_info>().wrapping_mul(FDTABSIZE_INIT as size_t))
                    as *mut file_info;
            fdtabsize = FDTABSIZE_INIT as uint32_t;
            fdtabusemask = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(
                ((FDTABSIZE_INIT + 31 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int) as size_t,
            )) as *mut uint32_t;
            if fdtab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"fdtab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"fdtab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if fdtab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut file_info
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"fdtab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2481 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"fdtab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
            if fdtabusemask.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2482 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"fdtabusemask\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2482 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"fdtabusemask\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if fdtabusemask
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2482 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"fdtabusemask\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsioint.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2482 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"fdtabusemask\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                abort();
            }
            i = 0 as uint32_t;
            while i < fdtabsize {
                mfs_fi_init(fdtab.offset(i as isize));
                i = i.wrapping_add(1);
            }
            memset(
                fdtabusemask as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<uint32_t>().wrapping_mul(
                    ((FDTABSIZE_INIT + 31 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int)
                        as size_t,
                ),
            );
            if (*mcfg).mkdir_copy_sgid < 0 as ::core::ffi::c_int {
                mkdir_copy_sgid = 1 as ::core::ffi::c_int;
            } else {
                mkdir_copy_sgid = (*mcfg).mkdir_copy_sgid;
            }
            if (*mcfg).sugid_clear_mode < 0 as ::core::ffi::c_int {
                sugid_clear_mode = SUGID_CLEAR_MODE_EXT;
            } else {
                sugid_clear_mode = (*mcfg).sugid_clear_mode;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_int_term() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < fdtabsize {
            mfs_int_close(i as ::core::ffi::c_int);
            mfs_fi_term(fdtab.offset(i as isize));
            i = i.wrapping_add(1);
        }
        free(fdtabusemask as *mut ::core::ffi::c_void);
        free(fdtab as *mut ::core::ffi::c_void);
        fdtab_lock();
        fdtab_unlock();
        // C pthread_mutex_destroy(&fdtablock): static FDTAB_LOCK needs no destroy.
        write_data_term();
        read_data_term();
        delay_term();
        plfsclient::csdb::term();
        fs_term();
        write_term();
        read_term();
        plfsclient::chunksdatacache::term();
        plfsclient::chunkrwlock::term();
        conncache_term();
        plfsclient::inoleng::term();
        plfsclient::stats::term();
        lcache_term();
        mfs_log_term();
    }
}
