pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum _bio {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    unsafe fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_skip(b: *mut bio, len: uint64_t);
    unsafe fn bio_error(b: *mut bio) -> uint8_t;
    unsafe fn sclass_incref(sclassid: uint16_t, r#type: uint8_t);
    unsafe fn sclass_decref(sclassid: uint16_t, r#type: uint8_t);
    unsafe fn sclass_get_arch_mode(sclassid: uint16_t) -> uint8_t;
    unsafe fn sclass_get_keeparch_storage_eights(sclassid: uint16_t, keepflag: uint8_t) -> uint8_t;
    unsafe fn sclass_get_keeparch_maxstorage_eights(sclassid: uint16_t) -> uint8_t;
    unsafe fn sclass_is_admin_only(sclassid: uint16_t) -> uint8_t;
    unsafe fn sclass_get_arch_delay(sclassid: uint16_t) -> uint16_t;
    unsafe fn sclass_get_arch_min_size(sclassid: uint16_t) -> uint64_t;
    unsafe fn sclass_get_min_trashretention(sclassid: uint16_t) -> uint16_t;
    unsafe fn chunk_mr_multi_modify(
        ts: uint32_t,
        nchunkid: *mut uint64_t,
        ochunkid: uint64_t,
        sclassid: uint8_t,
        opflag: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_mr_multi_truncate(
        ts: uint32_t,
        nchunkid: *mut uint64_t,
        ochunkid: uint64_t,
        sclassid: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_mr_unlock(ts: uint32_t, chunkid: uint64_t) -> ::core::ffi::c_int;
    unsafe fn chunk_mr_set_version(chunkid: uint64_t, version: uint32_t) -> ::core::ffi::c_int;
    unsafe fn chunk_count() -> uint32_t;
    unsafe fn chunk_counters_in_progress() -> uint8_t;
    unsafe fn chunk_get_archflag(chunkid: uint64_t, archflag: *mut uint8_t) -> ::core::ffi::c_int;
    unsafe fn chunk_set_archflag(
        chunkid: uint64_t,
        archflag: uint8_t,
        archflagchanged: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_set_trashflag(chunkid: uint64_t, trashflag: uint8_t) -> ::core::ffi::c_int;
    unsafe fn chunk_set_autoarch(
        chunkid: uint64_t,
        archreftime: uint32_t,
        archflagchanged: *mut uint32_t,
        intrash: uint8_t,
        trashflagchanged: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_fileloop_task(
        chunkid: uint64_t,
        sclassid: uint8_t,
        aftereof: uint8_t,
        archreftime: uint32_t,
        archflagchanged: *mut uint32_t,
        intrash: uint8_t,
        trashflagchanged: *mut uint32_t,
    ) -> chunkfloop;
    unsafe fn chunk_read_check(
        ts: uint32_t,
        chunkid: uint64_t,
        allow_recover: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_multi_modify(
        continueop: uint8_t,
        nchunkid: *mut uint64_t,
        ochunkid: uint64_t,
        sclassid: uint8_t,
        opflag: *mut uint8_t,
        clientip: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_multi_truncate(
        nchunkid: *mut uint64_t,
        ochunkid: uint64_t,
        length: uint32_t,
        sclassid: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_repair(
        sclassid: uint8_t,
        ochunkid: uint64_t,
        flags: uint8_t,
        nversion: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_get_eights_copies(chunkid: uint64_t, count: *mut uint8_t) -> uint8_t;
    unsafe fn chunk_get_storage_status(
        chunkid: uint64_t,
        fcopies: *mut uint8_t,
        ec8parts: *mut uint8_t,
        ec4parts: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn chunk_change_file(
        chunkid: uint64_t,
        prevsclassid: uint8_t,
        newsclassid: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_delete_file(chunkid: uint64_t, sclassid: uint8_t) -> ::core::ffi::c_int;
    unsafe fn chunk_add_file(chunkid: uint64_t, sclassid: uint8_t) -> ::core::ffi::c_int;
    unsafe fn chunk_unlock(ts: uint32_t, chunkid: uint64_t) -> ::core::ffi::c_int;
    unsafe fn matocsserv_have_availspace() -> ::core::ffi::c_int;
    unsafe fn matocsserv_getspace(
        totalspace: *mut uint64_t,
        availspace: *mut uint64_t,
        freespace: *mut uint64_t,
    );
    unsafe fn appendres_getvleng(inode: uint32_t) -> uint64_t;
    unsafe fn appendres_setvleng(inode: uint32_t, vlength: uint64_t);
    unsafe fn appendres_setrleng(inode: uint32_t, rlength: uint64_t);
    unsafe fn appendres_clear(inode: uint32_t);
    unsafe fn appendres_cleanall();
    unsafe fn appendres_init();
    unsafe fn of_sessions_info_for_inode(inode: uint32_t, dbuff: *mut uint8_t) -> uint32_t;
    unsafe fn of_isfileopen(inode: uint32_t) -> uint8_t;
    unsafe fn of_mr_acquire(sessionid: uint32_t, inode: uint32_t) -> ::core::ffi::c_int;
    unsafe fn xattr_namecheck(anleng: uint8_t, attrname: *const uint8_t) -> ::core::ffi::c_int;
    unsafe fn xattr_removeinode(inode: uint32_t);
    unsafe fn xattr_setattr(
        inode: uint32_t,
        anleng: uint8_t,
        attrname: *const uint8_t,
        avleng: uint32_t,
        attrvalue: *const uint8_t,
        mode: uint8_t,
    ) -> uint8_t;
    unsafe fn xattr_getattr(
        inode: uint32_t,
        anleng: uint8_t,
        attrname: *const uint8_t,
        avleng: *mut uint32_t,
        attrvalue: *mut *const uint8_t,
    ) -> uint8_t;
    unsafe fn xattr_listattr_leng(
        inode: uint32_t,
        xanode: *mut *mut ::core::ffi::c_void,
        xasize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn xattr_listattr_data(xanode: *mut ::core::ffi::c_void, xabuff: *mut uint8_t);
    unsafe fn xattr_getall(inode: uint32_t, dbuff: *mut uint8_t) -> uint32_t;
    unsafe fn xattr_check(
        inode: uint32_t,
        dbuff: *const uint8_t,
        leng: uint32_t,
        pleng: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn xattr_setall(inode: uint32_t, dbuff: *const uint8_t) -> uint8_t;
    unsafe fn xattr_copy(srcinode: uint32_t, dstinode: uint32_t) -> uint8_t;
    unsafe fn posix_acl_getmode(inode: uint32_t) -> uint16_t;
    unsafe fn posix_acl_setmode(inode: uint32_t, mode: uint16_t);
    unsafe fn posix_acl_accmode(
        inode: uint32_t,
        auid: uint32_t,
        agids: uint32_t,
        agid: *mut uint32_t,
        fuid: uint32_t,
        fgid: uint32_t,
    ) -> uint8_t;
    unsafe fn posix_acl_remove(inode: uint32_t, acltype: uint8_t);
    unsafe fn posix_acl_copydefaults(
        parent: uint32_t,
        inode: uint32_t,
        directory: uint8_t,
        mode: *mut uint16_t,
    ) -> uint8_t;
    unsafe fn posix_acl_set(
        inode: uint32_t,
        acltype: uint8_t,
        userperm: uint16_t,
        groupperm: uint16_t,
        otherperm: uint16_t,
        mask: uint16_t,
        namedusers: uint16_t,
        namedgroups: uint16_t,
        aclblob: *const uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn posix_acl_get_blobsize(
        inode: uint32_t,
        acltype: uint8_t,
        aclnode: *mut *mut ::core::ffi::c_void,
    ) -> int32_t;
    unsafe fn posix_acl_get_data(
        aclnode: *mut ::core::ffi::c_void,
        userperm: *mut uint16_t,
        groupperm: *mut uint16_t,
        otherperm: *mut uint16_t,
        mask: *mut uint16_t,
        namedusers: *mut uint16_t,
        namedgroups: *mut uint16_t,
        aclblob: *mut uint8_t,
    );
    unsafe fn posix_acl_getall(inode: uint32_t, acltype: uint8_t, dbuff: *mut uint8_t) -> uint32_t;
    unsafe fn posix_acl_check(
        inode: uint32_t,
        acltype: uint8_t,
        userperm: uint16_t,
        groupperm: uint16_t,
        otherperm: uint16_t,
        mask: uint16_t,
        namedusers: uint16_t,
        namedgroups: uint16_t,
        aclblob: *const uint8_t,
    ) -> uint8_t;
    unsafe fn posix_acl_copy(srcinode: uint32_t, dstinode: uint32_t, acltype: uint8_t) -> uint8_t;
    unsafe fn meta_version_inc() -> uint64_t;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn dcm_modify(inode: uint32_t, sessionid: uint32_t);
    unsafe fn glob_match(
        glob: *mut ::core::ffi::c_void,
        name: *const uint8_t,
        nleng: uint8_t,
    ) -> uint8_t;
    unsafe fn glob_cache_get(gnleng: uint8_t, gname: *const uint8_t) -> *mut ::core::ffi::c_void;
    unsafe fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    unsafe fn cfg_getsperiod(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> uint32_t;
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_msectime_register_fname(
        mseconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn main_time() -> uint32_t;
    unsafe fn main_keep_alive();
    unsafe fn changelog(format: *const ::core::ffi::c_char, ...);
    unsafe fn changelog_generate_gids(
        gids: uint32_t,
        gid: *mut uint32_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn changelog_escape_name(
        nleng: uint32_t,
        name: *const uint8_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn chash_new() -> *mut ::core::ffi::c_void;
    unsafe fn chash_find(h: *mut ::core::ffi::c_void, x: hash_key_t) -> *mut ::core::ffi::c_void;
    unsafe fn chash_add(h: *mut ::core::ffi::c_void, x: hash_key_t, v: *mut ::core::ffi::c_void);
    unsafe fn chash_erase(h: *mut ::core::ffi::c_void);
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
    unsafe fn patterns_find_matching(
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        scid: *mut uint8_t,
        trashretention: *mut uint16_t,
        seteattr: *mut uint8_t,
        clreattr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn missing_log_insert(
        chunkid: uint64_t,
        inode: uint32_t,
        indx: uint32_t,
        r#type: uint8_t,
    );
    unsafe fn missing_log_swap();
    unsafe fn rndu32() -> uint32_t;
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
pub type ssize_t = isize;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
pub type chunkfloop = ::core::ffi::c_uint;
pub const CHUNK_FLOOP_OK: chunkfloop = 7;
pub const CHUNK_FLOOP_UNDERGOAL: chunkfloop = 6;
pub const CHUNK_FLOOP_MISSING_PARTIALEC: chunkfloop = 5;
pub const CHUNK_FLOOP_MISSING_WRONGVERSION: chunkfloop = 4;
pub const CHUNK_FLOOP_MISSING_INVALID: chunkfloop = 3;
pub const CHUNK_FLOOP_MISSING_NOCOPY: chunkfloop = 2;
pub const CHUNK_FLOOP_DELETED: chunkfloop = 1;
pub const CHUNK_FLOOP_NOTFOUND: chunkfloop = 0;
pub type fsnode = _fsnode;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _fsnode {
    pub inode: uint32_t,
    pub ctime: uint32_t,
    pub mtime: uint32_t,
    pub atime: uint32_t,
    pub uid: uint32_t,
    pub gid: uint32_t,
    #[bitfield(name = "xattrflag", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "aclpermflag", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "acldefflag", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "keepmode", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "r#type", ty = "::core::ffi::c_uint", bits = "4..=7")]
    #[bitfield(name = "mode", ty = "::core::ffi::c_uint", bits = "8..=19")]
    pub xattrflag_aclpermflag_acldefflag_keepmode_type_mode: [u8; 3],
    pub sclassid: uint8_t,
    pub eattr: uint8_t,
    pub winattr: uint8_t,
    pub trashretention: uint16_t,
    pub parents: *mut fsedge,
    pub next: *mut _fsnode,
    pub data: _data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _data {
    pub ddata: _ddata,
    pub sdata: _sdata,
    pub devdata: _devdata,
    pub fdata: _fdata,
    pub odata: _odata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _odata {
    pub nlink: uint16_t,
    pub end: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fdata {
    pub length: uint64_t,
    pub chunktab: *mut uint64_t,
    pub chunks: uint32_t,
    pub nlink: uint16_t,
    pub realsize_ratio: uint8_t,
    pub end: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _devdata {
    pub rdev: uint32_t,
    pub nlink: uint16_t,
    pub end: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sdata {
    pub path: *mut uint8_t,
    pub pleng: uint32_t,
    pub nlink: uint16_t,
    pub end: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ddata {
    pub children: *mut fsedge,
    pub nlink: uint32_t,
    pub elements: uint32_t,
    pub stats: statsrecord,
    pub quota: *mut quotanode,
    pub end: uint8_t,
}
pub type quotanode = _quotanode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _quotanode {
    pub graceperiod: uint32_t,
    pub exceeded: uint8_t,
    pub flags: uint8_t,
    pub stimestamp: uint32_t,
    pub sinodes: uint32_t,
    pub hinodes: uint32_t,
    pub slength: uint64_t,
    pub hlength: uint64_t,
    pub ssize: uint64_t,
    pub hsize: uint64_t,
    pub srealsize: uint64_t,
    pub hrealsize: uint64_t,
    pub node: *mut _fsnode,
    pub next: *mut _quotanode,
    pub prev: *mut *mut _quotanode,
}
pub type statsrecord = _statsrecord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _statsrecord {
    pub inodes: uint32_t,
    pub dirs: uint32_t,
    pub files: uint32_t,
    pub chunks: uint32_t,
    pub length: uint64_t,
    pub size: uint64_t,
    pub realsize: uint64_t,
}
pub type fsedge = _fsedge;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fsedge {
    pub child: *mut _fsnode,
    pub parent: *mut _fsnode,
    pub nextchild: *mut _fsedge,
    pub nextparent: *mut _fsedge,
    pub prevchild: *mut *mut _fsedge,
    pub prevparent: *mut *mut _fsedge,
    pub next: *mut _fsedge,
    pub edgeid: uint64_t,
    pub hashval: uint32_t,
    pub nleng: uint16_t,
    pub name: [uint8_t; 1],
}
pub type chunktab_bucket = _chunktab_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunktab_bucket {
    pub firstfree: uint64_t,
    pub next: *mut _chunktab_bucket,
    pub bucket: [uint8_t; 1],
}
pub type fsedge_bucket = _fsedge_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fsedge_bucket {
    pub firstfree: uint32_t,
    pub next: *mut _fsedge_bucket,
    pub bucket: [uint8_t; 1],
}
pub type fsnode_bucket = _fsnode_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fsnode_bucket {
    pub firstfree: uint32_t,
    pub next: *mut _fsnode_bucket,
    pub bucket: [uint8_t; 1],
}
pub type freenode = _freenode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _freenode {
    pub inode: uint32_t,
    pub ftime: uint32_t,
    pub next: *mut _freenode,
}
pub type freenode_bucket = _freenode_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _freenode_bucket {
    pub bucket: [freenode; 625000],
    pub firstfree: uint32_t,
    pub next: *mut _freenode_bucket,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _node_list {
    pub node: *mut fsnode,
    pub next: *mut _node_list,
}
pub type symlink_bucket = _symlink_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _symlink_bucket {
    pub firstfree: uint32_t,
    pub next: *mut _symlink_bucket,
    pub bucket: [uint8_t; 1],
}
pub type fsnodes_snapshot_params = _fsnodes_snapshot_params;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fsnodes_snapshot_params {
    pub ts: uint32_t,
    pub smode: uint8_t,
    pub sesflags: uint8_t,
    pub cumask: uint16_t,
    pub uid: uint32_t,
    pub gids: uint32_t,
    pub gid: *mut uint32_t,
    pub inode_chksum: uint32_t,
    pub removed_object: uint32_t,
    pub same_file: uint32_t,
    pub existing_object: uint32_t,
    pub new_hardlink: uint32_t,
    pub new_object: uint32_t,
}
pub type hash_key_t = uint64_t;
pub type quotanode_bucket = _quotanode_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _quotanode_bucket {
    pub bucket: [quotanode; 104166],
    pub firstfree: uint32_t,
    pub next: *mut _quotanode_bucket,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct facldata {
    pub valid: uint8_t,
    pub userperm: uint16_t,
    pub groupperm: uint16_t,
    pub otherperm: uint16_t,
    pub mask: uint16_t,
    pub namedusers: uint16_t,
    pub namedgroups: uint16_t,
    pub nameddataptr: *const uint8_t,
}
pub type bstnode = _bstnode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bstnode {
    pub val: uint32_t,
    pub count: uint32_t,
    pub left: *mut _bstnode,
    pub right: *mut _bstnode,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_ANON: ::core::ffi::c_int = MAP_ANONYMOUS;
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const MFSCHUNKMASK: ::core::ffi::c_int = 0x3ffffff as ::core::ffi::c_int;
pub const MFSCHUNKBITS: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const MFSBLOCKSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const MFSBLOCKNEGMASK: ::core::ffi::c_int = 0x7fff0000 as ::core::ffi::c_int;
pub const MFSHDRSIZE: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const MFS_ROOT_ID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAXSCLASS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MFS_NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const MFS_SYMLINK_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MFS_PATH_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const TRASH_BUCKETS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const SUSTAINED_BUCKETS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOTDIR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_ERROR_EACCES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_ERROR_EEXIST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOTEMPTY: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const MFS_ERROR_INDEXTOOBIG: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MFS_ERROR_NOCHUNK: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSPACE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const MFS_ERROR_DELAYED: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MFS_ERROR_CANTCREATEPATH: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_ERROR_EROFS: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const MFS_ERROR_QUOTA: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOATTR: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const MFS_ERROR_ERANGE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT_NOCACHE: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const MFS_ERROR_ENAMETOOLONG: ::core::ffi::c_int = 58 as ::core::ffi::c_int;
pub const MFS_ERROR_EMLINK: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_CTIME: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_MTIME: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_ATIME: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_REVERSIBLE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_FAST: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_CHUNK: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const PATTERN_OMASK_SCLASS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PATTERN_OMASK_TRASHRETENTION: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PATTERN_OMASK_EATTR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const DISP_TYPE_FILE: ::core::ffi::c_int = 102;
pub const DISP_TYPE_DIRECTORY: ::core::ffi::c_int = 100;
pub const DISP_TYPE_SYMLINK: ::core::ffi::c_int = 108;
pub const DISP_TYPE_FIFO: ::core::ffi::c_int = 113;
pub const DISP_TYPE_BLOCKDEV: ::core::ffi::c_int = 98;
pub const DISP_TYPE_CHARDEV: ::core::ffi::c_int = 99;
pub const DISP_TYPE_SOCKET: ::core::ffi::c_int = 115;
pub const DISP_TYPE_TRASH: ::core::ffi::c_int = 116;
pub const DISP_TYPE_SUSTAINED: ::core::ffi::c_int = 114;
pub const DISP_TYPE_REMAP_STR: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"?fdlqbcstr??????\0")
};
pub const TYPE_FILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TYPE_DIRECTORY: ::core::ffi::c_int = 2;
pub const TYPE_SYMLINK: ::core::ffi::c_int = 3;
pub const TYPE_FIFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TYPE_BLOCKDEV: ::core::ffi::c_int = 5;
pub const TYPE_CHARDEV: ::core::ffi::c_int = 6;
pub const TYPE_SOCKET: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const TYPE_TRASH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TYPE_SUSTAINED: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const WINATTR_READ_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MODE_MASK_R: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MODE_MASK_W: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MODE_MASK_X: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOOKUP_ACCESS_MODES_RO: ::core::ffi::c_int = 0xff33 as ::core::ffi::c_int;
pub const LOOKUP_IMMUTABLE: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const LOOKUP_DIRECTMODE: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const LOOKUP_APPENDONLY: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const MODE_TO_ACCMODE: [uint8_t; 8] = [
    0x1 as uint8_t,
    0x3 as uint8_t,
    0x5 as uint8_t,
    0xf as uint8_t,
    0x11 as uint8_t,
    0x33 as uint8_t,
    0x55 as uint8_t,
    0xff as uint8_t,
];
pub const SET_WINATTR_FLAG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SET_MODE_FLAG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SET_UID_FLAG: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SET_GID_FLAG: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SET_MTIME_NOW_FLAG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SET_MTIME_FLAG: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SET_ATIME_FLAG: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SET_ATIME_NOW_FLAG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const MFS_RENAME_EXCHANGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_RENAME_NOREPLACE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DTYPE_TRASH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DTYPE_SUSTAINED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SMODE_SET: ::core::ffi::c_int = 0;
pub const SMODE_INCREASE: ::core::ffi::c_int = 1;
pub const SMODE_DECREASE: ::core::ffi::c_int = 2;
pub const SMODE_EXCHANGE: ::core::ffi::c_int = 3;
pub const SMODE_TMASK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SMODE_RMASK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const GMODE_RECURSIVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EATTR_BITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const EATTR_NOOWNER: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const EATTR_NOACACHE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const EATTR_NOECACHE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const EATTR_NODATACACHE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const EATTR_SNAPSHOT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const EATTR_UNDELETABLE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EATTR_APPENDONLY: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const EATTR_IMMUTABLE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const MATTR_NOACACHE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MATTR_NOECACHE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MATTR_ALLOWDATACACHE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MATTR_NOXATTR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MATTR_DIRECTMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MATTR_UNDELETABLE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const QUOTA_FLAG_SINODES: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const QUOTA_FLAG_SLENGTH: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const QUOTA_FLAG_SSIZE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const QUOTA_FLAG_SREALSIZE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const QUOTA_FLAG_HINODES: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const QUOTA_FLAG_HLENGTH: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const QUOTA_FLAG_HSIZE: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const QUOTA_FLAG_HREALSIZE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const QUOTA_PERIOD_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const APPEND_SLICE_FROM_NEG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const APPEND_SLICE_TO_NEG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const POSIX_ACL_ACCESS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_ACL_DEFAULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ARCHCTL_CLR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ARCHCTL_SET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GETDIR_FLAG_WITHATTR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_OPENED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_UPDATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_TIMEFIX: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_RESERVE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SESFLAG_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SESFLAG_IGNOREGID: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SESFLAG_ADMIN: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SESFLAG_MAPALL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SESFLAG_ATTRBIT: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SESFLAG_METARESTORE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_ALWAYS: ::core::ffi::c_int = 1;
pub const SUGID_CLEAR_MODE_OSX: ::core::ffi::c_int = 2;
pub const SUGID_CLEAR_MODE_BSD: ::core::ffi::c_int = 3;
pub const SUGID_CLEAR_MODE_EXT: ::core::ffi::c_int = 4;
pub const SUGID_CLEAR_MODE_XFS: ::core::ffi::c_int = 5;
pub const SNAPSHOT_MODE_CAN_OVERWRITE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SNAPSHOT_MODE_CPLIKE_ATTR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SNAPSHOT_MODE_FORCE_REMOVAL: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SNAPSHOT_MODE_PRESERVE_HARDLINKS: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SNAPSHOT_MODE_DELETE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OPEN_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPEN_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPEN_AFTER_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPEN_TRUNCATE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPEN_DIRECTMODE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPEN_APPENDONLY: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MISSING_CHUNK_TYPE_NOCOPY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MISSING_CHUNK_TYPE_INVALID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MISSING_CHUNK_TYPE_WRONGVERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MISSING_CHUNK_TYPE_PARTIALEC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_XATTR_REMOVE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_XATTR_NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const MFS_XATTR_SIZE_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const FULL_DIRECTORY_ADD_CHUNKID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FULL_DIRECTORY_ADD_SYMLINK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FULL_DIRECTORY_ADD_XATTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FULL_DIRECTORY_ADD_FACL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FULL_DIRECTORY_ADD_EATTR: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SET_ALL_WINATTR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SET_ALL_EATTR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SET_ALL_XATTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SET_ALL_FACL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CHUNKOPFLAG_CANMODTIME: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHUNKOPFLAG_CONTINUEOP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHUNKOPFLAG_CANUSERESERVESPACE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ATIME_ALWAYS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ATIME_FILES_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ATIME_RELATIVE_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ATIME_FILES_AND_RELATIVE_ONLY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ATIME_NEVER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ATTR_RECORD_SIZE: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put64bit(mut ptr: *mut *mut uint8_t, mut val: uint64_t) {
    unsafe {
        val = val.swap_bytes() as uint64_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            8 as size_t,
        );
        *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    unsafe {
        val = val.swap_bytes() as uint32_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put16bit(mut ptr: *mut *mut uint8_t, mut val: uint16_t) {
    unsafe {
        val = val.swap_bytes() as uint16_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put8bit(mut ptr: *mut *mut uint8_t, mut val: uint8_t) {
    unsafe {
        *(*ptr).offset(0 as isize) =
            (val as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *ptr = (*ptr).offset(1);
    }
}
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
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn hash32(mut key: uint32_t) -> uint32_t {
    unsafe {
        key = (!key).wrapping_add(key << 15 as ::core::ffi::c_int);
        key = key ^ key >> 12 as ::core::ffi::c_int;
        key = key.wrapping_add(key << 2 as ::core::ffi::c_int);
        key = key ^ key >> 4 as ::core::ffi::c_int;
        key = key.wrapping_mul(2057 as uint32_t);
        key = key ^ key >> 16 as ::core::ffi::c_int;
        return key;
    }
}
pub const HASHTAB_LOBITS: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const HASHTAB_HISIZE: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint >> HASHTAB_LOBITS;
pub const HASHTAB_LOSIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HASHTAB_LOBITS;
pub const HASHTAB_MASK: ::core::ffi::c_int = HASHTAB_LOSIZE - 1 as ::core::ffi::c_int;
pub const HASHTAB_MOVEFACTOR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const DEFAULT_SCLASS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DEFAULT_TRASHTIME: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const MAXFNAMELENG: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const MAX_INDEX: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
pub const EDGEID_HASHSIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const ARCHREFTIME_NEVER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut quotahead: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
static mut QuotaDefaultGracePeriod: uint32_t = 0;
static mut MaxAllowedHardLinks: uint16_t = 0;
static mut AtimeMode: uint8_t = 0;
static mut KeepEmptyFilesInTrash: uint8_t = 0;
static mut InodeReuseDelay: uint32_t = 0;
static mut freebitmask: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
static mut bitmasksize: uint32_t = 0;
static mut searchpos: uint32_t = 0;
static mut freetail: *mut *mut freenode = ::core::ptr::null_mut::<*mut freenode>();
static mut freelist: *mut freenode = ::core::ptr::null_mut::<freenode>();
static mut freelastts: uint32_t = 0;
static mut trash_bid: uint32_t = 0;
static mut sustained_bid: uint32_t = 0;
static mut trash: [*mut fsedge; 4096] = [::core::ptr::null_mut::<fsedge>(); 4096];
static mut sustained: [*mut fsedge; 256] = [::core::ptr::null_mut::<fsedge>(); 256];
static mut root: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
static mut edgehashtab: [*mut *mut fsedge; 128] = [::core::ptr::null_mut::<*mut fsedge>(); 128];
static mut edgerehashpos: uint32_t = 0;
static mut edgehashsize: uint32_t = 0;
static mut edgehashelem: uint32_t = 0;
static mut nodehashtab: [*mut *mut fsnode; 128] = [::core::ptr::null_mut::<*mut fsnode>(); 128];
static mut noderehashpos: uint32_t = 0;
static mut nodehashsize: uint32_t = 0;
static mut nodehashelem: uint32_t = 0;
static mut hashelements: uint32_t = 0;
static mut maxnodeid: uint32_t = 0;
static mut nodes: uint32_t = 0;
static mut nextedgeid: uint64_t = 0;
static mut edgesneedrenumeration: uint8_t = 0;
static mut trashspace: uint64_t = 0;
static mut sustainedspace: uint64_t = 0;
static mut trashnodes: uint32_t = 0;
static mut sustainednodes: uint32_t = 0;
static mut filenodes: uint32_t = 0;
static mut dirnodes: uint32_t = 0;
static mut edgeid_id_hashtab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
static mut edgeid_ptr_hashtab: *mut *mut fsedge = ::core::ptr::null_mut::<*mut fsedge>();
static mut snapshot_inodehash: *mut ::core::ffi::c_void =
    ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MSGBUFFSIZE: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
static mut fsinfo_files: uint32_t = 0 as uint32_t;
static mut fsinfo_ugfiles: uint32_t = 0 as uint32_t;
static mut fsinfo_mfiles: uint32_t = 0 as uint32_t;
static mut fsinfo_mtfiles: uint32_t = 0 as uint32_t;
static mut fsinfo_msfiles: uint32_t = 0 as uint32_t;
static mut fsinfo_chunks: uint32_t = 0 as uint32_t;
static mut fsinfo_ugchunks: uint32_t = 0 as uint32_t;
static mut fsinfo_mchunks: uint32_t = 0 as uint32_t;
static mut fsinfo_msgbuff: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut fsinfo_msgbuffleng: uint32_t = 0 as uint32_t;
static mut fsinfo_loopstart: uint32_t = 0 as uint32_t;
static mut fsinfo_loopend: uint32_t = 0 as uint32_t;
static mut stats_statfs: uint32_t = 0 as uint32_t;
static mut stats_getattr: uint32_t = 0 as uint32_t;
static mut stats_setattr: uint32_t = 0 as uint32_t;
static mut stats_lookup: uint32_t = 0 as uint32_t;
static mut stats_mkdir: uint32_t = 0 as uint32_t;
static mut stats_rmdir: uint32_t = 0 as uint32_t;
static mut stats_symlink: uint32_t = 0 as uint32_t;
static mut stats_readlink: uint32_t = 0 as uint32_t;
static mut stats_mknod: uint32_t = 0 as uint32_t;
static mut stats_unlink: uint32_t = 0 as uint32_t;
static mut stats_rename: uint32_t = 0 as uint32_t;
static mut stats_link: uint32_t = 0 as uint32_t;
static mut stats_readdir: uint32_t = 0 as uint32_t;
static mut stats_open: uint32_t = 0 as uint32_t;
static mut stats_readchunk: uint32_t = 0 as uint32_t;
static mut stats_writechunk: uint32_t = 0 as uint32_t;
static mut stats_snapshot: uint32_t = 0 as uint32_t;
static mut stats_truncate: uint32_t = 0 as uint32_t;
static mut stats_getxattr: uint32_t = 0 as uint32_t;
static mut stats_setxattr: uint32_t = 0 as uint32_t;
static mut stats_getfacl: uint32_t = 0 as uint32_t;
static mut stats_setfacl: uint32_t = 0 as uint32_t;
static mut stats_create: uint32_t = 0 as uint32_t;
static mut stats_meta: uint32_t = 0 as uint32_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_stats(mut stats: *mut uint32_t) {
    unsafe {
        *stats.offset(0 as isize) = stats_statfs;
        *stats.offset(1 as isize) = stats_getattr;
        *stats.offset(2 as isize) = stats_setattr;
        *stats.offset(3 as isize) = stats_lookup;
        *stats.offset(4 as isize) = stats_mkdir;
        *stats.offset(5 as isize) = stats_rmdir;
        *stats.offset(6 as isize) = stats_symlink;
        *stats.offset(7 as isize) = stats_readlink;
        *stats.offset(8 as isize) = stats_mknod;
        *stats.offset(9 as isize) = stats_unlink;
        *stats.offset(10 as isize) = stats_rename;
        *stats.offset(11 as isize) = stats_link;
        *stats.offset(12 as isize) = stats_readdir;
        *stats.offset(13 as isize) = stats_open;
        *stats.offset(14 as isize) = stats_readchunk;
        *stats.offset(15 as isize) = stats_writechunk;
        *stats.offset(16 as isize) = stats_snapshot;
        *stats.offset(17 as isize) = stats_truncate;
        *stats.offset(18 as isize) = stats_getxattr;
        *stats.offset(19 as isize) = stats_setxattr;
        *stats.offset(20 as isize) = stats_getfacl;
        *stats.offset(21 as isize) = stats_setfacl;
        *stats.offset(22 as isize) = stats_create;
        *stats.offset(23 as isize) = stats_meta;
        stats_statfs = 0 as uint32_t;
        stats_getattr = 0 as uint32_t;
        stats_setattr = 0 as uint32_t;
        stats_lookup = 0 as uint32_t;
        stats_mkdir = 0 as uint32_t;
        stats_rmdir = 0 as uint32_t;
        stats_symlink = 0 as uint32_t;
        stats_readlink = 0 as uint32_t;
        stats_mknod = 0 as uint32_t;
        stats_unlink = 0 as uint32_t;
        stats_rename = 0 as uint32_t;
        stats_link = 0 as uint32_t;
        stats_readdir = 0 as uint32_t;
        stats_open = 0 as uint32_t;
        stats_readchunk = 0 as uint32_t;
        stats_writechunk = 0 as uint32_t;
        stats_snapshot = 0 as uint32_t;
        stats_truncate = 0 as uint32_t;
        stats_getxattr = 0 as uint32_t;
        stats_setxattr = 0 as uint32_t;
        stats_getfacl = 0 as uint32_t;
        stats_setfacl = 0 as uint32_t;
        stats_create = 0 as uint32_t;
        stats_meta = 0 as uint32_t;
    }
}
static mut freenode_allocated: uint64_t = 0 as uint64_t;
static mut freenode_used: uint64_t = 0 as uint64_t;
#[inline]
unsafe extern "C" fn freenode_free(mut p: *mut freenode) {
    unsafe {
        *(p as *mut *mut ::core::ffi::c_void) = freenode_free_head;
        freenode_free_head = p as *mut ::core::ffi::c_void;
        freenode_used = (freenode_used as ::core::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<freenode>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn freenode_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = freenode_allocated;
        *used = freenode_used;
    }
}
static mut freenode_free_head: *mut ::core::ffi::c_void = NULL;
static mut freenode_buckets_head: *mut freenode_bucket = ::core::ptr::null_mut::<freenode_bucket>();
#[inline]
unsafe extern "C" fn freenode_free_all() {
    unsafe {
        let mut srb: *mut freenode_bucket = ::core::ptr::null_mut::<freenode_bucket>();
        let mut nsrb: *mut freenode_bucket = ::core::ptr::null_mut::<freenode_bucket>();
        srb = freenode_buckets_head;
        while !srb.is_null() {
            nsrb = (*srb).next as *mut freenode_bucket;
            munmap(
                srb as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<freenode_bucket>(),
            );
            srb = nsrb;
        }
        freenode_buckets_head = ::core::ptr::null_mut::<freenode_bucket>();
        freenode_free_head = NULL;
        freenode_allocated = 0 as uint64_t;
        freenode_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn freenode_malloc() -> *mut freenode {
    unsafe {
        let mut srb: *mut freenode_bucket = ::core::ptr::null_mut::<freenode_bucket>();
        let mut ret: *mut freenode = ::core::ptr::null_mut::<freenode>();
        if !freenode_free_head.is_null() {
            ret = freenode_free_head as *mut freenode;
            freenode_free_head = *(ret as *mut *mut ::core::ffi::c_void);
            freenode_used = (freenode_used as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<freenode>() as ::core::ffi::c_ulong)
                as uint64_t;
            return ret;
        }
        if freenode_buckets_head.is_null()
            || (*freenode_buckets_head).firstfree as usize
                == (10000000 as ::core::ffi::c_int as usize)
                    .wrapping_div(::core::mem::size_of::<freenode>())
        {
            srb = mmap(
                NULL,
                ::core::mem::size_of::<freenode_bucket>(),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut freenode_bucket;
            if srb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if srb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut freenode_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*srb).next = freenode_buckets_head as *mut _freenode_bucket;
            (*srb).firstfree = 0 as uint32_t;
            freenode_buckets_head = srb;
            freenode_allocated = (freenode_allocated as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<freenode_bucket>() as ::core::ffi::c_ulong)
                as uint64_t;
        }
        ret = (&raw mut (*freenode_buckets_head).bucket as *mut freenode)
            .offset((*freenode_buckets_head).firstfree as isize);
        (*freenode_buckets_head).firstfree = (*freenode_buckets_head).firstfree.wrapping_add(1);
        freenode_used = (freenode_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<freenode>() as ::core::ffi::c_ulong)
            as uint64_t;
        return ret;
    }
}
static mut quotanode_used: uint64_t = 0 as uint64_t;
#[inline]
unsafe extern "C" fn quotanode_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = quotanode_allocated;
        *used = quotanode_used;
    }
}
#[inline]
unsafe extern "C" fn quotanode_free_all() {
    unsafe {
        let mut srb: *mut quotanode_bucket = ::core::ptr::null_mut::<quotanode_bucket>();
        let mut nsrb: *mut quotanode_bucket = ::core::ptr::null_mut::<quotanode_bucket>();
        srb = quotanode_buckets_head;
        while !srb.is_null() {
            nsrb = (*srb).next as *mut quotanode_bucket;
            munmap(
                srb as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<quotanode_bucket>(),
            );
            srb = nsrb;
        }
        quotanode_buckets_head = ::core::ptr::null_mut::<quotanode_bucket>();
        quotanode_free_head = NULL;
        quotanode_allocated = 0 as uint64_t;
        quotanode_used = 0 as uint64_t;
    }
}
static mut quotanode_free_head: *mut ::core::ffi::c_void = NULL;
#[inline]
unsafe extern "C" fn quotanode_free(mut p: *mut quotanode) {
    unsafe {
        *(p as *mut *mut ::core::ffi::c_void) = quotanode_free_head;
        quotanode_free_head = p as *mut ::core::ffi::c_void;
        quotanode_used = (quotanode_used as ::core::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<quotanode>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
}
static mut quotanode_allocated: uint64_t = 0 as uint64_t;
static mut quotanode_buckets_head: *mut quotanode_bucket =
    ::core::ptr::null_mut::<quotanode_bucket>();
#[inline]
unsafe extern "C" fn quotanode_malloc() -> *mut quotanode {
    unsafe {
        let mut srb: *mut quotanode_bucket = ::core::ptr::null_mut::<quotanode_bucket>();
        let mut ret: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        if !quotanode_free_head.is_null() {
            ret = quotanode_free_head as *mut quotanode;
            quotanode_free_head = *(ret as *mut *mut ::core::ffi::c_void);
            quotanode_used = (quotanode_used as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<quotanode>() as ::core::ffi::c_ulong)
                as uint64_t;
            return ret;
        }
        if quotanode_buckets_head.is_null()
            || (*quotanode_buckets_head).firstfree as usize
                == (10000000 as ::core::ffi::c_int as usize)
                    .wrapping_div(::core::mem::size_of::<quotanode>())
        {
            srb = mmap(
                NULL,
                ::core::mem::size_of::<quotanode_bucket>(),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut quotanode_bucket;
            if srb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if srb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut quotanode_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*srb).next = quotanode_buckets_head as *mut _quotanode_bucket;
            (*srb).firstfree = 0 as uint32_t;
            quotanode_buckets_head = srb;
            quotanode_allocated = (quotanode_allocated as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<quotanode_bucket>() as ::core::ffi::c_ulong)
                as uint64_t;
        }
        ret = (&raw mut (*quotanode_buckets_head).bucket as *mut quotanode)
            .offset((*quotanode_buckets_head).firstfree as isize);
        (*quotanode_buckets_head).firstfree = (*quotanode_buckets_head).firstfree.wrapping_add(1);
        quotanode_used = (quotanode_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<quotanode>() as ::core::ffi::c_ulong)
            as uint64_t;
        return ret;
    }
}
pub const NODE_BUCKET_SIZE: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const NODE_MAX_INDX: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
static mut nrbheads: [*mut fsnode_bucket; 5] = [::core::ptr::null_mut::<fsnode_bucket>(); 5];
static mut nrbfreeheads: [*mut fsnode; 5] = [::core::ptr::null_mut::<fsnode>(); 5];
static mut nrbucketsize: [uint32_t; 5] = [0; 5];
static mut nrelemsize: [uint32_t; 5] = [0; 5];
static mut fsnode_allocated: uint64_t = 0;
static mut fsnode_used: uint64_t = 0;
#[inline]
unsafe extern "C" fn fsnode_init() {
    unsafe {
        let mut i: uint32_t = 0;
        nrelemsize[0 as usize] =
            (48 as ::core::ffi::c_ulong).wrapping_add(64 as ::core::ffi::c_ulong) as uint32_t;
        nrelemsize[1 as usize] =
            (48 as ::core::ffi::c_ulong).wrapping_add(23 as ::core::ffi::c_ulong) as uint32_t;
        nrelemsize[2 as usize] =
            (48 as ::core::ffi::c_ulong).wrapping_add(14 as ::core::ffi::c_ulong) as uint32_t;
        nrelemsize[3 as usize] =
            (48 as ::core::ffi::c_ulong).wrapping_add(6 as ::core::ffi::c_ulong) as uint32_t;
        nrelemsize[4 as usize] =
            (48 as ::core::ffi::c_ulong).wrapping_add(2 as ::core::ffi::c_ulong) as uint32_t;
        i = 0 as uint32_t;
        while i < NODE_MAX_INDX as uint32_t {
            nrbheads[i as usize] = ::core::ptr::null_mut::<fsnode_bucket>();
            nrbfreeheads[i as usize] = ::core::ptr::null_mut::<fsnode>();
            nrbucketsize[i as usize] = (NODE_BUCKET_SIZE as uint32_t)
                .wrapping_div(nrelemsize[i as usize])
                .wrapping_mul(nrelemsize[i as usize]);
            i = i.wrapping_add(1);
        }
        fsnode_allocated = 0 as uint64_t;
        fsnode_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn fsnode_cleanup() {
    unsafe {
        let mut nrb: *mut fsnode_bucket = ::core::ptr::null_mut::<fsnode_bucket>();
        let mut nnrb: *mut fsnode_bucket = ::core::ptr::null_mut::<fsnode_bucket>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < NODE_MAX_INDX as uint32_t {
            nrb = nrbheads[i as usize];
            while !nrb.is_null() {
                nnrb = (*nrb).next as *mut fsnode_bucket;
                munmap(
                    nrb as *mut ::core::ffi::c_void,
                    (16 as size_t).wrapping_add(nrbucketsize[i as usize] as size_t),
                );
                nrb = nnrb;
            }
            nrbheads[i as usize] = ::core::ptr::null_mut::<fsnode_bucket>();
            nrbfreeheads[i as usize] = ::core::ptr::null_mut::<fsnode>();
            i = i.wrapping_add(1);
        }
        fsnode_allocated = 0 as uint64_t;
        fsnode_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn fsnode_malloc(mut indx: uint8_t) -> *mut fsnode {
    unsafe {
        let mut nrb: *mut fsnode_bucket = ::core::ptr::null_mut::<fsnode_bucket>();
        let mut ret: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if (indx as ::core::ffi::c_int) < 5 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                413 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"indx<NODE_MAX_INDX\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                413 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"indx<NODE_MAX_INDX\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        if !nrbfreeheads[indx as usize].is_null() {
            ret = nrbfreeheads[indx as usize];
            nrbfreeheads[indx as usize] = (*ret).next as *mut fsnode;
            fsnode_used = fsnode_used.wrapping_add(nrelemsize[indx as usize] as uint64_t);
            return ret;
        }
        if nrbheads[indx as usize].is_null()
            || (*nrbheads[indx as usize])
                .firstfree
                .wrapping_add(nrelemsize[indx as usize])
                > nrbucketsize[indx as usize]
        {
            nrb = mmap(
                NULL,
                (16 as size_t).wrapping_add(nrbucketsize[indx as usize] as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut fsnode_bucket;
            if nrb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    426 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nrb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    426 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nrb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if nrb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut fsnode_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    426 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nrb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    426 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nrb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*nrb).next = nrbheads[indx as usize] as *mut _fsnode_bucket;
            (*nrb).firstfree = 0 as uint32_t;
            nrbheads[indx as usize] = nrb;
            fsnode_allocated = (fsnode_allocated as ::core::ffi::c_ulong).wrapping_add(
                (16 as ::core::ffi::c_ulong)
                    .wrapping_add(nrbucketsize[indx as usize] as ::core::ffi::c_ulong),
            ) as uint64_t;
        }
        ret = (&raw mut (**(&raw mut nrbheads as *mut *mut fsnode_bucket).offset(indx as isize))
            .bucket as *mut uint8_t)
            .offset((*nrbheads[indx as usize]).firstfree as isize) as *mut fsnode;
        (*nrbheads[indx as usize]).firstfree = (*nrbheads[indx as usize])
            .firstfree
            .wrapping_add(nrelemsize[indx as usize]);
        fsnode_used = fsnode_used.wrapping_add(nrelemsize[indx as usize] as uint64_t);
        return ret;
    }
}
#[inline]
unsafe extern "C" fn fsnode_free(mut n: *mut fsnode, mut indx: uint8_t) {
    unsafe {
        (*n).next = nrbfreeheads[indx as usize] as *mut _fsnode;
        nrbfreeheads[indx as usize] = n;
        fsnode_used = fsnode_used.wrapping_sub(nrelemsize[indx as usize] as uint64_t);
    }
}
#[inline]
unsafe extern "C" fn fsnode_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = fsnode_allocated;
        *used = fsnode_used;
    }
}
pub const EDGE_BUCKET_SIZE: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const EDGE_MAX_INDX: ::core::ffi::c_int = MFS_PATH_MAX / 8 as ::core::ffi::c_int;
static mut erbheads: [*mut fsedge_bucket; 128] = [::core::ptr::null_mut::<fsedge_bucket>(); 128];
static mut erbfreeheads: [*mut fsedge; 128] = [::core::ptr::null_mut::<fsedge>(); 128];
static mut erbucketsize: [uint32_t; 128] = [0; 128];
static mut fsedge_allocated: uint64_t = 0;
static mut fsedge_used: uint64_t = 0;
#[inline]
unsafe extern "C" fn fsedge_init() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut recsize: uint32_t = 0;
        i = 0 as uint32_t;
        while i < EDGE_MAX_INDX as uint32_t {
            erbheads[i as usize] = ::core::ptr::null_mut::<fsedge_bucket>();
            erbfreeheads[i as usize] = ::core::ptr::null_mut::<fsedge>();
            recsize = (i.wrapping_add(1 as uint32_t).wrapping_mul(8 as uint32_t)
                as ::core::ffi::c_ulong)
                .wrapping_add(
                    (70 as ::core::ffi::c_ulong).wrapping_add(7 as ::core::ffi::c_ulong)
                        & 0xfffffff8 as ::core::ffi::c_ulong,
                ) as uint32_t;
            erbucketsize[i as usize] = (EDGE_BUCKET_SIZE as uint32_t)
                .wrapping_div(recsize)
                .wrapping_mul(recsize);
            i = i.wrapping_add(1);
        }
        fsedge_allocated = 0 as uint64_t;
        fsedge_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn fsedge_cleanup() {
    unsafe {
        let mut erb: *mut fsedge_bucket = ::core::ptr::null_mut::<fsedge_bucket>();
        let mut nerb: *mut fsedge_bucket = ::core::ptr::null_mut::<fsedge_bucket>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < EDGE_MAX_INDX as uint32_t {
            erb = erbheads[i as usize];
            while !erb.is_null() {
                nerb = (*erb).next as *mut fsedge_bucket;
                munmap(
                    erb as *mut ::core::ffi::c_void,
                    (16 as size_t).wrapping_add(erbucketsize[i as usize] as size_t),
                );
                erb = nerb;
            }
            erbheads[i as usize] = ::core::ptr::null_mut::<fsedge_bucket>();
            erbfreeheads[i as usize] = ::core::ptr::null_mut::<fsedge>();
            i = i.wrapping_add(1);
        }
        fsedge_allocated = 0 as uint64_t;
        fsedge_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn fsedge_malloc(mut nleng: uint16_t) -> *mut fsedge {
    unsafe {
        let mut erb: *mut fsedge_bucket = ::core::ptr::null_mut::<fsedge_bucket>();
        let mut ret: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut indx: uint16_t = ((nleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as uint16_t;
        if (indx as ::core::ffi::c_int) < 1024 as ::core::ffi::c_int / 8 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                508 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"indx<EDGE_MAX_INDX\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                508 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"indx<EDGE_MAX_INDX\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        if !erbfreeheads[indx as usize].is_null() {
            ret = erbfreeheads[indx as usize];
            erbfreeheads[indx as usize] = (*ret).next as *mut fsedge;
            fsedge_used = (fsedge_used as ::core::ffi::c_ulong).wrapping_add(
                (((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_ulong)
                    .wrapping_add(
                        (70 as ::core::ffi::c_ulong).wrapping_add(7 as ::core::ffi::c_ulong)
                            & 0xfffffff8 as ::core::ffi::c_ulong,
                    ),
            ) as uint64_t;
            return ret;
        }
        if erbheads[indx as usize].is_null()
            || ((*erbheads[indx as usize]).firstfree as ::core::ffi::c_ulong)
                .wrapping_add(
                    (((nleng as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        * 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong)
                        .wrapping_add(
                            (70 as ::core::ffi::c_ulong)
                                .wrapping_add(7 as ::core::ffi::c_ulong)
                                & 0xfffffff8 as ::core::ffi::c_ulong,
                        ),
                ) > erbucketsize[indx as usize] as ::core::ffi::c_ulong
        {
            erb = mmap(
                NULL,
                (16 as size_t).wrapping_add(erbucketsize[indx as usize] as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut fsedge_bucket;
            if erb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"erb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"erb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if erb
                == ::core::ptr::with_exposed_provenance_mut::<
                    ::core::ffi::c_void,
                >(-1 as ::core::ffi::c_int as usize) as *mut fsedge_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(
                    *__errno_location(),
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"erb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"erb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*erb).next = erbheads[indx as usize] as *mut _fsedge_bucket;
            (*erb).firstfree = 0 as uint32_t;
            erbheads[indx as usize] = erb;
            fsedge_allocated = (fsedge_allocated as ::core::ffi::c_ulong)
                .wrapping_add(
                    (16 as ::core::ffi::c_ulong)
                        .wrapping_add(
                            erbucketsize[indx as usize] as ::core::ffi::c_ulong,
                        ),
                ) as uint64_t;
        }
        ret = (&raw mut (**(&raw mut erbheads as *mut *mut fsedge_bucket).offset(indx as isize))
            .bucket as *mut uint8_t)
            .offset((*erbheads[indx as usize]).firstfree as isize) as *mut fsedge;
        (*erbheads[indx as usize]).firstfree =
            ((*erbheads[indx as usize]).firstfree as ::core::ffi::c_ulong).wrapping_add(
                (((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_ulong)
                    .wrapping_add(
                        (70 as ::core::ffi::c_ulong).wrapping_add(7 as ::core::ffi::c_ulong)
                            & 0xfffffff8 as ::core::ffi::c_ulong,
                    ),
            ) as uint32_t;
        fsedge_used = (fsedge_used as ::core::ffi::c_ulong).wrapping_add(
            (((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                as ::core::ffi::c_ulong)
                .wrapping_add(
                    (70 as ::core::ffi::c_ulong).wrapping_add(7 as ::core::ffi::c_ulong)
                        & 0xfffffff8 as ::core::ffi::c_ulong,
                ),
        ) as uint64_t;
        return ret;
    }
}
#[inline]
unsafe extern "C" fn fsedge_free(mut e: *mut fsedge, mut nleng: uint16_t) {
    unsafe {
        let mut indx: uint16_t = ((nleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as uint16_t;
        (*e).next = erbfreeheads[indx as usize] as *mut _fsedge;
        erbfreeheads[indx as usize] = e;
        fsedge_used = (fsedge_used as ::core::ffi::c_ulong).wrapping_sub(
            (((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                as ::core::ffi::c_ulong)
                .wrapping_add(
                    (70 as ::core::ffi::c_ulong).wrapping_add(7 as ::core::ffi::c_ulong)
                        & 0xfffffff8 as ::core::ffi::c_ulong,
                ),
        ) as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn fsedge_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = fsedge_allocated;
        *used = fsedge_used;
    }
}
pub const SYMLINK_BUCKET_SIZE: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const SYMLINK_MAX_INDX: ::core::ffi::c_int = MFS_SYMLINK_MAX / 8 as ::core::ffi::c_int;
static mut stbheads: [*mut symlink_bucket; 512] = [::core::ptr::null_mut::<symlink_bucket>(); 512];
static mut stbfreeheads: [*mut uint8_t; 512] = [::core::ptr::null_mut::<uint8_t>(); 512];
static mut stbucketsize: [uint32_t; 512] = [0; 512];
static mut symlink_allocated: uint64_t = 0;
static mut symlink_used: uint64_t = 0;
#[inline]
unsafe extern "C" fn symlink_init() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut recsize: uint32_t = 0;
        i = 0 as uint32_t;
        while i < SYMLINK_MAX_INDX as uint32_t {
            stbheads[i as usize] = ::core::ptr::null_mut::<symlink_bucket>();
            stbfreeheads[i as usize] = ::core::ptr::null_mut::<uint8_t>();
            recsize = i.wrapping_add(1 as uint32_t).wrapping_mul(8 as uint32_t);
            stbucketsize[i as usize] = (SYMLINK_BUCKET_SIZE as uint32_t)
                .wrapping_div(recsize)
                .wrapping_mul(recsize);
            i = i.wrapping_add(1);
        }
        symlink_allocated = 0 as uint64_t;
        symlink_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn symlink_cleanup() {
    unsafe {
        let mut stb: *mut symlink_bucket = ::core::ptr::null_mut::<symlink_bucket>();
        let mut nstb: *mut symlink_bucket = ::core::ptr::null_mut::<symlink_bucket>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < SYMLINK_MAX_INDX as uint32_t {
            stb = stbheads[i as usize];
            while !stb.is_null() {
                nstb = (*stb).next as *mut symlink_bucket;
                munmap(
                    stb as *mut ::core::ffi::c_void,
                    (16 as size_t).wrapping_add(stbucketsize[i as usize] as size_t),
                );
                stb = nstb;
            }
            stbheads[i as usize] = ::core::ptr::null_mut::<symlink_bucket>();
            stbfreeheads[i as usize] = ::core::ptr::null_mut::<uint8_t>();
            i = i.wrapping_add(1);
        }
        symlink_allocated = 0 as uint64_t;
        symlink_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn symlink_malloc(mut pathleng: uint16_t) -> *mut uint8_t {
    unsafe {
        let mut stb: *mut symlink_bucket = ::core::ptr::null_mut::<symlink_bucket>();
        let mut ret: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut indx: uint16_t = ((pathleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as uint16_t;
        if (indx as ::core::ffi::c_int) < 4096 as ::core::ffi::c_int / 8 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                604 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"indx<SYMLINK_MAX_INDX\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                604 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"indx<SYMLINK_MAX_INDX\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        if !stbfreeheads[indx as usize].is_null() {
            ret = stbfreeheads[indx as usize];
            stbfreeheads[indx as usize] = *(ret as *mut *mut uint8_t);
            symlink_used = symlink_used.wrapping_add(
                ((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                    as uint64_t,
            );
            return ret;
        }
        if stbheads[indx as usize].is_null()
            || (*stbheads[indx as usize]).firstfree.wrapping_add(
                ((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                    as uint32_t,
            ) > stbucketsize[indx as usize]
        {
            stb = mmap(
                NULL,
                (16 as size_t).wrapping_add(stbucketsize[indx as usize] as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut symlink_bucket;
            if stb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    617 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"stb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    617 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"stb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if stb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut symlink_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    617 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"stb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    617 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"stb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*stb).next = stbheads[indx as usize] as *mut _symlink_bucket;
            (*stb).firstfree = 0 as uint32_t;
            stbheads[indx as usize] = stb;
            symlink_allocated = (symlink_allocated as ::core::ffi::c_ulong).wrapping_add(
                (16 as ::core::ffi::c_ulong)
                    .wrapping_add(stbucketsize[indx as usize] as ::core::ffi::c_ulong),
            ) as uint64_t;
        }
        ret = (&raw mut (**(&raw mut stbheads as *mut *mut symlink_bucket).offset(indx as isize))
            .bucket as *mut uint8_t)
            .offset((*stbheads[indx as usize]).firstfree as isize);
        (*stbheads[indx as usize]).firstfree = (*stbheads[indx as usize]).firstfree.wrapping_add(
            ((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                as uint32_t,
        );
        symlink_used = symlink_used.wrapping_add(
            ((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                as uint64_t,
        );
        return ret;
    }
}
#[inline]
unsafe extern "C" fn symlink_free(mut p: *mut uint8_t, mut pathleng: uint16_t) {
    unsafe {
        let mut indx: uint16_t = ((pathleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as uint16_t;
        *(p as *mut *mut uint8_t) = stbfreeheads[indx as usize];
        stbfreeheads[indx as usize] = p;
        symlink_used = symlink_used.wrapping_sub(
            ((indx as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                as uint64_t,
        );
    }
}
#[inline]
unsafe extern "C" fn symlink_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = symlink_allocated;
        *used = symlink_used;
    }
}
pub const CHUNKTAB_BUCKET_SIZE: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const CHUNKTAB_MAX_INDX: ::core::ffi::c_int = 121 as ::core::ffi::c_int;
static mut ctbheads: [*mut chunktab_bucket; 121] =
    [::core::ptr::null_mut::<chunktab_bucket>(); 121];
static mut ctbfreeheads: [*mut uint64_t; 121] = [::core::ptr::null_mut::<uint64_t>(); 121];
static mut ctbucketsize: [uint64_t; 121] = [0; 121];
static mut chunktabsize: [uint64_t; 121] = [0; 121];
static mut chunktab_allocated: uint64_t = 0;
static mut chunktab_used: uint64_t = 0;
#[inline]
unsafe extern "C" fn chunktab_init() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < CHUNKTAB_MAX_INDX as uint32_t {
            ctbheads[i as usize] = ::core::ptr::null_mut::<chunktab_bucket>();
            ctbfreeheads[i as usize] = ::core::ptr::null_mut::<uint64_t>();
            chunktabsize[i as usize] = (if i < 0x10 as uint32_t {
                i.wrapping_add(1 as uint32_t) as usize
            } else {
                (if i < 0x1f as uint32_t {
                    i.wrapping_sub(0xe as uint32_t)
                        .wrapping_mul(0x10 as uint32_t) as usize
                } else {
                    (if i < 0x2e as uint32_t {
                        i.wrapping_sub(0x1d as uint32_t)
                            .wrapping_mul(0x100 as uint32_t) as usize
                    } else {
                        (if i < 0x3d as uint32_t {
                            i.wrapping_sub(0x2c as uint32_t)
                                .wrapping_mul(0x1000 as uint32_t)
                                as usize
                        } else {
                            (if i < 0x4c as uint32_t {
                                i.wrapping_sub(0x3b as uint32_t)
                                    .wrapping_mul(0x10000 as uint32_t)
                                    as usize
                            } else {
                                (if i < 0x5b as uint32_t {
                                    i.wrapping_sub(0x4a as uint32_t)
                                        .wrapping_mul(0x100000 as uint32_t)
                                        as usize
                                } else {
                                    (if i < 0x6a as uint32_t {
                                        i.wrapping_sub(0x59 as uint32_t)
                                            .wrapping_mul(0x1000000 as uint32_t)
                                            as usize
                                    } else {
                                        (i.wrapping_sub(0x68 as uint32_t) as usize)
                                            .wrapping_mul(0x10000000 as usize)
                                    })
                                })
                            })
                        })
                    })
                })
            })
            .wrapping_mul(::core::mem::size_of::<uint64_t>())
                as uint64_t;
            ctbucketsize[i as usize] = (CHUNKTAB_BUCKET_SIZE as uint64_t)
                .wrapping_div(chunktabsize[i as usize])
                .wrapping_add(1 as uint64_t)
                .wrapping_mul(chunktabsize[i as usize]);
            i = i.wrapping_add(1);
        }
        chunktab_allocated = 0 as uint64_t;
        chunktab_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn chunktab_cleanup() {
    unsafe {
        let mut ctb: *mut chunktab_bucket = ::core::ptr::null_mut::<chunktab_bucket>();
        let mut nctb: *mut chunktab_bucket = ::core::ptr::null_mut::<chunktab_bucket>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < CHUNKTAB_MAX_INDX as uint32_t {
            ctb = ctbheads[i as usize];
            while !ctb.is_null() {
                nctb = (*ctb).next as *mut chunktab_bucket;
                munmap(
                    ctb as *mut ::core::ffi::c_void,
                    (16 as size_t).wrapping_add(ctbucketsize[i as usize] as size_t),
                );
                ctb = nctb;
            }
            ctbheads[i as usize] = ::core::ptr::null_mut::<chunktab_bucket>();
            ctbfreeheads[i as usize] = ::core::ptr::null_mut::<uint64_t>();
            i = i.wrapping_add(1);
        }
        chunktab_allocated = 0 as uint64_t;
        chunktab_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn chunktab_indx_malloc(mut indx: uint8_t) -> *mut uint64_t {
    unsafe {
        let mut ctb: *mut chunktab_bucket = ::core::ptr::null_mut::<chunktab_bucket>();
        let mut ret: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        if !ctbfreeheads[indx as usize].is_null() {
            ret = ctbfreeheads[indx as usize];
            ctbfreeheads[indx as usize] = *(ret as *mut *mut uint64_t);
            return ret;
        }
        if ctbheads[indx as usize].is_null()
            || (*ctbheads[indx as usize])
                .firstfree
                .wrapping_add(chunktabsize[indx as usize])
                > ctbucketsize[indx as usize]
        {
            ctb = mmap(
                NULL,
                (16 as size_t).wrapping_add(ctbucketsize[indx as usize] as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut chunktab_bucket;
            if ctb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    711 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ctb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    711 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ctb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if ctb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut chunktab_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    711 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ctb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    711 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ctb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*ctb).next = ctbheads[indx as usize] as *mut _chunktab_bucket;
            (*ctb).firstfree = 0 as uint64_t;
            ctbheads[indx as usize] = ctb;
            chunktab_allocated = (chunktab_allocated as ::core::ffi::c_ulong)
                .wrapping_add((16 as uint64_t).wrapping_add(ctbucketsize[indx as usize])
                    as ::core::ffi::c_ulong) as uint64_t;
        }
        ret = (&raw mut (**(&raw mut ctbheads as *mut *mut chunktab_bucket).offset(indx as isize))
            .bucket as *mut uint8_t)
            .offset((*ctbheads[indx as usize]).firstfree as isize) as *mut uint64_t;
        (*ctbheads[indx as usize]).firstfree = (*ctbheads[indx as usize])
            .firstfree
            .wrapping_add(chunktabsize[indx as usize]);
        return ret;
    }
}
#[inline]
unsafe extern "C" fn chunktab_indx_free(mut chunktab: *mut uint64_t, mut indx: uint8_t) {
    unsafe {
        *(chunktab as *mut *mut uint64_t) = ctbfreeheads[indx as usize];
        ctbfreeheads[indx as usize] = chunktab;
    }
}
#[inline]
unsafe extern "C" fn chunktab_malloc(mut chunks: uint32_t) -> *mut uint64_t {
    unsafe {
        let mut indx: uint8_t = (if chunks <= 0x10 as uint32_t {
            chunks.wrapping_sub(1 as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x100 as uint32_t {
            chunks
                .wrapping_add(0xf as uint32_t)
                .wrapping_div(0x10 as uint32_t)
                .wrapping_add(0xe as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x1000 as uint32_t {
            chunks
                .wrapping_add(0xff as uint32_t)
                .wrapping_div(0x100 as uint32_t)
                .wrapping_add(0x1d as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x10000 as uint32_t {
            chunks
                .wrapping_add(0xfff as uint32_t)
                .wrapping_div(0x1000 as uint32_t)
                .wrapping_add(0x2c as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x100000 as uint32_t {
            chunks
                .wrapping_add(0xffff as uint32_t)
                .wrapping_div(0x10000 as uint32_t)
                .wrapping_add(0x3b as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x1000000 as uint32_t {
            chunks
                .wrapping_add(0xfffff as uint32_t)
                .wrapping_div(0x100000 as uint32_t)
                .wrapping_add(0x4a as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x10000000 as uint32_t {
            chunks
                .wrapping_add(0xffffff as uint32_t)
                .wrapping_div(0x1000000 as uint32_t)
                .wrapping_add(0x59 as uint32_t) as ::core::ffi::c_ulong
        } else {
            (chunks as ::core::ffi::c_ulong)
                .wrapping_add(0xfffffff as ::core::ffi::c_ulong)
                .wrapping_div(0x10000000 as ::core::ffi::c_ulong)
                .wrapping_add(0x68 as ::core::ffi::c_ulong)
        }) as uint8_t;
        if chunks == 0 as uint32_t || indx as ::core::ffi::c_int >= CHUNKTAB_MAX_INDX {
            return ::core::ptr::null_mut::<uint64_t>();
        }
        chunktab_used = (chunktab_used as ::core::ffi::c_ulong).wrapping_add(
            (chunks as usize).wrapping_mul(::core::mem::size_of::<uint64_t>())
                as ::core::ffi::c_ulong,
        ) as uint64_t;
        return chunktab_indx_malloc(indx);
    }
}
#[inline]
unsafe extern "C" fn chunktab_free(mut chunktab: *mut uint64_t, mut chunks: uint32_t) {
    unsafe {
        let mut indx: uint8_t = (if chunks <= 0x10 as uint32_t {
            chunks.wrapping_sub(1 as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x100 as uint32_t {
            chunks
                .wrapping_add(0xf as uint32_t)
                .wrapping_div(0x10 as uint32_t)
                .wrapping_add(0xe as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x1000 as uint32_t {
            chunks
                .wrapping_add(0xff as uint32_t)
                .wrapping_div(0x100 as uint32_t)
                .wrapping_add(0x1d as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x10000 as uint32_t {
            chunks
                .wrapping_add(0xfff as uint32_t)
                .wrapping_div(0x1000 as uint32_t)
                .wrapping_add(0x2c as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x100000 as uint32_t {
            chunks
                .wrapping_add(0xffff as uint32_t)
                .wrapping_div(0x10000 as uint32_t)
                .wrapping_add(0x3b as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x1000000 as uint32_t {
            chunks
                .wrapping_add(0xfffff as uint32_t)
                .wrapping_div(0x100000 as uint32_t)
                .wrapping_add(0x4a as uint32_t) as ::core::ffi::c_ulong
        } else if chunks <= 0x10000000 as uint32_t {
            chunks
                .wrapping_add(0xffffff as uint32_t)
                .wrapping_div(0x1000000 as uint32_t)
                .wrapping_add(0x59 as uint32_t) as ::core::ffi::c_ulong
        } else {
            (chunks as ::core::ffi::c_ulong)
                .wrapping_add(0xfffffff as ::core::ffi::c_ulong)
                .wrapping_div(0x10000000 as ::core::ffi::c_ulong)
                .wrapping_add(0x68 as ::core::ffi::c_ulong)
        }) as uint8_t;
        if chunks == 0 as uint32_t || indx as ::core::ffi::c_int >= CHUNKTAB_MAX_INDX {
            return;
        }
        chunktab_used = (chunktab_used as ::core::ffi::c_ulong).wrapping_sub(
            (chunks as usize).wrapping_mul(::core::mem::size_of::<uint64_t>())
                as ::core::ffi::c_ulong,
        ) as uint64_t;
        chunktab_indx_free(chunktab, indx);
    }
}
#[inline]
unsafe extern "C" fn chunktab_realloc(
    mut oldchunktab: *mut uint64_t,
    mut oldchunks: uint32_t,
    mut newchunks: uint32_t,
) -> *mut uint64_t {
    unsafe {
        let mut newchunktab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut oldindx: uint8_t = 0;
        let mut newindx: uint8_t = 0;
        oldindx = (if oldchunks <= 0x10 as uint32_t {
            oldchunks.wrapping_sub(1 as uint32_t) as ::core::ffi::c_ulong
        } else if oldchunks <= 0x100 as uint32_t {
            oldchunks
                .wrapping_add(0xf as uint32_t)
                .wrapping_div(0x10 as uint32_t)
                .wrapping_add(0xe as uint32_t) as ::core::ffi::c_ulong
        } else if oldchunks <= 0x1000 as uint32_t {
            oldchunks
                .wrapping_add(0xff as uint32_t)
                .wrapping_div(0x100 as uint32_t)
                .wrapping_add(0x1d as uint32_t) as ::core::ffi::c_ulong
        } else if oldchunks <= 0x10000 as uint32_t {
            oldchunks
                .wrapping_add(0xfff as uint32_t)
                .wrapping_div(0x1000 as uint32_t)
                .wrapping_add(0x2c as uint32_t) as ::core::ffi::c_ulong
        } else if oldchunks <= 0x100000 as uint32_t {
            oldchunks
                .wrapping_add(0xffff as uint32_t)
                .wrapping_div(0x10000 as uint32_t)
                .wrapping_add(0x3b as uint32_t) as ::core::ffi::c_ulong
        } else if oldchunks <= 0x1000000 as uint32_t {
            oldchunks
                .wrapping_add(0xfffff as uint32_t)
                .wrapping_div(0x100000 as uint32_t)
                .wrapping_add(0x4a as uint32_t) as ::core::ffi::c_ulong
        } else if oldchunks <= 0x10000000 as uint32_t {
            oldchunks
                .wrapping_add(0xffffff as uint32_t)
                .wrapping_div(0x1000000 as uint32_t)
                .wrapping_add(0x59 as uint32_t) as ::core::ffi::c_ulong
        } else {
            (oldchunks as ::core::ffi::c_ulong)
                .wrapping_add(0xfffffff as ::core::ffi::c_ulong)
                .wrapping_div(0x10000000 as ::core::ffi::c_ulong)
                .wrapping_add(0x68 as ::core::ffi::c_ulong)
        }) as uint8_t;
        newindx = (if newchunks <= 0x10 as uint32_t {
            newchunks.wrapping_sub(1 as uint32_t) as ::core::ffi::c_ulong
        } else if newchunks <= 0x100 as uint32_t {
            newchunks
                .wrapping_add(0xf as uint32_t)
                .wrapping_div(0x10 as uint32_t)
                .wrapping_add(0xe as uint32_t) as ::core::ffi::c_ulong
        } else if newchunks <= 0x1000 as uint32_t {
            newchunks
                .wrapping_add(0xff as uint32_t)
                .wrapping_div(0x100 as uint32_t)
                .wrapping_add(0x1d as uint32_t) as ::core::ffi::c_ulong
        } else if newchunks <= 0x10000 as uint32_t {
            newchunks
                .wrapping_add(0xfff as uint32_t)
                .wrapping_div(0x1000 as uint32_t)
                .wrapping_add(0x2c as uint32_t) as ::core::ffi::c_ulong
        } else if newchunks <= 0x100000 as uint32_t {
            newchunks
                .wrapping_add(0xffff as uint32_t)
                .wrapping_div(0x10000 as uint32_t)
                .wrapping_add(0x3b as uint32_t) as ::core::ffi::c_ulong
        } else if newchunks <= 0x1000000 as uint32_t {
            newchunks
                .wrapping_add(0xfffff as uint32_t)
                .wrapping_div(0x100000 as uint32_t)
                .wrapping_add(0x4a as uint32_t) as ::core::ffi::c_ulong
        } else if newchunks <= 0x10000000 as uint32_t {
            newchunks
                .wrapping_add(0xffffff as uint32_t)
                .wrapping_div(0x1000000 as uint32_t)
                .wrapping_add(0x59 as uint32_t) as ::core::ffi::c_ulong
        } else {
            (newchunks as ::core::ffi::c_ulong)
                .wrapping_add(0xfffffff as ::core::ffi::c_ulong)
                .wrapping_div(0x10000000 as ::core::ffi::c_ulong)
                .wrapping_add(0x68 as ::core::ffi::c_ulong)
        }) as uint8_t;
        if oldindx as ::core::ffi::c_int == newindx as ::core::ffi::c_int {
            chunktab_used = (chunktab_used as ::core::ffi::c_ulong).wrapping_add(
                (newchunks as usize).wrapping_mul(::core::mem::size_of::<uint64_t>())
                    as ::core::ffi::c_ulong,
            ) as uint64_t;
            chunktab_used = (chunktab_used as ::core::ffi::c_ulong).wrapping_sub(
                (oldchunks as usize).wrapping_mul(::core::mem::size_of::<uint64_t>())
                    as ::core::ffi::c_ulong,
            ) as uint64_t;
            return oldchunktab;
        } else {
            if newindx as ::core::ffi::c_int >= CHUNKTAB_MAX_INDX {
                newchunktab = ::core::ptr::null_mut::<uint64_t>();
            } else {
                newchunktab = chunktab_indx_malloc(newindx);
                if !oldchunktab.is_null() && oldchunks > 0 as uint32_t {
                    if newchunks > oldchunks {
                        memcpy(
                            newchunktab as *mut ::core::ffi::c_void,
                            oldchunktab as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<uint64_t>().wrapping_mul(oldchunks as size_t),
                        );
                    } else {
                        memcpy(
                            newchunktab as *mut ::core::ffi::c_void,
                            oldchunktab as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<uint64_t>().wrapping_mul(newchunks as size_t),
                        );
                    }
                }
                chunktab_used = (chunktab_used as ::core::ffi::c_ulong).wrapping_add(
                    (newchunks as usize).wrapping_mul(::core::mem::size_of::<uint64_t>())
                        as ::core::ffi::c_ulong,
                ) as uint64_t;
            }
            if !oldchunktab.is_null() && (oldindx as ::core::ffi::c_int) < CHUNKTAB_MAX_INDX {
                chunktab_indx_free(oldchunktab, oldindx);
                chunktab_used = (chunktab_used as ::core::ffi::c_ulong).wrapping_sub(
                    (oldchunks as usize).wrapping_mul(::core::mem::size_of::<uint64_t>())
                        as ::core::ffi::c_ulong,
                ) as uint64_t;
            }
            return newchunktab;
        };
    }
}
#[inline]
unsafe extern "C" fn chunktab_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = chunktab_allocated;
        *used = chunktab_used;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_memusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated.offset(0 as isize) =
            ::core::mem::size_of::<*mut fsedge>().wrapping_mul(edgerehashpos as usize) as uint64_t;
        *used.offset(0 as isize) =
            ::core::mem::size_of::<*mut fsedge>().wrapping_mul(edgehashelem as usize) as uint64_t;
        fsedge_getusage(
            allocated.offset(1 as ::core::ffi::c_int as isize),
            used.offset(1 as ::core::ffi::c_int as isize),
        );
        *allocated.offset(2 as isize) =
            ::core::mem::size_of::<*mut fsnode>().wrapping_mul(noderehashpos as usize) as uint64_t;
        *used.offset(2 as isize) =
            ::core::mem::size_of::<*mut fsnode>().wrapping_mul(nodehashelem as usize) as uint64_t;
        fsnode_getusage(
            allocated.offset(3 as ::core::ffi::c_int as isize),
            used.offset(3 as ::core::ffi::c_int as isize),
        );
        freenode_getusage(
            allocated.offset(4 as ::core::ffi::c_int as isize),
            used.offset(4 as ::core::ffi::c_int as isize),
        );
        chunktab_getusage(
            allocated.offset(5 as ::core::ffi::c_int as isize),
            used.offset(5 as ::core::ffi::c_int as isize),
        );
        symlink_getusage(
            allocated.offset(6 as ::core::ffi::c_int as isize),
            used.offset(6 as ::core::ffi::c_int as isize),
        );
        quotanode_getusage(
            allocated.offset(7 as ::core::ffi::c_int as isize),
            used.offset(7 as ::core::ffi::c_int as isize),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsnodes_get_next_id() -> uint32_t {
    unsafe {
        let mut i: uint32_t = 0;
        let mut mask: uint32_t = 0;
        while searchpos < bitmasksize
            && *freebitmask.offset(searchpos as isize) == 0xffffffff as uint32_t
        {
            searchpos = searchpos.wrapping_add(1);
        }
        if searchpos == bitmasksize {
            let mut tmpfbm: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
            bitmasksize = bitmasksize.wrapping_add(0x80 as uint32_t);
            tmpfbm = freebitmask;
            freebitmask = realloc(
                freebitmask as *mut ::core::ffi::c_void,
                (bitmasksize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>()),
            ) as *mut uint32_t;
            if freebitmask.is_null() {
                free(tmpfbm as *mut ::core::ffi::c_void);
            }
            if freebitmask.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if freebitmask
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            memset(
                freebitmask.offset(searchpos as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (0x80 as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>()),
            );
        }
        mask = *freebitmask.offset(searchpos as isize);
        i = 0 as uint32_t;
        while mask & 1 as uint32_t != 0 {
            i = i.wrapping_add(1);
            mask >>= 1 as ::core::ffi::c_int;
        }
        mask = ((1 as ::core::ffi::c_int) << i) as uint32_t;
        *freebitmask.offset(searchpos as isize) |= mask;
        i = i.wrapping_add(searchpos << 5 as ::core::ffi::c_int);
        if i > maxnodeid {
            maxnodeid = i;
        }
        return i;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsnodes_free_fixts(mut ts: uint32_t) {
    unsafe {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"last freed inode has higher timestamp than the current one - fixing timestamps in free inodes list\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        let mut n: *mut freenode = ::core::ptr::null_mut::<freenode>();
        n = freelist;
        while !n.is_null() {
            if (*n).ftime > ts {
                (*n).ftime = ts;
            }
            n = (*n).next as *mut freenode;
        }
        freelastts = ts;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsnodes_free_id(mut inode: uint32_t, mut ts: uint32_t) {
    unsafe {
        let mut n: *mut freenode = ::core::ptr::null_mut::<freenode>();
        if ts < freelastts {
            fsnodes_free_fixts(ts);
        }
        n = freenode_malloc();
        (*n).inode = inode;
        (*n).ftime = ts;
        (*n).next = ::core::ptr::null_mut::<_freenode>();
        *freetail = n;
        freetail = &raw mut (*n).next as *mut *mut freenode;
        freelastts = ts;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_freeinodes(
    mut ts: uint32_t,
    mut sesflags: uint8_t,
    mut inodereusedelay: uint32_t,
    mut freeinodes: uint32_t,
    mut sustainedinodes: uint32_t,
    mut inode_chksum: uint32_t,
) -> uint8_t {
    unsafe {
        let mut si: uint32_t = 0;
        let mut fi: uint32_t = 0;
        let mut pos: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut ics: uint32_t = 0;
        let mut n: *mut freenode = ::core::ptr::null_mut::<freenode>();
        let mut an: *mut freenode = ::core::ptr::null_mut::<freenode>();
        let mut sn: *mut freenode = ::core::ptr::null_mut::<freenode>();
        let mut snt: *mut *mut freenode = ::core::ptr::null_mut::<*mut freenode>();
        fi = 0 as uint32_t;
        si = 0 as uint32_t;
        ics = 0 as uint32_t;
        n = freelist;
        sn = ::core::ptr::null_mut::<freenode>();
        snt = &raw mut sn;
        if ts < freelastts {
            fsnodes_free_fixts(ts);
        }
        while !n.is_null() && (*n).ftime.wrapping_add(inodereusedelay) < ts {
            ics ^= (*n).inode;
            if (sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                || sustainedinodes > 0 as uint32_t)
                && of_isfileopen((*n).inode) as ::core::ffi::c_int != 0
            {
                si = si.wrapping_add(1);
                an = (*n).next as *mut freenode;
                (*n).ftime = ts;
                (*n).next = ::core::ptr::null_mut::<_freenode>();
                *snt = n;
                snt = &raw mut (*n).next as *mut *mut freenode;
            } else {
                fi = fi.wrapping_add(1);
                pos = (*n).inode >> 5 as ::core::ffi::c_int;
                mask = ((1 as ::core::ffi::c_int) << ((*n).inode & 0x1f as uint32_t)) as uint32_t;
                *freebitmask.offset(pos as isize) &= !mask;
                if pos < searchpos {
                    searchpos = pos;
                }
                an = (*n).next as *mut freenode;
                freenode_free(n);
            }
            n = an;
        }
        if !n.is_null() {
            freelist = n;
        } else {
            freelist = ::core::ptr::null_mut::<freenode>();
            freetail = &raw mut freelist;
            freelastts = 0 as uint32_t;
        }
        if !sn.is_null() {
            *freetail = sn;
            freetail = snt;
            freelastts = ts;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if fi > 0 as uint32_t || si > 0 as uint32_t {
                changelog(
                    b"%u|FREEINODES(%u):%u,%u,%u\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    inodereusedelay,
                    fi,
                    si,
                    ics,
                );
            }
        } else {
            if freeinodes != fi
                || sustainedinodes != si
                || inode_chksum != 0 as uint32_t && inode_chksum != ics
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"FREEINODES data mismatch: my:(%u,%u,%u) != expected:(%u,%u,%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    fi,
                    si,
                    ics,
                    freeinodes,
                    sustainedinodes,
                    inode_chksum,
                );
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsnodes_freeinodes() {
    unsafe {
        fs_univ_freeinodes(
            main_time(),
            0 as uint8_t,
            InodeReuseDelay,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_freeinodes(
    mut ts: uint32_t,
    mut inodereusedelay: uint32_t,
    mut freeinodes: uint32_t,
    mut sustainedinodes: uint32_t,
    mut inode_chksum: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_freeinodes(
            ts,
            SESFLAG_METARESTORE as uint8_t,
            inodereusedelay,
            freeinodes,
            sustainedinodes,
            inode_chksum,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsnodes_init_freebitmask() {
    unsafe {
        bitmasksize = (0x100 as uint32_t)
            .wrapping_add(maxnodeid >> 5 as ::core::ffi::c_int & 0xffffff80 as uint32_t);
        freebitmask =
            malloc((bitmasksize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>()))
                as *mut uint32_t;
        if freebitmask.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                939 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                939 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if freebitmask
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint32_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                939 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                939 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        memset(
            freebitmask as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (bitmasksize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>()),
        );
        *freebitmask.offset(0 as isize) = 1 as uint32_t;
        searchpos = 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsnodes_used_inode(mut inode: uint32_t) {
    unsafe {
        let mut pos: uint32_t = 0;
        let mut mask: uint32_t = 0;
        pos = inode >> 5 as ::core::ffi::c_int;
        mask = ((1 as ::core::ffi::c_int) << (inode & 0x1f as uint32_t)) as uint32_t;
        if *freebitmask.offset(pos as isize) & mask != 0 {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"freebitmask: repeated inode: %u\0".as_ptr() as *const ::core::ffi::c_char,
                inode,
            );
        }
        if pos >= bitmasksize {
            let mut tmpfbm: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
            let mut oldsize: uint32_t = bitmasksize;
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"freebitmask overrun (%u>=%u)\0".as_ptr() as *const ::core::ffi::c_char,
                pos,
                bitmasksize,
            );
            bitmasksize = (0x100 as uint32_t).wrapping_add(pos & 0xffffff80 as uint32_t);
            tmpfbm = freebitmask;
            freebitmask = realloc(
                freebitmask as *mut ::core::ffi::c_void,
                (bitmasksize as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>()),
            ) as *mut uint32_t;
            if freebitmask.is_null() {
                free(tmpfbm as *mut ::core::ffi::c_void);
            }
            if freebitmask.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    963 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    963 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if freebitmask
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    963 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    963 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"freebitmask\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            memset(
                freebitmask.offset(oldsize as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (bitmasksize.wrapping_sub(oldsize) as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint32_t>()),
            );
        }
        *freebitmask.offset(pos as isize) |= mask;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_hash(
    mut parentid: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
) -> uint32_t {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut i: uint32_t = 0;
        hash = parentid
            .wrapping_mul(0x5f2318bd as uint32_t)
            .wrapping_add(nleng as uint32_t);
        i = 0 as uint32_t;
        while i < nleng as uint32_t {
            hash = hash
                .wrapping_mul(33 as uint32_t)
                .wrapping_add(*name.offset(i as isize) as uint32_t);
            i = i.wrapping_add(1);
        }
        return hash;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_calc_hash_size(mut elements: uint32_t) -> uint32_t {
    unsafe {
        let mut res: uint32_t = 1 as uint32_t;
        while elements != 0 {
            elements >>= 1 as ::core::ffi::c_int;
            res <<= 1 as ::core::ffi::c_int;
        }
        if res == 0 as uint32_t {
            res = 0x80000000 as ::core::ffi::c_uint as uint32_t;
        }
        if res < HASHTAB_LOSIZE as uint32_t {
            return HASHTAB_LOSIZE as uint32_t;
        }
        return res;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edge_hash_init() {
    unsafe {
        let mut i: uint16_t = 0;
        edgehashsize = 0 as uint32_t;
        edgehashelem = 0 as uint32_t;
        edgerehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            edgehashtab[i as usize] = ::core::ptr::null_mut::<*mut fsedge>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edge_hash_cleanup() {
    unsafe {
        let mut i: uint16_t = 0;
        edgehashelem = 0 as uint32_t;
        edgehashsize = 0 as uint32_t;
        edgerehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            if !edgehashtab[i as usize].is_null() {
                munmap(
                    edgehashtab[i as usize] as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<*mut fsedge>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                );
            }
            edgehashtab[i as usize] = ::core::ptr::null_mut::<*mut fsedge>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edge_hash_rehash() {
    unsafe {
        let mut i: uint16_t = 0;
        edgerehashpos = edgehashsize;
        edgehashsize = edgehashsize.wrapping_mul(2 as uint32_t);
        i = (edgehashsize >> HASHTAB_LOBITS).wrapping_div(2 as uint32_t) as uint16_t;
        while (i as uint32_t) < edgehashsize >> HASHTAB_LOBITS {
            edgehashtab[i as usize] = mmap(
                NULL,
                ::core::mem::size_of::<*mut fsedge>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut *mut fsedge;
            if edgehashtab[i as usize].is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1039 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"edgehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1039 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"edgehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if edgehashtab[i as usize]
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut fsedge
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1039 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"edgehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1039 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"edgehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edge_hash_move() {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut moved: uint32_t = 0 as uint32_t;
        let mut ehptr: *mut *mut fsedge = ::core::ptr::null_mut::<*mut fsedge>();
        let mut ehptralt: *mut *mut fsedge = ::core::ptr::null_mut::<*mut fsedge>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        mask = edgehashsize.wrapping_sub(1 as uint32_t);
        loop {
            if edgerehashpos >= edgehashsize {
                edgerehashpos = edgehashsize;
                return;
            }
            ehptr = edgehashtab[(edgerehashpos
                .wrapping_sub(edgehashsize.wrapping_div(2 as uint32_t))
                >> HASHTAB_LOBITS) as usize]
                .offset((edgerehashpos & HASHTAB_MASK as uint32_t) as isize);
            ehptralt = edgehashtab[(edgerehashpos >> HASHTAB_LOBITS) as usize]
                .offset((edgerehashpos & HASHTAB_MASK as uint32_t) as isize);
            *ehptralt = ::core::ptr::null_mut::<fsedge>();
            loop {
                e = *ehptr;
                if e.is_null() {
                    break;
                }
                hash = (*e).hashval & mask;
                if hash == edgerehashpos {
                    *ehptralt = e;
                    *ehptr = (*e).next as *mut fsedge;
                    ehptralt = &raw mut (*e).next as *mut *mut fsedge;
                    (*e).next = ::core::ptr::null_mut::<_fsedge>();
                } else {
                    ehptr = &raw mut (*e).next as *mut *mut fsedge;
                }
                moved = moved.wrapping_add(1);
            }
            edgerehashpos = edgerehashpos.wrapping_add(1);
            if moved >= HASHTAB_MOVEFACTOR as uint32_t {
                break;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edge_find(
    mut node: *mut fsnode,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
) -> *mut fsedge {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut hash: uint32_t = 0;
        let mut hashval: uint32_t = 0;
        if edgehashsize == 0 as uint32_t {
            return ::core::ptr::null_mut::<fsedge>();
        }
        hashval = fsnodes_hash((*node).inode, nleng, name);
        hash = hashval & edgehashsize.wrapping_sub(1 as uint32_t);
        if edgerehashpos < edgehashsize {
            fsnodes_edge_hash_move();
            if hash >= edgerehashpos {
                hash = hash.wrapping_sub(edgehashsize.wrapping_div(2 as uint32_t));
            }
        }
        e = *edgehashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        while !e.is_null() {
            if (*e).parent == node
                && (*e).hashval == hashval
                && (*e).nleng as ::core::ffi::c_int == nleng as ::core::ffi::c_int
                && memcmp(
                    &raw const (*e).name as *const uint8_t as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    name as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    nleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return e;
            }
            e = (*e).next as *mut fsedge;
        }
        return ::core::ptr::null_mut::<fsedge>();
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edge_delete(mut e: *mut fsedge) {
    unsafe {
        let mut ehptr: *mut *mut fsedge = ::core::ptr::null_mut::<*mut fsedge>();
        let mut eit: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut hash: uint32_t = 0;
        if edgehashsize == 0 as uint32_t {
            return;
        }
        hash = (*e).hashval & edgehashsize.wrapping_sub(1 as uint32_t);
        if edgerehashpos < edgehashsize {
            fsnodes_edge_hash_move();
            if hash >= edgerehashpos {
                hash = hash.wrapping_sub(edgehashsize.wrapping_div(2 as uint32_t));
            }
        }
        ehptr = edgehashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        loop {
            eit = *ehptr;
            if eit.is_null() {
                break;
            }
            if eit == e {
                *ehptr = (*e).next as *mut fsedge;
                edgehashelem = edgehashelem.wrapping_sub(1);
                return;
            }
            ehptr = &raw mut (*eit).next as *mut *mut fsedge;
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edge_add(mut e: *mut fsedge) {
    unsafe {
        let mut i: uint16_t = 0;
        let mut hash: uint32_t = 0;
        if edgehashsize == 0 as uint32_t {
            edgehashsize = fsnodes_calc_hash_size(hashelements);
            edgerehashpos = edgehashsize;
            edgehashelem = 0 as uint32_t;
            i = 0 as uint16_t;
            while (i as uint32_t) < edgehashsize >> HASHTAB_LOBITS {
                edgehashtab[i as usize] = mmap(
                    NULL,
                    ::core::mem::size_of::<*mut fsedge>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                    PROT_READ | PROT_WRITE,
                    MAP_ANON | MAP_PRIVATE,
                    -1 as ::core::ffi::c_int,
                    0 as __off64_t,
                ) as *mut *mut fsedge;
                if edgehashtab[i as usize].is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"edgehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"edgehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if edgehashtab[i as usize]
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut *mut fsedge
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"edgehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"edgehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                memset(
                    edgehashtab[i as usize] as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<*mut fsedge>(),
                );
                if (*edgehashtab[i as usize].offset(0 as isize)).is_null() {
                    memset(
                        edgehashtab[i as usize] as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<*mut fsedge>()
                            .wrapping_mul(HASHTAB_LOSIZE as size_t),
                    );
                } else {
                    hash = 0 as uint32_t;
                    while hash < HASHTAB_LOSIZE as uint32_t {
                        *edgehashtab[i as usize].offset(hash as isize) =
                            ::core::ptr::null_mut::<fsedge>();
                        hash = hash.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        (*e).hashval = fsnodes_hash(
            (*(*e).parent).inode,
            (*e).nleng,
            &raw const (*e).name as *const uint8_t,
        );
        hash = (*e).hashval & edgehashsize.wrapping_sub(1 as uint32_t);
        if edgerehashpos < edgehashsize {
            fsnodes_edge_hash_move();
            if hash >= edgerehashpos {
                hash = hash.wrapping_sub(edgehashsize.wrapping_div(2 as uint32_t));
            }
            (*e).next = *edgehashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut _fsedge;
            *edgehashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = e;
            edgehashelem = edgehashelem.wrapping_add(1);
        } else {
            (*e).next = *edgehashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut _fsedge;
            *edgehashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = e;
            edgehashelem = edgehashelem.wrapping_add(1);
            if edgehashelem > edgehashsize
                && edgehashsize >> HASHTAB_LOBITS < HASHTAB_HISIZE as uint32_t
            {
                fsnodes_edge_hash_rehash();
            }
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_nameisused(
    mut node: *mut fsnode,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        return if !fsnodes_edge_find(node, nleng, name).is_null() {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_lookup(
    mut node: *mut fsnode,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
) -> *mut fsedge {
    unsafe {
        if (*node).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return ::core::ptr::null_mut::<fsedge>();
        }
        return fsnodes_edge_find(node, nleng, name);
    }
}
#[inline]
unsafe extern "C" fn fsnodes_find_uniqname(
    mut node: *mut fsnode,
    mut req_nleng: uint8_t,
    mut req_name: *const uint8_t,
    mut uniq_nleng: *mut uint8_t,
    mut uniq_name: *mut uint8_t,
) {
    unsafe {
        let mut dot_pos: uint8_t = 0;
        let mut base_leng: uint8_t = 0;
        let mut fixed_base_leng: uint8_t = 0;
        let mut ext_leng: uint8_t = 0;
        let mut cnt: uint8_t = 0;
        let mut extra_chars: uint8_t = 0;
        let mut i: uint8_t = 0;
        let mut r: uint32_t = 0;
        dot_pos = (req_nleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint8_t;
        while dot_pos as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if *req_name.offset(dot_pos as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            {
                break;
            }
            dot_pos = dot_pos.wrapping_sub(1);
        }
        if dot_pos as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            base_leng = req_nleng;
            ext_leng = 0 as uint8_t;
        } else if req_nleng as ::core::ffi::c_int
            - dot_pos as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
            > 128 as ::core::ffi::c_int
        {
            base_leng = req_nleng;
            ext_leng = 0 as uint8_t;
            dot_pos = 0 as uint8_t;
        } else {
            base_leng = dot_pos;
            ext_leng = (req_nleng as ::core::ffi::c_int - dot_pos as ::core::ffi::c_int) as uint8_t;
        }
        cnt = 0 as uint8_t;
        loop {
            cnt = cnt.wrapping_add(1);
            if (cnt as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                extra_chars = 4 as uint8_t;
            } else {
                extra_chars = 11 as uint8_t;
            }
            if req_nleng as ::core::ffi::c_int
                > 255 as ::core::ffi::c_int - extra_chars as ::core::ffi::c_int
            {
                if base_leng as ::core::ffi::c_int > extra_chars as ::core::ffi::c_int {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"base_leng>extra_chars\0".as_ptr() as *const ::core::ffi::c_char,
                        b"name error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"base_leng>extra_chars\0".as_ptr() as *const ::core::ffi::c_char,
                        b"name error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                fixed_base_leng = (255 as ::core::ffi::c_int
                    - extra_chars as ::core::ffi::c_int
                    - ext_leng as ::core::ffi::c_int) as uint8_t;
            } else {
                fixed_base_leng = base_leng;
            }
            memcpy(
                uniq_name as *mut ::core::ffi::c_void,
                req_name as *const ::core::ffi::c_void,
                fixed_base_leng as size_t,
            );
            *uniq_name.offset(fixed_base_leng as isize) = ' ' as uint8_t;
            *uniq_name.offset(
                (fixed_base_leng as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) = '(' as uint8_t;
            if (cnt as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                *uniq_name.offset(
                    (fixed_base_leng as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = ('0' as ::core::ffi::c_int + cnt as ::core::ffi::c_int) as uint8_t;
                *uniq_name.offset(
                    (fixed_base_leng as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = ')' as uint8_t;
            } else {
                r = rndu32();
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
                    *uniq_name.offset(
                        (fixed_base_leng as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + i as ::core::ffi::c_int) as isize,
                    ) = ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                        *b"0123456789ABCDEF\0",
                    )[(r & 0xf as uint32_t) as usize] as uint8_t;
                    r >>= 4 as ::core::ffi::c_int;
                    i = i.wrapping_add(1);
                }
                *uniq_name.offset(
                    (fixed_base_leng as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = ')' as uint8_t;
            }
            if ext_leng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                memcpy(
                    uniq_name
                        .offset(fixed_base_leng as ::core::ffi::c_int as isize)
                        .offset(extra_chars as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    req_name.offset(dot_pos as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    ext_leng as size_t,
                );
            }
            *uniq_nleng = (fixed_base_leng as ::core::ffi::c_int
                + extra_chars as ::core::ffi::c_int
                + ext_leng as ::core::ffi::c_int) as uint8_t;
            if fsnodes_edge_find(node, *uniq_nleng as uint16_t, uniq_name as *const uint8_t)
                .is_null()
            {
                return;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_node_hash_init() {
    unsafe {
        let mut i: uint16_t = 0;
        nodehashsize = 0 as uint32_t;
        nodehashelem = 0 as uint32_t;
        noderehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            nodehashtab[i as usize] = ::core::ptr::null_mut::<*mut fsnode>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_node_hash_cleanup() {
    unsafe {
        let mut i: uint16_t = 0;
        nodehashelem = 0 as uint32_t;
        nodehashsize = 0 as uint32_t;
        noderehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            if !nodehashtab[i as usize].is_null() {
                munmap(
                    nodehashtab[i as usize] as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<*mut fsnode>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                );
            }
            nodehashtab[i as usize] = ::core::ptr::null_mut::<*mut fsnode>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_node_hash_rehash() {
    unsafe {
        let mut i: uint16_t = 0;
        noderehashpos = nodehashsize;
        nodehashsize = nodehashsize.wrapping_mul(2 as uint32_t);
        i = (nodehashsize >> HASHTAB_LOBITS).wrapping_div(2 as uint32_t) as uint16_t;
        while (i as uint32_t) < nodehashsize >> HASHTAB_LOBITS {
            nodehashtab[i as usize] = mmap(
                NULL,
                ::core::mem::size_of::<*mut fsnode>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut *mut fsnode;
            if nodehashtab[i as usize].is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nodehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nodehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if nodehashtab[i as usize]
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut fsnode
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nodehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nodehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_node_hash_move() {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut moved: uint32_t = 0 as uint32_t;
        let mut phptr: *mut *mut fsnode = ::core::ptr::null_mut::<*mut fsnode>();
        let mut phptralt: *mut *mut fsnode = ::core::ptr::null_mut::<*mut fsnode>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        mask = nodehashsize.wrapping_sub(1 as uint32_t);
        loop {
            if noderehashpos >= nodehashsize {
                noderehashpos = nodehashsize;
                return;
            }
            phptr = nodehashtab[(noderehashpos
                .wrapping_sub(nodehashsize.wrapping_div(2 as uint32_t))
                >> HASHTAB_LOBITS) as usize]
                .offset((noderehashpos & HASHTAB_MASK as uint32_t) as isize);
            phptralt = nodehashtab[(noderehashpos >> HASHTAB_LOBITS) as usize]
                .offset((noderehashpos & HASHTAB_MASK as uint32_t) as isize);
            *phptralt = ::core::ptr::null_mut::<fsnode>();
            loop {
                p = *phptr;
                if p.is_null() {
                    break;
                }
                hash = hash32((*p).inode) & mask;
                if hash == noderehashpos {
                    *phptralt = p;
                    *phptr = (*p).next as *mut fsnode;
                    phptralt = &raw mut (*p).next as *mut *mut fsnode;
                    (*p).next = ::core::ptr::null_mut::<_fsnode>();
                } else {
                    phptr = &raw mut (*p).next as *mut *mut fsnode;
                }
                moved = moved.wrapping_add(1);
            }
            noderehashpos = noderehashpos.wrapping_add(1);
            if moved >= HASHTAB_MOVEFACTOR as uint32_t {
                break;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_node_find(mut inode: uint32_t) -> *mut fsnode {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut hash: uint32_t = 0;
        if nodehashsize == 0 as uint32_t {
            return ::core::ptr::null_mut::<fsnode>();
        }
        hash = hash32(inode) & nodehashsize.wrapping_sub(1 as uint32_t);
        if noderehashpos < nodehashsize {
            fsnodes_node_hash_move();
            if hash >= noderehashpos {
                hash = hash.wrapping_sub(nodehashsize.wrapping_div(2 as uint32_t));
            }
        }
        p = *nodehashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        while !p.is_null() {
            if (*p).inode == inode {
                return p;
            }
            p = (*p).next as *mut fsnode;
        }
        return ::core::ptr::null_mut::<fsnode>();
    }
}
#[inline]
unsafe extern "C" fn fsnodes_node_delete(mut p: *mut fsnode) {
    unsafe {
        let mut phptr: *mut *mut fsnode = ::core::ptr::null_mut::<*mut fsnode>();
        let mut pit: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut hash: uint32_t = 0;
        if nodehashsize == 0 as uint32_t {
            return;
        }
        hash = hash32((*p).inode) & nodehashsize.wrapping_sub(1 as uint32_t);
        if noderehashpos < nodehashsize {
            fsnodes_node_hash_move();
            if hash >= noderehashpos {
                hash = hash.wrapping_sub(nodehashsize.wrapping_div(2 as uint32_t));
            }
        }
        phptr = nodehashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        loop {
            pit = *phptr;
            if pit.is_null() {
                break;
            }
            if pit == p {
                *phptr = (*p).next as *mut fsnode;
                nodehashelem = nodehashelem.wrapping_sub(1);
                return;
            }
            phptr = &raw mut (*pit).next as *mut *mut fsnode;
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_node_add(mut p: *mut fsnode) {
    unsafe {
        let mut i: uint16_t = 0;
        let mut hash: uint32_t = 0;
        if nodehashsize == 0 as uint32_t {
            nodehashsize = fsnodes_calc_hash_size(hashelements);
            noderehashpos = nodehashsize;
            nodehashelem = 0 as uint32_t;
            i = 0 as uint16_t;
            while (i as uint32_t) < nodehashsize >> HASHTAB_LOBITS {
                nodehashtab[i as usize] = mmap(
                    NULL,
                    ::core::mem::size_of::<*mut fsnode>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                    PROT_READ | PROT_WRITE,
                    MAP_ANON | MAP_PRIVATE,
                    -1 as ::core::ffi::c_int,
                    0 as __off64_t,
                ) as *mut *mut fsnode;
                if nodehashtab[i as usize].is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1377 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nodehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1377 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nodehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if nodehashtab[i as usize]
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut *mut fsnode
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1377 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nodehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1377 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nodehashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                memset(
                    nodehashtab[i as usize] as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<*mut fsnode>(),
                );
                if (*nodehashtab[i as usize].offset(0 as isize)).is_null() {
                    memset(
                        nodehashtab[i as usize] as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<*mut fsnode>()
                            .wrapping_mul(HASHTAB_LOSIZE as size_t),
                    );
                } else {
                    hash = 0 as uint32_t;
                    while hash < HASHTAB_LOSIZE as uint32_t {
                        *nodehashtab[i as usize].offset(hash as isize) =
                            ::core::ptr::null_mut::<fsnode>();
                        hash = hash.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        hash = hash32((*p).inode) & nodehashsize.wrapping_sub(1 as uint32_t);
        if noderehashpos < nodehashsize {
            fsnodes_node_hash_move();
            if hash >= noderehashpos {
                hash = hash.wrapping_sub(nodehashsize.wrapping_div(2 as uint32_t));
            }
            (*p).next = *nodehashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut _fsnode;
            *nodehashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = p;
            nodehashelem = nodehashelem.wrapping_add(1);
        } else {
            (*p).next = *nodehashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut _fsnode;
            *nodehashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = p;
            nodehashelem = nodehashelem.wrapping_add(1);
            if nodehashelem > nodehashsize
                && nodehashsize >> HASHTAB_LOBITS < HASHTAB_HISIZE as uint32_t
            {
                fsnodes_node_hash_rehash();
            }
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_isancestor(
    mut f: *mut fsnode,
    mut p: *mut fsnode,
) -> ::core::ffi::c_int {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        if (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
            || (*f).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
        {
            return 0 as ::core::ffi::c_int;
        }
        e = (*p).parents;
        while !e.is_null() {
            p = (*e).parent as *mut fsnode;
            while !p.is_null() {
                if f == p {
                    return 1 as ::core::ffi::c_int;
                }
                if !(*p).parents.is_null() {
                    if (*(*p).parents).nextparent.is_null() {
                    } else {
                        fprintf(
                            stderr,
                            b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1423 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"p->parents->nextparent==NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"directory has more than one parent !!!\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1423 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"p->parents->nextparent==NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"directory has more than one parent !!!\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        abort();
                    };
                    p = (*(*p).parents).parent as *mut fsnode;
                } else {
                    p = ::core::ptr::null_mut::<fsnode>();
                }
            }
            e = (*e).nextparent as *mut fsedge;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edgeid_init() {
    unsafe {
        let mut h: uint32_t = 0;
        edgeid_id_hashtab =
            malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(EDGEID_HASHSIZE as size_t))
                as *mut uint64_t;
        edgeid_ptr_hashtab =
            malloc(::core::mem::size_of::<*mut fsedge>().wrapping_mul(EDGEID_HASHSIZE as size_t))
                as *mut *mut fsedge;
        if edgeid_id_hashtab.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1437 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"edgeid_id_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1437 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"edgeid_id_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if edgeid_id_hashtab
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint64_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1437 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"edgeid_id_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1437 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"edgeid_id_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        if edgeid_ptr_hashtab.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1438 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"edgeid_ptr_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1438 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"edgeid_ptr_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if edgeid_ptr_hashtab
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut fsedge
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1438 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"edgeid_ptr_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1438 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"edgeid_ptr_hashtab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        h = 0 as uint32_t;
        while h < EDGEID_HASHSIZE as uint32_t {
            *edgeid_id_hashtab.offset(h as isize) = 0 as uint64_t;
            *edgeid_ptr_hashtab.offset(h as isize) = ::core::ptr::null_mut::<fsedge>();
            h = h.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edgeid_insert(mut e: *mut fsedge) {
    unsafe {
        let mut hashid: uint32_t = 0;
        hashid = (*e).edgeid.wrapping_rem(EDGEID_HASHSIZE as uint64_t) as uint32_t;
        *edgeid_id_hashtab.offset(hashid as isize) = (*e).edgeid;
        *edgeid_ptr_hashtab.offset(hashid as isize) = e;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edgeid_find(mut edgeid: uint64_t) -> *mut fsedge {
    unsafe {
        let mut hashid: uint32_t = 0;
        hashid = edgeid.wrapping_rem(EDGEID_HASHSIZE as uint64_t) as uint32_t;
        if *edgeid_id_hashtab.offset(hashid as isize) == edgeid {
            return *edgeid_ptr_hashtab.offset(hashid as isize);
        }
        return ::core::ptr::null_mut::<fsedge>();
    }
}
#[inline]
unsafe extern "C" fn fsnodes_edgeid_remove(mut e: *mut fsedge) {
    unsafe {
        let mut hashid: uint32_t = 0;
        hashid = (*e).edgeid.wrapping_rem(EDGEID_HASHSIZE as uint64_t) as uint32_t;
        if *edgeid_id_hashtab.offset(hashid as isize) == (*e).edgeid {
            *edgeid_ptr_hashtab.offset(hashid as isize) = ::core::ptr::null_mut::<fsedge>();
        }
    }
}
static mut keep_alive_ts: uint64_t = 0;
static mut keep_alive_cnt: uint32_t = 0;
#[inline]
unsafe extern "C" fn fsnodes_keep_alive_begin() {
    unsafe {
        keep_alive_ts = monotonic_useconds();
        keep_alive_cnt = 0 as uint32_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_keep_alive_check() {
    unsafe {
        keep_alive_cnt = keep_alive_cnt.wrapping_add(1);
        if keep_alive_cnt >= 10000 as uint32_t {
            if keep_alive_ts.wrapping_add(100000 as uint64_t) < monotonic_useconds() {
                main_keep_alive();
                keep_alive_ts = monotonic_useconds();
            }
            keep_alive_cnt = 0 as uint32_t;
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_type_convert(mut r#type: uint8_t) -> uint8_t {
    unsafe {
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
}
#[inline]
unsafe extern "C" fn fsnodes_new_quotanode(mut p: *mut fsnode) -> *mut quotanode {
    unsafe {
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        qn = quotanode_malloc();
        if qn.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1541 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"qn\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1541 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"qn\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if qn
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut quotanode
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1541 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"qn\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                1541 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"qn\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        memset(
            qn as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<quotanode>(),
        );
        (*qn).next = quotahead as *mut _quotanode;
        if !(*qn).next.is_null() {
            (*(*qn).next).prev = &raw mut (*qn).next;
        }
        (*qn).prev = &raw mut quotahead as *mut *mut _quotanode;
        quotahead = qn;
        (*qn).node = p as *mut _fsnode;
        (*p).data.ddata.quota = qn;
        return qn;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_delete_quotanode(mut p: *mut fsnode) {
    unsafe {
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        qn = (*p).data.ddata.quota;
        if !qn.is_null() {
            *(*qn).prev = (*qn).next;
            if !(*qn).next.is_null() {
                (*(*qn).next).prev = (*qn).prev;
            }
            quotanode_free(qn);
            (*p).data.ddata.quota = ::core::ptr::null_mut::<quotanode>();
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_check_quotanode(mut qn: *mut quotanode, mut ts: uint32_t) {
    unsafe {
        let mut psr: *mut statsrecord = ::core::ptr::null_mut::<statsrecord>();
        let mut inode: uint32_t = 0;
        let mut graceperiod: uint32_t = 0;
        let mut sq: uint8_t = 0;
        let mut chg: uint8_t = 0;
        let mut exceeded: uint8_t = 0;
        psr = &raw mut (*(*qn).node).data.ddata.stats;
        inode = (*(*qn).node).inode;
        sq = 0 as uint8_t;
        if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SINODES != 0 {
            if (*psr).inodes > (*qn).sinodes {
                sq = 1 as uint8_t;
            }
        }
        if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SLENGTH != 0 {
            if (*psr).length > (*qn).slength {
                sq = 1 as uint8_t;
            }
        }
        if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SSIZE != 0 {
            if (*psr).size > (*qn).ssize {
                sq = 1 as uint8_t;
            }
        }
        if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SREALSIZE != 0 {
            if (*psr).realsize > (*qn).srealsize {
                sq = 1 as uint8_t;
            }
        }
        chg = 0 as uint8_t;
        if sq as ::core::ffi::c_int == 0 as ::core::ffi::c_int && (*qn).stimestamp > 0 as uint32_t {
            (*qn).stimestamp = 0 as uint32_t;
            chg = 1 as uint8_t;
        } else if sq as ::core::ffi::c_int != 0 && (*qn).stimestamp == 0 as uint32_t {
            (*qn).stimestamp = ts;
            chg = 1 as uint8_t;
        }
        graceperiod = if (*qn).graceperiod > 0 as uint32_t {
            (*qn).graceperiod
        } else {
            QuotaDefaultGracePeriod
        };
        exceeded = (if (*qn).stimestamp != 0 && (*qn).stimestamp.wrapping_add(graceperiod) < ts {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if (*qn).exceeded as ::core::ffi::c_int != exceeded as ::core::ffi::c_int {
            (*qn).exceeded = exceeded;
            chg = 1 as uint8_t;
        }
        if chg != 0 {
            changelog(
                b"%u|QUOTA(%u,%hhu,%hhu,%u,%u,%u,%lu,%lu,%lu,%lu,%lu,%lu,%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ts,
                inode,
                (*qn).exceeded as ::core::ffi::c_int,
                (*qn).flags as ::core::ffi::c_int,
                (*qn).stimestamp,
                (*qn).sinodes,
                (*qn).hinodes,
                (*qn).slength,
                (*qn).hlength,
                (*qn).ssize,
                (*qn).hsize,
                (*qn).srealsize,
                (*qn).hrealsize,
                (*qn).graceperiod,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsnodes_check_all_quotas() {
    unsafe {
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        let mut now: uint32_t = 0;
        now = main_time();
        qn = quotahead;
        while !qn.is_null() {
            fsnodes_check_quotanode(qn, now);
            qn = (*qn).next as *mut quotanode;
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_test_quota_noparents(
    mut node: *mut fsnode,
    mut inodes: uint32_t,
    mut length: uint64_t,
    mut size: uint64_t,
    mut realsize: uint64_t,
) -> uint8_t {
    unsafe {
        let mut psr: *mut statsrecord = ::core::ptr::null_mut::<statsrecord>();
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        if !node.is_null()
            && (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
            && !(*node).data.ddata.quota.is_null()
        {
            psr = &raw mut (*node).data.ddata.stats;
            qn = (*node).data.ddata.quota;
        } else {
            return 0 as uint8_t;
        }
        if inodes > 0 as uint32_t && (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HINODES != 0 {
            if (*psr).inodes.wrapping_add(inodes) > (*qn).hinodes {
                return 1 as uint8_t;
            }
        }
        if length > 0 as uint64_t && (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HLENGTH != 0 {
            if (*psr).length.wrapping_add(length) > (*qn).hlength {
                return 1 as uint8_t;
            }
        }
        if size > 0 as uint64_t && (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HSIZE != 0 {
            if (*psr).size.wrapping_add(size) > (*qn).hsize {
                return 1 as uint8_t;
            }
        }
        if realsize > 0 as uint64_t && (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HREALSIZE != 0
        {
            if (*psr).realsize.wrapping_add(realsize) > (*qn).hrealsize {
                return 1 as uint8_t;
            }
        }
        if (*qn).exceeded != 0 {
            if inodes > 0 as uint32_t && (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SINODES != 0
            {
                if (*psr).inodes.wrapping_add(inodes) > (*qn).sinodes {
                    return 1 as uint8_t;
                }
            }
            if length > 0 as uint64_t && (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SLENGTH != 0
            {
                if (*psr).length.wrapping_add(length) > (*qn).slength {
                    return 1 as uint8_t;
                }
            }
            if size > 0 as uint64_t && (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SSIZE != 0 {
                if (*psr).size.wrapping_add(size) > (*qn).ssize {
                    return 1 as uint8_t;
                }
            }
            if realsize > 0 as uint64_t
                && (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SREALSIZE != 0
            {
                if (*psr).realsize.wrapping_add(realsize) > (*qn).srealsize {
                    return 1 as uint8_t;
                }
            }
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_test_quota(
    mut node: *mut fsnode,
    mut inodes: uint32_t,
    mut length: uint64_t,
    mut size: uint64_t,
    mut realsize: uint64_t,
) -> uint8_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        if fsnodes_test_quota_noparents(
            ::core::ptr::null_mut::<fsnode>(),
            inodes,
            length,
            size,
            realsize,
        ) != 0
        {
            return 1 as uint8_t;
        }
        if fsnodes_test_quota_noparents(node, inodes, length, size, realsize) != 0 {
            return 1 as uint8_t;
        }
        if !node.is_null() && node != root {
            e = (*node).parents;
            while !e.is_null() {
                if fsnodes_test_quota((*e).parent as *mut fsnode, inodes, length, size, realsize)
                    != 0
                {
                    return 1 as uint8_t;
                }
                e = (*e).nextparent as *mut fsedge;
            }
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_test_quota_for_uncommon_nodes(
    mut dstnode: *mut fsnode,
    mut srcnode: *mut fsnode,
    mut inodes: uint32_t,
    mut length: uint64_t,
    mut size: uint64_t,
    mut realsize: uint64_t,
) -> uint8_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut dhead: *mut _node_list = ::core::ptr::null_mut::<_node_list>();
        let mut shead: *mut _node_list = ::core::ptr::null_mut::<_node_list>();
        let mut nlptr: *mut _node_list = ::core::ptr::null_mut::<_node_list>();
        let mut ret: uint8_t = 0 as uint8_t;
        if dstnode == srcnode {
            return 0 as uint8_t;
        }
        dhead = ::core::ptr::null_mut::<_node_list>();
        while !dstnode.is_null() {
            if !(*dstnode).data.ddata.quota.is_null() {
                nlptr = malloc(::core::mem::size_of::<_node_list>()) as *mut _node_list;
                if nlptr.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1710 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nlptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1710 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nlptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if nlptr
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut _node_list
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1710 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nlptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1710 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nlptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*nlptr).node = dstnode;
                (*nlptr).next = dhead as *mut _node_list;
                dhead = nlptr;
            }
            e = (*dstnode).parents;
            if !e.is_null() {
                if (*e).nextparent.is_null() {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1717 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"e->nextparent==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"directory has more than one parent !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1717 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"e->nextparent==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"directory has more than one parent !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                dstnode = (*e).parent as *mut fsnode;
            } else {
                dstnode = ::core::ptr::null_mut::<fsnode>();
            }
        }
        shead = ::core::ptr::null_mut::<_node_list>();
        while !srcnode.is_null() {
            if !(*srcnode).data.ddata.quota.is_null() {
                nlptr = malloc(::core::mem::size_of::<_node_list>()) as *mut _node_list;
                if nlptr.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nlptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nlptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if nlptr
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut _node_list
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nlptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1727 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"nlptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    abort();
                }
                (*nlptr).node = srcnode;
                (*nlptr).next = shead as *mut _node_list;
                shead = nlptr;
            }
            e = (*srcnode).parents;
            if !e.is_null() {
                if (*e).nextparent.is_null() {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1734 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"e->nextparent==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"directory has more than one parent !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1734 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"e->nextparent==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"directory has more than one parent !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                srcnode = (*e).parent as *mut fsnode;
            } else {
                srcnode = ::core::ptr::null_mut::<fsnode>();
            }
        }
        while !shead.is_null() && !dhead.is_null() && (*shead).node == (*dhead).node {
            nlptr = shead;
            shead = (*shead).next as *mut _node_list;
            free(nlptr as *mut ::core::ffi::c_void);
            nlptr = dhead;
            dhead = (*dhead).next as *mut _node_list;
            free(nlptr as *mut ::core::ffi::c_void);
        }
        while !shead.is_null() {
            nlptr = shead;
            shead = (*shead).next as *mut _node_list;
            free(nlptr as *mut ::core::ffi::c_void);
        }
        while !dhead.is_null() {
            if fsnodes_test_quota((*dhead).node, inodes, length, size, realsize) != 0 {
                ret = 1 as uint8_t;
            }
            nlptr = dhead;
            dhead = (*dhead).next as *mut _node_list;
            free(nlptr as *mut ::core::ffi::c_void);
        }
        return ret;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_fix_realsize(mut parent: *mut fsnode, mut realsize_diff: uint64_t) {
    unsafe {
        let mut psr: *mut statsrecord = ::core::ptr::null_mut::<statsrecord>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        if !parent.is_null() {
            psr = &raw mut (*parent).data.ddata.stats;
            (*psr).realsize = (*psr).realsize.wrapping_add(realsize_diff);
            if parent != root {
                e = (*parent).parents;
                while !e.is_null() {
                    fsnodes_fix_realsize((*e).parent as *mut fsnode, realsize_diff);
                    e = (*e).nextparent as *mut fsedge;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_check_realsize(mut node: *mut fsnode) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut lastchunk: uint32_t = 0;
        let mut lastchunksize: uint32_t = 0;
        let mut size: uint64_t = 0;
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut new_realsize: uint64_t = 0;
        let mut old_realsize: uint64_t = 0;
        if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*node).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*node).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            let mut realsize_ratio: uint8_t = (sclass_get_keeparch_storage_eights(
                (*node).sclassid as uint16_t,
                (*node).keepmode() as uint8_t,
            ) as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as uint8_t;
            if realsize_ratio as ::core::ffi::c_int
                != (*node).data.fdata.realsize_ratio as ::core::ffi::c_int
            {
                size = 0 as uint64_t;
                if (*node).data.fdata.length > 0 as uint64_t {
                    lastchunk = ((*node).data.fdata.length.wrapping_sub(1 as uint64_t)
                        >> MFSCHUNKBITS) as uint32_t;
                    lastchunksize = (((*node).data.fdata.length.wrapping_sub(1 as uint64_t)
                        & MFSCHUNKMASK as uint64_t)
                        .wrapping_add(MFSBLOCKSIZE as uint64_t)
                        & MFSBLOCKNEGMASK as uint64_t)
                        .wrapping_add(MFSHDRSIZE as uint64_t)
                        as uint32_t;
                } else {
                    lastchunk = 0 as uint32_t;
                    lastchunksize = MFSHDRSIZE as uint32_t;
                }
                i = 0 as uint32_t;
                while i < (*node).data.fdata.chunks {
                    if *(*node).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t {
                        if i < lastchunk {
                            size = size.wrapping_add((MFSCHUNKSIZE + MFSHDRSIZE) as uint64_t);
                        } else if i == lastchunk {
                            size = size.wrapping_add(lastchunksize as uint64_t);
                        }
                    }
                    i = i.wrapping_add(1);
                }
                new_realsize = size
                    .wrapping_mul(realsize_ratio as uint64_t)
                    .wrapping_div(16 as uint64_t);
                old_realsize = size
                    .wrapping_mul((*node).data.fdata.realsize_ratio as uint64_t)
                    .wrapping_div(16 as uint64_t);
                e = (*node).parents;
                while !e.is_null() {
                    fsnodes_fix_realsize(
                        (*e).parent as *mut fsnode,
                        new_realsize.wrapping_sub(old_realsize),
                    );
                    e = (*e).nextparent as *mut fsedge;
                }
                (*node).data.fdata.realsize_ratio = realsize_ratio;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_get_stats(
    mut node: *mut fsnode,
    mut sr: *mut statsrecord,
    mut fix_realsize_ratio: uint8_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut lastchunk: uint32_t = 0;
        let mut lastchunksize: uint32_t = 0;
        match (*node).r#type() as ::core::ffi::c_int {
            TYPE_DIRECTORY => {
                *sr = (*node).data.ddata.stats;
                (*sr).inodes = (*sr).inodes.wrapping_add(1);
                (*sr).dirs = (*sr).dirs.wrapping_add(1);
            }
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                (*sr).inodes = 1 as uint32_t;
                (*sr).dirs = 0 as uint32_t;
                (*sr).files = 1 as uint32_t;
                (*sr).chunks = 0 as uint32_t;
                (*sr).length = (*node).data.fdata.length;
                (*sr).size = 0 as uint64_t;
                if (*node).data.fdata.length > 0 as uint64_t {
                    lastchunk = ((*node).data.fdata.length.wrapping_sub(1 as uint64_t)
                        >> MFSCHUNKBITS) as uint32_t;
                    lastchunksize = (((*node).data.fdata.length.wrapping_sub(1 as uint64_t)
                        & MFSCHUNKMASK as uint64_t)
                        .wrapping_add(MFSBLOCKSIZE as uint64_t)
                        & MFSBLOCKNEGMASK as uint64_t)
                        .wrapping_add(MFSHDRSIZE as uint64_t)
                        as uint32_t;
                } else {
                    lastchunk = 0 as uint32_t;
                    lastchunksize = MFSHDRSIZE as uint32_t;
                }
                i = 0 as uint32_t;
                while i < (*node).data.fdata.chunks {
                    if *(*node).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t {
                        if i < lastchunk {
                            (*sr).size = (*sr)
                                .size
                                .wrapping_add((MFSCHUNKSIZE + MFSHDRSIZE) as uint64_t);
                        } else if i == lastchunk {
                            (*sr).size = (*sr).size.wrapping_add(lastchunksize as uint64_t);
                        }
                        (*sr).chunks = (*sr).chunks.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                if fix_realsize_ratio as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                    let mut realsize_ratio: uint8_t = (sclass_get_keeparch_storage_eights(
                        (*node).sclassid as uint16_t,
                        (*node).keepmode() as uint8_t,
                    ) as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int)
                        as uint8_t;
                    if realsize_ratio as ::core::ffi::c_int
                        != (*node).data.fdata.realsize_ratio as ::core::ffi::c_int
                    {
                        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
                        let mut new_realsize: uint64_t = (*sr)
                            .size
                            .wrapping_mul(realsize_ratio as uint64_t)
                            .wrapping_div(16 as uint64_t);
                        let mut old_realsize: uint64_t = (*sr)
                            .size
                            .wrapping_mul((*node).data.fdata.realsize_ratio as uint64_t)
                            .wrapping_div(16 as uint64_t);
                        e = (*node).parents;
                        while !e.is_null() {
                            fsnodes_fix_realsize(
                                (*e).parent as *mut fsnode,
                                new_realsize.wrapping_sub(old_realsize),
                            );
                            e = (*e).nextparent as *mut fsedge;
                        }
                        (*node).data.fdata.realsize_ratio = realsize_ratio;
                    }
                } else if fix_realsize_ratio != 0 {
                    (*node).data.fdata.realsize_ratio =
                        (sclass_get_keeparch_storage_eights(
                            (*node).sclassid as uint16_t,
                            (*node).keepmode() as uint8_t,
                        ) as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int) as uint8_t;
                }
                (*sr).realsize = (*sr)
                    .size
                    .wrapping_mul((*node).data.fdata.realsize_ratio as uint64_t)
                    .wrapping_div(16 as uint64_t);
            }
            TYPE_SYMLINK => {
                (*sr).inodes = 1 as uint32_t;
                (*sr).files = 0 as uint32_t;
                (*sr).dirs = 0 as uint32_t;
                (*sr).chunks = 0 as uint32_t;
                (*sr).length = (*node).data.sdata.pleng as uint64_t;
                (*sr).size = 0 as uint64_t;
                (*sr).realsize = 0 as uint64_t;
            }
            _ => {
                (*sr).inodes = 1 as uint32_t;
                (*sr).files = 0 as uint32_t;
                (*sr).dirs = 0 as uint32_t;
                (*sr).chunks = 0 as uint32_t;
                (*sr).length = 0 as uint64_t;
                (*sr).size = 0 as uint64_t;
                (*sr).realsize = 0 as uint64_t;
            }
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_sub_stats(mut parent: *mut fsnode, mut sr: *mut statsrecord) {
    unsafe {
        let mut psr: *mut statsrecord = ::core::ptr::null_mut::<statsrecord>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        if !parent.is_null() {
            psr = &raw mut (*parent).data.ddata.stats;
            (*psr).inodes = (*psr).inodes.wrapping_sub((*sr).inodes);
            (*psr).dirs = (*psr).dirs.wrapping_sub((*sr).dirs);
            (*psr).files = (*psr).files.wrapping_sub((*sr).files);
            (*psr).chunks = (*psr).chunks.wrapping_sub((*sr).chunks);
            (*psr).length = (*psr).length.wrapping_sub((*sr).length);
            (*psr).size = (*psr).size.wrapping_sub((*sr).size);
            (*psr).realsize = (*psr).realsize.wrapping_sub((*sr).realsize);
            if parent != root {
                e = (*parent).parents;
                while !e.is_null() {
                    fsnodes_sub_stats((*e).parent as *mut fsnode, sr);
                    e = (*e).nextparent as *mut fsedge;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_add_stats(mut parent: *mut fsnode, mut sr: *mut statsrecord) {
    unsafe {
        let mut psr: *mut statsrecord = ::core::ptr::null_mut::<statsrecord>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        if !parent.is_null() {
            psr = &raw mut (*parent).data.ddata.stats;
            (*psr).inodes = (*psr).inodes.wrapping_add((*sr).inodes);
            (*psr).dirs = (*psr).dirs.wrapping_add((*sr).dirs);
            (*psr).files = (*psr).files.wrapping_add((*sr).files);
            (*psr).chunks = (*psr).chunks.wrapping_add((*sr).chunks);
            (*psr).length = (*psr).length.wrapping_add((*sr).length);
            (*psr).size = (*psr).size.wrapping_add((*sr).size);
            (*psr).realsize = (*psr).realsize.wrapping_add((*sr).realsize);
            if parent != root {
                e = (*parent).parents;
                while !e.is_null() {
                    fsnodes_add_stats((*e).parent as *mut fsnode, sr);
                    e = (*e).nextparent as *mut fsedge;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_add_sub_stats(
    mut parent: *mut fsnode,
    mut newsr: *mut statsrecord,
    mut prevsr: *mut statsrecord,
) {
    unsafe {
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        sr.inodes = (*newsr).inodes.wrapping_sub((*prevsr).inodes);
        sr.dirs = (*newsr).dirs.wrapping_sub((*prevsr).dirs);
        sr.files = (*newsr).files.wrapping_sub((*prevsr).files);
        sr.chunks = (*newsr).chunks.wrapping_sub((*prevsr).chunks);
        sr.length = (*newsr).length.wrapping_sub((*prevsr).length);
        sr.size = (*newsr).size.wrapping_sub((*prevsr).size);
        sr.realsize = (*newsr).realsize.wrapping_sub((*prevsr).realsize);
        fsnodes_add_stats(parent, &raw mut sr);
    }
}
#[inline]
unsafe extern "C" fn fsnodes_quota_fixspace(
    mut node: *mut fsnode,
    mut totalspace: *mut uint64_t,
    mut availspace: *mut uint64_t,
) {
    unsafe {
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut quotasize: uint64_t = 0;
        if !node.is_null()
            && (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
            && {
                qn = (*node).data.ddata.quota;
                !qn.is_null()
            }
            && (*qn).flags as ::core::ffi::c_int
                & (QUOTA_FLAG_HREALSIZE
                    | QUOTA_FLAG_SREALSIZE
                    | QUOTA_FLAG_HSIZE
                    | QUOTA_FLAG_SSIZE
                    | QUOTA_FLAG_HLENGTH
                    | QUOTA_FLAG_SLENGTH)
                != 0
        {
            fsnodes_get_stats(node, &raw mut sr, 2 as uint8_t);
            if (*qn).flags as ::core::ffi::c_int & (QUOTA_FLAG_HREALSIZE | QUOTA_FLAG_SREALSIZE)
                != 0
            {
                quotasize = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HREALSIZE != 0
                    && quotasize > (*qn).hrealsize
                {
                    quotasize = (*qn).hrealsize;
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SREALSIZE != 0
                    && quotasize > (*qn).srealsize
                {
                    quotasize = (*qn).srealsize;
                }
                if sr.realsize >= quotasize {
                    *availspace = 0 as uint64_t;
                } else if *availspace > quotasize.wrapping_sub(sr.realsize) {
                    *availspace = quotasize.wrapping_sub(sr.realsize);
                }
                if *totalspace > quotasize {
                    *totalspace = quotasize;
                }
                if sr.realsize.wrapping_add(*availspace) < *totalspace {
                    *totalspace = sr.realsize.wrapping_add(*availspace);
                }
            }
            if (*qn).flags as ::core::ffi::c_int & (QUOTA_FLAG_HSIZE | QUOTA_FLAG_SSIZE) != 0 {
                quotasize = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HSIZE != 0
                    && quotasize > (*qn).hsize
                {
                    quotasize = (*qn).hsize;
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SSIZE != 0
                    && quotasize > (*qn).ssize
                {
                    quotasize = (*qn).ssize;
                }
                if sr.size >= quotasize {
                    *availspace = 0 as uint64_t;
                } else if *availspace > quotasize.wrapping_sub(sr.size) {
                    *availspace = quotasize.wrapping_sub(sr.size);
                }
                if *totalspace > quotasize {
                    *totalspace = quotasize;
                }
                if sr.size.wrapping_add(*availspace) < *totalspace {
                    *totalspace = sr.size.wrapping_add(*availspace);
                }
            }
            if (*qn).flags as ::core::ffi::c_int & (QUOTA_FLAG_HLENGTH | QUOTA_FLAG_SLENGTH) != 0 {
                quotasize = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HLENGTH != 0
                    && quotasize > (*qn).hlength
                {
                    quotasize = (*qn).hlength;
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SLENGTH != 0
                    && quotasize > (*qn).slength
                {
                    quotasize = (*qn).slength;
                }
                if sr.length >= quotasize {
                    *availspace = 0 as uint64_t;
                } else if *availspace > quotasize.wrapping_sub(sr.length) {
                    *availspace = quotasize.wrapping_sub(sr.length);
                }
                if *totalspace > quotasize {
                    *totalspace = quotasize;
                }
                if sr.length.wrapping_add(*availspace) < *totalspace {
                    *totalspace = sr.length.wrapping_add(*availspace);
                }
            }
        }
        if !node.is_null() && node != root {
            e = (*node).parents;
            while !e.is_null() {
                fsnodes_quota_fixspace((*e).parent as *mut fsnode, totalspace, availspace);
                e = (*e).nextparent as *mut fsedge;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_accessmode(
    mut node: *mut fsnode,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut sesflags: uint8_t,
) -> uint8_t {
    unsafe {
        static mut modetoaccmode: [uint8_t; 8] = MODE_TO_ACCMODE;
        let mut modemask: uint8_t = 0;
        if uid == 0 as uint32_t {
            return modetoaccmode[0x7 as usize];
        }
        modemask = (if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            && (*node).winattr as ::core::ffi::c_int & WINATTR_READ_ONLY != 0
        {
            0x33 as ::core::ffi::c_int
        } else {
            0xff as ::core::ffi::c_int
        }) as uint8_t;
        if (*node).aclpermflag() != 0 {
            return (modemask as ::core::ffi::c_int
                & posix_acl_accmode((*node).inode, uid, gids, gid, (*node).uid, (*node).gid)
                    as ::core::ffi::c_int) as uint8_t;
        } else if uid == (*node).uid || (*node).eattr as ::core::ffi::c_int & EATTR_NOOWNER != 0 {
            return (modemask as ::core::ffi::c_int
                & modetoaccmode[((*node).mode() as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                    & 7 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
                as uint8_t;
        } else if sesflags as ::core::ffi::c_int & SESFLAG_IGNOREGID != 0 {
            return (modemask as ::core::ffi::c_int
                & modetoaccmode[(((*node).mode() as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
                    | (*node).mode() as ::core::ffi::c_int)
                    & 7 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
                as uint8_t;
        } else {
            while gids > 0 as uint32_t {
                gids = gids.wrapping_sub(1);
                if *gid.offset(gids as isize) == (*node).gid {
                    return (modemask as ::core::ffi::c_int
                        & modetoaccmode[((*node).mode() as ::core::ffi::c_int
                            >> 3 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int) as uint8_t;
                }
            }
            return (modemask as ::core::ffi::c_int
                & modetoaccmode
                    [((*node).mode() as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as uint8_t;
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_access_ext(
    mut node: *mut fsnode,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut modemask: uint8_t,
    mut sesflags: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        return if fsnodes_accessmode(node, uid, gids, gid, sesflags) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << (modemask as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
            != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_sticky_access(
    mut parent: *mut fsnode,
    mut node: *mut fsnode,
    mut uid: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        if uid == 0 as uint32_t
            || (*parent).mode() as ::core::ffi::c_int & 0o1000 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        if uid == (*parent).uid
            || (*parent).eattr as ::core::ffi::c_int & EATTR_NOOWNER != 0
            || uid == (*node).uid
            || (*node).eattr as ::core::ffi::c_int & EATTR_NOOWNER != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_nlink(mut rootinode: uint32_t, mut node: *mut fsnode) -> uint32_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut nlink: uint32_t = 0;
        nlink = 0 as uint32_t;
        if (*node).inode != rootinode {
            if rootinode == MFS_ROOT_ID as uint32_t {
                e = (*node).parents;
                while !e.is_null() {
                    nlink = nlink.wrapping_add(1);
                    e = (*e).nextparent as *mut fsedge;
                }
            } else {
                e = (*node).parents;
                while !e.is_null() {
                    p = (*e).parent as *mut fsnode;
                    while !p.is_null() {
                        if rootinode == (*p).inode {
                            nlink = nlink.wrapping_add(1);
                            p = ::core::ptr::null_mut::<fsnode>();
                        } else if !(*p).parents.is_null() {
                            p = (*(*p).parents).parent as *mut fsnode;
                        } else {
                            p = ::core::ptr::null_mut::<fsnode>();
                        }
                    }
                    e = (*e).nextparent as *mut fsedge;
                }
            }
        }
        return nlink;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_get_parents(
    mut rootinode: uint32_t,
    mut node: *mut fsnode,
    mut buff: *mut uint8_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if (*node).inode != rootinode {
            if rootinode == MFS_ROOT_ID as uint32_t {
                e = (*node).parents;
                while !e.is_null() {
                    put32bit(&raw mut buff, (*(*e).parent).inode);
                    e = (*e).nextparent as *mut fsedge;
                }
            } else {
                e = (*node).parents;
                while !e.is_null() {
                    p = (*e).parent as *mut fsnode;
                    while !p.is_null() {
                        if rootinode == (*p).inode {
                            if (*(*e).parent).inode == rootinode {
                                put32bit(&raw mut buff, MFS_ROOT_ID as uint32_t);
                            } else {
                                put32bit(&raw mut buff, (*(*e).parent).inode);
                            }
                            p = ::core::ptr::null_mut::<fsnode>();
                        } else if !(*p).parents.is_null() {
                            p = (*(*p).parents).parent as *mut fsnode;
                        } else {
                            p = ::core::ptr::null_mut::<fsnode>();
                        }
                    }
                    e = (*e).nextparent as *mut fsedge;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_get_paths_size(
    mut rootinode: uint32_t,
    mut node: *mut fsnode,
) -> uint32_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut totalpsize: uint32_t = 0;
        let mut psize: uint32_t = 0;
        totalpsize = 0 as uint32_t;
        if (*node).inode != rootinode {
            e = (*node).parents;
            while !e.is_null() {
                psize = (*e).nleng as uint32_t;
                p = (*e).parent as *mut fsnode;
                while !p.is_null() {
                    if rootinode == (*p).inode {
                        totalpsize = totalpsize.wrapping_add(psize.wrapping_add(4 as uint32_t));
                        p = ::core::ptr::null_mut::<fsnode>();
                    } else if !(*p).parents.is_null() {
                        psize = psize.wrapping_add(
                            ((*(*p).parents).nleng as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as uint32_t,
                        );
                        p = (*(*p).parents).parent as *mut fsnode;
                    } else {
                        p = ::core::ptr::null_mut::<fsnode>();
                    }
                }
                e = (*e).nextparent as *mut fsedge;
            }
        } else {
            return 5 as uint32_t;
        }
        return totalpsize;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_get_paths_data(
    mut rootinode: uint32_t,
    mut node: *mut fsnode,
    mut buff: *mut uint8_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut psize: uint32_t = 0;
        let mut b: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*node).inode != rootinode {
            e = (*node).parents;
            while !e.is_null() {
                psize = (*e).nleng as uint32_t;
                p = (*e).parent as *mut fsnode;
                while !p.is_null() {
                    if rootinode == (*p).inode {
                        put32bit(&raw mut buff, psize);
                        b = buff;
                        buff = buff.offset(psize as isize);
                        psize = psize.wrapping_sub((*e).nleng as uint32_t);
                        memcpy(
                            b.offset(psize as isize) as *mut ::core::ffi::c_void,
                            &raw const (*e).name as *const uint8_t as *const ::core::ffi::c_void,
                            (*e).nleng as size_t,
                        );
                        p = (*e).parent as *mut fsnode;
                        while !p.is_null() {
                            if rootinode == (*p).inode {
                                p = ::core::ptr::null_mut::<fsnode>();
                            } else if !(*p).parents.is_null() {
                                psize = psize.wrapping_sub(1);
                                *b.offset(psize as isize) = '/' as uint8_t;
                                psize = psize.wrapping_sub((*(*p).parents).nleng as uint32_t);
                                memcpy(
                                    b.offset(psize as isize) as *mut ::core::ffi::c_void,
                                    &raw const (*(*p).parents).name as *const uint8_t
                                        as *const ::core::ffi::c_void,
                                    (*(*p).parents).nleng as size_t,
                                );
                                p = (*(*p).parents).parent as *mut fsnode;
                            } else {
                                p = ::core::ptr::null_mut::<fsnode>();
                            }
                        }
                    } else if !(*p).parents.is_null() {
                        psize = psize.wrapping_add(
                            ((*(*p).parents).nleng as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                as uint32_t,
                        );
                        p = (*(*p).parents).parent as *mut fsnode;
                    } else {
                        p = ::core::ptr::null_mut::<fsnode>();
                    }
                }
                e = (*e).nextparent as *mut fsedge;
            }
        } else {
            put32bit(&raw mut buff, 1 as uint32_t);
            *buff = '/' as uint8_t;
            return;
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_fill_attr(
    mut node: *mut fsnode,
    mut parent: *mut fsnode,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut sesflags: uint8_t,
    mut attr: *mut uint8_t,
    mut addwinattr: uint8_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut r#type: uint8_t = 0;
        let mut flags: uint8_t = 0;
        let mut mode: uint16_t = 0;
        let mut dleng: uint64_t = 0;
        ptr = attr as *mut uint8_t;
        r#type = (*node).r#type() as uint8_t;
        if r#type as ::core::ffi::c_int == TYPE_TRASH
            || r#type as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            r#type = TYPE_FILE as uint8_t;
        }
        flags = 0 as uint8_t;
        if !parent.is_null() {
            if (*parent).eattr as ::core::ffi::c_int & EATTR_NOECACHE != 0 {
                flags = (flags as ::core::ffi::c_int | MATTR_NOECACHE) as uint8_t;
            }
        }
        if (*node).eattr as ::core::ffi::c_int & (EATTR_NOOWNER | EATTR_NOACACHE) != 0
            || sesflags as ::core::ffi::c_int & SESFLAG_MAPALL != 0
        {
            flags = (flags as ::core::ffi::c_int | MATTR_NOACACHE) as uint8_t;
        }
        if (*node).eattr as ::core::ffi::c_int & EATTR_NODATACACHE == 0 as ::core::ffi::c_int {
            flags = (flags as ::core::ffi::c_int | MATTR_ALLOWDATACACHE) as uint8_t;
        } else {
            flags = (flags as ::core::ffi::c_int | MATTR_DIRECTMODE) as uint8_t;
        }
        if (*node).xattrflag() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*node).aclpermflag() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*node).acldefflag() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            flags = (flags as ::core::ffi::c_int | MATTR_NOXATTR) as uint8_t;
        }
        if (*node).eattr as ::core::ffi::c_int
            & (EATTR_UNDELETABLE | EATTR_IMMUTABLE | EATTR_APPENDONLY)
            != 0
        {
            flags = (flags as ::core::ffi::c_int | MATTR_UNDELETABLE) as uint8_t;
        }
        if (*node).aclpermflag() != 0 {
            mode = (posix_acl_getmode((*node).inode) as ::core::ffi::c_int
                & 0o777 as ::core::ffi::c_int
                | (*node).mode() as ::core::ffi::c_int & 0o7000 as ::core::ffi::c_int)
                as uint16_t;
        } else {
            mode =
                ((*node).mode() as ::core::ffi::c_int & 0o7777 as ::core::ffi::c_int) as uint16_t;
        }
        if (*node).eattr as ::core::ffi::c_int & EATTR_NOOWNER != 0 && uid != 0 as uint32_t {
            mode = (mode as ::core::ffi::c_int & 0o7700 as ::core::ffi::c_int) as uint16_t;
            mode = (mode as ::core::ffi::c_int
                | (mode as ::core::ffi::c_int & 0o700 as ::core::ffi::c_int)
                    >> 3 as ::core::ffi::c_int) as uint16_t;
            mode = (mode as ::core::ffi::c_int
                | (mode as ::core::ffi::c_int & 0o700 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int) as uint16_t;
            if sesflags as ::core::ffi::c_int & SESFLAG_MAPALL != 0 {
                uid = auid;
                gid = agid;
            }
        } else if sesflags as ::core::ffi::c_int & SESFLAG_MAPALL != 0 && auid != 0 as uint32_t {
            if (*node).uid == uid {
                uid = auid;
            } else {
                uid = 0 as uint32_t;
            }
            if (*node).gid == gid {
                gid = agid;
            } else {
                gid = 0 as uint32_t;
            }
        } else {
            uid = (*node).uid;
            gid = (*node).gid;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
            put8bit(&raw mut ptr, flags);
            mode = (mode as ::core::ffi::c_int
                | (r#type as uint16_t as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as uint16_t;
            put16bit(&raw mut ptr, mode);
        } else {
            put8bit(
                &raw mut ptr,
                DISP_TYPE_REMAP_STR[r#type as usize] as uint8_t,
            );
            mode = (mode as ::core::ffi::c_int
                | (flags as uint16_t as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                as uint16_t;
            put16bit(&raw mut ptr, mode);
        }
        put32bit(&raw mut ptr, uid);
        put32bit(&raw mut ptr, gid);
        put32bit(&raw mut ptr, (*node).atime);
        put32bit(&raw mut ptr, (*node).mtime);
        put32bit(&raw mut ptr, (*node).ctime);
        match (*node).r#type() as ::core::ffi::c_int {
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                put32bit(&raw mut ptr, (*node).data.fdata.nlink as uint32_t);
                put64bit(&raw mut ptr, (*node).data.fdata.length);
            }
            TYPE_DIRECTORY => {
                dleng = (*node).data.ddata.stats.length;
                if dleng == 0 as uint64_t {
                    dleng = 1 as uint64_t;
                } else if dleng < 0x400 as uint64_t {
                    dleng = dleng.wrapping_mul(100 as uint64_t);
                } else if dleng < 0x100000 as uint64_t {
                    dleng = dleng.wrapping_mul(100 as uint64_t);
                    dleng >>= 10 as ::core::ffi::c_int;
                    dleng = dleng.wrapping_add(1000000 as uint64_t);
                } else if dleng < 0x40000000 as uint64_t {
                    dleng = dleng.wrapping_mul(100 as uint64_t);
                    dleng >>= 20 as ::core::ffi::c_int;
                    dleng = dleng.wrapping_add(2000000 as uint64_t);
                } else if dleng < 0x10000000000 as uint64_t {
                    dleng = dleng.wrapping_mul(100 as uint64_t);
                    dleng >>= 30 as ::core::ffi::c_int;
                    dleng = dleng.wrapping_add(3000000 as uint64_t);
                } else if dleng < 0x4000000000000 as uint64_t {
                    dleng = dleng.wrapping_mul(100 as uint64_t);
                    dleng >>= 40 as ::core::ffi::c_int;
                    dleng = dleng.wrapping_add(4000000 as uint64_t);
                } else if dleng < 0x1000000000000000 as uint64_t {
                    dleng >>= 10 as ::core::ffi::c_int;
                    dleng = dleng.wrapping_mul(100 as uint64_t);
                    dleng >>= 40 as ::core::ffi::c_int;
                    dleng = dleng.wrapping_add(5000000 as uint64_t);
                } else {
                    dleng >>= 10 as ::core::ffi::c_int;
                    dleng = dleng.wrapping_mul(100 as uint64_t);
                    dleng >>= 50 as ::core::ffi::c_int;
                    dleng = dleng.wrapping_add(6000000 as uint64_t);
                }
                put32bit(&raw mut ptr, (*node).data.ddata.nlink);
                put64bit(&raw mut ptr, dleng);
            }
            TYPE_SYMLINK => {
                put32bit(&raw mut ptr, (*node).data.sdata.nlink as uint32_t);
                let c2rust_fresh0 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh0 = 0 as uint8_t;
                let c2rust_fresh1 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh1 = 0 as uint8_t;
                let c2rust_fresh2 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh2 = 0 as uint8_t;
                let c2rust_fresh3 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh3 = 0 as uint8_t;
                put32bit(&raw mut ptr, (*node).data.sdata.pleng);
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                put32bit(&raw mut ptr, (*node).data.devdata.nlink as uint32_t);
                put32bit(&raw mut ptr, (*node).data.devdata.rdev);
                let c2rust_fresh4 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh4 = 0 as uint8_t;
                let c2rust_fresh5 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh5 = 0 as uint8_t;
                let c2rust_fresh6 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh6 = 0 as uint8_t;
                let c2rust_fresh7 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh7 = 0 as uint8_t;
            }
            _ => {
                put32bit(&raw mut ptr, (*node).data.odata.nlink as uint32_t);
                let c2rust_fresh8 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh8 = 0 as uint8_t;
                let c2rust_fresh9 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh9 = 0 as uint8_t;
                let c2rust_fresh10 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh10 = 0 as uint8_t;
                let c2rust_fresh11 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh11 = 0 as uint8_t;
                let c2rust_fresh12 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh12 = 0 as uint8_t;
                let c2rust_fresh13 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh13 = 0 as uint8_t;
                let c2rust_fresh14 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh14 = 0 as uint8_t;
                let c2rust_fresh15 = ptr;
                ptr = ptr.offset(1);
                *c2rust_fresh15 = 0 as uint8_t;
            }
        }
        if addwinattr != 0 {
            put8bit(&raw mut ptr, (*node).winattr);
        }
    }
}
pub const CHECK_CTIME: ::core::ffi::c_int = SCLASS_ARCH_MODE_CTIME;
pub const CHECK_MTIME: ::core::ffi::c_int = SCLASS_ARCH_MODE_MTIME;
pub const CHECK_ATIME: ::core::ffi::c_int = SCLASS_ARCH_MODE_ATIME;
#[inline]
unsafe extern "C" fn fsnodes_checkarchmode(
    mut obj: *mut fsnode,
    mut ts: uint32_t,
    mut fields_to_check: uint8_t,
) -> uint32_t {
    unsafe {
        let mut aflagchanged: uint32_t = 0;
        let mut arch_delay_sec: uint32_t = 0;
        let mut arch_delay: uint16_t = 0;
        let mut arch_min_size: uint64_t = 0;
        let mut arch_mode: uint8_t = 0;
        let mut aflag: uint8_t = 0;
        let mut i: uint32_t = 0;
        let mut reftime: uint32_t = 0;
        let mut arch_chk_time: uint32_t = 0;
        aflagchanged = 0 as uint32_t;
        if (*obj).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*obj).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*obj).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            if (*obj).keepmode() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                arch_mode = sclass_get_arch_mode((*obj).sclassid as uint16_t);
                arch_delay = sclass_get_arch_delay((*obj).sclassid as uint16_t);
                arch_min_size = sclass_get_arch_min_size((*obj).sclassid as uint16_t);
                if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_REVERSIBLE != 0
                    && ((arch_delay as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        || arch_min_size > 0 as uint64_t)
                        && arch_mode as ::core::ffi::c_int & fields_to_check as ::core::ffi::c_int
                            != 0)
                {
                    if (*obj).data.fdata.length < arch_min_size {
                        aflag = 0 as uint8_t;
                    } else {
                        reftime = 0 as uint32_t;
                        if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_CTIME != 0 {
                            if (*obj).ctime > reftime {
                                reftime = (*obj).ctime;
                            }
                        }
                        if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_MTIME != 0 {
                            if (*obj).mtime > reftime {
                                reftime = (*obj).mtime;
                            }
                        }
                        if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_ATIME != 0 {
                            if (*obj).atime > reftime {
                                reftime = (*obj).atime;
                            }
                        }
                        arch_delay_sec = (arch_delay as uint32_t).wrapping_mul(3600 as uint32_t);
                        arch_chk_time = reftime.wrapping_add(arch_delay_sec);
                        aflag = (if arch_chk_time < ts && arch_chk_time >= reftime {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as uint8_t;
                    }
                    if aflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        i = 0 as uint32_t;
                        while i < (*obj).data.fdata.chunks {
                            if *(*obj).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t {
                                chunk_set_archflag(
                                    *(*obj).data.fdata.chunktab.offset(i as isize),
                                    aflag,
                                    &raw mut aflagchanged,
                                );
                            }
                            i = i.wrapping_add(1);
                        }
                        (*obj).set_keepmode(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                }
            }
        }
        return aflagchanged;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_remove_edge(mut ts: uint32_t, mut e: *mut fsedge) {
    unsafe {
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        if !(*e).parent.is_null() {
            fsnodes_edgeid_remove(e);
            fsnodes_get_stats((*e).child as *mut fsnode, &raw mut sr, 0 as uint8_t);
            fsnodes_sub_stats((*e).parent as *mut fsnode, &raw mut sr);
            (*(*e).parent).ctime = ts;
            (*(*e).parent).mtime = (*(*e).parent).ctime;
            (*(*e).parent).data.ddata.elements = (*(*e).parent).data.ddata.elements.wrapping_sub(1);
            match (*(*e).child).r#type() as ::core::ffi::c_int {
                TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                    (*(*e).child).data.fdata.nlink = (*(*e).child).data.fdata.nlink.wrapping_sub(1);
                }
                TYPE_DIRECTORY => {
                    (*(*e).parent).data.ddata.nlink =
                        (*(*e).parent).data.ddata.nlink.wrapping_sub(1);
                }
                TYPE_SYMLINK => {
                    (*(*e).child).data.sdata.nlink = (*(*e).child).data.sdata.nlink.wrapping_sub(1);
                }
                TYPE_BLOCKDEV | TYPE_CHARDEV => {
                    (*(*e).child).data.devdata.nlink =
                        (*(*e).child).data.devdata.nlink.wrapping_sub(1);
                }
                _ => {
                    (*(*e).child).data.odata.nlink = (*(*e).child).data.odata.nlink.wrapping_sub(1);
                }
            }
            (*(*e).parent).eattr =
                ((*(*e).parent).eattr as ::core::ffi::c_int & !EATTR_SNAPSHOT) as uint8_t;
        }
        if ts > 0 as uint32_t && !(*e).child.is_null() {
            (*(*e).child).ctime = ts;
            fsnodes_checkarchmode((*e).child as *mut fsnode, ts, CHECK_CTIME as uint8_t);
        }
        *(*e).prevchild = (*e).nextchild;
        if !(*e).nextchild.is_null() {
            (*(*e).nextchild).prevchild = (*e).prevchild;
        }
        *(*e).prevparent = (*e).nextparent;
        if !(*e).nextparent.is_null() {
            (*(*e).nextparent).prevparent = (*e).prevparent;
        }
        if !(*e).parent.is_null() {
            fsnodes_edge_delete(e);
        }
        fsedge_free(e, (*e).nleng);
    }
}
#[inline]
unsafe extern "C" fn fsnodes_link(
    mut ts: uint32_t,
    mut parent: *mut fsnode,
    mut child: *mut fsnode,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        e = fsedge_malloc(nleng);
        if e.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"e\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"e\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if e
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut fsedge
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"e\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2479 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"e\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        if nextedgeid < 0x7fffffffffffffff as uint64_t {
            let c2rust_fresh16 = nextedgeid;
            nextedgeid = nextedgeid.wrapping_sub(1);
            (*e).edgeid = c2rust_fresh16;
        } else {
            (*e).edgeid = 0 as uint64_t;
        }
        (*e).nleng = nleng;
        memcpy(
            &raw const (*e).name as *const uint8_t as *mut uint8_t as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        (*e).child = child as *mut _fsnode;
        (*e).parent = parent as *mut _fsnode;
        (*e).nextchild = (*parent).data.ddata.children as *mut _fsedge;
        if !(*e).nextchild.is_null() {
            (*(*e).nextchild).prevchild = &raw mut (*e).nextchild;
        }
        (*parent).data.ddata.children = e;
        (*e).prevchild = &raw mut (*parent).data.ddata.children as *mut *mut _fsedge;
        (*e).nextparent = (*child).parents as *mut _fsedge;
        if !(*e).nextparent.is_null() {
            (*(*e).nextparent).prevparent = &raw mut (*e).nextparent;
        }
        (*child).parents = e;
        (*e).prevparent = &raw mut (*child).parents as *mut *mut _fsedge;
        fsnodes_edge_add(e);
        (*parent).data.ddata.elements = (*parent).data.ddata.elements.wrapping_add(1);
        match (*child).r#type() as ::core::ffi::c_int {
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                (*child).data.fdata.nlink = (*child).data.fdata.nlink.wrapping_add(1);
            }
            TYPE_DIRECTORY => {
                (*parent).data.ddata.nlink = (*parent).data.ddata.nlink.wrapping_add(1);
            }
            TYPE_SYMLINK => {
                (*child).data.sdata.nlink = (*child).data.sdata.nlink.wrapping_add(1);
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                (*child).data.devdata.nlink = (*child).data.devdata.nlink.wrapping_add(1);
            }
            _ => {
                (*child).data.odata.nlink = (*child).data.odata.nlink.wrapping_add(1);
            }
        }
        (*parent).eattr = ((*parent).eattr as ::core::ffi::c_int & !EATTR_SNAPSHOT) as uint8_t;
        fsnodes_get_stats(child, &raw mut sr, 1 as uint8_t);
        fsnodes_add_stats(parent, &raw mut sr);
        if ts > 0 as uint32_t {
            (*parent).ctime = ts;
            (*parent).mtime = (*parent).ctime;
            (*child).ctime = ts;
            fsnodes_checkarchmode(child, ts, CHECK_CTIME as uint8_t);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fsnodes_create_node(
    mut ts: uint32_t,
    mut node: *mut fsnode,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut r#type: uint8_t,
    mut mode: uint16_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut copysgid: uint8_t,
) -> *mut fsnode {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut aclcopied: uint8_t = 0;
        match r#type as ::core::ffi::c_int {
            TYPE_DIRECTORY => {
                p = fsnode_malloc(0 as uint8_t);
            }
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                p = fsnode_malloc(1 as uint8_t);
            }
            TYPE_SYMLINK => {
                p = fsnode_malloc(2 as uint8_t);
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                p = fsnode_malloc(3 as uint8_t);
            }
            _ => {
                p = fsnode_malloc(4 as uint8_t);
            }
        }
        if p.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if p
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut fsnode
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2559 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        nodes = nodes.wrapping_add(1);
        if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
            dirnodes = dirnodes.wrapping_add(1);
        }
        if r#type as ::core::ffi::c_int == TYPE_FILE {
            filenodes = filenodes.wrapping_add(1);
        }
        (*p).inode = fsnodes_get_next_id();
        (*p).set_xattrflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).set_aclpermflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).set_acldefflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).set_keepmode(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).set_type(r#type as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).atime = ts;
        (*p).mtime = (*p).atime;
        (*p).ctime = (*p).mtime;
        if r#type as ::core::ffi::c_int == TYPE_DIRECTORY
            || r#type as ::core::ffi::c_int == TYPE_FILE
        {
            (*p).sclassid = (*node).sclassid;
            sclass_incref((*p).sclassid as uint16_t, (*p).r#type() as uint8_t);
            (*p).trashretention = (*node).trashretention;
        } else {
            (*p).sclassid = 0 as uint8_t;
            sclass_incref((*p).sclassid as uint16_t, (*p).r#type() as uint8_t);
            (*p).trashretention = DEFAULT_TRASHTIME as uint16_t;
        }
        if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
            (*p).eattr = ((*node).eattr as ::core::ffi::c_int
                & !(EATTR_SNAPSHOT | EATTR_UNDELETABLE | EATTR_APPENDONLY | EATTR_IMMUTABLE))
                as uint8_t;
        } else {
            (*p).eattr = ((*node).eattr as ::core::ffi::c_int
                & !(EATTR_NOECACHE
                    | EATTR_SNAPSHOT
                    | EATTR_UNDELETABLE
                    | EATTR_APPENDONLY
                    | EATTR_IMMUTABLE)) as uint8_t;
        }
        (*p).winattr = 0 as uint8_t;
        if (*node).acldefflag() != 0 {
            aclcopied = posix_acl_copydefaults(
                (*node).inode,
                (*p).inode,
                (if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t,
                &raw mut mode,
            );
            (*p).set_mode(mode as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else {
            aclcopied = 0 as uint8_t;
            (*p).set_mode(
                (mode as ::core::ffi::c_int & !(cumask as ::core::ffi::c_int))
                    as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
        }
        (*p).uid = uid;
        if (*node).mode() as ::core::ffi::c_int & 0o2000 as ::core::ffi::c_int
            == 0o2000 as ::core::ffi::c_int
        {
            (*p).gid = (*node).gid;
            if copysgid as ::core::ffi::c_int != 0 && r#type as ::core::ffi::c_int == TYPE_DIRECTORY
            {
                (*p).set_mode((*p).mode() | 0o2000 as ::core::ffi::c_int as ::core::ffi::c_uint);
            }
        } else {
            (*p).gid = gid;
        }
        match r#type as ::core::ffi::c_int {
            TYPE_DIRECTORY => {
                memset(
                    &raw mut (*p).data.ddata.stats as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<statsrecord>(),
                );
                (*p).data.ddata.quota = ::core::ptr::null_mut::<quotanode>();
                (*p).data.ddata.children = ::core::ptr::null_mut::<fsedge>();
                (*p).data.ddata.nlink = 2 as uint32_t;
                (*p).data.ddata.elements = 0 as uint32_t;
            }
            TYPE_TRASH | TYPE_SUSTAINED | TYPE_FILE => {
                (*p).data.fdata.length = 0 as uint64_t;
                (*p).data.fdata.chunks = 0 as uint32_t;
                (*p).data.fdata.chunktab = ::core::ptr::null_mut::<uint64_t>();
                (*p).data.fdata.nlink = 0 as uint16_t;
            }
            TYPE_SYMLINK => {
                (*p).data.sdata.pleng = 0 as uint32_t;
                (*p).data.sdata.path = ::core::ptr::null_mut::<uint8_t>();
                (*p).data.sdata.nlink = 0 as uint16_t;
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                (*p).data.devdata.rdev = 0 as uint32_t;
                (*p).data.devdata.nlink = 0 as uint16_t;
            }
            _ => {
                (*p).data.odata.nlink = 0 as uint16_t;
            }
        }
        (*p).parents = ::core::ptr::null_mut::<fsedge>();
        fsnodes_node_add(p);
        fsnodes_link(ts, node, p, nleng, name);
        if aclcopied as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            (*p).set_aclpermflag(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if aclcopied as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
            (*p).set_acldefflag(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        return p;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_getpath_size(mut e: *mut fsedge) -> uint32_t {
    unsafe {
        let mut size: uint32_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if e.is_null() {
            return 0 as uint32_t;
        }
        p = (*e).parent as *mut fsnode;
        size = (*e).nleng as uint32_t;
        while p != root && !(*p).parents.is_null() {
            size = size.wrapping_add(
                ((*(*p).parents).nleng as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
            p = (*(*p).parents).parent as *mut fsnode;
        }
        return size;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_getpath_data(
    mut e: *mut fsedge,
    mut path: *mut uint8_t,
    mut size: uint32_t,
) {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if e.is_null() {
            return;
        }
        if size >= (*e).nleng as uint32_t {
            size = size.wrapping_sub((*e).nleng as uint32_t);
            memcpy(
                path.offset(size as isize) as *mut ::core::ffi::c_void,
                &raw const (*e).name as *const uint8_t as *const ::core::ffi::c_void,
                (*e).nleng as size_t,
            );
        } else if size > 0 as uint32_t {
            memcpy(
                path as *mut ::core::ffi::c_void,
                (&raw const (*e).name as *const uint8_t)
                    .offset(((*e).nleng as uint32_t).wrapping_sub(size) as isize)
                    as *const ::core::ffi::c_void,
                size as size_t,
            );
            size = 0 as uint32_t;
        }
        if size > 0 as uint32_t {
            size = size.wrapping_sub(1);
            *path.offset(size as isize) = '/' as uint8_t;
        }
        p = (*e).parent as *mut fsnode;
        while p != root && !(*p).parents.is_null() {
            if size >= (*(*p).parents).nleng as uint32_t {
                size = size.wrapping_sub((*(*p).parents).nleng as uint32_t);
                memcpy(
                    path.offset(size as isize) as *mut ::core::ffi::c_void,
                    &raw const (*(*p).parents).name as *const uint8_t as *const ::core::ffi::c_void,
                    (*(*p).parents).nleng as size_t,
                );
            } else if size > 0 as uint32_t {
                memcpy(
                    path as *mut ::core::ffi::c_void,
                    (&raw const (*(*p).parents).name as *const uint8_t)
                        .offset(((*(*p).parents).nleng as uint32_t).wrapping_sub(size) as isize)
                        as *const ::core::ffi::c_void,
                    size as size_t,
                );
                size = 0 as uint32_t;
            }
            if size > 0 as uint32_t {
                size = size.wrapping_sub(1);
                *path.offset(size as isize) = '/' as uint8_t;
            }
            p = (*(*p).parents).parent as *mut fsnode;
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_getpath(
    mut e: *mut fsedge,
    mut pleng: *mut uint16_t,
    mut path: *mut *mut uint8_t,
) {
    unsafe {
        let mut size: uint32_t = 0;
        let mut ret: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = (*e).parent as *mut fsnode;
        size = (*e).nleng as uint32_t;
        while p != root && !(*p).parents.is_null() {
            size = size.wrapping_add(
                ((*(*p).parents).nleng as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            );
            p = (*(*p).parents).parent as *mut fsnode;
        }
        if size > MFS_PATH_MAX as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"path too long !!! - truncate\0".as_ptr() as *const ::core::ffi::c_char,
            );
            size = MFS_PATH_MAX as uint32_t;
        }
        *pleng = size as uint16_t;
        ret = malloc(size as size_t) as *mut uint8_t;
        if ret.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2709 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ret\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2709 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ret\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if ret
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2709 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ret\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                2709 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ret\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        size = size.wrapping_sub((*e).nleng as uint32_t);
        memcpy(
            ret.offset(size as isize) as *mut ::core::ffi::c_void,
            &raw const (*e).name as *const uint8_t as *const ::core::ffi::c_void,
            (*e).nleng as size_t,
        );
        if size > 0 as uint32_t {
            size = size.wrapping_sub(1);
            *ret.offset(size as isize) = '/' as uint8_t;
        }
        p = (*e).parent as *mut fsnode;
        while p != root && !(*p).parents.is_null() {
            if size >= (*(*p).parents).nleng as uint32_t {
                size = size.wrapping_sub((*(*p).parents).nleng as uint32_t);
                memcpy(
                    ret.offset(size as isize) as *mut ::core::ffi::c_void,
                    &raw const (*(*p).parents).name as *const uint8_t as *const ::core::ffi::c_void,
                    (*(*p).parents).nleng as size_t,
                );
            } else if size > 0 as uint32_t {
                memcpy(
                    ret as *mut ::core::ffi::c_void,
                    (&raw const (*(*p).parents).name as *const uint8_t)
                        .offset(((*(*p).parents).nleng as uint32_t).wrapping_sub(size) as isize)
                        as *const ::core::ffi::c_void,
                    size as size_t,
                );
                size = 0 as uint32_t;
            }
            if size > 0 as uint32_t {
                size = size.wrapping_sub(1);
                *ret.offset(size as isize) = '/' as uint8_t;
            }
            p = (*(*p).parents).parent as *mut fsnode;
        }
        *path = ret;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_getdetached(
    mut start: *mut fsedge,
    mut dbuff: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut sptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut c: uint8_t = 0;
        let mut result: uint32_t = 0 as uint32_t;
        e = start;
        while !e.is_null() {
            if (*e).nleng as ::core::ffi::c_int > 240 as ::core::ffi::c_int {
                if !dbuff.is_null() {
                    *dbuff = 240 as uint8_t;
                    dbuff = dbuff.offset(1);
                    memcpy(
                        dbuff as *mut ::core::ffi::c_void,
                        b"(...)\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        5 as size_t,
                    );
                    dbuff = dbuff.offset(5 as ::core::ffi::c_int as isize);
                    sptr = (&raw const (*e).name as *const uint8_t).offset(
                        ((*e).nleng as ::core::ffi::c_int - 235 as ::core::ffi::c_int) as isize,
                    );
                    c = 0 as uint8_t;
                    while (c as ::core::ffi::c_int) < 235 as ::core::ffi::c_int {
                        if *sptr as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                            *dbuff = '|' as uint8_t;
                        } else {
                            *dbuff = *sptr;
                        }
                        sptr = sptr.offset(1);
                        dbuff = dbuff.offset(1);
                        c = c.wrapping_add(1);
                    }
                }
                result = result.wrapping_add(245 as uint32_t);
            } else {
                if !dbuff.is_null() {
                    *dbuff = (*e).nleng as uint8_t;
                    dbuff = dbuff.offset(1);
                    sptr = &raw const (*e).name as *const uint8_t;
                    c = 0 as uint8_t;
                    while (c as ::core::ffi::c_int) < (*e).nleng as ::core::ffi::c_int {
                        if *sptr as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                            *dbuff = '|' as uint8_t;
                        } else {
                            *dbuff = *sptr;
                        }
                        sptr = sptr.offset(1);
                        dbuff = dbuff.offset(1);
                        c = c.wrapping_add(1);
                    }
                }
                result = result.wrapping_add(
                    (5 as ::core::ffi::c_int + (*e).nleng as ::core::ffi::c_int) as uint32_t,
                );
            }
            if !dbuff.is_null() {
                put32bit(&raw mut dbuff, (*(*e).child).inode);
            }
            e = (*e).nextchild as *mut fsedge;
        }
        return result;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_listtrash(
    mut start: *mut fsedge,
    mut format: uint8_t,
    mut uid: uint32_t,
    mut mints: uint32_t,
    mut maxts: uint32_t,
    mut glob: *mut ::core::ffi::c_void,
    mut total: *mut uint32_t,
    mut userfiles: *mut uint32_t,
    mut matchfiles: *mut uint32_t,
    mut dbuff: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut maxtime: uint32_t = 0;
        let mut size: uint32_t = 0 as uint32_t;
        e = start;
        while !e.is_null() {
            p = (*e).child as *mut fsnode;
            *total = (*total).wrapping_add(1);
            maxtime = (*p).ctime;
            if (*p).atime > maxtime {
                maxtime = (*p).atime;
            }
            if (*p).mtime > maxtime {
                maxtime = (*p).mtime;
            }
            if uid == 0xffffffff as uint32_t || uid == (*p).uid {
                *userfiles = (*userfiles).wrapping_add(1);
                if maxtime >= mints
                    && maxtime <= maxts
                    && glob_match(
                        glob,
                        &raw const (*e).name as *const uint8_t,
                        (*e).nleng as uint8_t,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    *matchfiles = (*matchfiles).wrapping_add(1);
                    size = size.wrapping_add(
                        (4 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + (*e).nleng as ::core::ffi::c_int) as uint32_t,
                    );
                    if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                        size = size
                            .wrapping_add((ATTR_RECORD_SIZE + 2 as ::core::ffi::c_int) as uint32_t);
                    }
                    if !dbuff.is_null() {
                        put32bit(&raw mut dbuff, (*p).inode);
                        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                            fsnodes_fill_attr(
                                p,
                                ::core::ptr::null_mut::<fsnode>(),
                                (*p).uid,
                                (*p).gid,
                                (*p).uid,
                                (*p).gid,
                                SESFLAG_ATTRBIT as uint8_t,
                                dbuff as *mut uint8_t,
                                1 as uint8_t,
                            );
                            dbuff = dbuff.offset(ATTR_RECORD_SIZE as isize);
                            put16bit(&raw mut dbuff, (*p).trashretention);
                        }
                        put32bit(&raw mut dbuff, (*e).nleng as uint32_t);
                        memcpy(
                            dbuff as *mut ::core::ffi::c_void,
                            &raw const (*e).name as *const uint8_t as *const ::core::ffi::c_void,
                            (*e).nleng as size_t,
                        );
                        dbuff = dbuff.offset((*e).nleng as ::core::ffi::c_int as isize);
                    }
                }
            }
            e = (*e).nextchild as *mut fsedge;
        }
        return size;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_listsustained(
    mut start: *mut fsedge,
    mut uid: uint32_t,
    mut glob: *mut ::core::ffi::c_void,
    mut total: *mut uint32_t,
    mut matchfiles: *mut uint32_t,
    mut dbuff: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut size: uint32_t = 0 as uint32_t;
        let mut l: uint32_t = 0;
        e = start;
        while !e.is_null() {
            p = (*e).child as *mut fsnode;
            *total = (*total).wrapping_add(1);
            if (uid == 0xffffffff as uint32_t || uid == (*p).uid)
                && glob_match(
                    glob,
                    &raw const (*e).name as *const uint8_t,
                    (*e).nleng as uint8_t,
                ) as ::core::ffi::c_int
                    != 0
            {
                *matchfiles = (*matchfiles).wrapping_add(1);
                if !dbuff.is_null() {
                    put32bit(&raw mut dbuff, (*p).inode);
                }
                l = of_sessions_info_for_inode((*p).inode, dbuff);
                if !dbuff.is_null() {
                    dbuff = dbuff.offset(l as isize);
                    put32bit(&raw mut dbuff, (*e).nleng as uint32_t);
                    memcpy(
                        dbuff as *mut ::core::ffi::c_void,
                        &raw const (*e).name as *const uint8_t as *const ::core::ffi::c_void,
                        (*e).nleng as size_t,
                    );
                    dbuff = dbuff.offset((*e).nleng as ::core::ffi::c_int as isize);
                }
                size = size.wrapping_add(
                    ((8 as ::core::ffi::c_int + (*e).nleng as ::core::ffi::c_int) as uint32_t)
                        .wrapping_add(l),
                );
            }
            e = (*e).nextchild as *mut fsedge;
        }
        return size;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_readdirfull(
    mut e: *mut fsedge,
    mut flags: uint8_t,
    mut maxentries: uint32_t,
    mut dbuff: *mut uint8_t,
) -> uint64_t {
    unsafe {
        let mut result: uint64_t = 0 as uint64_t;
        let mut i: uint32_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        while maxentries > 0 as uint32_t && !e.is_null() {
            p = (*e).child as *mut fsnode;
            if dbuff.is_null() {
                result = result.wrapping_add(
                    (5 as ::core::ffi::c_int + ATTR_RECORD_SIZE + (*e).nleng as ::core::ffi::c_int)
                        as uint64_t,
                );
            } else {
                *dbuff.offset(0 as isize) = (*e).nleng as uint8_t;
                dbuff = dbuff.offset(1);
                memcpy(
                    dbuff as *mut ::core::ffi::c_void,
                    &raw const (*e).name as *const uint8_t as *const ::core::ffi::c_void,
                    (*e).nleng as size_t,
                );
                dbuff = dbuff.offset((*e).nleng as ::core::ffi::c_int as isize);
                put32bit(&raw mut dbuff, (*p).inode);
                fsnodes_fill_attr(
                    p,
                    ::core::ptr::null_mut::<fsnode>(),
                    0 as uint32_t,
                    0 as uint32_t,
                    0 as uint32_t,
                    0 as uint32_t,
                    SESFLAG_ATTRBIT as uint8_t,
                    dbuff as *mut uint8_t,
                    1 as uint8_t,
                );
                dbuff = dbuff.offset(ATTR_RECORD_SIZE as isize);
            }
            if (*p).r#type() as ::core::ffi::c_int == TYPE_FILE
                && flags as ::core::ffi::c_int & FULL_DIRECTORY_ADD_CHUNKID != 0
            {
                if dbuff.is_null() {
                    result = result.wrapping_add(
                        (4 as uint32_t)
                            .wrapping_add((8 as uint32_t).wrapping_mul((*p).data.fdata.chunks))
                            as uint64_t,
                    );
                } else {
                    put32bit(&raw mut dbuff, (*p).data.fdata.chunks);
                    i = 0 as uint32_t;
                    while i < (*p).data.fdata.chunks {
                        put64bit(&raw mut dbuff, *(*p).data.fdata.chunktab.offset(i as isize));
                        i = i.wrapping_add(1);
                    }
                }
            }
            if (*p).r#type() as ::core::ffi::c_int == TYPE_SYMLINK
                && flags as ::core::ffi::c_int & FULL_DIRECTORY_ADD_SYMLINK != 0
            {
                if dbuff.is_null() {
                    result = result.wrapping_add((*p).data.sdata.pleng as uint64_t);
                } else {
                    memcpy(
                        dbuff as *mut ::core::ffi::c_void,
                        (*p).data.sdata.path as *const ::core::ffi::c_void,
                        (*p).data.sdata.pleng as size_t,
                    );
                    dbuff = dbuff.offset((*p).data.sdata.pleng as isize);
                }
            }
            if flags as ::core::ffi::c_int & FULL_DIRECTORY_ADD_EATTR != 0 {
                if dbuff.is_null() {
                    result = result.wrapping_add(1);
                } else {
                    put8bit(&raw mut dbuff, (*p).eattr);
                }
            }
            if flags as ::core::ffi::c_int & FULL_DIRECTORY_ADD_XATTR != 0 {
                if (*p).xattrflag() != 0 {
                    let mut leng: uint32_t = 0;
                    leng = xattr_getall((*p).inode, dbuff);
                    if dbuff.is_null() {
                        result = result.wrapping_add(leng as uint64_t);
                    } else {
                        dbuff = dbuff.offset(leng as isize);
                    }
                } else if dbuff.is_null() {
                    result = result.wrapping_add(2 as uint64_t);
                } else {
                    put16bit(&raw mut dbuff, 0 as uint16_t);
                }
            }
            if flags as ::core::ffi::c_int & FULL_DIRECTORY_ADD_FACL != 0 {
                if (*p).aclpermflag() as ::core::ffi::c_int != 0
                    || (*p).acldefflag() as ::core::ffi::c_int != 0
                {
                    let mut aclmask_ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mut aclmask: uint8_t = 0;
                    let mut leng_0: uint32_t = 0;
                    aclmask = 0 as uint8_t;
                    result = result.wrapping_add(1 as uint64_t);
                    aclmask_ptr = dbuff;
                    if !dbuff.is_null() {
                        dbuff = dbuff.offset(1 as ::core::ffi::c_int as isize);
                    }
                    if (*p).aclpermflag() != 0 {
                        leng_0 = posix_acl_getall((*p).inode, POSIX_ACL_ACCESS as uint8_t, dbuff);
                        if leng_0 > 0 as uint32_t {
                            if dbuff.is_null() {
                                result = result.wrapping_add(leng_0 as uint64_t);
                            } else {
                                dbuff = dbuff.offset(leng_0 as isize);
                            }
                            aclmask = (aclmask as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
                                as uint8_t;
                        }
                    }
                    if (*p).acldefflag() != 0 {
                        leng_0 = posix_acl_getall((*p).inode, POSIX_ACL_DEFAULT as uint8_t, dbuff);
                        if leng_0 > 0 as uint32_t {
                            if dbuff.is_null() {
                                result = result.wrapping_add(leng_0 as uint64_t);
                            } else {
                                dbuff = dbuff.offset(leng_0 as isize);
                            }
                            aclmask = (aclmask as ::core::ffi::c_int | 2 as ::core::ffi::c_int)
                                as uint8_t;
                        }
                    }
                    if !aclmask_ptr.is_null() {
                        put8bit(&raw mut aclmask_ptr, aclmask);
                    }
                } else if dbuff.is_null() {
                    result = result.wrapping_add(1 as uint64_t);
                } else {
                    put8bit(&raw mut dbuff, 0 as uint8_t);
                }
            }
            e = (*e).nextchild as *mut fsedge;
            maxentries = maxentries.wrapping_sub(1);
        }
        if !dbuff.is_null() {
            if !e.is_null() {
                return (*e).edgeid;
            } else {
                return 0x7fffffffffffffff as uint64_t;
            }
        } else {
            return result;
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_readdirsize(
    mut p: *mut fsnode,
    mut e: *mut fsedge,
    mut maxentries: uint32_t,
    mut nedgeid: uint64_t,
    mut attrmode: uint8_t,
) -> uint64_t {
    unsafe {
        let mut result: uint64_t = 0 as uint64_t;
        let mut attrsize: uint8_t = (if attrmode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else if attrmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            35 as ::core::ffi::c_int
        } else {
            ATTR_RECORD_SIZE
        }) as uint8_t;
        while maxentries > 0 as uint32_t && nedgeid < 0x7fffffffffffffff as uint64_t {
            if nedgeid == 0 as uint64_t {
                result = result.wrapping_add(
                    (attrsize as ::core::ffi::c_int
                        + 5 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as uint64_t,
                );
                nedgeid = 1 as uint64_t;
            } else {
                if nedgeid == 1 as uint64_t {
                    result = result.wrapping_add(
                        (attrsize as ::core::ffi::c_int
                            + 5 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int) as uint64_t,
                    );
                    e = (*p).data.ddata.children;
                } else if !e.is_null() {
                    result = result.wrapping_add(
                        (attrsize as ::core::ffi::c_int
                            + 5 as ::core::ffi::c_int
                            + (*e).nleng as ::core::ffi::c_int) as uint64_t,
                    );
                    e = (*e).nextchild as *mut fsedge;
                }
                if !e.is_null() {
                    nedgeid = (*e).edgeid;
                } else {
                    nedgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                }
            }
            maxentries = maxentries.wrapping_sub(1);
        }
        return result;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_readdirdata(
    mut rootinode: uint32_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut sesflags: uint8_t,
    mut p: *mut fsnode,
    mut e: *mut fsedge,
    mut maxentries: uint32_t,
    mut nedgeidp: *mut uint64_t,
    mut dbuff: *mut uint8_t,
    mut attrmode: uint8_t,
) {
    unsafe {
        let mut nedgeid: uint64_t = *nedgeidp;
        while maxentries > 0 as uint32_t && nedgeid < 0x7fffffffffffffff as uint64_t {
            if nedgeid == 0 as uint64_t {
                *dbuff.offset(0 as isize) = 1 as uint8_t;
                *dbuff.offset(1 as isize) = '.' as uint8_t;
                dbuff = dbuff.offset(2 as ::core::ffi::c_int as isize);
                if (*p).inode != rootinode {
                    put32bit(&raw mut dbuff, (*p).inode);
                } else {
                    put32bit(&raw mut dbuff, MFS_ROOT_ID as uint32_t);
                }
                if attrmode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                    fsnodes_fill_attr(
                        p,
                        p,
                        uid,
                        gid,
                        auid,
                        agid,
                        sesflags,
                        dbuff as *mut uint8_t,
                        1 as uint8_t,
                    );
                    dbuff = dbuff.offset(ATTR_RECORD_SIZE as isize);
                } else if attrmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fsnodes_fill_attr(
                        p,
                        p,
                        uid,
                        gid,
                        auid,
                        agid,
                        sesflags,
                        dbuff as *mut uint8_t,
                        0 as uint8_t,
                    );
                    dbuff = dbuff.offset(35 as ::core::ffi::c_int as isize);
                } else if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                    put8bit(&raw mut dbuff, TYPE_DIRECTORY as uint8_t);
                } else {
                    put8bit(&raw mut dbuff, 'd' as uint8_t);
                }
                nedgeid = 1 as uint64_t;
            } else {
                if nedgeid == 1 as uint64_t {
                    *dbuff.offset(0 as isize) = 2 as uint8_t;
                    *dbuff.offset(1 as isize) = '.' as uint8_t;
                    *dbuff.offset(2 as isize) = '.' as uint8_t;
                    dbuff = dbuff.offset(3 as ::core::ffi::c_int as isize);
                    if (*p).inode == rootinode {
                        put32bit(&raw mut dbuff, MFS_ROOT_ID as uint32_t);
                        if attrmode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                            fsnodes_fill_attr(
                                p,
                                p,
                                uid,
                                gid,
                                auid,
                                agid,
                                sesflags,
                                dbuff as *mut uint8_t,
                                1 as uint8_t,
                            );
                            dbuff = dbuff.offset(ATTR_RECORD_SIZE as isize);
                        } else if attrmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                            fsnodes_fill_attr(
                                p,
                                p,
                                uid,
                                gid,
                                auid,
                                agid,
                                sesflags,
                                dbuff as *mut uint8_t,
                                0 as uint8_t,
                            );
                            dbuff = dbuff.offset(35 as ::core::ffi::c_int as isize);
                        } else if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                            put8bit(&raw mut dbuff, TYPE_DIRECTORY as uint8_t);
                        } else {
                            put8bit(&raw mut dbuff, 'd' as uint8_t);
                        }
                    } else {
                        if !(*p).parents.is_null() && (*(*(*p).parents).parent).inode != rootinode {
                            put32bit(&raw mut dbuff, (*(*(*p).parents).parent).inode);
                        } else {
                            put32bit(&raw mut dbuff, MFS_ROOT_ID as uint32_t);
                        }
                        if attrmode != 0 {
                            if !(*p).parents.is_null() {
                                if attrmode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                    fsnodes_fill_attr(
                                        (*(*p).parents).parent as *mut fsnode,
                                        p,
                                        uid,
                                        gid,
                                        auid,
                                        agid,
                                        sesflags,
                                        dbuff as *mut uint8_t,
                                        1 as uint8_t,
                                    );
                                } else {
                                    fsnodes_fill_attr(
                                        (*(*p).parents).parent as *mut fsnode,
                                        p,
                                        uid,
                                        gid,
                                        auid,
                                        agid,
                                        sesflags,
                                        dbuff as *mut uint8_t,
                                        0 as uint8_t,
                                    );
                                }
                            } else if rootinode == MFS_ROOT_ID as uint32_t {
                                if attrmode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                    fsnodes_fill_attr(
                                        root,
                                        p,
                                        uid,
                                        gid,
                                        auid,
                                        agid,
                                        sesflags,
                                        dbuff as *mut uint8_t,
                                        1 as uint8_t,
                                    );
                                } else {
                                    fsnodes_fill_attr(
                                        root,
                                        p,
                                        uid,
                                        gid,
                                        auid,
                                        agid,
                                        sesflags,
                                        dbuff as *mut uint8_t,
                                        0 as uint8_t,
                                    );
                                }
                            } else {
                                let mut rn: *mut fsnode = fsnodes_node_find(rootinode);
                                if !rn.is_null() {
                                    if attrmode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                        fsnodes_fill_attr(
                                            rn,
                                            p,
                                            uid,
                                            gid,
                                            auid,
                                            agid,
                                            sesflags,
                                            dbuff as *mut uint8_t,
                                            1 as uint8_t,
                                        );
                                    } else {
                                        fsnodes_fill_attr(
                                            rn,
                                            p,
                                            uid,
                                            gid,
                                            auid,
                                            agid,
                                            sesflags,
                                            dbuff as *mut uint8_t,
                                            0 as uint8_t,
                                        );
                                    }
                                } else if attrmode as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                                {
                                    memset(
                                        dbuff as *mut ::core::ffi::c_void,
                                        0 as ::core::ffi::c_int,
                                        ATTR_RECORD_SIZE as size_t,
                                    );
                                } else {
                                    memset(
                                        dbuff as *mut ::core::ffi::c_void,
                                        0 as ::core::ffi::c_int,
                                        35 as size_t,
                                    );
                                }
                            }
                            dbuff = dbuff.offset(
                                (if attrmode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                    ATTR_RECORD_SIZE
                                } else {
                                    35 as ::core::ffi::c_int
                                }) as isize,
                            );
                        } else if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                            put8bit(&raw mut dbuff, TYPE_DIRECTORY as uint8_t);
                        } else {
                            put8bit(&raw mut dbuff, 'd' as uint8_t);
                        }
                    }
                    e = (*p).data.ddata.children;
                } else if !e.is_null() {
                    *dbuff.offset(0 as isize) = (*e).nleng as uint8_t;
                    dbuff = dbuff.offset(1);
                    memcpy(
                        dbuff as *mut ::core::ffi::c_void,
                        &raw const (*e).name as *const uint8_t as *const ::core::ffi::c_void,
                        (*e).nleng as size_t,
                    );
                    dbuff = dbuff.offset((*e).nleng as ::core::ffi::c_int as isize);
                    put32bit(&raw mut dbuff, (*(*e).child).inode);
                    if attrmode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                        fsnodes_fill_attr(
                            (*e).child as *mut fsnode,
                            p,
                            uid,
                            gid,
                            auid,
                            agid,
                            sesflags,
                            dbuff as *mut uint8_t,
                            1 as uint8_t,
                        );
                        dbuff = dbuff.offset(ATTR_RECORD_SIZE as isize);
                    } else if attrmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                        fsnodes_fill_attr(
                            (*e).child as *mut fsnode,
                            p,
                            uid,
                            gid,
                            auid,
                            agid,
                            sesflags,
                            dbuff as *mut uint8_t,
                            0 as uint8_t,
                        );
                        dbuff = dbuff.offset(35 as ::core::ffi::c_int as isize);
                    } else if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                        put8bit(&raw mut dbuff, (*(*e).child).r#type() as uint8_t);
                    } else {
                        put8bit(
                            &raw mut dbuff,
                            DISP_TYPE_REMAP_STR[(*(*e).child).r#type() as usize] as uint8_t,
                        );
                    }
                    e = (*e).nextchild as *mut fsedge;
                }
                if !e.is_null() {
                    nedgeid = (*e).edgeid;
                } else {
                    nedgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                }
            }
            maxentries = maxentries.wrapping_sub(1);
        }
        *nedgeidp = nedgeid;
        if !e.is_null() {
            fsnodes_edgeid_insert(e);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_checkfile(
    mut p: *mut fsnode,
    mut mode: uint8_t,
    mut chunkcount: *mut uint32_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut fcopies: uint8_t = 0;
        let mut ec8parts: uint8_t = 0;
        let mut ec4parts: uint8_t = 0;
        let mut tabindx: uint16_t = 0;
        if mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = 0 as uint32_t;
            while i < 12 as uint32_t {
                *chunkcount.offset(i as isize) = 0 as uint32_t;
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                chunkcount as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<uint32_t>().wrapping_mul(2774 as size_t),
            );
        }
        i = 0 as uint32_t;
        while i < (*p).data.fdata.chunks {
            chunkid = *(*p).data.fdata.chunktab.offset(i as isize);
            tabindx = 0 as uint16_t;
            if chunkid > 0 as uint64_t {
                if chunk_get_storage_status(
                    chunkid,
                    &raw mut fcopies,
                    &raw mut ec8parts,
                    &raw mut ec4parts,
                ) as ::core::ffi::c_int
                    == MFS_STATUS_OK
                {
                    if fcopies as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                        fcopies = 10 as uint8_t;
                    }
                    if ec8parts as ::core::ffi::c_int > 17 as ::core::ffi::c_int {
                        ec8parts = 17 as uint8_t;
                    }
                    if ec4parts as ::core::ffi::c_int > 13 as ::core::ffi::c_int {
                        ec4parts = 13 as uint8_t;
                    }
                    if mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        tabindx = fcopies as uint16_t;
                        if ec8parts as ::core::ffi::c_int > 7 as ::core::ffi::c_int {
                            tabindx = (tabindx as ::core::ffi::c_int
                                + (ec8parts as ::core::ffi::c_int - 7 as ::core::ffi::c_int))
                                as uint16_t;
                        }
                        if ec4parts as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
                            tabindx = (tabindx as ::core::ffi::c_int
                                + (ec4parts as ::core::ffi::c_int - 3 as ::core::ffi::c_int))
                                as uint16_t;
                        }
                        if tabindx as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                            tabindx = 10 as uint16_t;
                        }
                    } else if mode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                        if ec4parts as ::core::ffi::c_int > ec8parts as ::core::ffi::c_int {
                            tabindx = (2 as ::core::ffi::c_uint).wrapping_add(
                                (ec4parts as ::core::ffi::c_uint)
                                    .wrapping_add(4 as ::core::ffi::c_uint)
                                    .wrapping_mul(11 as ::core::ffi::c_uint)
                                    .wrapping_add(fcopies as ::core::ffi::c_uint),
                            ) as uint16_t;
                        } else {
                            tabindx = (2 as ::core::ffi::c_uint).wrapping_add(
                                (ec8parts as ::core::ffi::c_uint)
                                    .wrapping_mul(11 as ::core::ffi::c_uint)
                                    .wrapping_add(fcopies as ::core::ffi::c_uint),
                            ) as uint16_t;
                        }
                    } else {
                        tabindx = (2 as ::core::ffi::c_uint)
                            .wrapping_add(
                                (ec4parts as ::core::ffi::c_uint)
                                    .wrapping_mul(198 as ::core::ffi::c_uint),
                            )
                            .wrapping_add(
                                (ec8parts as ::core::ffi::c_uint)
                                    .wrapping_mul(11 as ::core::ffi::c_uint),
                            )
                            .wrapping_add(fcopies as ::core::ffi::c_uint)
                            as uint16_t;
                    }
                }
            } else if mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                tabindx = 11 as uint16_t;
            } else {
                tabindx = 1 as uint16_t;
            }
            *chunkcount.offset(tabindx as isize) =
                (*chunkcount.offset(tabindx as isize)).wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_append_slice_of_chunks(
    mut ts: uint32_t,
    mut dstobj: *mut fsnode,
    mut srcobj: *mut fsnode,
    mut slice_from: uint32_t,
    mut slice_to: uint32_t,
) -> uint8_t {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut length: uint64_t = 0;
        let mut i: uint32_t = 0;
        let mut srcchunks: uint32_t = 0;
        let mut dstchunks: uint32_t = 0;
        let mut newchunks: uint32_t = 0;
        let mut lastsrcchunk: uint32_t = 0;
        let mut psr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut nsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        if (*srcobj).data.fdata.length > 0 as uint64_t {
            lastsrcchunk = ((*srcobj).data.fdata.length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS)
                as uint32_t;
        } else {
            lastsrcchunk = 0 as uint32_t;
        }
        if slice_from == 0xffffffff as uint32_t && slice_to == 0 as uint32_t {
            slice_from = 0 as uint32_t;
            slice_to = lastsrcchunk;
        }
        if slice_to > lastsrcchunk || slice_from > lastsrcchunk || slice_from > slice_to {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (*srcobj).data.fdata.length > 0 as uint64_t {
            srcchunks = slice_to
                .wrapping_sub(slice_from)
                .wrapping_add(1 as uint32_t);
        } else {
            srcchunks = 0 as uint32_t;
        }
        if (*dstobj).data.fdata.length > 0 as uint64_t {
            dstchunks = (1 as uint64_t).wrapping_add(
                (*dstobj).data.fdata.length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS,
            ) as uint32_t;
        } else {
            dstchunks = 0 as uint32_t;
        }
        newchunks = srcchunks.wrapping_add(dstchunks);
        if newchunks < dstchunks {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        if newchunks.wrapping_sub(1 as uint32_t) > MAX_INDEX as uint32_t {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        fsnodes_get_stats(dstobj, &raw mut psr, 0 as uint8_t);
        if newchunks > (*dstobj).data.fdata.chunks {
            if (*dstobj).data.fdata.chunktab.is_null() {
                (*dstobj).data.fdata.chunktab = chunktab_malloc(newchunks);
            } else {
                (*dstobj).data.fdata.chunktab = chunktab_realloc(
                    (*dstobj).data.fdata.chunktab,
                    (*dstobj).data.fdata.chunks,
                    newchunks,
                );
            }
            if (*dstobj).data.fdata.chunktab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dstobj->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dstobj->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*dstobj).data.fdata.chunktab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dstobj->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dstobj->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            i = (*dstobj).data.fdata.chunks;
            while i < newchunks {
                *(*dstobj).data.fdata.chunktab.offset(i as isize) = 0 as uint64_t;
                i = i.wrapping_add(1);
            }
            (*dstobj).data.fdata.chunks = newchunks;
        }
        i = dstchunks;
        while i < (*dstobj).data.fdata.chunks {
            chunkid = *(*dstobj).data.fdata.chunktab.offset(i as isize);
            if chunkid > 0 as uint64_t {
                if chunk_delete_file(chunkid, (*dstobj).sclassid) != MFS_STATUS_OK {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"structure error - chunk %016lX not found (inode: %u ; index: %u)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        chunkid,
                        (*dstobj).inode,
                        i,
                    );
                }
            }
            *(*dstobj).data.fdata.chunktab.offset(i as isize) = 0 as uint64_t;
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < srcchunks {
            if slice_from.wrapping_add(i) < (*srcobj).data.fdata.chunks {
                chunkid = *(*srcobj)
                    .data
                    .fdata
                    .chunktab
                    .offset(slice_from.wrapping_add(i) as isize);
            } else {
                chunkid = 0 as uint64_t;
            }
            *(*dstobj)
                .data
                .fdata
                .chunktab
                .offset(i.wrapping_add(dstchunks) as isize) = chunkid;
            if chunkid > 0 as uint64_t {
                if chunk_add_file(chunkid, (*dstobj).sclassid) != MFS_STATUS_OK {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"structure error - chunk %016lX not found (inode: %u ; index: %u)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        chunkid,
                        (*srcobj).inode,
                        i.wrapping_add(slice_from),
                    );
                }
            }
            i = i.wrapping_add(1);
        }
        if slice_to >= lastsrcchunk {
            length = ((dstchunks as uint64_t) << MFSCHUNKBITS)
                .wrapping_add((*srcobj).data.fdata.length)
                .wrapping_sub((slice_from as uint64_t) << MFSCHUNKBITS);
        } else {
            length = (newchunks as uint64_t) << MFSCHUNKBITS;
        }
        if (*dstobj).r#type() as ::core::ffi::c_int == TYPE_TRASH {
            trashspace = trashspace.wrapping_sub((*dstobj).data.fdata.length);
            trashspace = trashspace.wrapping_add(length);
        } else if (*dstobj).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
            sustainedspace = sustainedspace.wrapping_sub((*dstobj).data.fdata.length);
            sustainedspace = sustainedspace.wrapping_add(length);
        }
        (*dstobj).data.fdata.length = length;
        fsnodes_get_stats(dstobj, &raw mut nsr, 1 as uint8_t);
        e = (*dstobj).parents;
        while !e.is_null() {
            fsnodes_add_sub_stats((*e).parent as *mut fsnode, &raw mut nsr, &raw mut psr);
            e = (*e).nextparent as *mut fsedge;
        }
        (*dstobj).mtime = ts;
        (*dstobj).atime = ts;
        (*srcobj).atime = ts;
        fsnodes_checkarchmode(dstobj, ts, (CHECK_MTIME | CHECK_ATIME) as uint8_t);
        fsnodes_checkarchmode(srcobj, ts, CHECK_ATIME as uint8_t);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_settrashflag(
    mut obj: *mut fsnode,
    mut trashflag: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as uint32_t;
        while i < (*obj).data.fdata.chunks {
            if *(*obj).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t {
                if chunk_set_trashflag(*(*obj).data.fdata.chunktab.offset(i as isize), trashflag)
                    == MFS_STATUS_OK
                {
                    res |= 1 as ::core::ffi::c_int;
                }
            }
            i = i.wrapping_add(1);
        }
        return res;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_changefilesclassid(mut obj: *mut fsnode, mut sclassid: uint8_t) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut psr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut nsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        fsnodes_get_stats(obj, &raw mut psr, 0 as uint8_t);
        i = 0 as uint32_t;
        while i < (*obj).data.fdata.chunks {
            if *(*obj).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t {
                chunk_change_file(
                    *(*obj).data.fdata.chunktab.offset(i as isize),
                    (*obj).sclassid,
                    sclassid,
                );
            }
            i = i.wrapping_add(1);
        }
        sclass_decref((*obj).sclassid as uint16_t, (*obj).r#type() as uint8_t);
        (*obj).sclassid = sclassid;
        sclass_incref(sclassid as uint16_t, (*obj).r#type() as uint8_t);
        fsnodes_get_stats(obj, &raw mut nsr, 1 as uint8_t);
        e = (*obj).parents;
        while !e.is_null() {
            fsnodes_add_sub_stats((*e).parent as *mut fsnode, &raw mut nsr, &raw mut psr);
            e = (*e).nextparent as *mut fsedge;
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_setlength(mut obj: *mut fsnode, mut length: uint64_t) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut chunks: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut psr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut nsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        fsnodes_get_stats(obj, &raw mut psr, 0 as uint8_t);
        if (*obj).r#type() as ::core::ffi::c_int == TYPE_TRASH {
            trashspace = trashspace.wrapping_sub((*obj).data.fdata.length);
            trashspace = trashspace.wrapping_add(length);
        } else if (*obj).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
            sustainedspace = sustainedspace.wrapping_sub((*obj).data.fdata.length);
            sustainedspace = sustainedspace.wrapping_add(length);
        }
        (*obj).data.fdata.length = length;
        if length > 0 as uint64_t {
            chunks = (length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS)
                .wrapping_add(1 as uint64_t) as uint32_t;
        } else {
            chunks = 0 as uint32_t;
        }
        i = chunks;
        while i < (*obj).data.fdata.chunks {
            chunkid = *(*obj).data.fdata.chunktab.offset(i as isize);
            if chunkid > 0 as uint64_t {
                if chunk_delete_file(chunkid, (*obj).sclassid) != MFS_STATUS_OK {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"structure error - chunk %016lX not found (inode: %u ; index: %u)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        chunkid,
                        (*obj).inode,
                        i,
                    );
                }
            }
            *(*obj).data.fdata.chunktab.offset(i as isize) = 0 as uint64_t;
            i = i.wrapping_add(1);
        }
        if chunks > 0 as uint32_t {
            if chunks < (*obj).data.fdata.chunks && !(*obj).data.fdata.chunktab.is_null() {
                (*obj).data.fdata.chunktab =
                    chunktab_realloc((*obj).data.fdata.chunktab, (*obj).data.fdata.chunks, chunks);
                if (*obj).data.fdata.chunktab.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3360 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"obj->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3360 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"obj->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*obj).data.fdata.chunktab
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint64_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3360 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"obj->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3360 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"obj->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*obj).data.fdata.chunks = chunks;
            }
        } else if (*obj).data.fdata.chunks > 0 as uint32_t && !(*obj).data.fdata.chunktab.is_null()
        {
            chunktab_free((*obj).data.fdata.chunktab, (*obj).data.fdata.chunks);
            (*obj).data.fdata.chunktab = ::core::ptr::null_mut::<uint64_t>();
            (*obj).data.fdata.chunks = 0 as uint32_t;
        }
        fsnodes_get_stats(obj, &raw mut nsr, 1 as uint8_t);
        e = (*obj).parents;
        while !e.is_null() {
            fsnodes_add_sub_stats((*e).parent as *mut fsnode, &raw mut nsr, &raw mut psr);
            e = (*e).nextparent as *mut fsedge;
        }
        (*obj).eattr = ((*obj).eattr as ::core::ffi::c_int & !EATTR_SNAPSHOT) as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_remove_node(mut ts: uint32_t, mut toremove: *mut fsnode) {
    unsafe {
        if !(*toremove).parents.is_null() {
            return;
        }
        fsnodes_node_delete(toremove);
        nodes = nodes.wrapping_sub(1);
        if (*toremove).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            dirnodes = dirnodes.wrapping_sub(1);
            fsnodes_delete_quotanode(toremove);
        }
        if (*toremove).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*toremove).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*toremove).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            let mut i: uint32_t = 0;
            let mut chunkid: uint64_t = 0;
            filenodes = filenodes.wrapping_sub(1);
            i = 0 as uint32_t;
            while i < (*toremove).data.fdata.chunks {
                chunkid = *(*toremove).data.fdata.chunktab.offset(i as isize);
                if chunkid > 0 as uint64_t {
                    if chunk_delete_file(chunkid, (*toremove).sclassid) != MFS_STATUS_OK {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"structure error - chunk %016lX not found (inode: %u ; index: %u)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            chunkid,
                            (*toremove).inode,
                            i,
                        );
                    }
                }
                i = i.wrapping_add(1);
            }
            if !(*toremove).data.fdata.chunktab.is_null() {
                chunktab_free(
                    (*toremove).data.fdata.chunktab,
                    (*toremove).data.fdata.chunks,
                );
            }
        }
        if (*toremove).r#type() as ::core::ffi::c_int == TYPE_SYMLINK {
            if !(*toremove).data.sdata.path.is_null() {
                symlink_free(
                    (*toremove).data.sdata.path,
                    (*toremove).data.sdata.pleng as uint16_t,
                );
            }
        }
        sclass_decref(
            (*toremove).sclassid as uint16_t,
            (*toremove).r#type() as uint8_t,
        );
        fsnodes_free_id((*toremove).inode, ts);
        if (*toremove).xattrflag() != 0 {
            xattr_removeinode((*toremove).inode);
        }
        if (*toremove).aclpermflag() != 0 {
            posix_acl_remove((*toremove).inode, POSIX_ACL_ACCESS as uint8_t);
        }
        if (*toremove).acldefflag() != 0 {
            posix_acl_remove((*toremove).inode, POSIX_ACL_DEFAULT as uint8_t);
        }
        dcm_modify((*toremove).inode, 0 as uint32_t);
        match (*toremove).r#type() as ::core::ffi::c_int {
            TYPE_DIRECTORY => {
                fsnode_free(toremove, 0 as uint8_t);
            }
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                fsnode_free(toremove, 1 as uint8_t);
            }
            TYPE_SYMLINK => {
                fsnode_free(toremove, 2 as uint8_t);
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                fsnode_free(toremove, 3 as uint8_t);
            }
            _ => {
                fsnode_free(toremove, 4 as uint8_t);
            }
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_unlink(mut ts: uint32_t, mut e: *mut fsedge) {
    unsafe {
        let mut child: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut bid: uint32_t = 0;
        let mut pleng: uint16_t = 0 as uint16_t;
        let mut path: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut gototrash: uint8_t = 0;
        let mut isopen: uint8_t = 0;
        gototrash = 0 as uint8_t;
        child = (*e).child as *mut fsnode;
        isopen = of_isfileopen((*child).inode);
        if (*child).r#type() as ::core::ffi::c_int == TYPE_FILE
            && (*child).trashretention as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            if (*child).data.fdata.length > 0 as uint64_t
                || (*child).data.fdata.length == 0 as uint64_t
                    && (KeepEmptyFilesInTrash as ::core::ffi::c_int != 0
                        || isopen as ::core::ffi::c_int != 0)
            {
                gototrash = 1 as uint8_t;
            }
        }
        if (*(*child).parents).nextparent.is_null() {
            if (*child).r#type() as ::core::ffi::c_int == TYPE_FILE
                && (gototrash as ::core::ffi::c_int != 0 || isopen as ::core::ffi::c_int != 0)
            {
                fsnodes_getpath(e, &raw mut pleng, &raw mut path);
            }
        }
        fsnodes_remove_edge(ts, e);
        if (*child).parents.is_null() {
            if (*child).r#type() as ::core::ffi::c_int == TYPE_FILE {
                if gototrash != 0 {
                    bid = (*child).inode.wrapping_rem(TRASH_BUCKETS as uint32_t);
                    (*child).set_type(TYPE_TRASH as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*child).ctime = ts;
                    fsnodes_checkarchmode(child, ts, CHECK_CTIME as uint8_t);
                    e = fsedge_malloc(pleng);
                    if e.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3472 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"e\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3472 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"e\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if e
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut fsedge
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3472 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"e\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3472 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"e\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                    if nextedgeid < 0x7fffffffffffffff as uint64_t {
                        let c2rust_fresh17 = nextedgeid;
                        nextedgeid = nextedgeid.wrapping_sub(1);
                        (*e).edgeid = c2rust_fresh17;
                    } else {
                        (*e).edgeid = 0 as uint64_t;
                    }
                    (*e).nleng = pleng;
                    memcpy(
                        &raw const (*e).name as *const uint8_t as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        path as *const ::core::ffi::c_void,
                        pleng as size_t,
                    );
                    (*e).child = child as *mut _fsnode;
                    (*e).parent = ::core::ptr::null_mut::<_fsnode>();
                    (*e).nextchild = trash[bid as usize] as *mut _fsedge;
                    (*e).nextparent = ::core::ptr::null_mut::<_fsedge>();
                    (*e).prevchild = (&raw mut trash as *mut *mut fsedge).offset(bid as isize)
                        as *mut *mut _fsedge;
                    (*e).prevparent = &raw mut (*child).parents as *mut *mut _fsedge;
                    if !(*e).nextchild.is_null() {
                        (*(*e).nextchild).prevchild = &raw mut (*e).nextchild;
                    }
                    trash[bid as usize] = e;
                    (*child).parents = e;
                    trashspace = trashspace.wrapping_add((*child).data.fdata.length);
                    trashnodes = trashnodes.wrapping_add(1);
                } else if isopen != 0 {
                    bid = (*child).inode.wrapping_rem(SUSTAINED_BUCKETS as uint32_t);
                    (*child).set_type(TYPE_SUSTAINED as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    e = fsedge_malloc(pleng);
                    if e.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3497 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"e\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3497 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"e\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if e
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut fsedge
                    {
                        let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3497 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"e\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3497 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"e\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_0,
                        );
                        abort();
                    }
                    if nextedgeid < 0x7fffffffffffffff as uint64_t {
                        let c2rust_fresh18 = nextedgeid;
                        nextedgeid = nextedgeid.wrapping_sub(1);
                        (*e).edgeid = c2rust_fresh18;
                    } else {
                        (*e).edgeid = 0 as uint64_t;
                    }
                    (*e).nleng = pleng;
                    memcpy(
                        &raw const (*e).name as *const uint8_t as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        path as *const ::core::ffi::c_void,
                        pleng as size_t,
                    );
                    (*e).child = child as *mut _fsnode;
                    (*e).parent = ::core::ptr::null_mut::<_fsnode>();
                    (*e).nextchild = sustained[bid as usize] as *mut _fsedge;
                    (*e).nextparent = ::core::ptr::null_mut::<_fsedge>();
                    (*e).prevchild = (&raw mut sustained as *mut *mut fsedge).offset(bid as isize)
                        as *mut *mut _fsedge;
                    (*e).prevparent = &raw mut (*child).parents as *mut *mut _fsedge;
                    if !(*e).nextchild.is_null() {
                        (*(*e).nextchild).prevchild = &raw mut (*e).nextchild;
                    }
                    sustained[bid as usize] = e;
                    (*child).parents = e;
                    sustainedspace = sustainedspace.wrapping_add((*child).data.fdata.length);
                    sustainednodes = sustainednodes.wrapping_add(1);
                } else {
                    fsnodes_remove_node(ts, child);
                }
            } else {
                fsnodes_remove_node(ts, child);
            }
        }
        if !path.is_null() {
            free(path as *mut ::core::ffi::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_purge(mut ts: uint32_t, mut p: *mut fsnode) -> ::core::ffi::c_int {
    unsafe {
        let mut bid: uint32_t = 0;
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        e = (*p).parents;
        if (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH {
            trashspace = trashspace.wrapping_sub((*p).data.fdata.length);
            trashnodes = trashnodes.wrapping_sub(1);
            if of_isfileopen((*p).inode) != 0 {
                bid = (*p).inode.wrapping_rem(SUSTAINED_BUCKETS as uint32_t);
                (*p).set_type(TYPE_SUSTAINED as ::core::ffi::c_uint as ::core::ffi::c_uint);
                sustainedspace = sustainedspace.wrapping_add((*p).data.fdata.length);
                sustainednodes = sustainednodes.wrapping_add(1);
                *(*e).prevchild = (*e).nextchild;
                if !(*e).nextchild.is_null() {
                    (*(*e).nextchild).prevchild = (*e).prevchild;
                }
                (*e).nextchild = sustained[bid as usize] as *mut _fsedge;
                (*e).prevchild = (&raw mut sustained as *mut *mut fsedge).offset(bid as isize)
                    as *mut *mut _fsedge;
                if !(*e).nextchild.is_null() {
                    (*(*e).nextchild).prevchild = &raw mut (*e).nextchild;
                }
                sustained[bid as usize] = e;
                return 0 as ::core::ffi::c_int;
            } else {
                fsnodes_remove_edge(ts, e);
                fsnodes_remove_node(ts, p);
                return 1 as ::core::ffi::c_int;
            }
        } else if (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
            sustainedspace = sustainedspace.wrapping_sub((*p).data.fdata.length);
            sustainednodes = sustainednodes.wrapping_sub(1);
            fsnodes_remove_edge(ts, e);
            fsnodes_remove_node(ts, p);
            return 1 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_undel(mut ts: uint32_t, mut node: *mut fsnode) -> uint8_t {
    unsafe {
        let mut pleng: uint16_t = 0;
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut new: uint8_t = 0;
        let mut i: uint32_t = 0;
        let mut partleng: uint32_t = 0;
        let mut dots: uint32_t = 0;
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut pe: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut n: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        e = (*node).parents;
        pleng = (*e).nleng;
        path = &raw const (*e).name as *const uint8_t;
        if path.is_null() {
            return MFS_ERROR_CANTCREATEPATH as uint8_t;
        }
        while *path as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            && pleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            path = path.offset(1);
            pleng = pleng.wrapping_sub(1);
        }
        if pleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return MFS_ERROR_CANTCREATEPATH as uint8_t;
        }
        partleng = 0 as uint32_t;
        dots = 0 as uint32_t;
        i = 0 as uint32_t;
        while i < pleng as uint32_t {
            if *path.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return MFS_ERROR_CANTCREATEPATH as uint8_t;
            } else if *path.offset(i as isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                if partleng == 0 as uint32_t {
                    return MFS_ERROR_CANTCREATEPATH as uint8_t;
                }
                if partleng == dots && partleng <= 2 as uint32_t {
                    return MFS_ERROR_CANTCREATEPATH as uint8_t;
                }
                partleng = 0 as uint32_t;
                dots = 0 as uint32_t;
            } else {
                if *path.offset(i as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
                    dots = dots.wrapping_add(1);
                }
                partleng = partleng.wrapping_add(1);
                if partleng > MAXFNAMELENG as uint32_t {
                    return MFS_ERROR_CANTCREATEPATH as uint8_t;
                }
            }
            i = i.wrapping_add(1);
        }
        if partleng == 0 as uint32_t {
            return MFS_ERROR_CANTCREATEPATH as uint8_t;
        }
        if partleng == dots && partleng <= 2 as uint32_t {
            return MFS_ERROR_CANTCREATEPATH as uint8_t;
        }
        n = ::core::ptr::null_mut::<fsnode>();
        p = root;
        new = 0 as uint8_t;
        loop {
            if !(*p).data.ddata.quota.is_null()
                && (*(*p).data.ddata.quota).exceeded as ::core::ffi::c_int != 0
            {
                return MFS_ERROR_QUOTA as uint8_t;
            }
            if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 {
                return MFS_ERROR_EPERM as uint8_t;
            }
            partleng = 0 as uint32_t;
            while partleng < pleng as uint32_t
                && *path.offset(partleng as isize) as ::core::ffi::c_int
                    != '/' as ::core::ffi::c_int
            {
                partleng = partleng.wrapping_add(1);
            }
            if partleng == pleng as uint32_t {
                if fsnodes_nameisused(p, partleng as uint16_t, path) != 0 {
                    return MFS_ERROR_EEXIST as uint8_t;
                }
                (*node).set_type(TYPE_FILE as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*node).ctime = ts;
                fsnodes_checkarchmode(node, ts, CHECK_CTIME as uint8_t);
                fsnodes_link(ts, p, node, partleng as uint16_t, path);
                fsnodes_remove_edge(ts, e);
                trashspace = trashspace.wrapping_sub((*node).data.fdata.length);
                trashnodes = trashnodes.wrapping_sub(1);
                fsnodes_settrashflag(node, 0 as uint8_t);
                return MFS_STATUS_OK as uint8_t;
            } else {
                if new as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    pe = fsnodes_lookup(p, partleng as uint16_t, path);
                    if pe.is_null() {
                        new = 1 as uint8_t;
                    } else {
                        n = (*pe).child as *mut fsnode;
                        if (*n).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                            return MFS_ERROR_CANTCREATEPATH as uint8_t;
                        }
                    }
                }
                if new as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    n = fsnodes_create_node(
                        ts,
                        p,
                        partleng as uint16_t,
                        path,
                        TYPE_DIRECTORY as uint8_t,
                        0o755 as uint16_t,
                        0 as uint16_t,
                        0 as uint32_t,
                        0 as uint32_t,
                        0 as uint8_t,
                    );
                }
                p = n;
            }
            path = path.offset(partleng.wrapping_add(1 as uint32_t) as isize);
            pleng =
                (pleng as uint32_t).wrapping_sub(partleng.wrapping_add(1 as uint32_t)) as uint16_t;
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_getsclass_recursive(
    mut node: *mut fsnode,
    mut gmode: uint8_t,
    mut fgtab: *mut uint32_t,
    mut dgtab: *mut uint32_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        fsnodes_keep_alive_check();
        if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*node).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*node).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            *fgtab.offset((*node).sclassid as isize) =
                (*fgtab.offset((*node).sclassid as isize)).wrapping_add(1);
        } else if (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            *dgtab.offset((*node).sclassid as isize) =
                (*dgtab.offset((*node).sclassid as isize)).wrapping_add(1);
            if gmode as ::core::ffi::c_int == GMODE_RECURSIVE {
                e = (*node).data.ddata.children;
                while !e.is_null() {
                    fsnodes_getsclass_recursive((*e).child as *mut fsnode, gmode, fgtab, dgtab);
                    e = (*e).nextchild as *mut fsedge;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_bst_add(mut n: *mut *mut bstnode, mut val: uint32_t) {
    unsafe {
        while !(*n).is_null() {
            if val < (**n).val {
                n = &raw mut (**n).left as *mut *mut bstnode;
            } else if val > (**n).val {
                n = &raw mut (**n).right as *mut *mut bstnode;
            } else {
                (**n).count = (**n).count.wrapping_add(1);
                return;
            }
        }
        *n = malloc(::core::mem::size_of::<bstnode>()) as *mut bstnode;
        if (*n).is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                3702 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"*n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                3702 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"*n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if *n
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut bstnode
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                3702 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"*n\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                3702 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"*n\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (**n).val = val;
        (**n).count = 1 as uint32_t;
        (**n).left = ::core::ptr::null_mut::<_bstnode>();
        (**n).right = ::core::ptr::null_mut::<_bstnode>();
    }
}
#[inline]
unsafe extern "C" fn fsnodes_bst_nodes(mut n: *mut bstnode) -> uint32_t {
    unsafe {
        if !n.is_null() {
            return (1 as uint32_t)
                .wrapping_add(fsnodes_bst_nodes((*n).left as *mut bstnode))
                .wrapping_add(fsnodes_bst_nodes((*n).right as *mut bstnode));
        } else {
            return 0 as uint32_t;
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_bst_storedata(mut n: *mut bstnode, mut ptr: *mut *mut uint8_t) {
    unsafe {
        if !n.is_null() {
            fsnodes_bst_storedata((*n).left as *mut bstnode, ptr);
            put32bit(ptr, (*n).val);
            put32bit(ptr, (*n).count);
            fsnodes_bst_storedata((*n).right as *mut bstnode, ptr);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_bst_free(mut n: *mut bstnode) {
    unsafe {
        if !n.is_null() {
            fsnodes_bst_free((*n).left as *mut bstnode);
            fsnodes_bst_free((*n).right as *mut bstnode);
            free(n as *mut ::core::ffi::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_gettrashretention_recursive(
    mut node: *mut fsnode,
    mut gmode: uint8_t,
    mut bstrootfiles: *mut *mut bstnode,
    mut bstrootdirs: *mut *mut bstnode,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut trashseconds: uint32_t = 0;
        fsnodes_keep_alive_check();
        trashseconds = (*node).trashretention as uint32_t;
        trashseconds = trashseconds.wrapping_mul(3600 as uint32_t);
        if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*node).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*node).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            fsnodes_bst_add(bstrootfiles, trashseconds);
        } else if (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            fsnodes_bst_add(bstrootdirs, trashseconds);
            if gmode as ::core::ffi::c_int == GMODE_RECURSIVE {
                e = (*node).data.ddata.children;
                while !e.is_null() {
                    fsnodes_gettrashretention_recursive(
                        (*e).child as *mut fsnode,
                        gmode,
                        bstrootfiles,
                        bstrootdirs,
                    );
                    e = (*e).nextchild as *mut fsedge;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_geteattr_recursive(
    mut node: *mut fsnode,
    mut gmode: uint8_t,
    mut feattrtab: *mut uint32_t,
    mut deattrtab: *mut uint32_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        fsnodes_keep_alive_check();
        if (*node).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            *feattrtab.offset(
                ((*node).eattr as ::core::ffi::c_int
                    & (EATTR_NOOWNER
                        | EATTR_NOACACHE
                        | EATTR_NODATACACHE
                        | EATTR_SNAPSHOT
                        | EATTR_UNDELETABLE
                        | EATTR_APPENDONLY
                        | EATTR_IMMUTABLE)) as isize,
            ) = (*feattrtab.offset(
                ((*node).eattr as ::core::ffi::c_int
                    & (EATTR_NOOWNER
                        | EATTR_NOACACHE
                        | EATTR_NODATACACHE
                        | EATTR_SNAPSHOT
                        | EATTR_UNDELETABLE
                        | EATTR_APPENDONLY
                        | EATTR_IMMUTABLE)) as isize,
            ))
            .wrapping_add(1);
        } else {
            *deattrtab.offset((*node).eattr as isize) =
                (*deattrtab.offset((*node).eattr as isize)).wrapping_add(1);
            if gmode as ::core::ffi::c_int == GMODE_RECURSIVE {
                e = (*node).data.ddata.children;
                while !e.is_null() {
                    fsnodes_geteattr_recursive(
                        (*e).child as *mut fsnode,
                        gmode,
                        feattrtab,
                        deattrtab,
                    );
                    e = (*e).nextchild as *mut fsedge;
                }
            }
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_getarch_recursive(
    mut node: *mut fsnode,
    mut archchunks: *mut uint64_t,
    mut notarchchunks: *mut uint64_t,
    mut archinodes: *mut uint32_t,
    mut partinodes: *mut uint32_t,
    mut notarchinodes: *mut uint32_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut archflag: uint8_t = 0;
        let mut j: uint32_t = 0;
        let mut archived: uint32_t = 0;
        let mut notarchived: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        fsnodes_keep_alive_check();
        if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*node).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*node).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            archived = 0 as uint32_t;
            notarchived = 0 as uint32_t;
            j = 0 as uint32_t;
            while j < (*node).data.fdata.chunks {
                chunkid = *(*node).data.fdata.chunktab.offset(j as isize);
                if chunkid > 0 as uint64_t {
                    if chunk_get_archflag(chunkid, &raw mut archflag) == MFS_STATUS_OK {
                        if archflag != 0 {
                            archived = archived.wrapping_add(1);
                        } else {
                            notarchived = notarchived.wrapping_add(1);
                        }
                    }
                }
                j = j.wrapping_add(1);
            }
            if archived > 0 as uint32_t && notarchived > 0 as uint32_t {
                *partinodes = (*partinodes).wrapping_add(1);
            } else if archived == 0 as uint32_t && notarchived > 0 as uint32_t {
                *notarchinodes = (*notarchinodes).wrapping_add(1);
            } else if notarchived == 0 as uint32_t && archived > 0 as uint32_t {
                *archinodes = (*archinodes).wrapping_add(1);
            }
            *archchunks = (*archchunks).wrapping_add(archived as uint64_t);
            *notarchchunks = (*notarchchunks).wrapping_add(notarchived as uint64_t);
        } else if (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            e = (*node).data.ddata.children;
            while !e.is_null() {
                fsnodes_getarch_recursive(
                    (*e).child as *mut fsnode,
                    archchunks,
                    notarchchunks,
                    archinodes,
                    partinodes,
                    notarchinodes,
                );
                e = (*e).nextchild as *mut fsedge;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_setsclass_recursive_test_quota(
    mut node: *mut fsnode,
    mut uid: uint32_t,
    mut storage_eights: uint8_t,
    mut recursive: uint8_t,
    mut realsize: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut i: uint32_t = 0;
        let mut lastchunk: uint32_t = 0;
        let mut lastchunksize: uint32_t = 0;
        let mut rs: uint64_t = 0;
        let mut size: uint64_t = 0;
        fsnodes_keep_alive_check();
        if (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
            && recursive as ::core::ffi::c_int != 0
        {
            rs = 0 as uint64_t;
            e = (*node).data.ddata.children;
            while !e.is_null() {
                if fsnodes_setsclass_recursive_test_quota(
                    (*e).child as *mut fsnode,
                    uid,
                    storage_eights,
                    2 as uint8_t,
                    &raw mut rs,
                ) != 0
                {
                    return 1 as uint8_t;
                }
                e = (*e).nextchild as *mut fsedge;
            }
            if (recursive as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                if fsnodes_test_quota(node, 0 as uint32_t, 0 as uint64_t, 0 as uint64_t, rs) != 0 {
                    return 1 as uint8_t;
                }
            } else if fsnodes_test_quota_noparents(
                node,
                0 as uint32_t,
                0 as uint64_t,
                0 as uint64_t,
                rs,
            ) != 0
            {
                return 1 as uint8_t;
            }
            *realsize = (*realsize).wrapping_add(rs);
            return 0 as uint8_t;
        } else if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*node).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*node).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            if !((*node).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int
                && uid != 0 as uint32_t
                && (*node).uid != uid)
                && (*node).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE == 0 as ::core::ffi::c_int
            {
                if storage_eights as ::core::ffi::c_int
                    > sclass_get_keeparch_maxstorage_eights((*node).sclassid as uint16_t)
                        as ::core::ffi::c_int
                {
                    size = 0 as uint64_t;
                    if (*node).data.fdata.length > 0 as uint64_t {
                        lastchunk = ((*node).data.fdata.length.wrapping_sub(1 as uint64_t)
                            >> MFSCHUNKBITS) as uint32_t;
                        lastchunksize = (((*node).data.fdata.length.wrapping_sub(1 as uint64_t)
                            & MFSCHUNKMASK as uint64_t)
                            .wrapping_add(MFSBLOCKSIZE as uint64_t)
                            & MFSBLOCKNEGMASK as uint64_t)
                            .wrapping_add(MFSHDRSIZE as uint64_t)
                            as uint32_t;
                    } else {
                        lastchunk = 0 as uint32_t;
                        lastchunksize = MFSHDRSIZE as uint32_t;
                    }
                    i = 0 as uint32_t;
                    while i < (*node).data.fdata.chunks {
                        if *(*node).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t {
                            if i < lastchunk {
                                size = size.wrapping_add((MFSCHUNKSIZE + MFSHDRSIZE) as uint64_t);
                            } else if i == lastchunk {
                                size = size.wrapping_add(lastchunksize as uint64_t);
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                    rs = size
                        .wrapping_mul(
                            (storage_eights as ::core::ffi::c_int
                                - sclass_get_keeparch_maxstorage_eights(
                                    (*node).sclassid as uint16_t,
                                ) as ::core::ffi::c_int) as uint64_t,
                        )
                        .wrapping_div(8 as uint64_t);
                    if (recursive as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                        if fsnodes_test_quota(node, 0 as uint32_t, 0 as uint64_t, 0 as uint64_t, rs)
                            != 0
                        {
                            return 1 as uint8_t;
                        }
                    }
                    *realsize = (*realsize).wrapping_add(rs);
                }
            }
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_setsclass_recursive(
    mut node: *mut fsnode,
    mut ts: uint32_t,
    mut uid: uint32_t,
    mut src_sclassid: uint8_t,
    mut dst_sclassid: uint8_t,
    mut smode: uint8_t,
    mut admin: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut set: uint8_t = 0;
        fsnodes_keep_alive_check();
        if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
            || (*node).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*node).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            if (*node).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int
                && uid != 0 as uint32_t
                && (*node).uid != uid
                || (*node).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
                || (sclass_is_admin_only((*node).sclassid as uint16_t) as ::core::ffi::c_int != 0
                    || sclass_is_admin_only(dst_sclassid as uint16_t) as ::core::ffi::c_int != 0)
                    && admin as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                *nsinodes = (*nsinodes).wrapping_add(1);
            } else {
                set = 0 as uint8_t;
                match smode as ::core::ffi::c_int & SMODE_TMASK {
                    SMODE_SET => {
                        if (*node).sclassid as ::core::ffi::c_int
                            != dst_sclassid as ::core::ffi::c_int
                        {
                            set = 1 as uint8_t;
                        }
                    }
                    SMODE_INCREASE => {
                        if ((*node).sclassid as ::core::ffi::c_int)
                            < dst_sclassid as ::core::ffi::c_int
                        {
                            set = 1 as uint8_t;
                        }
                    }
                    SMODE_DECREASE => {
                        if (*node).sclassid as ::core::ffi::c_int
                            > dst_sclassid as ::core::ffi::c_int
                        {
                            set = 1 as uint8_t;
                        }
                    }
                    SMODE_EXCHANGE => {
                        if (*node).sclassid as ::core::ffi::c_int
                            == src_sclassid as ::core::ffi::c_int
                        {
                            set = 1 as uint8_t;
                        }
                    }
                    _ => {}
                }
                if set != 0 {
                    if (*node).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                        fsnodes_changefilesclassid(node, dst_sclassid);
                        *sinodes = (*sinodes).wrapping_add(1);
                    } else {
                        sclass_decref((*node).sclassid as uint16_t, (*node).r#type() as uint8_t);
                        (*node).sclassid = dst_sclassid;
                        sclass_incref(dst_sclassid as uint16_t, (*node).r#type() as uint8_t);
                        *sinodes = (*sinodes).wrapping_add(1);
                    }
                    (*node).ctime = ts;
                    fsnodes_checkarchmode(node, ts, CHECK_CTIME as uint8_t);
                } else {
                    *ncinodes = (*ncinodes).wrapping_add(1);
                }
            }
            if (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
                && smode as ::core::ffi::c_int & SMODE_RMASK != 0
            {
                e = (*node).data.ddata.children;
                while !e.is_null() {
                    fsnodes_setsclass_recursive(
                        (*e).child as *mut fsnode,
                        ts,
                        uid,
                        src_sclassid,
                        dst_sclassid,
                        smode,
                        admin,
                        sinodes,
                        ncinodes,
                        nsinodes,
                    );
                    e = (*e).nextchild as *mut fsedge;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_settrashretention_recursive(
    mut node: *mut fsnode,
    mut ts: uint32_t,
    mut uid: uint32_t,
    mut trashretention: uint16_t,
    mut smode: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut set: uint8_t = 0;
        fsnodes_keep_alive_check();
        if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
            || (*node).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*node).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            if (*node).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int
                && uid != 0 as uint32_t
                && (*node).uid != uid
                || (*node).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
            {
                *nsinodes = (*nsinodes).wrapping_add(1);
            } else {
                set = 0 as uint8_t;
                match smode as ::core::ffi::c_int & SMODE_TMASK {
                    SMODE_SET => {
                        if (*node).trashretention as ::core::ffi::c_int
                            != trashretention as ::core::ffi::c_int
                        {
                            (*node).trashretention = trashretention;
                            set = 1 as uint8_t;
                        }
                    }
                    SMODE_INCREASE => {
                        if ((*node).trashretention as ::core::ffi::c_int)
                            < trashretention as ::core::ffi::c_int
                        {
                            (*node).trashretention = trashretention;
                            set = 1 as uint8_t;
                        }
                    }
                    SMODE_DECREASE => {
                        if (*node).trashretention as ::core::ffi::c_int
                            > trashretention as ::core::ffi::c_int
                        {
                            (*node).trashretention = trashretention;
                            set = 1 as uint8_t;
                        }
                    }
                    _ => {}
                }
                if set != 0 {
                    *sinodes = (*sinodes).wrapping_add(1);
                    (*node).ctime = ts;
                    fsnodes_checkarchmode(node, ts, CHECK_CTIME as uint8_t);
                } else {
                    *ncinodes = (*ncinodes).wrapping_add(1);
                }
            }
            if (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
                && smode as ::core::ffi::c_int & SMODE_RMASK != 0
            {
                e = (*node).data.ddata.children;
                while !e.is_null() {
                    fsnodes_settrashretention_recursive(
                        (*e).child as *mut fsnode,
                        ts,
                        uid,
                        trashretention,
                        smode,
                        sinodes,
                        ncinodes,
                        nsinodes,
                    );
                    e = (*e).nextchild as *mut fsedge;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_seteattr_recursive(
    mut node: *mut fsnode,
    mut ts: uint32_t,
    mut uid: uint32_t,
    mut eattr: uint8_t,
    mut smode: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut neweattr: uint8_t = 0;
        let mut seattr: uint8_t = 0;
        fsnodes_keep_alive_check();
        if (*node).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int
            && uid != 0 as uint32_t
            && (*node).uid != uid
            || (*node).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
                && eattr as ::core::ffi::c_int & EATTR_NOOWNER != 0
        {
            *nsinodes = (*nsinodes).wrapping_add(1);
        } else {
            seattr = eattr;
            if (*node).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                (*node).eattr = ((*node).eattr as ::core::ffi::c_int & !EATTR_NOECACHE) as uint8_t;
                seattr = (seattr as ::core::ffi::c_int & !EATTR_NOECACHE) as uint8_t;
            }
            neweattr = (*node).eattr;
            match smode as ::core::ffi::c_int & SMODE_TMASK {
                SMODE_SET => {
                    neweattr = seattr;
                }
                SMODE_INCREASE => {
                    neweattr =
                        (neweattr as ::core::ffi::c_int | seattr as ::core::ffi::c_int) as uint8_t;
                }
                SMODE_DECREASE => {
                    neweattr = (neweattr as ::core::ffi::c_int & !(seattr as ::core::ffi::c_int))
                        as uint8_t;
                }
                _ => {}
            }
            if neweattr as ::core::ffi::c_int != (*node).eattr as ::core::ffi::c_int {
                (*node).eattr = neweattr;
                *sinodes = (*sinodes).wrapping_add(1);
                (*node).ctime = ts;
                fsnodes_checkarchmode(node, ts, CHECK_CTIME as uint8_t);
            } else {
                *ncinodes = (*ncinodes).wrapping_add(1);
            }
        }
        if (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
            && smode as ::core::ffi::c_int & SMODE_RMASK != 0
        {
            e = (*node).data.ddata.children;
            while !e.is_null() {
                fsnodes_seteattr_recursive(
                    (*e).child as *mut fsnode,
                    ts,
                    uid,
                    eattr,
                    smode,
                    sinodes,
                    ncinodes,
                    nsinodes,
                );
                e = (*e).nextchild as *mut fsedge;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_chgarch_recursive(
    mut node: *mut fsnode,
    mut ts: uint32_t,
    mut uid: uint32_t,
    mut cmd: uint8_t,
    mut chgchunks: *mut uint64_t,
    mut notchgchunks: *mut uint64_t,
    mut nsinodes: *mut uint32_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut aflagchanged: uint32_t = 0;
        let mut allchunks: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        fsnodes_keep_alive_check();
        if (*node).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*node).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*node).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            if (*node).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int
                && uid != 0 as uint32_t
                && (*node).uid != uid
                || (*node).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
            {
                *nsinodes = (*nsinodes).wrapping_add(1);
            } else {
                aflagchanged = 0 as uint32_t;
                allchunks = 0 as uint32_t;
                j = 0 as uint32_t;
                while j < (*node).data.fdata.chunks {
                    chunkid = *(*node).data.fdata.chunktab.offset(j as isize);
                    if chunkid > 0 as uint64_t {
                        allchunks = allchunks.wrapping_add(1);
                        chunk_set_archflag(
                            chunkid,
                            (if cmd as ::core::ffi::c_int == ARCHCTL_SET {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }) as uint8_t,
                            &raw mut aflagchanged,
                        );
                    }
                    j = j.wrapping_add(1);
                }
                (*node).set_keepmode(
                    (if cmd as ::core::ffi::c_int == ARCHCTL_CLR {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
                *chgchunks = (*chgchunks).wrapping_add(aflagchanged as uint64_t);
                *notchgchunks =
                    (*notchgchunks).wrapping_add(allchunks.wrapping_sub(aflagchanged) as uint64_t);
                if cmd as ::core::ffi::c_int == ARCHCTL_CLR {
                    (*node).ctime = ts;
                }
                fsnodes_check_realsize(node);
            }
        }
        if (*node).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            e = (*node).data.ddata.children;
            while !e.is_null() {
                fsnodes_chgarch_recursive(
                    (*e).child as *mut fsnode,
                    ts,
                    uid,
                    cmd,
                    chgchunks,
                    notchgchunks,
                    nsinodes,
                );
                e = (*e).nextchild as *mut fsedge;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_remove_snapshot_test(
    mut e: *mut fsedge,
    mut args: *mut fsnodes_snapshot_params,
) -> uint8_t {
    unsafe {
        let mut n: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ie: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut status: uint8_t = 0;
        n = (*e).child as *mut fsnode;
        fsnodes_keep_alive_check();
        if (*n).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            if (*n).eattr as ::core::ffi::c_int & EATTR_APPENDONLY != 0 {
                return MFS_ERROR_EPERM as uint8_t;
            } else if fsnodes_access_ext(
                n,
                (*args).uid,
                (*args).gids,
                (*args).gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                (*args).sesflags,
            ) != 0
            {
                ie = (*n).data.ddata.children;
                while !ie.is_null() {
                    status = fsnodes_remove_snapshot_test(ie, args);
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        return status;
                    }
                    ie = (*ie).nextchild as *mut fsedge;
                }
            } else {
                return MFS_ERROR_EACCES as uint8_t;
            }
        }
        if (*n).eattr as ::core::ffi::c_int & EATTR_SNAPSHOT == 0 as ::core::ffi::c_int
            || (*n).eattr as ::core::ffi::c_int
                & (EATTR_UNDELETABLE | EATTR_IMMUTABLE | EATTR_APPENDONLY)
                != 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_remove_snapshot(
    mut e: *mut fsedge,
    mut args: *mut fsnodes_snapshot_params,
    mut remove_allowed: uint8_t,
) {
    unsafe {
        let mut eattr_back: uint8_t = 0;
        let mut n: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ie: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut ien: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        n = (*e).child as *mut fsnode;
        fsnodes_keep_alive_check();
        if (*n).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            eattr_back = (*n).eattr;
            if fsnodes_access_ext(
                n,
                (*args).uid,
                (*args).gids,
                (*args).gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                (*args).sesflags,
            ) != 0
            {
                ie = (*n).data.ddata.children;
                while !ie.is_null() {
                    ien = (*ie).nextchild as *mut fsedge;
                    fsnodes_remove_snapshot(
                        ie,
                        args,
                        (if (*n).eattr as ::core::ffi::c_int & (EATTR_APPENDONLY | EATTR_IMMUTABLE)
                            != 0
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }) as uint8_t,
                    );
                    ie = ien;
                }
            }
            if !(*n).data.ddata.children.is_null() {
                return;
            }
            (*n).eattr = eattr_back;
        }
        if (*n).eattr as ::core::ffi::c_int & EATTR_SNAPSHOT != 0 as ::core::ffi::c_int
            && (*n).eattr as ::core::ffi::c_int
                & (EATTR_UNDELETABLE | EATTR_IMMUTABLE | EATTR_APPENDONLY)
                == 0 as ::core::ffi::c_int
            && remove_allowed as ::core::ffi::c_int != 0
        {
            (*n).trashretention = 0 as uint16_t;
            (*args).inode_chksum ^= (*n).inode;
            (*args).removed_object = (*args).removed_object.wrapping_add(1);
            fsnodes_unlink((*args).ts, e);
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_snapshot(
    mut srcnode: *mut fsnode,
    mut parentnode: *mut fsnode,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut newflag: uint8_t,
    mut args: *mut fsnodes_snapshot_params,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut dstnode: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut i: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut rec: uint8_t = 0;
        let mut accessstatus: uint8_t = 0;
        fsnodes_keep_alive_check();
        if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            rec = fsnodes_access_ext(
                srcnode,
                (*args).uid,
                (*args).gids,
                (*args).gid,
                (MODE_MASK_R | MODE_MASK_X) as uint8_t,
                (*args).sesflags,
            ) as uint8_t;
            accessstatus = 1 as uint8_t;
        } else if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_FILE {
            rec = 0 as uint8_t;
            accessstatus = fsnodes_access_ext(
                srcnode,
                (*args).uid,
                (*args).gids,
                (*args).gid,
                MODE_MASK_R as uint8_t,
                (*args).sesflags,
            ) as uint8_t;
        } else {
            rec = 0 as uint8_t;
            accessstatus = 1 as uint8_t;
        }
        if accessstatus as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return;
        }
        if newflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int && {
            e = fsnodes_lookup(parentnode, nleng as uint16_t, name);
            !e.is_null()
        } {
            dstnode = (*e).child as *mut fsnode;
            if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                (*args).existing_object = (*args).existing_object.wrapping_add(1);
                if rec != 0 {
                    e = (*srcnode).data.ddata.children;
                    while !e.is_null() {
                        fsnodes_snapshot(
                            (*e).child as *mut fsnode,
                            dstnode,
                            (*e).nleng as uint32_t,
                            &raw const (*e).name as *const uint8_t,
                            0 as uint8_t,
                            args,
                        );
                        e = (*e).nextchild as *mut fsedge;
                    }
                }
            } else if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_FILE {
                let mut same: uint8_t = 0;
                if (*dstnode).data.fdata.length == (*srcnode).data.fdata.length
                    && (*dstnode).data.fdata.chunks == (*srcnode).data.fdata.chunks
                {
                    same = 1 as uint8_t;
                    i = 0 as uint32_t;
                    while i < (*srcnode).data.fdata.chunks && same as ::core::ffi::c_int != 0 {
                        if *(*srcnode).data.fdata.chunktab.offset(i as isize)
                            != *(*dstnode).data.fdata.chunktab.offset(i as isize)
                        {
                            same = 0 as uint8_t;
                        }
                        i = i.wrapping_add(1);
                    }
                } else {
                    same = 0 as uint8_t;
                }
                if same as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    let mut psr: statsrecord = statsrecord {
                        inodes: 0,
                        dirs: 0,
                        files: 0,
                        chunks: 0,
                        length: 0,
                        size: 0,
                        realsize: 0,
                    };
                    let mut nsr: statsrecord = statsrecord {
                        inodes: 0,
                        dirs: 0,
                        files: 0,
                        chunks: 0,
                        length: 0,
                        size: 0,
                        realsize: 0,
                    };
                    (*args).inode_chksum ^= (*dstnode).inode;
                    fsnodes_unlink((*args).ts, e);
                    if (*args).smode as ::core::ffi::c_int & SNAPSHOT_MODE_CPLIKE_ATTR != 0 {
                        dstnode = fsnodes_create_node(
                            (*args).ts,
                            parentnode,
                            nleng as uint16_t,
                            name,
                            TYPE_FILE as uint8_t,
                            (*srcnode).mode() as uint16_t,
                            (*args).cumask,
                            (*args).uid,
                            *(*args).gid.offset(0 as isize),
                            0 as uint8_t,
                        );
                    } else if (*args).uid == 0 as uint32_t || (*args).uid == (*srcnode).uid {
                        dstnode = fsnodes_create_node(
                            (*args).ts,
                            parentnode,
                            nleng as uint16_t,
                            name,
                            TYPE_FILE as uint8_t,
                            ((*srcnode).mode() as ::core::ffi::c_int & 0xfff as ::core::ffi::c_int)
                                as uint16_t,
                            0 as uint16_t,
                            (*srcnode).uid,
                            (*srcnode).gid,
                            0 as uint8_t,
                        );
                    } else {
                        dstnode = fsnodes_create_node(
                            (*args).ts,
                            parentnode,
                            nleng as uint16_t,
                            name,
                            TYPE_FILE as uint8_t,
                            ((*srcnode).mode() as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int)
                                as uint16_t,
                            0 as uint16_t,
                            (*args).uid,
                            *(*args).gid.offset(0 as isize),
                            0 as uint8_t,
                        );
                    }
                    (*args).existing_object = (*args).existing_object.wrapping_add(1);
                    (*args).inode_chksum ^= (*dstnode).inode;
                    fsnodes_get_stats(dstnode, &raw mut psr, 0 as uint8_t);
                    sclass_decref(
                        (*dstnode).sclassid as uint16_t,
                        (*dstnode).r#type() as uint8_t,
                    );
                    (*dstnode).sclassid = (*srcnode).sclassid;
                    sclass_incref(
                        (*dstnode).sclassid as uint16_t,
                        (*dstnode).r#type() as uint8_t,
                    );
                    (*dstnode).trashretention = (*srcnode).trashretention;
                    if (*srcnode).data.fdata.chunks > 0 as uint32_t {
                        (*dstnode).data.fdata.chunktab =
                            chunktab_malloc((*srcnode).data.fdata.chunks);
                        if (*dstnode).data.fdata.chunktab.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                4181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"dstnode->data.fdata.chunktab\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                4181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"dstnode->data.fdata.chunktab\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if (*dstnode).data.fdata.chunktab
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut uint64_t
                        {
                            let mut _mfs_errorstring: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                4181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"dstnode->data.fdata.chunktab\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_errorstring,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                4181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"dstnode->data.fdata.chunktab\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_errorstring,
                            );
                            abort();
                        }
                        (*dstnode).data.fdata.chunks = (*srcnode).data.fdata.chunks;
                        i = 0 as uint32_t;
                        while i < (*srcnode).data.fdata.chunks {
                            chunkid = *(*srcnode).data.fdata.chunktab.offset(i as isize);
                            *(*dstnode).data.fdata.chunktab.offset(i as isize) = chunkid;
                            if chunkid > 0 as uint64_t {
                                if chunk_add_file(chunkid, (*dstnode).sclassid) != MFS_STATUS_OK {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"structure error - chunk %016lX not found (inode: %u ; index: %u)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        chunkid,
                                        (*srcnode).inode,
                                        i,
                                    );
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                    } else {
                        (*dstnode).data.fdata.chunktab = ::core::ptr::null_mut::<uint64_t>();
                        (*dstnode).data.fdata.chunks = 0 as uint32_t;
                    }
                    (*dstnode).data.fdata.length = (*srcnode).data.fdata.length;
                    fsnodes_get_stats(dstnode, &raw mut nsr, 1 as uint8_t);
                    fsnodes_add_sub_stats(parentnode, &raw mut nsr, &raw mut psr);
                } else {
                    (*args).same_file = (*args).same_file.wrapping_add(1);
                }
            } else if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_SYMLINK {
                (*args).existing_object = (*args).existing_object.wrapping_add(1);
                if (*dstnode).data.sdata.pleng != (*srcnode).data.sdata.pleng {
                    let mut sr: statsrecord = statsrecord {
                        inodes: 0,
                        dirs: 0,
                        files: 0,
                        chunks: 0,
                        length: 0,
                        size: 0,
                        realsize: 0,
                    };
                    memset(
                        &raw mut sr as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<statsrecord>(),
                    );
                    sr.length = (*srcnode).data.sdata.pleng as uint64_t;
                    sr.length = sr
                        .length
                        .wrapping_sub((*dstnode).data.sdata.pleng as uint64_t);
                    fsnodes_add_stats(parentnode, &raw mut sr);
                }
                if !(*dstnode).data.sdata.path.is_null() {
                    symlink_free(
                        (*dstnode).data.sdata.path,
                        (*dstnode).data.sdata.pleng as uint16_t,
                    );
                }
                if (*srcnode).data.sdata.pleng > 0 as uint32_t {
                    (*dstnode).data.sdata.path =
                        symlink_malloc((*srcnode).data.sdata.pleng as uint16_t);
                    if (*dstnode).data.sdata.path.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*dstnode).data.sdata.path
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint8_t
                    {
                        let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_0,
                        );
                        abort();
                    }
                    memcpy(
                        (*dstnode).data.sdata.path as *mut ::core::ffi::c_void,
                        (*srcnode).data.sdata.path as *const ::core::ffi::c_void,
                        (*srcnode).data.sdata.pleng as size_t,
                    );
                    (*dstnode).data.sdata.pleng = (*srcnode).data.sdata.pleng;
                } else {
                    (*dstnode).data.sdata.path = ::core::ptr::null_mut::<uint8_t>();
                    (*dstnode).data.sdata.pleng = 0 as uint32_t;
                }
            } else if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_BLOCKDEV
                || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_CHARDEV
            {
                (*args).existing_object = (*args).existing_object.wrapping_add(1);
                (*dstnode).data.devdata.rdev = (*srcnode).data.devdata.rdev;
            } else {
                (*args).existing_object = (*args).existing_object.wrapping_add(1);
            }
            if (*args).smode as ::core::ffi::c_int & SNAPSHOT_MODE_CPLIKE_ATTR != 0 {
                (*dstnode).uid = (*args).uid;
                (*dstnode).gid = *(*args).gid.offset(0 as isize);
                (*dstnode).set_mode(
                    ((*srcnode).mode() as ::core::ffi::c_int
                        & !((*args).cumask as ::core::ffi::c_int))
                        as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
                (*dstnode).ctime = (*args).ts;
            } else {
                if (*args).uid == 0 as uint32_t || (*args).uid == (*srcnode).uid {
                    (*dstnode).set_mode((*srcnode).mode() as ::core::ffi::c_uint);
                    (*dstnode).uid = (*srcnode).uid;
                    (*dstnode).gid = (*srcnode).gid;
                    (*dstnode).ctime = (*srcnode).ctime;
                } else {
                    (*dstnode).set_mode(
                        ((*srcnode).mode() as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int)
                            as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                    (*dstnode).uid = (*args).uid;
                    (*dstnode).gid = *(*args).gid.offset(0 as isize);
                    (*dstnode).ctime = (*args).ts;
                }
                (*dstnode).atime = (*srcnode).atime;
                (*dstnode).mtime = (*srcnode).mtime;
            }
            fsnodes_checkarchmode(
                dstnode,
                (*args).ts,
                (CHECK_CTIME | CHECK_MTIME | CHECK_ATIME) as uint8_t,
            );
            (*dstnode).eattr = ((*dstnode).eattr as ::core::ffi::c_int | EATTR_SNAPSHOT) as uint8_t;
        } else if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
            || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_SYMLINK
            || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_BLOCKDEV
            || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_CHARDEV
            || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_SOCKET
            || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_FIFO
        {
            let mut psr_0: statsrecord = statsrecord {
                inodes: 0,
                dirs: 0,
                files: 0,
                chunks: 0,
                length: 0,
                size: 0,
                realsize: 0,
            };
            let mut nsr_0: statsrecord = statsrecord {
                inodes: 0,
                dirs: 0,
                files: 0,
                chunks: 0,
                length: 0,
                size: 0,
                realsize: 0,
            };
            if (*args).smode as ::core::ffi::c_int & SNAPSHOT_MODE_PRESERVE_HARDLINKS != 0
                && (*srcnode).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
                && !(*(*srcnode).parents).nextparent.is_null()
            {
                dstnode =
                    chash_find(snapshot_inodehash, (*srcnode).inode as hash_key_t) as *mut fsnode;
                if !dstnode.is_null() {
                    (*args).new_hardlink = (*args).new_hardlink.wrapping_add(1);
                    fsnodes_link((*args).ts, parentnode, dstnode, nleng as uint16_t, name);
                    return;
                }
            }
            if (*args).smode as ::core::ffi::c_int & SNAPSHOT_MODE_CPLIKE_ATTR != 0 {
                dstnode = fsnodes_create_node(
                    (*args).ts,
                    parentnode,
                    nleng as uint16_t,
                    name,
                    (*srcnode).r#type() as uint8_t,
                    (*srcnode).mode() as uint16_t,
                    (*args).cumask,
                    (*args).uid,
                    *(*args).gid.offset(0 as isize),
                    0 as uint8_t,
                );
            } else if (*args).uid == 0 as uint32_t || (*args).uid == (*srcnode).uid {
                dstnode = fsnodes_create_node(
                    (*args).ts,
                    parentnode,
                    nleng as uint16_t,
                    name,
                    (*srcnode).r#type() as uint8_t,
                    (*srcnode).mode() as uint16_t,
                    0 as uint16_t,
                    (*srcnode).uid,
                    (*srcnode).gid,
                    0 as uint8_t,
                );
            } else {
                dstnode = fsnodes_create_node(
                    (*args).ts,
                    parentnode,
                    nleng as uint16_t,
                    name,
                    (*srcnode).r#type() as uint8_t,
                    ((*srcnode).mode() as ::core::ffi::c_int & 0x3ff as ::core::ffi::c_int)
                        as uint16_t,
                    0 as uint16_t,
                    (*args).uid,
                    *(*args).gid.offset(0 as isize),
                    0 as uint8_t,
                );
            }
            (*args).inode_chksum ^= (*dstnode).inode;
            (*args).new_object = (*args).new_object.wrapping_add(1);
            if (*args).smode as ::core::ffi::c_int & SNAPSHOT_MODE_PRESERVE_HARDLINKS != 0
                && (*srcnode).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
                && !(*(*srcnode).parents).nextparent.is_null()
            {
                chash_add(
                    snapshot_inodehash,
                    (*srcnode).inode as hash_key_t,
                    dstnode as *mut ::core::ffi::c_void,
                );
            }
            fsnodes_get_stats(dstnode, &raw mut psr_0, 0 as uint8_t);
            if (*args).smode as ::core::ffi::c_int & SNAPSHOT_MODE_CPLIKE_ATTR
                == 0 as ::core::ffi::c_int
            {
                sclass_decref(
                    (*dstnode).sclassid as uint16_t,
                    (*dstnode).r#type() as uint8_t,
                );
                (*dstnode).sclassid = (*srcnode).sclassid;
                sclass_incref(
                    (*dstnode).sclassid as uint16_t,
                    (*dstnode).r#type() as uint8_t,
                );
                (*dstnode).trashretention = (*srcnode).trashretention;
                (*dstnode).eattr = (*srcnode).eattr;
                (*dstnode).winattr = (*srcnode).winattr;
                (*dstnode).set_mode((*srcnode).mode() as ::core::ffi::c_uint);
                if (*args).uid != 0 as uint32_t && (*args).uid != (*srcnode).uid {
                    (*dstnode).set_mode(
                        (*dstnode).mode() & 0x3ff as ::core::ffi::c_int as ::core::ffi::c_uint,
                    );
                }
                if (*srcnode).xattrflag() != 0 {
                    (*dstnode).set_xattrflag(xattr_copy((*srcnode).inode, (*dstnode).inode)
                        as ::core::ffi::c_uint
                        as ::core::ffi::c_uint);
                }
                if (*srcnode).aclpermflag() != 0 {
                    (*dstnode).set_aclpermflag(posix_acl_copy(
                        (*srcnode).inode,
                        (*dstnode).inode,
                        POSIX_ACL_ACCESS as uint8_t,
                    ) as ::core::ffi::c_uint
                        as ::core::ffi::c_uint);
                }
                if (*srcnode).acldefflag() != 0 {
                    (*dstnode).set_acldefflag(posix_acl_copy(
                        (*srcnode).inode,
                        (*dstnode).inode,
                        POSIX_ACL_DEFAULT as uint8_t,
                    ) as ::core::ffi::c_uint
                        as ::core::ffi::c_uint);
                }
                (*dstnode).set_keepmode(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                if rec != 0 {
                    e = (*srcnode).data.ddata.children;
                    while !e.is_null() {
                        fsnodes_snapshot(
                            (*e).child as *mut fsnode,
                            dstnode,
                            (*e).nleng as uint32_t,
                            &raw const (*e).name as *const uint8_t,
                            1 as uint8_t,
                            args,
                        );
                        e = (*e).nextchild as *mut fsedge;
                    }
                }
            } else if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_FILE {
                if (*srcnode).data.fdata.chunks > 0 as uint32_t {
                    (*dstnode).data.fdata.chunktab = chunktab_malloc((*srcnode).data.fdata.chunks);
                    if (*dstnode).data.fdata.chunktab.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4308 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.fdata.chunktab\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4308 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.fdata.chunktab\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*dstnode).data.fdata.chunktab
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint64_t
                    {
                        let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4308 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.fdata.chunktab\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4308 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.fdata.chunktab\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_errorstring_1,
                        );
                        abort();
                    }
                    (*dstnode).data.fdata.chunks = (*srcnode).data.fdata.chunks;
                    i = 0 as uint32_t;
                    while i < (*srcnode).data.fdata.chunks {
                        chunkid = *(*srcnode).data.fdata.chunktab.offset(i as isize);
                        *(*dstnode).data.fdata.chunktab.offset(i as isize) = chunkid;
                        if chunkid > 0 as uint64_t {
                            if chunk_add_file(chunkid, (*dstnode).sclassid) != MFS_STATUS_OK {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"structure error - chunk %016lX not found (inode: %u ; index: %u)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    chunkid,
                                    (*srcnode).inode,
                                    i,
                                );
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                } else {
                    (*dstnode).data.fdata.chunktab = ::core::ptr::null_mut::<uint64_t>();
                    (*dstnode).data.fdata.chunks = 0 as uint32_t;
                }
                (*dstnode).data.fdata.length = (*srcnode).data.fdata.length;
                fsnodes_get_stats(dstnode, &raw mut nsr_0, 1 as uint8_t);
                fsnodes_add_sub_stats(parentnode, &raw mut nsr_0, &raw mut psr_0);
            } else if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_SYMLINK {
                if (*srcnode).data.sdata.pleng > 0 as uint32_t {
                    (*dstnode).data.sdata.path =
                        symlink_malloc((*srcnode).data.sdata.pleng as uint16_t);
                    if (*dstnode).data.sdata.path.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*dstnode).data.sdata.path
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint8_t
                    {
                        let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            4329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"dstnode->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_2,
                        );
                        abort();
                    }
                    memcpy(
                        (*dstnode).data.sdata.path as *mut ::core::ffi::c_void,
                        (*srcnode).data.sdata.path as *const ::core::ffi::c_void,
                        (*srcnode).data.sdata.pleng as size_t,
                    );
                    (*dstnode).data.sdata.pleng = (*srcnode).data.sdata.pleng;
                }
                fsnodes_get_stats(dstnode, &raw mut nsr_0, 1 as uint8_t);
                fsnodes_add_sub_stats(parentnode, &raw mut nsr_0, &raw mut psr_0);
            } else if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_BLOCKDEV
                || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_CHARDEV
            {
                (*dstnode).data.devdata.rdev = (*srcnode).data.devdata.rdev;
            }
            if (*args).smode as ::core::ffi::c_int & SNAPSHOT_MODE_CPLIKE_ATTR
                == 0 as ::core::ffi::c_int
            {
                (*dstnode).atime = (*srcnode).atime;
                (*dstnode).mtime = (*srcnode).mtime;
                (*dstnode).ctime = (*srcnode).ctime;
                fsnodes_checkarchmode(
                    dstnode,
                    (*args).ts,
                    (CHECK_CTIME | CHECK_MTIME | CHECK_ATIME) as uint8_t,
                );
            }
            (*dstnode).eattr = ((*dstnode).eattr as ::core::ffi::c_int | EATTR_SNAPSHOT) as uint8_t;
        }
    }
}
#[inline]
unsafe extern "C" fn fsnodes_snapshot_test(
    mut origsrcnode: *mut fsnode,
    mut srcnode: *mut fsnode,
    mut parentnode: *mut fsnode,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut canoverwrite: uint8_t,
) -> uint8_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut dstnode: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut status: uint8_t = 0;
        fsnodes_keep_alive_check();
        e = fsnodes_lookup(parentnode, nleng as uint16_t, name);
        if !e.is_null() {
            dstnode = (*e).child as *mut fsnode;
            if dstnode == origsrcnode {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            if (*dstnode).eattr as ::core::ffi::c_int
                & (EATTR_UNDELETABLE | EATTR_IMMUTABLE | EATTR_APPENDONLY)
                != 0
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if (*dstnode).r#type() as ::core::ffi::c_int
                != (*srcnode).r#type() as ::core::ffi::c_int
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_TRASH
                || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                e = (*srcnode).data.ddata.children;
                while !e.is_null() {
                    status = fsnodes_snapshot_test(
                        origsrcnode,
                        (*e).child as *mut fsnode,
                        dstnode,
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                        canoverwrite,
                    );
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        return status;
                    }
                    e = (*e).nextchild as *mut fsedge;
                }
            } else if canoverwrite as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return MFS_ERROR_EEXIST as uint8_t;
            }
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_snapshot_recursive_test_quota(
    mut srcnode: *mut fsnode,
    mut parentnode: *mut fsnode,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut inodes: *mut uint32_t,
    mut length: *mut uint64_t,
    mut size: *mut uint64_t,
    mut realsize: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut dstnode: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut lastchunk: uint32_t = 0;
        let mut lastchunksize: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut fsize: uint64_t = 0;
        fsnodes_keep_alive_check();
        e = fsnodes_lookup(parentnode, nleng as uint16_t, name);
        if !e.is_null() {
            dstnode = (*e).child as *mut fsnode;
            *inodes = (*inodes).wrapping_add(1);
            if (*dstnode).r#type() as ::core::ffi::c_int == TYPE_FILE {
                *length = (*length).wrapping_add((*dstnode).data.fdata.length);
                if (*dstnode).data.fdata.length > 0 as uint64_t {
                    lastchunk = ((*dstnode).data.fdata.length.wrapping_sub(1 as uint64_t)
                        >> MFSCHUNKBITS) as uint32_t;
                    lastchunksize = (((*dstnode).data.fdata.length.wrapping_sub(1 as uint64_t)
                        & MFSCHUNKMASK as uint64_t)
                        .wrapping_add(MFSBLOCKSIZE as uint64_t)
                        & MFSBLOCKNEGMASK as uint64_t)
                        .wrapping_add(MFSHDRSIZE as uint64_t)
                        as uint32_t;
                } else {
                    lastchunk = 0 as uint32_t;
                    lastchunksize = MFSHDRSIZE as uint32_t;
                }
                fsize = 0 as uint64_t;
                i = 0 as uint32_t;
                while i < (*dstnode).data.fdata.chunks {
                    if *(*dstnode).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t {
                        if i < lastchunk {
                            fsize = fsize.wrapping_add((MFSCHUNKSIZE + MFSHDRSIZE) as uint64_t);
                        } else if i == lastchunk {
                            fsize = fsize.wrapping_add(lastchunksize as uint64_t);
                        }
                    }
                    i = i.wrapping_add(1);
                }
                *size = (*size).wrapping_add(fsize);
                *realsize = (*realsize).wrapping_add(
                    fsize
                        .wrapping_mul(sclass_get_keeparch_maxstorage_eights(
                            (*dstnode).sclassid as uint16_t,
                        ) as uint64_t)
                        .wrapping_div(8 as uint64_t),
                );
            } else if (*dstnode).r#type() as ::core::ffi::c_int == TYPE_SYMLINK {
                *length = (*length).wrapping_add((*dstnode).data.sdata.pleng as uint64_t);
            } else if (*dstnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                let mut common_inodes: uint32_t = 0;
                let mut common_length: uint64_t = 0;
                let mut common_size: uint64_t = 0;
                let mut common_realsize: uint64_t = 0;
                let mut ssr: statsrecord = statsrecord {
                    inodes: 0,
                    dirs: 0,
                    files: 0,
                    chunks: 0,
                    length: 0,
                    size: 0,
                    realsize: 0,
                };
                fsnodes_get_stats(srcnode, &raw mut ssr, 2 as uint8_t);
                common_inodes = 0 as uint32_t;
                common_length = 0 as uint64_t;
                common_size = 0 as uint64_t;
                common_realsize = 0 as uint64_t;
                e = (*srcnode).data.ddata.children;
                while !e.is_null() {
                    if fsnodes_snapshot_recursive_test_quota(
                        (*e).child as *mut fsnode,
                        dstnode,
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                        &raw mut common_inodes,
                        &raw mut common_length,
                        &raw mut common_size,
                        &raw mut common_realsize,
                    ) != 0
                    {
                        return 1 as uint8_t;
                    }
                    e = (*e).nextchild as *mut fsedge;
                }
                if ssr.inodes > common_inodes {
                    ssr.inodes = ssr.inodes.wrapping_sub(common_inodes);
                } else {
                    ssr.inodes = 0 as uint32_t;
                }
                if ssr.length > common_length {
                    ssr.length = ssr.length.wrapping_sub(common_length);
                } else {
                    ssr.length = 0 as uint64_t;
                }
                if ssr.size > common_size {
                    ssr.size = ssr.size.wrapping_sub(common_size);
                } else {
                    ssr.size = 0 as uint64_t;
                }
                if ssr.realsize > common_realsize {
                    ssr.realsize = ssr.realsize.wrapping_sub(common_realsize);
                } else {
                    ssr.realsize = 0 as uint64_t;
                }
                if fsnodes_test_quota_noparents(
                    dstnode,
                    ssr.inodes,
                    ssr.length,
                    ssr.size,
                    ssr.realsize,
                ) != 0
                {
                    return 1 as uint8_t;
                }
                *inodes = (*inodes).wrapping_add(common_inodes);
                *length = (*length).wrapping_add(common_length);
                *size = (*size).wrapping_add(common_size);
                *realsize = (*realsize).wrapping_add(common_realsize);
            }
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_namecheck(
    mut nleng: uint32_t,
    mut name: *const uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        if nleng == 0 as uint32_t || nleng > MAXFNAMELENG as uint32_t {
            return -1 as ::core::ffi::c_int;
        }
        if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            if nleng == 1 as uint32_t {
                return -1 as ::core::ffi::c_int;
            }
            if nleng == 2 as uint32_t
                && *name.offset(1 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
        }
        i = 0 as uint32_t;
        while i < nleng {
            if *name.offset(i as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || *name.offset(i as isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_node_find_ext(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: *mut uint32_t,
    mut rootnode: *mut *mut fsnode,
    mut node: *mut *mut fsnode,
    mut skipancestor: uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut rn: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE != 0
            || rootinode == MFS_ROOT_ID as uint32_t
        {
            rn = root;
            p = fsnodes_node_find(*inode);
            if p.is_null() {
                *node = ::core::ptr::null_mut::<fsnode>();
                return 0 as uint8_t;
            }
        } else if rootinode == 0 as uint32_t {
            rn = ::core::ptr::null_mut::<fsnode>();
            p = fsnodes_node_find(*inode);
            if p.is_null()
                || (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
                    && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
            {
                *node = ::core::ptr::null_mut::<fsnode>();
                return 0 as uint8_t;
            }
        } else {
            rn = fsnodes_node_find(rootinode);
            if rn.is_null() || (*rn).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                *node = ::core::ptr::null_mut::<fsnode>();
                return 0 as uint8_t;
            }
            if *inode == MFS_ROOT_ID as uint32_t {
                *inode = rootinode;
                p = rn;
            } else {
                p = fsnodes_node_find(*inode);
                if p.is_null() {
                    *node = ::core::ptr::null_mut::<fsnode>();
                    return 0 as uint8_t;
                }
                if skipancestor as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && fsnodes_isancestor(rn, p) == 0
                {
                    *node = ::core::ptr::null_mut::<fsnode>();
                    return 0 as uint8_t;
                }
            }
        }
        if !rootnode.is_null() {
            *rootnode = rn;
        }
        *node = p;
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_check_inode(mut inode: uint32_t) -> uint8_t {
    unsafe {
        return (if !fsnodes_node_find(inode).is_null() {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_set_xattrflag(mut inode: uint32_t) {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if !p.is_null() {
            (*p).set_xattrflag(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_del_xattrflag(mut inode: uint32_t) {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if !p.is_null() {
            (*p).set_xattrflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_set_aclflag(mut inode: uint32_t, mut acltype: uint8_t) {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if !p.is_null() {
            if acltype as ::core::ffi::c_int == POSIX_ACL_ACCESS {
                (*p).set_aclpermflag(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if acltype as ::core::ffi::c_int == POSIX_ACL_DEFAULT {
                (*p).set_acldefflag(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_del_aclflag(mut inode: uint32_t, mut acltype: uint8_t) {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if !p.is_null() {
            if acltype as ::core::ffi::c_int == POSIX_ACL_ACCESS {
                (*p).set_aclpermflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if acltype as ::core::ffi::c_int == POSIX_ACL_DEFAULT {
                (*p).set_acldefflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_mode(mut inode: uint32_t) -> uint16_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if !p.is_null() {
            return (*p).mode() as uint16_t;
        }
        return 0 as uint16_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_access(mut ts: uint32_t, mut inode: uint32_t) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        (*p).atime = ts;
        fsnodes_checkarchmode(p, ts, CHECK_ATIME as uint8_t);
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readsustained_size(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut dbuffsize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut bid: uint32_t = 0;
        if rootinode != 0 as uint32_t {
            return MFS_ERROR_EPERM as uint8_t;
        }
        *dbuffsize = 0 as uint32_t;
        bid = 0 as uint32_t;
        while bid < SUSTAINED_BUCKETS as uint32_t {
            *dbuffsize = (*dbuffsize).wrapping_add(fsnodes_getdetached(
                sustained[bid as usize],
                ::core::ptr::null_mut::<uint8_t>(),
            ));
            bid = bid.wrapping_add(1);
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readsustained_data(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut dbuff: *mut uint8_t,
) {
    unsafe {
        let mut pos: uint32_t = 0;
        let mut bid: uint32_t = 0;
        pos = 0 as uint32_t;
        bid = 0 as uint32_t;
        while bid < SUSTAINED_BUCKETS as uint32_t {
            pos = pos.wrapping_add(fsnodes_getdetached(
                sustained[bid as usize],
                dbuff.offset(pos as isize),
            ));
            bid = bid.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readtrash_size(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut bid: uint32_t,
    mut dbuffsize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        if rootinode != 0 as uint32_t {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if bid >= TRASH_BUCKETS as uint32_t {
            *dbuffsize = 0 as uint32_t;
            bid = 0 as uint32_t;
            while bid < TRASH_BUCKETS as uint32_t {
                *dbuffsize = (*dbuffsize).wrapping_add(fsnodes_getdetached(
                    trash[bid as usize],
                    ::core::ptr::null_mut::<uint8_t>(),
                ));
                bid = bid.wrapping_add(1);
            }
        } else {
            *dbuffsize =
                fsnodes_getdetached(trash[bid as usize], ::core::ptr::null_mut::<uint8_t>());
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readtrash_data(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut bid: uint32_t,
    mut dbuff: *mut uint8_t,
) {
    unsafe {
        if bid >= TRASH_BUCKETS as uint32_t {
            let mut pos: uint32_t = 0;
            pos = 0 as uint32_t;
            bid = 0 as uint32_t;
            while bid < TRASH_BUCKETS as uint32_t {
                pos = pos.wrapping_add(fsnodes_getdetached(
                    trash[bid as usize],
                    dbuff.offset(pos as isize),
                ));
                bid = bid.wrapping_add(1);
            }
        } else {
            fsnodes_getdetached(trash[bid as usize], dbuff);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_listtrash(
    mut partno: uint8_t,
    mut format: uint8_t,
    mut uid: uint32_t,
    mut mints: uint32_t,
    mut maxts: uint32_t,
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
    mut dbuff: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut countersptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut minbid: uint16_t = 0;
        let mut maxbid: uint16_t = 0;
        let mut bid: uint16_t = 0;
        let mut size: uint32_t = 0;
        let mut s: uint32_t = 0;
        let mut total: uint32_t = 0;
        let mut userfiles: uint32_t = 0;
        let mut matchfiles: uint32_t = 0;
        let mut glob: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        minbid =
            (TRASH_BUCKETS / 256 as ::core::ffi::c_int * partno as ::core::ffi::c_int) as uint16_t;
        if partno as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
            maxbid = TRASH_BUCKETS as uint16_t;
        } else {
            maxbid = (TRASH_BUCKETS / 256 as ::core::ffi::c_int
                * (partno as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                as uint16_t;
        }
        size = 12 as uint32_t;
        countersptr = dbuff;
        if !dbuff.is_null() {
            dbuff = dbuff.offset(12 as ::core::ffi::c_int as isize);
        }
        glob = glob_cache_get(gnleng, gname);
        total = 0 as uint32_t;
        userfiles = 0 as uint32_t;
        matchfiles = 0 as uint32_t;
        bid = minbid;
        while (bid as ::core::ffi::c_int) < maxbid as ::core::ffi::c_int {
            s = fsnodes_listtrash(
                trash[bid as usize],
                format,
                uid,
                mints,
                maxts,
                glob,
                &raw mut total,
                &raw mut userfiles,
                &raw mut matchfiles,
                dbuff,
            );
            size = size.wrapping_add(s);
            if !dbuff.is_null() {
                dbuff = dbuff.offset(s as isize);
            }
            bid = bid.wrapping_add(1);
        }
        if !countersptr.is_null() {
            put32bit(&raw mut countersptr, total);
            put32bit(&raw mut countersptr, userfiles);
            put32bit(&raw mut countersptr, matchfiles);
        }
        return size;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_listsustained(
    mut partno: uint8_t,
    mut uid: uint32_t,
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
    mut dbuff: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut countersptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut minbid: uint16_t = 0;
        let mut maxbid: uint16_t = 0;
        let mut bid: uint16_t = 0;
        let mut size: uint32_t = 0;
        let mut s: uint32_t = 0;
        let mut total: uint32_t = 0;
        let mut matchfiles: uint32_t = 0;
        let mut glob: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        minbid = (SUSTAINED_BUCKETS / 256 as ::core::ffi::c_int * partno as ::core::ffi::c_int)
            as uint16_t;
        if partno as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
            maxbid = SUSTAINED_BUCKETS as uint16_t;
        } else {
            maxbid = (SUSTAINED_BUCKETS / 256 as ::core::ffi::c_int
                * (partno as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                as uint16_t;
        }
        size = 8 as uint32_t;
        countersptr = dbuff;
        if !dbuff.is_null() {
            dbuff = dbuff.offset(8 as ::core::ffi::c_int as isize);
        }
        glob = glob_cache_get(gnleng, gname);
        total = 0 as uint32_t;
        matchfiles = 0 as uint32_t;
        bid = minbid;
        while (bid as ::core::ffi::c_int) < maxbid as ::core::ffi::c_int {
            s = fsnodes_listsustained(
                sustained[bid as usize],
                uid,
                glob,
                &raw mut total,
                &raw mut matchfiles,
                dbuff,
            );
            size = size.wrapping_add(s);
            if !dbuff.is_null() {
                dbuff = dbuff.offset(s as isize);
            }
            bid = bid.wrapping_add(1);
        }
        if !countersptr.is_null() {
            put32bit(&raw mut countersptr, total);
            put32bit(&raw mut countersptr, matchfiles);
        }
        return size;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_trash_recover(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut parent_dst: uint32_t,
    mut pleng_dst: uint32_t,
    mut path_dst: *const uint8_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut copysgid: uint8_t,
    mut used_pleng: *mut uint32_t,
    mut used_path: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut dwd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ndwd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut sp: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut ne: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut ppos: uint32_t = 0;
        let mut cpos: uint32_t = 0;
        let mut uppos: uint32_t = 0;
        let mut funiq: uint8_t = 0;
        let mut chgname: uint8_t = 0;
        let mut used_nleng: uint8_t = 0;
        let mut used_name: [uint8_t; 255] = [0; 255];
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut parent_dst,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut dwd,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        sp = fsnodes_node_find(inode);
        if sp.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if uid != 0 as uint32_t && (*sp).uid != uid {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*sp).r#type() as ::core::ffi::c_int != TYPE_TRASH {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (*dwd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(
                dwd,
                uid,
                gids,
                gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                sesflags,
            ) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            fsnodes_get_stats(sp, &raw mut sr, 2 as uint8_t);
            if fsnodes_test_quota(dwd, sr.inodes, sr.length, sr.size, sr.realsize) != 0 {
                return MFS_ERROR_QUOTA as uint8_t;
            }
        }
        e = (*sp).parents;
        if pleng_dst == 0 as uint32_t {
            pleng_dst = (*e).nleng as uint32_t;
            path_dst = &raw const (*e).name as *const uint8_t;
        }
        if pleng_dst == 0 as uint32_t
            || *path_dst.offset(pleng_dst.wrapping_sub(1 as uint32_t) as isize)
                as ::core::ffi::c_int
                == '/' as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        ppos = 0 as uint32_t;
        loop {
            while ppos < pleng_dst
                && *path_dst.offset(ppos as isize) as ::core::ffi::c_int
                    == '/' as ::core::ffi::c_int
            {
                ppos = ppos.wrapping_add(1);
            }
            cpos = ppos;
            while ppos < pleng_dst
                && *path_dst.offset(ppos as isize) as ::core::ffi::c_int
                    != '/' as ::core::ffi::c_int
            {
                if *path_dst.offset(ppos as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
                ppos = ppos.wrapping_add(1);
            }
            if ppos.wrapping_sub(cpos) > MAXFNAMELENG as uint32_t {
                return MFS_ERROR_ENAMETOOLONG as uint8_t;
            }
            if fsnodes_namecheck(ppos.wrapping_sub(cpos), path_dst.offset(cpos as isize))
                < 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            if *path_dst.offset(cpos as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (ppos.wrapping_sub(cpos) == 1 as uint32_t
                    || ppos.wrapping_sub(cpos) == 2 as uint32_t
                        && *path_dst.offset(cpos.wrapping_add(1 as uint32_t) as isize)
                            as ::core::ffi::c_int
                            == '.' as ::core::ffi::c_int)
            {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            if ppos >= pleng_dst {
                break;
            }
        }
        ppos = 0 as uint32_t;
        uppos = 0 as uint32_t;
        chgname = 0 as uint8_t;
        loop {
            while ppos < pleng_dst
                && *path_dst.offset(ppos as isize) as ::core::ffi::c_int
                    == '/' as ::core::ffi::c_int
            {
                ppos = ppos.wrapping_add(1);
            }
            cpos = ppos;
            while ppos < pleng_dst
                && *path_dst.offset(ppos as isize) as ::core::ffi::c_int
                    != '/' as ::core::ffi::c_int
            {
                ppos = ppos.wrapping_add(1);
            }
            funiq = 0 as uint8_t;
            ndwd = ::core::ptr::null_mut::<fsnode>();
            if ppos < pleng_dst {
                ne = fsnodes_lookup(
                    dwd,
                    ppos.wrapping_sub(cpos) as uint16_t,
                    path_dst.offset(cpos as isize),
                );
                if ne.is_null() {
                    ndwd = ::core::ptr::null_mut::<fsnode>();
                } else {
                    ndwd = (*ne).child as *mut fsnode;
                }
                if !ndwd.is_null() {
                    if (*ndwd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                        funiq = 1 as uint8_t;
                        ndwd = ::core::ptr::null_mut::<fsnode>();
                    } else if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE
                        == 0 as ::core::ffi::c_int
                        && fsnodes_access_ext(
                            ndwd,
                            uid,
                            gids,
                            gid,
                            (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                            sesflags,
                        ) == 0
                    {
                        funiq = 1 as uint8_t;
                        ndwd = ::core::ptr::null_mut::<fsnode>();
                    } else if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE
                        == 0 as ::core::ffi::c_int
                        && fsnodes_test_quota(dwd, sr.inodes, sr.length, sr.size, sr.realsize)
                            as ::core::ffi::c_int
                            != 0
                    {
                        funiq = 1 as uint8_t;
                        ndwd = ::core::ptr::null_mut::<fsnode>();
                    }
                }
            } else if fsnodes_nameisused(
                dwd,
                ppos.wrapping_sub(cpos) as uint16_t,
                path_dst.offset(cpos as isize),
            ) != 0
            {
                funiq = 1 as uint8_t;
            }
            if funiq != 0 {
                fsnodes_find_uniqname(
                    dwd,
                    ppos.wrapping_sub(cpos) as uint8_t,
                    path_dst.offset(cpos as isize),
                    &raw mut used_nleng,
                    &raw mut used_name as *mut uint8_t,
                );
                chgname = 1 as uint8_t;
            } else {
                used_nleng = ppos.wrapping_sub(cpos) as uint8_t;
                memcpy(
                    &raw mut used_name as *mut uint8_t as *mut ::core::ffi::c_void,
                    path_dst.offset(cpos as isize) as *const ::core::ffi::c_void,
                    used_nleng as size_t,
                );
            }
            if uppos.wrapping_add(used_nleng as uint32_t) < MFS_PATH_MAX as uint32_t {
                memcpy(
                    used_path.offset(uppos as isize) as *mut ::core::ffi::c_void,
                    &raw mut used_name as *mut uint8_t as *const ::core::ffi::c_void,
                    used_nleng as size_t,
                );
                uppos = uppos.wrapping_add(used_nleng as uint32_t);
            } else if uppos < MFS_PATH_MAX as uint32_t {
                memcpy(
                    used_path.offset(uppos as isize) as *mut ::core::ffi::c_void,
                    &raw mut used_name as *mut uint8_t as *const ::core::ffi::c_void,
                    (MFS_PATH_MAX as uint32_t).wrapping_sub(uppos) as size_t,
                );
                uppos = MFS_PATH_MAX as uint32_t;
            }
            if ppos < pleng_dst {
                if uppos < MFS_PATH_MAX as uint32_t {
                    let c2rust_fresh20 = uppos;
                    uppos = uppos.wrapping_add(1);
                    *used_path.offset(c2rust_fresh20 as isize) = '/' as uint8_t;
                }
                if ndwd.is_null() {
                    ndwd = fsnodes_create_node(
                        ts,
                        dwd,
                        used_nleng as uint16_t,
                        &raw mut used_name as *mut uint8_t,
                        TYPE_DIRECTORY as uint8_t,
                        0o777 as uint16_t,
                        cumask,
                        uid,
                        *gid.offset(0 as isize),
                        copysgid,
                    );
                }
                dwd = ndwd;
            } else {
                (*sp).set_type(TYPE_FILE as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*sp).ctime = ts;
                fsnodes_checkarchmode(sp, ts, CHECK_CTIME as uint8_t);
                fsnodes_link(
                    ts,
                    dwd,
                    sp,
                    used_nleng as uint16_t,
                    &raw mut used_name as *mut uint8_t,
                );
                fsnodes_remove_edge(ts, e);
                trashspace = trashspace.wrapping_sub((*sp).data.fdata.length);
                trashnodes = trashnodes.wrapping_sub(1);
                fsnodes_settrashflag(sp, 0 as uint8_t);
            }
            if ppos >= pleng_dst {
                break;
            }
        }
        if chgname != 0 {
            *used_pleng = uppos;
        } else {
            *used_pleng = 0 as uint32_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|TRASH_RECOVER(%u,%u,%s,%hu,%u,%u,%hhu):(%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ts,
                inode,
                parent_dst,
                changelog_escape_name(pleng_dst, path_dst),
                cumask as ::core::ffi::c_int,
                uid,
                *gid.offset(0 as isize),
                copysgid as ::core::ffi::c_int,
                changelog_escape_name(*used_pleng, used_path as *const uint8_t),
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_trash_recover(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut parent_dst: uint32_t,
    mut pleng_dst: uint32_t,
    mut path_dst: *const uint8_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut copysgid: uint8_t,
    mut used_pleng: *mut uint32_t,
    mut used_path: *mut uint8_t,
) -> uint8_t {
    unsafe {
        return fs_univ_trash_recover(
            main_time(),
            rootinode,
            sesflags,
            inode,
            parent_dst,
            pleng_dst,
            path_dst,
            cumask,
            uid,
            gids,
            gid,
            copysgid,
            used_pleng,
            used_path,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_trash_recover(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut parent_dst: uint32_t,
    mut pleng_dst: uint32_t,
    mut path_dst: *const uint8_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut copysgid: uint8_t,
    mut created_pleng: uint32_t,
    mut created_path: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut used_pleng: uint32_t = 0;
        let mut used_path: [uint8_t; 1024] = [0; 1024];
        let mut status: uint8_t = 0;
        status = fs_univ_trash_recover(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            inode,
            parent_dst,
            pleng_dst,
            path_dst,
            cumask,
            uid,
            1 as uint32_t,
            &raw mut gid,
            copysgid,
            &raw mut used_pleng,
            &raw mut used_path as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if used_pleng != created_pleng
            || used_pleng > 0 as uint32_t
                && memcmp(
                    &raw mut used_path as *mut uint8_t as *const ::core::ffi::c_void,
                    created_path as *const ::core::ffi::c_void,
                    used_pleng as size_t,
                ) != 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_trash_remove(
    mut ts: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
) -> uint8_t {
    unsafe {
        let mut sp: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        sp = fsnodes_node_find(inode);
        if sp.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if uid != 0 as uint32_t && (*sp).uid != uid {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*sp).r#type() as ::core::ffi::c_int != TYPE_TRASH {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        fsnodes_purge(ts, sp);
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|TRASH_REMOVE(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode,
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_trash_remove(
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_trash_remove(main_time(), sesflags, inode, uid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_trash_remove(mut ts: uint32_t, mut inode: uint32_t) -> uint8_t {
    unsafe {
        return fs_univ_trash_remove(ts, SESFLAG_METARESTORE as uint8_t, inode, 0 as uint32_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getdetachedattr(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut attr: *mut uint8_t,
    mut dtype: uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        memset(
            attr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ATTR_RECORD_SIZE as size_t,
        );
        if rootinode != 0 as uint32_t {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if !(dtype as uint32_t <= 2 as uint32_t) {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if dtype as ::core::ffi::c_int == DTYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if dtype as ::core::ffi::c_int == DTYPE_SUSTAINED
            && (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        fsnodes_fill_attr(
            p,
            ::core::ptr::null_mut::<fsnode>(),
            (*p).uid,
            (*p).gid,
            (*p).uid,
            (*p).gid,
            sesflags,
            attr,
            1 as uint8_t,
        );
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_gettrashpath(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut pleng: *mut uint32_t,
    mut path: *mut *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        *pleng = 0 as uint32_t;
        *path = ::core::ptr::null::<uint8_t>();
        if rootinode != 0 as uint32_t {
            return MFS_ERROR_EPERM as uint8_t;
        }
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        *pleng = (*(*p).parents).nleng as uint32_t;
        *path = &raw const (*(*p).parents).name as *const uint8_t;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_setpath(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut trash_cid: uint32_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut i: uint32_t = 0;
        if rootinode != 0 as uint32_t {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if pleng == 0 as uint32_t || pleng > MFS_PATH_MAX as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        i = 0 as uint32_t;
        while i < pleng {
            if *path.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            i = i.wrapping_add(1);
        }
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        fsnodes_remove_edge(0 as uint32_t, (*p).parents);
        e = fsedge_malloc(pleng as uint16_t);
        if e.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                5056 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"e\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                5056 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"e\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if e
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut fsedge
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                5056 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"e\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                5056 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"e\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        if nextedgeid < 0x7fffffffffffffff as uint64_t {
            let c2rust_fresh19 = nextedgeid;
            nextedgeid = nextedgeid.wrapping_sub(1);
            (*e).edgeid = c2rust_fresh19;
        } else {
            (*e).edgeid = 0 as uint64_t;
        }
        trash_cid = inode.wrapping_rem(TRASH_BUCKETS as uint32_t);
        (*e).nleng = pleng as uint16_t;
        memcpy(
            &raw const (*e).name as *const uint8_t as *mut uint8_t as *mut ::core::ffi::c_void,
            path as *const ::core::ffi::c_void,
            pleng as size_t,
        );
        (*e).child = p as *mut _fsnode;
        (*e).parent = ::core::ptr::null_mut::<_fsnode>();
        (*e).nextchild = trash[trash_cid as usize] as *mut _fsedge;
        (*e).nextparent = ::core::ptr::null_mut::<_fsedge>();
        (*e).prevchild =
            (&raw mut trash as *mut *mut fsedge).offset(trash_cid as isize) as *mut *mut _fsedge;
        (*e).prevparent = &raw mut (*p).parents as *mut *mut _fsedge;
        if !(*e).nextchild.is_null() {
            (*(*e).nextchild).prevchild = &raw mut (*e).nextchild;
        }
        trash[trash_cid as usize] = e;
        (*p).parents = e;
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|SETPATH(%u,%s)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                inode,
                changelog_escape_name(pleng, path),
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_settrashpath(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
) -> uint8_t {
    unsafe {
        return fs_univ_setpath(rootinode, sesflags, inode, pleng, path);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_setpath(mut inode: uint32_t, mut path: *const uint8_t) -> uint8_t {
    unsafe {
        return fs_univ_setpath(
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            inode,
            strlen(path as *mut ::core::ffi::c_char) as uint32_t,
            path,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_undel(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut status: uint8_t = 0;
        if rootinode != 0 as uint32_t {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        status = fsnodes_undel(ts, p);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|UNDEL(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode,
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_undel(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_undel(main_time(), rootinode, sesflags, inode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_undel(mut ts: uint32_t, mut inode: uint32_t) -> uint8_t {
    unsafe {
        return fs_univ_undel(ts, 0 as uint32_t, SESFLAG_METARESTORE as uint8_t, inode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_purge(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if rootinode != 0 as uint32_t {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        fsnodes_purge(ts, p);
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|PURGE(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode,
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_purge(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_purge(main_time(), rootinode, sesflags, inode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_purge(mut ts: uint32_t, mut inode: uint32_t) -> uint8_t {
    unsafe {
        return fs_univ_purge(ts, 0 as uint32_t, SESFLAG_METARESTORE as uint8_t, inode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_parents_count(
    mut rootinode: uint32_t,
    mut inode: uint32_t,
    mut cnt: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fsnodes_node_find_ext(
            rootinode,
            0 as uint8_t,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        *cnt = fsnodes_nlink(rootinode, p);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_parents_data(
    mut rootinode: uint32_t,
    mut inode: uint32_t,
    mut buff: *mut uint8_t,
) {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fsnodes_node_find_ext(
            rootinode,
            0 as uint8_t,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) != 0
        {
            fsnodes_get_parents(rootinode, p, buff);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_paths_size(
    mut rootinode: uint32_t,
    mut inode: uint32_t,
    mut psize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fsnodes_node_find_ext(
            rootinode,
            0 as uint8_t,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            *psize = (9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t;
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH {
            *psize = (7 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int
                + (*(*p).parents).nleng as ::core::ffi::c_int) as uint32_t;
        } else if (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
            *psize = (11 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int
                + (*(*p).parents).nleng as ::core::ffi::c_int) as uint32_t;
        } else {
            *psize = fsnodes_get_paths_size(rootinode, p);
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_paths_data(
    mut rootinode: uint32_t,
    mut inode: uint32_t,
    mut buff: *mut uint8_t,
) {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fsnodes_node_find_ext(
            rootinode,
            0 as uint8_t,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) != 0
        {
            if (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH {
                put32bit(
                    &raw mut buff,
                    (7 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        + (*(*p).parents).nleng as ::core::ffi::c_int)
                        as uint32_t,
                );
                memcpy(
                    buff as *mut ::core::ffi::c_void,
                    b"./TRASH (\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    9 as size_t,
                );
                memcpy(
                    buff.offset(9 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                    &raw const (*(*p).parents).name as *const uint8_t as *const ::core::ffi::c_void,
                    (*(*p).parents).nleng as size_t,
                );
                *buff.offset(
                    (9 as ::core::ffi::c_int + (*(*p).parents).nleng as ::core::ffi::c_int)
                        as isize,
                ) = ')' as uint8_t;
            } else if (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
                put32bit(
                    &raw mut buff,
                    (11 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        + (*(*p).parents).nleng as ::core::ffi::c_int)
                        as uint32_t,
                );
                memcpy(
                    buff as *mut ::core::ffi::c_void,
                    b"./SUSTAINED (\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    13 as size_t,
                );
                memcpy(
                    buff.offset(13 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                    &raw const (*(*p).parents).name as *const uint8_t as *const ::core::ffi::c_void,
                    (*(*p).parents).nleng as size_t,
                );
                *buff.offset(
                    (13 as ::core::ffi::c_int + (*(*p).parents).nleng as ::core::ffi::c_int)
                        as isize,
                ) = ')' as uint8_t;
            } else {
                fsnodes_get_paths_data(rootinode, p, buff);
            }
        } else {
            put32bit(&raw mut buff, 9 as uint32_t);
            memcpy(
                buff as *mut ::core::ffi::c_void,
                b"(deleted)\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                9 as size_t,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_info(
    mut totalspace: *mut uint64_t,
    mut availspace: *mut uint64_t,
    mut freespace: *mut uint64_t,
    mut trspace: *mut uint64_t,
    mut trnodes: *mut uint32_t,
    mut respace: *mut uint64_t,
    mut renodes: *mut uint32_t,
    mut inodes: *mut uint32_t,
    mut dnodes: *mut uint32_t,
    mut fnodes: *mut uint32_t,
) {
    unsafe {
        matocsserv_getspace(totalspace, availspace, freespace);
        *trspace = trashspace;
        *trnodes = trashnodes;
        *respace = sustainedspace;
        *renodes = sustainednodes;
        *inodes = nodes;
        *dnodes = dirnodes;
        *fnodes = filenodes;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_charts_data(
    mut file_objects: *mut uint32_t,
    mut meta_objects: *mut uint32_t,
) {
    unsafe {
        *file_objects = filenodes;
        *meta_objects = nodes.wrapping_sub(filenodes);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getrootinode(
    mut rootinode: *mut uint32_t,
    mut path: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut nleng: uint32_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        name = path;
        p = root;
        loop {
            while *name as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                name = name.offset(1);
            }
            if *name as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                *rootinode = (*p).inode;
                return MFS_STATUS_OK as uint8_t;
            }
            nleng = 0 as uint32_t;
            while *name.offset(nleng as isize) as ::core::ffi::c_int != 0
                && *name.offset(nleng as isize) as ::core::ffi::c_int != '/' as ::core::ffi::c_int
            {
                nleng = nleng.wrapping_add(1);
            }
            if fsnodes_namecheck(nleng, name) < 0 as ::core::ffi::c_int {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            e = fsnodes_lookup(p, nleng as uint16_t, name);
            if e.is_null() {
                return MFS_ERROR_ENOENT as uint8_t;
            }
            p = (*e).child as *mut fsnode;
            if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                return MFS_ERROR_ENOTDIR as uint8_t;
            }
            name = name.offset(nleng as isize);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_path_lookup(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut base_inode: uint32_t,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut parent_inode: *mut uint32_t,
    mut last_inode: *mut uint32_t,
    mut nleng: *mut uint8_t,
    mut name: *mut uint8_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut rn: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut pptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pend: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut partlen: uint8_t = 0;
        let mut emptypath: uint8_t = 0;
        *parent_inode = 0 as uint32_t;
        *last_inode = 0 as uint32_t;
        memset(
            attr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ATTR_RECORD_SIZE as size_t,
        );
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut base_inode,
            &raw mut rn,
            &raw mut wd,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT_NOCACHE as uint8_t;
        }
        emptypath = 1 as uint8_t;
        pptr = path;
        pend = path.offset(pleng as isize);
        partlen = 0 as uint8_t;
        *nleng = 0 as uint8_t;
        while *pptr as ::core::ffi::c_int != 0 && pptr < pend {
            if *pptr as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                if partlen as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    emptypath = 0 as uint8_t;
                    if (*wd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                        return MFS_ERROR_ENOTDIR as uint8_t;
                    }
                    if fsnodes_access_ext(wd, uid, gids, gid, MODE_MASK_X as uint8_t, sesflags) == 0
                    {
                        return MFS_ERROR_EACCES as uint8_t;
                    }
                    if partlen as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                        && *name.offset(0 as isize) as ::core::ffi::c_int
                            == '.' as ::core::ffi::c_int
                        && *name.offset(1 as isize) as ::core::ffi::c_int
                            == '.' as ::core::ffi::c_int
                    {
                        if wd == rn {
                            return MFS_ERROR_EPERM as uint8_t;
                        }
                        if !(*wd).parents.is_null() {
                            wd = (*(*wd).parents).parent as *mut fsnode;
                        } else {
                            return MFS_ERROR_ENOENT as uint8_t;
                        }
                    } else if partlen as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        || *name.offset(0 as isize) as ::core::ffi::c_int
                            != '.' as ::core::ffi::c_int
                    {
                        if fsnodes_namecheck(partlen as uint32_t, name as *const uint8_t)
                            < 0 as ::core::ffi::c_int
                        {
                            return MFS_ERROR_EINVAL as uint8_t;
                        }
                        e = fsnodes_lookup(wd, partlen as uint16_t, name as *const uint8_t);
                        if e.is_null() {
                            return MFS_ERROR_ENOENT as uint8_t;
                        } else {
                            wd = (*e).child as *mut fsnode;
                        }
                    }
                }
                partlen = 0 as uint8_t;
            } else {
                if partlen as ::core::ffi::c_int == MFS_NAME_MAX {
                    return MFS_ERROR_ENAMETOOLONG as uint8_t;
                }
                let c2rust_fresh21 = partlen;
                partlen = partlen.wrapping_add(1);
                *name.offset(c2rust_fresh21 as isize) = *pptr;
            }
            pptr = pptr.offset(1);
        }
        if partlen as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if (*wd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                return MFS_ERROR_ENOTDIR as uint8_t;
            }
            if fsnodes_access_ext(wd, uid, gids, gid, MODE_MASK_X as uint8_t, sesflags) == 0 {
                return MFS_ERROR_EACCES as uint8_t;
            }
            if partlen as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            {
                p = wd;
                wd = ::core::ptr::null_mut::<fsnode>();
                *nleng = 0 as uint8_t;
            } else if partlen as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                && *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && *name.offset(1 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            {
                if wd == rn {
                    return MFS_ERROR_EPERM as uint8_t;
                }
                if !(*wd).parents.is_null() {
                    p = (*(*wd).parents).parent as *mut fsnode;
                } else {
                    return MFS_ERROR_ENOENT as uint8_t;
                }
                wd = ::core::ptr::null_mut::<fsnode>();
                *nleng = 0 as uint8_t;
            } else {
                if fsnodes_namecheck(partlen as uint32_t, name as *const uint8_t)
                    < 0 as ::core::ffi::c_int
                {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
                e = fsnodes_lookup(wd, partlen as uint16_t, name as *const uint8_t);
                if e.is_null() {
                    p = ::core::ptr::null_mut::<fsnode>();
                } else {
                    p = (*e).child as *mut fsnode;
                }
                *nleng = partlen;
            }
        } else if emptypath != 0 {
            p = wd;
            wd = ::core::ptr::null_mut::<fsnode>();
            *nleng = 0 as uint8_t;
        } else {
            p = ::core::ptr::null_mut::<fsnode>();
        }
        if !wd.is_null() {
            if wd == rn {
                *parent_inode = MFS_ROOT_ID as uint32_t;
            } else {
                *parent_inode = (*wd).inode;
            }
        }
        if !p.is_null() {
            if p == rn {
                *last_inode = MFS_ROOT_ID as uint32_t;
            } else {
                *last_inode = (*p).inode;
            }
            fsnodes_fill_attr(
                p,
                wd,
                uid,
                *gid.offset(0 as isize),
                auid,
                agid,
                sesflags,
                attr,
                1 as uint8_t,
            );
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_statfs(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut totalspace: *mut uint64_t,
    mut availspace: *mut uint64_t,
    mut freespace: *mut uint64_t,
    mut trspace: *mut uint64_t,
    mut respace: *mut uint64_t,
    mut inodes: *mut uint32_t,
) {
    unsafe {
        let mut rn: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        if rootinode == MFS_ROOT_ID as uint32_t {
            *trspace = trashspace;
            *respace = sustainedspace;
            rn = root;
        } else {
            *trspace = 0 as uint64_t;
            *respace = 0 as uint64_t;
            rn = fsnodes_node_find(rootinode);
        }
        if rn.is_null() || (*rn).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            *totalspace = 0 as uint64_t;
            *availspace = 0 as uint64_t;
            *freespace = 0 as uint64_t;
            *inodes = 0 as uint32_t;
        } else {
            matocsserv_getspace(totalspace, availspace, freespace);
            fsnodes_quota_fixspace(rn, totalspace, availspace);
            fsnodes_quota_fixspace(rn, totalspace, freespace);
            fsnodes_get_stats(rn, &raw mut sr, 2 as uint8_t);
            *inodes = sr.inodes;
        }
        stats_statfs = stats_statfs.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_access(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut modemask: ::core::ffi::c_int,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 && modemask & MODE_MASK_W != 0 {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            1 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 && modemask & MODE_MASK_W != 0 {
            return MFS_ERROR_EPERM as uint8_t;
        }
        return (if fsnodes_access_ext(p, uid, gids, gid, modemask as uint8_t, sesflags) != 0 {
            MFS_STATUS_OK
        } else {
            MFS_ERROR_EACCES
        }) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_lookup(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
    mut allow_recover: uint8_t,
    mut accmode: *mut uint16_t,
    mut filenode: *mut uint8_t,
    mut validchunk: *mut uint8_t,
    mut chunkid: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut wd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut rn: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        *inode = 0 as uint32_t;
        memset(
            attr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ATTR_RECORD_SIZE as size_t,
        );
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut parent,
            &raw mut rn,
            &raw mut wd,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT_NOCACHE as uint8_t;
        }
        if (*wd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        if fsnodes_access_ext(wd, uid, gids, gid, MODE_MASK_X as uint8_t, sesflags) == 0 {
            return MFS_ERROR_EACCES as uint8_t;
        }
        p = ::core::ptr::null_mut::<fsnode>();
        if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            if nleng as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                if parent == rootinode {
                    *inode = MFS_ROOT_ID as uint32_t;
                } else {
                    *inode = (*wd).inode;
                }
                p = wd;
            }
            if nleng as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                && *name.offset(1 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            {
                if parent == rootinode {
                    *inode = MFS_ROOT_ID as uint32_t;
                    p = wd;
                } else if !(*wd).parents.is_null() {
                    if (*(*(*wd).parents).parent).inode == rootinode {
                        *inode = MFS_ROOT_ID as uint32_t;
                    } else {
                        *inode = (*(*(*wd).parents).parent).inode;
                    }
                    p = (*(*wd).parents).parent as *mut fsnode;
                } else {
                    *inode = MFS_ROOT_ID as uint32_t;
                    p = rn;
                }
            }
        }
        if p.is_null() {
            if fsnodes_namecheck(nleng as uint32_t, name) < 0 as ::core::ffi::c_int {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            e = fsnodes_lookup(wd, nleng, name);
            if e.is_null() {
                if (*wd).eattr as ::core::ffi::c_int & EATTR_NOECACHE != 0 {
                    return MFS_ERROR_ENOENT_NOCACHE as uint8_t;
                } else {
                    return MFS_ERROR_ENOENT as uint8_t;
                }
            }
            p = (*e).child as *mut fsnode;
            *inode = (*p).inode;
        }
        if !filenode.is_null() {
            *filenode = (if (*p).r#type() as ::core::ffi::c_int == TYPE_FILE
                || (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH
                || (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
        }
        fsnodes_fill_attr(
            p,
            wd,
            uid,
            *gid.offset(0 as isize),
            auid,
            agid,
            sesflags,
            attr,
            1 as uint8_t,
        );
        if !accmode.is_null() {
            *accmode = fsnodes_accessmode(p, uid, gids, gid, sesflags) as uint16_t;
            if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 {
                *accmode = (*accmode as ::core::ffi::c_int & LOOKUP_ACCESS_MODES_RO) as uint16_t;
                *accmode = (*accmode as ::core::ffi::c_int | LOOKUP_IMMUTABLE) as uint16_t;
            }
            if (*p).eattr as ::core::ffi::c_int & EATTR_APPENDONLY != 0 {
                *accmode = (*accmode as ::core::ffi::c_int | LOOKUP_APPENDONLY) as uint16_t;
            }
            if (*p).eattr as ::core::ffi::c_int & EATTR_NODATACACHE != 0 {
                *accmode = (*accmode as ::core::ffi::c_int | LOOKUP_DIRECTMODE) as uint16_t;
            }
        }
        if !validchunk.is_null() && !chunkid.is_null() {
            *validchunk = 0 as uint8_t;
            *chunkid = 0 as uint64_t;
            if (*p).r#type() as ::core::ffi::c_int == TYPE_FILE
                || (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH
                || (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
            {
                if (*p).data.fdata.chunks >= 1 as uint32_t {
                    *chunkid = *(*p).data.fdata.chunktab.offset(0 as isize);
                    if *chunkid == 0 as uint64_t {
                        *validchunk = 1 as uint8_t;
                    } else if chunk_read_check(main_time(), *chunkid, allow_recover)
                        == MFS_STATUS_OK
                    {
                        *validchunk = 1 as uint8_t;
                    }
                }
            }
        }
        stats_lookup = stats_lookup.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getattr(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        memset(
            attr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ATTR_RECORD_SIZE as size_t,
        );
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            opened,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        fsnodes_fill_attr(
            p,
            ::core::ptr::null_mut::<fsnode>(),
            uid,
            gid,
            auid,
            agid,
            sesflags,
            attr,
            1 as uint8_t,
        );
        stats_getattr = stats_getattr.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_try_setlength(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut disflags: uint8_t,
    mut length: uint64_t,
    mut indx: *mut uint32_t,
    mut prevchunkid: *mut uint64_t,
    mut chunkid: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            (flags as ::core::ffi::c_int & TRUNCATE_FLAG_OPENED) as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_APPENDONLY != 0
            && flags as ::core::ffi::c_int & (TRUNCATE_FLAG_RESERVE | TRUNCATE_FLAG_UPDATE)
                == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if flags as ::core::ffi::c_int & TRUNCATE_FLAG_OPENED == 0 as ::core::ffi::c_int {
            if fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_W as uint8_t, sesflags) == 0 {
                return MFS_ERROR_EACCES as uint8_t;
            }
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if flags as ::core::ffi::c_int & TRUNCATE_FLAG_UPDATE != 0 {
            return MFS_STATUS_OK as uint8_t;
        }
        if flags as ::core::ffi::c_int & TRUNCATE_FLAG_RESERVE != 0 {
            let mut vleng: uint64_t = 0;
            vleng = appendres_getvleng(inode);
            if vleng < (*p).data.fdata.length {
                vleng = (*p).data.fdata.length;
            }
            if length.wrapping_add(vleng) < length {
                return MFS_ERROR_INDEXTOOBIG as uint8_t;
            }
            length = length.wrapping_add(vleng);
        } else {
            if disflags as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                if length < (*p).data.fdata.length {
                    return MFS_ERROR_EPERM as uint8_t;
                }
            }
            if disflags as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                if length > (*p).data.fdata.length {
                    return MFS_ERROR_EPERM as uint8_t;
                }
            }
        }
        if length > (*p).data.fdata.length {
            let mut lastchunk_pre: uint32_t = 0;
            let mut lastchunksize_pre: uint32_t = 0;
            let mut lastchunk_post: uint32_t = 0;
            let mut lastchunksize_post: uint32_t = 0;
            let mut size_diff: uint64_t = 0;
            if (*p).data.fdata.length > 0 as uint64_t {
                lastchunk_pre = ((*p).data.fdata.length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS)
                    as uint32_t;
                lastchunksize_pre = (((*p).data.fdata.length.wrapping_sub(1 as uint64_t)
                    & MFSCHUNKMASK as uint64_t)
                    .wrapping_add(MFSBLOCKSIZE as uint64_t)
                    & MFSBLOCKNEGMASK as uint64_t)
                    .wrapping_add(MFSHDRSIZE as uint64_t)
                    as uint32_t;
            } else {
                lastchunk_pre = 0 as uint32_t;
                lastchunksize_pre = MFSHDRSIZE as uint32_t;
            }
            if length > 0 as uint64_t {
                lastchunk_post = (length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS) as uint32_t;
                lastchunksize_post =
                    ((length.wrapping_sub(1 as uint64_t) & MFSCHUNKMASK as uint64_t)
                        .wrapping_add(MFSBLOCKSIZE as uint64_t)
                        & MFSBLOCKNEGMASK as uint64_t)
                        .wrapping_add(MFSHDRSIZE as uint64_t) as uint32_t;
            } else {
                lastchunk_post = 0 as uint32_t;
                lastchunksize_post = MFSHDRSIZE as uint32_t;
            }
            if lastchunk_post > MAX_INDEX as uint32_t {
                return MFS_ERROR_INDEXTOOBIG as uint8_t;
            }
            if (*p).data.fdata.chunktab.is_null()
                || *(*p).data.fdata.chunktab.offset(lastchunk_pre as isize) == 0 as uint64_t
            {
                size_diff = 0 as uint64_t;
            } else if lastchunk_post > lastchunk_pre {
                size_diff = ((MFSCHUNKSIZE + MFSHDRSIZE) as uint32_t)
                    .wrapping_sub(lastchunksize_pre) as uint64_t;
            } else {
                size_diff = lastchunksize_post.wrapping_sub(lastchunksize_pre) as uint64_t;
            }
            if fsnodes_test_quota(
                p,
                0 as uint32_t,
                length.wrapping_sub((*p).data.fdata.length),
                size_diff,
                (sclass_get_keeparch_maxstorage_eights((*p).sclassid as uint16_t) as uint64_t)
                    .wrapping_mul(size_diff)
                    .wrapping_div(8 as uint64_t),
            ) != 0
            {
                return MFS_ERROR_QUOTA as uint8_t;
            }
        }
        if flags as ::core::ffi::c_int & TRUNCATE_FLAG_RESERVE == 0 as ::core::ffi::c_int
            && length != (*p).data.fdata.length
        {
            if length & MFSCHUNKMASK as uint64_t != 0 {
                *indx = (length >> MFSCHUNKBITS) as uint32_t;
                if *indx < (*p).data.fdata.chunks {
                    let mut ochunkid: uint64_t = *(*p).data.fdata.chunktab.offset(*indx as isize);
                    if ochunkid > 0 as uint64_t {
                        let mut status: uint8_t = 0;
                        let mut nchunkid: uint64_t = 0;
                        status = chunk_multi_truncate(
                            &raw mut nchunkid,
                            ochunkid,
                            (length & MFSCHUNKMASK as uint64_t) as uint32_t,
                            (*p).sclassid,
                        ) as uint8_t;
                        *prevchunkid = ochunkid;
                        if status as ::core::ffi::c_int != MFS_STATUS_OK {
                            return status;
                        }
                        *(*p).data.fdata.chunktab.offset(*indx as isize) = nchunkid;
                        *chunkid = nchunkid;
                        changelog(
                            b"%u|TRUNC(%u,%u):%lu\0".as_ptr() as *const ::core::ffi::c_char,
                            main_time(),
                            inode,
                            *indx,
                            nchunkid,
                        );
                        return MFS_ERROR_DELAYED as uint8_t;
                    }
                }
            }
        }
        stats_truncate = stats_truncate.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_trunc(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut nchunkid: uint64_t,
) -> uint8_t {
    unsafe {
        let mut ochunkid: uint64_t = 0;
        let mut status: uint8_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if indx > MAX_INDEX as uint32_t {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        if indx >= (*p).data.fdata.chunks {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        ochunkid = *(*p).data.fdata.chunktab.offset(indx as isize);
        status = chunk_mr_multi_truncate(ts, &raw mut nchunkid, ochunkid, (*p).sclassid) as uint8_t;
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        *(*p).data.fdata.chunktab.offset(indx as isize) = nchunkid;
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_end_setlength(mut chunkid: uint64_t) -> uint8_t {
    unsafe {
        let mut ts: uint32_t = 0;
        ts = main_time();
        changelog(
            b"%u|UNLOCK(%lu)\0".as_ptr() as *const ::core::ffi::c_char,
            ts,
            chunkid,
        );
        return chunk_unlock(ts, chunkid) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_unlock(mut ts: uint32_t, mut chunkid: uint64_t) -> uint8_t {
    unsafe {
        let mut status: uint8_t = 0;
        status = chunk_mr_unlock(ts, chunkid) as uint8_t;
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            meta_version_inc();
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_do_setlength(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut length: uint64_t,
    mut attr: *mut uint8_t,
    mut prevlength: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ts: uint32_t = main_time();
        let mut chtime: uint8_t = 1 as uint8_t;
        memset(
            attr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ATTR_RECORD_SIZE as size_t,
        );
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            (flags as ::core::ffi::c_int & TRUNCATE_FLAG_OPENED) as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if flags as ::core::ffi::c_int & TRUNCATE_FLAG_RESERVE != 0 {
            let mut vleng: uint64_t = 0;
            vleng = appendres_getvleng(inode);
            if vleng < (*p).data.fdata.length {
                vleng = (*p).data.fdata.length;
            }
            *prevlength = vleng;
            length = length.wrapping_add(vleng);
            appendres_setvleng(inode, length);
        } else {
            *prevlength = (*p).data.fdata.length;
            if flags as ::core::ffi::c_int & TRUNCATE_FLAG_UPDATE != 0 {
                if length > (*p).data.fdata.length {
                    fsnodes_setlength(p, length);
                    changelog(
                        b"%u|LENGTH(%u,%lu,0)\0".as_ptr() as *const ::core::ffi::c_char,
                        ts,
                        inode,
                        (*p).data.fdata.length,
                    );
                    fsnodes_checkarchmode(p, ts, 0 as uint8_t);
                }
            } else {
                if length == (*p).data.fdata.length
                    && flags as ::core::ffi::c_int & TRUNCATE_FLAG_TIMEFIX != 0
                {
                    chtime = 0 as uint8_t;
                }
                fsnodes_setlength(p, length);
                changelog(
                    b"%u|LENGTH(%u,%lu,%hhu)\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    inode,
                    (*p).data.fdata.length,
                    chtime as ::core::ffi::c_int,
                );
                if chtime != 0 {
                    (*p).mtime = ts;
                    (*p).ctime = (*p).mtime;
                    fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME) as uint8_t);
                } else {
                    fsnodes_checkarchmode(p, ts, 0 as uint8_t);
                }
                stats_truncate = stats_truncate.wrapping_add(1);
            }
            appendres_clear(inode);
        }
        fsnodes_fill_attr(
            p,
            ::core::ptr::null_mut::<fsnode>(),
            uid,
            gid,
            auid,
            agid,
            sesflags,
            attr,
            1 as uint8_t,
        );
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_setattr(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut setmask: uint8_t,
    mut attrmode: uint16_t,
    mut attruid: uint32_t,
    mut attrgid: uint32_t,
    mut attratime: uint32_t,
    mut attrmtime: uint32_t,
    mut winattr: uint8_t,
    mut sugidclearmode: uint8_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut gf: uint8_t = 0;
        let mut i: uint32_t = 0;
        let mut ts: uint32_t = main_time();
        memset(
            attr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ATTR_RECORD_SIZE as size_t,
        );
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            opened,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if !(uid != 0 as uint32_t
            && setmask as ::core::ffi::c_int == SET_MODE_FLAG
            && attrmode as ::core::ffi::c_int
                == (*p).mode() as ::core::ffi::c_int & 0o1777 as ::core::ffi::c_int
            && (*p).mode() as ::core::ffi::c_int & 0o6000 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
        {
            if uid != 0 as uint32_t
                && sesflags as ::core::ffi::c_int & SESFLAG_MAPALL != 0
                && setmask as ::core::ffi::c_int & (SET_UID_FLAG | SET_GID_FLAG) != 0
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if (*p).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int {
                if uid != 0 as uint32_t
                    && uid != (*p).uid
                    && setmask as ::core::ffi::c_int
                        & (SET_MODE_FLAG
                            | SET_UID_FLAG
                            | SET_GID_FLAG
                            | SET_ATIME_FLAG
                            | SET_MTIME_FLAG)
                        != 0
                {
                    return MFS_ERROR_EPERM as uint8_t;
                }
                if uid != 0 as uint32_t
                    && uid != (*p).uid
                    && setmask as ::core::ffi::c_int & (SET_ATIME_NOW_FLAG | SET_MTIME_NOW_FLAG)
                        != 0
                {
                    if fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_W as uint8_t, sesflags) == 0
                    {
                        return MFS_ERROR_EACCES as uint8_t;
                    }
                }
            }
            if uid != 0 as uint32_t
                && uid != attruid
                && setmask as ::core::ffi::c_int & SET_UID_FLAG != 0
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_IGNOREGID == 0 as ::core::ffi::c_int {
                if uid != 0 as uint32_t && setmask as ::core::ffi::c_int & SET_GID_FLAG != 0 {
                    gf = 0 as uint8_t;
                    i = 0 as uint32_t;
                    while i < gids && gf as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        if *gid.offset(i as isize) == attrgid {
                            gf = 1 as uint8_t;
                        }
                        i = i.wrapping_add(1);
                    }
                    if gf as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        return MFS_ERROR_EPERM as uint8_t;
                    }
                }
            }
        }
        if setmask as ::core::ffi::c_int & (SET_UID_FLAG | SET_GID_FLAG) != 0
            && setmask as ::core::ffi::c_int & SET_MODE_FLAG != 0
        {
            attrmode = (attrmode as ::core::ffi::c_int
                | (*p).mode() as ::core::ffi::c_int & 0o6000 as ::core::ffi::c_int)
                as uint16_t;
        }
        if (*p).mode() as ::core::ffi::c_int & 0o6000 as ::core::ffi::c_int != 0
            && setmask as ::core::ffi::c_int & (SET_UID_FLAG | SET_GID_FLAG) != 0
        {
            match sugidclearmode as ::core::ffi::c_int {
                SUGID_CLEAR_MODE_ALWAYS => {
                    (*p).set_mode(
                        (*p).mode() & 0o1777 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    );
                    attrmode =
                        (attrmode as ::core::ffi::c_int & 0o1777 as ::core::ffi::c_int) as uint16_t;
                }
                SUGID_CLEAR_MODE_OSX => {
                    if uid != 0 as uint32_t {
                        (*p).set_mode(
                            (*p).mode() & 0o1777 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        );
                        attrmode = (attrmode as ::core::ffi::c_int & 0o1777 as ::core::ffi::c_int)
                            as uint16_t;
                    }
                }
                SUGID_CLEAR_MODE_BSD => {
                    if uid != 0 as uint32_t
                        && setmask as ::core::ffi::c_int & SET_GID_FLAG != 0
                        && (*p).gid != attrgid
                    {
                        (*p).set_mode(
                            (*p).mode() & 0o1777 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        );
                        attrmode = (attrmode as ::core::ffi::c_int & 0o1777 as ::core::ffi::c_int)
                            as uint16_t;
                    }
                }
                SUGID_CLEAR_MODE_EXT => {
                    if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                        if (*p).mode() as ::core::ffi::c_int & 0o10 as ::core::ffi::c_int != 0 {
                            (*p).set_mode(
                                (*p).mode() & 0o1777 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                            attrmode = (attrmode as ::core::ffi::c_int
                                & 0o1777 as ::core::ffi::c_int)
                                as uint16_t;
                        } else {
                            (*p).set_mode(
                                (*p).mode() & 0o3777 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                            attrmode = (attrmode as ::core::ffi::c_int
                                & 0o3777 as ::core::ffi::c_int)
                                as uint16_t;
                        }
                    }
                }
                SUGID_CLEAR_MODE_XFS => {
                    if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                        if (*p).mode() as ::core::ffi::c_int & 0o10 as ::core::ffi::c_int != 0 {
                            (*p).set_mode(
                                (*p).mode() & 0o1777 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                            attrmode = (attrmode as ::core::ffi::c_int
                                & 0o1777 as ::core::ffi::c_int)
                                as uint16_t;
                        } else {
                            (*p).set_mode(
                                (*p).mode() & 0o3777 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            );
                            attrmode = (attrmode as ::core::ffi::c_int
                                & 0o3777 as ::core::ffi::c_int)
                                as uint16_t;
                        }
                    } else if uid != 0 as uint32_t {
                        (*p).set_mode(
                            (*p).mode() & 0o1777 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        );
                        attrmode = (attrmode as ::core::ffi::c_int & 0o1777 as ::core::ffi::c_int)
                            as uint16_t;
                    }
                }
                _ => {}
            }
        }
        if setmask as ::core::ffi::c_int & SET_UID_FLAG != 0 {
            (*p).uid = attruid;
        }
        if setmask as ::core::ffi::c_int & SET_GID_FLAG != 0 {
            (*p).gid = attrgid;
        }
        if setmask as ::core::ffi::c_int & SET_MODE_FLAG != 0 {
            if uid != 0 as uint32_t
                && attrmode as ::core::ffi::c_int & 0o2000 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                gf = 0 as uint8_t;
                i = 0 as uint32_t;
                while i < gids && gf as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if *gid.offset(i as isize) == (*p).gid {
                        gf = 1 as uint8_t;
                    }
                    i = i.wrapping_add(1);
                }
                if gf as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    attrmode =
                        (attrmode as ::core::ffi::c_int & 0o5777 as ::core::ffi::c_int) as uint16_t;
                }
            }
            if (*p).aclpermflag() != 0 {
                posix_acl_setmode((*p).inode, attrmode);
                (*p).set_mode((*p).mode() & 0o70 as ::core::ffi::c_int as ::core::ffi::c_uint);
                attrmode =
                    (attrmode as ::core::ffi::c_int & 0o7707 as ::core::ffi::c_int) as uint16_t;
                (*p).set_mode((*p).mode() | attrmode as ::core::ffi::c_int as ::core::ffi::c_uint);
            } else {
                (*p).set_mode(attrmode as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        if setmask as ::core::ffi::c_int & SET_ATIME_FLAG != 0 {
            (*p).atime = attratime;
        }
        if setmask as ::core::ffi::c_int & SET_MTIME_FLAG != 0 {
            (*p).mtime = attrmtime;
        }
        if setmask as ::core::ffi::c_int & SET_ATIME_NOW_FLAG != 0 {
            (*p).atime = ts;
        }
        if setmask as ::core::ffi::c_int & SET_MTIME_NOW_FLAG != 0 {
            (*p).mtime = ts;
        }
        if setmask as ::core::ffi::c_int & SET_WINATTR_FLAG != 0 {
            (*p).winattr = winattr;
        }
        changelog(
            b"%u|ATTR(%u,%hu,%u,%u,%u,%u,%hhu,%hu)\0".as_ptr() as *const ::core::ffi::c_char,
            ts,
            inode,
            (*p).mode() as uint16_t as ::core::ffi::c_int,
            (*p).uid,
            (*p).gid,
            (*p).atime,
            (*p).mtime,
            (*p).winattr as ::core::ffi::c_int,
            (if (*p).aclpermflag() as ::core::ffi::c_int != 0 {
                ((posix_acl_getmode((*p).inode) as ::core::ffi::c_int
                    & 0o7777 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    .wrapping_add((1 as ::core::ffi::c_uint) << 12 as ::core::ffi::c_int)
            } else {
                0 as ::core::ffi::c_uint
            }) as uint16_t as ::core::ffi::c_int,
        );
        (*p).ctime = ts;
        fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME | CHECK_ATIME) as uint8_t);
        fsnodes_fill_attr(
            p,
            ::core::ptr::null_mut::<fsnode>(),
            uid,
            *gid.offset(0 as isize),
            auid,
            agid,
            sesflags,
            attr,
            1 as uint8_t,
        );
        stats_setattr = stats_setattr.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_attr(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut mode: uint16_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut atime: uint32_t,
    mut mtime: uint32_t,
    mut winattr: uint8_t,
    mut aclmode: uint16_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if mode as ::core::ffi::c_int > 0o7777 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        (*p).set_mode(mode as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if (*p).aclpermflag() != 0 {
            if aclmode as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                aclmode = (mode as ::core::ffi::c_int & 0o707 as ::core::ffi::c_int) as uint16_t;
                aclmode = (aclmode as ::core::ffi::c_int | 0o70 as ::core::ffi::c_int) as uint16_t;
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"set attributes for inode %u with posix acl - emergency set mask to 'rwx' - upgrade all masters to newest version and check ACL's for this inode\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    inode,
                );
            }
            posix_acl_setmode((*p).inode, aclmode);
        }
        (*p).uid = uid;
        (*p).gid = gid;
        (*p).atime = atime;
        (*p).mtime = mtime;
        (*p).ctime = ts;
        (*p).winattr = winattr;
        fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME | CHECK_ATIME) as uint8_t);
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_length(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut length: uint64_t,
    mut canmodmtime: uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        fsnodes_setlength(p, length);
        if canmodmtime != 0 {
            (*p).ctime = ts;
            (*p).mtime = (*p).ctime;
            fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME) as uint8_t);
        } else {
            fsnodes_checkarchmode(p, ts, 0 as uint8_t);
        }
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_set_additional_attributes(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut uid: uint32_t,
    mut data: *const uint8_t,
    mut leng: uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut checkonly: uint8_t = 0;
        let mut winattr: uint8_t = 0;
        let mut eattr: uint8_t = 0;
        let mut xattrblob: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pleng: uint32_t = 0;
        let mut same: uint8_t = 0;
        let mut aclmask: uint8_t = 0;
        let mut defaultacl: facldata = facldata {
            valid: 0,
            userperm: 0,
            groupperm: 0,
            otherperm: 0,
            mask: 0,
            namedusers: 0,
            namedgroups: 0,
            nameddataptr: ::core::ptr::null::<uint8_t>(),
        };
        let mut accessacl: facldata = facldata {
            valid: 0,
            userperm: 0,
            groupperm: 0,
            otherperm: 0,
            mask: 0,
            namedusers: 0,
            namedgroups: 0,
            nameddataptr: ::core::ptr::null::<uint8_t>(),
        };
        let mut curracl: *mut facldata = ::core::ptr::null_mut::<facldata>();
        let mut blob: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut bleng: uint32_t = 0;
        blob = data;
        bleng = leng;
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int
            && uid != 0 as uint32_t
            && uid != (*p).uid
        {
            checkonly = 1 as uint8_t;
        } else {
            checkonly = 0 as uint8_t;
        }
        winattr = 0 as uint8_t;
        eattr = 0 as uint8_t;
        same = 0 as uint8_t;
        aclmask = 0 as uint8_t;
        xattrblob = ::core::ptr::null::<uint8_t>();
        defaultacl.valid = 0 as uint8_t;
        accessacl.valid = 0 as uint8_t;
        if flags as ::core::ffi::c_int & SET_ALL_WINATTR != 0 {
            if leng < 1 as uint32_t {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            winattr = get8bit(&raw mut data);
            leng = leng.wrapping_sub(1);
        }
        if flags as ::core::ffi::c_int & SET_ALL_EATTR != 0 {
            if leng < 1 as uint32_t {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            eattr = get8bit(&raw mut data);
            leng = leng.wrapping_sub(1);
            if eattr as ::core::ffi::c_int
                & !(EATTR_NOOWNER
                    | EATTR_NOACACHE
                    | EATTR_NOECACHE
                    | EATTR_NODATACACHE
                    | EATTR_SNAPSHOT
                    | EATTR_UNDELETABLE
                    | EATTR_APPENDONLY
                    | EATTR_IMMUTABLE)
                != 0
            {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        }
        if flags as ::core::ffi::c_int & SET_ALL_XATTR != 0 {
            same = xattr_check((*p).inode, data, leng, &raw mut pleng);
            if same as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            xattrblob = data;
            data = data.offset(pleng as isize);
            leng = leng.wrapping_sub(pleng);
        }
        if flags as ::core::ffi::c_int & SET_ALL_FACL != 0 {
            let mut i: uint8_t = 0;
            let mut testmask: uint8_t = 0;
            let mut naclleng: uint32_t = 0;
            if leng < 1 as uint32_t {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            aclmask = get8bit(&raw mut data);
            leng = leng.wrapping_sub(1);
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
                if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    curracl = &raw mut accessacl;
                    testmask = 1 as uint8_t;
                } else {
                    curracl = &raw mut defaultacl;
                    testmask = 2 as uint8_t;
                }
                if aclmask as ::core::ffi::c_int & testmask as ::core::ffi::c_int != 0 {
                    if leng < 12 as uint32_t {
                        return MFS_ERROR_EINVAL as uint8_t;
                    }
                    (*curracl).valid = 1 as uint8_t;
                    (*curracl).userperm = get16bit(&raw mut data);
                    (*curracl).groupperm = get16bit(&raw mut data);
                    (*curracl).otherperm = get16bit(&raw mut data);
                    (*curracl).mask = get16bit(&raw mut data);
                    (*curracl).namedusers = get16bit(&raw mut data);
                    (*curracl).namedgroups = get16bit(&raw mut data);
                    leng = leng.wrapping_sub(12 as uint32_t);
                    naclleng = ((*curracl).namedusers as uint32_t)
                        .wrapping_add((*curracl).namedgroups as uint32_t)
                        .wrapping_mul(6 as uint32_t);
                    if leng < naclleng {
                        return MFS_ERROR_EINVAL as uint8_t;
                    }
                    (*curracl).nameddataptr = data;
                    data = data.offset(naclleng as isize);
                    leng = leng.wrapping_sub(naclleng);
                } else {
                    (*curracl).valid = 0 as uint8_t;
                }
                i = i.wrapping_add(1);
            }
        }
        if leng > 0 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if flags as ::core::ffi::c_int & SET_ALL_WINATTR != 0 {
            if checkonly != 0 {
                if (*p).winattr as ::core::ffi::c_int != winattr as ::core::ffi::c_int {
                    return MFS_ERROR_EPERM as uint8_t;
                }
            } else {
                (*p).winattr = winattr;
                stats_setattr = stats_setattr.wrapping_add(1);
            }
        }
        if flags as ::core::ffi::c_int & SET_ALL_EATTR != 0 {
            if checkonly != 0 {
                if (*p).eattr as ::core::ffi::c_int != eattr as ::core::ffi::c_int {
                    return MFS_ERROR_EPERM as uint8_t;
                }
            } else {
                (*p).eattr = eattr;
                stats_meta = stats_meta.wrapping_add(1);
            }
        }
        if flags as ::core::ffi::c_int & SET_ALL_XATTR != 0 {
            if checkonly != 0 {
                if same as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return MFS_ERROR_EPERM as uint8_t;
                }
            } else if same as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*p).set_xattrflag(xattr_setall((*p).inode, xattrblob) as ::core::ffi::c_uint
                    as ::core::ffi::c_uint);
            }
        }
        if flags as ::core::ffi::c_int & SET_ALL_FACL != 0 {
            let mut pmode: uint16_t = 0;
            if checkonly != 0 {
                if defaultacl.valid != 0 {
                    if (*p).acldefflag() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        return MFS_ERROR_EPERM as uint8_t;
                    }
                    if posix_acl_check(
                        (*p).inode,
                        POSIX_ACL_DEFAULT as uint8_t,
                        defaultacl.userperm,
                        defaultacl.groupperm,
                        defaultacl.otherperm,
                        defaultacl.mask,
                        defaultacl.namedusers,
                        defaultacl.namedgroups,
                        defaultacl.nameddataptr,
                    ) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        return MFS_ERROR_EPERM as uint8_t;
                    }
                } else if (*p).acldefflag() != 0 {
                    return MFS_ERROR_EPERM as uint8_t;
                }
                if accessacl.valid != 0 {
                    if (*p).aclpermflag() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        return MFS_ERROR_EPERM as uint8_t;
                    }
                    if posix_acl_check(
                        (*p).inode,
                        POSIX_ACL_ACCESS as uint8_t,
                        accessacl.userperm,
                        accessacl.groupperm,
                        accessacl.otherperm,
                        accessacl.mask,
                        accessacl.namedusers,
                        accessacl.namedgroups,
                        accessacl.nameddataptr,
                    ) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        return MFS_ERROR_EPERM as uint8_t;
                    }
                } else if (*p).aclpermflag() != 0 {
                    return MFS_ERROR_EPERM as uint8_t;
                }
            } else {
                if defaultacl.valid != 0 {
                    posix_acl_set(
                        (*p).inode,
                        POSIX_ACL_DEFAULT as uint8_t,
                        defaultacl.userperm,
                        defaultacl.groupperm,
                        defaultacl.otherperm,
                        defaultacl.mask,
                        defaultacl.namedusers,
                        defaultacl.namedgroups,
                        defaultacl.nameddataptr,
                    );
                } else if (*p).acldefflag() != 0 {
                    posix_acl_remove((*p).inode, POSIX_ACL_DEFAULT as uint8_t);
                    (*p).set_acldefflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                if accessacl.valid != 0 {
                    pmode = (*p).mode() as uint16_t;
                    posix_acl_set(
                        (*p).inode,
                        POSIX_ACL_ACCESS as uint8_t,
                        accessacl.userperm,
                        accessacl.groupperm,
                        accessacl.otherperm,
                        accessacl.mask,
                        accessacl.namedusers,
                        accessacl.namedgroups,
                        accessacl.nameddataptr,
                    );
                    (*p).set_mode(
                        (*p).mode() & 0o7000 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    );
                    (*p).set_mode(
                        (*p).mode()
                            | ((accessacl.userperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                                << 6 as ::core::ffi::c_int
                                | (accessacl.groupperm as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                    << 3 as ::core::ffi::c_int
                                | accessacl.otherperm as ::core::ffi::c_int
                                    & 7 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint,
                    );
                    if (*p).mode() as ::core::ffi::c_int != pmode as ::core::ffi::c_int {
                        (*p).ctime = ts;
                    }
                } else if (*p).aclpermflag() != 0 {
                    posix_acl_remove((*p).inode, POSIX_ACL_ACCESS as uint8_t);
                    (*p).set_aclpermflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|ADDATTR(%u,%u,%s)\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                (*p).inode,
                flags as ::core::ffi::c_int,
                changelog_escape_name(bleng, blob),
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_set_additional_attributes(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut uid: uint32_t,
    mut data: *const uint8_t,
    mut leng: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_set_additional_attributes(
            main_time(),
            rootinode,
            sesflags,
            inode,
            flags,
            uid,
            data,
            leng,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_additionalattr(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut data: *const uint8_t,
    mut leng: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_set_additional_attributes(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            inode,
            flags,
            0 as uint32_t,
            data,
            leng,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readlink(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut pleng: *mut uint32_t,
    mut path: *mut *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ts: uint32_t = main_time();
        *pleng = 0 as uint32_t;
        *path = ::core::ptr::null_mut::<uint8_t>();
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_SYMLINK {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        *pleng = (*p).data.sdata.pleng;
        *path = (*p).data.sdata.path;
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY == 0 as ::core::ffi::c_int
            && (*p).atime != ts
        {
            if AtimeMode as ::core::ffi::c_int == ATIME_ALWAYS
                || ((*p).atime <= (*p).ctime && ts >= (*p).ctime
                    || (*p).atime <= (*p).mtime && ts >= (*p).mtime
                    || (*p).atime.wrapping_add(86400 as uint32_t) < ts)
                    && AtimeMode as ::core::ffi::c_int == ATIME_RELATIVE_ONLY
            {
                (*p).atime = ts;
                fsnodes_checkarchmode(p, ts, CHECK_ATIME as uint8_t);
                changelog(
                    b"%u|ACCESS(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    inode,
                );
            }
        }
        stats_readlink = stats_readlink.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_symlink(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut newpath: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut i: uint32_t = 0;
        *inode = 0 as uint32_t;
        if !attr.is_null() {
            memset(
                attr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ATTR_RECORD_SIZE as size_t,
            );
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if pleng == 0 as uint32_t || pleng > MFS_SYMLINK_MAX as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        i = 0 as uint32_t;
        while i < pleng {
            if *path.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            i = i.wrapping_add(1);
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut parent,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut wd,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*wd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*wd).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(
                wd,
                uid,
                gids,
                gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                sesflags,
            ) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if fsnodes_namecheck(nleng as uint32_t, name) < 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if fsnodes_nameisused(wd, nleng, name) != 0 {
            return MFS_ERROR_EEXIST as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_test_quota(
                wd,
                1 as uint32_t,
                pleng as uint64_t,
                0 as uint64_t,
                0 as uint64_t,
            ) as ::core::ffi::c_int
                != 0
        {
            return MFS_ERROR_QUOTA as uint8_t;
        }
        newpath = symlink_malloc(pleng as uint16_t);
        if newpath.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                6234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newpath\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                6234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newpath\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if newpath
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                6234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newpath\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                6234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"newpath\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        p = fsnodes_create_node(
            ts,
            wd,
            nleng,
            name,
            TYPE_SYMLINK as uint8_t,
            0o777 as uint16_t,
            0 as uint16_t,
            uid,
            *gid.offset(0 as isize),
            0 as uint8_t,
        );
        memcpy(
            newpath as *mut ::core::ffi::c_void,
            path as *const ::core::ffi::c_void,
            pleng as size_t,
        );
        (*p).data.sdata.path = newpath;
        (*p).data.sdata.pleng = pleng;
        memset(
            &raw mut sr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<statsrecord>(),
        );
        sr.length = pleng as uint64_t;
        fsnodes_add_stats(wd, &raw mut sr);
        *inode = (*p).inode;
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if !attr.is_null() {
                fsnodes_fill_attr(
                    p,
                    wd,
                    uid,
                    *gid.offset(0 as isize),
                    auid,
                    agid,
                    sesflags,
                    attr,
                    1 as uint8_t,
                );
            }
            changelog(
                b"%u|SYMLINK(%u,%s,%s,%u,%u):%u\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                parent,
                changelog_escape_name(nleng as uint32_t, name),
                changelog_escape_name(pleng, newpath),
                uid,
                *gid.offset(0 as isize),
                (*p).inode,
            );
        } else {
            meta_version_inc();
        }
        stats_symlink = stats_symlink.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_symlink(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        return fs_univ_symlink(
            main_time(),
            rootinode,
            sesflags,
            parent,
            nleng,
            name,
            pleng,
            path,
            uid,
            gids,
            gid,
            auid,
            agid,
            inode,
            attr,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_symlink(
    mut ts: uint32_t,
    mut parent: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut path: *const uint8_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut inode: uint32_t,
) -> uint8_t {
    unsafe {
        let mut rinode: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = fs_univ_symlink(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            parent,
            nleng as uint16_t,
            name,
            strlen(path as *mut ::core::ffi::c_char) as uint32_t,
            path,
            uid,
            1 as uint32_t,
            &raw mut gid,
            0 as uint32_t,
            0 as uint32_t,
            &raw mut rinode,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if rinode != inode {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"SYMLINK data mismatch: my:%u != expected:%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rinode,
                inode,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_create(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut r#type: uint8_t,
    mut mode: uint16_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut rdev: uint32_t,
    mut copysgid: uint8_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
    mut oflags: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut omask: uint8_t = 0;
        let mut forcescid: uint8_t = 0;
        let mut chlogmode: uint16_t = 0;
        let mut forcetrashretention: uint16_t = 0;
        let mut forceseteattr: uint8_t = 0;
        let mut forceclreattr: uint8_t = 0;
        *inode = 0 as uint32_t;
        if !attr.is_null() {
            memset(
                attr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ATTR_RECORD_SIZE as size_t,
            );
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if r#type as ::core::ffi::c_int != TYPE_FILE
            && r#type as ::core::ffi::c_int != TYPE_SOCKET
            && r#type as ::core::ffi::c_int != TYPE_FIFO
            && r#type as ::core::ffi::c_int != TYPE_BLOCKDEV
            && r#type as ::core::ffi::c_int != TYPE_CHARDEV
            && r#type as ::core::ffi::c_int != TYPE_DIRECTORY
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (r#type as ::core::ffi::c_int == TYPE_BLOCKDEV
            || r#type as ::core::ffi::c_int == TYPE_CHARDEV)
            && uid != 0 as uint32_t
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut parent,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut wd,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*wd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*wd).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(
                wd,
                uid,
                gids,
                gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                sesflags,
            ) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if fsnodes_namecheck(nleng as uint32_t, name) < 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if fsnodes_nameisused(wd, nleng, name) != 0 {
            return MFS_ERROR_EEXIST as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_test_quota(
                wd,
                1 as uint32_t,
                0 as uint64_t,
                0 as uint64_t,
                0 as uint64_t,
            ) as ::core::ffi::c_int
                != 0
        {
            return MFS_ERROR_QUOTA as uint8_t;
        }
        p = fsnodes_create_node(
            ts,
            wd,
            nleng,
            name,
            r#type,
            mode,
            cumask,
            uid,
            *gid.offset(0 as isize),
            copysgid,
        );
        omask = patterns_find_matching(
            uid,
            gids,
            gid,
            nleng as uint8_t,
            name as *const uint8_t,
            &raw mut forcescid,
            &raw mut forcetrashretention,
            &raw mut forceseteattr,
            &raw mut forceclreattr,
        );
        if (*p).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
            || (*p).r#type() as ::core::ffi::c_int == TYPE_FILE
        {
            if omask as ::core::ffi::c_int & PATTERN_OMASK_SCLASS != 0
                && forcescid as ::core::ffi::c_int != (*p).sclassid as ::core::ffi::c_int
            {
                sclass_decref((*p).sclassid as uint16_t, (*p).r#type() as uint8_t);
                (*p).sclassid = forcescid;
                sclass_incref((*p).sclassid as uint16_t, (*p).r#type() as uint8_t);
            }
            if omask as ::core::ffi::c_int & PATTERN_OMASK_TRASHRETENTION != 0 {
                (*p).trashretention = forcetrashretention;
            }
        }
        if omask as ::core::ffi::c_int & PATTERN_OMASK_EATTR != 0 {
            if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                forceseteattr = (forceseteattr as ::core::ffi::c_int & !EATTR_NOECACHE) as uint8_t;
            }
            (*p).eattr =
                ((*p).eattr as ::core::ffi::c_int | forceseteattr as ::core::ffi::c_int) as uint8_t;
            (*p).eattr = ((*p).eattr as ::core::ffi::c_int & !(forceclreattr as ::core::ffi::c_int))
                as uint8_t;
        }
        if r#type as ::core::ffi::c_int == TYPE_BLOCKDEV
            || r#type as ::core::ffi::c_int == TYPE_CHARDEV
        {
            (*p).data.devdata.rdev = rdev;
        }
        *inode = (*p).inode;
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if !attr.is_null() {
                fsnodes_fill_attr(
                    p,
                    wd,
                    uid,
                    *gid.offset(0 as isize),
                    auid,
                    agid,
                    sesflags,
                    attr,
                    1 as uint8_t,
                );
            }
            if !oflags.is_null() {
                *oflags = 0 as uint8_t;
                if (*p).eattr as ::core::ffi::c_int & EATTR_NODATACACHE != 0 {
                    *oflags = (*oflags as ::core::ffi::c_int | OPEN_DIRECTMODE) as uint8_t;
                }
                if (*p).eattr as ::core::ffi::c_int & EATTR_APPENDONLY != 0 {
                    *oflags = (*oflags as ::core::ffi::c_int | OPEN_APPENDONLY) as uint8_t;
                }
            }
            chlogmode = mode;
            if copysgid as ::core::ffi::c_int != 0 && r#type as ::core::ffi::c_int == TYPE_DIRECTORY
            {
                chlogmode =
                    (chlogmode as ::core::ffi::c_int & 0xfbff as ::core::ffi::c_int) as uint16_t;
                chlogmode = (chlogmode as ::core::ffi::c_int
                    | (*p).mode() as ::core::ffi::c_int & 0x400 as ::core::ffi::c_int)
                    as uint16_t;
            }
            changelog(
                b"%u|CREATE(%u,%s,%hhu,%hu,%hu,%u,%u,%u):%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ts,
                parent,
                changelog_escape_name(nleng as uint32_t, name),
                r#type as ::core::ffi::c_int,
                chlogmode as ::core::ffi::c_int,
                cumask as ::core::ffi::c_int,
                (*p).uid,
                (*p).gid,
                rdev,
                (*p).inode,
            );
        } else {
            meta_version_inc();
        }
        if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
            stats_mkdir = stats_mkdir.wrapping_add(1);
        } else if r#type as ::core::ffi::c_int == TYPE_FILE {
            stats_create = stats_create.wrapping_add(1);
        } else {
            stats_mknod = stats_mknod.wrapping_add(1);
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mknod(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut r#type: uint8_t,
    mut mode: uint16_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut rdev: uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
    mut oflags: *mut uint8_t,
) -> uint8_t {
    unsafe {
        if r#type as ::core::ffi::c_int > 15 as ::core::ffi::c_int {
            r#type = fsnodes_type_convert(r#type);
        }
        return fs_univ_create(
            main_time(),
            rootinode,
            sesflags,
            parent,
            nleng,
            name,
            r#type,
            mode,
            cumask,
            uid,
            gids,
            gid,
            auid,
            agid,
            rdev,
            0 as uint8_t,
            inode,
            attr,
            oflags,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mkdir(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut mode: uint16_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut copysgid: uint8_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        return fs_univ_create(
            main_time(),
            rootinode,
            sesflags,
            parent,
            nleng,
            name,
            TYPE_DIRECTORY as uint8_t,
            mode,
            cumask,
            uid,
            gids,
            gid,
            auid,
            agid,
            0 as uint32_t,
            copysgid,
            inode,
            attr,
            ::core::ptr::null_mut::<uint8_t>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_create(
    mut ts: uint32_t,
    mut parent: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut r#type: uint8_t,
    mut mode: uint16_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut rdev: uint32_t,
    mut inode: uint32_t,
) -> uint8_t {
    unsafe {
        let mut rinode: uint32_t = 0;
        let mut status: uint8_t = 0;
        if r#type as ::core::ffi::c_int > 15 as ::core::ffi::c_int {
            r#type = fsnodes_type_convert(r#type);
        }
        status = fs_univ_create(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            parent,
            nleng as uint16_t,
            name,
            r#type,
            mode,
            cumask,
            uid,
            1 as uint32_t,
            &raw mut gid,
            0 as uint32_t,
            0 as uint32_t,
            rdev,
            0 as uint8_t,
            &raw mut rinode,
            ::core::ptr::null_mut::<uint8_t>(),
            ::core::ptr::null_mut::<uint8_t>(),
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if rinode != inode {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CREATE data mismatch: my:%u != expected:%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rinode,
                inode,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_unlink(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut dirmode: uint8_t,
    mut inode: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut parent,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut wd,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*wd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*wd).eattr as ::core::ffi::c_int & (EATTR_IMMUTABLE | EATTR_APPENDONLY) != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(
                wd,
                uid,
                gids,
                gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                sesflags,
            ) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if fsnodes_namecheck(nleng as uint32_t, name) < 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        e = fsnodes_lookup(wd, nleng, name);
        if e.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_sticky_access(wd, (*e).child as *mut fsnode, uid) == 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*(*e).child).eattr as ::core::ffi::c_int
                & (EATTR_UNDELETABLE | EATTR_IMMUTABLE | EATTR_APPENDONLY)
                != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if dirmode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if (*(*e).child).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                    return MFS_ERROR_EPERM as uint8_t;
                }
            } else {
                if (*(*e).child).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                    return MFS_ERROR_ENOTDIR as uint8_t;
                }
                if !(*(*e).child).data.ddata.children.is_null() {
                    return MFS_ERROR_ENOTEMPTY as uint8_t;
                }
            }
        } else if (*(*e).child).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            dirmode = 1 as uint8_t;
            if !(*(*e).child).data.ddata.children.is_null() {
                return MFS_ERROR_ENOTEMPTY as uint8_t;
            }
        } else {
            dirmode = 0 as uint8_t;
        }
        if !inode.is_null() {
            *inode = (*(*e).child).inode;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|UNLINK(%u,%s):%u\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                parent,
                changelog_escape_name(nleng as uint32_t, name),
                (*(*e).child).inode,
            );
        } else {
            meta_version_inc();
        }
        fsnodes_unlink(ts, e);
        if dirmode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            stats_unlink = stats_unlink.wrapping_add(1);
        } else {
            stats_rmdir = stats_rmdir.wrapping_add(1);
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_unlink(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_unlink(
            main_time(),
            rootinode,
            sesflags,
            parent,
            nleng,
            name,
            uid,
            gids,
            gid,
            0 as uint8_t,
            inode,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_rmdir(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent: uint32_t,
    mut nleng: uint16_t,
    mut name: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_unlink(
            main_time(),
            rootinode,
            sesflags,
            parent,
            nleng,
            name,
            uid,
            gids,
            gid,
            1 as uint8_t,
            inode,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_unlink(
    mut ts: uint32_t,
    mut parent: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut inode: uint32_t,
) -> uint8_t {
    unsafe {
        let mut rinode: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = fs_univ_unlink(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            parent,
            nleng as uint16_t,
            name,
            0 as uint32_t,
            0 as uint32_t,
            ::core::ptr::null_mut::<uint32_t>(),
            0 as uint8_t,
            &raw mut rinode,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if rinode != inode {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"UNLINK data mismatch: my:%u != expected:%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rinode,
                inode,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_move(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent_src: uint32_t,
    mut nleng_src: uint16_t,
    mut name_src: *const uint8_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint16_t,
    mut name_dst: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut rmode: uint8_t,
    mut delflags: uint8_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut swd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut se: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut dwd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut de: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut srcnode: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut dstnode: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ssr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut dsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut diff: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut omask: uint8_t = 0;
        let mut forcescid: uint8_t = 0;
        let mut forcetrashretention: uint16_t = 0;
        let mut forceseteattr: uint8_t = 0;
        let mut forceclreattr: uint8_t = 0;
        *inode = 0 as uint32_t;
        if !attr.is_null() {
            memset(
                attr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ATTR_RECORD_SIZE as size_t,
            );
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut parent_src,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut swd,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || fsnodes_node_find_ext(
                rootinode,
                sesflags,
                &raw mut parent_dst,
                ::core::ptr::null_mut::<*mut fsnode>(),
                &raw mut dwd,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*swd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*swd).eattr as ::core::ffi::c_int & (EATTR_IMMUTABLE | EATTR_APPENDONLY) != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(
                swd,
                uid,
                gids,
                gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                sesflags,
            ) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if fsnodes_namecheck(nleng_src as uint32_t, name_src) < 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        se = fsnodes_lookup(swd, nleng_src, name_src);
        if se.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        srcnode = (*se).child as *mut fsnode;
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_sticky_access(swd, srcnode, uid) == 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*dwd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*dwd).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(
                dwd,
                uid,
                gids,
                gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                sesflags,
            ) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*srcnode).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            if fsnodes_isancestor(srcnode, dwd) != 0 {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            if parent_src != parent_dst
                && sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                && fsnodes_access_ext(srcnode, uid, gids, gid, MODE_MASK_W as uint8_t, sesflags)
                    == 0
            {
                return MFS_ERROR_EACCES as uint8_t;
            }
        }
        if fsnodes_namecheck(nleng_dst as uint32_t, name_dst) < 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        de = fsnodes_lookup(dwd, nleng_dst, name_dst);
        if !de.is_null() {
            dstnode = (*de).child as *mut fsnode;
            if rmode as ::core::ffi::c_int == MFS_RENAME_NOREPLACE {
                return MFS_ERROR_EEXIST as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                && (*dwd).eattr as ::core::ffi::c_int & EATTR_APPENDONLY != 0
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                && fsnodes_sticky_access(dwd, dstnode, uid) == 0
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if rmode as ::core::ffi::c_int == MFS_RENAME_EXCHANGE {
                if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                    && (*dstnode).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
                {
                    return MFS_ERROR_EPERM as uint8_t;
                }
                if (*dstnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                    if fsnodes_isancestor(dstnode, swd) != 0 {
                        return MFS_ERROR_EINVAL as uint8_t;
                    }
                    if parent_src != parent_dst
                        && sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE
                            == 0 as ::core::ffi::c_int
                        && fsnodes_access_ext(
                            dstnode,
                            uid,
                            gids,
                            gid,
                            MODE_MASK_W as uint8_t,
                            sesflags,
                        ) == 0
                    {
                        return MFS_ERROR_EACCES as uint8_t;
                    }
                }
            } else {
                if delflags as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    if (*dstnode).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                        return MFS_ERROR_EPERM as uint8_t;
                    }
                }
                if delflags as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                    if (*dstnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                        return MFS_ERROR_EPERM as uint8_t;
                    }
                }
                if (*dstnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
                    && !(*dstnode).data.ddata.children.is_null()
                {
                    return MFS_ERROR_ENOTEMPTY as uint8_t;
                }
            }
        } else {
            dstnode = ::core::ptr::null_mut::<fsnode>();
            if rmode as ::core::ffi::c_int == MFS_RENAME_EXCHANGE {
                return MFS_ERROR_ENOENT as uint8_t;
            }
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if rmode as ::core::ffi::c_int == MFS_RENAME_EXCHANGE {
                fsnodes_get_stats(srcnode, &raw mut ssr, 2 as uint8_t);
                fsnodes_get_stats(dstnode, &raw mut dsr, 2 as uint8_t);
                if ssr.inodes > dsr.inodes {
                    diff.inodes = ssr.inodes.wrapping_sub(dsr.inodes);
                } else {
                    diff.inodes = 0 as uint32_t;
                }
                if ssr.length > dsr.length {
                    diff.length = ssr.length.wrapping_sub(dsr.length);
                } else {
                    diff.length = 0 as uint64_t;
                }
                if ssr.size > dsr.size {
                    diff.size = ssr.size.wrapping_sub(dsr.size);
                } else {
                    diff.size = 0 as uint64_t;
                }
                if ssr.realsize > dsr.realsize {
                    diff.realsize = ssr.realsize.wrapping_sub(dsr.realsize);
                } else {
                    diff.realsize = 0 as uint64_t;
                }
                if fsnodes_test_quota_for_uncommon_nodes(
                    dwd,
                    swd,
                    diff.inodes,
                    diff.length,
                    diff.size,
                    diff.realsize,
                ) != 0
                {
                    return MFS_ERROR_QUOTA as uint8_t;
                }
                if dsr.inodes > ssr.inodes {
                    diff.inodes = dsr.inodes.wrapping_sub(ssr.inodes);
                } else {
                    diff.inodes = 0 as uint32_t;
                }
                if dsr.length > ssr.length {
                    diff.length = dsr.length.wrapping_sub(ssr.length);
                } else {
                    diff.length = 0 as uint64_t;
                }
                if dsr.size > ssr.size {
                    diff.size = dsr.size.wrapping_sub(ssr.size);
                } else {
                    diff.size = 0 as uint64_t;
                }
                if dsr.realsize > ssr.realsize {
                    diff.realsize = dsr.realsize.wrapping_sub(ssr.realsize);
                } else {
                    diff.realsize = 0 as uint64_t;
                }
                if fsnodes_test_quota_for_uncommon_nodes(
                    swd,
                    dwd,
                    diff.inodes,
                    diff.length,
                    diff.size,
                    diff.realsize,
                ) != 0
                {
                    return MFS_ERROR_QUOTA as uint8_t;
                }
            } else {
                fsnodes_get_stats(srcnode, &raw mut ssr, 2 as uint8_t);
                if !dstnode.is_null() {
                    fsnodes_get_stats(dstnode, &raw mut dsr, 2 as uint8_t);
                    if ssr.inodes > dsr.inodes {
                        ssr.inodes = ssr.inodes.wrapping_sub(dsr.inodes);
                    } else {
                        ssr.inodes = 0 as uint32_t;
                    }
                    if ssr.length > dsr.length {
                        ssr.length = ssr.length.wrapping_sub(dsr.length);
                    } else {
                        ssr.length = 0 as uint64_t;
                    }
                    if ssr.size > dsr.size {
                        ssr.size = ssr.size.wrapping_sub(dsr.size);
                    } else {
                        ssr.size = 0 as uint64_t;
                    }
                    if ssr.realsize > dsr.realsize {
                        ssr.realsize = ssr.realsize.wrapping_sub(dsr.realsize);
                    } else {
                        ssr.realsize = 0 as uint64_t;
                    }
                }
                if fsnodes_test_quota_for_uncommon_nodes(
                    dwd,
                    swd,
                    ssr.inodes,
                    ssr.length,
                    ssr.size,
                    ssr.realsize,
                ) != 0
                {
                    return MFS_ERROR_QUOTA as uint8_t;
                }
            }
        }
        if rmode as ::core::ffi::c_int == MFS_RENAME_EXCHANGE {
            fsnodes_remove_edge(ts, se);
            fsnodes_remove_edge(ts, de);
            fsnodes_link(ts, dwd, srcnode, nleng_dst, name_dst);
            fsnodes_link(ts, swd, dstnode, nleng_src, name_src);
        } else {
            if !de.is_null() {
                fsnodes_unlink(ts, de);
            }
            fsnodes_remove_edge(ts, se);
            fsnodes_link(ts, dwd, srcnode, nleng_dst, name_dst);
            omask = patterns_find_matching(
                uid,
                gids,
                gid,
                nleng_dst as uint8_t,
                name_dst as *const uint8_t,
                &raw mut forcescid,
                &raw mut forcetrashretention,
                &raw mut forceseteattr,
                &raw mut forceclreattr,
            );
            if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY
                || (*srcnode).r#type() as ::core::ffi::c_int == TYPE_FILE
            {
                if omask as ::core::ffi::c_int & PATTERN_OMASK_SCLASS != 0
                    && forcescid as ::core::ffi::c_int != (*srcnode).sclassid as ::core::ffi::c_int
                {
                    if (*srcnode).r#type() as ::core::ffi::c_int == TYPE_FILE {
                        fsnodes_changefilesclassid(srcnode, forcescid);
                    } else {
                        sclass_decref(
                            (*srcnode).sclassid as uint16_t,
                            (*srcnode).r#type() as uint8_t,
                        );
                        (*srcnode).sclassid = forcescid;
                        sclass_incref(
                            (*srcnode).sclassid as uint16_t,
                            (*srcnode).r#type() as uint8_t,
                        );
                    }
                }
                if omask as ::core::ffi::c_int & PATTERN_OMASK_TRASHRETENTION != 0 {
                    (*srcnode).trashretention = forcetrashretention;
                }
            }
            if omask as ::core::ffi::c_int & PATTERN_OMASK_EATTR != 0 {
                if (*srcnode).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                    forceseteattr =
                        (forceseteattr as ::core::ffi::c_int & !EATTR_NOECACHE) as uint8_t;
                }
                (*srcnode).eattr = ((*srcnode).eattr as ::core::ffi::c_int
                    | forceseteattr as ::core::ffi::c_int)
                    as uint8_t;
                (*srcnode).eattr = ((*srcnode).eattr as ::core::ffi::c_int
                    & !(forceclreattr as ::core::ffi::c_int))
                    as uint8_t;
            }
            *inode = (*srcnode).inode;
            if !attr.is_null() {
                fsnodes_fill_attr(
                    srcnode,
                    dwd,
                    uid,
                    *gid.offset(0 as isize),
                    auid,
                    agid,
                    sesflags,
                    attr,
                    1 as uint8_t,
                );
            }
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|MOVE(%u,%s,%u,%s,%hhu):%u\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                parent_src,
                changelog_escape_name(nleng_src as uint32_t, name_src),
                parent_dst,
                changelog_escape_name(nleng_dst as uint32_t, name_dst),
                rmode as ::core::ffi::c_int,
                *inode,
            );
        } else {
            meta_version_inc();
        }
        stats_rename = stats_rename.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_rename(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut parent_src: uint32_t,
    mut nleng_src: uint16_t,
    mut name_src: *const uint8_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint16_t,
    mut name_dst: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut rmode: uint8_t,
    mut delflags: uint8_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        return fs_univ_move(
            main_time(),
            rootinode,
            sesflags,
            parent_src,
            nleng_src,
            name_src,
            parent_dst,
            nleng_dst,
            name_dst,
            uid,
            gids,
            gid,
            auid,
            agid,
            rmode,
            delflags,
            inode,
            attr,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_move(
    mut ts: uint32_t,
    mut parent_src: uint32_t,
    mut nleng_src: uint32_t,
    mut name_src: *const uint8_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint32_t,
    mut name_dst: *const uint8_t,
    mut rmode: uint8_t,
    mut inode: uint32_t,
) -> uint8_t {
    unsafe {
        let mut rinode: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = fs_univ_move(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            parent_src,
            nleng_src as uint16_t,
            name_src,
            parent_dst,
            nleng_dst as uint16_t,
            name_dst,
            0 as uint32_t,
            0 as uint32_t,
            ::core::ptr::null_mut::<uint32_t>(),
            0 as uint32_t,
            0 as uint32_t,
            rmode,
            0 as uint8_t,
            &raw mut rinode,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if rinode != inode {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MOVE data mismatch: my:%u != expected:%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rinode,
                inode,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_link(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode_src: uint32_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint16_t,
    mut name_dst: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut sp: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut dwd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut nlink: uint16_t = 0;
        *inode = 0 as uint32_t;
        if !attr.is_null() {
            memset(
                attr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ATTR_RECORD_SIZE as size_t,
            );
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode_src,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut sp,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || fsnodes_node_find_ext(
                rootinode,
                sesflags,
                &raw mut parent_dst,
                ::core::ptr::null_mut::<*mut fsnode>(),
                &raw mut dwd,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*sp).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        nlink = 0 as uint16_t;
        match (*sp).r#type() as ::core::ffi::c_int {
            TYPE_TRASH | TYPE_SUSTAINED => return MFS_ERROR_ENOENT as uint8_t,
            TYPE_DIRECTORY => return MFS_ERROR_EPERM as uint8_t,
            TYPE_FILE => {
                nlink = (*sp).data.fdata.nlink;
            }
            TYPE_SOCKET | TYPE_FIFO => {
                nlink = (*sp).data.odata.nlink;
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                nlink = (*sp).data.devdata.nlink;
            }
            TYPE_SYMLINK => {
                nlink = (*sp).data.sdata.nlink;
            }
            _ => {}
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && nlink as ::core::ffi::c_int >= MaxAllowedHardLinks as ::core::ffi::c_int
        {
            return MFS_ERROR_EMLINK as uint8_t;
        }
        if (*dwd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_ENOTDIR as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*dwd).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(
                dwd,
                uid,
                gids,
                gid,
                (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                sesflags,
            ) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if fsnodes_namecheck(nleng_dst as uint32_t, name_dst) < 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if fsnodes_nameisused(dwd, nleng_dst, name_dst) != 0 {
            return MFS_ERROR_EEXIST as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            fsnodes_get_stats(sp, &raw mut sr, 2 as uint8_t);
            if fsnodes_test_quota(dwd, sr.inodes, sr.length, sr.size, sr.realsize) != 0 {
                return MFS_ERROR_QUOTA as uint8_t;
            }
        }
        fsnodes_link(ts, dwd, sp, nleng_dst, name_dst);
        *inode = inode_src;
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if !attr.is_null() {
                fsnodes_fill_attr(
                    sp,
                    dwd,
                    uid,
                    *gid.offset(0 as isize),
                    auid,
                    agid,
                    sesflags,
                    attr,
                    1 as uint8_t,
                );
            }
            changelog(
                b"%u|LINK(%u,%u,%s)\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode_src,
                parent_dst,
                changelog_escape_name(nleng_dst as uint32_t, name_dst),
            );
        } else {
            meta_version_inc();
        }
        stats_link = stats_link.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_link(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode_src: uint32_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint16_t,
    mut name_dst: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        return fs_univ_link(
            main_time(),
            rootinode,
            sesflags,
            inode_src,
            parent_dst,
            nleng_dst,
            name_dst,
            uid,
            gids,
            gid,
            auid,
            agid,
            inode,
            attr,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_link(
    mut ts: uint32_t,
    mut inode_src: uint32_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint32_t,
    mut name_dst: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut rinode: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = fs_univ_link(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            inode_src,
            parent_dst,
            nleng_dst as uint16_t,
            name_dst,
            0 as uint32_t,
            0 as uint32_t,
            ::core::ptr::null_mut::<uint32_t>(),
            0 as uint32_t,
            0 as uint32_t,
            &raw mut rinode,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if rinode != inode_src {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"LINK data mismatch: my:%u != expected:%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rinode,
                inode_src,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_snapshot(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode_src: uint32_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint16_t,
    mut name_dst: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut smode: uint8_t,
    mut cumask: uint16_t,
    mut inodecheck: uint32_t,
    mut removed: uint32_t,
    mut same: uint32_t,
    mut existing: uint32_t,
    mut hardlinks: uint32_t,
    mut new: uint32_t,
) -> uint8_t {
    unsafe {
        let mut ssr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut common_inodes: uint32_t = 0;
        let mut common_length: uint64_t = 0;
        let mut common_size: uint64_t = 0;
        let mut common_realsize: uint64_t = 0;
        let mut args: fsnodes_snapshot_params = fsnodes_snapshot_params {
            ts: 0,
            smode: 0,
            sesflags: 0,
            cumask: 0,
            uid: 0,
            gids: 0,
            gid: ::core::ptr::null_mut::<uint32_t>(),
            inode_chksum: 0,
            removed_object: 0,
            same_file: 0,
            existing_object: 0,
            new_hardlink: 0,
            new_object: 0,
        };
        let mut sp: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut dwd: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut status: uint8_t = 0;
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        memset(
            &raw mut args as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<fsnodes_snapshot_params>(),
        );
        args.inode_chksum = 0 as uint32_t;
        args.ts = ts;
        args.smode = smode;
        args.sesflags = sesflags;
        args.uid = uid;
        args.gids = gids;
        args.gid = gid;
        args.cumask = cumask;
        if smode as ::core::ffi::c_int & SNAPSHOT_MODE_DELETE != 0 {
            if inode_src != 0 as uint32_t {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            if fsnodes_node_find_ext(
                rootinode,
                sesflags,
                &raw mut parent_dst,
                ::core::ptr::null_mut::<*mut fsnode>(),
                &raw mut dwd,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_ENOENT as uint8_t;
            }
            if (*dwd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                && fsnodes_access_ext(
                    dwd,
                    uid,
                    gids,
                    gid,
                    (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                    sesflags,
                ) == 0
            {
                return MFS_ERROR_EACCES as uint8_t;
            }
            if fsnodes_namecheck(nleng_dst as uint32_t, name_dst) < 0 as ::core::ffi::c_int {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            e = fsnodes_lookup(dwd, nleng_dst, name_dst);
            if e.is_null() {
                return MFS_ERROR_ENOENT as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                && fsnodes_sticky_access(dwd, (*e).child as *mut fsnode, uid) == 0
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            fsnodes_keep_alive_begin();
            if smode as ::core::ffi::c_int & SNAPSHOT_MODE_FORCE_REMOVAL == 0 as ::core::ffi::c_int
                && sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            {
                if (*dwd).eattr as ::core::ffi::c_int & (EATTR_APPENDONLY | EATTR_IMMUTABLE) != 0 {
                    return MFS_ERROR_EPERM as uint8_t;
                }
                status = fsnodes_remove_snapshot_test(e, &raw mut args);
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    return status;
                }
            }
            fsnodes_remove_snapshot(
                e,
                &raw mut args,
                (if (*dwd).eattr as ::core::ffi::c_int & (EATTR_APPENDONLY | EATTR_IMMUTABLE) != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as uint8_t,
            );
        } else {
            if inode_src == 0 as uint32_t {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            if fsnodes_node_find_ext(
                rootinode,
                sesflags,
                &raw mut inode_src,
                ::core::ptr::null_mut::<*mut fsnode>(),
                &raw mut sp,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                || fsnodes_node_find_ext(
                    rootinode,
                    sesflags,
                    &raw mut parent_dst,
                    ::core::ptr::null_mut::<*mut fsnode>(),
                    &raw mut dwd,
                    0 as uint8_t,
                ) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_ENOENT as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                && (*dwd).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                && fsnodes_access_ext(sp, uid, gids, gid, MODE_MASK_R as uint8_t, sesflags) == 0
            {
                return MFS_ERROR_EACCES as uint8_t;
            }
            if (*dwd).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if (*sp).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                if sp == dwd || fsnodes_isancestor(sp, dwd) != 0 {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
                && fsnodes_access_ext(
                    dwd,
                    uid,
                    gids,
                    gid,
                    (MODE_MASK_W | MODE_MASK_X) as uint8_t,
                    sesflags,
                ) == 0
            {
                return MFS_ERROR_EACCES as uint8_t;
            }
            if fsnodes_namecheck(nleng_dst as uint32_t, name_dst) < 0 as ::core::ffi::c_int {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            fsnodes_keep_alive_begin();
            status = fsnodes_snapshot_test(
                sp,
                sp,
                dwd,
                nleng_dst as uint32_t,
                name_dst,
                (smode as ::core::ffi::c_int & SNAPSHOT_MODE_CAN_OVERWRITE) as uint8_t,
            );
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                return status;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
                fsnodes_get_stats(sp, &raw mut ssr, 2 as uint8_t);
                common_inodes = 0 as uint32_t;
                common_length = 0 as uint64_t;
                common_size = 0 as uint64_t;
                common_realsize = 0 as uint64_t;
                if fsnodes_snapshot_recursive_test_quota(
                    sp,
                    dwd,
                    nleng_dst as uint32_t,
                    name_dst,
                    &raw mut common_inodes,
                    &raw mut common_length,
                    &raw mut common_size,
                    &raw mut common_realsize,
                ) != 0
                {
                    return MFS_ERROR_QUOTA as uint8_t;
                }
                if ssr.inodes > common_inodes {
                    ssr.inodes = ssr.inodes.wrapping_sub(common_inodes);
                } else {
                    ssr.inodes = 0 as uint32_t;
                }
                if ssr.length > common_length {
                    ssr.length = ssr.length.wrapping_sub(common_length);
                } else {
                    ssr.length = 0 as uint64_t;
                }
                if ssr.size > common_size {
                    ssr.size = ssr.size.wrapping_sub(common_size);
                } else {
                    ssr.size = 0 as uint64_t;
                }
                if ssr.realsize > common_realsize {
                    ssr.realsize = ssr.realsize.wrapping_sub(common_realsize);
                } else {
                    ssr.realsize = 0 as uint64_t;
                }
                if fsnodes_test_quota(dwd, ssr.inodes, ssr.length, ssr.size, ssr.realsize) != 0 {
                    return MFS_ERROR_QUOTA as uint8_t;
                }
            }
            fsnodes_snapshot(
                sp,
                dwd,
                nleng_dst as uint32_t,
                name_dst,
                0 as uint8_t,
                &raw mut args,
            );
            if smode as ::core::ffi::c_int & SNAPSHOT_MODE_PRESERVE_HARDLINKS != 0 {
                chash_erase(snapshot_inodehash);
            }
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|SNAPSHOT(%u,%u,%s,%hhu,%hhu,%u,%s,%hu):%u,%u,%u,%u,%u,%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ts,
                inode_src,
                parent_dst,
                changelog_escape_name(nleng_dst as uint32_t, name_dst),
                smode as ::core::ffi::c_int,
                sesflags as ::core::ffi::c_int,
                uid,
                changelog_generate_gids(gids, gid),
                cumask as ::core::ffi::c_int,
                args.inode_chksum,
                args.removed_object,
                args.same_file,
                args.existing_object,
                args.new_hardlink,
                args.new_object,
            );
        } else {
            if inodecheck | removed | same | existing | hardlinks | new != 0 as uint32_t
                && (inodecheck != args.inode_chksum
                    || removed != args.removed_object
                    || same != args.same_file
                    || existing != args.existing_object
                    || hardlinks != args.new_hardlink
                    || new != args.new_object)
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"SNAPSHOT data mismatch: my:(%u,%u,%u,%u,%u,%u) != expected:(%u,%u,%u,%u,%u,%u)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    args.inode_chksum,
                    args.removed_object,
                    args.same_file,
                    args.existing_object,
                    args.new_hardlink,
                    args.new_object,
                    inodecheck,
                    removed,
                    same,
                    existing,
                    hardlinks,
                    new,
                );
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            meta_version_inc();
        }
        stats_snapshot = stats_snapshot.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_snapshot(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode_src: uint32_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint16_t,
    mut name_dst: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut smode: uint8_t,
    mut cumask: uint16_t,
) -> uint8_t {
    unsafe {
        return fs_univ_snapshot(
            main_time(),
            rootinode,
            sesflags,
            inode_src,
            parent_dst,
            nleng_dst,
            name_dst,
            uid,
            gids,
            gid,
            smode,
            cumask,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_snapshot(
    mut ts: uint32_t,
    mut inode_src: uint32_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint16_t,
    mut name_dst: *mut uint8_t,
    mut smode: uint8_t,
    mut sesflags: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut cumask: uint16_t,
    mut inodecheck: uint32_t,
    mut removed: uint32_t,
    mut same: uint32_t,
    mut existing: uint32_t,
    mut hardlinks: uint32_t,
    mut new: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_snapshot(
            ts,
            0 as uint32_t,
            (sesflags as ::core::ffi::c_int | SESFLAG_METARESTORE) as uint8_t,
            inode_src,
            parent_dst,
            nleng_dst,
            name_dst,
            uid,
            gids,
            gid,
            smode,
            cumask,
            inodecheck,
            removed,
            same,
            existing,
            hardlinks,
            new,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_append_slice(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut flags: uint8_t,
    mut inode: uint32_t,
    mut inode_src: uint32_t,
    mut slice_from: uint32_t,
    mut slice_to: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut fleng: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut dstchunks: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut lastsrcchunk: uint32_t = 0;
        let mut lastsrcchunksize: uint32_t = 0;
        let mut lastdstchunk: uint32_t = 0;
        let mut lastdstchunksize: uint32_t = 0;
        let mut addlength: uint64_t = 0;
        let mut newlength: uint64_t = 0;
        let mut lengdiff: uint64_t = 0;
        let mut sizediff: uint64_t = 0;
        let mut status: uint8_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut sp: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if inode == inode_src {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode_src,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut sp,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || fsnodes_node_find_ext(
                rootinode,
                sesflags,
                &raw mut inode,
                ::core::ptr::null_mut::<*mut fsnode>(),
                &raw mut p,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*sp).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*sp).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*sp).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(sp, uid, gids, gid, MODE_MASK_R as uint8_t, sesflags) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_W as uint8_t, sesflags) == 0
        {
            return MFS_ERROR_EACCES as uint8_t;
        }
        if (*sp).data.fdata.length == 0 as uint64_t {
            if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE != 0 as ::core::ffi::c_int {
                meta_version_inc();
            } else if !fleng.is_null() {
                *fleng = (*p).data.fdata.length;
            }
            return MFS_STATUS_OK as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            lastsrcchunk =
                ((*sp).data.fdata.length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS) as uint32_t;
            if flags as ::core::ffi::c_int & APPEND_SLICE_FROM_NEG != 0 {
                if slice_from > lastsrcchunk.wrapping_add(1 as uint32_t) {
                    slice_from = 0 as uint32_t;
                } else {
                    slice_from = lastsrcchunk
                        .wrapping_add(1 as uint32_t)
                        .wrapping_sub(slice_from);
                }
            }
            if flags as ::core::ffi::c_int & APPEND_SLICE_TO_NEG != 0 {
                if slice_to > lastsrcchunk.wrapping_add(1 as uint32_t) {
                    if !fleng.is_null() {
                        *fleng = (*p).data.fdata.length;
                    }
                    return MFS_STATUS_OK as uint8_t;
                } else {
                    slice_to = lastsrcchunk
                        .wrapping_add(1 as uint32_t)
                        .wrapping_sub(slice_to);
                }
            }
            if slice_to <= slice_from {
                if !fleng.is_null() {
                    *fleng = (*p).data.fdata.length;
                }
                return MFS_STATUS_OK as uint8_t;
            }
            slice_to = slice_to.wrapping_sub(1);
            if slice_to >= lastsrcchunk {
                slice_to = lastsrcchunk;
                if slice_to < slice_from {
                    if !fleng.is_null() {
                        *fleng = (*p).data.fdata.length;
                    }
                    return MFS_STATUS_OK as uint8_t;
                }
                addlength = (*sp)
                    .data
                    .fdata
                    .length
                    .wrapping_sub((slice_from as uint64_t) << MFSCHUNKBITS);
                lastsrcchunksize = (((*sp).data.fdata.length.wrapping_sub(1 as uint64_t)
                    & MFSCHUNKMASK as uint64_t)
                    .wrapping_add(MFSBLOCKSIZE as uint64_t)
                    & MFSBLOCKNEGMASK as uint64_t)
                    .wrapping_add(MFSHDRSIZE as uint64_t)
                    as uint32_t;
            } else {
                if slice_to < slice_from {
                    if !fleng.is_null() {
                        *fleng = (*p).data.fdata.length;
                    }
                    return MFS_STATUS_OK as uint8_t;
                }
                addlength = ((1 as uint32_t)
                    .wrapping_add(slice_to)
                    .wrapping_sub(slice_from) as uint64_t)
                    << MFSCHUNKBITS;
                lastsrcchunksize = (MFSCHUNKSIZE + MFSHDRSIZE) as uint32_t;
            }
            if (*p).data.fdata.length > 0 as uint64_t {
                lastdstchunk = ((*p).data.fdata.length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS)
                    as uint32_t;
                dstchunks = lastdstchunk.wrapping_add(1 as uint32_t);
                lastdstchunksize = (((*p).data.fdata.length.wrapping_sub(1 as uint64_t)
                    & MFSCHUNKMASK as uint64_t)
                    .wrapping_add(MFSBLOCKSIZE as uint64_t)
                    & MFSBLOCKNEGMASK as uint64_t)
                    .wrapping_add(MFSHDRSIZE as uint64_t)
                    as uint32_t;
            } else {
                lastdstchunk = 0xffffffff as ::core::ffi::c_uint as uint32_t;
                dstchunks = 0 as uint32_t;
                lastdstchunksize = MFSHDRSIZE as uint32_t;
            }
            newlength = ((dstchunks as uint64_t) << MFSCHUNKBITS).wrapping_add(addlength);
            if newlength < (*p).data.fdata.length {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            lengdiff = newlength.wrapping_sub((*p).data.fdata.length);
            sizediff = 0 as uint64_t;
            i = slice_from;
            while i <= slice_to {
                if i < (*sp).data.fdata.chunks
                    && *(*sp).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t
                {
                    if i < lastsrcchunk {
                        sizediff = sizediff.wrapping_add((MFSCHUNKSIZE + MFSHDRSIZE) as uint64_t);
                    } else if i == lastsrcchunk {
                        sizediff = sizediff.wrapping_add(lastsrcchunksize as uint64_t);
                    }
                }
                i = i.wrapping_add(1);
            }
            i = 0 as uint32_t;
            while i < (*p).data.fdata.chunks {
                if *(*p).data.fdata.chunktab.offset(i as isize) > 0 as uint64_t {
                    if i > lastdstchunk || dstchunks == 0 as uint32_t {
                        sizediff = sizediff.wrapping_sub((MFSCHUNKSIZE + MFSHDRSIZE) as uint64_t);
                    } else if i == lastdstchunk {
                        sizediff = sizediff.wrapping_add(
                            ((MFSCHUNKSIZE + MFSHDRSIZE) as uint32_t).wrapping_sub(lastdstchunksize)
                                as uint64_t,
                        );
                    }
                }
                i = i.wrapping_add(1);
            }
            if fsnodes_test_quota(
                p,
                0 as uint32_t,
                lengdiff,
                sizediff,
                (sclass_get_keeparch_maxstorage_eights((*p).sclassid as uint16_t) as uint64_t)
                    .wrapping_mul(sizediff)
                    .wrapping_div(8 as uint64_t),
            ) != 0
            {
                return MFS_ERROR_QUOTA as uint8_t;
            }
        }
        status = fsnodes_append_slice_of_chunks(ts, p, sp, slice_from, slice_to);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|APPEND(%u,%u,%u,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode,
                inode_src,
                slice_from,
                slice_to,
            );
            if !fleng.is_null() {
                *fleng = (*p).data.fdata.length;
            }
        } else {
            meta_version_inc();
        }
        stats_snapshot = stats_snapshot.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_append_slice(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut flags: uint8_t,
    mut inode: uint32_t,
    mut inode_src: uint32_t,
    mut slice_from: uint32_t,
    mut slice_to: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut fleng: *mut uint64_t,
) -> uint8_t {
    unsafe {
        return fs_univ_append_slice(
            main_time(),
            rootinode,
            sesflags,
            flags,
            inode,
            inode_src,
            slice_from,
            slice_to,
            uid,
            gids,
            gid,
            fleng,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_append_slice(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut inode_src: uint32_t,
    mut slice_from: uint32_t,
    mut slice_to: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_append_slice(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            0 as uint8_t,
            inode,
            inode_src,
            slice_from,
            slice_to,
            0 as uint32_t,
            0 as uint32_t,
            ::core::ptr::null_mut::<uint32_t>(),
            ::core::ptr::null_mut::<uint64_t>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readdirfull(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut maxentries: uint32_t,
    mut nedgeidp: *mut uint64_t,
    mut dedge: *mut *mut ::core::ffi::c_void,
    mut dbuff: *mut uint8_t,
    mut dbuffsize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut nedgeid: uint64_t = 0;
        let mut r: uint64_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        if dbuff.is_null() {
            nedgeid = *nedgeidp;
            if nedgeid == 0 as uint64_t || nedgeid == 1 as uint64_t {
                e = ::core::ptr::null_mut::<fsedge>();
            } else {
                e = fsnodes_edgeid_find(nedgeid);
            }
            if e.is_null() {
                if fsnodes_node_find_ext(
                    rootinode,
                    sesflags,
                    &raw mut inode,
                    ::core::ptr::null_mut::<*mut fsnode>(),
                    &raw mut p,
                    0 as uint8_t,
                ) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    return MFS_ERROR_ENOENT as uint8_t;
                }
                if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                    return MFS_ERROR_ENOTDIR as uint8_t;
                }
                e = (*p).data.ddata.children;
                while !e.is_null() && (*e).edgeid < nedgeid {
                    e = (*e).nextchild as *mut fsedge;
                }
            }
            *dedge = e as *mut ::core::ffi::c_void;
        } else {
            e = *dedge as *mut fsedge;
        }
        r = fsnodes_readdirfull(e, flags, maxentries, dbuff);
        if dbuff.is_null() {
            if r > 0xffff0000 as uint64_t {
                return MFS_ERROR_ERANGE as uint8_t;
            }
            *dbuffsize = r as uint32_t;
        } else {
            *nedgeidp = r;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readdir_size(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut flags: uint8_t,
    mut maxentries: uint32_t,
    mut nedgeid: uint64_t,
    mut dnode: *mut *mut ::core::ffi::c_void,
    mut dedge: *mut *mut ::core::ffi::c_void,
    mut dbuffsize: *mut uint32_t,
    mut attrmode: uint8_t,
) -> uint8_t {
    unsafe {
        let mut r: uint64_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        *dnode = NULL;
        *dbuffsize = 0 as uint32_t;
        if nedgeid == 0 as uint64_t || nedgeid == 1 as uint64_t {
            e = ::core::ptr::null_mut::<fsedge>();
        } else {
            e = fsnodes_edgeid_find(nedgeid);
        }
        if e.is_null() {
            if fsnodes_node_find_ext(
                rootinode,
                sesflags,
                &raw mut inode,
                ::core::ptr::null_mut::<*mut fsnode>(),
                &raw mut p,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_ENOENT as uint8_t;
            }
            if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                return MFS_ERROR_ENOTDIR as uint8_t;
            }
            if nedgeid == 0 as uint64_t {
                if flags as ::core::ffi::c_int & GETDIR_FLAG_WITHATTR != 0 {
                    if fsnodes_access_ext(
                        p,
                        uid,
                        gids,
                        gid,
                        (MODE_MASK_R | MODE_MASK_X) as uint8_t,
                        sesflags,
                    ) == 0
                    {
                        return MFS_ERROR_EACCES as uint8_t;
                    }
                } else if fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_R as uint8_t, sesflags)
                    == 0
                {
                    return MFS_ERROR_EACCES as uint8_t;
                }
            } else {
                e = (*p).data.ddata.children;
                while !e.is_null() && (*e).edgeid < nedgeid {
                    e = (*e).nextchild as *mut fsedge;
                }
            }
        } else {
            p = (*e).parent as *mut fsnode;
        }
        *dnode = p as *mut ::core::ffi::c_void;
        *dedge = e as *mut ::core::ffi::c_void;
        r = fsnodes_readdirsize(
            p,
            e,
            maxentries,
            nedgeid,
            (if flags as ::core::ffi::c_int & GETDIR_FLAG_WITHATTR != 0 {
                attrmode as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t,
        );
        if r > 0xffff0000 as uint64_t {
            return MFS_ERROR_ERANGE as uint8_t;
        }
        *dbuffsize = r as uint32_t;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readdir_data(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut flags: uint8_t,
    mut maxentries: uint32_t,
    mut nedgeid: *mut uint64_t,
    mut dnode: *mut ::core::ffi::c_void,
    mut dedge: *mut ::core::ffi::c_void,
    mut dbuff: *mut uint8_t,
    mut attrmode: uint8_t,
) {
    unsafe {
        let mut p: *mut fsnode = dnode as *mut fsnode;
        let mut e: *mut fsedge = dedge as *mut fsedge;
        let mut ts: uint32_t = main_time();
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY == 0 as ::core::ffi::c_int
            && (*p).atime != ts
        {
            if AtimeMode as ::core::ffi::c_int == ATIME_ALWAYS
                || ((*p).atime <= (*p).ctime && ts >= (*p).ctime
                    || (*p).atime <= (*p).mtime && ts >= (*p).mtime
                    || (*p).atime.wrapping_add(86400 as uint32_t) < ts)
                    && AtimeMode as ::core::ffi::c_int == ATIME_RELATIVE_ONLY
            {
                (*p).atime = ts;
                fsnodes_checkarchmode(p, ts, CHECK_ATIME as uint8_t);
                changelog(
                    b"%u|ACCESS(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    (*p).inode,
                );
            }
        }
        fsnodes_readdirdata(
            rootinode,
            uid,
            gid,
            auid,
            agid,
            sesflags,
            p,
            e,
            maxentries,
            nedgeid,
            dbuff,
            (if flags as ::core::ffi::c_int & GETDIR_FLAG_WITHATTR != 0 {
                attrmode as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t,
        );
        stats_readdir = stats_readdir.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_filechunk(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut chunkid: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if indx > MAX_INDEX as uint32_t {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        if indx < (*p).data.fdata.chunks {
            *chunkid = *(*p).data.fdata.chunktab.offset(indx as isize);
        } else {
            return MFS_ERROR_NOCHUNK as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_checkfile(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut mode: uint8_t,
    mut chunkcount: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        fsnodes_checkfile(p, mode, chunkcount);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_opencheck(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut flags: uint8_t,
    mut attr: *mut uint8_t,
    mut oflags: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
            && flags as ::core::ffi::c_int & (OPEN_WRITE | OPEN_TRUNCATE) != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0
            && flags as ::core::ffi::c_int & (OPEN_WRITE | OPEN_TRUNCATE) != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_APPENDONLY != 0
            && flags as ::core::ffi::c_int & OPEN_TRUNCATE != 0
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if flags as ::core::ffi::c_int & OPEN_AFTER_CREATE == 0 as ::core::ffi::c_int {
            let mut modemask: uint8_t = 0 as uint8_t;
            if flags as ::core::ffi::c_int & OPEN_READ != 0 {
                modemask = (modemask as ::core::ffi::c_int | MODE_MASK_R) as uint8_t;
            }
            if flags as ::core::ffi::c_int & OPEN_WRITE != 0 {
                modemask = (modemask as ::core::ffi::c_int | MODE_MASK_W) as uint8_t;
            }
            if fsnodes_access_ext(p, uid, gids, gid, modemask, sesflags) == 0 {
                return MFS_ERROR_EACCES as uint8_t;
            }
            if flags as ::core::ffi::c_int & OPEN_TRUNCATE != 0 {
                let mut ts: uint32_t = main_time();
                if fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_W as uint8_t, sesflags) == 0 {
                    return MFS_ERROR_EACCES as uint8_t;
                }
                fsnodes_setlength(p, 0 as uint64_t);
                appendres_clear(inode);
                changelog(
                    b"%u|LENGTH(%u,0,1)\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    inode,
                );
                (*p).mtime = ts;
                (*p).ctime = (*p).mtime;
                fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME) as uint8_t);
            }
        }
        fsnodes_fill_attr(
            p,
            ::core::ptr::null_mut::<fsnode>(),
            uid,
            *gid.offset(0 as isize),
            auid,
            agid,
            sesflags,
            attr,
            1 as uint8_t,
        );
        *oflags = 0 as uint8_t;
        if (*p).eattr as ::core::ffi::c_int & EATTR_NODATACACHE != 0 {
            *oflags = (*oflags as ::core::ffi::c_int | OPEN_DIRECTMODE) as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_APPENDONLY != 0 {
            *oflags = (*oflags as ::core::ffi::c_int | OPEN_APPENDONLY) as uint8_t;
        }
        stats_open = stats_open.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readchunk(
    mut inode: uint32_t,
    mut sesflags: uint32_t,
    mut indx: uint32_t,
    mut chunkopflags: uint8_t,
    mut allow_recover: uint8_t,
    mut chunkid: *mut uint64_t,
    mut length: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ts: uint32_t = main_time();
        *chunkid = 0 as uint64_t;
        *length = 0 as uint64_t;
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if indx > MAX_INDEX as uint32_t {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        if indx < (*p).data.fdata.chunks {
            *chunkid = *(*p).data.fdata.chunktab.offset(indx as isize);
        }
        if *chunkid > 0 as uint64_t {
            status = chunk_read_check(ts, *chunkid, allow_recover);
            if status != MFS_STATUS_OK {
                return status as uint8_t;
            }
        }
        *length = (*p).data.fdata.length;
        if (sesflags != SESFLAG_READONLY as uint32_t) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && (*p).atime != ts
            && chunkopflags as ::core::ffi::c_int & CHUNKOPFLAG_CANMODTIME != 0
        {
            if AtimeMode as ::core::ffi::c_int == ATIME_ALWAYS
                || AtimeMode as ::core::ffi::c_int == ATIME_FILES_ONLY
                || ((*p).atime <= (*p).ctime && ts >= (*p).ctime
                    || (*p).atime <= (*p).mtime && ts >= (*p).mtime
                    || (*p).atime.wrapping_add(86400 as uint32_t) < ts)
                    && (AtimeMode as ::core::ffi::c_int == ATIME_RELATIVE_ONLY
                        || AtimeMode as ::core::ffi::c_int == ATIME_FILES_AND_RELATIVE_ONLY)
            {
                (*p).atime = ts;
                fsnodes_checkarchmode(p, ts, CHECK_ATIME as uint8_t);
                changelog(
                    b"%u|ACCESS(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    inode,
                );
            }
        }
        stats_readchunk = stats_readchunk.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_writechunk(
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut chunkopflags: uint8_t,
    mut prevchunkid: *mut uint64_t,
    mut chunkid: *mut uint64_t,
    mut length: *mut uint64_t,
    mut opflag: *mut uint8_t,
    mut clientip: uint32_t,
) -> uint8_t {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut i: uint32_t = 0;
        let mut ochunkid: uint64_t = 0;
        let mut nchunkid: uint64_t = 0;
        let mut nlength: uint64_t = 0;
        let mut lengdiff: uint64_t = 0;
        let mut psr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut nsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ts: uint32_t = main_time();
        let mut lastchunk: uint32_t = 0;
        let mut lastchunksize: uint32_t = 0;
        let mut sizediff: uint32_t = 0;
        if matocsserv_have_availspace() == 0 as ::core::ffi::c_int
            && chunkopflags as ::core::ffi::c_int & CHUNKOPFLAG_CANUSERESERVESPACE
                == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_NOSPACE as uint8_t;
        }
        *chunkid = 0 as uint64_t;
        *length = 0 as uint64_t;
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if indx > MAX_INDEX as uint32_t {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        if (*p).data.fdata.length > 0 as uint64_t {
            lastchunk =
                ((*p).data.fdata.length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS) as uint32_t;
            lastchunksize = (((*p).data.fdata.length.wrapping_sub(1 as uint64_t)
                & MFSCHUNKMASK as uint64_t)
                .wrapping_add(MFSBLOCKSIZE as uint64_t)
                & MFSBLOCKNEGMASK as uint64_t)
                .wrapping_add(MFSHDRSIZE as uint64_t) as uint32_t;
        } else {
            lastchunk = 0 as uint32_t;
            lastchunksize = MFSHDRSIZE as uint32_t;
        }
        if indx < lastchunk {
            nlength = (*p).data.fdata.length;
            if indx >= (*p).data.fdata.chunks
                || *(*p).data.fdata.chunktab.offset(indx as isize) == 0 as uint64_t
            {
                sizediff = (MFSCHUNKSIZE + MFSHDRSIZE) as uint32_t;
            } else {
                sizediff = 0 as uint32_t;
            }
        } else if indx == lastchunk {
            nlength = (*p).data.fdata.length;
            if indx >= (*p).data.fdata.chunks
                || *(*p).data.fdata.chunktab.offset(indx as isize) == 0 as uint64_t
            {
                sizediff = (MFSCHUNKSIZE + MFSHDRSIZE) as uint32_t;
            } else {
                sizediff = ((MFSCHUNKSIZE + MFSHDRSIZE) as uint32_t).wrapping_sub(lastchunksize);
            }
        } else {
            nlength = indx as uint64_t;
            nlength <<= MFSCHUNKBITS;
            sizediff = (MFSCHUNKSIZE + MFSHDRSIZE) as uint32_t;
        }
        lengdiff = nlength.wrapping_sub((*p).data.fdata.length);
        if fsnodes_test_quota(
            p,
            0 as uint32_t,
            lengdiff,
            sizediff as uint64_t,
            (sclass_get_keeparch_maxstorage_eights((*p).sclassid as uint16_t) as uint32_t)
                .wrapping_mul(sizediff)
                .wrapping_div(8 as uint32_t) as uint64_t,
        ) != 0
        {
            return MFS_ERROR_QUOTA as uint8_t;
        }
        fsnodes_get_stats(p, &raw mut psr, 0 as uint8_t);
        if indx >= (*p).data.fdata.chunks {
            let mut newchunks: uint32_t = indx.wrapping_add(1 as uint32_t);
            if (*p).data.fdata.chunktab.is_null() {
                (*p).data.fdata.chunktab = chunktab_malloc(newchunks);
            } else {
                (*p).data.fdata.chunktab =
                    chunktab_realloc((*p).data.fdata.chunktab, (*p).data.fdata.chunks, newchunks);
            }
            if (*p).data.fdata.chunktab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*p).data.fdata.chunktab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            i = (*p).data.fdata.chunks;
            while i < newchunks {
                *(*p).data.fdata.chunktab.offset(i as isize) = 0 as uint64_t;
                i = i.wrapping_add(1);
            }
            (*p).data.fdata.chunks = newchunks;
        }
        ochunkid = *(*p).data.fdata.chunktab.offset(indx as isize);
        *prevchunkid = ochunkid;
        status = chunk_multi_modify(
            (if chunkopflags as ::core::ffi::c_int & CHUNKOPFLAG_CONTINUEOP != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t,
            &raw mut nchunkid,
            ochunkid,
            (*p).sclassid,
            opflag,
            clientip,
        );
        if status != MFS_STATUS_OK {
            return status as uint8_t;
        }
        *(*p).data.fdata.chunktab.offset(indx as isize) = nchunkid;
        if nlength > (*p).data.fdata.length {
            if (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH {
                trashspace = trashspace.wrapping_sub((*p).data.fdata.length);
                trashspace = trashspace.wrapping_add(nlength);
            } else if (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
                sustainedspace = sustainedspace.wrapping_sub((*p).data.fdata.length);
                sustainedspace = sustainedspace.wrapping_add(nlength);
            }
            (*p).data.fdata.length = nlength;
        }
        fsnodes_get_stats(p, &raw mut nsr, 1 as uint8_t);
        e = (*p).parents;
        while !e.is_null() {
            fsnodes_add_sub_stats((*e).parent as *mut fsnode, &raw mut nsr, &raw mut psr);
            e = (*e).nextparent as *mut fsedge;
        }
        (*p).eattr = ((*p).eattr as ::core::ffi::c_int & !EATTR_SNAPSHOT) as uint8_t;
        *chunkid = nchunkid;
        *length = (*p).data.fdata.length;
        changelog(
            b"%u|WRITE(%u,%u,%hhu,%u):%lu\0".as_ptr() as *const ::core::ffi::c_char,
            ts,
            inode,
            indx,
            *opflag as ::core::ffi::c_int,
            if chunkopflags as ::core::ffi::c_int & CHUNKOPFLAG_CANMODTIME != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
            nchunkid,
        );
        if chunkopflags as ::core::ffi::c_int & CHUNKOPFLAG_CANMODTIME != 0 {
            (*p).ctime = ts;
            (*p).mtime = (*p).ctime;
            fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME) as uint8_t);
        }
        stats_writechunk = stats_writechunk.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_write(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut opflag: uint8_t,
    mut canmodmtime: uint8_t,
    mut nchunkid: uint64_t,
) -> uint8_t {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut i: uint32_t = 0;
        let mut ochunkid: uint64_t = 0;
        let mut nlength: uint64_t = 0;
        let mut psr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut nsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if indx > MAX_INDEX as uint32_t {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        fsnodes_get_stats(p, &raw mut psr, 0 as uint8_t);
        if indx >= (*p).data.fdata.chunks {
            let mut newchunks: uint32_t = indx.wrapping_add(1 as uint32_t);
            if (*p).data.fdata.chunktab.is_null() {
                (*p).data.fdata.chunktab = chunktab_malloc(newchunks);
            } else {
                (*p).data.fdata.chunktab =
                    chunktab_realloc((*p).data.fdata.chunktab, (*p).data.fdata.chunks, newchunks);
            }
            if (*p).data.fdata.chunktab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7526 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7526 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*p).data.fdata.chunktab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7526 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7526 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            i = (*p).data.fdata.chunks;
            while i < newchunks {
                *(*p).data.fdata.chunktab.offset(i as isize) = 0 as uint64_t;
                i = i.wrapping_add(1);
            }
            (*p).data.fdata.chunks = newchunks;
        }
        nlength = indx as uint64_t;
        nlength <<= MFSCHUNKBITS;
        if nlength < (*p).data.fdata.length {
            nlength = (*p).data.fdata.length;
        }
        ochunkid = *(*p).data.fdata.chunktab.offset(indx as isize);
        status = chunk_mr_multi_modify(ts, &raw mut nchunkid, ochunkid, (*p).sclassid, opflag);
        if status != MFS_STATUS_OK {
            return status as uint8_t;
        }
        *(*p).data.fdata.chunktab.offset(indx as isize) = nchunkid;
        if nlength > (*p).data.fdata.length {
            if (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH {
                trashspace = trashspace.wrapping_sub((*p).data.fdata.length);
                trashspace = trashspace.wrapping_add(nlength);
            } else if (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
                sustainedspace = sustainedspace.wrapping_sub((*p).data.fdata.length);
                sustainedspace = sustainedspace.wrapping_add(nlength);
            }
            (*p).data.fdata.length = nlength;
        }
        fsnodes_get_stats(p, &raw mut nsr, 1 as uint8_t);
        e = (*p).parents;
        while !e.is_null() {
            fsnodes_add_sub_stats((*e).parent as *mut fsnode, &raw mut nsr, &raw mut psr);
            e = (*e).nextparent as *mut fsedge;
        }
        (*p).eattr = ((*p).eattr as ::core::ffi::c_int & !EATTR_SNAPSHOT) as uint8_t;
        if canmodmtime != 0 {
            (*p).ctime = ts;
            (*p).mtime = (*p).ctime;
            fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME) as uint8_t);
        }
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fs_univ_rollback(
    mut mr: uint8_t,
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut prevchunkid: uint64_t,
    mut chunkid: uint64_t,
) -> uint8_t {
    unsafe {
        let mut psr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut nsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if indx > MAX_INDEX as uint32_t {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        if indx >= (*p).data.fdata.chunks {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if prevchunkid != chunkid {
            fsnodes_get_stats(p, &raw mut psr, 0 as uint8_t);
            if prevchunkid > 0 as uint64_t {
                chunk_add_file(prevchunkid, (*p).sclassid);
            }
            chunk_delete_file(chunkid, (*p).sclassid);
            *(*p).data.fdata.chunktab.offset(indx as isize) = prevchunkid;
            fsnodes_get_stats(p, &raw mut nsr, 1 as uint8_t);
            e = (*p).parents;
            while !e.is_null() {
                fsnodes_add_sub_stats((*e).parent as *mut fsnode, &raw mut nsr, &raw mut psr);
                e = (*e).nextparent as *mut fsedge;
            }
        }
        if mr != 0 {
            meta_version_inc();
            chunk_mr_unlock(ts, chunkid);
        } else {
            ts = main_time();
            changelog(
                b"%u|ROLLBACK(%u,%u,%lu,%lu)\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode,
                indx,
                prevchunkid,
                chunkid,
            );
            chunk_unlock(ts, chunkid);
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_rollback(
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut prevchunkid: uint64_t,
    mut chunkid: uint64_t,
) -> uint8_t {
    unsafe {
        return fs_univ_rollback(
            0 as uint8_t,
            0 as uint32_t,
            inode,
            indx,
            prevchunkid,
            chunkid,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_rollback(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut prevchunkid: uint64_t,
    mut chunkid: uint64_t,
) -> uint8_t {
    unsafe {
        return fs_univ_rollback(1 as uint8_t, ts, inode, indx, prevchunkid, chunkid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_writeend(
    mut inode: uint32_t,
    mut length: uint64_t,
    mut chunkid: uint64_t,
    mut chunkopflags: uint8_t,
    mut flenghaschanged: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut ts: uint32_t = main_time();
        if length > 0 as uint64_t {
            let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
            p = fsnodes_node_find(inode);
            if p.is_null() {
                return MFS_ERROR_ENOENT as uint8_t;
            }
            if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
                && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
                && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
            if length > (*p).data.fdata.length {
                if fsnodes_test_quota(
                    p,
                    0 as uint32_t,
                    length.wrapping_sub((*p).data.fdata.length),
                    0 as uint64_t,
                    0 as uint64_t,
                ) != 0
                {
                    return MFS_ERROR_QUOTA as uint8_t;
                }
                fsnodes_setlength(p, length);
                if chunkopflags as ::core::ffi::c_int & CHUNKOPFLAG_CANMODTIME != 0 {
                    (*p).ctime = ts;
                    (*p).mtime = (*p).ctime;
                    fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME) as uint8_t);
                } else {
                    fsnodes_checkarchmode(p, ts, 0 as uint8_t);
                }
                changelog(
                    b"%u|LENGTH(%u,%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    inode,
                    length,
                    if chunkopflags as ::core::ffi::c_int & CHUNKOPFLAG_CANMODTIME != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
                if !flenghaschanged.is_null() {
                    *flenghaschanged = 1 as uint8_t;
                }
                appendres_setrleng(inode, length);
            }
        }
        changelog(
            b"%u|UNLOCK(%lu)\0".as_ptr() as *const ::core::ffi::c_char,
            ts,
            chunkid,
        );
        return chunk_unlock(ts, chunkid) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_amtime(
    mut inode: uint32_t,
    mut ts: uint32_t,
    mut xatime: uint32_t,
    mut xmtime: uint32_t,
    mut xctime: uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        (*p).atime = xatime;
        (*p).mtime = xmtime;
        (*p).ctime = xctime;
        fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME | CHECK_ATIME) as uint8_t);
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_amtime_update(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inodetab: *mut uint32_t,
    mut atimetab: *mut uint32_t,
    mut mtimetab: *mut uint32_t,
    mut cnt: uint32_t,
) {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut i: uint32_t = 0;
        let mut atime: uint32_t = 0;
        let mut mtime: uint32_t = 0;
        let mut ts: uint32_t = 0;
        let mut chg: uint8_t = 0;
        ts = main_time();
        i = 0 as uint32_t;
        while i < cnt {
            if fsnodes_node_find_ext(
                rootinode,
                sesflags,
                inodetab.offset(i as isize),
                ::core::ptr::null_mut::<*mut fsnode>(),
                &raw mut p,
                1 as uint8_t,
            ) != 0
            {
                chg = 0 as uint8_t;
                atime = *atimetab.offset(i as isize);
                mtime = *mtimetab.offset(i as isize);
                if (*p).atime < atime {
                    if AtimeMode as ::core::ffi::c_int == ATIME_ALWAYS
                        || AtimeMode as ::core::ffi::c_int == ATIME_FILES_ONLY
                        || ((*p).atime <= (*p).ctime && atime >= (*p).ctime
                            || (*p).atime <= (*p).mtime && atime >= (*p).mtime
                            || (*p).atime.wrapping_add(86400 as uint32_t) < atime)
                            && (AtimeMode as ::core::ffi::c_int == ATIME_RELATIVE_ONLY
                                || AtimeMode as ::core::ffi::c_int == ATIME_FILES_AND_RELATIVE_ONLY)
                    {
                        (*p).atime = atime;
                        chg = 1 as uint8_t;
                    }
                }
                if (*p).mtime < mtime {
                    (*p).mtime = mtime;
                    (*p).ctime = (*p).mtime;
                    chg = 1 as uint8_t;
                }
                if chg != 0 {
                    fsnodes_checkarchmode(
                        p,
                        ts,
                        (CHECK_CTIME | CHECK_MTIME | CHECK_ATIME) as uint8_t,
                    );
                    changelog(
                        b"%u|AMTIME(%u,%u,%u,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                        ts,
                        *inodetab.offset(i as isize),
                        (*p).atime,
                        (*p).mtime,
                        (*p).ctime,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_repair(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut flags: uint8_t,
    mut notchanged: *mut uint32_t,
    mut erased: *mut uint32_t,
    mut repaired: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut nversion: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut psr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut nsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ts: uint32_t = main_time();
        *notchanged = 0 as uint32_t;
        *erased = 0 as uint32_t;
        *repaired = 0 as uint32_t;
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_W as uint8_t, sesflags) == 0 {
            return MFS_ERROR_EACCES as uint8_t;
        }
        fsnodes_get_stats(p, &raw mut psr, 0 as uint8_t);
        indx = 0 as uint32_t;
        while indx < (*p).data.fdata.chunks {
            if chunk_repair(
                (*p).sclassid,
                *(*p).data.fdata.chunktab.offset(indx as isize),
                flags,
                &raw mut nversion,
            ) != 0
            {
                changelog(
                    b"%u|REPAIR(%u,%u):%u\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    inode,
                    indx,
                    nversion,
                );
                (*p).ctime = ts;
                (*p).mtime = (*p).ctime;
                fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME) as uint8_t);
                if nversion > 0 as uint32_t {
                    *repaired = (*repaired).wrapping_add(1);
                } else {
                    *(*p).data.fdata.chunktab.offset(indx as isize) = 0 as uint64_t;
                    *erased = (*erased).wrapping_add(1);
                }
            } else {
                *notchanged = (*notchanged).wrapping_add(1);
            }
            indx = indx.wrapping_add(1);
        }
        fsnodes_get_stats(p, &raw mut nsr, 1 as uint8_t);
        e = (*p).parents;
        while !e.is_null() {
            fsnodes_add_sub_stats((*e).parent as *mut fsnode, &raw mut nsr, &raw mut psr);
            e = (*e).nextparent as *mut fsedge;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_repair(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut nversion: uint32_t,
) -> uint8_t {
    unsafe {
        let mut psr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut nsr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut status: uint8_t = 0;
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if indx > MAX_INDEX as uint32_t {
            return MFS_ERROR_INDEXTOOBIG as uint8_t;
        }
        if indx >= (*p).data.fdata.chunks {
            return MFS_ERROR_NOCHUNK as uint8_t;
        }
        if *(*p).data.fdata.chunktab.offset(indx as isize) == 0 as uint64_t {
            return MFS_ERROR_NOCHUNK as uint8_t;
        }
        fsnodes_get_stats(p, &raw mut psr, 0 as uint8_t);
        if nversion == 0 as uint32_t {
            status = chunk_delete_file(
                *(*p).data.fdata.chunktab.offset(indx as isize),
                (*p).sclassid,
            ) as uint8_t;
            *(*p).data.fdata.chunktab.offset(indx as isize) = 0 as uint64_t;
            if status as ::core::ffi::c_int == MFS_STATUS_OK {
                meta_version_inc();
            }
        } else {
            status = chunk_mr_set_version(*(*p).data.fdata.chunktab.offset(indx as isize), nversion)
                as uint8_t;
        }
        fsnodes_get_stats(p, &raw mut nsr, 1 as uint8_t);
        e = (*p).parents;
        while !e.is_null() {
            fsnodes_add_sub_stats((*e).parent as *mut fsnode, &raw mut nsr, &raw mut psr);
            e = (*e).nextparent as *mut fsedge;
        }
        (*p).ctime = ts;
        (*p).mtime = (*p).ctime;
        fsnodes_checkarchmode(p, ts, (CHECK_CTIME | CHECK_MTIME) as uint8_t);
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getsclass(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut gmode: uint8_t,
    mut fgtab: *mut uint32_t,
    mut dgtab: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        memset(
            fgtab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (MAXSCLASS as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>()),
        );
        memset(
            dgtab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (MAXSCLASS as size_t).wrapping_mul(::core::mem::size_of::<uint32_t>()),
        );
        if !(gmode as uint32_t <= 1 as uint32_t) {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            && (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        fsnodes_keep_alive_begin();
        fsnodes_getsclass_recursive(p, gmode, fgtab, dgtab);
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_gettrashretention_prepare(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut gmode: uint8_t,
    mut fptr: *mut *mut ::core::ffi::c_void,
    mut dptr: *mut *mut ::core::ffi::c_void,
    mut fnodes: *mut uint32_t,
    mut dnodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut froot: *mut bstnode = ::core::ptr::null_mut::<bstnode>();
        let mut droot: *mut bstnode = ::core::ptr::null_mut::<bstnode>();
        froot = ::core::ptr::null_mut::<bstnode>();
        droot = ::core::ptr::null_mut::<bstnode>();
        *fptr = NULL;
        *dptr = NULL;
        *fnodes = 0 as uint32_t;
        *dnodes = 0 as uint32_t;
        if !(gmode as uint32_t <= 1 as uint32_t) {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            && (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        fsnodes_keep_alive_begin();
        fsnodes_gettrashretention_recursive(p, gmode, &raw mut froot, &raw mut droot);
        *fptr = froot as *mut ::core::ffi::c_void;
        *dptr = droot as *mut ::core::ffi::c_void;
        *fnodes = fsnodes_bst_nodes(froot);
        *dnodes = fsnodes_bst_nodes(droot);
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_gettrashretention_store(
    mut fptr: *mut ::core::ffi::c_void,
    mut dptr: *mut ::core::ffi::c_void,
    mut buff: *mut uint8_t,
) {
    unsafe {
        let mut froot: *mut bstnode = ::core::ptr::null_mut::<bstnode>();
        let mut droot: *mut bstnode = ::core::ptr::null_mut::<bstnode>();
        froot = fptr as *mut bstnode;
        droot = dptr as *mut bstnode;
        fsnodes_bst_storedata(froot, &raw mut buff);
        fsnodes_bst_storedata(droot, &raw mut buff);
        fsnodes_bst_free(froot);
        fsnodes_bst_free(droot);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_geteattr(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut gmode: uint8_t,
    mut feattrtab: *mut uint32_t,
    mut deattrtab: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        memset(
            feattrtab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (((1 as ::core::ffi::c_int) << EATTR_BITS) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint32_t>()),
        );
        memset(
            deattrtab as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (((1 as ::core::ffi::c_int) << EATTR_BITS) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint32_t>()),
        );
        if !(gmode as uint32_t <= 1 as uint32_t) {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        fsnodes_keep_alive_begin();
        fsnodes_geteattr_recursive(p, gmode, feattrtab, deattrtab);
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_setsclass(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut src_sclassid: uint8_t,
    mut dst_sclassid: uint8_t,
    mut smode: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut realsize: uint64_t = 0;
        let mut admin: uint8_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        *sinodes = 0 as uint32_t;
        *ncinodes = 0 as uint32_t;
        *nsinodes = 0 as uint32_t;
        if !(smode as uint32_t <= 7 as uint32_t)
            || dst_sclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || src_sclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (smode as ::core::ffi::c_int & SMODE_TMASK == SMODE_INCREASE
            || smode as ::core::ffi::c_int & SMODE_TMASK == SMODE_DECREASE)
            && dst_sclassid as ::core::ffi::c_int > 9 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            && (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        fsnodes_keep_alive_begin();
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if smode as ::core::ffi::c_int & SMODE_TMASK == SMODE_SET
                || smode as ::core::ffi::c_int & SMODE_TMASK == SMODE_INCREASE
            {
                realsize = 0 as uint64_t;
                if fsnodes_setsclass_recursive_test_quota(
                    p,
                    uid,
                    sclass_get_keeparch_maxstorage_eights(dst_sclassid as uint16_t),
                    (if smode as ::core::ffi::c_int & SMODE_RMASK != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    &raw mut realsize,
                ) != 0
                {
                    return MFS_ERROR_QUOTA as uint8_t;
                }
            }
        }
        admin = (if sesflags as ::core::ffi::c_int & (SESFLAG_ADMIN | SESFLAG_METARESTORE) != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        fsnodes_setsclass_recursive(
            p,
            ts,
            uid,
            src_sclassid,
            dst_sclassid,
            smode,
            admin,
            sinodes,
            ncinodes,
            nsinodes,
        );
        if smode as ::core::ffi::c_int & SMODE_RMASK == 0 as ::core::ffi::c_int
            && *nsinodes > 0 as uint32_t
            && *sinodes == 0 as uint32_t
            && *ncinodes == 0 as uint32_t
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|SETSCLASS(%u,%u,%hhu,%hhu,%hhu):%u,%u,%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ts,
                inode,
                uid,
                src_sclassid as ::core::ffi::c_int,
                dst_sclassid as ::core::ffi::c_int,
                smode as ::core::ffi::c_int,
                *sinodes,
                *ncinodes,
                *nsinodes,
            );
        } else {
            meta_version_inc();
        }
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_setsclass(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut src_sclassid: uint8_t,
    mut dst_sclassid: uint8_t,
    mut smode: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_setsclass(
            main_time(),
            rootinode,
            sesflags,
            inode,
            uid,
            src_sclassid,
            dst_sclassid,
            smode,
            sinodes,
            ncinodes,
            nsinodes,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_setsclass(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut src_sclassid: uint8_t,
    mut dst_sclassid: uint8_t,
    mut smode: uint8_t,
    mut sinodes: uint32_t,
    mut ncinodes: uint32_t,
    mut nsinodes: uint32_t,
) -> uint8_t {
    unsafe {
        let mut si: uint32_t = 0;
        let mut nci: uint32_t = 0;
        let mut nsi: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = fs_univ_setsclass(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            inode,
            uid,
            src_sclassid,
            dst_sclassid,
            smode,
            &raw mut si,
            &raw mut nci,
            &raw mut nsi,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if sinodes != si || ncinodes != nci || nsinodes != nsi {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"SETSCLASS data mismatch: my:(%u,%u,%u) != expected:(%u,%u,%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                si,
                nci,
                nsi,
                sinodes,
                ncinodes,
                nsinodes,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_settrashretention(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut trashretention: uint32_t,
    mut smode: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        *sinodes = 0 as uint32_t;
        *ncinodes = 0 as uint32_t;
        *nsinodes = 0 as uint32_t;
        if !(smode as uint32_t <= 7 as uint32_t) {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int
            && sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0
        {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            && (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        fsnodes_keep_alive_begin();
        fsnodes_settrashretention_recursive(
            p,
            ts,
            uid,
            trashretention
                .wrapping_add(3599 as uint32_t)
                .wrapping_div(3600 as uint32_t) as uint16_t,
            smode,
            sinodes,
            ncinodes,
            nsinodes,
        );
        if smode as ::core::ffi::c_int & SMODE_RMASK == 0 as ::core::ffi::c_int
            && *nsinodes > 0 as uint32_t
            && *sinodes == 0 as uint32_t
            && *ncinodes == 0 as uint32_t
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|SETTRASHTIME(%u,%u,%u,%hhu):%u,%u,%u\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode,
                uid,
                trashretention,
                smode as ::core::ffi::c_int,
                *sinodes,
                *ncinodes,
                *nsinodes,
            );
        } else {
            meta_version_inc();
        }
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_settrashretention(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut trashretention: uint32_t,
    mut smode: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_settrashretention(
            main_time(),
            rootinode,
            sesflags,
            inode,
            uid,
            trashretention,
            smode,
            sinodes,
            ncinodes,
            nsinodes,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_settrashretention(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut trashretention: uint32_t,
    mut smode: uint8_t,
    mut sinodes: uint32_t,
    mut ncinodes: uint32_t,
    mut nsinodes: uint32_t,
) -> uint8_t {
    unsafe {
        let mut si: uint32_t = 0;
        let mut nci: uint32_t = 0;
        let mut nsi: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = fs_univ_settrashretention(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            inode,
            uid,
            trashretention,
            smode,
            &raw mut si,
            &raw mut nci,
            &raw mut nsi,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if sinodes != si || ncinodes != nci || nsinodes != nsi {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"SETTRASHTIME data mismatch: my:(%u,%u,%u) != expected:(%u,%u,%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                si,
                nci,
                nsi,
                sinodes,
                ncinodes,
                nsinodes,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_seteattr(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut eattr: uint8_t,
    mut smode: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        *sinodes = 0 as uint32_t;
        *ncinodes = 0 as uint32_t;
        *nsinodes = 0 as uint32_t;
        if !(smode as uint32_t <= 7 as uint32_t)
            || eattr as ::core::ffi::c_int
                & !(EATTR_NOOWNER
                    | EATTR_NOACACHE
                    | EATTR_NOECACHE
                    | EATTR_NODATACACHE
                    | EATTR_SNAPSHOT
                    | EATTR_UNDELETABLE
                    | EATTR_APPENDONLY
                    | EATTR_IMMUTABLE)
                != 0
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
                return MFS_ERROR_EROFS as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_ADMIN == 0 as ::core::ffi::c_int
                && eattr as ::core::ffi::c_int
                    & (EATTR_UNDELETABLE | EATTR_APPENDONLY | EATTR_IMMUTABLE)
                    != 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        fsnodes_keep_alive_begin();
        fsnodes_seteattr_recursive(p, ts, uid, eattr, smode, sinodes, ncinodes, nsinodes);
        if smode as ::core::ffi::c_int & SMODE_RMASK == 0 as ::core::ffi::c_int
            && *nsinodes > 0 as uint32_t
            && *sinodes == 0 as uint32_t
            && *ncinodes == 0 as uint32_t
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|SETEATTR(%u,%u,%hhu,%hhu):%u,%u,%u\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode,
                uid,
                eattr as ::core::ffi::c_int,
                smode as ::core::ffi::c_int,
                *sinodes,
                *ncinodes,
                *nsinodes,
            );
        } else {
            meta_version_inc();
        }
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_seteattr(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut eattr: uint8_t,
    mut smode: uint8_t,
    mut sinodes: *mut uint32_t,
    mut ncinodes: *mut uint32_t,
    mut nsinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_seteattr(
            main_time(),
            rootinode,
            sesflags,
            inode,
            uid,
            eattr,
            smode,
            sinodes,
            ncinodes,
            nsinodes,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_seteattr(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut eattr: uint8_t,
    mut smode: uint8_t,
    mut sinodes: uint32_t,
    mut ncinodes: uint32_t,
    mut nsinodes: uint32_t,
) -> uint8_t {
    unsafe {
        let mut si: uint32_t = 0;
        let mut nci: uint32_t = 0;
        let mut nsi: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = fs_univ_seteattr(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            inode,
            uid,
            eattr,
            smode,
            &raw mut si,
            &raw mut nci,
            &raw mut nsi,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if sinodes != si || ncinodes != nci || nsinodes != nsi {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"SETEATTR data mismatch: my:(%u,%u,%u) != expected:(%u,%u,%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                si,
                nci,
                nsi,
                sinodes,
                ncinodes,
                nsinodes,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_listxattr_leng(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut xanode: *mut *mut ::core::ffi::c_void,
    mut xasize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        *xasize = 0 as uint32_t;
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            opened,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if opened as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_R as uint8_t, sesflags) == 0 {
                return MFS_ERROR_EACCES as uint8_t;
            }
        }
        return xattr_listattr_leng(inode, xanode, xasize);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_listxattr_data(
    mut xanode: *mut ::core::ffi::c_void,
    mut xabuff: *mut uint8_t,
) {
    unsafe {
        xattr_listattr_data(xanode, xabuff);
        stats_getxattr = stats_getxattr.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_setxattr(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut anleng: uint8_t,
    mut attrname: *const uint8_t,
    mut avleng: uint32_t,
    mut attrvalue: *const uint8_t,
    mut mode: uint8_t,
) -> uint8_t {
    unsafe {
        let mut ts: uint32_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut status: uint8_t = 0;
        ts = main_time();
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            opened,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if uid != 0 as uint32_t {
            if anleng as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
                && memcmp(
                    attrname as *const ::core::ffi::c_void,
                    b"trusted.\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                || anleng as ::core::ffi::c_int >= 9 as ::core::ffi::c_int
                    && memcmp(
                        attrname as *const ::core::ffi::c_void,
                        b"security.\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        9 as size_t,
                    ) == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int
            && uid != 0 as uint32_t
            && uid != (*p).uid
        {
            if anleng as ::core::ffi::c_int >= 7 as ::core::ffi::c_int
                && memcmp(
                    attrname as *const ::core::ffi::c_void,
                    b"system.\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if opened as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_W as uint8_t, sesflags) == 0 {
                return MFS_ERROR_EACCES as uint8_t;
            }
        }
        if xattr_namecheck(anleng, attrname) < 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if mode as ::core::ffi::c_int > MFS_XATTR_REMOVE {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        status = xattr_setattr(inode, anleng, attrname, avleng, attrvalue, mode);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        (*p).ctime = ts;
        fsnodes_checkarchmode(p, ts, CHECK_CTIME as uint8_t);
        changelog(
            b"%u|SETXATTR(%u,%s,%s,%hhu)\0".as_ptr() as *const ::core::ffi::c_char,
            ts,
            inode,
            changelog_escape_name(anleng as uint32_t, attrname),
            changelog_escape_name(avleng, attrvalue),
            mode as ::core::ffi::c_int,
        );
        stats_setxattr = stats_setxattr.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getxattr(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut anleng: uint8_t,
    mut attrname: *const uint8_t,
    mut avleng: *mut uint32_t,
    mut attrvalue: *mut *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            opened,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if uid != 0 as uint32_t {
            if anleng as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
                && memcmp(
                    attrname as *const ::core::ffi::c_void,
                    b"trusted.\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        if opened as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if fsnodes_access_ext(p, uid, gids, gid, MODE_MASK_R as uint8_t, sesflags) == 0 {
                return MFS_ERROR_EACCES as uint8_t;
            }
        }
        if xattr_namecheck(anleng, attrname) < 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        stats_getxattr = stats_getxattr.wrapping_add(1);
        return xattr_getattr(inode, anleng, attrname, avleng, attrvalue);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_setxattr(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut anleng: uint32_t,
    mut attrname: *const uint8_t,
    mut avleng: uint32_t,
    mut attrvalue: *const uint8_t,
    mut mode: uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut status: uint8_t = 0;
        if anleng == 0 as uint32_t
            || anleng > MFS_XATTR_NAME_MAX as uint32_t
            || avleng > MFS_XATTR_SIZE_MAX as uint32_t
            || mode > MFS_XATTR_REMOVE as uint32_t
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        status = xattr_setattr(
            inode,
            anleng as uint8_t,
            attrname,
            avleng,
            attrvalue,
            mode as uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        (*p).ctime = ts;
        fsnodes_checkarchmode(p, ts, CHECK_CTIME as uint8_t);
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            meta_version_inc();
        }
        stats_setxattr = stats_setxattr.wrapping_add(1);
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_setfacl(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut acltype: uint8_t,
    mut userperm: uint16_t,
    mut groupperm: uint16_t,
    mut otherperm: uint16_t,
    mut mask: uint16_t,
    mut namedusers: uint16_t,
    mut namedgroups: uint16_t,
    mut aclblob: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut ts: uint32_t = 0;
        let mut pmode: uint16_t = 0;
        let mut chg: uint8_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        ts = main_time();
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_NOOWNER == 0 as ::core::ffi::c_int
            && uid != 0 as uint32_t
            && uid != (*p).uid
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if (*p).eattr as ::core::ffi::c_int & EATTR_IMMUTABLE != 0 {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if acltype as ::core::ffi::c_int != POSIX_ACL_ACCESS
            && acltype as ::core::ffi::c_int != POSIX_ACL_DEFAULT
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        pmode = (*p).mode() as uint16_t;
        chg = 0 as uint8_t;
        if userperm as ::core::ffi::c_int
            & groupperm as ::core::ffi::c_int
            & otherperm as ::core::ffi::c_int
            & mask as ::core::ffi::c_int
            == 0xffff as ::core::ffi::c_int
            && namedusers as ::core::ffi::c_int | namedgroups as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            posix_acl_remove(inode, acltype);
            if acltype as ::core::ffi::c_int == POSIX_ACL_ACCESS {
                (*p).set_aclpermflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if acltype as ::core::ffi::c_int == POSIX_ACL_DEFAULT {
                (*p).set_acldefflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        } else {
            if acltype as ::core::ffi::c_int == POSIX_ACL_DEFAULT
                && (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            {
                return MFS_ERROR_EACCES as uint8_t;
            }
            if userperm as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int {
                userperm = ((*p).mode() as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                    & 7 as ::core::ffi::c_int) as uint16_t;
            }
            if groupperm as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int {
                groupperm = ((*p).mode() as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
                    & 7 as ::core::ffi::c_int) as uint16_t;
            }
            if otherperm as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int {
                otherperm =
                    ((*p).mode() as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as uint16_t;
            }
            posix_acl_set(
                inode,
                acltype,
                userperm,
                groupperm,
                otherperm,
                mask,
                namedusers,
                namedgroups,
                aclblob,
            );
            if acltype as ::core::ffi::c_int == POSIX_ACL_ACCESS {
                (*p).set_mode((*p).mode() & 0o7000 as ::core::ffi::c_int as ::core::ffi::c_uint);
                (*p).set_mode(
                    (*p).mode()
                        | ((userperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int
                            | (groupperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                                << 3 as ::core::ffi::c_int
                            | otherperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint,
                );
            }
            if (*p).mode() as ::core::ffi::c_int != pmode as ::core::ffi::c_int {
                (*p).ctime = ts;
                fsnodes_checkarchmode(p, ts, CHECK_CTIME as uint8_t);
                chg = 1 as uint8_t;
            }
        }
        changelog(
            b"%u|SETACL(%u,%u,%u,%hhu,%hu,%hu,%hu,%hu,%hu,%hu,%s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            ts,
            inode,
            (*p).mode() as ::core::ffi::c_int,
            chg as ::core::ffi::c_int,
            acltype as ::core::ffi::c_int,
            userperm as ::core::ffi::c_int,
            groupperm as ::core::ffi::c_int,
            otherperm as ::core::ffi::c_int,
            mask as ::core::ffi::c_int,
            namedusers as ::core::ffi::c_int,
            namedgroups as ::core::ffi::c_int,
            changelog_escape_name(
                ((namedusers as ::core::ffi::c_int + namedgroups as ::core::ffi::c_int)
                    * 6 as ::core::ffi::c_int) as uint32_t,
                aclblob,
            ),
        );
        stats_setfacl = stats_setfacl.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getfacl_size(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut acltype: uint8_t,
    mut custom: *mut *mut ::core::ffi::c_void,
    mut aclblobsize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut bsize: int32_t = 0;
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if acltype as ::core::ffi::c_int == POSIX_ACL_ACCESS
            && (*p).aclpermflag() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || acltype as ::core::ffi::c_int == POSIX_ACL_DEFAULT
                && (*p).acldefflag() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOATTR as uint8_t;
        }
        bsize = posix_acl_get_blobsize(inode, acltype, custom);
        if bsize < 0 as int32_t {
            return MFS_ERROR_ENOATTR as uint8_t;
        }
        *aclblobsize = bsize as uint32_t;
        stats_getfacl = stats_getfacl.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getfacl_data(
    mut custom: *mut ::core::ffi::c_void,
    mut userperm: *mut uint16_t,
    mut groupperm: *mut uint16_t,
    mut otherperm: *mut uint16_t,
    mut mask: *mut uint16_t,
    mut namedusers: *mut uint16_t,
    mut namedgroups: *mut uint16_t,
    mut aclblob: *mut uint8_t,
) {
    unsafe {
        posix_acl_get_data(
            custom,
            userperm,
            groupperm,
            otherperm,
            mask,
            namedusers,
            namedgroups,
            aclblob,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_setacl(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut mode: uint16_t,
    mut changectime: uint8_t,
    mut acltype: uint8_t,
    mut userperm: uint16_t,
    mut groupperm: uint16_t,
    mut otherperm: uint16_t,
    mut mask: uint16_t,
    mut namedusers: uint16_t,
    mut namedgroups: uint16_t,
    mut aclblob: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if acltype as ::core::ffi::c_int != POSIX_ACL_ACCESS
            && acltype as ::core::ffi::c_int != POSIX_ACL_DEFAULT
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if userperm as ::core::ffi::c_int
            & groupperm as ::core::ffi::c_int
            & otherperm as ::core::ffi::c_int
            & mask as ::core::ffi::c_int
            == 0xffff as ::core::ffi::c_int
            && namedusers as ::core::ffi::c_int | namedgroups as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            posix_acl_remove(inode, acltype);
            if acltype as ::core::ffi::c_int == POSIX_ACL_ACCESS {
                (*p).set_aclpermflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if acltype as ::core::ffi::c_int == POSIX_ACL_DEFAULT {
                (*p).set_acldefflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        } else {
            posix_acl_set(
                inode,
                acltype,
                userperm,
                groupperm,
                otherperm,
                mask,
                namedusers,
                namedgroups,
                aclblob,
            );
            (*p).set_mode(mode as ::core::ffi::c_uint as ::core::ffi::c_uint);
            if changectime != 0 {
                (*p).ctime = ts;
                fsnodes_checkarchmode(p, ts, CHECK_CTIME as uint8_t);
            }
        }
        meta_version_inc();
        stats_setfacl = stats_setfacl.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_quotacontrol(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut delflag: uint8_t,
    mut flags: *mut uint8_t,
    mut defaultgp: *mut uint8_t,
    mut graceperiod: *mut uint32_t,
    mut sinodes: *mut uint32_t,
    mut slength: *mut uint64_t,
    mut ssize: *mut uint64_t,
    mut srealsize: *mut uint64_t,
    mut hinodes: *mut uint32_t,
    mut hlength: *mut uint64_t,
    mut hsize: *mut uint64_t,
    mut hrealsize: *mut uint64_t,
    mut curinodes: *mut uint32_t,
    mut curlength: *mut uint64_t,
    mut cursize: *mut uint64_t,
    mut currealsize: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        let mut psr: *mut statsrecord = ::core::ptr::null_mut::<statsrecord>();
        let mut chg: uint8_t = 0;
        chg = (if *flags as ::core::ffi::c_int != 0 || *graceperiod != 0xffffffff as uint32_t {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if chg != 0 {
            if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
                return MFS_ERROR_EROFS as uint8_t;
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_ADMIN == 0 as ::core::ffi::c_int {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        if rootinode == 0 as uint32_t {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_EPERM as uint8_t;
        }
        qn = (*p).data.ddata.quota;
        if delflag != 0 {
            if !qn.is_null() {
                (*qn).flags = ((*qn).flags as ::core::ffi::c_int & !(*flags as ::core::ffi::c_int))
                    as uint8_t;
                if (*qn).flags as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    chg = 1 as uint8_t;
                    fsnodes_delete_quotanode(p);
                    qn = ::core::ptr::null_mut::<quotanode>();
                }
            }
        } else if chg != 0 {
            if qn.is_null() {
                qn = fsnodes_new_quotanode(p);
            }
            if !qn.is_null() {
                (*qn).flags =
                    ((*qn).flags as ::core::ffi::c_int | *flags as ::core::ffi::c_int) as uint8_t;
                if *graceperiod != 0xffffffff as uint32_t {
                    (*qn).graceperiod = *graceperiod;
                }
                if *flags as ::core::ffi::c_int & QUOTA_FLAG_SINODES != 0 {
                    (*qn).sinodes = *sinodes;
                }
                if *flags as ::core::ffi::c_int & QUOTA_FLAG_SLENGTH != 0 {
                    (*qn).slength = *slength;
                }
                if *flags as ::core::ffi::c_int & QUOTA_FLAG_SSIZE != 0 {
                    (*qn).ssize = *ssize;
                }
                if *flags as ::core::ffi::c_int & QUOTA_FLAG_SREALSIZE != 0 {
                    (*qn).srealsize = *srealsize;
                }
                if *flags as ::core::ffi::c_int & QUOTA_FLAG_HINODES != 0 {
                    (*qn).hinodes = *hinodes;
                }
                if *flags as ::core::ffi::c_int & QUOTA_FLAG_HLENGTH != 0 {
                    (*qn).hlength = *hlength;
                }
                if *flags as ::core::ffi::c_int & QUOTA_FLAG_HSIZE != 0 {
                    (*qn).hsize = *hsize;
                }
                if *flags as ::core::ffi::c_int & QUOTA_FLAG_HREALSIZE != 0 {
                    (*qn).hrealsize = *hrealsize;
                }
            }
        }
        if !qn.is_null() {
            if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SINODES == 0 as ::core::ffi::c_int {
                (*qn).sinodes = 0 as uint32_t;
            }
            if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HINODES == 0 as ::core::ffi::c_int {
                (*qn).hinodes = 0 as uint32_t;
            }
            if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SLENGTH == 0 as ::core::ffi::c_int {
                (*qn).slength = 0 as uint64_t;
            }
            if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HLENGTH == 0 as ::core::ffi::c_int {
                (*qn).hlength = 0 as uint64_t;
            }
            if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SSIZE == 0 as ::core::ffi::c_int {
                (*qn).ssize = 0 as uint64_t;
            }
            if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HSIZE == 0 as ::core::ffi::c_int {
                (*qn).hsize = 0 as uint64_t;
            }
            if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SREALSIZE == 0 as ::core::ffi::c_int {
                (*qn).srealsize = 0 as uint64_t;
            }
            if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HREALSIZE == 0 as ::core::ffi::c_int {
                (*qn).hrealsize = 0 as uint64_t;
            }
            *flags = (*qn).flags;
            *defaultgp = (if (*qn).graceperiod == 0 as uint32_t {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            *graceperiod = if (*qn).graceperiod != 0 as uint32_t {
                (*qn).graceperiod
            } else {
                QuotaDefaultGracePeriod
            };
            *sinodes = (*qn).sinodes;
            *slength = (*qn).slength;
            *ssize = (*qn).ssize;
            *srealsize = (*qn).srealsize;
            *hinodes = (*qn).hinodes;
            *hlength = (*qn).hlength;
            *hsize = (*qn).hsize;
            *hrealsize = (*qn).hrealsize;
        } else {
            *flags = 0 as uint8_t;
            *defaultgp = 0 as uint8_t;
            *graceperiod = 0 as uint32_t;
            *sinodes = 0 as uint32_t;
            *slength = 0 as uint64_t;
            *ssize = 0 as uint64_t;
            *srealsize = 0 as uint64_t;
            *hinodes = 0 as uint32_t;
            *hlength = 0 as uint64_t;
            *hsize = 0 as uint64_t;
            *hrealsize = 0 as uint64_t;
        }
        psr = &raw mut (*p).data.ddata.stats;
        *curinodes = (*psr).inodes;
        *curlength = (*psr).length;
        *cursize = (*psr).size;
        *currealsize = (*psr).realsize;
        if chg != 0 {
            if !qn.is_null() {
                changelog(
                    b"%u|QUOTA(%u,%hhu,%hhu,%u,%u,%u,%lu,%lu,%lu,%lu,%lu,%lu,%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    main_time(),
                    inode,
                    (*qn).exceeded as ::core::ffi::c_int,
                    (*qn).flags as ::core::ffi::c_int,
                    (*qn).stimestamp,
                    (*qn).sinodes,
                    (*qn).hinodes,
                    (*qn).slength,
                    (*qn).hlength,
                    (*qn).ssize,
                    (*qn).hsize,
                    (*qn).srealsize,
                    (*qn).hrealsize,
                    (*qn).graceperiod,
                );
            } else {
                changelog(
                    b"%u|QUOTA(%u,0,0,0,0,0,0,0,0,0,0,0,0)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    main_time(),
                    inode,
                );
            }
        }
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_quota(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut exceeded: uint8_t,
    mut flags: uint8_t,
    mut stimestamp: uint32_t,
    mut sinodes: uint32_t,
    mut hinodes: uint32_t,
    mut slength: uint64_t,
    mut hlength: uint64_t,
    mut ssize: uint64_t,
    mut hsize: uint64_t,
    mut srealsize: uint64_t,
    mut hrealsize: uint64_t,
    mut graceperiod: uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        p = fsnodes_node_find(inode);
        if p.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
            return MFS_ERROR_EPERM as uint8_t;
        }
        qn = (*p).data.ddata.quota;
        if flags as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if !qn.is_null() {
                fsnodes_delete_quotanode(p);
            }
        } else {
            if qn.is_null() {
                qn = fsnodes_new_quotanode(p);
            }
            (*qn).flags = flags;
            (*qn).graceperiod = graceperiod;
            (*qn).exceeded = exceeded;
            (*qn).stimestamp = stimestamp;
            (*qn).sinodes = sinodes;
            (*qn).slength = slength;
            (*qn).ssize = ssize;
            (*qn).srealsize = srealsize;
            (*qn).hinodes = hinodes;
            (*qn).hlength = hlength;
            (*qn).hsize = hsize;
            (*qn).hrealsize = hrealsize;
        }
        meta_version_inc();
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getquotainfo(mut buff: *mut uint8_t, mut ver: uint8_t) -> uint32_t {
    unsafe {
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        let mut psr: *mut statsrecord = ::core::ptr::null_mut::<statsrecord>();
        let mut size: uint32_t = 0;
        let mut ts: uint32_t = 0;
        let mut graceperiod: uint32_t = 0;
        if !buff.is_null() {
            ts = main_time();
        } else {
            ts = 0 as uint32_t;
        }
        qn = quotahead;
        while !qn.is_null() {
            if (*qn).node.is_null() {
                size = 0 as uint32_t;
            } else {
                size = fsnodes_getpath_size((*(*qn).node).parents);
            }
            if buff.is_null() {
                ts = ts.wrapping_add(
                    ((4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                            * (4 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int) as uint32_t)
                        .wrapping_add(size),
                );
            } else {
                psr = &raw mut (*(*qn).node).data.ddata.stats;
                put32bit(&raw mut buff, (*(*qn).node).inode);
                put32bit(&raw mut buff, size.wrapping_add(1 as uint32_t));
                put8bit(&raw mut buff, '/' as uint8_t);
                fsnodes_getpath_data((*(*qn).node).parents, buff, size);
                buff = buff.offset(size as isize);
                graceperiod = if (*qn).graceperiod != QUOTA_PERIOD_DEFAULT as uint32_t {
                    (*qn).graceperiod
                } else {
                    QuotaDefaultGracePeriod
                };
                put32bit(&raw mut buff, graceperiod);
                if ver != 0 {
                    put8bit(
                        &raw mut buff,
                        ((if (*qn).exceeded as ::core::ffi::c_int != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) | (if (*qn).graceperiod == QUOTA_PERIOD_DEFAULT as uint32_t {
                            2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint8_t,
                    );
                } else {
                    put8bit(&raw mut buff, (*qn).exceeded);
                }
                put8bit(&raw mut buff, (*qn).flags);
                if (*qn).stimestamp == 0 as uint32_t {
                    put32bit(&raw mut buff, 0xffffffff as uint32_t);
                } else if (*qn).stimestamp.wrapping_add(graceperiod) < ts {
                    put32bit(&raw mut buff, 0 as uint32_t);
                } else {
                    put32bit(
                        &raw mut buff,
                        (*qn).stimestamp.wrapping_add(graceperiod).wrapping_sub(ts),
                    );
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SINODES != 0 {
                    put32bit(&raw mut buff, (*qn).sinodes);
                } else {
                    put32bit(&raw mut buff, 0 as uint32_t);
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SLENGTH != 0 {
                    put64bit(&raw mut buff, (*qn).slength);
                } else {
                    put64bit(&raw mut buff, 0 as uint64_t);
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SSIZE != 0 {
                    put64bit(&raw mut buff, (*qn).ssize);
                } else {
                    put64bit(&raw mut buff, 0 as uint64_t);
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_SREALSIZE != 0 {
                    put64bit(&raw mut buff, (*qn).srealsize);
                } else {
                    put64bit(&raw mut buff, 0 as uint64_t);
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HINODES != 0 {
                    put32bit(&raw mut buff, (*qn).hinodes);
                } else {
                    put32bit(&raw mut buff, 0 as uint32_t);
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HLENGTH != 0 {
                    put64bit(&raw mut buff, (*qn).hlength);
                } else {
                    put64bit(&raw mut buff, 0 as uint64_t);
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HSIZE != 0 {
                    put64bit(&raw mut buff, (*qn).hsize);
                } else {
                    put64bit(&raw mut buff, 0 as uint64_t);
                }
                if (*qn).flags as ::core::ffi::c_int & QUOTA_FLAG_HREALSIZE != 0 {
                    put64bit(&raw mut buff, (*qn).hrealsize);
                } else {
                    put64bit(&raw mut buff, 0 as uint64_t);
                }
                put32bit(&raw mut buff, (*psr).inodes);
                put64bit(&raw mut buff, (*psr).length);
                put64bit(&raw mut buff, (*psr).size);
                put64bit(&raw mut buff, (*psr).realsize);
            }
            qn = (*qn).next as *mut quotanode;
        }
        return ts;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_archget(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut archchunks: *mut uint64_t,
    mut notarchchunks: *mut uint64_t,
    mut archinodes: *mut uint32_t,
    mut partinodes: *mut uint32_t,
    mut notarchinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            && (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        fsnodes_keep_alive_begin();
        *archchunks = 0 as uint64_t;
        *notarchchunks = 0 as uint64_t;
        *archinodes = 0 as uint32_t;
        *partinodes = 0 as uint32_t;
        *notarchinodes = 0 as uint32_t;
        fsnodes_getarch_recursive(
            p,
            archchunks,
            notarchchunks,
            archinodes,
            partinodes,
            notarchinodes,
        );
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_archchg(
    mut ts: uint32_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut cmd: uint8_t,
    mut chgchunks: *mut uint64_t,
    mut notchgchunks: *mut uint64_t,
    mut nsinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        *chgchunks = 0 as uint64_t;
        *notchgchunks = 0 as uint64_t;
        *nsinodes = 0 as uint32_t;
        if cmd as ::core::ffi::c_int != ARCHCTL_CLR && cmd as ::core::ffi::c_int != ARCHCTL_SET {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
            return MFS_ERROR_EROFS as uint8_t;
        }
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            && (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        fsnodes_keep_alive_begin();
        fsnodes_chgarch_recursive(p, ts, uid, cmd, chgchunks, notchgchunks, nsinodes);
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            && *nsinodes > 0 as uint32_t
            && *chgchunks == 0 as uint64_t
            && *notchgchunks == 0 as uint64_t
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|ARCHCHG(%u,%u,%hhu):%lu,%lu,%u\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                inode,
                uid,
                cmd as ::core::ffi::c_int,
                *chgchunks,
                *notchgchunks,
                *nsinodes,
            );
        } else {
            meta_version_inc();
        }
        stats_meta = stats_meta.wrapping_add(1);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_archchg(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut cmd: uint8_t,
    mut chgchunks: *mut uint64_t,
    mut notchgchunks: *mut uint64_t,
    mut nsinodes: *mut uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_archchg(
            main_time(),
            rootinode,
            sesflags,
            inode,
            uid,
            cmd,
            chgchunks,
            notchgchunks,
            nsinodes,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_archchg(
    mut ts: uint32_t,
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut cmd: uint8_t,
    mut chgchunks: uint64_t,
    mut notchgchunks: uint64_t,
    mut nsinodes: uint32_t,
) -> uint8_t {
    unsafe {
        let mut cc: uint64_t = 0;
        let mut ncc: uint64_t = 0;
        let mut nsi: uint32_t = 0;
        let mut status: uint8_t = 0;
        status = fs_univ_archchg(
            ts,
            0 as uint32_t,
            SESFLAG_METARESTORE as uint8_t,
            inode,
            uid,
            cmd,
            &raw mut cc,
            &raw mut ncc,
            &raw mut nsi,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            return status;
        }
        if cc != chgchunks || ncc != notchgchunks || nsi != nsinodes {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ARCHCHG data mismatch: my:(%lu,%lu,%u) != expected:(%lu,%lu,%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                cc,
                ncc,
                nsi,
                chgchunks,
                notchgchunks,
                nsinodes,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getdirpath_size(mut inode: uint32_t) -> uint32_t {
    unsafe {
        let mut node: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        node = fsnodes_node_find(inode);
        if !node.is_null() {
            if (*node).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                return 15 as uint32_t;
            } else {
                return (1 as uint32_t).wrapping_add(fsnodes_getpath_size((*node).parents));
            }
        } else {
            return 11 as uint32_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getdirpath_data(
    mut inode: uint32_t,
    mut buff: *mut uint8_t,
    mut size: uint32_t,
) {
    unsafe {
        let mut node: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        node = fsnodes_node_find(inode);
        if !node.is_null() {
            if (*node).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                if size >= 15 as uint32_t {
                    memcpy(
                        buff as *mut ::core::ffi::c_void,
                        b"(not directory)\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        15 as size_t,
                    );
                    return;
                }
            } else if size > 0 as uint32_t {
                *buff.offset(0 as isize) = '/' as uint8_t;
                fsnodes_getpath_data(
                    (*node).parents,
                    buff.offset(1 as ::core::ffi::c_int as isize),
                    size.wrapping_sub(1 as uint32_t),
                );
                return;
            }
        } else if size >= 11 as uint32_t {
            memcpy(
                buff as *mut ::core::ffi::c_void,
                b"(not found)\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                11 as size_t,
            );
            return;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_dir_stats(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut inode: uint32_t,
    mut inodes: *mut uint32_t,
    mut dirs: *mut uint32_t,
    mut files: *mut uint32_t,
    mut chunks: *mut uint32_t,
    mut length: *mut uint64_t,
    mut size: *mut uint64_t,
    mut rsize: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*p).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
            && (*p).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*p).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*p).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        fsnodes_get_stats(p, &raw mut sr, 2 as uint8_t);
        *inodes = sr.inodes;
        *dirs = sr.dirs;
        *files = sr.files;
        *chunks = sr.chunks;
        *length = sr.length;
        *size = sr.size;
        *rsize = sr.realsize;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_node_info(
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut eights_mode: uint8_t,
    mut inode: uint32_t,
    mut maxentries: uint32_t,
    mut continueid: uint64_t,
    mut ptr: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut i: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut lastchunk: uint32_t = 0;
        let mut lastchunksize: uint32_t = 0;
        let mut eights_copies: uint8_t = 0;
        let mut nextcidptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut ncontid: uint64_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut ret: uint32_t = 0 as uint32_t;
        if fsnodes_node_find_ext(
            rootinode,
            sesflags,
            &raw mut inode,
            ::core::ptr::null_mut::<*mut fsnode>(),
            &raw mut p,
            0 as uint8_t,
        ) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            ret = 1 as uint32_t;
            if !ptr.is_null() {
                put8bit(&raw mut ptr, MFS_ERROR_ENOENT as uint8_t);
            }
            return ret;
        }
        if (*p).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
            ret = 10 as uint32_t;
            if !ptr.is_null() {
                put16bit(&raw mut ptr, 1 as uint16_t);
                nextcidptr = ptr;
                ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
            } else {
                nextcidptr = ::core::ptr::null_mut::<uint8_t>();
            }
            ncontid = continueid;
            if continueid == 0 as uint64_t {
                e = (*p).data.ddata.children;
            } else {
                e = fsnodes_edgeid_find(continueid);
            }
            if e.is_null() {
                e = (*p).data.ddata.children;
                while !e.is_null() && (*e).edgeid < continueid {
                    e = (*e).nextchild as *mut fsedge;
                }
                if !e.is_null() {
                    ncontid = (*e).edgeid;
                } else {
                    ncontid = 0 as uint64_t;
                }
            }
            while !e.is_null() && maxentries > 0 as uint32_t {
                if !ptr.is_null() {
                    put32bit(&raw mut ptr, (*(*e).child).inode);
                }
                ret = ret.wrapping_add(4 as uint32_t);
                maxentries = maxentries.wrapping_sub(1);
                e = (*e).nextchild as *mut fsedge;
                if !e.is_null() {
                    ncontid = (*e).edgeid;
                } else {
                    ncontid = 0 as uint64_t;
                }
            }
            if !e.is_null() {
                fsnodes_edgeid_insert(e);
            }
            if !nextcidptr.is_null() {
                put64bit(&raw mut nextcidptr, ncontid);
            }
        } else if (*p).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            ret = 18 as uint32_t;
            if !ptr.is_null() {
                if eights_mode != 0 {
                    put16bit(&raw mut ptr, 3 as uint16_t);
                } else {
                    put16bit(&raw mut ptr, 2 as uint16_t);
                }
                nextcidptr = ptr;
                ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
                put64bit(&raw mut ptr, (*p).data.fdata.length);
            } else {
                nextcidptr = ::core::ptr::null_mut::<uint8_t>();
            }
            if (*p).data.fdata.length > 0 as uint64_t {
                lastchunk = ((*p).data.fdata.length.wrapping_sub(1 as uint64_t) >> MFSCHUNKBITS)
                    as uint32_t;
                lastchunksize = (((*p).data.fdata.length.wrapping_sub(1 as uint64_t)
                    & MFSCHUNKMASK as uint64_t)
                    .wrapping_add(MFSBLOCKSIZE as uint64_t)
                    & MFSBLOCKNEGMASK as uint64_t)
                    .wrapping_add(MFSHDRSIZE as uint64_t)
                    as uint32_t;
            } else {
                lastchunk = 0 as uint32_t;
                lastchunksize = MFSHDRSIZE as uint32_t;
            }
            ncontid = continueid;
            i = continueid as uint32_t;
            while i < (*p).data.fdata.chunks && maxentries > 0 as uint32_t {
                chunkid = *(*p).data.fdata.chunktab.offset(i as isize);
                if chunkid > 0 as uint64_t
                    && chunk_get_eights_copies(chunkid, &raw mut eights_copies)
                        as ::core::ffi::c_int
                        == MFS_STATUS_OK
                {
                    if !ptr.is_null() {
                        put64bit(&raw mut ptr, chunkid);
                        if i < lastchunk {
                            put32bit(&raw mut ptr, (MFSCHUNKSIZE + MFSHDRSIZE) as uint32_t);
                        } else if i == lastchunk {
                            put32bit(&raw mut ptr, lastchunksize);
                        } else {
                            put32bit(&raw mut ptr, 0 as uint32_t);
                        }
                        if eights_mode != 0 {
                            put8bit(&raw mut ptr, eights_copies);
                        } else {
                            put8bit(
                                &raw mut ptr,
                                (eights_copies as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
                                    as uint8_t,
                            );
                        }
                    }
                    ret = ret.wrapping_add(13 as uint32_t);
                    ncontid = i.wrapping_add(1 as uint32_t) as uint64_t;
                    maxentries = maxentries.wrapping_sub(1);
                }
                i = i.wrapping_add(1);
            }
            if ncontid >= (*p).data.fdata.chunks as uint64_t {
                ncontid = 0 as uint64_t;
            }
            if !nextcidptr.is_null() {
                put64bit(&raw mut nextcidptr, ncontid);
            }
        } else {
            ret = 2 as uint32_t;
            if !ptr.is_null() {
                put16bit(&raw mut ptr, 0 as uint16_t);
            }
        }
        return ret;
    }
}
#[inline]
unsafe extern "C" fn fs_add_file_to_chunks(mut f: *mut fsnode) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        if (*f).r#type() as ::core::ffi::c_int == TYPE_FILE
            || (*f).r#type() as ::core::ffi::c_int == TYPE_TRASH
            || (*f).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            i = 0 as uint32_t;
            while i < (*f).data.fdata.chunks {
                chunkid = *(*f).data.fdata.chunktab.offset(i as isize);
                if chunkid > 0 as uint64_t {
                    chunk_add_file(chunkid, (*f).sclassid);
                }
                i = i.wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_add_files_to_chunks() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        i = 0 as uint32_t;
        while i < noderehashpos {
            p = *nodehashtab[(i >> HASHTAB_LOBITS) as usize]
                .offset((i & HASHTAB_MASK as uint32_t) as isize);
            while !p.is_null() {
                fs_add_file_to_chunks(p);
                p = (*p).next as *mut fsnode;
            }
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_set_file_chunk(
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut chunkid: uint64_t,
) -> uint8_t {
    unsafe {
        let mut node: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut oldchunkid: uint64_t = 0;
        node = fsnodes_node_find(inode);
        if node.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if indx >= (*node).data.fdata.chunks {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"set_file_chunk index too big: indx:%u ; chunks:%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                indx,
                (*node).data.fdata.chunks,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        oldchunkid = *(*node).data.fdata.chunktab.offset(indx as isize);
        if oldchunkid > 0 as uint64_t {
            chunk_delete_file(oldchunkid, (*node).sclassid);
        }
        *(*node).data.fdata.chunktab.offset(indx as isize) = chunkid;
        if chunkid > 0 as uint64_t {
            chunk_add_file(chunkid, (*node).sclassid);
        }
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_autoarch(
    mut inode: uint32_t,
    mut archreftime: uint32_t,
    mut intrash: uint8_t,
    mut archchgchunks: uint32_t,
    mut trashchgchunks: uint32_t,
) -> uint8_t {
    unsafe {
        let mut node: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut aflagschanged: uint32_t = 0;
        let mut tflagschanged: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        node = fsnodes_node_find(inode);
        if node.is_null() {
            return MFS_ERROR_ENOENT as uint8_t;
        }
        if (*node).r#type() as ::core::ffi::c_int != TYPE_FILE
            && (*node).r#type() as ::core::ffi::c_int != TYPE_TRASH
            && (*node).r#type() as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            return MFS_ERROR_EPERM as uint8_t;
        }
        aflagschanged = 0 as uint32_t;
        tflagschanged = 0 as uint32_t;
        j = 0 as uint32_t;
        while j < (*node).data.fdata.chunks {
            chunkid = *(*node).data.fdata.chunktab.offset(j as isize);
            if chunkid > 0 as uint64_t {
                status = chunk_set_autoarch(
                    chunkid,
                    archreftime,
                    &raw mut aflagschanged,
                    intrash,
                    &raw mut tflagschanged,
                );
                if status != MFS_STATUS_OK {
                    return status as uint8_t;
                }
            }
            j = j.wrapping_add(1);
        }
        if aflagschanged != archchgchunks || tflagschanged != trashchgchunks {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"auto_arch wrong number of changed chunks: changed:(arch:%u,trash:%u) ; expected:(arch:%u,trash:%u)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                aflagschanged,
                tflagschanged,
                archchgchunks,
                trashchgchunks,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_test_getdata(
    mut loopstart: *mut uint32_t,
    mut loopend: *mut uint32_t,
    mut files: *mut uint32_t,
    mut ugfiles: *mut uint32_t,
    mut mfiles: *mut uint32_t,
    mut mtfiles: *mut uint32_t,
    mut msfiles: *mut uint32_t,
    mut chunks: *mut uint32_t,
    mut ugchunks: *mut uint32_t,
    mut mchunks: *mut uint32_t,
    mut msgbuff: *mut *mut ::core::ffi::c_char,
    mut msgbuffleng: *mut uint32_t,
) {
    unsafe {
        *loopstart = fsinfo_loopstart;
        *loopend = fsinfo_loopend;
        *files = fsinfo_files;
        *ugfiles = fsinfo_ugfiles;
        *mfiles = fsinfo_mfiles;
        *mtfiles = fsinfo_mtfiles;
        *msfiles = fsinfo_msfiles;
        *chunks = fsinfo_chunks;
        *ugchunks = fsinfo_ugchunks;
        *mchunks = fsinfo_mchunks;
        *msgbuff = fsinfo_msgbuff;
        *msgbuffleng = fsinfo_msgbuffleng;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_test_log_inconsistency(
    mut e: *mut fsedge,
    mut iname: *const ::core::ffi::c_char,
    mut buff: *mut ::core::ffi::c_char,
    mut size: uint32_t,
) -> uint32_t {
    unsafe {
        let mut leng: uint32_t = 0;
        leng = 0 as uint32_t;
        if !(*e).parent.is_null() {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"structure error - %s inconsistency (edge: %u,%s -> %u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iname,
                (*(*e).parent).inode,
                changelog_escape_name(
                    (*e).nleng as uint32_t,
                    &raw const (*e).name as *const uint8_t,
                ),
                (*(*e).child).inode,
            );
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"structure error - %s inconsistency (edge: %u,%s -> %u)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    iname,
                    (*(*e).parent).inode,
                    changelog_escape_name(
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                    ),
                    (*(*e).child).inode,
                ) as uint32_t);
            }
        } else if (*(*e).child).r#type() as ::core::ffi::c_int == TYPE_TRASH {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"structure error - %s inconsistency (edge: TRASH,%s -> %u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iname,
                changelog_escape_name(
                    (*e).nleng as uint32_t,
                    &raw const (*e).name as *const uint8_t,
                ),
                (*(*e).child).inode,
            );
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"structure error - %s inconsistency (edge: TRASH,%s -> %u)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    iname,
                    changelog_escape_name(
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                    ),
                    (*(*e).child).inode,
                ) as uint32_t);
            }
        } else if (*(*e).child).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"structure error - %s inconsistency (edge: SUSTAINED,%s -> %u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iname,
                changelog_escape_name(
                    (*e).nleng as uint32_t,
                    &raw const (*e).name as *const uint8_t,
                ),
                (*(*e).child).inode,
            );
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"structure error - %s inconsistency (edge: SUSTAINED,%s -> %u)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    iname,
                    changelog_escape_name(
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                    ),
                    (*(*e).child).inode,
                ) as uint32_t);
            }
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"structure error - %s inconsistency (edge: NULL,%s -> %u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                iname,
                changelog_escape_name(
                    (*e).nleng as uint32_t,
                    &raw const (*e).name as *const uint8_t,
                ),
                (*(*e).child).inode,
            );
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buff.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"structure error - %s inconsistency (edge: NULL,%s -> %u)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    iname,
                    changelog_escape_name(
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                    ),
                    (*(*e).child).inode,
                ) as uint32_t);
            }
        }
        return leng;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_test_files() {
    unsafe {
        static mut i: uint32_t = 0 as uint32_t;
        let mut j: uint32_t = 0;
        let mut k: uint32_t = 0;
        let mut lengchunks: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut valid: uint8_t = 0;
        let mut ugflag: uint8_t = 0;
        let mut aflagchanged: uint32_t = 0;
        let mut tflagchanged: uint32_t = 0;
        let mut allchunks: uint32_t = 0;
        let mut arch_mode: uint8_t = 0;
        let mut arch_delay: uint16_t = 0;
        let mut arch_delay_sec: uint32_t = 0;
        let mut arch_min_size: uint64_t = 0;
        let mut archreftime: uint32_t = 0;
        static mut files: uint32_t = 0 as uint32_t;
        static mut ugfiles: uint32_t = 0 as uint32_t;
        static mut mfiles: uint32_t = 0 as uint32_t;
        static mut mtfiles: uint32_t = 0 as uint32_t;
        static mut msfiles: uint32_t = 0 as uint32_t;
        static mut chunks: uint32_t = 0 as uint32_t;
        static mut ugchunks: uint32_t = 0 as uint32_t;
        static mut mchunks: uint32_t = 0 as uint32_t;
        static mut notfoundchunks: uint32_t = 0 as uint32_t;
        static mut msgbuff: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        static mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        static mut leng: uint32_t = 0 as uint32_t;
        let mut now: uint32_t = 0;
        let mut f: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        now = main_time();
        if chunk_counters_in_progress() != 0 {
            return;
        }
        if i == 0 as uint32_t {
            if notfoundchunks > 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"unknown chunks: %u\0".as_ptr() as *const ::core::ffi::c_char,
                    notfoundchunks,
                );
                if leng < MSGBUFFSIZE as uint32_t {
                    leng = leng.wrapping_add(snprintf(
                        msgbuff.offset(leng as isize),
                        (MSGBUFFSIZE as uint32_t).wrapping_sub(leng) as size_t,
                        b"unknown chunks: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                        notfoundchunks,
                    ) as uint32_t);
                }
                notfoundchunks = 0 as uint32_t;
            }
            fsinfo_files = files;
            fsinfo_ugfiles = ugfiles;
            fsinfo_mfiles = mfiles;
            fsinfo_mtfiles = mtfiles;
            fsinfo_msfiles = msfiles;
            fsinfo_chunks = chunks;
            fsinfo_ugchunks = ugchunks;
            fsinfo_mchunks = mchunks;
            files = 0 as uint32_t;
            ugfiles = 0 as uint32_t;
            mfiles = 0 as uint32_t;
            mtfiles = 0 as uint32_t;
            msfiles = 0 as uint32_t;
            chunks = 0 as uint32_t;
            ugchunks = 0 as uint32_t;
            mchunks = 0 as uint32_t;
            missing_log_swap();
            if fsinfo_msgbuff.is_null() {
                fsinfo_msgbuff = malloc(MSGBUFFSIZE as size_t) as *mut ::core::ffi::c_char;
                if fsinfo_msgbuff.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        8992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"fsinfo_msgbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        8992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"fsinfo_msgbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if fsinfo_msgbuff
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut ::core::ffi::c_char
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        8992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"fsinfo_msgbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        8992 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"fsinfo_msgbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            tmp = fsinfo_msgbuff;
            fsinfo_msgbuff = msgbuff;
            msgbuff = tmp;
            if leng > MSGBUFFSIZE as uint32_t {
                fsinfo_msgbuffleng = MSGBUFFSIZE as uint32_t;
            } else {
                fsinfo_msgbuffleng = leng;
            }
            leng = 0 as uint32_t;
            fsinfo_loopstart = fsinfo_loopend;
            fsinfo_loopend = now;
        }
        k = 0 as uint32_t;
        while k < nodehashsize.wrapping_div(32768 as uint32_t) && i < noderehashpos {
            f = *nodehashtab[(i >> HASHTAB_LOBITS) as usize]
                .offset((i & HASHTAB_MASK as uint32_t) as isize);
            while !f.is_null() {
                if (*f).r#type() as ::core::ffi::c_int == TYPE_FILE
                    || (*f).r#type() as ::core::ffi::c_int == TYPE_TRASH
                    || (*f).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
                {
                    valid = 1 as uint8_t;
                    ugflag = 0 as uint8_t;
                    aflagchanged = 0 as uint32_t;
                    tflagchanged = 0 as uint32_t;
                    allchunks = 0 as uint32_t;
                    arch_mode = sclass_get_arch_mode((*f).sclassid as uint16_t);
                    arch_delay = sclass_get_arch_delay((*f).sclassid as uint16_t);
                    arch_min_size = sclass_get_arch_min_size((*f).sclassid as uint16_t);
                    if (*f).r#type() as ::core::ffi::c_int != TYPE_FILE {
                        archreftime = ARCHREFTIME_NEVER as uint32_t;
                    } else if (*f).data.fdata.length < arch_min_size {
                        archreftime = ARCHREFTIME_NEVER as uint32_t;
                    } else if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_CHUNK != 0 {
                        if arch_delay as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            archreftime = ARCHREFTIME_NEVER as uint32_t;
                        } else {
                            arch_delay_sec =
                                (arch_delay as uint32_t).wrapping_mul(3600 as uint32_t);
                            if arch_delay_sec > now {
                                archreftime = ARCHREFTIME_NEVER as uint32_t;
                            } else {
                                archreftime = now.wrapping_sub(arch_delay_sec);
                            }
                        }
                    } else if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_FAST != 0 {
                        archreftime = 0xffffffff as ::core::ffi::c_uint as uint32_t;
                    } else if arch_delay as ::core::ffi::c_uint == 0 as ::core::ffi::c_uint {
                        archreftime = ARCHREFTIME_NEVER as uint32_t;
                    } else {
                        let mut arch_chk_time: uint32_t = 0 as uint32_t;
                        let mut reftime: uint32_t = 0 as uint32_t;
                        if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_CTIME != 0 {
                            if (*f).ctime > reftime {
                                reftime = (*f).ctime;
                            }
                        }
                        if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_MTIME != 0 {
                            if (*f).mtime > reftime {
                                reftime = (*f).mtime;
                            }
                        }
                        if arch_mode as ::core::ffi::c_int & SCLASS_ARCH_MODE_ATIME != 0 {
                            if (*f).atime > reftime {
                                reftime = (*f).atime;
                            }
                        }
                        arch_delay_sec = (arch_delay as uint32_t).wrapping_mul(3600 as uint32_t);
                        arch_chk_time = reftime.wrapping_add(arch_delay_sec);
                        archreftime = (if arch_chk_time < now && arch_chk_time >= reftime {
                            0xffffffff as ::core::ffi::c_uint
                        } else {
                            ARCHREFTIME_NEVER as ::core::ffi::c_uint
                        }) as uint32_t;
                    }
                    lengchunks = ((*f)
                        .data
                        .fdata
                        .length
                        .wrapping_add(MFSCHUNKMASK as uint64_t)
                        >> MFSCHUNKBITS) as uint32_t;
                    j = 0 as uint32_t;
                    while j < (*f).data.fdata.chunks {
                        chunkid = *(*f).data.fdata.chunktab.offset(j as isize);
                        if chunkid > 0 as uint64_t {
                            allchunks = allchunks.wrapping_add(1);
                            match chunk_fileloop_task(
                                chunkid,
                                (*f).sclassid,
                                (if j >= lengchunks {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as uint8_t,
                                archreftime,
                                &raw mut aflagchanged,
                                (if (*f).r#type() as ::core::ffi::c_int == TYPE_TRASH {
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) as uint8_t,
                                &raw mut tflagchanged,
                            ) as ::core::ffi::c_uint
                            {
                                0 => {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"structure error - chunk %016lX not found (inode: %u ; index: %u)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        chunkid,
                                        (*f).inode,
                                        j,
                                    );
                                    if leng < MSGBUFFSIZE as uint32_t {
                                        leng = leng
                                            .wrapping_add(
                                                snprintf(
                                                    msgbuff.offset(leng as isize),
                                                    (MSGBUFFSIZE as uint32_t).wrapping_sub(leng) as size_t,
                                                    b"structure error - chunk %016lX not found (inode: %u ; index: %u)\n\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                    chunkid,
                                                    (*f).inode,
                                                    j,
                                                ) as uint32_t,
                                            );
                                    }
                                    notfoundchunks = notfoundchunks.wrapping_add(1);
                                    if notfoundchunks.wrapping_rem(1000 as uint32_t)
                                        == 0 as uint32_t
                                    {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"unknown chunks: %u ...\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            notfoundchunks,
                                        );
                                    }
                                    valid = 0 as uint8_t;
                                    mchunks = mchunks.wrapping_add(1);
                                }
                                1 => {
                                    *(*f).data.fdata.chunktab.offset(j as isize) = 0 as uint64_t;
                                    allchunks = allchunks.wrapping_sub(1);
                                    changelog(
                                        b"%u|SETFILECHUNK(%u,%u,0)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        main_time(),
                                        (*f).inode,
                                        j,
                                    );
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_NOTICE,
                                        b"inode: %u ; index: %u - removed not existing chunk exceeding file size (chunkid: %016lX)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        (*f).inode,
                                        j,
                                        chunkid,
                                    );
                                }
                                2 => {
                                    missing_log_insert(
                                        chunkid,
                                        (*f).inode,
                                        j,
                                        MISSING_CHUNK_TYPE_NOCOPY as uint8_t,
                                    );
                                    valid = 0 as uint8_t;
                                    mchunks = mchunks.wrapping_add(1);
                                }
                                3 => {
                                    missing_log_insert(
                                        chunkid,
                                        (*f).inode,
                                        j,
                                        MISSING_CHUNK_TYPE_INVALID as uint8_t,
                                    );
                                    valid = 0 as uint8_t;
                                    mchunks = mchunks.wrapping_add(1);
                                }
                                4 => {
                                    missing_log_insert(
                                        chunkid,
                                        (*f).inode,
                                        j,
                                        MISSING_CHUNK_TYPE_WRONGVERSION as uint8_t,
                                    );
                                    valid = 0 as uint8_t;
                                    mchunks = mchunks.wrapping_add(1);
                                }
                                5 => {
                                    missing_log_insert(
                                        chunkid,
                                        (*f).inode,
                                        j,
                                        MISSING_CHUNK_TYPE_PARTIALEC as uint8_t,
                                    );
                                    valid = 0 as uint8_t;
                                    mchunks = mchunks.wrapping_add(1);
                                }
                                6 => {
                                    ugflag = 1 as uint8_t;
                                    ugchunks = ugchunks.wrapping_add(1);
                                }
                                7 | _ => {}
                            }
                        }
                        j = j.wrapping_add(1);
                    }
                    if aflagchanged != 0 || tflagchanged != 0 {
                        changelog(
                            b"%u|AUTOARCH(%u,%u,%u):%u,%u\0".as_ptr() as *const ::core::ffi::c_char,
                            now,
                            (*f).inode,
                            archreftime,
                            if (*f).r#type() as ::core::ffi::c_int == TYPE_TRASH {
                                1 as ::core::ffi::c_uint
                            } else {
                                0 as ::core::ffi::c_uint
                            },
                            aflagchanged,
                            tflagchanged,
                        );
                        if aflagchanged != 0 {
                            (*f).set_keepmode(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        }
                    }
                    if valid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        if (*f).r#type() as ::core::ffi::c_int == TYPE_TRASH {
                            mtfiles = mtfiles.wrapping_add(1);
                        } else if (*f).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
                            msfiles = msfiles.wrapping_add(1);
                        } else {
                            e = (*f).parents;
                            while !e.is_null() {
                                mfiles = mfiles.wrapping_add(1);
                                e = (*e).nextparent as *mut fsedge;
                            }
                        }
                    } else if ugflag != 0 {
                        ugfiles = ugfiles.wrapping_add(1);
                    }
                    files = files.wrapping_add(1);
                    chunks = chunks.wrapping_add(allchunks);
                    fsnodes_check_realsize(f);
                }
                e = (*f).parents;
                while !e.is_null() {
                    if (*e).child != f {
                        if !(*e).parent.is_null() {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"structure error - edge->child/child->edges (node: %u ; edge: %u,%s -> %u)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                (*f).inode,
                                (*(*e).parent).inode,
                                changelog_escape_name(
                                    (*e).nleng as uint32_t,
                                    &raw const (*e).name as *const uint8_t,
                                ),
                                (*(*e).child).inode,
                            );
                            if leng < MSGBUFFSIZE as uint32_t {
                                leng = leng
                                    .wrapping_add(
                                        snprintf(
                                            msgbuff.offset(leng as isize),
                                            (MSGBUFFSIZE as uint32_t).wrapping_sub(leng) as size_t,
                                            b"structure error - edge->child/child->edges (node: %u ; edge: %u,%s -> %u)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            (*f).inode,
                                            (*(*e).parent).inode,
                                            changelog_escape_name(
                                                (*e).nleng as uint32_t,
                                                &raw const (*e).name as *const uint8_t,
                                            ),
                                            (*(*e).child).inode,
                                        ) as uint32_t,
                                    );
                            }
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"structure error - edge->child/child->edges (node: %u ; edge: NULL,%s -> %u)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                (*f).inode,
                                changelog_escape_name(
                                    (*e).nleng as uint32_t,
                                    &raw const (*e).name as *const uint8_t,
                                ),
                                (*(*e).child).inode,
                            );
                            if leng < MSGBUFFSIZE as uint32_t {
                                leng = leng
                                    .wrapping_add(
                                        snprintf(
                                            msgbuff.offset(leng as isize),
                                            (MSGBUFFSIZE as uint32_t).wrapping_sub(leng) as size_t,
                                            b"structure error - edge->child/child->edges (node: %u ; edge: NULL,%s -> %u)\n\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            (*f).inode,
                                            changelog_escape_name(
                                                (*e).nleng as uint32_t,
                                                &raw const (*e).name as *const uint8_t,
                                            ),
                                            (*(*e).child).inode,
                                        ) as uint32_t,
                                    );
                            }
                        }
                    } else if !(*e).nextchild.is_null() {
                        if (*(*e).nextchild).prevchild != &raw mut (*e).nextchild {
                            if leng < MSGBUFFSIZE as uint32_t {
                                leng = leng.wrapping_add(fs_test_log_inconsistency(
                                    e,
                                    b"nextchild/prevchild\0".as_ptr() as *const ::core::ffi::c_char,
                                    msgbuff.offset(leng as isize),
                                    (MSGBUFFSIZE as uint32_t).wrapping_sub(leng),
                                ));
                            } else {
                                fs_test_log_inconsistency(
                                    e,
                                    b"nextchild/prevchild\0".as_ptr() as *const ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    0 as uint32_t,
                                );
                            }
                        }
                    } else if !(*e).nextparent.is_null() {
                        if (*(*e).nextparent).prevparent != &raw mut (*e).nextparent {
                            if leng < MSGBUFFSIZE as uint32_t {
                                leng = leng.wrapping_add(fs_test_log_inconsistency(
                                    e,
                                    b"nextparent/prevparent\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    msgbuff.offset(leng as isize),
                                    (MSGBUFFSIZE as uint32_t).wrapping_sub(leng),
                                ));
                            } else {
                                fs_test_log_inconsistency(
                                    e,
                                    b"nextparent/prevparent\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    0 as uint32_t,
                                );
                            }
                        }
                    }
                    e = (*e).nextparent as *mut fsedge;
                }
                if (*f).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                    e = (*f).data.ddata.children;
                    while !e.is_null() {
                        if (*e).parent != f {
                            if !(*e).parent.is_null() {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"structure error - edge->parent/parent->edges (node: %u ; edge: %u,%s -> %u)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*f).inode,
                                    (*(*e).parent).inode,
                                    changelog_escape_name(
                                        (*e).nleng as uint32_t,
                                        &raw const (*e).name as *const uint8_t,
                                    ),
                                    (*(*e).child).inode,
                                );
                                if leng < MSGBUFFSIZE as uint32_t {
                                    leng = leng
                                        .wrapping_add(
                                            snprintf(
                                                msgbuff.offset(leng as isize),
                                                (MSGBUFFSIZE as uint32_t).wrapping_sub(leng) as size_t,
                                                b"structure error - edge->parent/parent->edges (node: %u ; edge: %u,%s -> %u)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                (*f).inode,
                                                (*(*e).parent).inode,
                                                changelog_escape_name(
                                                    (*e).nleng as uint32_t,
                                                    &raw const (*e).name as *const uint8_t,
                                                ),
                                                (*(*e).child).inode,
                                            ) as uint32_t,
                                        );
                                }
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"structure error - edge->parent/parent->edges (node: %u ; edge: NULL,%s -> %u)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*f).inode,
                                    changelog_escape_name(
                                        (*e).nleng as uint32_t,
                                        &raw const (*e).name as *const uint8_t,
                                    ),
                                    (*(*e).child).inode,
                                );
                                if leng < MSGBUFFSIZE as uint32_t {
                                    leng = leng
                                        .wrapping_add(
                                            snprintf(
                                                msgbuff.offset(leng as isize),
                                                (MSGBUFFSIZE as uint32_t).wrapping_sub(leng) as size_t,
                                                b"structure error - edge->parent/parent->edges (node: %u ; edge: NULL,%s -> %u)\n\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                (*f).inode,
                                                changelog_escape_name(
                                                    (*e).nleng as uint32_t,
                                                    &raw const (*e).name as *const uint8_t,
                                                ),
                                                (*(*e).child).inode,
                                            ) as uint32_t,
                                        );
                                }
                            }
                        } else if !(*e).nextchild.is_null() {
                            if (*(*e).nextchild).prevchild != &raw mut (*e).nextchild {
                                if leng < MSGBUFFSIZE as uint32_t {
                                    leng = leng.wrapping_add(fs_test_log_inconsistency(
                                        e,
                                        b"nextchild/prevchild\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        msgbuff.offset(leng as isize),
                                        (MSGBUFFSIZE as uint32_t).wrapping_sub(leng),
                                    ));
                                } else {
                                    fs_test_log_inconsistency(
                                        e,
                                        b"nextchild/prevchild\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                        0 as uint32_t,
                                    );
                                }
                            }
                        } else if !(*e).nextparent.is_null() {
                            if (*(*e).nextparent).prevparent != &raw mut (*e).nextparent {
                                if leng < MSGBUFFSIZE as uint32_t {
                                    leng = leng.wrapping_add(fs_test_log_inconsistency(
                                        e,
                                        b"nextparent/prevparent\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        msgbuff.offset(leng as isize),
                                        (MSGBUFFSIZE as uint32_t).wrapping_sub(leng),
                                    ));
                                } else {
                                    fs_test_log_inconsistency(
                                        e,
                                        b"nextparent/prevparent\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                        0 as uint32_t,
                                    );
                                }
                            }
                        }
                        e = (*e).nextchild as *mut fsedge;
                    }
                }
                f = (*f).next as *mut fsnode;
            }
            k = k.wrapping_add(1);
            i = i.wrapping_add(1);
        }
        if i >= noderehashpos {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"structure check loop\0".as_ptr() as *const ::core::ffi::c_char,
            );
            i = 0 as uint32_t;
        }
    }
}
#[inline]
unsafe extern "C" fn fs_univ_empty_trash_part(
    mut ts: uint32_t,
    mut bid: uint32_t,
    mut fi: *mut uint32_t,
    mut si: *mut uint32_t,
    mut tfi: *mut uint32_t,
) -> uint32_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut trashseconds: uint64_t = 0;
        let mut ics: uint32_t = 0;
        ics = 0 as uint32_t;
        e = trash[bid as usize];
        while !e.is_null() {
            p = (*e).child as *mut fsnode;
            e = (*e).nextchild as *mut fsedge;
            trashseconds = (*p).trashretention as uint64_t;
            trashseconds = trashseconds.wrapping_mul(3600 as uint64_t);
            if ((*p).atime as uint64_t).wrapping_add(trashseconds) < ts as uint64_t
                && ((*p).mtime as uint64_t).wrapping_add(trashseconds) < ts as uint64_t
                && ((*p).ctime as uint64_t).wrapping_add(trashseconds) < ts as uint64_t
            {
                ics ^= (*p).inode;
                if fsnodes_purge(ts, p) != 0 {
                    *fi = (*fi).wrapping_add(1);
                } else {
                    *si = (*si).wrapping_add(1);
                }
            }
        }
        e = trash[bid as usize];
        while !e.is_null() {
            p = (*e).child as *mut fsnode;
            e = (*e).nextchild as *mut fsedge;
            if (*p).mtime.wrapping_add(300 as uint32_t) < ts
                && (*p).trashretention as ::core::ffi::c_int
                    >= sclass_get_min_trashretention((*p).sclassid as uint16_t)
                        as ::core::ffi::c_int
            {
                if fsnodes_settrashflag(p, 1 as uint8_t) != 0 {
                    *tfi = (*tfi).wrapping_add(1);
                }
            }
        }
        return ics;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_emptytrash(
    mut ts: uint32_t,
    mut sesflags: uint8_t,
    mut bid: uint32_t,
    mut freeinodes: uint32_t,
    mut sustainedinodes: uint32_t,
    mut trashflaginodes: uint32_t,
    mut inode_chksum: uint32_t,
) -> uint8_t {
    unsafe {
        let mut fi: uint32_t = 0;
        let mut si: uint32_t = 0;
        let mut tfi: uint32_t = 0;
        let mut ics: uint32_t = 0;
        fi = 0 as uint32_t;
        si = 0 as uint32_t;
        tfi = 0 as uint32_t;
        ics = 0 as uint32_t;
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            bid = trash_bid;
            trash_bid = trash_bid.wrapping_add(1);
            if trash_bid >= TRASH_BUCKETS as uint32_t {
                trash_bid = 0 as uint32_t;
            }
        }
        if bid >= TRASH_BUCKETS as uint32_t {
            bid = 0 as uint32_t;
            while bid < TRASH_BUCKETS as uint32_t {
                ics ^= fs_univ_empty_trash_part(ts, bid, &raw mut fi, &raw mut si, &raw mut tfi);
                bid = bid.wrapping_add(1);
            }
        } else {
            ics ^= fs_univ_empty_trash_part(ts, bid, &raw mut fi, &raw mut si, &raw mut tfi);
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if fi | si | tfi > 0 as uint32_t {
                changelog(
                    b"%u|EMPTYTRASH(%u):%u,%u,%u,%u\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    bid,
                    fi,
                    si,
                    tfi,
                    ics,
                );
            }
        } else {
            if freeinodes != fi
                || sustainedinodes != si
                || trashflaginodes != 0xffffffff as uint32_t && trashflaginodes != tfi
                || inode_chksum != 0 as uint32_t && ics != inode_chksum
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"EMPTYTRASH data mismatch: my:(%u,%u,%u,%u) != expected:(%u,%u,%u,%u)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    fi,
                    si,
                    tfi,
                    ics,
                    freeinodes,
                    sustainedinodes,
                    trashflaginodes,
                    inode_chksum,
                );
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_emptytrash() {
    unsafe {
        fs_univ_emptytrash(
            main_time(),
            0 as uint8_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_emptytrash(
    mut ts: uint32_t,
    mut bid: uint32_t,
    mut freeinodes: uint32_t,
    mut sustainedinodes: uint32_t,
    mut trashflaginodes: uint32_t,
    mut inode_chksum: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_emptytrash(
            ts,
            SESFLAG_METARESTORE as uint8_t,
            bid,
            freeinodes,
            sustainedinodes,
            trashflaginodes,
            inode_chksum,
        );
    }
}
#[inline]
unsafe extern "C" fn fs_univ_empty_sustained_part(
    mut ts: uint32_t,
    mut bid: uint32_t,
    mut fi: *mut uint32_t,
) -> uint32_t {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut ics: uint32_t = 0;
        ics = 0 as uint32_t;
        e = sustained[bid as usize];
        while !e.is_null() {
            p = (*e).child as *mut fsnode;
            e = (*e).nextchild as *mut fsedge;
            if of_isfileopen((*p).inode) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                ics ^= (*p).inode;
                fsnodes_purge(ts, p);
                *fi = (*fi).wrapping_add(1);
            }
        }
        return ics;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_univ_emptysustained(
    mut ts: uint32_t,
    mut sesflags: uint8_t,
    mut bid: uint32_t,
    mut freeinodes: uint32_t,
    mut inode_chksum: uint32_t,
) -> uint8_t {
    unsafe {
        let mut fi: uint32_t = 0;
        let mut ics: uint32_t = 0;
        fi = 0 as uint32_t;
        ics = 0 as uint32_t;
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            bid = sustained_bid;
            sustained_bid = sustained_bid.wrapping_add(1);
            if sustained_bid >= SUSTAINED_BUCKETS as uint32_t {
                sustained_bid = 0 as uint32_t;
            }
        }
        if bid >= SUSTAINED_BUCKETS as uint32_t {
            bid = 0 as uint32_t;
            while bid < SUSTAINED_BUCKETS as uint32_t {
                ics ^= fs_univ_empty_sustained_part(ts, bid, &raw mut fi);
                bid = bid.wrapping_add(1);
            }
        } else {
            ics ^= fs_univ_empty_sustained_part(ts, bid, &raw mut fi);
        }
        if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
            if fi > 0 as uint32_t {
                changelog(
                    b"%u|EMPTYSUSTAINED(%u):%u,%u\0".as_ptr() as *const ::core::ffi::c_char,
                    ts,
                    bid,
                    fi,
                    ics,
                );
            }
        } else {
            if freeinodes != fi || inode_chksum != 0 as uint32_t && inode_chksum != ics {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"EMPTYSUSTAINED data mismatch: my:(%u,%u) != expected:(%u,%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    fi,
                    ics,
                    freeinodes,
                    inode_chksum,
                );
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_emptysustained() {
    unsafe {
        fs_univ_emptysustained(
            main_time(),
            0 as uint8_t,
            0 as uint32_t,
            0 as uint32_t,
            0 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_emptysustained(
    mut ts: uint32_t,
    mut bid: uint32_t,
    mut freeinodes: uint32_t,
    mut inode_chksum: uint32_t,
) -> uint8_t {
    unsafe {
        return fs_univ_emptysustained(
            ts,
            SESFLAG_METARESTORE as uint8_t,
            bid,
            freeinodes,
            inode_chksum,
        );
    }
}
#[inline]
unsafe extern "C" fn fs_renumerate_edges(mut p: *mut fsnode) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut fedgeid: uint64_t = 0;
        fedgeid = nextedgeid;
        e = (*p).data.ddata.children;
        while !e.is_null() {
            fedgeid = fedgeid.wrapping_sub(1);
            e = (*e).nextchild as *mut fsedge;
        }
        nextedgeid = fedgeid;
        e = (*p).data.ddata.children;
        while !e.is_null() {
            let c2rust_fresh22 = fedgeid;
            fedgeid = fedgeid.wrapping_add(1);
            (*e).edgeid = c2rust_fresh22;
            e = (*e).nextchild as *mut fsedge;
        }
        e = (*p).data.ddata.children;
        while !e.is_null() {
            fsnodes_keep_alive_check();
            if (*(*e).child).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                fs_renumerate_edges((*e).child as *mut fsnode);
            }
            e = (*e).nextchild as *mut fsedge;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mr_renumerate_edges(mut expected_nextedgeid: uint64_t) -> uint8_t {
    unsafe {
        nextedgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
        nextedgeid = nextedgeid.wrapping_sub(1);
        fsnodes_keep_alive_begin();
        fs_renumerate_edges(root);
        edgesneedrenumeration = 0 as uint8_t;
        if nextedgeid != expected_nextedgeid {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"RENUMERATEEDGES data mismatch: my:%lu != expected:%lu\0".as_ptr()
                    as *const ::core::ffi::c_char,
                nextedgeid,
                expected_nextedgeid,
            );
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_renumerate_edge_test() {
    unsafe {
        if nextedgeid == 0x7fffffffffffffff as uint64_t
            || edgesneedrenumeration as ::core::ffi::c_int != 0
        {
            nextedgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
            nextedgeid = nextedgeid.wrapping_sub(1);
            fsnodes_keep_alive_begin();
            fs_renumerate_edges(root);
            edgesneedrenumeration = 0 as uint8_t;
            changelog(
                b"%u|RENUMERATEEDGES():%lu\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                nextedgeid,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_cleanupedges() {
    unsafe {
        let mut bid: uint32_t = 0;
        fsedge_cleanup();
        fsnodes_edge_hash_cleanup();
        bid = 0 as uint32_t;
        while bid < TRASH_BUCKETS as uint32_t {
            trash[bid as usize] = ::core::ptr::null_mut::<fsedge>();
            bid = bid.wrapping_add(1);
        }
        bid = 0 as uint32_t;
        while bid < SUSTAINED_BUCKETS as uint32_t {
            sustained[bid as usize] = ::core::ptr::null_mut::<fsedge>();
            bid = bid.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_cleanupnodes() {
    unsafe {
        fsnode_cleanup();
        chunktab_cleanup();
        symlink_cleanup();
        fsnodes_node_hash_cleanup();
        root = ::core::ptr::null_mut::<fsnode>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_cleanupfreenodes() {
    unsafe {
        free(freebitmask as *mut ::core::ffi::c_void);
        freebitmask = ::core::ptr::null_mut::<uint32_t>();
        bitmasksize = 0 as uint32_t;
        searchpos = 0 as uint32_t;
        freenode_free_all();
        freelist = ::core::ptr::null_mut::<freenode>();
        freetail = &raw mut freelist;
        freelastts = 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_cleanup() {
    unsafe {
        fprintf(
            stderr,
            b"cleaning objects ...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        fs_cleanupnodes();
        fprintf(stderr, b" done\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            stderr,
            b"cleaning names ...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        fs_cleanupedges();
        fprintf(stderr, b" done\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            stderr,
            b"cleaning deletion timestamps ...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        fs_cleanupfreenodes();
        fprintf(stderr, b" done\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            stderr,
            b"cleaning quota definitions ...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        quotanode_free_all();
        fprintf(stderr, b" done\n\0".as_ptr() as *const ::core::ffi::c_char);
        quotahead = ::core::ptr::null_mut::<quotanode>();
        trashspace = 0 as uint64_t;
        sustainedspace = 0 as uint64_t;
        trashnodes = 0 as uint32_t;
        sustainednodes = 0 as uint32_t;
        nodes = 0 as uint32_t;
        dirnodes = 0 as uint32_t;
        filenodes = 0 as uint32_t;
        maxnodeid = 0 as uint32_t;
        hashelements = 0 as uint32_t;
        appendres_cleanall();
    }
}
#[inline]
unsafe extern "C" fn fs_storeedge(
    mut e: *mut fsedge,
    mut fd: *mut bio,
    mut uedgebuff: *mut uint8_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if e.is_null() {
            memset(
                uedgebuff as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as size_t,
            );
            if bio_write(
                fd,
                uedgebuff as *const ::core::ffi::c_void,
                (4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as uint64_t,
            ) != (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as int64_t
            {
                return;
            }
            return;
        }
        ptr = uedgebuff;
        if (*e).parent.is_null() {
            put32bit(&raw mut ptr, 0 as uint32_t);
        } else {
            put32bit(&raw mut ptr, (*(*e).parent).inode);
        }
        put32bit(&raw mut ptr, (*(*e).child).inode);
        put64bit(&raw mut ptr, (*e).edgeid);
        put16bit(&raw mut ptr, (*e).nleng);
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            &raw const (*e).name as *const uint8_t as *const ::core::ffi::c_void,
            (*e).nleng as size_t,
        );
        if bio_write(
            fd,
            uedgebuff as *const ::core::ffi::c_void,
            (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + (*e).nleng as ::core::ffi::c_int) as uint64_t,
        ) != (4 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + (*e).nleng as ::core::ffi::c_int) as int64_t
        {
            return;
        }
    }
}
#[inline]
unsafe extern "C" fn fs_loadedge(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut uedgebuff: [uint8_t; 18] = [0; 18];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut parent_id: uint32_t = 0;
        let mut child_id: uint32_t = 0;
        let mut edgeid: uint64_t = 0;
        let mut nleng: uint16_t = 0;
        let mut bid: uint32_t = 0;
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        let mut sr: statsrecord = statsrecord {
            inodes: 0,
            dirs: 0,
            files: 0,
            chunks: 0,
            length: 0,
            size: 0,
            realsize: 0,
        };
        static mut root_tail: *mut *mut fsedge = ::core::ptr::null_mut::<*mut fsedge>();
        static mut root_edgeid: uint64_t = 0;
        static mut current_tail: *mut *mut fsedge = ::core::ptr::null_mut::<*mut fsedge>();
        static mut current_edgeid: uint64_t = 0;
        static mut current_parent: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        static mut current_parent_id: uint32_t = 0;
        static mut current_trash_bid: uint32_t = 0;
        static mut current_sustained_bid: uint32_t = 0;
        static mut nl: uint8_t = 0;
        static mut bsize: ssize_t = 0;
        if fd.is_null() {
            current_trash_bid = TRASH_BUCKETS as uint32_t;
            current_sustained_bid = SUSTAINED_BUCKETS as uint32_t;
            current_parent_id = 0 as uint32_t;
            current_parent = ::core::ptr::null_mut::<fsnode>();
            current_tail = ::core::ptr::null_mut::<*mut fsedge>();
            root_tail = ::core::ptr::null_mut::<*mut fsedge>();
            root_edgeid = 0 as uint64_t;
            current_edgeid = 0 as uint64_t;
            nl = 1 as uint8_t;
            if mver as ::core::ffi::c_int <= 0x10 as ::core::ffi::c_int {
                bsize = (4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as ssize_t;
            } else {
                bsize = (4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as ssize_t;
            }
            return 0 as ::core::ffi::c_int;
        }
        if bio_read(
            fd,
            &raw mut uedgebuff as *mut uint8_t as *mut ::core::ffi::c_void,
            bsize as uint64_t,
        ) != bsize as int64_t
        {
            let mut err: ::core::ffi::c_int = *__errno_location();
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
                nl = 0 as uint8_t;
            }
            *__errno_location() = err;
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading edge: read error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut uedgebuff as *mut uint8_t;
        parent_id = get32bit(&raw mut ptr);
        child_id = get32bit(&raw mut ptr);
        if parent_id == 0 as uint32_t && child_id == 0 as uint32_t {
            return 1 as ::core::ffi::c_int;
        }
        if mver as ::core::ffi::c_int > 0x10 as ::core::ffi::c_int {
            edgeid = get64bit(&raw mut ptr);
        } else {
            edgeid = 0 as uint64_t;
        }
        nleng = get16bit(&raw mut ptr);
        if nleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
                nl = 0 as uint8_t;
            }
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading edge: %u->%u error: empty name\0".as_ptr() as *const ::core::ffi::c_char,
                parent_id,
                child_id,
            );
            if ignoreflag == 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"use option '-i' to generate name replacement\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            } else {
                let mut tmpname: [::core::ffi::c_char; 20] = [0; 20];
                let mut len: uint8_t = 0;
                len = snprintf(
                    &raw mut tmpname as *mut ::core::ffi::c_char,
                    20 as size_t,
                    b"(empty %u)\0".as_ptr() as *const ::core::ffi::c_char,
                    child_id,
                ) as uint8_t;
                if len as ::core::ffi::c_int > 20 as ::core::ffi::c_int {
                    len = 20 as uint8_t;
                }
                e = fsedge_malloc(len as uint16_t);
                if e.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        9555 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"e\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        9555 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"e\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if e
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut fsedge
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        9555 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"e\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        9555 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"e\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                memcpy(
                    &raw const (*e).name as *const uint8_t as *mut uint8_t
                        as *mut ::core::ffi::c_void,
                    &raw mut tmpname as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    len as size_t,
                );
                (*e).nleng = len as uint16_t;
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_NOTICE,
                    b"loading edge: %u->%u empty filename replaced by '(empty %u)'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent_id,
                    child_id,
                    child_id,
                );
            }
        } else if parent_id == 0 as uint32_t && nleng as ::core::ffi::c_int > MFS_PATH_MAX {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"loading edge: %u->%u error: name too long (%hu) -> truncate\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parent_id,
                child_id,
                nleng as ::core::ffi::c_int,
            );
            e = fsedge_malloc(MFS_PATH_MAX as uint16_t);
            if e.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if e
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut fsedge
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9563 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            bio_skip(fd, (nleng as ::core::ffi::c_int - MFS_PATH_MAX) as uint64_t);
            (*e).nleng = MFS_PATH_MAX as uint16_t;
        } else if parent_id > 0 as uint32_t && nleng as ::core::ffi::c_int > MFS_NAME_MAX {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"loading edge: %u->%u error: name too long (%hu) -> truncate\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parent_id,
                child_id,
                nleng as ::core::ffi::c_int,
            );
            e = fsedge_malloc(MFS_NAME_MAX as uint16_t);
            if e.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9569 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9569 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if e
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut fsedge
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9569 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9569 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
            bio_skip(fd, (nleng as ::core::ffi::c_int - MFS_NAME_MAX) as uint64_t);
            (*e).nleng = MFS_NAME_MAX as uint16_t;
        } else {
            e = fsedge_malloc(nleng);
            if e.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9574 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9574 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if e
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut fsedge
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9574 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    9574 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"e\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                abort();
            }
            (*e).nleng = nleng;
        }
        if nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if bio_read(
                fd,
                &raw const (*e).name as *const uint8_t as *mut uint8_t as *mut ::core::ffi::c_void,
                (*e).nleng as uint64_t,
            ) != (*e).nleng as int64_t
            {
                let mut err_0: ::core::ffi::c_int = *__errno_location();
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                *__errno_location() = err_0;
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading edge: read error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                fsedge_free(e, nleng);
                return -1 as ::core::ffi::c_int;
            }
        }
        (*e).child = fsnodes_node_find(child_id) as *mut _fsnode;
        if (*e).child.is_null() {
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
                nl = 0 as uint8_t;
            }
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading edge: %u,%s->%u error: child not found\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parent_id,
                changelog_escape_name(
                    (*e).nleng as uint32_t,
                    &raw const (*e).name as *const uint8_t,
                ),
                child_id,
            );
            fsedge_free(e, nleng);
            if ignoreflag == 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"use option '-i' to ignore all entries pointing to nonexisting objects\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        if parent_id == 0 as uint32_t {
            if (*(*e).child).r#type() as ::core::ffi::c_int == TYPE_TRASH {
                bid = child_id.wrapping_rem(TRASH_BUCKETS as uint32_t);
                if bid != current_trash_bid {
                    current_tail = (&raw mut trash as *mut *mut fsedge).offset(bid as isize);
                    while !(*current_tail).is_null() {
                        current_tail = &raw mut (**current_tail).nextchild as *mut *mut fsedge;
                    }
                }
                (*e).parent = ::core::ptr::null_mut::<_fsnode>();
                *current_tail = e;
                (*e).prevchild = current_tail as *mut *mut _fsedge;
                (*e).nextchild = ::core::ptr::null_mut::<_fsedge>();
                current_tail = &raw mut (*e).nextchild as *mut *mut fsedge;
                current_trash_bid = bid;
                trashspace = trashspace.wrapping_add((*(*e).child).data.fdata.length);
                trashnodes = trashnodes.wrapping_add(1);
            } else if (*(*e).child).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED {
                bid = child_id.wrapping_rem(SUSTAINED_BUCKETS as uint32_t);
                if bid != current_sustained_bid {
                    current_tail = (&raw mut sustained as *mut *mut fsedge).offset(bid as isize);
                    while !(*current_tail).is_null() {
                        current_tail = &raw mut (**current_tail).nextchild as *mut *mut fsedge;
                    }
                }
                (*e).parent = ::core::ptr::null_mut::<_fsnode>();
                *current_tail = e;
                (*e).prevchild = current_tail as *mut *mut _fsedge;
                (*e).nextchild = ::core::ptr::null_mut::<_fsedge>();
                current_tail = &raw mut (*e).nextchild as *mut *mut fsedge;
                current_sustained_bid = bid;
                sustainedspace = sustainedspace.wrapping_add((*(*e).child).data.fdata.length);
                sustainednodes = sustainednodes.wrapping_add(1);
            } else {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading edge: %u,%s->%u error: bad child type (%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent_id,
                    changelog_escape_name(
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                    ),
                    child_id,
                    (*(*e).child).r#type() as ::core::ffi::c_int,
                );
                fsedge_free(e, nleng);
                return -1 as ::core::ffi::c_int;
            }
        } else {
            if current_parent_id == parent_id {
                (*e).parent = current_parent as *mut _fsnode;
            } else {
                (*e).parent = fsnodes_node_find(parent_id) as *mut _fsnode;
            }
            if (*e).parent.is_null() {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading edge: %u,%s->%u error: parent not found\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent_id,
                    changelog_escape_name(
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                    ),
                    child_id,
                );
                if ignoreflag != 0 {
                    (*e).parent = fsnodes_node_find(MFS_ROOT_ID as uint32_t) as *mut _fsnode;
                    if (*e).parent.is_null()
                        || (*(*e).parent).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
                    {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading edge: %u,%s->%u root dir not found !!!\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            parent_id,
                            changelog_escape_name(
                                (*e).nleng as uint32_t,
                                &raw const (*e).name as *const uint8_t,
                            ),
                            child_id,
                        );
                        fsedge_free(e, nleng);
                        return -1 as ::core::ffi::c_int;
                    }
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"loading edge: %u,%s->%u attaching node to root dir\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        parent_id,
                        changelog_escape_name(
                            (*e).nleng as uint32_t,
                            &raw const (*e).name as *const uint8_t,
                        ),
                        child_id,
                    );
                    parent_id = MFS_ROOT_ID as uint32_t;
                } else {
                    fprintf(
                        stderr,
                        b"use option '-i' to attach this node to root dir\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    fsedge_free(e, nleng);
                    return -1 as ::core::ffi::c_int;
                }
            }
            if (*(*e).parent).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading edge: %u,%s->%u error: bad parent type (%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent_id,
                    changelog_escape_name(
                        (*e).nleng as uint32_t,
                        &raw const (*e).name as *const uint8_t,
                    ),
                    child_id,
                    (*(*e).parent).r#type() as ::core::ffi::c_int,
                );
                if ignoreflag != 0 {
                    (*e).parent = fsnodes_node_find(MFS_ROOT_ID as uint32_t) as *mut _fsnode;
                    if (*e).parent.is_null()
                        || (*(*e).parent).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY
                    {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading edge: %u,%s->%u root dir not found !!!\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            parent_id,
                            changelog_escape_name(
                                (*e).nleng as uint32_t,
                                &raw const (*e).name as *const uint8_t,
                            ),
                            child_id,
                        );
                        fsedge_free(e, nleng);
                        return -1 as ::core::ffi::c_int;
                    }
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"loading edge: %u,%s->%u attaching node to root dir\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        parent_id,
                        changelog_escape_name(
                            (*e).nleng as uint32_t,
                            &raw const (*e).name as *const uint8_t,
                        ),
                        child_id,
                    );
                    parent_id = MFS_ROOT_ID as uint32_t;
                } else {
                    fprintf(
                        stderr,
                        b"use option '-i' to attach this node to root dir\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    fsedge_free(e, nleng);
                    return -1 as ::core::ffi::c_int;
                }
            }
            if parent_id == MFS_ROOT_ID as uint32_t {
                if root_tail.is_null() {
                    root_tail = &raw mut (*(*e).parent).data.ddata.children;
                    while !(*root_tail).is_null() {
                        root_edgeid = (**root_tail).edgeid;
                        root_tail = &raw mut (**root_tail).nextchild as *mut *mut fsedge;
                    }
                }
            } else if current_parent_id != parent_id {
                if !(*(*e).parent).data.ddata.children.is_null() {
                    if nl != 0 {
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        nl = 0 as uint8_t;
                    }
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading edge: %u,%s->%u error: parent node sequence error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        parent_id,
                        changelog_escape_name(
                            (*e).nleng as uint32_t,
                            &raw const (*e).name as *const uint8_t,
                        ),
                        child_id,
                    );
                    if ignoreflag != 0 {
                        current_tail = &raw mut (*(*e).parent).data.ddata.children;
                        while !(*current_tail).is_null() {
                            current_edgeid = (**current_tail).edgeid;
                            current_tail = &raw mut (**current_tail).nextchild as *mut *mut fsedge;
                        }
                    } else {
                        fsedge_free(e, nleng);
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    current_tail = &raw mut (*(*e).parent).data.ddata.children;
                    current_edgeid = 0 as uint64_t;
                }
                current_parent_id = parent_id;
                current_parent = (*e).parent as *mut fsnode;
            }
            (*e).nextchild = ::core::ptr::null_mut::<_fsedge>();
            if parent_id == MFS_ROOT_ID as uint32_t {
                *root_tail = e;
                (*e).prevchild = root_tail as *mut *mut _fsedge;
                root_tail = &raw mut (*e).nextchild as *mut *mut fsedge;
                if edgeid <= root_edgeid {
                    if edgesneedrenumeration as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"edgeid mismatch detected - force edgeid renumeration\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        edgesneedrenumeration = 1 as uint8_t;
                    }
                }
                root_edgeid = edgeid;
            } else {
                *current_tail = e;
                (*e).prevchild = current_tail as *mut *mut _fsedge;
                current_tail = &raw mut (*e).nextchild as *mut *mut fsedge;
                if edgeid <= current_edgeid {
                    if edgesneedrenumeration as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"edgeid mismatch detected - force edgeid renumeration\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        edgesneedrenumeration = 1 as uint8_t;
                    }
                }
                current_edgeid = edgeid;
            }
            (*(*e).parent).data.ddata.elements = (*(*e).parent).data.ddata.elements.wrapping_add(1);
            match (*(*e).child).r#type() as ::core::ffi::c_int {
                TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                    (*(*e).child).data.fdata.nlink = (*(*e).child).data.fdata.nlink.wrapping_add(1);
                }
                TYPE_DIRECTORY => {
                    (*(*e).parent).data.ddata.nlink =
                        (*(*e).parent).data.ddata.nlink.wrapping_add(1);
                }
                TYPE_SYMLINK => {
                    (*(*e).child).data.sdata.nlink = (*(*e).child).data.sdata.nlink.wrapping_add(1);
                }
                TYPE_BLOCKDEV | TYPE_CHARDEV => {
                    (*(*e).child).data.devdata.nlink =
                        (*(*e).child).data.devdata.nlink.wrapping_add(1);
                }
                _ => {
                    (*(*e).child).data.odata.nlink = (*(*e).child).data.odata.nlink.wrapping_add(1);
                }
            }
            fsnodes_edge_add(e);
        }
        (*e).nextparent = (*(*e).child).parents as *mut _fsedge;
        if !(*e).nextparent.is_null() {
            (*(*e).nextparent).prevparent = &raw mut (*e).nextparent;
        }
        (*(*e).child).parents = e;
        (*e).prevparent = &raw mut (*(*e).child).parents as *mut *mut _fsedge;
        if !(*e).parent.is_null() {
            fsnodes_get_stats((*e).child as *mut fsnode, &raw mut sr, 1 as uint8_t);
            fsnodes_add_stats((*e).parent as *mut fsnode, &raw mut sr);
        }
        (*e).edgeid = edgeid;
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn fs_storenode(
    mut f: *mut fsnode,
    mut fd: *mut bio,
    mut unodebuff: *mut uint8_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut chptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut ch: uint32_t = 0;
        if f.is_null() || unodebuff.is_null() {
            if bio_write(
                fd,
                b"\0\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as uint64_t,
            ) != 1 as int64_t
            {
                return;
            }
            return;
        }
        ptr = unodebuff;
        put8bit(&raw mut ptr, (*f).r#type() as uint8_t);
        put32bit(&raw mut ptr, (*f).inode);
        put8bit(&raw mut ptr, (*f).sclassid);
        put8bit(&raw mut ptr, (*f).eattr);
        put8bit(&raw mut ptr, (*f).winattr);
        put16bit(&raw mut ptr, (*f).mode() as uint16_t);
        put32bit(&raw mut ptr, (*f).uid);
        put32bit(&raw mut ptr, (*f).gid);
        put32bit(&raw mut ptr, (*f).atime);
        put32bit(&raw mut ptr, (*f).mtime);
        put32bit(&raw mut ptr, (*f).ctime);
        put16bit(&raw mut ptr, (*f).trashretention);
        match (*f).r#type() as ::core::ffi::c_int {
            TYPE_DIRECTORY | TYPE_SOCKET | TYPE_FIFO => {
                if bio_write(
                    fd,
                    unodebuff as *const ::core::ffi::c_void,
                    (1 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as uint64_t,
                ) != (1 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as int64_t
                {
                    return;
                }
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                put32bit(&raw mut ptr, (*f).data.devdata.rdev);
                if bio_write(
                    fd,
                    unodebuff as *const ::core::ffi::c_void,
                    (1 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int) as uint64_t,
                ) != (1 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as int64_t
                {
                    return;
                }
            }
            TYPE_SYMLINK => {
                put32bit(&raw mut ptr, (*f).data.sdata.pleng);
                if bio_write(
                    fd,
                    unodebuff as *const ::core::ffi::c_void,
                    (1 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int) as uint64_t,
                ) != (1 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as int64_t
                {
                    return;
                }
                if bio_write(
                    fd,
                    (*f).data.sdata.path as *const ::core::ffi::c_void,
                    (*f).data.sdata.pleng as uint64_t,
                ) != (*f).data.sdata.pleng as int64_t
                {
                    return;
                }
            }
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                put64bit(&raw mut ptr, (*f).data.fdata.length);
                ch = 0 as uint32_t;
                indx = 0 as uint32_t;
                while indx < (*f).data.fdata.chunks {
                    if *(*f).data.fdata.chunktab.offset(indx as isize) != 0 as uint64_t {
                        ch = indx.wrapping_add(1 as uint32_t);
                    }
                    indx = indx.wrapping_add(1);
                }
                put32bit(&raw mut ptr, ch);
                if bio_write(
                    fd,
                    unodebuff as *const ::core::ffi::c_void,
                    (1 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int) as uint64_t,
                ) != (1 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as int64_t
                {
                    return;
                }
                indx = 0 as uint32_t;
                while ch > 65536 as uint32_t {
                    chptr = ptr;
                    i = 0 as uint32_t;
                    while i < 65536 as uint32_t {
                        put64bit(
                            &raw mut chptr,
                            *(*f).data.fdata.chunktab.offset(indx as isize),
                        );
                        indx = indx.wrapping_add(1);
                        i = i.wrapping_add(1);
                    }
                    if bio_write(
                        fd,
                        ptr as *const ::core::ffi::c_void,
                        (8 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int) as uint64_t,
                    ) != (8 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int) as int64_t
                    {
                        return;
                    }
                    ch = ch.wrapping_sub(65536 as uint32_t);
                }
                chptr = ptr;
                i = 0 as uint32_t;
                while i < ch {
                    put64bit(
                        &raw mut chptr,
                        *(*f).data.fdata.chunktab.offset(indx as isize),
                    );
                    indx = indx.wrapping_add(1);
                    i = i.wrapping_add(1);
                }
                if ch > 0 as uint32_t {
                    if bio_write(
                        fd,
                        ptr as *const ::core::ffi::c_void,
                        (8 as uint32_t).wrapping_mul(ch) as uint64_t,
                    ) != (8 as uint32_t).wrapping_mul(ch) as int64_t
                    {
                        return;
                    }
                }
            }
            _ => {}
        };
    }
}
#[inline]
unsafe extern "C" fn fs_loadnode(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
    mut unodebuff: *mut uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut chptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut r#type: uint8_t = 0;
        let mut trashseconds: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut ch: uint32_t = 0;
        let mut sessionids: uint32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        static mut nl: uint8_t = 0;
        let mut hdrsize: uint32_t = 0;
        if fd.is_null() || unodebuff.is_null() {
            nl = 1 as uint8_t;
            return 0 as ::core::ffi::c_int;
        }
        if bio_read(
            fd,
            &raw mut r#type as *mut ::core::ffi::c_void,
            1 as uint64_t,
        ) != 1 as int64_t
        {
            return -1 as ::core::ffi::c_int;
        }
        if r#type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if mver as ::core::ffi::c_int <= 0x11 as ::core::ffi::c_int {
            hdrsize = (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint32_t;
        } else if mver as ::core::ffi::c_int <= 0x13 as ::core::ffi::c_int {
            hdrsize = (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint32_t;
        } else {
            hdrsize = (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as uint32_t;
        }
        if mver as ::core::ffi::c_int <= 0x12 as ::core::ffi::c_int {
            r#type = fsnodes_type_convert(r#type);
        }
        match r#type as ::core::ffi::c_int {
            TYPE_DIRECTORY | TYPE_FIFO | TYPE_SOCKET => {
                if bio_read(
                    fd,
                    unodebuff as *mut ::core::ffi::c_void,
                    hdrsize as uint64_t,
                ) != hdrsize as int64_t
                {
                    let mut err: ::core::ffi::c_int = *__errno_location();
                    if nl != 0 {
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        nl = 0 as uint8_t;
                    }
                    *__errno_location() = err;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading node: read error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV | TYPE_SYMLINK => {
                if bio_read(
                    fd,
                    unodebuff as *mut ::core::ffi::c_void,
                    hdrsize.wrapping_add(4 as uint32_t) as uint64_t,
                ) != hdrsize.wrapping_add(4 as uint32_t) as int64_t
                {
                    let mut err_0: ::core::ffi::c_int = *__errno_location();
                    if nl != 0 {
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        nl = 0 as uint8_t;
                    }
                    *__errno_location() = err_0;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading node: read error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                if mver as ::core::ffi::c_int <= 0x13 as ::core::ffi::c_int {
                    if bio_read(
                        fd,
                        unodebuff as *mut ::core::ffi::c_void,
                        hdrsize
                            .wrapping_add(8 as uint32_t)
                            .wrapping_add(4 as uint32_t)
                            .wrapping_add(2 as uint32_t) as uint64_t,
                    ) != hdrsize
                        .wrapping_add(8 as uint32_t)
                        .wrapping_add(4 as uint32_t)
                        .wrapping_add(2 as uint32_t) as int64_t
                    {
                        let mut err_1: ::core::ffi::c_int = *__errno_location();
                        if nl != 0 {
                            fputc('\n' as ::core::ffi::c_int, stderr);
                            nl = 0 as uint8_t;
                        }
                        *__errno_location() = err_1;
                        mfs_log(
                            MFSLOG_ERRNO_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading node: read error\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                } else if bio_read(
                    fd,
                    unodebuff as *mut ::core::ffi::c_void,
                    hdrsize
                        .wrapping_add(8 as uint32_t)
                        .wrapping_add(4 as uint32_t) as uint64_t,
                ) != hdrsize
                    .wrapping_add(8 as uint32_t)
                    .wrapping_add(4 as uint32_t) as int64_t
                {
                    let mut err_2: ::core::ffi::c_int = *__errno_location();
                    if nl != 0 {
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        nl = 0 as uint8_t;
                    }
                    *__errno_location() = err_2;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading node: read error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
            _ => {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading node: unrecognized node type: %hhu\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    r#type as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        ptr = unodebuff;
        match r#type as ::core::ffi::c_int {
            TYPE_DIRECTORY => {
                p = fsnode_malloc(0 as uint8_t);
            }
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                p = fsnode_malloc(1 as uint8_t);
            }
            TYPE_SYMLINK => {
                p = fsnode_malloc(2 as uint8_t);
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                p = fsnode_malloc(3 as uint8_t);
            }
            _ => {
                p = fsnode_malloc(4 as uint8_t);
            }
        }
        if p.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                9994 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                9994 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if p
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut fsnode
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                9994 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                9994 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*p).set_xattrflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).set_aclpermflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).set_acldefflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).set_keepmode(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).set_type(r#type as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*p).inode = get32bit(&raw mut ptr);
        (*p).sclassid = get8bit(&raw mut ptr);
        if r#type as ::core::ffi::c_int != TYPE_DIRECTORY
            && r#type as ::core::ffi::c_int != TYPE_FILE
            && r#type as ::core::ffi::c_int != TYPE_TRASH
            && r#type as ::core::ffi::c_int != TYPE_SUSTAINED
        {
            (*p).sclassid = 0 as uint8_t;
        }
        sclass_incref((*p).sclassid as uint16_t, (*p).r#type() as uint8_t);
        if mver as ::core::ffi::c_int <= 0x11 as ::core::ffi::c_int {
            let mut flagsmode: uint16_t = get16bit(&raw mut ptr);
            (*p).eattr = (flagsmode as ::core::ffi::c_int >> 12 as ::core::ffi::c_int) as uint8_t;
            (*p).winattr = 0 as uint8_t;
            (*p).set_mode(
                (flagsmode as ::core::ffi::c_int & 0xfff as ::core::ffi::c_int)
                    as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
        } else {
            (*p).eattr = get8bit(&raw mut ptr);
            if mver as ::core::ffi::c_int >= 0x14 as ::core::ffi::c_int {
                (*p).winattr = get8bit(&raw mut ptr);
            } else {
                (*p).winattr = 0 as uint8_t;
            }
            (*p).set_mode(get16bit(&raw mut ptr) as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        (*p).uid = get32bit(&raw mut ptr);
        (*p).gid = get32bit(&raw mut ptr);
        (*p).atime = get32bit(&raw mut ptr);
        (*p).mtime = get32bit(&raw mut ptr);
        (*p).ctime = get32bit(&raw mut ptr);
        if mver as ::core::ffi::c_int <= 0x13 as ::core::ffi::c_int {
            trashseconds = get32bit(&raw mut ptr);
            (*p).trashretention = trashseconds
                .wrapping_add(3599 as uint32_t)
                .wrapping_div(3600 as uint32_t) as uint16_t;
        } else {
            (*p).trashretention = get16bit(&raw mut ptr);
        }
        match r#type as ::core::ffi::c_int {
            TYPE_DIRECTORY => {
                memset(
                    &raw mut (*p).data.ddata.stats as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<statsrecord>(),
                );
                (*p).data.ddata.quota = ::core::ptr::null_mut::<quotanode>();
                (*p).data.ddata.children = ::core::ptr::null_mut::<fsedge>();
                (*p).data.ddata.nlink = 2 as uint32_t;
                (*p).data.ddata.elements = 0 as uint32_t;
            }
            TYPE_SOCKET | TYPE_FIFO => {
                (*p).data.odata.nlink = 0 as uint16_t;
            }
            TYPE_BLOCKDEV | TYPE_CHARDEV => {
                (*p).data.devdata.nlink = 0 as uint16_t;
                (*p).data.devdata.rdev = get32bit(&raw mut ptr);
            }
            TYPE_SYMLINK => {
                (*p).data.sdata.nlink = 0 as uint16_t;
                pleng = get32bit(&raw mut ptr);
                (*p).data.sdata.pleng = pleng;
                if pleng > 0 as uint32_t {
                    if pleng > MFS_SYMLINK_MAX as uint32_t {
                        (*p).data.sdata.pleng = 22 as uint32_t;
                        (*p).data.sdata.path = symlink_malloc((*p).data.sdata.pleng as uint16_t);
                        if (*p).data.sdata.path.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                10056 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"p->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                10056 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"p->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if (*p).data.sdata.path
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut uint8_t
                        {
                            let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                10056 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"p->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                10056 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"p->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            abort();
                        }
                        memcpy(
                            (*p).data.sdata.path as *mut ::core::ffi::c_void,
                            b"... path too long ...\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            (*p).data.sdata.pleng as size_t,
                        );
                        bio_skip(fd, pleng as uint64_t);
                    } else {
                        (*p).data.sdata.path = symlink_malloc(pleng as uint16_t);
                        if (*p).data.sdata.path.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                10061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"p->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                10061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"p->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if (*p).data.sdata.path
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut uint8_t
                        {
                            let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                10061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"p->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_1,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                10061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"p->data.sdata.path\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_1,
                            );
                            abort();
                        }
                        if bio_read(
                            fd,
                            (*p).data.sdata.path as *mut ::core::ffi::c_void,
                            pleng as uint64_t,
                        ) != pleng as int64_t
                        {
                            let mut err_3: ::core::ffi::c_int = *__errno_location();
                            if nl != 0 {
                                fputc('\n' as ::core::ffi::c_int, stderr);
                                nl = 0 as uint8_t;
                            }
                            *__errno_location() = err_3;
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_ERR,
                                b"loading node: read error\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            symlink_free((*p).data.sdata.path, pleng as uint16_t);
                            fsnode_free(p, 2 as uint8_t);
                            return -1 as ::core::ffi::c_int;
                        }
                    }
                } else {
                    (*p).data.sdata.path = ::core::ptr::null_mut::<uint8_t>();
                }
            }
            TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                (*p).data.fdata.nlink = 0 as uint16_t;
                (*p).data.fdata.length = get64bit(&raw mut ptr);
                ch = get32bit(&raw mut ptr);
                (*p).data.fdata.chunks = ch;
                if mver as ::core::ffi::c_int <= 0x13 as ::core::ffi::c_int {
                    sessionids = get16bit(&raw mut ptr) as uint32_t;
                } else {
                    sessionids = 0 as uint32_t;
                }
                if ch > 0 as uint32_t {
                    (*p).data.fdata.chunktab = chunktab_malloc(ch);
                    if (*p).data.fdata.chunktab.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            10093 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            10093 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*p).data.fdata.chunktab
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint64_t
                    {
                        let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            10093 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            10093 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"p->data.fdata.chunktab\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_2,
                        );
                        abort();
                    }
                } else {
                    (*p).data.fdata.chunktab = ::core::ptr::null_mut::<uint64_t>();
                }
                indx = 0 as uint32_t;
                while ch > 65536 as uint32_t {
                    chptr = ptr;
                    if bio_read(
                        fd,
                        ptr as *mut uint8_t as *mut ::core::ffi::c_void,
                        (8 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int) as uint64_t,
                    ) != (8 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int) as int64_t
                    {
                        let mut err_4: ::core::ffi::c_int = *__errno_location();
                        if nl != 0 {
                            fputc('\n' as ::core::ffi::c_int, stderr);
                            nl = 0 as uint8_t;
                        }
                        *__errno_location() = err_4;
                        mfs_log(
                            MFSLOG_ERRNO_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading node: read error\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if !(*p).data.fdata.chunktab.is_null() {
                            chunktab_free((*p).data.fdata.chunktab, (*p).data.fdata.chunks);
                        }
                        fsnode_free(p, 1 as uint8_t);
                        return -1 as ::core::ffi::c_int;
                    }
                    i = 0 as uint32_t;
                    while i < 65536 as uint32_t {
                        *(*p).data.fdata.chunktab.offset(indx as isize) = get64bit(&raw mut chptr);
                        indx = indx.wrapping_add(1);
                        i = i.wrapping_add(1);
                    }
                    ch = ch.wrapping_sub(65536 as uint32_t);
                }
                if bio_read(
                    fd,
                    ptr as *mut uint8_t as *mut ::core::ffi::c_void,
                    (8 as uint32_t)
                        .wrapping_mul(ch)
                        .wrapping_add((4 as uint32_t).wrapping_mul(sessionids))
                        as uint64_t,
                ) != (8 as uint32_t)
                    .wrapping_mul(ch)
                    .wrapping_add((4 as uint32_t).wrapping_mul(sessionids))
                    as int64_t
                {
                    let mut err_5: ::core::ffi::c_int = *__errno_location();
                    if nl != 0 {
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        nl = 0 as uint8_t;
                    }
                    *__errno_location() = err_5;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading node: read error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if !(*p).data.fdata.chunktab.is_null() {
                        chunktab_free((*p).data.fdata.chunktab, (*p).data.fdata.chunks);
                    }
                    fsnode_free(p, 1 as uint8_t);
                    return -1 as ::core::ffi::c_int;
                }
                i = 0 as uint32_t;
                while i < ch {
                    *(*p).data.fdata.chunktab.offset(indx as isize) = get64bit(&raw mut ptr);
                    indx = indx.wrapping_add(1);
                    i = i.wrapping_add(1);
                }
                while sessionids != 0 {
                    sessionid = get32bit(&raw mut ptr);
                    of_mr_acquire(sessionid, (*p).inode);
                    sessionids = sessionids.wrapping_sub(1);
                }
            }
            _ => {}
        }
        (*p).parents = ::core::ptr::null_mut::<fsedge>();
        if !fsnodes_node_find((*p).inode).is_null() {
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
                nl = 0 as uint8_t;
            }
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading node %u error: node already exists\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*p).inode,
            );
            if (*p).r#type() as ::core::ffi::c_int == TYPE_FILE
                || (*p).r#type() as ::core::ffi::c_int == TYPE_TRASH
                || (*p).r#type() as ::core::ffi::c_int == TYPE_SUSTAINED
            {
                if !(*p).data.fdata.chunktab.is_null() {
                    chunktab_free((*p).data.fdata.chunktab, (*p).data.fdata.chunks);
                }
            }
            if (*p).r#type() as ::core::ffi::c_int == TYPE_SYMLINK {
                if !(*p).data.sdata.path.is_null() {
                    symlink_free((*p).data.sdata.path, (*p).data.sdata.pleng as uint16_t);
                }
            }
            match (*p).r#type() as ::core::ffi::c_int {
                TYPE_DIRECTORY => {
                    fsnode_free(p, 0 as uint8_t);
                }
                TYPE_FILE | TYPE_TRASH | TYPE_SUSTAINED => {
                    fsnode_free(p, 1 as uint8_t);
                }
                TYPE_SYMLINK => {
                    fsnode_free(p, 2 as uint8_t);
                }
                TYPE_BLOCKDEV | TYPE_CHARDEV => {
                    fsnode_free(p, 3 as uint8_t);
                }
                _ => {
                    fsnode_free(p, 4 as uint8_t);
                }
            }
            if ignoreflag == 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"use option '-i' to ignore\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        } else {
            fsnodes_node_add(p);
            fsnodes_used_inode((*p).inode);
            nodes = nodes.wrapping_add(1);
            if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
                dirnodes = dirnodes.wrapping_add(1);
            }
            if r#type as ::core::ffi::c_int == TYPE_FILE
                || r#type as ::core::ffi::c_int == TYPE_TRASH
                || r#type as ::core::ffi::c_int == TYPE_SUSTAINED
            {
                filenodes = filenodes.wrapping_add(1);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_storenodes(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut i: uint32_t = 0;
        let mut auxbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut hdr: [uint8_t; 8] = [0; 8];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        if fd.is_null() {
            return 0x14 as uint8_t;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        put32bit(&raw mut ptr, maxnodeid);
        put32bit(&raw mut ptr, nodes);
        if bio_write(
            fd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        ) != 8 as int64_t
        {
            return 0xff as uint8_t;
        }
        auxbuff = malloc(
            (1 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as size_t,
        ) as *mut uint8_t;
        i = 0 as uint32_t;
        while i < noderehashpos {
            p = *nodehashtab[(i >> HASHTAB_LOBITS) as usize]
                .offset((i & HASHTAB_MASK as uint32_t) as isize);
            while !p.is_null() && bio_error(fd) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                fs_storenode(p, fd, auxbuff);
                p = (*p).next as *mut fsnode;
            }
            i = i.wrapping_add(1);
        }
        free(auxbuff as *mut ::core::ffi::c_void);
        fs_storenode(
            ::core::ptr::null_mut::<fsnode>(),
            fd,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn fs_storeedgelist(
    mut e: *mut fsedge,
    mut fd: *mut bio,
    mut auxbuff: *mut uint8_t,
) {
    unsafe {
        while !e.is_null() && bio_error(fd) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            fs_storeedge(e, fd, auxbuff);
            e = (*e).nextchild as *mut fsedge;
        }
    }
}
#[inline]
unsafe extern "C" fn fs_storeedges_rec(
    mut f: *mut fsnode,
    mut fd: *mut bio,
    mut auxbuff: *mut uint8_t,
) {
    unsafe {
        let mut e: *mut fsedge = ::core::ptr::null_mut::<fsedge>();
        fs_storeedgelist((*f).data.ddata.children, fd, auxbuff);
        e = (*f).data.ddata.children;
        while !e.is_null() && bio_error(fd) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if (*(*e).child).r#type() as ::core::ffi::c_int == TYPE_DIRECTORY {
                fs_storeedges_rec((*e).child as *mut fsnode, fd, auxbuff);
            }
            e = (*e).nextchild as *mut fsedge;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_storeedges(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut bid: uint32_t = 0;
        let mut auxbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut hdr: [uint8_t; 8] = [0; 8];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if fd.is_null() {
            return 0x11 as uint8_t;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        put64bit(&raw mut ptr, nextedgeid);
        if bio_write(
            fd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        ) != 8 as int64_t
        {
            return 0xff as uint8_t;
        }
        auxbuff = malloc(
            (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 65535 as ::core::ffi::c_int) as size_t,
        ) as *mut uint8_t;
        fs_storeedges_rec(root, fd, auxbuff);
        bid = 0 as uint32_t;
        while bid < TRASH_BUCKETS as uint32_t {
            fs_storeedgelist(trash[bid as usize], fd, auxbuff);
            bid = bid.wrapping_add(1);
        }
        bid = 0 as uint32_t;
        while bid < SUSTAINED_BUCKETS as uint32_t {
            fs_storeedgelist(sustained[bid as usize], fd, auxbuff);
            bid = bid.wrapping_add(1);
        }
        fs_storeedge(::core::ptr::null_mut::<fsedge>(), fd, auxbuff);
        free(auxbuff as *mut ::core::ffi::c_void);
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_lostnode(mut p: *mut fsnode) -> ::core::ffi::c_int {
    unsafe {
        let mut artname: [uint8_t; 40] = [0; 40];
        let mut i: uint32_t = 0;
        let mut l: uint32_t = 0;
        i = 0 as uint32_t;
        loop {
            if i == 0 as uint32_t {
                l = snprintf(
                    &raw mut artname as *mut uint8_t as *mut ::core::ffi::c_char,
                    40 as size_t,
                    b"lost_node_%u\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).inode,
                ) as uint32_t;
            } else {
                l = snprintf(
                    &raw mut artname as *mut uint8_t as *mut ::core::ffi::c_char,
                    40 as size_t,
                    b"lost_node_%u.%u\0".as_ptr() as *const ::core::ffi::c_char,
                    (*p).inode,
                    i,
                ) as uint32_t;
            }
            if fsnodes_nameisused(root, l as uint16_t, &raw mut artname as *mut uint8_t) == 0 {
                fsnodes_link(
                    0 as uint32_t,
                    root,
                    p,
                    l as uint16_t,
                    &raw mut artname as *mut uint8_t,
                );
                return 1 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
            if i == 0 {
                break;
            }
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_checknodes() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut nl: uint8_t = 0;
        let mut p: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        nl = 1 as uint8_t;
        i = 0 as uint32_t;
        while i < noderehashpos {
            p = *nodehashtab[(i >> HASHTAB_LOBITS) as usize]
                .offset((i & HASHTAB_MASK as uint32_t) as isize);
            while !p.is_null() {
                if (*p).parents.is_null() && p != root {
                    if nl != 0 {
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        nl = 0 as uint8_t;
                    }
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"found orphaned inode: %u\0".as_ptr() as *const ::core::ffi::c_char,
                        (*p).inode,
                    );
                    if fs_lostnode(p) < 0 as ::core::ffi::c_int {
                        return -1 as ::core::ffi::c_int;
                    }
                }
                p = (*p).next as *mut fsnode;
            }
            i = i.wrapping_add(1);
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_importnodes(
    mut fd: *mut bio,
    mut mni: uint32_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut s: ::core::ffi::c_int = 0;
        let mut auxbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        maxnodeid = mni;
        hashelements = 1 as uint32_t;
        fsnodes_init_freebitmask();
        auxbuff = malloc(
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as size_t,
        ) as *mut uint8_t;
        fs_loadnode(
            ::core::ptr::null_mut::<bio>(),
            0 as uint8_t,
            ignoreflag,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        loop {
            s = fs_loadnode(fd, 0x10 as uint8_t, ignoreflag, auxbuff);
            if s != 0 as ::core::ffi::c_int {
                break;
            }
        }
        free(auxbuff as *mut ::core::ffi::c_void);
        return if s < 0 as ::core::ffi::c_int {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_loadnodes(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut s: ::core::ffi::c_int = 0;
        let mut auxbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut hdr: [uint8_t; 8] = [0; 8];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        if mver as ::core::ffi::c_int >= 0x11 as ::core::ffi::c_int {
            if bio_read(
                fd,
                &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
                8 as uint64_t,
            ) != 8 as int64_t
            {
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut hdr as *mut uint8_t;
            maxnodeid = get32bit(&raw mut ptr);
            hashelements = get32bit(&raw mut ptr);
        } else {
            if bio_read(
                fd,
                &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
                4 as uint64_t,
            ) != 4 as int64_t
            {
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut hdr as *mut uint8_t;
            maxnodeid = get32bit(&raw mut ptr);
            hashelements = 1 as uint32_t;
        }
        fsnodes_init_freebitmask();
        auxbuff = malloc(
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * 65536 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as size_t,
        ) as *mut uint8_t;
        fs_loadnode(
            ::core::ptr::null_mut::<bio>(),
            0 as uint8_t,
            ignoreflag,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        loop {
            s = fs_loadnode(fd, mver, ignoreflag, auxbuff);
            if s != 0 as ::core::ffi::c_int {
                break;
            }
        }
        free(auxbuff as *mut ::core::ffi::c_void);
        return if s < 0 as ::core::ffi::c_int {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_loadedges(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut s: ::core::ffi::c_int = 0;
        let mut hdr: [uint8_t; 8] = [0; 8];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        if mver as ::core::ffi::c_int >= 0x11 as ::core::ffi::c_int {
            if bio_read(
                fd,
                &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
                8 as uint64_t,
            ) != 8 as int64_t
            {
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut hdr as *mut uint8_t;
            nextedgeid = get64bit(&raw mut ptr);
            edgesneedrenumeration = 0 as uint8_t;
        } else {
            nextedgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
            edgesneedrenumeration = 1 as uint8_t;
        }
        fs_loadedge(::core::ptr::null_mut::<bio>(), mver, ignoreflag);
        loop {
            s = fs_loadedge(fd, mver, ignoreflag);
            if s != 0 as ::core::ffi::c_int {
                break;
            }
        }
        return if s < 0 as ::core::ffi::c_int {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_storefree(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut wbuff: [uint8_t; 8] = [0; 8];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut n: *mut freenode = ::core::ptr::null_mut::<freenode>();
        let mut l: uint32_t = 0;
        if fd.is_null() {
            return 0x10 as uint8_t;
        }
        l = 0 as uint32_t;
        n = freelist;
        while !n.is_null() {
            l = l.wrapping_add(1);
            n = (*n).next as *mut freenode;
        }
        ptr = &raw mut wbuff as *mut uint8_t;
        put32bit(&raw mut ptr, l);
        if bio_write(
            fd,
            &raw mut wbuff as *mut uint8_t as *const ::core::ffi::c_void,
            4 as uint64_t,
        ) != 4 as int64_t
        {
            return 0xff as uint8_t;
        }
        ptr = &raw mut wbuff as *mut uint8_t;
        n = freelist;
        while !n.is_null() {
            ptr = &raw mut wbuff as *mut uint8_t;
            put32bit(&raw mut ptr, (*n).inode);
            put32bit(&raw mut ptr, (*n).ftime);
            if bio_write(
                fd,
                &raw mut wbuff as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint64_t,
            ) != 8 as int64_t
            {
                return 0xff as uint8_t;
            }
            n = (*n).next as *mut freenode;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_loadfree(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rbuff: [uint8_t; 8192] = [0; 8192];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut n: *mut freenode = ::core::ptr::null_mut::<freenode>();
        let mut l: uint32_t = 0;
        let mut t: uint32_t = 0;
        let mut nodeid: uint32_t = 0;
        let mut ftime: uint32_t = 0;
        let mut prevftime: uint32_t = 0;
        let mut nl: uint8_t = 1 as uint8_t;
        if bio_read(
            fd,
            &raw mut rbuff as *mut uint8_t as *mut ::core::ffi::c_void,
            4 as uint64_t,
        ) != 4 as int64_t
        {
            let mut err: ::core::ffi::c_int = *__errno_location();
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
            }
            *__errno_location() = err;
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading free nodes: read error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut rbuff as *mut uint8_t;
        t = get32bit(&raw mut ptr);
        prevftime = 0 as uint32_t;
        freelist = ::core::ptr::null_mut::<freenode>();
        freetail = &raw mut freelist;
        freelastts = 0 as uint32_t;
        l = 0 as uint32_t;
        while t > 0 as uint32_t {
            if l == 0 as uint32_t {
                if t > 1024 as uint32_t {
                    if bio_read(
                        fd,
                        &raw mut rbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                        (8 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as uint64_t,
                    ) != (8 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as int64_t
                    {
                        let mut err_0: ::core::ffi::c_int = *__errno_location();
                        if nl != 0 {
                            fputc('\n' as ::core::ffi::c_int, stderr);
                        }
                        *__errno_location() = err_0;
                        mfs_log(
                            MFSLOG_ERRNO_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading free nodes: read error\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    l = 1024 as uint32_t;
                } else {
                    if bio_read(
                        fd,
                        &raw mut rbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                        (8 as uint32_t).wrapping_mul(t) as uint64_t,
                    ) != (8 as uint32_t).wrapping_mul(t) as int64_t
                    {
                        let mut err_1: ::core::ffi::c_int = *__errno_location();
                        if nl != 0 {
                            fputc('\n' as ::core::ffi::c_int, stderr);
                        }
                        *__errno_location() = err_1;
                        mfs_log(
                            MFSLOG_ERRNO_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading free nodes: read error\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    l = t;
                }
                ptr = &raw mut rbuff as *mut uint8_t;
            }
            nodeid = get32bit(&raw mut ptr);
            ftime = get32bit(&raw mut ptr);
            if ftime < prevftime {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading free nodes: bad timestamp order (inode:%u ; free timestamp:%u ; previous free timestamp:%u)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    nodeid,
                    ftime,
                    prevftime,
                );
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"skipping remaining free (deleted) nodes\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    bio_skip(
                        fd,
                        t.wrapping_sub(l).wrapping_mul(1024 as uint32_t) as uint64_t,
                    );
                    return 0 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"use option '-i' to skip loading free (deleted) inodes\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            prevftime = ftime;
            n = freenode_malloc();
            (*n).inode = nodeid;
            (*n).ftime = ftime;
            (*n).next = ::core::ptr::null_mut::<_freenode>();
            *freetail = n;
            freetail = &raw mut (*n).next as *mut *mut freenode;
            freelastts = ftime;
            fsnodes_used_inode(nodeid);
            l = l.wrapping_sub(1);
            t = t.wrapping_sub(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_storequota(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut wbuff: [uint8_t; 70] = [0; 70];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut l: uint32_t = 0;
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        if fd.is_null() {
            return 0x11 as uint8_t;
        }
        l = 0 as uint32_t;
        qn = quotahead;
        while !qn.is_null() {
            l = l.wrapping_add(1);
            qn = (*qn).next as *mut quotanode;
        }
        ptr = &raw mut wbuff as *mut uint8_t;
        put32bit(&raw mut ptr, l);
        if bio_write(
            fd,
            &raw mut wbuff as *mut uint8_t as *const ::core::ffi::c_void,
            4 as uint64_t,
        ) != 4 as int64_t
        {
            return 0xff as uint8_t;
        }
        qn = quotahead;
        while !qn.is_null() {
            ptr = &raw mut wbuff as *mut uint8_t;
            if (*qn).node.is_null() {
                put32bit(&raw mut ptr, 0 as uint32_t);
            } else {
                put32bit(&raw mut ptr, (*(*qn).node).inode);
            }
            put32bit(&raw mut ptr, (*qn).graceperiod);
            put8bit(&raw mut ptr, (*qn).exceeded);
            put8bit(&raw mut ptr, (*qn).flags);
            put32bit(&raw mut ptr, (*qn).stimestamp);
            put32bit(&raw mut ptr, (*qn).sinodes);
            put32bit(&raw mut ptr, (*qn).hinodes);
            put64bit(&raw mut ptr, (*qn).slength);
            put64bit(&raw mut ptr, (*qn).hlength);
            put64bit(&raw mut ptr, (*qn).ssize);
            put64bit(&raw mut ptr, (*qn).hsize);
            put64bit(&raw mut ptr, (*qn).srealsize);
            put64bit(&raw mut ptr, (*qn).hrealsize);
            if bio_write(
                fd,
                &raw mut wbuff as *mut uint8_t as *const ::core::ffi::c_void,
                70 as uint64_t,
            ) != 70 as int64_t
            {
                return 0xff as uint8_t;
            }
            qn = (*qn).next as *mut quotanode;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_loadquota(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rbuff: [uint8_t; 70] = [0; 70];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut qn: *mut quotanode = ::core::ptr::null_mut::<quotanode>();
        let mut r#fn: *mut fsnode = ::core::ptr::null_mut::<fsnode>();
        let mut l: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut nl: uint8_t = 1 as uint8_t;
        let mut rsize: int32_t = 0;
        if bio_read(
            fd,
            &raw mut rbuff as *mut uint8_t as *mut ::core::ffi::c_void,
            4 as uint64_t,
        ) != 4 as int64_t
        {
            let mut err: ::core::ffi::c_int = *__errno_location();
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
            }
            *__errno_location() = err;
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading quota: read error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut rbuff as *mut uint8_t;
        l = get32bit(&raw mut ptr);
        quotahead = ::core::ptr::null_mut::<quotanode>();
        rsize = (if mver as ::core::ffi::c_int == 0x10 as ::core::ffi::c_int {
            66 as ::core::ffi::c_int
        } else {
            70 as ::core::ffi::c_int
        }) as int32_t;
        while l > 0 as uint32_t {
            l = l.wrapping_sub(1);
            if bio_read(
                fd,
                &raw mut rbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                rsize as uint64_t,
            ) != rsize as int64_t
            {
                let mut err_0: ::core::ffi::c_int = *__errno_location();
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                }
                *__errno_location() = err_0;
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading quota: read error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut rbuff as *mut uint8_t;
            inode = get32bit(&raw mut ptr);
            if inode == 0 as uint32_t {
                continue;
            }
            r#fn = fsnodes_node_find(inode);
            if r#fn.is_null() || (*r#fn).r#type() as ::core::ffi::c_int != TYPE_DIRECTORY {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"quota defined for %s inode: %u\0".as_ptr() as *const ::core::ffi::c_char,
                    if r#fn.is_null() {
                        b"non existing\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"not directory\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    inode,
                );
                if ignoreflag != 0 {
                    ptr = ptr.offset((rsize - 4 as int32_t) as isize);
                } else {
                    fprintf(
                        stderr,
                        b"use option '-i' to ignore incorrect quota definitions\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                qn = fsnodes_new_quotanode(r#fn);
                if mver as ::core::ffi::c_int == 0x10 as ::core::ffi::c_int {
                    (*qn).graceperiod = QUOTA_PERIOD_DEFAULT as uint32_t;
                } else {
                    (*qn).graceperiod = get32bit(&raw mut ptr);
                }
                (*qn).exceeded = get8bit(&raw mut ptr);
                (*qn).flags = get8bit(&raw mut ptr);
                (*qn).stimestamp = get32bit(&raw mut ptr);
                (*qn).sinodes = get32bit(&raw mut ptr);
                (*qn).hinodes = get32bit(&raw mut ptr);
                (*qn).slength = get64bit(&raw mut ptr);
                (*qn).hlength = get64bit(&raw mut ptr);
                (*qn).ssize = get64bit(&raw mut ptr);
                (*qn).hsize = get64bit(&raw mut ptr);
                (*qn).srealsize = get64bit(&raw mut ptr);
                (*qn).hrealsize = get64bit(&raw mut ptr);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_new() {
    unsafe {
        nextedgeid = (0x7fffffffffffffff as ::core::ffi::c_ulong)
            .wrapping_sub(1 as ::core::ffi::c_ulong) as uint64_t;
        edgesneedrenumeration = 0 as uint8_t;
        hashelements = 1 as uint32_t;
        maxnodeid = MFS_ROOT_ID as uint32_t;
        fsnodes_init_freebitmask();
        root = fsnode_malloc(0 as uint8_t);
        if root.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                10633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"root\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                10633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"root\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if root
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut fsnode
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                10633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"root\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/filesystem.c\0".as_ptr() as *const ::core::ffi::c_char,
                10633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"root\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*root).inode = MFS_ROOT_ID as uint32_t;
        (*root).set_xattrflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*root).set_aclpermflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*root).set_acldefflag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*root).set_keepmode(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*root).set_type(TYPE_DIRECTORY as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*root).atime = main_time();
        (*root).mtime = (*root).atime;
        (*root).ctime = (*root).mtime;
        (*root).sclassid = DEFAULT_SCLASS as uint8_t;
        sclass_incref((*root).sclassid as uint16_t, (*root).r#type() as uint8_t);
        (*root).trashretention = DEFAULT_TRASHTIME as uint16_t;
        (*root).eattr = 0 as uint8_t;
        (*root).winattr = 0 as uint8_t;
        (*root).set_mode(0o777 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*root).uid = 0 as uint32_t;
        (*root).gid = 0 as uint32_t;
        memset(
            &raw mut (*root).data.ddata.stats as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<statsrecord>(),
        );
        (*root).data.ddata.quota = ::core::ptr::null_mut::<quotanode>();
        (*root).data.ddata.children = ::core::ptr::null_mut::<fsedge>();
        (*root).data.ddata.elements = 0 as uint32_t;
        (*root).data.ddata.nlink = 2 as uint32_t;
        (*root).parents = ::core::ptr::null_mut::<fsedge>();
        fsnodes_node_add(root);
        fsnodes_used_inode((*root).inode);
        nodes = 1 as uint32_t;
        dirnodes = 1 as uint32_t;
        filenodes = 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_set_root_times(mut ts: uint32_t) -> ::core::ffi::c_int {
    unsafe {
        root = fsnodes_node_find(MFS_ROOT_ID as uint32_t);
        if root.is_null() {
            return -1 as ::core::ffi::c_int;
        }
        (*root).atime = ts;
        (*root).mtime = (*root).atime;
        (*root).ctime = (*root).mtime;
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_printinfo() {
    unsafe {
        fprintf(
            stderr,
            b"all inodes: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            nodes,
        );
        fprintf(
            stderr,
            b"directory inodes: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            dirnodes,
        );
        fprintf(
            stderr,
            b"file inodes: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            filenodes,
        );
        fprintf(
            stderr,
            b"chunks: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            chunk_count(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_afterload() {
    unsafe {
        fprintf(
            stderr,
            b"connecting files and chunks ... \0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        fs_add_files_to_chunks();
        fprintf(stderr, b"ok\n\0".as_ptr() as *const ::core::ffi::c_char);
        fs_printinfo();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_check_consistency(
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut profdata: ::core::ffi::c_double = 0.;
        root = fsnodes_node_find(MFS_ROOT_ID as uint32_t);
        if root.is_null() {
            fprintf(
                stderr,
                b"root node not found !!!\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error reading metadata (no root)\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if ignoreflag == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        profdata = monotonic_seconds();
        fprintf(
            stderr,
            b"checking filesystem consistency ... \0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        if fs_checknodes() < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        profdata = monotonic_seconds() - profdata;
        fprintf(
            stderr,
            b"ok (%.4lf)\n\0".as_ptr() as *const ::core::ffi::c_char,
            profdata,
        );
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_reload() {
    unsafe {
        if cfg_isdefined(b"QUOTA_TIME_LIMIT\0".as_ptr() as *const ::core::ffi::c_char) != 0
            && cfg_isdefined(b"QUOTA_DEFAULT_GRACE_PERIOD\0".as_ptr() as *const ::core::ffi::c_char)
                == 0
        {
            QuotaDefaultGracePeriod = cfg_getuint32(
                b"QUOTA_TIME_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
                (7 as ::core::ffi::c_int * 86400 as ::core::ffi::c_int) as uint32_t,
            );
        } else {
            QuotaDefaultGracePeriod = cfg_getsperiod(
                b"QUOTA_DEFAULT_GRACE_PERIOD\0".as_ptr() as *const ::core::ffi::c_char,
                b"1w\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        AtimeMode = cfg_getuint8(
            b"ATIME_MODE\0".as_ptr() as *const ::core::ffi::c_char,
            2 as uint8_t,
        );
        if AtimeMode as ::core::ffi::c_int > ATIME_NEVER {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"unrecognized value for ATIME_MODE - using defaults\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            AtimeMode = 0 as uint8_t;
        }
        KeepEmptyFilesInTrash = cfg_getuint8(
            b"KEEP_EMPTY_FILES_IN_TRASH\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint8_t,
        );
        MaxAllowedHardLinks = cfg_getuint32(
            b"MAX_ALLOWED_HARD_LINKS\0".as_ptr() as *const ::core::ffi::c_char,
            32767 as uint32_t,
        ) as uint16_t;
        if (MaxAllowedHardLinks as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MAX_ALLOWED_HARD_LINKS is lower than 8 - less than minimum number of hard links required by POSIX - setting to 8\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            MaxAllowedHardLinks = 8 as uint16_t;
        }
        if MaxAllowedHardLinks as ::core::ffi::c_int > 65000 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MAX_ALLOWED_HARD_LINKS is higher than 65000 - setting to 65000\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            MaxAllowedHardLinks = 65000 as uint16_t;
        }
        InodeReuseDelay = cfg_getsperiod(
            b"INODE_REUSE_DELAY\0".as_ptr() as *const ::core::ffi::c_char,
            b"1d\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if InodeReuseDelay < 300 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"INODE_REUSE_DELAY is lower than 300 - setting to 300\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            InodeReuseDelay = 300 as uint32_t;
        }
        if InodeReuseDelay > 3000000 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"INODE_REUSE_DELAY is higher than 3000000 - setting to 3000000\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            InodeReuseDelay = 3000000 as uint32_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_strinit() -> ::core::ffi::c_int {
    unsafe {
        let mut bid: uint32_t = 0;
        trash_bid = 0 as uint32_t;
        sustained_bid = 0 as uint32_t;
        root = ::core::ptr::null_mut::<fsnode>();
        bid = 0 as uint32_t;
        while bid < TRASH_BUCKETS as uint32_t {
            trash[bid as usize] = ::core::ptr::null_mut::<fsedge>();
            bid = bid.wrapping_add(1);
        }
        bid = 0 as uint32_t;
        while bid < SUSTAINED_BUCKETS as uint32_t {
            sustained[bid as usize] = ::core::ptr::null_mut::<fsedge>();
            bid = bid.wrapping_add(1);
        }
        trashspace = 0 as uint64_t;
        sustainedspace = 0 as uint64_t;
        trashnodes = 0 as uint32_t;
        sustainednodes = 0 as uint32_t;
        quotahead = ::core::ptr::null_mut::<quotanode>();
        freelist = ::core::ptr::null_mut::<freenode>();
        freetail = &raw mut freelist;
        freelastts = 0 as uint32_t;
        fsnodes_edgeid_init();
        fsnodes_node_hash_init();
        fsnodes_edge_hash_init();
        fsnode_init();
        fsedge_init();
        symlink_init();
        chunktab_init();
        appendres_init();
        fs_reload();
        snapshot_inodehash = chash_new();
        main_reload_register_fname(
            Some(fs_reload as unsafe extern "C" fn() -> ()),
            b"fs_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_msectime_register_fname(
            100 as uint32_t,
            0 as uint32_t,
            Some(fs_test_files as unsafe extern "C" fn() -> ()),
            b"fs_test_files\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(fsnodes_check_all_quotas as unsafe extern "C" fn() -> ()),
            b"fsnodes_check_all_quotas\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(fs_emptytrash as unsafe extern "C" fn() -> ()),
            b"fs_emptytrash\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(fs_emptysustained as unsafe extern "C" fn() -> ()),
            b"fs_emptysustained\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            60 as uint32_t,
            0 as uint32_t,
            Some(fsnodes_freeinodes as unsafe extern "C" fn() -> ()),
            b"fsnodes_freeinodes\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
