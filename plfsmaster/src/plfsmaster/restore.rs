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
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn shp_get(vs: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    unsafe fn shp_inc(vs: *mut ::core::ffi::c_void);
    unsafe fn shp_dec(vs: *mut ::core::ffi::c_void);
    unsafe fn fs_mr_access(ts: uint32_t, inode: uint32_t) -> uint8_t;
    unsafe fn fs_mr_append_slice(
        ts: uint32_t,
        inode: uint32_t,
        inode_src: uint32_t,
        slice_from: uint32_t,
        slice_to: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_attr(
        ts: uint32_t,
        inode: uint32_t,
        mode: uint16_t,
        uid: uint32_t,
        gid: uint32_t,
        atime: uint32_t,
        mtime: uint32_t,
        winattr: uint8_t,
        attrmode: uint16_t,
    ) -> uint8_t;
    unsafe fn fs_mr_amtime(
        inode: uint32_t,
        ts: uint32_t,
        atime: uint32_t,
        mtime: uint32_t,
        ctime: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_create(
        ts: uint32_t,
        parent: uint32_t,
        nleng: uint32_t,
        name: *const uint8_t,
        r#type: uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gid: uint32_t,
        rdev: uint32_t,
        inode: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_emptytrash(
        ts: uint32_t,
        bid: uint32_t,
        freeinodes: uint32_t,
        sustainedinodes: uint32_t,
        trashflaginodes: uint32_t,
        inode_chksum: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_emptysustained(
        ts: uint32_t,
        bid: uint32_t,
        freeinodes: uint32_t,
        inode_chksum: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_freeinodes(
        ts: uint32_t,
        inodereusedelay: uint32_t,
        freeinodes: uint32_t,
        sustainedinodes: uint32_t,
        inode_chksum: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_link(
        ts: uint32_t,
        inode_src: uint32_t,
        parent_dst: uint32_t,
        nleng_dst: uint32_t,
        name_dst: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mr_length(
        ts: uint32_t,
        inode: uint32_t,
        length: uint64_t,
        canmodmtime: uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mr_move(
        ts: uint32_t,
        parent_src: uint32_t,
        nleng_src: uint32_t,
        name_src: *const uint8_t,
        parent_dst: uint32_t,
        nleng_dst: uint32_t,
        name_dst: *const uint8_t,
        rmode: uint8_t,
        inode: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_repair(
        ts: uint32_t,
        inode: uint32_t,
        indx: uint32_t,
        nversion: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_symlink(
        ts: uint32_t,
        parent: uint32_t,
        nleng: uint32_t,
        name: *const uint8_t,
        path: *const uint8_t,
        uid: uint32_t,
        gid: uint32_t,
        inode: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_setpath(inode: uint32_t, path: *const uint8_t) -> uint8_t;
    unsafe fn fs_mr_snapshot(
        ts: uint32_t,
        inode_src: uint32_t,
        parent_dst: uint32_t,
        nleng_dst: uint16_t,
        name_dst: *mut uint8_t,
        smode: uint8_t,
        sesflags: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        cumask: uint16_t,
        inodecheck: uint32_t,
        removed: uint32_t,
        same: uint32_t,
        existing: uint32_t,
        hardlinks: uint32_t,
        new: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_unlink(
        ts: uint32_t,
        parent: uint32_t,
        nleng: uint32_t,
        name: *const uint8_t,
        inode: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_trash_recover(
        ts: uint32_t,
        inode: uint32_t,
        parent_dst: uint32_t,
        pleng_dst: uint32_t,
        path_dst: *const uint8_t,
        cumask: uint16_t,
        uid: uint32_t,
        gid: uint32_t,
        copysgid: uint8_t,
        created_pleng: uint32_t,
        created_path: *const uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mr_trash_remove(ts: uint32_t, inode: uint32_t) -> uint8_t;
    unsafe fn fs_mr_purge(ts: uint32_t, inode: uint32_t) -> uint8_t;
    unsafe fn fs_mr_undel(ts: uint32_t, inode: uint32_t) -> uint8_t;
    unsafe fn fs_mr_trunc(
        ts: uint32_t,
        inode: uint32_t,
        indx: uint32_t,
        chunkid: uint64_t,
    ) -> uint8_t;
    unsafe fn fs_mr_write(
        ts: uint32_t,
        inode: uint32_t,
        indx: uint32_t,
        opflag: uint8_t,
        canmodmtime: uint8_t,
        chunkid: uint64_t,
    ) -> uint8_t;
    unsafe fn fs_mr_rollback(
        ts: uint32_t,
        inode: uint32_t,
        indx: uint32_t,
        prevchunkid: uint64_t,
        chunkid: uint64_t,
    ) -> uint8_t;
    unsafe fn fs_mr_unlock(ts: uint32_t, chunkid: uint64_t) -> uint8_t;
    unsafe fn fs_mr_setsclass(
        ts: uint32_t,
        inode: uint32_t,
        uid: uint32_t,
        src_sclassid: uint8_t,
        dst_sclassid: uint8_t,
        smode: uint8_t,
        sinodes: uint32_t,
        ncinodes: uint32_t,
        nsinodes: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_settrashretention(
        ts: uint32_t,
        inode: uint32_t,
        uid: uint32_t,
        trashretention: uint32_t,
        smode: uint8_t,
        sinodes: uint32_t,
        ncinodes: uint32_t,
        nsinodes: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_seteattr(
        ts: uint32_t,
        inode: uint32_t,
        uid: uint32_t,
        eattr: uint8_t,
        smode: uint8_t,
        sinodes: uint32_t,
        ncinodes: uint32_t,
        nsinodes: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_setxattr(
        ts: uint32_t,
        inode: uint32_t,
        anleng: uint32_t,
        attrname: *const uint8_t,
        avleng: uint32_t,
        attrvalue: *const uint8_t,
        mode: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_setacl(
        ts: uint32_t,
        inode: uint32_t,
        mode: uint16_t,
        changectime: uint8_t,
        acltype: uint8_t,
        userperm: uint16_t,
        groupperm: uint16_t,
        otherperm: uint16_t,
        mask: uint16_t,
        namedusers: uint16_t,
        namedgroups: uint16_t,
        aclblob: *const uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mr_quota(
        ts: uint32_t,
        inode: uint32_t,
        exceeded: uint8_t,
        flags: uint8_t,
        stimestamp: uint32_t,
        sinodes: uint32_t,
        hinodes: uint32_t,
        slength: uint64_t,
        hlength: uint64_t,
        ssize: uint64_t,
        hsize: uint64_t,
        srealsize: uint64_t,
        hrealsize: uint64_t,
        graceperiod: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_archchg(
        ts: uint32_t,
        inode: uint32_t,
        uid: uint32_t,
        flags: uint8_t,
        chgchunks: uint64_t,
        notchgchunks: uint64_t,
        nsinodes: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_set_file_chunk(inode: uint32_t, indx: uint32_t, chunkdid: uint64_t) -> uint8_t;
    unsafe fn fs_mr_autoarch(
        inode: uint32_t,
        archreftime: uint32_t,
        intrash: uint8_t,
        archchgchunks: uint32_t,
        trashchgchunks: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_additionalattr(
        ts: uint32_t,
        inode: uint32_t,
        flags: uint8_t,
        data: *const uint8_t,
        leng: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_mr_renumerate_edges(expected_nextedgeid: uint64_t) -> uint8_t;
    unsafe fn sessions_mr_seschanged(
        sessionid: uint32_t,
        exportscsum: uint64_t,
        rootinode: uint32_t,
        sesflags: uint8_t,
        umaskval: uint16_t,
        rootuid: uint32_t,
        rootgid: uint32_t,
        mapalluid: uint32_t,
        mapallgid: uint32_t,
        sclassgroups: uint16_t,
        mintrashretention: uint32_t,
        maxtrashretention: uint32_t,
        disables: uint32_t,
        peerip: uint32_t,
        info: *const uint8_t,
        ileng: uint32_t,
    ) -> uint8_t;
    unsafe fn sessions_mr_sesadd(
        exportscsum: uint64_t,
        rootinode: uint32_t,
        sesflags: uint8_t,
        umaskval: uint16_t,
        rootuid: uint32_t,
        rootgid: uint32_t,
        mapalluid: uint32_t,
        mapallgid: uint32_t,
        sclassgroups: uint16_t,
        mintrashretention: uint32_t,
        maxtrashretention: uint32_t,
        disables: uint32_t,
        peerip: uint32_t,
        info: *const uint8_t,
        ileng: uint32_t,
        sessionid: uint32_t,
    ) -> uint8_t;
    unsafe fn sessions_mr_sesdel(sessionid: uint32_t) -> uint8_t;
    unsafe fn sessions_mr_connected(sessionid: uint32_t) -> uint8_t;
    unsafe fn sessions_mr_disconnected(sessionid: uint32_t, disctime: uint32_t) -> uint8_t;
    unsafe fn sessions_mr_session(sessionid: uint32_t) -> uint8_t;
    unsafe fn of_mr_acquire(sessionid: uint32_t, inode: uint32_t) -> ::core::ffi::c_int;
    unsafe fn of_mr_release(sessionid: uint32_t, inode: uint32_t) -> ::core::ffi::c_int;
    unsafe fn flock_mr_change(
        inode: uint32_t,
        sessionid: uint32_t,
        owner: uint64_t,
        cmd: ::core::ffi::c_char,
    ) -> uint8_t;
    unsafe fn posix_lock_mr_change(
        inode: uint32_t,
        sessionid: uint32_t,
        owner: uint64_t,
        cmd: ::core::ffi::c_char,
        start: uint64_t,
        end: uint64_t,
        pid: uint32_t,
    ) -> uint8_t;
    unsafe fn csdb_mr_op(csop: uint8_t, ip: uint32_t, port: uint16_t, arg: uint32_t) -> uint8_t;
    unsafe fn sclass_maskorgroup_to_labelexpr(
        labelexpr: *mut [uint8_t; 128],
        labelmasks: *mut uint32_t,
        labelscnt: uint8_t,
    );
    unsafe fn sclass_mr_ec_version(ec_new_version: uint8_t) -> uint8_t;
    unsafe fn sclass_mr_set_entry(
        nleng: uint8_t,
        name: *const uint8_t,
        esclassid: uint16_t,
        new_flag: uint8_t,
        dleng: uint8_t,
        desc: *const uint8_t,
        priority: uint32_t,
        export_group: uint8_t,
        admin_only: uint8_t,
        labels_mode: uint8_t,
        arch_mode: uint8_t,
        arch_delay: uint16_t,
        arch_min_size: uint64_t,
        min_trashretention: uint16_t,
        create: *mut storagemode,
        keep: *mut storagemode,
        arch: *mut storagemode,
        trash: *mut storagemode,
    ) -> uint8_t;
    unsafe fn sclass_mr_duplicate_entry(
        oldnleng: uint8_t,
        oldname: *const uint8_t,
        newnleng: uint8_t,
        newname: *const uint8_t,
        essclassid: uint16_t,
        edsclassid: uint16_t,
    ) -> uint8_t;
    unsafe fn sclass_mr_rename_entry(
        oldnleng: uint8_t,
        oldname: *const uint8_t,
        newnleng: uint8_t,
        newname: *const uint8_t,
        esclassid: uint16_t,
    ) -> uint8_t;
    unsafe fn sclass_mr_delete_entry(
        nleng: uint8_t,
        name: *const uint8_t,
        esclassid: uint16_t,
    ) -> uint8_t;
    unsafe fn chunk_mr_increase_version(chunkid: uint64_t) -> ::core::ffi::c_int;
    unsafe fn chunk_mr_set_version(chunkid: uint64_t, version: uint32_t) -> ::core::ffi::c_int;
    unsafe fn chunk_mr_nextchunkid(nchunkid: uint64_t) -> ::core::ffi::c_int;
    unsafe fn chunk_mr_chunkadd(
        ts: uint32_t,
        chunkid: uint64_t,
        version: uint32_t,
        lockedto: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_mr_chunkdel(
        ts: uint32_t,
        chunkid: uint64_t,
        version: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn chunk_mr_flagsclr(ts: uint32_t, chunkid: uint64_t) -> ::core::ffi::c_int;
    unsafe fn patterns_mr_add(
        gnleng: uint8_t,
        gname: *const uint8_t,
        euid: uint32_t,
        egid: uint32_t,
        priority: uint8_t,
        omask: uint8_t,
        scid: uint8_t,
        trashretention: uint16_t,
        seteattr: uint8_t,
        clreattr: uint8_t,
    ) -> uint8_t;
    unsafe fn patterns_mr_delete(
        gnleng: uint8_t,
        gname: *const uint8_t,
        euid: uint32_t,
        egid: uint32_t,
    ) -> uint8_t;
    unsafe fn meta_version_inc() -> uint64_t;
    unsafe fn meta_version() -> uint64_t;
    unsafe fn meta_mr_setmetaid(newmetaid: uint64_t) -> uint8_t;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _storagemode {
    pub uniqmask: uint32_t,
    pub ec_data_chksum_parts: uint8_t,
    pub has_labels: uint8_t,
    pub matching_servers: uint8_t,
    pub valid_ec_counters: uint8_t,
    pub replallowed: uint16_t,
    pub overloaded: uint16_t,
    pub allvalid: uint16_t,
    pub data_replallowed: uint16_t,
    pub data_overloaded: uint16_t,
    pub data_allvalid: uint16_t,
    pub chksum_replallowed: uint16_t,
    pub chksum_overloaded: uint16_t,
    pub chksum_allvalid: uint16_t,
    pub both_replallowed: uint16_t,
    pub both_overloaded: uint16_t,
    pub both_allvalid: uint16_t,
    pub labels_mode: uint8_t,
    pub labelscnt: uint8_t,
    pub labelexpr: [[uint8_t; 128]; 9],
}
pub type storagemode = _storagemode;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MASKORGROUP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCLASS_EXPR_MAX_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MAXLABELSCNT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_ERROR_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_CTIME: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LABELS_MODE_GLOBAL: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mfsstrerr(mut status: uint8_t) -> *const ::core::ffi::c_char {
    unsafe {
        static mut errtab: [*const ::core::ffi::c_char; 65] = [
            b"OK\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not permitted\0".as_ptr() as *const ::core::ffi::c_char,
            b"Not a directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such file or directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Permission denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"File exists\0".as_ptr() as *const ::core::ffi::c_char,
            b"Invalid argument\0".as_ptr() as *const ::core::ffi::c_char,
            b"Directory not empty\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk lost\0".as_ptr() as *const ::core::ffi::c_char,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Index too big\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk locked\0".as_ptr() as *const ::core::ffi::c_char,
            b"No chunk servers\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such chunk\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk is busy\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect register BLOB\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not completed\0".as_ptr() as *const ::core::ffi::c_char,
            b"File not opened\0".as_ptr() as *const ::core::ffi::c_char,
            b"Write not started\0".as_ptr() as *const ::core::ffi::c_char,
            b"Wrong chunk version\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk already exists\0".as_ptr() as *const ::core::ffi::c_char,
            b"No space left\0".as_ptr() as *const ::core::ffi::c_char,
            b"IO error\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect block number\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect size\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect offset\0".as_ptr() as *const ::core::ffi::c_char,
            b"Can't connect\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect chunk id\0".as_ptr() as *const ::core::ffi::c_char,
            b"Disconnected\0".as_ptr() as *const ::core::ffi::c_char,
            b"CRC error\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation delayed\0".as_ptr() as *const ::core::ffi::c_char,
            b"Can't create path\0".as_ptr() as *const ::core::ffi::c_char,
            b"Data mismatch\0".as_ptr() as *const ::core::ffi::c_char,
            b"Read-only file system\0".as_ptr() as *const ::core::ffi::c_char,
            b"Quota exceeded\0".as_ptr() as *const ::core::ffi::c_char,
            b"Bad session id\0".as_ptr() as *const ::core::ffi::c_char,
            b"Password is needed\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect password\0".as_ptr() as *const ::core::ffi::c_char,
            b"Attribute not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not supported\0".as_ptr() as *const ::core::ffi::c_char,
            b"Result too large\0".as_ptr() as *const ::core::ffi::c_char,
            b"Entity not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"Entity is active\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunkserver not present\0".as_ptr() as *const ::core::ffi::c_char,
            b"Waiting on lock\0".as_ptr() as *const ::core::ffi::c_char,
            b"Resource temporarily unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            b"Interrupted system call\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation canceled\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such file or directory (not cacheable)\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not permitted (mfs admin only)\0".as_ptr() as *const ::core::ffi::c_char,
            b"Class name already in use\0".as_ptr() as *const ::core::ffi::c_char,
            b"Maximum number of classes reached\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such class\0".as_ptr() as *const ::core::ffi::c_char,
            b"Class in use\0".as_ptr() as *const ::core::ffi::c_char,
            b"One of MFS instance components is too old to perform this operation\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"Pattern already defined\0".as_ptr() as *const ::core::ffi::c_char,
            b"Maximum number of patterns reached\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such pattern\0".as_ptr() as *const ::core::ffi::c_char,
            b"File name too long\0".as_ptr() as *const ::core::ffi::c_char,
            b"Too many links\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation timed out\0".as_ptr() as *const ::core::ffi::c_char,
            b"Bad file descriptor\0".as_ptr() as *const ::core::ffi::c_char,
            b"File too large\0".as_ptr() as *const ::core::ffi::c_char,
            b"Is a directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Unknown MFS error\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        if status as ::core::ffi::c_int > MFS_ERROR_MAX {
            status = MFS_ERROR_MAX as uint8_t;
        }
        return errtab[status as usize];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_idle(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        meta_version_inc();
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_access(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_access(ts, inode) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_addattr(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut attrblobleng: uint32_t = 0;
        let mut flags: uint8_t = 0;
        static mut attrblob: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut attrblobsize: uint32_t = 0 as uint32_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if tmp > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        flags = tmp as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        attrblobleng = 0 as uint32_t;
        loop {
            let c2rust_fresh0 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh0;
            if _tmp_c as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh1 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh1;
                let c2rust_fresh2 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh2;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if attrblobleng >= attrblobsize {
                attrblobsize = attrblobleng.wrapping_add(1000 as uint32_t);
                if attrblob.is_null() {
                    attrblob = malloc(attrblobsize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_buff: *mut uint8_t = attrblob;
                    attrblob = realloc(attrblob as *mut ::core::ffi::c_void, attrblobsize as size_t)
                        as *mut uint8_t;
                    if attrblob.is_null() {
                        free(_tmp_buff as *mut ::core::ffi::c_void);
                    }
                }
                if attrblob.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        360 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"attrblob\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        360 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"attrblob\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if attrblob
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        360 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"attrblob\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        360 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"attrblob\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            let c2rust_fresh3 = attrblobleng;
            attrblobleng = attrblobleng.wrapping_add(1);
            *attrblob.offset(c2rust_fresh3 as isize) = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_additionalattr(ts, inode, flags, attrblob, attrblobleng)
            as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_append(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut inode_src: uint32_t = 0;
        let mut slice_from: uint32_t = 0;
        let mut slice_to: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode_src = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
            slice_from = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            slice_to = 0 as uint32_t;
        } else {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            slice_from = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_1 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            slice_to = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_2 as *const ::core::ffi::c_char;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_append_slice(ts, inode, inode_src, slice_from, slice_to)
            as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_acquire(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut cuid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        cuid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return of_mr_acquire(inode, cuid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_archchg(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut nsinodes: uint32_t = 0;
        let mut flags: uint8_t = 0;
        let mut chgchunks: uint64_t = 0;
        let mut notchgchunks: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if tmp > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        flags = tmp as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chgchunks = strtoull(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        notchgchunks = strtoull(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        nsinodes = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        return fs_mr_archchg(ts, inode, uid, flags, chgchunks, notchgchunks, nsinodes)
            as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_amtime(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut xatime: uint32_t = 0;
        let mut xmtime: uint32_t = 0;
        let mut xctime: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        xatime = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        xmtime = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        xctime = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_amtime(inode, ts, xatime, xmtime, xctime) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_autoarch(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut archreftime: uint32_t = 0;
        let mut archchgchunks: uint32_t = 0;
        let mut trashchgchunks: uint32_t = 0;
        let mut intrash: uint8_t = 0;
        let mut format: uint8_t = 0;
        format = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        archreftime = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            format = 1 as uint8_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp: uint32_t = 0;
            let mut eptr_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_1 as *const ::core::ffi::c_char;
            if tmp > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp,
                );
                return -1 as ::core::ffi::c_int;
            }
            intrash = tmp as uint8_t;
        } else {
            intrash = 1 as uint8_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        archchgchunks = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if format as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_3: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            trashchgchunks = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_3 as *const ::core::ffi::c_char;
        } else {
            trashchgchunks = 0 as uint32_t;
        }
        return fs_mr_autoarch(inode, archreftime, intrash, archchgchunks, trashchgchunks)
            as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_attr(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut mode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gid: uint32_t = 0;
        let mut atime: uint32_t = 0;
        let mut mtime: uint32_t = 0;
        let mut winattr: uint32_t = 0;
        let mut aclmode: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mode = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        gid = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        atime = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mtime = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_5: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            aclmode = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_5 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                winattr = aclmode;
                let mut eptr_6: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                aclmode = strtoul(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_6 as *const ::core::ffi::c_char;
            } else {
                winattr = 0 as uint32_t;
            }
        } else {
            aclmode = mode;
            winattr = 0 as uint32_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_attr(
            ts,
            inode,
            mode as uint16_t,
            uid,
            gid,
            atime,
            mtime,
            winattr as uint8_t,
            aclmode as uint16_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_create(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gid: uint32_t = 0;
        let mut rdev: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut mode: uint16_t = 0;
        let mut cumask: uint16_t = 0;
        let mut r#type: uint8_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        parent = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh4 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh4;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh5 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh5;
                let c2rust_fresh6 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh6;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh7 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name[c2rust_fresh7 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            let mut tmp: uint32_t = 0;
            let mut eptr_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_0 as *const ::core::ffi::c_char;
            if tmp > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp,
                );
                return -1 as ::core::ffi::c_int;
            }
            r#type = tmp as uint8_t;
        } else {
            r#type = *ptr as uint8_t;
            ptr = ptr.offset(1);
        }
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_0: uint32_t = 0;
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_0 = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if tmp_0 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_0,
            );
            return -1 as ::core::ffi::c_int;
        }
        mode = tmp_0 as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if (r#type as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
            let mut tmp_1: uint32_t = 0;
            let mut eptr_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_1 = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_2 as *const ::core::ffi::c_char;
            if tmp_1 > 65535 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-65535 expected)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    tmp_1,
                );
                return -1 as ::core::ffi::c_int;
            }
            cumask = tmp_1 as uint16_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        } else {
            cumask = 0 as uint16_t;
        }
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        gid = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rdev = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_6: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_6 as *const ::core::ffi::c_char;
        return fs_mr_create(
            ts,
            parent,
            strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint32_t,
            &raw mut name as *mut uint8_t,
            r#type,
            mode,
            cumask,
            uid,
            gid,
            rdev,
            inode,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_csdbop(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut op: uint32_t = 0;
        let mut ip: uint32_t = 0;
        let mut port: uint32_t = 0;
        let mut arg: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if tmp > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        op = tmp;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ip = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_0: uint32_t = 0;
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_0 = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if tmp_0 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_0,
            );
            return -1 as ::core::ffi::c_int;
        }
        port = tmp_0;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        arg = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return csdb_mr_op(op as uint8_t, ip, port as uint16_t, arg) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_csadd(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ip: uint32_t = 0;
        let mut port: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ip = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        port = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return csdb_mr_op(0 as uint8_t, ip, port as uint16_t, 0 as uint32_t) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_csdel(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ip: uint32_t = 0;
        let mut port: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ip = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        port = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return csdb_mr_op(1 as uint8_t, ip, port as uint16_t, 0 as uint32_t) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_chunkadd(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut lockedto: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        version = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        lockedto = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return chunk_mr_chunkadd(ts, chunkid, version, lockedto);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_chunkdel(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        version = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return chunk_mr_chunkdel(ts, chunkid, version);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_chunkflagsclr(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut chunkid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return chunk_mr_flagsclr(ts, chunkid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_emptytrash(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sustainedinodes: uint32_t = 0;
        let mut freeinodes: uint32_t = 0;
        let mut trashflaginodes: uint32_t = 0;
        let mut inode_chksum: uint32_t = 0;
        let mut bid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            bid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr as *const ::core::ffi::c_char;
        } else {
            bid = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        freeinodes = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sustainedinodes = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            inode_chksum = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_2 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                trashflaginodes = inode_chksum;
                let mut eptr_3: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                inode_chksum = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_3 as *const ::core::ffi::c_char;
            } else {
                trashflaginodes = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            }
        } else {
            inode_chksum = 0 as uint32_t;
            trashflaginodes = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        }
        return fs_mr_emptytrash(
            ts,
            bid,
            freeinodes,
            sustainedinodes,
            trashflaginodes,
            inode_chksum,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_emptysustained(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut freeinodes: uint32_t = 0;
        let mut inode_chksum: uint32_t = 0;
        let mut bid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            bid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr as *const ::core::ffi::c_char;
        } else {
            bid = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        freeinodes = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            inode_chksum = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_1 as *const ::core::ffi::c_char;
        } else {
            inode_chksum = 0 as uint32_t;
        }
        return fs_mr_emptysustained(ts, bid, freeinodes, inode_chksum) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_flock(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut owner: uint64_t = 0;
        let mut cmd: ::core::ffi::c_char = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sessionid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        owner = strtoull(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        cmd = *ptr;
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return flock_mr_change(inode, sessionid, owner, cmd) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_freeinodes(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sustainedinodes: uint32_t = 0;
        let mut freeinodes: uint32_t = 0;
        let mut inode_chksum: uint32_t = 0;
        let mut inodereusedelay: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
            inodereusedelay = 86400 as uint32_t;
        } else {
            let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            inodereusedelay = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr as *const ::core::ffi::c_char;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        freeinodes = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sustainedinodes = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_1 as *const ::core::ffi::c_char;
        } else {
            sustainedinodes = 0 as uint32_t;
        }
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            inode_chksum = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_2 as *const ::core::ffi::c_char;
        } else {
            inode_chksum = 0 as uint32_t;
        }
        return fs_mr_freeinodes(
            ts,
            inodereusedelay,
            freeinodes,
            sustainedinodes,
            inode_chksum,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_incversion(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut chunkid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return chunk_mr_increase_version(chunkid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_setversion(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        version = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return chunk_mr_set_version(chunkid, version);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_link(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut parent: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        parent = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh8 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh8;
            if !(_tmp_c as ::core::ffi::c_int != ')' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh9 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh9;
                let c2rust_fresh10 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh10;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh11 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name[c2rust_fresh11 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_link(
            ts,
            inode,
            parent,
            strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint32_t,
            &raw mut name as *mut uint8_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_length(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut length: uint64_t = 0;
        let mut canmodmtime: uint8_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        length = strtoull(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp: uint32_t = 0;
            let mut eptr_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_1 as *const ::core::ffi::c_char;
            if tmp > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp,
                );
                return -1 as ::core::ffi::c_int;
            }
            canmodmtime = tmp as uint8_t;
        } else {
            canmodmtime = 1 as uint8_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_length(ts, inode, length, canmodmtime) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_move(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut parent_src: uint32_t = 0;
        let mut parent_dst: uint32_t = 0;
        let mut name_src: [uint8_t; 256] = [0; 256];
        let mut name_dst: [uint8_t; 256] = [0; 256];
        let mut rmode: uint8_t = 0;
        let mut s: ::core::ffi::c_char = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        parent_src = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name_src as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh12 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh12;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh13 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh13;
                let c2rust_fresh14 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh14;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh15 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name_src[c2rust_fresh15 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name_src[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        parent_dst = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_clptr: *const ::core::ffi::c_char = ptr;
        let mut _tmp_c_0: ::core::ffi::c_char = 0;
        s = '#' as ::core::ffi::c_char;
        while s as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
            _tmp_c_0 = *_tmp_clptr;
            _tmp_clptr = _tmp_clptr.offset(1);
            if (_tmp_c_0 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int >= 127 as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '(' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == ')' as ::core::ffi::c_int
            {
                s = _tmp_c_0;
            }
        }
        if s as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            let mut _tmp_i_0: uint32_t = 0;
            let mut _tmp_c_1: ::core::ffi::c_char = 0;
            let mut _tmp_h1_0: ::core::ffi::c_char = 0;
            let mut _tmp_h2_0: ::core::ffi::c_char = 0;
            memset(
                &raw mut name_dst as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                256 as size_t,
            );
            _tmp_i_0 = 0 as uint32_t;
            loop {
                let c2rust_fresh16 = ptr;
                ptr = ptr.offset(1);
                _tmp_c_1 = *c2rust_fresh16;
                if !(_tmp_c_1 as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                    && _tmp_i_0 < 255 as uint32_t)
                {
                    break;
                }
                if _tmp_c_1 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                    || _tmp_c_1 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                    || _tmp_c_1 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_c_1 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                    let c2rust_fresh17 = ptr;
                    ptr = ptr.offset(1);
                    _tmp_h1_0 = *c2rust_fresh17;
                    let c2rust_fresh18 = ptr;
                    ptr = ptr.offset(1);
                    _tmp_h2_0 = *c2rust_fresh18;
                    if _tmp_h1_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && _tmp_h1_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    } else if _tmp_h1_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                        && _tmp_h1_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                    {
                        _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                            - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                            as ::core::ffi::c_char;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    if _tmp_h2_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && _tmp_h2_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    } else if _tmp_h2_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                        && _tmp_h2_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                    {
                        _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                            - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                            as ::core::ffi::c_char;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    _tmp_c_1 = (_tmp_h1_0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                        + _tmp_h2_0 as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                }
                let c2rust_fresh19 = _tmp_i_0;
                _tmp_i_0 = _tmp_i_0.wrapping_add(1);
                name_dst[c2rust_fresh19 as usize] = _tmp_c_1 as uint8_t;
            }
            ptr = ptr.offset(-1);
            name_dst[_tmp_i_0 as usize] = 0 as uint8_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp: uint32_t = 0;
            let mut eptr_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_1 as *const ::core::ffi::c_char;
            if tmp > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp,
                );
                return -1 as ::core::ffi::c_int;
            }
            rmode = tmp as uint8_t;
        } else {
            let mut _tmp_i_1: uint32_t = 0;
            let mut _tmp_c_2: ::core::ffi::c_char = 0;
            let mut _tmp_h1_1: ::core::ffi::c_char = 0;
            let mut _tmp_h2_1: ::core::ffi::c_char = 0;
            memset(
                &raw mut name_dst as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                256 as size_t,
            );
            _tmp_i_1 = 0 as uint32_t;
            loop {
                let c2rust_fresh20 = ptr;
                ptr = ptr.offset(1);
                _tmp_c_2 = *c2rust_fresh20;
                if !(_tmp_c_2 as ::core::ffi::c_int != ')' as ::core::ffi::c_int
                    && _tmp_i_1 < 255 as uint32_t)
                {
                    break;
                }
                if _tmp_c_2 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                    || _tmp_c_2 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                    || _tmp_c_2 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ')' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_c_2 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                    let c2rust_fresh21 = ptr;
                    ptr = ptr.offset(1);
                    _tmp_h1_1 = *c2rust_fresh21;
                    let c2rust_fresh22 = ptr;
                    ptr = ptr.offset(1);
                    _tmp_h2_1 = *c2rust_fresh22;
                    if _tmp_h1_1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && _tmp_h1_1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        _tmp_h1_1 = (_tmp_h1_1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    } else if _tmp_h1_1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                        && _tmp_h1_1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                    {
                        _tmp_h1_1 = (_tmp_h1_1 as ::core::ffi::c_int
                            - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                            as ::core::ffi::c_char;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    if _tmp_h2_1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && _tmp_h2_1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        _tmp_h2_1 = (_tmp_h2_1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    } else if _tmp_h2_1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                        && _tmp_h2_1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                    {
                        _tmp_h2_1 = (_tmp_h2_1 as ::core::ffi::c_int
                            - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                            as ::core::ffi::c_char;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    _tmp_c_2 = (_tmp_h1_1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                        + _tmp_h2_1 as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                }
                let c2rust_fresh23 = _tmp_i_1;
                _tmp_i_1 = _tmp_i_1.wrapping_add(1);
                name_dst[c2rust_fresh23 as usize] = _tmp_c_2 as uint8_t;
            }
            ptr = ptr.offset(-1);
            name_dst[_tmp_i_1 as usize] = 0 as uint8_t;
            rmode = 0 as uint8_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        return fs_mr_move(
            ts,
            parent_src,
            strlen(&raw mut name_src as *mut uint8_t as *mut ::core::ffi::c_char) as uint32_t,
            &raw mut name_src as *mut uint8_t,
            parent_dst,
            strlen(&raw mut name_dst as *mut uint8_t as *mut ::core::ffi::c_char) as uint32_t,
            &raw mut name_dst as *mut uint8_t,
            rmode,
            inode,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_nextchunkid(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut chunkid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return chunk_mr_nextchunkid(chunkid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_patadd(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut gname: [uint8_t; 256] = [0; 256];
        let mut euid: uint32_t = 0;
        let mut egid: uint32_t = 0;
        let mut priority: uint8_t = 0;
        let mut omask: uint8_t = 0;
        let mut scid: uint8_t = 0;
        let mut seteattr: uint8_t = 0;
        let mut clreattr: uint8_t = 0;
        let mut trashretention: uint16_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut gname as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh24 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh24;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh25 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh25;
                let c2rust_fresh26 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh26;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh27 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            gname[c2rust_fresh27 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        gname[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        euid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        egid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if tmp > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        priority = tmp as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_0: uint32_t = 0;
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_0 = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if tmp_0 > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_0,
            );
            return -1 as ::core::ffi::c_int;
        }
        omask = tmp_0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_1: uint32_t = 0;
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_1 = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if tmp_1 > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_1,
            );
            return -1 as ::core::ffi::c_int;
        }
        scid = tmp_1 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_2: uint32_t = 0;
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_2 = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if tmp_2 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_2,
            );
            return -1 as ::core::ffi::c_int;
        }
        trashretention = tmp_2 as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_3: uint32_t = 0;
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_3 = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        if tmp_3 > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_3,
            );
            return -1 as ::core::ffi::c_int;
        }
        seteattr = tmp_3 as uint8_t;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_4: uint32_t = 0;
            let mut eptr_6: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_4 = strtoul(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_6 as *const ::core::ffi::c_char;
            if tmp_4 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_4,
                );
                return -1 as ::core::ffi::c_int;
            }
            clreattr = tmp_4 as uint8_t;
        } else {
            clreattr = 0 as uint8_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return patterns_mr_add(
            strlen(&raw mut gname as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut gname as *mut uint8_t,
            euid,
            egid,
            priority,
            omask,
            scid,
            trashretention,
            seteattr,
            clreattr,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_patdel(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut gname: [uint8_t; 256] = [0; 256];
        let mut euid: uint32_t = 0;
        let mut egid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut gname as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh28 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh28;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh29 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh29;
                let c2rust_fresh30 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh30;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh31 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            gname[c2rust_fresh31 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        gname[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        euid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        egid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return patterns_mr_delete(
            strlen(&raw mut gname as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut gname as *mut uint8_t,
            euid,
            egid,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_posixlock(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut pid: uint32_t = 0;
        let mut owner: uint64_t = 0;
        let mut start: uint64_t = 0;
        let mut end: uint64_t = 0;
        let mut cmd: ::core::ffi::c_char = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sessionid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        owner = strtoull(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        cmd = *ptr;
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        start = strtoull(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        end = strtoull(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        pid = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return posix_lock_mr_change(inode, sessionid, owner, cmd, start, end, pid)
            as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_purge(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_purge(ts, inode) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_quota(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut stimestamp: uint32_t = 0;
        let mut sinodes: uint32_t = 0;
        let mut hinodes: uint32_t = 0;
        let mut slength: uint64_t = 0;
        let mut ssize: uint64_t = 0;
        let mut srealsize: uint64_t = 0;
        let mut hlength: uint64_t = 0;
        let mut hsize: uint64_t = 0;
        let mut hrealsize: uint64_t = 0;
        let mut flags: uint32_t = 0;
        let mut exceeded: uint32_t = 0;
        let mut timelimit: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        exceeded = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        flags = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        stimestamp = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sinodes = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        hinodes = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        slength = strtoull(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_6: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        hlength = strtoull(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_6 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_7: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ssize = strtoull(ptr, &raw mut eptr_7, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_7 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_8: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        hsize = strtoull(ptr, &raw mut eptr_8, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_8 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_9: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        srealsize = strtoull(ptr, &raw mut eptr_9, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_9 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_10: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        hrealsize = strtoull(ptr, &raw mut eptr_10, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_10 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_11: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            timelimit = strtoul(ptr, &raw mut eptr_11, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_11 as *const ::core::ffi::c_char;
        } else {
            timelimit = 0 as uint32_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_quota(
            ts,
            inode,
            exceeded as uint8_t,
            flags as uint8_t,
            stimestamp,
            sinodes,
            hinodes,
            slength,
            hlength,
            ssize,
            hsize,
            srealsize,
            hrealsize,
            timelimit,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_release(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut cuid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        cuid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return of_mr_release(inode, cuid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_repair(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut version: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        indx = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        version = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        return fs_mr_repair(ts, inode, indx, version) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_renumedges(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut enextedgeid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        enextedgeid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        return fs_mr_renumerate_edges(enextedgeid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_session(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sessionid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sessionid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        return sessions_mr_session(sessionid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_sesadd(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rootinode: uint32_t = 0;
        let mut sesflags: uint32_t = 0;
        let mut peerip: uint32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut rootuid: uint32_t = 0;
        let mut rootgid: uint32_t = 0;
        let mut mapalluid: uint32_t = 0;
        let mut mapallgid: uint32_t = 0;
        let mut mingoal: uint32_t = 0;
        let mut maxgoal: uint32_t = 0;
        let mut sclassgroups: uint32_t = 0;
        let mut mintrashretention: uint32_t = 0;
        let mut maxtrashretention: uint32_t = 0;
        let mut disables: uint32_t = 0;
        let mut umaskval: uint16_t = 0;
        let mut exportscsum: uint64_t = 0;
        let mut ileng: uint32_t = 0;
        static mut info: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut infosize: uint32_t = 0 as uint32_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != '#' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    '#' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            exportscsum = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
            ptr = eptr as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        } else {
            exportscsum = 0 as uint64_t;
        }
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rootinode = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sesflags = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == '0' as ::core::ffi::c_int {
            if (*ptr.offset(1 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
                || *ptr.offset(1 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
                || (*ptr.offset(2 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
                || *ptr.offset(2 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
                || (*ptr.offset(3 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
                || *ptr.offset(3 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"wrong session umask ('%c%c%c' - octal number expected)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *ptr.offset(1 as isize) as ::core::ffi::c_int,
                    *ptr.offset(2 as isize) as ::core::ffi::c_int,
                    *ptr.offset(3 as isize) as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            umaskval = ((*ptr.offset(1 as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                * 64 as ::core::ffi::c_int
                + (*ptr.offset(2 as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                    * 8 as ::core::ffi::c_int
                + (*ptr.offset(3 as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
                as uint16_t;
            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        } else {
            umaskval = 0 as uint16_t;
        }
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rootuid = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rootgid = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mapalluid = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mapallgid = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == '0' as ::core::ffi::c_int
            && *ptr.offset(1 as isize) as ::core::ffi::c_int == 'x' as ::core::ffi::c_int
        {
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            let mut eptr_6: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sclassgroups = strtoul(ptr, &raw mut eptr_6, 16 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_6 as *const ::core::ffi::c_char;
            if sclassgroups > 0xffff as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"wrong sclassgroups (0x%X)\0".as_ptr() as *const ::core::ffi::c_char,
                    sclassgroups,
                );
                return -1 as ::core::ffi::c_int;
            }
        } else {
            let mut eptr_7: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            mingoal = strtoul(ptr, &raw mut eptr_7, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_7 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_8: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            maxgoal = strtoul(ptr, &raw mut eptr_8, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_8 as *const ::core::ffi::c_char;
            sclassgroups = 1 as uint32_t;
            while mingoal <= maxgoal {
                sclassgroups |= ((1 as ::core::ffi::c_int) << mingoal) as uint32_t;
                mingoal = mingoal.wrapping_add(1);
            }
        }
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_9: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mintrashretention = strtoul(ptr, &raw mut eptr_9, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_9 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_10: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        maxtrashretention = strtoul(ptr, &raw mut eptr_10, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_10 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr.offset(0 as isize) as ::core::ffi::c_int == '0' as ::core::ffi::c_int
            && *ptr.offset(1 as isize) as ::core::ffi::c_int == 'x' as ::core::ffi::c_int
        {
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            let mut eptr_11: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            disables = strtoul(ptr, &raw mut eptr_11, 16 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_11 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        } else {
            disables = 0 as uint32_t;
        }
        let mut eptr_12: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        peerip = strtoul(ptr, &raw mut eptr_12, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_12 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        ileng = 0 as uint32_t;
        loop {
            let c2rust_fresh32 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh32;
            if _tmp_c as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh33 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh33;
                let c2rust_fresh34 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh34;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if ileng >= infosize {
                infosize = ileng.wrapping_add(1000 as uint32_t);
                if info.is_null() {
                    info = malloc(infosize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_buff: *mut uint8_t = info;
                    info = realloc(info as *mut ::core::ffi::c_void, infosize as size_t)
                        as *mut uint8_t;
                    if info.is_null() {
                        free(_tmp_buff as *mut ::core::ffi::c_void);
                    }
                }
                if info.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1089 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1089 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if info
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1089 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"info\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1089 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"info\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            let c2rust_fresh35 = ileng;
            ileng = ileng.wrapping_add(1);
            *info.offset(c2rust_fresh35 as isize) = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_13: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sessionid = strtoul(ptr, &raw mut eptr_13, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_13 as *const ::core::ffi::c_char;
        return sessions_mr_sesadd(
            exportscsum,
            rootinode,
            sesflags as uint8_t,
            umaskval,
            rootuid,
            rootgid,
            mapalluid,
            mapallgid,
            sclassgroups as uint16_t,
            mintrashretention,
            maxtrashretention,
            disables,
            peerip,
            info,
            ileng,
            sessionid,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_seschanged(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rootinode: uint32_t = 0;
        let mut sesflags: uint32_t = 0;
        let mut peerip: uint32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut rootuid: uint32_t = 0;
        let mut rootgid: uint32_t = 0;
        let mut mapalluid: uint32_t = 0;
        let mut mapallgid: uint32_t = 0;
        let mut mingoal: uint32_t = 0;
        let mut maxgoal: uint32_t = 0;
        let mut sclassgroups: uint32_t = 0;
        let mut mintrashretention: uint32_t = 0;
        let mut maxtrashretention: uint32_t = 0;
        let mut disables: uint32_t = 0;
        let mut umaskval: uint16_t = 0;
        let mut exportscsum: uint64_t = 0;
        let mut ileng: uint32_t = 0;
        static mut info: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut infosize: uint32_t = 0 as uint32_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sessionid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != '#' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    '#' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            exportscsum = strtoull(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint64_t;
            ptr = eptr_0 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        } else {
            exportscsum = 0 as uint64_t;
        }
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rootinode = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sesflags = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == '0' as ::core::ffi::c_int {
            if (*ptr.offset(1 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
                || *ptr.offset(1 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
                || (*ptr.offset(2 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
                || *ptr.offset(2 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
                || (*ptr.offset(3 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
                || *ptr.offset(3 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"wrong session umask ('%c%c%c' - octal number expected)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *ptr.offset(1 as isize) as ::core::ffi::c_int,
                    *ptr.offset(2 as isize) as ::core::ffi::c_int,
                    *ptr.offset(3 as isize) as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            umaskval = ((*ptr.offset(1 as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                * 64 as ::core::ffi::c_int
                + (*ptr.offset(2 as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                    * 8 as ::core::ffi::c_int
                + (*ptr.offset(3 as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
                as uint16_t;
            ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        } else {
            umaskval = 0 as uint16_t;
        }
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rootuid = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rootgid = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mapalluid = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_6: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mapallgid = strtoul(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_6 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == '0' as ::core::ffi::c_int
            && *ptr.offset(1 as isize) as ::core::ffi::c_int == 'x' as ::core::ffi::c_int
        {
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            let mut eptr_7: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sclassgroups = strtoul(ptr, &raw mut eptr_7, 16 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_7 as *const ::core::ffi::c_char;
            if sclassgroups > 0xffff as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"wrong sclassgroups (0x%X)\0".as_ptr() as *const ::core::ffi::c_char,
                    sclassgroups,
                );
                return -1 as ::core::ffi::c_int;
            }
        } else {
            let mut eptr_8: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            mingoal = strtoul(ptr, &raw mut eptr_8, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_8 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_9: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            maxgoal = strtoul(ptr, &raw mut eptr_9, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_9 as *const ::core::ffi::c_char;
            sclassgroups = 1 as uint32_t;
            while mingoal <= maxgoal {
                sclassgroups |= ((1 as ::core::ffi::c_int) << mingoal) as uint32_t;
                mingoal = mingoal.wrapping_add(1);
            }
        }
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_10: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mintrashretention = strtoul(ptr, &raw mut eptr_10, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_10 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_11: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        maxtrashretention = strtoul(ptr, &raw mut eptr_11, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_11 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr.offset(0 as isize) as ::core::ffi::c_int == '0' as ::core::ffi::c_int
            && *ptr.offset(1 as isize) as ::core::ffi::c_int == 'x' as ::core::ffi::c_int
        {
            ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
            let mut eptr_12: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            disables = strtoul(ptr, &raw mut eptr_12, 16 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_12 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
        } else {
            disables = 0 as uint32_t;
        }
        let mut eptr_13: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        peerip = strtoul(ptr, &raw mut eptr_13, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_13 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        ileng = 0 as uint32_t;
        loop {
            let c2rust_fresh36 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh36;
            if _tmp_c as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh37 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh37;
                let c2rust_fresh38 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh38;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if ileng >= infosize {
                infosize = ileng.wrapping_add(1000 as uint32_t);
                if info.is_null() {
                    info = malloc(infosize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_buff: *mut uint8_t = info;
                    info = realloc(info as *mut ::core::ffi::c_void, infosize as size_t)
                        as *mut uint8_t;
                    if info.is_null() {
                        free(_tmp_buff as *mut ::core::ffi::c_void);
                    }
                }
                if info.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if info
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"info\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"info\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            let c2rust_fresh39 = ileng;
            ileng = ileng.wrapping_add(1);
            *info.offset(c2rust_fresh39 as isize) = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return sessions_mr_seschanged(
            sessionid,
            exportscsum,
            rootinode,
            sesflags as uint8_t,
            umaskval,
            rootuid,
            rootgid,
            mapalluid,
            mapallgid,
            sclassgroups as uint16_t,
            mintrashretention,
            maxtrashretention,
            disables,
            peerip,
            info,
            ileng,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_sesdel(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sessionid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sessionid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return sessions_mr_sesdel(sessionid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_sesconnected(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sessionid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sessionid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return sessions_mr_connected(sessionid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_sesdisconnected(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sessionid: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sessionid = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return sessions_mr_disconnected(sessionid, ts) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_rollback(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut prevchunkid: uint64_t = 0;
        let mut chunkid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        indx = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        prevchunkid = strtoull(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_rollback(ts, inode, indx, prevchunkid, chunkid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_seteattr(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut ci: uint32_t = 0;
        let mut nci: uint32_t = 0;
        let mut npi: uint32_t = 0;
        let mut eattr: uint8_t = 0;
        let mut smode: uint8_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        eattr = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint8_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        smode = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint8_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ci = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        nci = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        npi = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        return fs_mr_seteattr(ts, inode, uid, eattr, smode, ci, nci, npi) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_setfilechunk(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        indx = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_set_file_chunk(inode, indx, chunkid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_setgoal(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut ci: uint32_t = 0;
        let mut nci: uint32_t = 0;
        let mut npi: uint32_t = 0;
        let mut sclassid: uint8_t = 0;
        let mut smode: uint8_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sclassid = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint8_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        smode = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint8_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ci = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        nci = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        npi = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        return fs_mr_setsclass(ts, inode, uid, sclassid, sclassid, smode, ci, nci, npi)
            as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_setsclass(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut ci: uint32_t = 0;
        let mut nci: uint32_t = 0;
        let mut npi: uint32_t = 0;
        let mut src_sclassid: uint8_t = 0;
        let mut dst_sclassid: uint8_t = 0;
        let mut smode: uint8_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        src_sclassid = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint8_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        dst_sclassid = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint8_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        smode = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint8_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ci = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        nci = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_6: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        npi = strtoul(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_6 as *const ::core::ffi::c_char;
        return fs_mr_setsclass(
            ts,
            inode,
            uid,
            src_sclassid,
            dst_sclassid,
            smode,
            ci,
            nci,
            npi,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_setmetaid(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut metaid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        metaid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return meta_mr_setmetaid(metaid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_setpath(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        static mut path: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut pathsize: uint32_t = 0 as uint32_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh40 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh40;
            if _tmp_c as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh41 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh41;
                let c2rust_fresh42 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh42;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if _tmp_i >= pathsize {
                pathsize = _tmp_i.wrapping_add(1000 as uint32_t);
                if path.is_null() {
                    path = malloc(pathsize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_path: *mut uint8_t = path;
                    path = realloc(path as *mut ::core::ffi::c_void, pathsize as size_t)
                        as *mut uint8_t;
                    if path.is_null() {
                        free(_tmp_path as *mut ::core::ffi::c_void);
                    }
                }
                if path.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if path
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            let c2rust_fresh43 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            *path.offset(c2rust_fresh43 as isize) = _tmp_c as uint8_t;
        }
        if _tmp_i >= pathsize {
            pathsize = _tmp_i.wrapping_add(1000 as uint32_t);
            if path.is_null() {
                path = malloc(pathsize as size_t) as *mut uint8_t;
            } else {
                let mut _tmp_path_0: *mut uint8_t = path;
                path =
                    realloc(path as *mut ::core::ffi::c_void, pathsize as size_t) as *mut uint8_t;
                if path.is_null() {
                    free(_tmp_path_0 as *mut ::core::ffi::c_void);
                }
            }
            if path.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if path
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        ptr = ptr.offset(-1);
        *path.offset(_tmp_i as isize) = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_setpath(inode, path) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_settrashretention(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut ci: uint32_t = 0;
        let mut nci: uint32_t = 0;
        let mut npi: uint32_t = 0;
        let mut trashretention: uint32_t = 0;
        let mut smode: uint8_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        trashretention = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        smode = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint8_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ci = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        nci = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        npi = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        return fs_mr_settrashretention(ts, inode, uid, trashretention, smode, ci, nci, npi)
            as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_setxattr(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut valueleng: uint32_t = 0;
        let mut mode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        static mut value: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut valuesize: uint32_t = 0 as uint32_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh44 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh44;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh45 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh45;
                let c2rust_fresh46 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh46;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh47 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name[c2rust_fresh47 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_c_0: ::core::ffi::c_char = 0;
        let mut _tmp_h1_0: ::core::ffi::c_char = 0;
        let mut _tmp_h2_0: ::core::ffi::c_char = 0;
        valueleng = 0 as uint32_t;
        loop {
            let c2rust_fresh48 = ptr;
            ptr = ptr.offset(1);
            _tmp_c_0 = *c2rust_fresh48;
            if _tmp_c_0 as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh49 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1_0 = *c2rust_fresh49;
                let c2rust_fresh50 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2_0 = *c2rust_fresh50;
                if _tmp_h1_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c_0 = (_tmp_h1_0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if valueleng >= valuesize {
                valuesize = valueleng.wrapping_add(1000 as uint32_t);
                if value.is_null() {
                    value = malloc(valuesize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_buff: *mut uint8_t = value;
                    value = realloc(value as *mut ::core::ffi::c_void, valuesize as size_t)
                        as *mut uint8_t;
                    if value.is_null() {
                        free(_tmp_buff as *mut ::core::ffi::c_void);
                    }
                }
                if value.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1368 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"value\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1368 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"value\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if value
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1368 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"value\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1368 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"value\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            let c2rust_fresh51 = valueleng;
            valueleng = valueleng.wrapping_add(1);
            *value.offset(c2rust_fresh51 as isize) = _tmp_c_0 as uint8_t;
        }
        ptr = ptr.offset(-1);
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mode = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_setxattr(
            ts,
            inode,
            strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint32_t,
            &raw mut name as *mut uint8_t,
            valueleng,
            value,
            mode,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_setacl(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut aclblobleng: uint32_t = 0;
        let mut acltype: uint8_t = 0;
        let mut changectime: uint8_t = 0;
        let mut mode: uint16_t = 0;
        let mut userperm: uint16_t = 0;
        let mut groupperm: uint16_t = 0;
        let mut otherperm: uint16_t = 0;
        let mut mask: uint16_t = 0;
        let mut namedusers: uint16_t = 0;
        let mut namedgroups: uint16_t = 0;
        static mut aclblob: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut aclblobsize: uint32_t = 0 as uint32_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if tmp > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        mode = tmp as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_0: uint32_t = 0;
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_0 = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if tmp_0 > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_0,
            );
            return -1 as ::core::ffi::c_int;
        }
        changectime = tmp_0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_1: uint32_t = 0;
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_1 = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if tmp_1 > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_1,
            );
            return -1 as ::core::ffi::c_int;
        }
        acltype = tmp_1 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_2: uint32_t = 0;
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_2 = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if tmp_2 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_2,
            );
            return -1 as ::core::ffi::c_int;
        }
        userperm = tmp_2 as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_3: uint32_t = 0;
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_3 = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if tmp_3 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_3,
            );
            return -1 as ::core::ffi::c_int;
        }
        groupperm = tmp_3 as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_4: uint32_t = 0;
        let mut eptr_5: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_4 = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_5 as *const ::core::ffi::c_char;
        if tmp_4 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_4,
            );
            return -1 as ::core::ffi::c_int;
        }
        otherperm = tmp_4 as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_5: uint32_t = 0;
        let mut eptr_6: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_5 = strtoul(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_6 as *const ::core::ffi::c_char;
        if tmp_5 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_5,
            );
            return -1 as ::core::ffi::c_int;
        }
        mask = tmp_5 as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_6: uint32_t = 0;
        let mut eptr_7: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_6 = strtoul(ptr, &raw mut eptr_7, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_7 as *const ::core::ffi::c_char;
        if tmp_6 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_6,
            );
            return -1 as ::core::ffi::c_int;
        }
        namedusers = tmp_6 as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_7: uint32_t = 0;
        let mut eptr_8: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_7 = strtoul(ptr, &raw mut eptr_8, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_8 as *const ::core::ffi::c_char;
        if tmp_7 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_7,
            );
            return -1 as ::core::ffi::c_int;
        }
        namedgroups = tmp_7 as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        aclblobleng = 0 as uint32_t;
        loop {
            let c2rust_fresh52 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh52;
            if _tmp_c as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh53 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh53;
                let c2rust_fresh54 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh54;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if aclblobleng >= aclblobsize {
                aclblobsize = aclblobleng.wrapping_add(1000 as uint32_t);
                if aclblob.is_null() {
                    aclblob = malloc(aclblobsize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_buff: *mut uint8_t = aclblob;
                    aclblob = realloc(aclblob as *mut ::core::ffi::c_void, aclblobsize as size_t)
                        as *mut uint8_t;
                    if aclblob.is_null() {
                        free(_tmp_buff as *mut ::core::ffi::c_void);
                    }
                }
                if aclblob.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1403 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"aclblob\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1403 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"aclblob\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if aclblob
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1403 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"aclblob\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1403 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"aclblob\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            let c2rust_fresh55 = aclblobleng;
            aclblobleng = aclblobleng.wrapping_add(1);
            *aclblob.offset(c2rust_fresh55 as isize) = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if aclblobleng
            != (6 as uint32_t).wrapping_mul(
                (namedusers as ::core::ffi::c_int + namedgroups as ::core::ffi::c_int) as uint32_t,
            )
        {
            return MFS_ERROR_MISMATCH;
        }
        return fs_mr_setacl(
            ts,
            inode,
            mode,
            changectime,
            acltype,
            userperm,
            groupperm,
            otherperm,
            mask,
            namedusers,
            namedgroups,
            aclblob,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_snapshot(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut parent: uint32_t = 0;
        let mut smode: uint32_t = 0;
        let mut sesflags: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut umask: uint32_t = 0;
        let mut inodecheck: uint32_t = 0;
        let mut removed: uint32_t = 0;
        let mut same: uint32_t = 0;
        let mut existing: uint32_t = 0;
        let mut hardlinks: uint32_t = 0;
        let mut new: uint32_t = 0;
        static mut gidstr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut gidstrsize: uint32_t = 0 as uint32_t;
        static mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut gidsize: uint32_t = 0 as uint32_t;
        let mut gidleng: uint32_t = 0;
        let mut gidtab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut name: [uint8_t; 256] = [0; 256];
        let mut mode: uint8_t = 0;
        let mut i: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        parent = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh56 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh56;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh57 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh57;
                let c2rust_fresh58 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh58;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh59 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name[c2rust_fresh59 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        smode = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sesflags = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
            mode = 2 as uint8_t;
        } else {
            let mut eptr_4: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            gids = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_4 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            mode = 0 as uint8_t;
            i = 0 as uint32_t;
            while *ptr.offset(i as isize) as ::core::ffi::c_int != 0
                && *ptr.offset(i as isize) as ::core::ffi::c_int != ')' as ::core::ffi::c_int
            {
                if (*ptr.offset(i as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
                    || *ptr.offset(i as isize) as ::core::ffi::c_int > '9' as ::core::ffi::c_int
                {
                    mode = 1 as uint8_t;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
        if mode as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            if mode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                let mut _tmp_c_0: ::core::ffi::c_char = 0;
                let mut eptr_5: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                gids = 0 as uint32_t;
                let c2rust_fresh60 = ptr;
                ptr = ptr.offset(1);
                _tmp_c_0 = *c2rust_fresh60;
                if _tmp_c_0 as ::core::ffi::c_int != '[' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '[' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                loop {
                    let c2rust_fresh61 = ptr;
                    ptr = ptr.offset(1);
                    _tmp_c_0 = *c2rust_fresh61;
                    if _tmp_c_0 as ::core::ffi::c_int == ']' as ::core::ffi::c_int {
                        break;
                    }
                    if _tmp_c_0 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                        || _tmp_c_0 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                        || _tmp_c_0 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: ']' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    if _tmp_c_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && _tmp_c_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(-1);
                        if gids >= gidsize {
                            gidsize = gids.wrapping_add(32 as uint32_t);
                            if gid.is_null() {
                                gid = malloc(
                                    (gidsize as size_t)
                                        .wrapping_mul(::core::mem::size_of::<uint32_t>()),
                                ) as *mut uint32_t;
                            } else {
                                let mut _tmp_buff: *mut uint32_t = gid;
                                gid = realloc(
                                    gid as *mut ::core::ffi::c_void,
                                    (gidsize as size_t)
                                        .wrapping_mul(::core::mem::size_of::<uint32_t>()),
                                ) as *mut uint32_t;
                                if gid.is_null() {
                                    free(_tmp_buff as *mut ::core::ffi::c_void);
                                }
                            }
                            if gid.is_null() {
                                fprintf(
                                    stderr,
                                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"gid\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"gid\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                abort();
                            } else if gid
                                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                    -1 as ::core::ffi::c_int as usize,
                                ) as *mut uint32_t
                            {
                                let mut _mfs_errorstring: *const ::core::ffi::c_char =
                                    strerr(*__errno_location());
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"gid\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"gid\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring,
                                );
                                abort();
                            }
                        }
                        let c2rust_fresh62 = gids;
                        gids = gids.wrapping_add(1);
                        *gid.offset(c2rust_fresh62 as isize) =
                            strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
                        ptr = eptr_5 as *const ::core::ffi::c_char;
                    } else if _tmp_c_0 as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: number or ',' expected\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            filename,
                            lv,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                }
                gidtab = gid;
            } else {
                let mut _tmp_c_1: ::core::ffi::c_char = 0;
                let mut _tmp_h1_0: ::core::ffi::c_char = 0;
                let mut _tmp_h2_0: ::core::ffi::c_char = 0;
                gidleng = 0 as uint32_t;
                loop {
                    let c2rust_fresh63 = ptr;
                    ptr = ptr.offset(1);
                    _tmp_c_1 = *c2rust_fresh63;
                    if _tmp_c_1 as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                        break;
                    }
                    if _tmp_c_1 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                        || _tmp_c_1 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                        || _tmp_c_1 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    if _tmp_c_1 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                        let c2rust_fresh64 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h1_0 = *c2rust_fresh64;
                        let c2rust_fresh65 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h2_0 = *c2rust_fresh65;
                        if _tmp_h1_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h1_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h1_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h1_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                                filename,
                                lv,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if _tmp_h2_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h2_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h2_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h2_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                                filename,
                                lv,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        _tmp_c_1 = (_tmp_h1_0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                            + _tmp_h2_0 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    }
                    if gidleng >= gidstrsize {
                        gidstrsize = gidleng.wrapping_add(1000 as uint32_t);
                        if gidstr.is_null() {
                            gidstr = malloc(gidstrsize as size_t) as *mut uint8_t;
                        } else {
                            let mut _tmp_buff_0: *mut uint8_t = gidstr;
                            gidstr =
                                realloc(gidstr as *mut ::core::ffi::c_void, gidstrsize as size_t)
                                    as *mut uint8_t;
                            if gidstr.is_null() {
                                free(_tmp_buff_0 as *mut ::core::ffi::c_void);
                            }
                        }
                        if gidstr.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"gidstr\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"gidstr\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if gidstr
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
                                b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"gidstr\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"gidstr\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            abort();
                        }
                    }
                    let c2rust_fresh66 = gidleng;
                    gidleng = gidleng.wrapping_add(1);
                    *gidstr.offset(c2rust_fresh66 as isize) = _tmp_c_1 as uint8_t;
                }
                ptr = ptr.offset(-1);
                if gids.wrapping_mul(4 as uint32_t) != gidleng {
                    return MFS_ERROR_MISMATCH;
                }
                gidtab = gidstr as *mut uint32_t;
            }
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_6: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            umask = strtoul(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_6 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ':' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut eptr_7: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                inodecheck = strtoul(ptr, &raw mut eptr_7, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_7 as *const ::core::ffi::c_char;
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut eptr_8: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                removed = strtoul(ptr, &raw mut eptr_8, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_8 as *const ::core::ffi::c_char;
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut eptr_9: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                same = strtoul(ptr, &raw mut eptr_9, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_9 as *const ::core::ffi::c_char;
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut eptr_10: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                existing = strtoul(ptr, &raw mut eptr_10, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_10 as *const ::core::ffi::c_char;
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut eptr_11: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                hardlinks = strtoul(ptr, &raw mut eptr_11, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_11 as *const ::core::ffi::c_char;
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut eptr_12: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                new = strtoul(ptr, &raw mut eptr_12, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_12 as *const ::core::ffi::c_char;
            } else {
                inodecheck = 0 as uint32_t;
                removed = 0 as uint32_t;
                same = 0 as uint32_t;
                existing = 0 as uint32_t;
                hardlinks = 0 as uint32_t;
                new = 0 as uint32_t;
            }
            return fs_mr_snapshot(
                ts,
                inode,
                parent,
                strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint16_t,
                &raw mut name as *mut uint8_t,
                smode as uint8_t,
                sesflags as uint8_t,
                uid,
                gids,
                gidtab,
                umask as uint16_t,
                inodecheck,
                removed,
                same,
                existing,
                hardlinks,
                new,
            ) as ::core::ffi::c_int;
        } else {
            let mut eptr_13: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            umask = strtoul(ptr, &raw mut eptr_13, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_13 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            return fs_mr_snapshot(
                ts,
                inode,
                parent,
                strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint16_t,
                &raw mut name as *mut uint8_t,
                smode as uint8_t,
                sesflags as uint8_t,
                uid,
                1 as uint32_t,
                &raw mut gids,
                umask as uint16_t,
                0 as uint32_t,
                0 as uint32_t,
                0 as uint32_t,
                0 as uint32_t,
                0 as uint32_t,
                0 as uint32_t,
            ) as ::core::ffi::c_int;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_symlink(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut parent: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        static mut path: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut pathsize: uint32_t = 0 as uint32_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        parent = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh67 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh67;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh68 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh68;
                let c2rust_fresh69 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh69;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh70 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name[c2rust_fresh70 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i_0: uint32_t = 0;
        let mut _tmp_c_0: ::core::ffi::c_char = 0;
        let mut _tmp_h1_0: ::core::ffi::c_char = 0;
        let mut _tmp_h2_0: ::core::ffi::c_char = 0;
        _tmp_i_0 = 0 as uint32_t;
        loop {
            let c2rust_fresh71 = ptr;
            ptr = ptr.offset(1);
            _tmp_c_0 = *c2rust_fresh71;
            if _tmp_c_0 as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh72 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1_0 = *c2rust_fresh72;
                let c2rust_fresh73 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2_0 = *c2rust_fresh73;
                if _tmp_h1_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c_0 = (_tmp_h1_0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if _tmp_i_0 >= pathsize {
                pathsize = _tmp_i_0.wrapping_add(1000 as uint32_t);
                if path.is_null() {
                    path = malloc(pathsize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_path: *mut uint8_t = path;
                    path = realloc(path as *mut ::core::ffi::c_void, pathsize as size_t)
                        as *mut uint8_t;
                    if path.is_null() {
                        free(_tmp_path as *mut ::core::ffi::c_void);
                    }
                }
                if path.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1505 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1505 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if path
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1505 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1505 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            let c2rust_fresh74 = _tmp_i_0;
            _tmp_i_0 = _tmp_i_0.wrapping_add(1);
            *path.offset(c2rust_fresh74 as isize) = _tmp_c_0 as uint8_t;
        }
        if _tmp_i_0 >= pathsize {
            pathsize = _tmp_i_0.wrapping_add(1000 as uint32_t);
            if path.is_null() {
                path = malloc(pathsize as size_t) as *mut uint8_t;
            } else {
                let mut _tmp_path_0: *mut uint8_t = path;
                path =
                    realloc(path as *mut ::core::ffi::c_void, pathsize as size_t) as *mut uint8_t;
                if path.is_null() {
                    free(_tmp_path_0 as *mut ::core::ffi::c_void);
                }
            }
            if path.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1505 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1505 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if path
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1505 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1505 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        ptr = ptr.offset(-1);
        *path.offset(_tmp_i_0 as isize) = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        gid = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        return fs_mr_symlink(
            ts,
            parent,
            strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint32_t,
            &raw mut name as *mut uint8_t,
            path,
            uid,
            gid,
            inode,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_scecon(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return sclass_mr_ec_version(1 as uint8_t) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_scecversion(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ec_new_version: uint8_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if tmp > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        ec_new_version = tmp as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return sclass_mr_ec_version(ec_new_version) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_scdel(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut name: [uint8_t; 256] = [0; 256];
        let mut spid: uint16_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh75 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh75;
            if !(_tmp_c as ::core::ffi::c_int != ')' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh76 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh76;
                let c2rust_fresh77 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh77;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh78 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name[c2rust_fresh78 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if tmp > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        spid = tmp as uint16_t;
        return sclass_mr_delete_entry(
            strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut name as *mut uint8_t,
            spid,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_scdup(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sname: [uint8_t; 256] = [0; 256];
        let mut dname: [uint8_t; 256] = [0; 256];
        let mut sspid: uint16_t = 0;
        let mut dspid: uint16_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut sname as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh79 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh79;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh80 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh80;
                let c2rust_fresh81 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh81;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh82 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            sname[c2rust_fresh82 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        sname[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i_0: uint32_t = 0;
        let mut _tmp_c_0: ::core::ffi::c_char = 0;
        let mut _tmp_h1_0: ::core::ffi::c_char = 0;
        let mut _tmp_h2_0: ::core::ffi::c_char = 0;
        memset(
            &raw mut dname as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i_0 = 0 as uint32_t;
        loop {
            let c2rust_fresh83 = ptr;
            ptr = ptr.offset(1);
            _tmp_c_0 = *c2rust_fresh83;
            if !(_tmp_c_0 as ::core::ffi::c_int != ')' as ::core::ffi::c_int
                && _tmp_i_0 < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh84 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1_0 = *c2rust_fresh84;
                let c2rust_fresh85 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2_0 = *c2rust_fresh85;
                if _tmp_h1_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c_0 = (_tmp_h1_0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh86 = _tmp_i_0;
            _tmp_i_0 = _tmp_i_0.wrapping_add(1);
            dname[c2rust_fresh86 as usize] = _tmp_c_0 as uint8_t;
        }
        ptr = ptr.offset(-1);
        dname[_tmp_i_0 as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if tmp > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        sspid = tmp as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_0: uint32_t = 0;
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_0 = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if tmp_0 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_0,
            );
            return -1 as ::core::ffi::c_int;
        }
        dspid = tmp_0 as uint16_t;
        return sclass_mr_duplicate_entry(
            strlen(&raw mut sname as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut sname as *mut uint8_t,
            strlen(&raw mut dname as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut dname as *mut uint8_t,
            sspid,
            dspid,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_scren(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sname: [uint8_t; 256] = [0; 256];
        let mut dname: [uint8_t; 256] = [0; 256];
        let mut spid: uint16_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut sname as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh87 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh87;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh88 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh88;
                let c2rust_fresh89 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh89;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh90 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            sname[c2rust_fresh90 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        sname[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i_0: uint32_t = 0;
        let mut _tmp_c_0: ::core::ffi::c_char = 0;
        let mut _tmp_h1_0: ::core::ffi::c_char = 0;
        let mut _tmp_h2_0: ::core::ffi::c_char = 0;
        memset(
            &raw mut dname as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i_0 = 0 as uint32_t;
        loop {
            let c2rust_fresh91 = ptr;
            ptr = ptr.offset(1);
            _tmp_c_0 = *c2rust_fresh91;
            if !(_tmp_c_0 as ::core::ffi::c_int != ')' as ::core::ffi::c_int
                && _tmp_i_0 < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh92 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1_0 = *c2rust_fresh92;
                let c2rust_fresh93 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2_0 = *c2rust_fresh93;
                if _tmp_h1_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c_0 = (_tmp_h1_0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh94 = _tmp_i_0;
            _tmp_i_0 = _tmp_i_0.wrapping_add(1);
            dname[c2rust_fresh94 as usize] = _tmp_c_0 as uint8_t;
        }
        ptr = ptr.offset(-1);
        dname[_tmp_i_0 as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if tmp > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        spid = tmp as uint16_t;
        return sclass_mr_rename_entry(
            strlen(&raw mut sname as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut sname as *mut uint8_t,
            strlen(&raw mut dname as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut dname as *mut uint8_t,
            spid,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_scset(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    _ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut name: [uint8_t; 256] = [0; 256];
        let mut desc: [uint8_t; 256] = [0; 256];
        let mut spid: uint16_t = 0;
        let mut arch_delay: uint16_t = 0;
        let mut min_trashretention: uint16_t = 0;
        let mut arch_min_size: uint64_t = 0;
        let mut version: uint8_t = 0;
        let mut new_flag: uint8_t = 0;
        let mut export_group: uint8_t = 0;
        let mut adminonly: uint8_t = 0;
        let mut labels_mode: uint8_t = 0;
        let mut arch_mode: uint8_t = 0;
        let mut i: uint8_t = 0;
        let mut priority: uint32_t = 0;
        let mut create: storagemode = storagemode {
            uniqmask: 0,
            ec_data_chksum_parts: 0,
            has_labels: 0,
            matching_servers: 0,
            valid_ec_counters: 0,
            replallowed: 0,
            overloaded: 0,
            allvalid: 0,
            data_replallowed: 0,
            data_overloaded: 0,
            data_allvalid: 0,
            chksum_replallowed: 0,
            chksum_overloaded: 0,
            chksum_allvalid: 0,
            both_replallowed: 0,
            both_overloaded: 0,
            both_allvalid: 0,
            labels_mode: 0,
            labelscnt: 0,
            labelexpr: [[0; 128]; 9],
        };
        let mut keep: storagemode = storagemode {
            uniqmask: 0,
            ec_data_chksum_parts: 0,
            has_labels: 0,
            matching_servers: 0,
            valid_ec_counters: 0,
            replallowed: 0,
            overloaded: 0,
            allvalid: 0,
            data_replallowed: 0,
            data_overloaded: 0,
            data_allvalid: 0,
            chksum_replallowed: 0,
            chksum_overloaded: 0,
            chksum_allvalid: 0,
            both_replallowed: 0,
            both_overloaded: 0,
            both_allvalid: 0,
            labels_mode: 0,
            labelscnt: 0,
            labelexpr: [[0; 128]; 9],
        };
        let mut arch: storagemode = storagemode {
            uniqmask: 0,
            ec_data_chksum_parts: 0,
            has_labels: 0,
            matching_servers: 0,
            valid_ec_counters: 0,
            replallowed: 0,
            overloaded: 0,
            allvalid: 0,
            data_replallowed: 0,
            data_overloaded: 0,
            data_allvalid: 0,
            chksum_replallowed: 0,
            chksum_overloaded: 0,
            chksum_allvalid: 0,
            both_replallowed: 0,
            both_overloaded: 0,
            both_allvalid: 0,
            labels_mode: 0,
            labelscnt: 0,
            labelexpr: [[0; 128]; 9],
        };
        let mut trash: storagemode = storagemode {
            uniqmask: 0,
            ec_data_chksum_parts: 0,
            has_labels: 0,
            matching_servers: 0,
            valid_ec_counters: 0,
            replallowed: 0,
            overloaded: 0,
            allvalid: 0,
            data_replallowed: 0,
            data_overloaded: 0,
            data_allvalid: 0,
            chksum_replallowed: 0,
            chksum_overloaded: 0,
            chksum_allvalid: 0,
            both_replallowed: 0,
            both_overloaded: 0,
            both_allvalid: 0,
            labels_mode: 0,
            labelscnt: 0,
            labelexpr: [[0; 128]; 9],
        };
        let mut old_labelmasks: [uint32_t; 36] = [0; 36];
        let mut labelexpr_leng: uint32_t = 0;
        static mut labelexpr_buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut labelexpr_size: uint32_t = 0 as uint32_t;
        version = 0 as uint8_t;
        memset(
            &raw mut create as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<storagemode>(),
        );
        memset(
            &raw mut keep as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<storagemode>(),
        );
        memset(
            &raw mut arch as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<storagemode>(),
        );
        memset(
            &raw mut trash as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<storagemode>(),
        );
        create.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
        keep.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
        arch.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
        trash.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh95 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh95;
            if !(_tmp_c as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh96 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh96;
                let c2rust_fresh97 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh97;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh98 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name[c2rust_fresh98 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if tmp > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        new_flag = tmp as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == 'W' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != 'W' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    'W' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_0: uint32_t = 0;
            let mut eptr_0: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_0 = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_0 as *const ::core::ffi::c_char;
            if tmp_0 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_0,
                );
                return -1 as ::core::ffi::c_int;
            }
            create.labelscnt = tmp_0 as uint8_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != 'K' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    'K' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_1: uint32_t = 0;
            let mut eptr_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_1 = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_1 as *const ::core::ffi::c_char;
            if tmp_1 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_1,
                );
                return -1 as ::core::ffi::c_int;
            }
            keep.labelscnt = tmp_1 as uint8_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != 'A' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    'A' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_2: uint32_t = 0;
            let mut eptr_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_2 = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_2 as *const ::core::ffi::c_char;
            if tmp_2 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_2,
                );
                return -1 as ::core::ffi::c_int;
            }
            arch.labelscnt = tmp_2 as uint8_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_3: uint32_t = 0;
            let mut eptr_3: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_3 = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_3 as *const ::core::ffi::c_char;
            if tmp_3 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_3,
                );
                return -1 as ::core::ffi::c_int;
            }
            labels_mode = tmp_3 as uint8_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_4: uint32_t = 0;
            let mut eptr_4: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_4 = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_4 as *const ::core::ffi::c_char;
            if tmp_4 > 65535 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-65535 expected)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    tmp_4,
                );
                return -1 as ::core::ffi::c_int;
            }
            arch_delay = tmp_4 as uint16_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_5: uint32_t = 0;
            let mut eptr_5: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_5 = strtoul(ptr, &raw mut eptr_5, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_5 as *const ::core::ffi::c_char;
            if tmp_5 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_5,
                );
                return -1 as ::core::ffi::c_int;
            }
            adminonly = tmp_5 as uint8_t;
            if create.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                || keep.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                || arch.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
            {
                return MFS_ERROR_EINVAL;
            }
            if create.labelscnt as ::core::ffi::c_int
                + keep.labelscnt as ::core::ffi::c_int
                + arch.labelscnt as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                if *ptr as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        '-' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
            } else {
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int)
                    < create.labelscnt as ::core::ffi::c_int * MASKORGROUP
                {
                    if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    ptr = ptr.offset(1);
                    let mut eptr_6: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    old_labelmasks[i as usize] =
                        strtoul(ptr, &raw mut eptr_6, 10 as ::core::ffi::c_int) as uint32_t;
                    ptr = eptr_6 as *const ::core::ffi::c_char;
                    i = i.wrapping_add(1);
                }
                sclass_maskorgroup_to_labelexpr(
                    &raw mut create.labelexpr as *mut [uint8_t; 128],
                    &raw mut old_labelmasks as *mut uint32_t,
                    create.labelscnt,
                );
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int * MASKORGROUP
                {
                    if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    ptr = ptr.offset(1);
                    let mut eptr_7: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    old_labelmasks[i as usize] =
                        strtoul(ptr, &raw mut eptr_7, 10 as ::core::ffi::c_int) as uint32_t;
                    ptr = eptr_7 as *const ::core::ffi::c_char;
                    i = i.wrapping_add(1);
                }
                sclass_maskorgroup_to_labelexpr(
                    &raw mut keep.labelexpr as *mut [uint8_t; 128],
                    &raw mut old_labelmasks as *mut uint32_t,
                    keep.labelscnt,
                );
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < arch.labelscnt as ::core::ffi::c_int * MASKORGROUP
                {
                    if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    ptr = ptr.offset(1);
                    let mut eptr_8: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    old_labelmasks[i as usize] =
                        strtoul(ptr, &raw mut eptr_8, 10 as ::core::ffi::c_int) as uint32_t;
                    ptr = eptr_8 as *const ::core::ffi::c_char;
                    i = i.wrapping_add(1);
                }
                sclass_maskorgroup_to_labelexpr(
                    &raw mut arch.labelexpr as *mut [uint8_t; 128],
                    &raw mut old_labelmasks as *mut uint32_t,
                    arch.labelscnt,
                );
                arch_delay =
                    (arch_delay as ::core::ffi::c_int * 24 as ::core::ffi::c_int) as uint16_t;
            }
            desc[0 as usize] = 0 as uint8_t;
            priority = 0 as uint32_t;
            export_group = 0 as uint8_t;
            min_trashretention = 0 as uint16_t;
            arch_min_size = 0 as uint64_t;
            arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
        } else if *ptr as ::core::ffi::c_int == 'C' as ::core::ffi::c_int
            || *ptr as ::core::ffi::c_int == '1' as ::core::ffi::c_int
        {
            if *ptr as ::core::ffi::c_int == '1' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != '1' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        '1' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut _tmp_i_0: uint32_t = 0;
                let mut _tmp_c_0: ::core::ffi::c_char = 0;
                let mut _tmp_h1_0: ::core::ffi::c_char = 0;
                let mut _tmp_h2_0: ::core::ffi::c_char = 0;
                memset(
                    &raw mut desc as *mut uint8_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    256 as size_t,
                );
                _tmp_i_0 = 0 as uint32_t;
                loop {
                    let c2rust_fresh99 = ptr;
                    ptr = ptr.offset(1);
                    _tmp_c_0 = *c2rust_fresh99;
                    if !(_tmp_c_0 as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                        && _tmp_i_0 < 255 as uint32_t)
                    {
                        break;
                    }
                    if _tmp_c_0 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                        || _tmp_c_0 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                        || _tmp_c_0 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    if _tmp_c_0 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                        let c2rust_fresh100 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h1_0 = *c2rust_fresh100;
                        let c2rust_fresh101 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h2_0 = *c2rust_fresh101;
                        if _tmp_h1_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h1_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h1_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h1_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                                filename,
                                lv,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if _tmp_h2_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h2_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h2_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h2_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                                filename,
                                lv,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        _tmp_c_0 = (_tmp_h1_0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                            + _tmp_h2_0 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    }
                    let c2rust_fresh102 = _tmp_i_0;
                    _tmp_i_0 = _tmp_i_0.wrapping_add(1);
                    desc[c2rust_fresh102 as usize] = _tmp_c_0 as uint8_t;
                }
                ptr = ptr.offset(-1);
                desc[_tmp_i_0 as usize] = 0 as uint8_t;
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut eptr_9: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                priority = strtoul(ptr, &raw mut eptr_9, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_9 as *const ::core::ffi::c_char;
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut tmp_6: uint32_t = 0;
                let mut eptr_10: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp_6 = strtoul(ptr, &raw mut eptr_10, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_10 as *const ::core::ffi::c_char;
                if tmp_6 > 255 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"value too big (%u - 0-255 expected)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        tmp_6,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                export_group = tmp_6 as uint8_t;
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                version = 1 as uint8_t;
            } else {
                desc[0 as usize] = 0 as uint8_t;
                priority = 0 as uint32_t;
                export_group = 0 as uint8_t;
            }
            if *ptr as ::core::ffi::c_int != 'C' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    'C' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    '(' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_7: uint32_t = 0;
            let mut eptr_11: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_7 = strtoul(ptr, &raw mut eptr_11, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_11 as *const ::core::ffi::c_char;
            if tmp_7 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_7,
                );
                return -1 as ::core::ffi::c_int;
            }
            create.labelscnt = tmp_7 as uint8_t;
            if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ':' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_12: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            create.uniqmask = strtoul(ptr, &raw mut eptr_12, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_12 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ':' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut tmp_8: uint32_t = 0;
                let mut eptr_13: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp_8 = strtoul(ptr, &raw mut eptr_13, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_13 as *const ::core::ffi::c_char;
                if tmp_8 > 255 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"value too big (%u - 0-255 expected)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        tmp_8,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                create.labels_mode = tmp_8 as uint8_t;
            }
            if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != 'K' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    'K' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    '(' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_9: uint32_t = 0;
            let mut eptr_14: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_9 = strtoul(ptr, &raw mut eptr_14, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_14 as *const ::core::ffi::c_char;
            if tmp_9 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_9,
                );
                return -1 as ::core::ffi::c_int;
            }
            keep.labelscnt = tmp_9 as uint8_t;
            if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ':' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_15: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            keep.uniqmask = strtoul(ptr, &raw mut eptr_15, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_15 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ':' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut tmp_10: uint32_t = 0;
                let mut eptr_16: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp_10 = strtoul(ptr, &raw mut eptr_16, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_16 as *const ::core::ffi::c_char;
                if tmp_10 > 255 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"value too big (%u - 0-255 expected)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        tmp_10,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                keep.labels_mode = tmp_10 as uint8_t;
            }
            if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != 'A' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    'A' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    '(' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_11: uint32_t = 0;
            let mut eptr_17: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_11 = strtoul(ptr, &raw mut eptr_17, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_17 as *const ::core::ffi::c_char;
            if tmp_11 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_11,
                );
                return -1 as ::core::ffi::c_int;
            }
            arch.labelscnt = tmp_11 as uint8_t;
            if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ':' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_18: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            arch.uniqmask = strtoul(ptr, &raw mut eptr_18, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_18 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ':' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_12: uint32_t = 0;
            let mut eptr_19: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_12 = strtoul(ptr, &raw mut eptr_19, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_19 as *const ::core::ffi::c_char;
            if tmp_12 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_12,
                );
                return -1 as ::core::ffi::c_int;
            }
            arch.ec_data_chksum_parts = tmp_12 as uint8_t;
            if *ptr as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ':' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut tmp_13: uint32_t = 0;
                let mut eptr_20: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp_13 = strtoul(ptr, &raw mut eptr_20, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_20 as *const ::core::ffi::c_char;
                if tmp_13 > 255 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"value too big (%u - 0-255 expected)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        tmp_13,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                arch.labels_mode = tmp_13 as uint8_t;
            }
            if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != 'T' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    'T' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    '(' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_14: uint32_t = 0;
            let mut eptr_21: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_14 = strtoul(ptr, &raw mut eptr_21, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_21 as *const ::core::ffi::c_char;
            if tmp_14 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_14,
                );
                return -1 as ::core::ffi::c_int;
            }
            trash.labelscnt = tmp_14 as uint8_t;
            if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ':' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_22: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            trash.uniqmask = strtoul(ptr, &raw mut eptr_22, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_22 as *const ::core::ffi::c_char;
            if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ':' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_15: uint32_t = 0;
            let mut eptr_23: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_15 = strtoul(ptr, &raw mut eptr_23, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_23 as *const ::core::ffi::c_char;
            if tmp_15 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_15,
                );
                return -1 as ::core::ffi::c_int;
            }
            trash.ec_data_chksum_parts = tmp_15 as uint8_t;
            if *ptr as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ':' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut tmp_16: uint32_t = 0;
                let mut eptr_24: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp_16 = strtoul(ptr, &raw mut eptr_24, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_24 as *const ::core::ffi::c_char;
                if tmp_16 > 255 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"value too big (%u - 0-255 expected)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        tmp_16,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                trash.labels_mode = tmp_16 as uint8_t;
            }
            if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_17: uint32_t = 0;
            let mut eptr_25: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_17 = strtoul(ptr, &raw mut eptr_25, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_25 as *const ::core::ffi::c_char;
            if tmp_17 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_17,
                );
                return -1 as ::core::ffi::c_int;
            }
            labels_mode = tmp_17 as uint8_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            if *ptr as ::core::ffi::c_int == '(' as ::core::ffi::c_int {
                if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        '(' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut tmp_18: uint32_t = 0;
                let mut eptr_26: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp_18 = strtoul(ptr, &raw mut eptr_26, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_26 as *const ::core::ffi::c_char;
                if tmp_18 > 255 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"value too big (%u - 0-255 expected)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        tmp_18,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                arch_mode = tmp_18 as uint8_t;
                if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ':' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                let mut tmp_19: uint32_t = 0;
                let mut eptr_27: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp_19 = strtoul(ptr, &raw mut eptr_27, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_27 as *const ::core::ffi::c_char;
                if tmp_19 > 65535 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"value too big (%u - 0-65535 expected)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        tmp_19,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                arch_delay = tmp_19 as uint16_t;
                if *ptr as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                    if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ':' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    ptr = ptr.offset(1);
                    let mut eptr_28: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    arch_min_size =
                        strtoull(ptr, &raw mut eptr_28, 10 as ::core::ffi::c_int) as uint64_t;
                    ptr = eptr_28 as *const ::core::ffi::c_char;
                } else {
                    arch_min_size = 0 as uint64_t;
                }
                if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ')' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
            } else {
                let mut tmp_20: uint32_t = 0;
                let mut eptr_29: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmp_20 = strtoul(ptr, &raw mut eptr_29, 10 as ::core::ffi::c_int) as uint32_t;
                ptr = eptr_29 as *const ::core::ffi::c_char;
                if tmp_20 > 65535 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"value too big (%u - 0-65535 expected)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        tmp_20,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                arch_delay = tmp_20 as uint16_t;
                arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
                arch_min_size = 0 as uint64_t;
            }
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_21: uint32_t = 0;
            let mut eptr_30: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_21 = strtoul(ptr, &raw mut eptr_30, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_30 as *const ::core::ffi::c_char;
            if tmp_21 > 65535 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-65535 expected)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    tmp_21,
                );
                return -1 as ::core::ffi::c_int;
            }
            min_trashretention = tmp_21 as uint16_t;
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp_22: uint32_t = 0;
            let mut eptr_31: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp_22 = strtoul(ptr, &raw mut eptr_31, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_31 as *const ::core::ffi::c_char;
            if tmp_22 > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp_22,
                );
                return -1 as ::core::ffi::c_int;
            }
            adminonly = tmp_22 as uint8_t;
            if create.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                || keep.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                || arch.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                || trash.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
            {
                return MFS_ERROR_EINVAL;
            }
            if create.labelscnt as ::core::ffi::c_int
                + keep.labelscnt as ::core::ffi::c_int
                + arch.labelscnt as ::core::ffi::c_int
                + trash.labelscnt as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ',' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                if *ptr as ::core::ffi::c_int != '-' as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        '-' as ::core::ffi::c_int,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
            } else {
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < create.labelscnt as ::core::ffi::c_int {
                    if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    ptr = ptr.offset(1);
                    let mut _tmp_h1_1: ::core::ffi::c_char = 0;
                    let mut _tmp_h2_1: ::core::ffi::c_char = 0;
                    labelexpr_leng = 0 as uint32_t;
                    loop {
                        let c2rust_fresh103 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h1_1 = *c2rust_fresh103;
                        if _tmp_h1_1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h1_1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h1_1 = (_tmp_h1_1 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h1_1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h1_1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h1_1 = (_tmp_h1_1 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            ptr = ptr.offset(-1);
                            break;
                        }
                        let c2rust_fresh104 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h2_1 = *c2rust_fresh104;
                        if _tmp_h2_1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h2_1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h2_1 = (_tmp_h2_1 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h2_1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h2_1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h2_1 = (_tmp_h2_1 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            ptr = ptr.offset(-1);
                            break;
                        }
                        _tmp_h1_1 = (_tmp_h1_1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                            + _tmp_h2_1 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                        if labelexpr_leng >= labelexpr_size {
                            labelexpr_size = labelexpr_leng.wrapping_add(1000 as uint32_t);
                            if labelexpr_buff.is_null() {
                                labelexpr_buff = malloc(labelexpr_size as size_t) as *mut uint8_t;
                            } else {
                                let mut _tmp_buff: *mut uint8_t = labelexpr_buff;
                                labelexpr_buff = realloc(
                                    labelexpr_buff as *mut ::core::ffi::c_void,
                                    labelexpr_size as size_t,
                                ) as *mut uint8_t;
                                if labelexpr_buff.is_null() {
                                    free(_tmp_buff as *mut ::core::ffi::c_void);
                                }
                            }
                            if labelexpr_buff.is_null() {
                                fprintf(
                                    stderr,
                                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1754 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1754 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                abort();
                            } else if labelexpr_buff
                                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                    -1 as ::core::ffi::c_int as usize,
                                ) as *mut uint8_t
                            {
                                let mut _mfs_errorstring: *const ::core::ffi::c_char =
                                    strerr(*__errno_location());
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1754 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1754 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring,
                                );
                                abort();
                            }
                        }
                        let c2rust_fresh105 = labelexpr_leng;
                        labelexpr_leng = labelexpr_leng.wrapping_add(1);
                        *labelexpr_buff.offset(c2rust_fresh105 as isize) = _tmp_h1_1 as uint8_t;
                    }
                    if labelexpr_leng > SCLASS_EXPR_MAX_SIZE as uint32_t {
                        return MFS_ERROR_EINVAL;
                    }
                    memcpy(
                        &raw mut *(&raw mut create.labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        labelexpr_buff as *const ::core::ffi::c_void,
                        labelexpr_leng as size_t,
                    );
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int {
                    if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    ptr = ptr.offset(1);
                    let mut _tmp_h1_2: ::core::ffi::c_char = 0;
                    let mut _tmp_h2_2: ::core::ffi::c_char = 0;
                    labelexpr_leng = 0 as uint32_t;
                    loop {
                        let c2rust_fresh106 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h1_2 = *c2rust_fresh106;
                        if _tmp_h1_2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h1_2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h1_2 = (_tmp_h1_2 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h1_2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h1_2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h1_2 = (_tmp_h1_2 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            ptr = ptr.offset(-1);
                            break;
                        }
                        let c2rust_fresh107 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h2_2 = *c2rust_fresh107;
                        if _tmp_h2_2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h2_2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h2_2 = (_tmp_h2_2 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h2_2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h2_2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h2_2 = (_tmp_h2_2 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            ptr = ptr.offset(-1);
                            break;
                        }
                        _tmp_h1_2 = (_tmp_h1_2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                            + _tmp_h2_2 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                        if labelexpr_leng >= labelexpr_size {
                            labelexpr_size = labelexpr_leng.wrapping_add(1000 as uint32_t);
                            if labelexpr_buff.is_null() {
                                labelexpr_buff = malloc(labelexpr_size as size_t) as *mut uint8_t;
                            } else {
                                let mut _tmp_buff_0: *mut uint8_t = labelexpr_buff;
                                labelexpr_buff = realloc(
                                    labelexpr_buff as *mut ::core::ffi::c_void,
                                    labelexpr_size as size_t,
                                ) as *mut uint8_t;
                                if labelexpr_buff.is_null() {
                                    free(_tmp_buff_0 as *mut ::core::ffi::c_void);
                                }
                            }
                            if labelexpr_buff.is_null() {
                                fprintf(
                                    stderr,
                                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                abort();
                            } else if labelexpr_buff
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
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring_0,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring_0,
                                );
                                abort();
                            }
                        }
                        let c2rust_fresh108 = labelexpr_leng;
                        labelexpr_leng = labelexpr_leng.wrapping_add(1);
                        *labelexpr_buff.offset(c2rust_fresh108 as isize) = _tmp_h1_2 as uint8_t;
                    }
                    if labelexpr_leng > SCLASS_EXPR_MAX_SIZE as uint32_t {
                        return MFS_ERROR_EINVAL;
                    }
                    memcpy(
                        &raw mut *(&raw mut keep.labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        labelexpr_buff as *const ::core::ffi::c_void,
                        labelexpr_leng as size_t,
                    );
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < arch.labelscnt as ::core::ffi::c_int {
                    if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    ptr = ptr.offset(1);
                    let mut _tmp_h1_3: ::core::ffi::c_char = 0;
                    let mut _tmp_h2_3: ::core::ffi::c_char = 0;
                    labelexpr_leng = 0 as uint32_t;
                    loop {
                        let c2rust_fresh109 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h1_3 = *c2rust_fresh109;
                        if _tmp_h1_3 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h1_3 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h1_3 = (_tmp_h1_3 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h1_3 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h1_3 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h1_3 = (_tmp_h1_3 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            ptr = ptr.offset(-1);
                            break;
                        }
                        let c2rust_fresh110 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h2_3 = *c2rust_fresh110;
                        if _tmp_h2_3 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h2_3 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h2_3 = (_tmp_h2_3 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h2_3 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h2_3 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h2_3 = (_tmp_h2_3 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            ptr = ptr.offset(-1);
                            break;
                        }
                        _tmp_h1_3 = (_tmp_h1_3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                            + _tmp_h2_3 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                        if labelexpr_leng >= labelexpr_size {
                            labelexpr_size = labelexpr_leng.wrapping_add(1000 as uint32_t);
                            if labelexpr_buff.is_null() {
                                labelexpr_buff = malloc(labelexpr_size as size_t) as *mut uint8_t;
                            } else {
                                let mut _tmp_buff_1: *mut uint8_t = labelexpr_buff;
                                labelexpr_buff = realloc(
                                    labelexpr_buff as *mut ::core::ffi::c_void,
                                    labelexpr_size as size_t,
                                ) as *mut uint8_t;
                                if labelexpr_buff.is_null() {
                                    free(_tmp_buff_1 as *mut ::core::ffi::c_void);
                                }
                            }
                            if labelexpr_buff.is_null() {
                                fprintf(
                                    stderr,
                                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                abort();
                            } else if labelexpr_buff
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
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring_1,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1770 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring_1,
                                );
                                abort();
                            }
                        }
                        let c2rust_fresh111 = labelexpr_leng;
                        labelexpr_leng = labelexpr_leng.wrapping_add(1);
                        *labelexpr_buff.offset(c2rust_fresh111 as isize) = _tmp_h1_3 as uint8_t;
                    }
                    if labelexpr_leng > SCLASS_EXPR_MAX_SIZE as uint32_t {
                        return MFS_ERROR_EINVAL;
                    }
                    memcpy(
                        &raw mut *(&raw mut arch.labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        labelexpr_buff as *const ::core::ffi::c_void,
                        labelexpr_leng as size_t,
                    );
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < trash.labelscnt as ::core::ffi::c_int {
                    if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                            filename,
                            lv,
                            ',' as ::core::ffi::c_int,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    ptr = ptr.offset(1);
                    let mut _tmp_h1_4: ::core::ffi::c_char = 0;
                    let mut _tmp_h2_4: ::core::ffi::c_char = 0;
                    labelexpr_leng = 0 as uint32_t;
                    loop {
                        let c2rust_fresh112 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h1_4 = *c2rust_fresh112;
                        if _tmp_h1_4 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h1_4 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h1_4 = (_tmp_h1_4 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h1_4 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h1_4 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h1_4 = (_tmp_h1_4 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            ptr = ptr.offset(-1);
                            break;
                        }
                        let c2rust_fresh113 = ptr;
                        ptr = ptr.offset(1);
                        _tmp_h2_4 = *c2rust_fresh113;
                        if _tmp_h2_4 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && _tmp_h2_4 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                        {
                            _tmp_h2_4 = (_tmp_h2_4 as ::core::ffi::c_int
                                - '0' as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        } else if _tmp_h2_4 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                            && _tmp_h2_4 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            _tmp_h2_4 = (_tmp_h2_4 as ::core::ffi::c_int
                                - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                                as ::core::ffi::c_char;
                        } else {
                            ptr = ptr.offset(-1);
                            break;
                        }
                        _tmp_h1_4 = (_tmp_h1_4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                            + _tmp_h2_4 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                        if labelexpr_leng >= labelexpr_size {
                            labelexpr_size = labelexpr_leng.wrapping_add(1000 as uint32_t);
                            if labelexpr_buff.is_null() {
                                labelexpr_buff = malloc(labelexpr_size as size_t) as *mut uint8_t;
                            } else {
                                let mut _tmp_buff_2: *mut uint8_t = labelexpr_buff;
                                labelexpr_buff = realloc(
                                    labelexpr_buff as *mut ::core::ffi::c_void,
                                    labelexpr_size as size_t,
                                ) as *mut uint8_t;
                                if labelexpr_buff.is_null() {
                                    free(_tmp_buff_2 as *mut ::core::ffi::c_void);
                                }
                            }
                            if labelexpr_buff.is_null() {
                                fprintf(
                                    stderr,
                                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1778 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1778 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                abort();
                            } else if labelexpr_buff
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
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1778 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring_2,
                                );
                                fprintf(
                                    stderr,
                                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    1778 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    b"labelexpr_buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    _mfs_errorstring_2,
                                );
                                abort();
                            }
                        }
                        let c2rust_fresh114 = labelexpr_leng;
                        labelexpr_leng = labelexpr_leng.wrapping_add(1);
                        *labelexpr_buff.offset(c2rust_fresh114 as isize) = _tmp_h1_4 as uint8_t;
                    }
                    if labelexpr_leng > SCLASS_EXPR_MAX_SIZE as uint32_t {
                        return MFS_ERROR_EINVAL;
                    }
                    memcpy(
                        &raw mut *(&raw mut trash.labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        labelexpr_buff as *const ::core::ffi::c_void,
                        labelexpr_leng as size_t,
                    );
                    i = i.wrapping_add(1);
                }
            }
        } else {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"wrong storage class format ('%c' found - 'W' or 'C' expected)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                *ptr as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_23: uint32_t = 0;
        let mut eptr_32: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_23 = strtoul(ptr, &raw mut eptr_32, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_32 as *const ::core::ffi::c_char;
        if tmp_23 > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_23,
            );
            return -1 as ::core::ffi::c_int;
        }
        spid = tmp_23 as uint16_t;
        if version as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if (spid as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                export_group = spid as uint8_t;
            }
        }
        return sclass_mr_set_entry(
            strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut name as *mut uint8_t,
            spid,
            new_flag,
            strlen(&raw mut desc as *mut uint8_t as *mut ::core::ffi::c_char) as uint8_t,
            &raw mut desc as *mut uint8_t,
            priority,
            export_group,
            adminonly,
            labels_mode,
            arch_mode,
            arch_delay,
            arch_min_size,
            min_trashretention,
            &raw mut create,
            &raw mut keep,
            &raw mut arch,
            &raw mut trash,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_trash_recover(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut parent: uint32_t = 0;
        let mut cumask: uint16_t = 0;
        let mut uid: uint32_t = 0;
        let mut gid: uint32_t = 0;
        let mut copysgid: uint8_t = 0;
        static mut path: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut pathsize: uint32_t = 0 as uint32_t;
        static mut created_path: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut created_pathsize: uint32_t = 0 as uint32_t;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        parent = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh115 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh115;
            if _tmp_c as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh116 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh116;
                let c2rust_fresh117 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh117;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if _tmp_i >= pathsize {
                pathsize = _tmp_i.wrapping_add(1000 as uint32_t);
                if path.is_null() {
                    path = malloc(pathsize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_path: *mut uint8_t = path;
                    path = realloc(path as *mut ::core::ffi::c_void, pathsize as size_t)
                        as *mut uint8_t;
                    if path.is_null() {
                        free(_tmp_path as *mut ::core::ffi::c_void);
                    }
                }
                if path.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if path
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            let c2rust_fresh118 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            *path.offset(c2rust_fresh118 as isize) = _tmp_c as uint8_t;
        }
        if _tmp_i >= pathsize {
            pathsize = _tmp_i.wrapping_add(1000 as uint32_t);
            if path.is_null() {
                path = malloc(pathsize as size_t) as *mut uint8_t;
            } else {
                let mut _tmp_path_0: *mut uint8_t = path;
                path =
                    realloc(path as *mut ::core::ffi::c_void, pathsize as size_t) as *mut uint8_t;
                if path.is_null() {
                    free(_tmp_path_0 as *mut ::core::ffi::c_void);
                }
            }
            if path.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if path
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        ptr = ptr.offset(-1);
        *path.offset(_tmp_i as isize) = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp: uint32_t = 0;
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        if tmp > 65535 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-65535 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp,
            );
            return -1 as ::core::ffi::c_int;
        }
        cumask = tmp as uint16_t;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        uid = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_2 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        gid = strtoul(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut tmp_0: uint32_t = 0;
        let mut eptr_4: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp_0 = strtoul(ptr, &raw mut eptr_4, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_4 as *const ::core::ffi::c_char;
        if tmp_0 > 255 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                tmp_0,
            );
            return -1 as ::core::ffi::c_int;
        }
        copysgid = tmp_0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i_0: uint32_t = 0;
        let mut _tmp_c_0: ::core::ffi::c_char = 0;
        let mut _tmp_h1_0: ::core::ffi::c_char = 0;
        let mut _tmp_h2_0: ::core::ffi::c_char = 0;
        _tmp_i_0 = 0 as uint32_t;
        loop {
            let c2rust_fresh119 = ptr;
            ptr = ptr.offset(1);
            _tmp_c_0 = *c2rust_fresh119;
            if _tmp_c_0 as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
                break;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c_0 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c_0 as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh120 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1_0 = *c2rust_fresh120;
                let c2rust_fresh121 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2_0 = *c2rust_fresh121;
                if _tmp_h1_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1_0 = (_tmp_h1_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2_0 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2_0 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2_0 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2_0 = (_tmp_h2_0 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c_0 = (_tmp_h1_0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2_0 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            if _tmp_i_0 >= created_pathsize {
                created_pathsize = _tmp_i_0.wrapping_add(1000 as uint32_t);
                if created_path.is_null() {
                    created_path = malloc(created_pathsize as size_t) as *mut uint8_t;
                } else {
                    let mut _tmp_path_1: *mut uint8_t = created_path;
                    created_path = realloc(
                        created_path as *mut ::core::ffi::c_void,
                        created_pathsize as size_t,
                    ) as *mut uint8_t;
                    if created_path.is_null() {
                        free(_tmp_path_1 as *mut ::core::ffi::c_void);
                    }
                }
                if created_path.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"created_path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"created_path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if created_path
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
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"created_path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"created_path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    abort();
                }
            }
            let c2rust_fresh122 = _tmp_i_0;
            _tmp_i_0 = _tmp_i_0.wrapping_add(1);
            *created_path.offset(c2rust_fresh122 as isize) = _tmp_c_0 as uint8_t;
        }
        if _tmp_i_0 >= created_pathsize {
            created_pathsize = _tmp_i_0.wrapping_add(1000 as uint32_t);
            if created_path.is_null() {
                created_path = malloc(created_pathsize as size_t) as *mut uint8_t;
            } else {
                let mut _tmp_path_2: *mut uint8_t = created_path;
                created_path = realloc(
                    created_path as *mut ::core::ffi::c_void,
                    created_pathsize as size_t,
                ) as *mut uint8_t;
                if created_path.is_null() {
                    free(_tmp_path_2 as *mut ::core::ffi::c_void);
                }
            }
            if created_path.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"created_path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"created_path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if created_path
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"created_path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/restore.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1827 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"created_path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                abort();
            }
        }
        ptr = ptr.offset(-1);
        *created_path.offset(_tmp_i_0 as isize) = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_trash_recover(
            ts,
            inode,
            parent,
            strlen(path as *mut ::core::ffi::c_char) as uint32_t,
            path,
            cumask,
            uid,
            gid,
            copysgid,
            strlen(created_path as *mut ::core::ffi::c_char) as uint32_t,
            created_path,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_trash_remove(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_trash_remove(ts, inode) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_undel(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_undel(ts, inode) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_unlink(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut parent: uint32_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        parent = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut _tmp_i: uint32_t = 0;
        let mut _tmp_c: ::core::ffi::c_char = 0;
        let mut _tmp_h1: ::core::ffi::c_char = 0;
        let mut _tmp_h2: ::core::ffi::c_char = 0;
        memset(
            &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            256 as size_t,
        );
        _tmp_i = 0 as uint32_t;
        loop {
            let c2rust_fresh123 = ptr;
            ptr = ptr.offset(1);
            _tmp_c = *c2rust_fresh123;
            if !(_tmp_c as ::core::ffi::c_int != ')' as ::core::ffi::c_int
                && _tmp_i < 255 as uint32_t)
            {
                break;
            }
            if _tmp_c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || _tmp_c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ')' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            if _tmp_c as ::core::ffi::c_int == '%' as ::core::ffi::c_int {
                let c2rust_fresh124 = ptr;
                ptr = ptr.offset(1);
                _tmp_h1 = *c2rust_fresh124;
                let c2rust_fresh125 = ptr;
                ptr = ptr.offset(1);
                _tmp_h2 = *c2rust_fresh125;
                if _tmp_h1 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h1 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h1 = (_tmp_h1 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if _tmp_h2 as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else if _tmp_h2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                    && _tmp_h2 as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                {
                    _tmp_h2 = (_tmp_h2 as ::core::ffi::c_int
                        - ('A' as ::core::ffi::c_int - 10 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: hex expected\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                _tmp_c = (_tmp_h1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                    + _tmp_h2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
            }
            let c2rust_fresh126 = _tmp_i;
            _tmp_i = _tmp_i.wrapping_add(1);
            name[c2rust_fresh126 as usize] = _tmp_c as uint8_t;
        }
        ptr = ptr.offset(-1);
        name[_tmp_i as usize] = 0 as uint8_t;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        return fs_mr_unlink(
            ts,
            parent,
            strlen(&raw mut name as *mut uint8_t as *mut ::core::ffi::c_char) as uint32_t,
            &raw mut name as *mut uint8_t,
            inode,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_unlock(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut chunkid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        return fs_mr_unlock(ts, chunkid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_trunc(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        indx = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_1 as *const ::core::ffi::c_char;
        return fs_mr_trunc(ts, inode, indx, chunkid) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_write(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut ts: uint32_t,
    mut ptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut opflag: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut canmodmtime: uint8_t = 0;
        if *ptr as ::core::ffi::c_int != '(' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '(' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ',' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        indx = strtoul(ptr, &raw mut eptr_0, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr_0 as *const ::core::ffi::c_char;
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut eptr_1: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            opflag = strtoul(ptr, &raw mut eptr_1, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_1 as *const ::core::ffi::c_char;
        } else {
            opflag = 1 as uint32_t;
        }
        if *ptr as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
            if *ptr as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                    lv,
                    ',' as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            let mut tmp: uint32_t = 0;
            let mut eptr_2: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp = strtoul(ptr, &raw mut eptr_2, 10 as ::core::ffi::c_int) as uint32_t;
            ptr = eptr_2 as *const ::core::ffi::c_char;
            if tmp > 255 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"value too big (%u - 0-255 expected)\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp,
                );
                return -1 as ::core::ffi::c_int;
            }
            canmodmtime = tmp as uint8_t;
        } else {
            canmodmtime = 1 as uint8_t;
        }
        if *ptr as ::core::ffi::c_int != ')' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ')' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                ':' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        let mut eptr_3: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunkid = strtoull(ptr, &raw mut eptr_3, 10 as ::core::ffi::c_int) as uint64_t;
        ptr = eptr_3 as *const ::core::ffi::c_char;
        return fs_mr_write(ts, inode, indx, opflag as uint8_t, canmodmtime, chunkid)
            as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_line(
    mut filename: *const ::core::ffi::c_char,
    mut lv: uint64_t,
    mut line: *const ::core::ffi::c_char,
    mut rts: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut ts: uint32_t = 0;
        let mut hc: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        status = MFS_ERROR_MISMATCH;
        ptr = line;
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ts = strtoul(ptr, &raw mut eptr, 10 as ::core::ffi::c_int) as uint32_t;
        ptr = eptr as *const ::core::ffi::c_char;
        if !rts.is_null() {
            *rts = ts;
        }
        if *ptr as ::core::ffi::c_int != '|' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"%s:%lu: '%c' expected\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
                lv,
                '|' as ::core::ffi::c_int,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = ptr.offset(1);
        hc = (*(ptr as *mut uint8_t).offset(0 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*(ptr as *mut uint8_t).offset(1 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*(ptr as *mut uint8_t).offset(2 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*(ptr as *mut uint8_t).offset(3 as isize) as ::core::ffi::c_uint)
            as uint32_t;
        match hc {
            1229212741 => {
                return do_idle(
                    filename,
                    lv,
                    ts,
                    ptr.offset(4 as ::core::ffi::c_int as isize),
                );
            }
            1094927173 => {
                if strncmp(
                    ptr,
                    b"ACCESS\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_access(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1094992961 => {
                if strncmp(
                    ptr,
                    b"ADDATTR\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_addattr(
                        filename,
                        lv,
                        ts,
                        ptr.offset(7 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1096045650 => {
                return do_attr(
                    filename,
                    lv,
                    ts,
                    ptr.offset(4 as ::core::ffi::c_int as isize),
                );
            }
            1095782469 => {
                if strncmp(
                    ptr,
                    b"APPEND\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_append(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1094930773 => {
                if strncmp(
                    ptr,
                    b"ACQUIRE\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_acquire(
                        filename,
                        lv,
                        ts,
                        ptr.offset(7 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1095849289 => {
                if strncmp(
                    ptr,
                    b"AQUIRE\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_acquire(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1095910216 => {
                if strncmp(
                    ptr,
                    b"ARCHCHG\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_archchg(
                        filename,
                        lv,
                        ts,
                        ptr.offset(7 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1095586889 => {
                if strncmp(
                    ptr,
                    b"AMTIME\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_amtime(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1096111183 => {
                if strncmp(
                    ptr,
                    b"AUTOARCH\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_autoarch(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1129465153 => {
                if strncmp(
                    ptr,
                    b"CREATE\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_create(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1128813902 => {
                if strncmp(
                    ptr,
                    b"CHUNKADD\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_chunkadd(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                } else if strncmp(
                    ptr,
                    b"CHUNKDEL\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_chunkdel(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                } else if strncmp(
                    ptr,
                    b"CHUNKFLAGSCLR\0".as_ptr() as *const ::core::ffi::c_char,
                    13 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_chunkflagsclr(
                        filename,
                        lv,
                        ts,
                        ptr.offset(13 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1129529668 => {
                if strncmp(
                    ptr,
                    b"CSADD\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_csadd(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1129530434 => {
                if strncmp(
                    ptr,
                    b"CSDBOP\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_csdbop(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1129530437 => {
                if strncmp(
                    ptr,
                    b"CSDEL\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_csdel(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1129665364 => {
                if strncmp(
                    ptr,
                    b"CUSTOMER\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_session(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1162694740 => {
                if strncmp(
                    ptr,
                    b"EMPTYTRASH\0".as_ptr() as *const ::core::ffi::c_char,
                    10 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_emptytrash(
                        filename,
                        lv,
                        ts,
                        ptr.offset(10 as ::core::ffi::c_int as isize),
                    );
                } else if strncmp(
                    ptr,
                    b"EMPTYSUSTAINED\0".as_ptr() as *const ::core::ffi::c_char,
                    14 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_emptysustained(
                        filename,
                        lv,
                        ts,
                        ptr.offset(14 as ::core::ffi::c_int as isize),
                    );
                } else if strncmp(
                    ptr,
                    b"EMPTYRESERVED\0".as_ptr() as *const ::core::ffi::c_char,
                    13 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_emptysustained(
                        filename,
                        lv,
                        ts,
                        ptr.offset(13 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1179406147 => {
                if strncmp(
                    ptr,
                    b"FLOCK\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_flock(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1179796805 => {
                if strncmp(
                    ptr,
                    b"FREEINODES\0".as_ptr() as *const ::core::ffi::c_char,
                    10 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_freeinodes(
                        filename,
                        lv,
                        ts,
                        ptr.offset(10 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1229865814 => {
                if strncmp(
                    ptr,
                    b"INCVERSION\0".as_ptr() as *const ::core::ffi::c_char,
                    10 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_incversion(
                        filename,
                        lv,
                        ts,
                        ptr.offset(10 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1279610439 => {
                if strncmp(
                    ptr,
                    b"LENGTH\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_length(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1279872587 => {
                return do_link(
                    filename,
                    lv,
                    ts,
                    ptr.offset(4 as ::core::ffi::c_int as isize),
                );
            }
            1297045061 => {
                return do_move(
                    filename,
                    lv,
                    ts,
                    ptr.offset(4 as ::core::ffi::c_int as isize),
                );
            }
            1313167444 => {
                if strncmp(
                    ptr,
                    b"NEXTCHUNKID\0".as_ptr() as *const ::core::ffi::c_char,
                    11 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_nextchunkid(
                        filename,
                        lv,
                        ts,
                        ptr.offset(11 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1346458689 => {
                if strncmp(
                    ptr,
                    b"PATADD\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_patadd(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1346458692 => {
                if strncmp(
                    ptr,
                    b"PATDEL\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_patdel(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1347375945 => {
                if strncmp(
                    ptr,
                    b"POSIXLOCK\0".as_ptr() as *const ::core::ffi::c_char,
                    9 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_posixlock(
                        filename,
                        lv,
                        ts,
                        ptr.offset(9 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1347768903 => {
                if strncmp(
                    ptr,
                    b"PURGE\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_purge(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1364545364 => {
                if strncmp(
                    ptr,
                    b"QUOTA\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_quota(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1380273221 => {
                if strncmp(
                    ptr,
                    b"RELEASE\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_release(
                        filename,
                        lv,
                        ts,
                        ptr.offset(7 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1380273749 => {
                if strncmp(
                    ptr,
                    b"RENUMERATEEDGES\0".as_ptr() as *const ::core::ffi::c_char,
                    15 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_renumedges(
                        filename,
                        lv,
                        ts,
                        ptr.offset(15 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1380274241 => {
                if strncmp(
                    ptr,
                    b"REPAIR\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_repair(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1380928588 => {
                if strncmp(
                    ptr,
                    b"ROLLBACK\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_rollback(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1396917317 => {
                if strncmp(
                    ptr,
                    b"SCDEL\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_scdel(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1396917333 => {
                if strncmp(
                    ptr,
                    b"SCDUP\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_scdup(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1396917571 => {
                if strncmp(
                    ptr,
                    b"SCECVERSION\0".as_ptr() as *const ::core::ffi::c_char,
                    11 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_scecversion(
                        filename,
                        lv,
                        ts,
                        ptr.offset(11 as ::core::ffi::c_int as isize),
                    );
                } else if strncmp(
                    ptr,
                    b"SCECON\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_scecon(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1396920897 => {
                if strncmp(
                    ptr,
                    b"SCRAIDON\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_scecon(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1396920901 => {
                if strncmp(
                    ptr,
                    b"SCREN\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_scren(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1396921157 => {
                if strncmp(
                    ptr,
                    b"SCSET\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_scset(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052225 => {
                if strncmp(
                    ptr,
                    b"SESADD\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_sesadd(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052227 => {
                if strncmp(
                    ptr,
                    b"SESCHANGED\0".as_ptr() as *const ::core::ffi::c_char,
                    10 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_seschanged(
                        filename,
                        lv,
                        ts,
                        ptr.offset(10 as ::core::ffi::c_int as isize),
                    );
                } else if strncmp(
                    ptr,
                    b"SESCONNECTED\0".as_ptr() as *const ::core::ffi::c_char,
                    12 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_sesconnected(
                        filename,
                        lv,
                        ts,
                        ptr.offset(12 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052228 => {
                if strncmp(
                    ptr,
                    b"SESDEL\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_sesdel(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                } else if strncmp(
                    ptr,
                    b"SESDISCONNECTED\0".as_ptr() as *const ::core::ffi::c_char,
                    15 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_sesdisconnected(
                        filename,
                        lv,
                        ts,
                        ptr.offset(15 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052243 => {
                if strncmp(
                    ptr,
                    b"SESSION\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_session(
                        filename,
                        lv,
                        ts,
                        ptr.offset(7 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052481 => {
                if strncmp(
                    ptr,
                    b"SETACL\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_setacl(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052485 => {
                if strncmp(
                    ptr,
                    b"SETEATTR\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_seteattr(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052486 => {
                if strncmp(
                    ptr,
                    b"SETFILECHUNK\0".as_ptr() as *const ::core::ffi::c_char,
                    12 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_setfilechunk(
                        filename,
                        lv,
                        ts,
                        ptr.offset(12 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052487 => {
                if strncmp(
                    ptr,
                    b"SETGOAL\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_setgoal(
                        filename,
                        lv,
                        ts,
                        ptr.offset(7 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052493 => {
                if strncmp(
                    ptr,
                    b"SETMETAID\0".as_ptr() as *const ::core::ffi::c_char,
                    9 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_setmetaid(
                        filename,
                        lv,
                        ts,
                        ptr.offset(9 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052496 => {
                if strncmp(
                    ptr,
                    b"SETPATH\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_setpath(
                        filename,
                        lv,
                        ts,
                        ptr.offset(7 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052499 => {
                if strncmp(
                    ptr,
                    b"SETSCLASS\0".as_ptr() as *const ::core::ffi::c_char,
                    9 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_setsclass(
                        filename,
                        lv,
                        ts,
                        ptr.offset(9 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052500 => {
                if strncmp(
                    ptr,
                    b"SETTRASHTIME\0".as_ptr() as *const ::core::ffi::c_char,
                    12 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_settrashretention(
                        filename,
                        lv,
                        ts,
                        ptr.offset(12 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052502 => {
                if strncmp(
                    ptr,
                    b"SETVERSION\0".as_ptr() as *const ::core::ffi::c_char,
                    10 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_setversion(
                        filename,
                        lv,
                        ts,
                        ptr.offset(10 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397052504 => {
                if strncmp(
                    ptr,
                    b"SETXATTR\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_setxattr(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1397637456 => {
                if strncmp(
                    ptr,
                    b"SNAPSHOT\0".as_ptr() as *const ::core::ffi::c_char,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_snapshot(
                        filename,
                        lv,
                        ts,
                        ptr.offset(8 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1398361420 => {
                if strncmp(
                    ptr,
                    b"SYMLINK\0".as_ptr() as *const ::core::ffi::c_char,
                    7 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_symlink(
                        filename,
                        lv,
                        ts,
                        ptr.offset(7 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1414676819 => {
                if strncmp(
                    ptr,
                    b"TRASH_RECOVER\0".as_ptr() as *const ::core::ffi::c_char,
                    13 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_trash_recover(
                        filename,
                        lv,
                        ts,
                        ptr.offset(13 as ::core::ffi::c_int as isize),
                    );
                } else if strncmp(
                    ptr,
                    b"TRASH_REMOVE\0".as_ptr() as *const ::core::ffi::c_char,
                    12 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_trash_remove(
                        filename,
                        lv,
                        ts,
                        ptr.offset(12 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1414681934 => {
                if strncmp(
                    ptr,
                    b"TRUNC\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_trunc(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1431192645 => {
                if strncmp(
                    ptr,
                    b"UNDEL\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_undel(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1431194697 => {
                if strncmp(
                    ptr,
                    b"UNLINK\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_unlink(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1431194703 => {
                if strncmp(
                    ptr,
                    b"UNLOCK\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_unlock(
                        filename,
                        lv,
                        ts,
                        ptr.offset(6 as ::core::ffi::c_int as isize),
                    );
                }
            }
            1465010516 => {
                if strncmp(
                    ptr,
                    b"WRITE\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    return do_write(
                        filename,
                        lv,
                        ts,
                        ptr.offset(5 as ::core::ffi::c_int as isize),
                    );
                }
            }
            _ => {}
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"%s:%lu: unknown entry '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
            lv,
            ptr,
        );
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_net(
    mut lv: uint64_t,
    mut ptr: *const ::core::ffi::c_char,
    mut rts: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        if lv != meta_version() {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"desync - invalid meta version (version in packet: %lu / expected: %lu / packet data: %s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                lv,
                meta_version(),
                ptr,
            );
            return -1 as ::core::ffi::c_int;
        }
        status = restore_line(
            b"NET\0".as_ptr() as *const ::core::ffi::c_char,
            lv,
            ptr,
            rts,
        );
        if status < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"desync - operation (%s) parse error\0".as_ptr() as *const ::core::ffi::c_char,
                ptr,
            );
            return -1 as ::core::ffi::c_int;
        }
        if status != MFS_STATUS_OK {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"desync - operation (%s) error: %d (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                ptr,
                status,
                mfsstrerr(status as uint8_t),
            );
            return -1 as ::core::ffi::c_int;
        }
        if lv.wrapping_add(1 as uint64_t) != meta_version() {
            if lv.wrapping_add(1 as uint64_t) > meta_version() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"desync - meta version has not been increased after the operation (%s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ptr,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"desync - meta version has been increased more then once after the operation (%s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ptr,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
static mut v: uint64_t = 0 as uint64_t;
static mut lastv: uint64_t = 0 as uint64_t;
static mut lastshfn: *mut ::core::ffi::c_void = NULL;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_file(
    mut shfilename: *mut ::core::ffi::c_void,
    mut lv: uint64_t,
    mut ptr: *const ::core::ffi::c_char,
    mut vlevel: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut lastfn: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut filename: *mut ::core::ffi::c_char =
            shp_get(shfilename) as *mut ::core::ffi::c_char;
        if lastv == 0 as uint64_t || v == 0 as uint64_t || lastshfn.is_null() {
            v = meta_version();
            lastv = lv.wrapping_sub(1 as uint64_t);
            lastfn =
                b"(no file)\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else {
            lastfn = shp_get(lastshfn) as *mut ::core::ffi::c_char;
        }
        if vlevel as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"filename: %s ; current meta version: %lu ; previous changeid: %lu ; current changeid: %lu ; change data%s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                filename,
                v,
                lastv,
                lv,
                ptr,
            );
        }
        if lv < lastv {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"merge error - possibly corrupted input file - ignore entry (filename: %s)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                filename,
            );
            return 0 as ::core::ffi::c_int;
        } else if lv >= v {
            if lv == lastv {
                if vlevel as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_INFO,
                        b"duplicated entry: %lu (previous file: %s, current file: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        lv,
                        lastfn,
                        filename,
                    );
                }
            } else if lv > lastv.wrapping_add(1 as uint64_t) {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"hole in change files (entries from %s:%lu to %s:%lu are missing) - add more files\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    lastfn,
                    lastv.wrapping_add(1 as uint64_t),
                    filename,
                    lv.wrapping_sub(1 as uint64_t),
                );
                return -2 as ::core::ffi::c_int;
            } else {
                if vlevel as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_INFO,
                        b"%s: change%s\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        ptr,
                    );
                }
                status = restore_line(filename, lv, ptr, ::core::ptr::null_mut::<uint32_t>());
                if status < 0 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
                if status > 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: operation (%s) error: %d (%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        filename,
                        lv,
                        ptr,
                        status,
                        mfsstrerr(status as uint8_t),
                    );
                    return -1 as ::core::ffi::c_int;
                }
                v = meta_version();
                if lv.wrapping_add(1 as uint64_t) != v {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"%s:%lu: version mismatch\n\0".as_ptr() as *const ::core::ffi::c_char,
                        filename,
                        lv,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
        }
        lastv = lv;
        if shfilename != lastshfn {
            shp_inc(shfilename);
            if !lastshfn.is_null() {
                shp_dec(lastshfn);
            }
            lastshfn = shfilename;
        }
        return 0 as ::core::ffi::c_int;
    }
}
