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
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn random() -> ::core::ffi::c_long;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn sclass_maskorgroup_to_labelexpr(
        labelexpr: *mut [uint8_t; 128],
        labelmasks: *mut uint32_t,
        labelscnt: uint8_t,
    );
    unsafe fn sclass_labelexpr_to_maskorgroup(
        labelmasks: *mut uint32_t,
        labelexpr: *mut [uint8_t; 128],
        labelscnt: uint8_t,
    ) -> uint8_t;
    unsafe fn sclass_ec_version() -> uint8_t;
    unsafe fn sclass_info(buff: *mut uint8_t, fver: uint8_t) -> uint32_t;
    unsafe fn sclass_create_entry(
        nleng: uint8_t,
        name: *const uint8_t,
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
    unsafe fn sclass_change_entry(
        nleng: uint8_t,
        name: *const uint8_t,
        chgmask: uint16_t,
        dleng: *mut uint8_t,
        desc: *mut uint8_t,
        priority: *mut uint32_t,
        export_group: *mut uint8_t,
        admin_only: *mut uint8_t,
        labels_mode: *mut uint8_t,
        arch_mode: *mut uint8_t,
        arch_delay: *mut uint16_t,
        arch_min_size: *mut uint64_t,
        min_trashretention: *mut uint16_t,
        create: *mut storagemode,
        keep: *mut storagemode,
        arch: *mut storagemode,
        trash: *mut storagemode,
    ) -> uint8_t;
    unsafe fn sclass_delete_entry(nleng: uint8_t, name: *const uint8_t) -> uint8_t;
    unsafe fn sclass_duplicate_entry(
        oldnleng: uint8_t,
        oldname: *const uint8_t,
        newnleng: uint8_t,
        newname: *const uint8_t,
    ) -> uint8_t;
    unsafe fn sclass_rename_entry(
        oldnleng: uint8_t,
        oldname: *const uint8_t,
        newnleng: uint8_t,
        newname: *const uint8_t,
    ) -> uint8_t;
    unsafe fn sclass_list_entries(buff: *mut uint8_t, sclsmode: uint8_t) -> uint32_t;
    unsafe fn sclass_find_by_name(nleng: uint8_t, name: *const uint8_t) -> uint8_t;
    unsafe fn sclass_get_nleng(sclassid: uint8_t) -> uint8_t;
    unsafe fn sclass_get_name(sclassid: uint8_t) -> *const uint8_t;
    unsafe fn chunk_store_info(buff: *mut uint8_t) -> uint32_t;
    unsafe fn chunk_store_chunkcounters(buff: *mut uint8_t, matrixid: uint8_t, classid: int16_t);
    unsafe fn chunk_info(
        allchunks: *mut uint32_t,
        copychunks: *mut uint32_t,
        ec8chunks: *mut uint32_t,
        ec4chunks: *mut uint32_t,
        copies: *mut uint64_t,
        ec8parts: *mut uint64_t,
        ec4parts: *mut uint64_t,
        hypotheticalcopies: *mut uint64_t,
    );
    unsafe fn chunk_counters_in_progress() -> uint8_t;
    unsafe fn chunk_locked_or_busy(cptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    unsafe fn chunk_get_version_and_csdata(
        mode: uint8_t,
        chunkid: uint64_t,
        clientip: uint32_t,
        version: *mut uint32_t,
        count: *mut uint8_t,
        cs_data: *mut uint8_t,
        split: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn chunk_get_version_and_copies(
        mode: uint8_t,
        chunkid: uint64_t,
        clientip: uint32_t,
        version: *mut uint32_t,
        chunkmtime: *mut uint32_t,
        count: *mut uint8_t,
        cs_data: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn chunk_get_version(chunkid: uint64_t, version: *mut uint32_t) -> uint8_t;
    unsafe fn chunk_get_memusage(allocated: *mut uint64_t, used: *mut uint64_t);
    unsafe fn matomlserv_mloglist_size() -> uint32_t;
    unsafe fn matomlserv_mloglist_data(ptr: *mut uint8_t);
    unsafe fn sessions_attach_session(
        vsesdata: *mut ::core::ffi::c_void,
        peerip: uint32_t,
        version: uint32_t,
    );
    unsafe fn sessions_close_session(vsesdata: *mut ::core::ffi::c_void);
    unsafe fn sessions_disconnection(vsesdata: *mut ::core::ffi::c_void);
    unsafe fn sessions_find_session(sessionid: uint32_t) -> *mut ::core::ffi::c_void;
    unsafe fn sessions_datasize(vmode: uint8_t) -> uint32_t;
    unsafe fn sessions_datafill(ptr: *mut uint8_t, vmode: uint8_t);
    unsafe fn sessions_force_remove(sessionid: uint32_t) -> uint8_t;
    unsafe fn sessions_new_session(
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
    ) -> *mut ::core::ffi::c_void;
    unsafe fn sessions_chg_session(
        vsesdata: *mut ::core::ffi::c_void,
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
    ) -> uint32_t;
    unsafe fn sessions_get_id(vsesdata: *mut ::core::ffi::c_void) -> uint32_t;
    unsafe fn sessions_get_exportscsum(vsesdata: *mut ::core::ffi::c_void) -> uint64_t;
    unsafe fn sessions_get_peerip(vsesdata: *mut ::core::ffi::c_void) -> uint32_t;
    unsafe fn sessions_get_rootinode(vsesdata: *mut ::core::ffi::c_void) -> uint32_t;
    unsafe fn sessions_get_sesflags(vsesdata: *mut ::core::ffi::c_void) -> uint32_t;
    unsafe fn sessions_get_umask(vsesdata: *mut ::core::ffi::c_void) -> uint16_t;
    unsafe fn sessions_get_disables(vsesdata: *mut ::core::ffi::c_void) -> uint32_t;
    unsafe fn sessions_is_root_remapped(vsesdata: *mut ::core::ffi::c_void) -> uint8_t;
    unsafe fn sessions_check_sclass(
        vsesdata: *mut ::core::ffi::c_void,
        smode: uint8_t,
        sclassid: uint8_t,
    ) -> uint8_t;
    unsafe fn sessions_check_trashretention(
        vsesdata: *mut ::core::ffi::c_void,
        smode: uint8_t,
        trashretention: uint32_t,
    ) -> uint8_t;
    unsafe fn sessions_inc_stats(vsesdata: *mut ::core::ffi::c_void, statid: uint8_t);
    unsafe fn sessions_add_stats(
        vsesdata: *mut ::core::ffi::c_void,
        statid: uint8_t,
        value: uint64_t,
    );
    unsafe fn sessions_ugid_remap(
        vsesdata: *mut ::core::ffi::c_void,
        auid: *mut uint32_t,
        agid: *mut uint32_t,
    );
    unsafe fn csdb_servlist_data(mode: uint8_t, ptr: *mut uint8_t, clientip: uint32_t) -> uint32_t;
    unsafe fn csdb_remove_server(ip: uint32_t, port: uint16_t) -> uint8_t;
    unsafe fn csdb_back_to_work(ip: uint32_t, port: uint16_t) -> uint8_t;
    unsafe fn csdb_maintenance(ip: uint32_t, port: uint16_t, onoff: uint8_t) -> uint8_t;
    unsafe fn fs_info(
        totalspace: *mut uint64_t,
        availspace: *mut uint64_t,
        freespace: *mut uint64_t,
        trspace: *mut uint64_t,
        trnodes: *mut uint32_t,
        respace: *mut uint64_t,
        renodes: *mut uint32_t,
        inodes: *mut uint32_t,
        dnodes: *mut uint32_t,
        fnodes: *mut uint32_t,
    );
    unsafe fn fs_test_getdata(
        loopstart: *mut uint32_t,
        loopend: *mut uint32_t,
        files: *mut uint32_t,
        ugfiles: *mut uint32_t,
        mfiles: *mut uint32_t,
        mtfiles: *mut uint32_t,
        msfiles: *mut uint32_t,
        chunks: *mut uint32_t,
        ugchunks: *mut uint32_t,
        mchunks: *mut uint32_t,
        msgbuff: *mut *mut ::core::ffi::c_char,
        msgbuffleng: *mut uint32_t,
    );
    unsafe fn fs_getrootinode(rootinode: *mut uint32_t, path: *const uint8_t) -> uint8_t;
    unsafe fn fs_path_lookup(
        rootinode: uint32_t,
        sesflags: uint8_t,
        base_inode: uint32_t,
        pleng: uint32_t,
        path: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        parent_inode: *mut uint32_t,
        last_inode: *mut uint32_t,
        nleng: *mut uint8_t,
        name: *mut uint8_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_statfs(
        rootinode: uint32_t,
        sesflags: uint8_t,
        totalspace: *mut uint64_t,
        availspace: *mut uint64_t,
        freespace: *mut uint64_t,
        trashspace: *mut uint64_t,
        sustainedspace: *mut uint64_t,
        inodes: *mut uint32_t,
    );
    unsafe fn fs_access(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        modemask: ::core::ffi::c_int,
    ) -> uint8_t;
    unsafe fn fs_lookup(
        rootinode: uint32_t,
        sesflags: uint8_t,
        parent: uint32_t,
        nleng: uint16_t,
        name: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
        allow_recover: uint8_t,
        accmode: *mut uint16_t,
        filenode: *mut uint8_t,
        validchunk: *mut uint8_t,
        chunkid: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn fs_getattr(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gid: uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_setattr(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
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
    unsafe fn fs_set_additional_attributes(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        flags: uint8_t,
        uid: uint32_t,
        data: *const uint8_t,
        leng: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_try_setlength(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        flags: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        disflags: uint8_t,
        length: uint64_t,
        indx: *mut uint32_t,
        prevchunkid: *mut uint64_t,
        chunkid: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn fs_end_setlength(chunkid: uint64_t) -> uint8_t;
    unsafe fn fs_do_setlength(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        flags: uint8_t,
        uid: uint32_t,
        gid: uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        length: uint64_t,
        attr: *mut uint8_t,
        prevlength: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn fs_readlink(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        pleng: *mut uint32_t,
        path: *mut *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_symlink(
        rootinode: uint32_t,
        sesflags: uint8_t,
        parent: uint32_t,
        nleng: uint16_t,
        name: *const uint8_t,
        pleng: uint32_t,
        path: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mknod(
        rootinode: uint32_t,
        sesflags: uint8_t,
        parent: uint32_t,
        nleng: uint16_t,
        name: *const uint8_t,
        r#type: uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        rdev: uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
        oflags: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mkdir(
        rootinode: uint32_t,
        sesflags: uint8_t,
        parent: uint32_t,
        nleng: uint16_t,
        name: *const uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        copysgid: uint8_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_unlink(
        rootinode: uint32_t,
        sesflags: uint8_t,
        parent: uint32_t,
        nleng: uint16_t,
        name: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_rmdir(
        rootinode: uint32_t,
        sesflags: uint8_t,
        parent: uint32_t,
        nleng: uint16_t,
        name: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_rename(
        rootinode: uint32_t,
        sesflags: uint8_t,
        parent_src: uint32_t,
        nleng_src: uint16_t,
        name_src: *const uint8_t,
        parent_dst: uint32_t,
        nleng_dst: uint16_t,
        name_dst: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        rmode: uint8_t,
        delflags: uint8_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_link(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode_src: uint32_t,
        parent_dst: uint32_t,
        nleng_dst: uint16_t,
        name_dst: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_snapshot(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode_src: uint32_t,
        parent_dst: uint32_t,
        nleng_dst: uint16_t,
        name_dst: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        smode: uint8_t,
        requmask: uint16_t,
    ) -> uint8_t;
    unsafe fn fs_append_slice(
        rootinode: uint32_t,
        sesflags: uint8_t,
        flags: uint8_t,
        inode: uint32_t,
        inode_src: uint32_t,
        slice_from: uint32_t,
        slice_to: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        fleng: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn fs_readdir_size(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        flags: uint8_t,
        maxentries: uint32_t,
        nedgeid: uint64_t,
        dnode: *mut *mut ::core::ffi::c_void,
        dedge: *mut *mut ::core::ffi::c_void,
        dbuffsize: *mut uint32_t,
        attrmode: uint8_t,
    ) -> uint8_t;
    unsafe fn fs_readdir_data(
        rootinode: uint32_t,
        sesflags: uint8_t,
        uid: uint32_t,
        gid: uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        flags: uint8_t,
        maxentries: uint32_t,
        nedgeid: *mut uint64_t,
        dnode: *mut ::core::ffi::c_void,
        dedge: *mut ::core::ffi::c_void,
        dbuff: *mut uint8_t,
        attrmode: uint8_t,
    );
    unsafe fn fs_filechunk(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        indx: uint32_t,
        chunkid: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn fs_checkfile(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        mode: uint8_t,
        chunkcount: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_opencheck(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        auid: uint32_t,
        agid: uint32_t,
        flags: uint8_t,
        attr: *mut uint8_t,
        oflags: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_readchunk(
        inode: uint32_t,
        sesflags: uint32_t,
        indx: uint32_t,
        chunkopflags: uint8_t,
        allow_recover: uint8_t,
        chunkid: *mut uint64_t,
        length: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn fs_writechunk(
        inode: uint32_t,
        indx: uint32_t,
        chunkopflags: uint8_t,
        prevchunkid: *mut uint64_t,
        chunkid: *mut uint64_t,
        length: *mut uint64_t,
        opflag: *mut uint8_t,
        clientip: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_writeend(
        inode: uint32_t,
        length: uint64_t,
        chunkid: uint64_t,
        chunkopflags: uint8_t,
        flenghaschanged: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_rollback(
        inode: uint32_t,
        indx: uint32_t,
        prevchunkid: uint64_t,
        chunkid: uint64_t,
    ) -> uint8_t;
    unsafe fn fs_repair(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        flags: uint8_t,
        notchanged: *mut uint32_t,
        erased: *mut uint32_t,
        repaired: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_amtime_update(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inodetab: *mut uint32_t,
        atimetab: *mut uint32_t,
        mtimetab: *mut uint32_t,
        cnt: uint32_t,
    );
    unsafe fn fs_getsclass(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        gmode: uint8_t,
        fgtab: *mut uint32_t,
        dgtab: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_setsclass(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        src_sclassid: uint8_t,
        dst_sclassid: uint8_t,
        smode: uint8_t,
        sinodes: *mut uint32_t,
        ncinodes: *mut uint32_t,
        nsinodes: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_gettrashretention_prepare(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        gmode: uint8_t,
        fptr: *mut *mut ::core::ffi::c_void,
        dptr: *mut *mut ::core::ffi::c_void,
        fnodes: *mut uint32_t,
        dnodes: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_gettrashretention_store(
        fptr: *mut ::core::ffi::c_void,
        dptr: *mut ::core::ffi::c_void,
        buff: *mut uint8_t,
    );
    unsafe fn fs_settrashretention(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        trashretention: uint32_t,
        smode: uint8_t,
        sinodes: *mut uint32_t,
        ncinodes: *mut uint32_t,
        nsinodes: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_geteattr(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        gmode: uint8_t,
        feattrtab: *mut uint32_t,
        deattrtab: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_seteattr(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        eattr: uint8_t,
        smode: uint8_t,
        sinodes: *mut uint32_t,
        ncinodes: *mut uint32_t,
        nsinodes: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_listxattr_leng(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        xanode: *mut *mut ::core::ffi::c_void,
        xasize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_listxattr_data(xanode: *mut ::core::ffi::c_void, xabuff: *mut uint8_t);
    unsafe fn fs_setxattr(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        anleng: uint8_t,
        attrname: *const uint8_t,
        avleng: uint32_t,
        attrvalue: *const uint8_t,
        mode: uint8_t,
    ) -> uint8_t;
    unsafe fn fs_getxattr(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        anleng: uint8_t,
        attrname: *const uint8_t,
        avleng: *mut uint32_t,
        attrvalue: *mut *const uint8_t,
    ) -> uint8_t;
    unsafe fn fs_setfacl(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        acltype: uint8_t,
        userperm: uint16_t,
        groupperm: uint16_t,
        otherperm: uint16_t,
        mask: uint16_t,
        namedusers: uint16_t,
        namedgroups: uint16_t,
        aclblob: *const uint8_t,
    ) -> uint8_t;
    unsafe fn fs_getfacl_size(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        acltype: uint8_t,
        custom: *mut *mut ::core::ffi::c_void,
        aclblobsize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_getfacl_data(
        custom: *mut ::core::ffi::c_void,
        userperm: *mut uint16_t,
        groupperm: *mut uint16_t,
        otherperm: *mut uint16_t,
        mask: *mut uint16_t,
        namedusers: *mut uint16_t,
        namedgroups: *mut uint16_t,
        aclblob: *mut uint8_t,
    );
    unsafe fn fs_get_parents_count(
        rootinode: uint32_t,
        inode: uint32_t,
        cnt: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_get_parents_data(rootinode: uint32_t, inode: uint32_t, buff: *mut uint8_t);
    unsafe fn fs_get_paths_size(
        rootinode: uint32_t,
        inode: uint32_t,
        psize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_get_paths_data(rootinode: uint32_t, inode: uint32_t, buff: *mut uint8_t);
    unsafe fn fs_archget(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        archchunks: *mut uint64_t,
        notarchchunks: *mut uint64_t,
        archinodes: *mut uint32_t,
        partinodes: *mut uint32_t,
        notarchinodes: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_archchg(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        uid: uint32_t,
        cmd: uint8_t,
        chgchunks: *mut uint64_t,
        notchgchunks: *mut uint64_t,
        nsinodes: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_node_info(
        rootinode: uint32_t,
        sesflags: uint8_t,
        eights_mode: uint8_t,
        inode: uint32_t,
        maxentries: uint32_t,
        continueid: uint64_t,
        ptr: *mut uint8_t,
    ) -> uint32_t;
    unsafe fn fs_readdirfull(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        flags: uint8_t,
        maxentries: uint32_t,
        nedgeidp: *mut uint64_t,
        dedge: *mut *mut ::core::ffi::c_void,
        dbuff: *mut uint8_t,
        dbuffsize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_trash_recover(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        parent_dst: uint32_t,
        pleng_dst: uint32_t,
        path_dst: *const uint8_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        copysgid: uint8_t,
        used_pleng: *mut uint32_t,
        used_path: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_trash_remove(sesflags: uint8_t, inode: uint32_t, uid: uint32_t) -> uint8_t;
    unsafe fn fs_readsustained_size(
        rootinode: uint32_t,
        sesflags: uint8_t,
        dbuffsize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_readsustained_data(rootinode: uint32_t, sesflags: uint8_t, dbuff: *mut uint8_t);
    unsafe fn fs_listsustained(
        partno: uint8_t,
        uid: uint32_t,
        gnleng: uint8_t,
        gname: *const uint8_t,
        dbuff: *mut uint8_t,
    ) -> uint32_t;
    unsafe fn fs_readtrash_size(
        rootinode: uint32_t,
        sesflags: uint8_t,
        tid: uint32_t,
        dbuffsize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_readtrash_data(
        rootinode: uint32_t,
        sesflags: uint8_t,
        tid: uint32_t,
        dbuff: *mut uint8_t,
    );
    unsafe fn fs_gettrashpath(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        pleng: *mut uint32_t,
        path: *mut *const uint8_t,
    ) -> uint8_t;
    unsafe fn fs_settrashpath(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        pleng: uint32_t,
        path: *const uint8_t,
    ) -> uint8_t;
    unsafe fn fs_purge(rootinode: uint32_t, sesflags: uint8_t, inode: uint32_t) -> uint8_t;
    unsafe fn fs_undel(rootinode: uint32_t, sesflags: uint8_t, inode: uint32_t) -> uint8_t;
    unsafe fn fs_listtrash(
        partno: uint8_t,
        format: uint8_t,
        uid: uint32_t,
        mints: uint32_t,
        maxts: uint32_t,
        gnleng: uint8_t,
        gname: *const uint8_t,
        dbuff: *mut uint8_t,
    ) -> uint32_t;
    unsafe fn fs_getdetachedattr(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        attr: *mut uint8_t,
        dtype: uint8_t,
    ) -> uint8_t;
    unsafe fn fs_get_dir_stats(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        inodes: *mut uint32_t,
        dirs: *mut uint32_t,
        files: *mut uint32_t,
        chunks: *mut uint32_t,
        length: *mut uint64_t,
        size: *mut uint64_t,
        rsize: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn fs_quotacontrol(
        rootinode: uint32_t,
        sesflags: uint8_t,
        inode: uint32_t,
        delflag: uint8_t,
        flags: *mut uint8_t,
        defaultgp: *mut uint8_t,
        graceperiod: *mut uint32_t,
        sinodes: *mut uint32_t,
        slength: *mut uint64_t,
        ssize: *mut uint64_t,
        srealsize: *mut uint64_t,
        hinodes: *mut uint32_t,
        hlength: *mut uint64_t,
        hsize: *mut uint64_t,
        hrealsize: *mut uint64_t,
        curinodes: *mut uint32_t,
        curlength: *mut uint64_t,
        cursize: *mut uint64_t,
        currealsize: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn fs_getquotainfo(buff: *mut uint8_t, ver: uint8_t) -> uint32_t;
    unsafe fn fs_get_memusage(allocated: *mut uint64_t, used: *mut uint64_t);
    unsafe fn of_openfile(sessionid: uint32_t, inode: uint32_t);
    unsafe fn of_sync(sessionid: uint32_t, inode: *mut uint32_t, inodecnt: uint32_t);
    unsafe fn of_isfileopened_by_session(inode: uint32_t, sessionid: uint32_t) -> uint8_t;
    unsafe fn of_lsof(sessionid: uint32_t, buff: *mut uint8_t) -> uint32_t;
    unsafe fn flock_locks_cmd(
        connptr: *mut ::core::ffi::c_void,
        sessionid: uint32_t,
        message_id: uint32_t,
        req_id: uint32_t,
        inode: uint32_t,
        lock_owner: uint64_t,
        op: uint8_t,
    ) -> uint8_t;
    unsafe fn flock_list(inode: uint32_t, buff: *mut uint8_t) -> uint32_t;
    unsafe fn flock_disconnected(connptr: *mut ::core::ffi::c_void);
    unsafe fn posix_lock_cmd(
        connptr: *mut ::core::ffi::c_void,
        sessionid: uint32_t,
        msgid: uint32_t,
        reqid: uint32_t,
        inode: uint32_t,
        owner: uint64_t,
        op: uint8_t,
        ltype: *mut uint8_t,
        start: *mut uint64_t,
        end: *mut uint64_t,
        pid: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn posix_lock_list(inode: uint32_t, buff: *mut uint8_t) -> uint32_t;
    unsafe fn posix_lock_disconnected(connptr: *mut ::core::ffi::c_void);
    unsafe fn meta_version() -> uint64_t;
    unsafe fn meta_get_id() -> uint64_t;
    unsafe fn meta_info(
        lsstore: *mut uint32_t,
        lstime: *mut uint32_t,
        lsstat: *mut uint8_t,
        lsmetavers: *mut uint64_t,
        lschecksum: *mut uint32_t,
    );
    unsafe fn rndu8() -> uint8_t;
    unsafe fn exports_info_size(versmode: uint8_t) -> uint32_t;
    unsafe fn exports_info_data(versmode: uint8_t, buff: *mut uint8_t);
    unsafe fn exports_check(
        ip: uint32_t,
        version: uint32_t,
        path: *const uint8_t,
        rndcode: *const uint8_t,
        passcode: *const uint8_t,
        sesflags: *mut uint8_t,
        umaskval: *mut uint16_t,
        rootuid: *mut uint32_t,
        rootgid: *mut uint32_t,
        mapalluid: *mut uint32_t,
        mapallgid: *mut uint32_t,
        sclassgroups: *mut uint16_t,
        mintrashretention: *mut uint32_t,
        maxtrashretention: *mut uint32_t,
        disables: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn exports_reload();
    unsafe fn exports_checksum() -> uint64_t;
    unsafe fn dcm_open(inode: uint32_t, sessionid: uint32_t) -> ::core::ffi::c_int;
    unsafe fn dcm_access(inode: uint32_t, sessionid: uint32_t);
    unsafe fn dcm_modify(inode: uint32_t, sessionid: uint32_t);
    unsafe fn charts_monotonic_data(buff: *mut uint8_t) -> uint32_t;
    unsafe fn charts_makedata(
        buff: *mut uint8_t,
        number: uint32_t,
        maxentries: uint32_t,
        multimode: uint8_t,
    ) -> uint32_t;
    unsafe fn charts_make_png(
        chartid: uint32_t,
        chartwidth: uint32_t,
        chartheight: uint32_t,
    ) -> uint32_t;
    unsafe fn charts_get_png(buff: *mut uint8_t);
    unsafe fn chartsdata_resusage(mem: *mut uint64_t, syscpu: *mut uint64_t, usrcpu: *mut uint64_t);
    unsafe fn patterns_add(
        gnleng: uint8_t,
        gname: *const uint8_t,
        euid: uint32_t,
        egid: uint32_t,
        priority: uint8_t,
        omask: uint8_t,
        scnleng: uint8_t,
        scname: *const uint8_t,
        trashretention: uint16_t,
        seteattr: uint8_t,
        clreattr: uint8_t,
    ) -> uint8_t;
    unsafe fn patterns_delete(
        gnleng: uint8_t,
        gname: *const uint8_t,
        euid: uint32_t,
        egid: uint32_t,
    ) -> uint8_t;
    unsafe fn patterns_list(buff: *mut uint8_t) -> uint32_t;
    unsafe fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cfg_getdefaultstr(name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getdefaultfile(
        name: *const ::core::ffi::c_char,
        maxleng: uint32_t,
    ) -> *mut cfg_buff;
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    unsafe fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_keepalive_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_poll_register_fname(
        desc: Option<unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()>,
        serve: Option<unsafe extern "C" fn(*mut pollfd) -> ()>,
        dname: *const ::core::ffi::c_char,
        sname: *const ::core::ffi::c_char,
    );
    unsafe fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn main_utime() -> uint64_t;
    unsafe fn univallocstrip(ip: uint32_t) -> *mut ::core::ffi::c_char;
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpresolve(
        hostname: *const ::core::ffi::c_char,
        service: *const ::core::ffi::c_char,
        ip: *mut uint32_t,
        port: *mut uint16_t,
        passiveflag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpsetacceptfilter(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpreuseaddr(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnumlisten(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
        queue: uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpaccept(lsock_0: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpgetpeer(
        sock: ::core::ffi::c_int,
        ip: *mut uint32_t,
        port: *mut uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
    unsafe fn missing_log_getdata(buff: *mut uint8_t, mode: uint8_t) -> uint32_t;
    unsafe fn iptosesid_add(ip: uint32_t, sessionid: uint32_t);
    unsafe fn iptosesid_check(ip: uint32_t) -> uint8_t;
    unsafe fn iptosesid_get(ip: uint32_t) -> uint32_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
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
pub type time_t = __time_t;
pub type int16_t = i16;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matoclserventry {
    pub registered: uint8_t,
    pub mode: uint8_t,
    pub sock: ::core::ffi::c_int,
    pub pdescpos: int32_t,
    pub lastread: ::core::ffi::c_double,
    pub lastwrite: ::core::ffi::c_double,
    pub input_hdr: [uint8_t; 8],
    pub input_startptr: *mut uint8_t,
    pub input_bytesleft: uint32_t,
    pub input_end: uint8_t,
    pub asize: uint8_t,
    pub input_packet: *mut in_packetstruct,
    pub inputhead: *mut in_packetstruct,
    pub inputtail: *mut *mut in_packetstruct,
    pub outputhead: *mut out_packetstruct,
    pub outputtail: *mut *mut out_packetstruct,
    pub version: uint32_t,
    pub peerip: uint32_t,
    pub strip: *mut ::core::ffi::c_char,
    pub timeout: uint16_t,
    pub working_flags: uint8_t,
    pub passwordrnd: [uint8_t; 32],
    pub path: *mut uint8_t,
    pub info: *mut uint8_t,
    pub ileng: uint32_t,
    pub usepassword: uint8_t,
    pub passwordmd5: [uint8_t; 16],
    pub sesdata: *mut ::core::ffi::c_void,
    pub next: *mut matoclserventry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct out_packetstruct {
    pub next: *mut out_packetstruct,
    pub startptr: *mut uint8_t,
    pub bytesleft: uint32_t,
    pub data: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_packetstruct {
    pub next: *mut in_packetstruct,
    pub r#type: uint32_t,
    pub leng: uint32_t,
    pub data: [uint8_t; 1],
}
pub const NOTREGISTERED: C2Rust_Unnamed_1 = 0;
pub const DATA: C2Rust_Unnamed = 1;
pub type lwchunks = _lwchunks;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lwchunks {
    pub chunkid: uint64_t,
    pub fleng: uint64_t,
    pub eptr: *mut matoclserventry,
    pub time: ::core::ffi::c_double,
    pub msgid: uint32_t,
    pub inode: uint32_t,
    pub indx: uint32_t,
    pub uid: uint32_t,
    pub gids: uint32_t,
    pub gid: *mut uint32_t,
    pub auid: uint32_t,
    pub agid: uint32_t,
    pub chunkopflags: uint8_t,
    pub flags: uint8_t,
    pub r#type: uint8_t,
    pub status: uint8_t,
    pub next: *mut _lwchunks,
}
pub const FUSE_READ: C2Rust_Unnamed_0 = 1;
pub const REGISTERED: C2Rust_Unnamed_1 = 1;
pub type swchunks = _swchunks;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _swchunks {
    pub chunkid: uint64_t,
    pub prevchunkid: uint64_t,
    pub eptr: *mut matoclserventry,
    pub fleng: uint64_t,
    pub msgid: uint32_t,
    pub inode: uint32_t,
    pub indx: uint32_t,
    pub uid: uint32_t,
    pub gid: uint32_t,
    pub auid: uint32_t,
    pub agid: uint32_t,
    pub flags: uint8_t,
    pub r#type: uint8_t,
    pub next: *mut _swchunks,
}
pub const FUSE_WRITE: C2Rust_Unnamed_0 = 0;
pub const FUSE_TRUNCATE: C2Rust_Unnamed_0 = 2;
pub const FUSE_CREATE: C2Rust_Unnamed_0 = 3;
pub const KILL: C2Rust_Unnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub const FINISH: C2Rust_Unnamed = 2;
pub type storagemode = _storagemode;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_buff {
    pub leng: uint32_t,
    pub data: [uint8_t; 1],
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const MFS_ROOT_ID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MASKORGROUP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAXSCLASS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const SCLASS_EXPR_MAX_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MAXLABELSCNT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MFS_PATH_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MFS_GIDS_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_ERROR_EACCES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_LOCKED: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MFS_ERROR_CHUNKBUSY: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const MFS_ERROR_IO: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const MFS_ERROR_DELAYED: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MFS_ERROR_EROFS: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const MFS_ERROR_BADSESSIONID: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const MFS_ERROR_NOPASSWORD: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const MFS_ERROR_BADPASSWORD: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const MFS_ERROR_WAITING: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const MFS_ERROR_EAGAIN: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT_NOCACHE: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const MFS_ERROR_EPERM_NOTADMIN: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSUCHCLASS: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const MFS_ERROR_INCOMPATVERSION: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_CTIME: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TYPE_FILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TYPE_FIFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TYPE_BLOCKDEV: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TYPE_CHARDEV: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TYPE_SOCKET: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LOOKUP_ACCESS_MODES_IO: ::core::ffi::c_int = 0xfc as ::core::ffi::c_int;
pub const LOOKUP_ACCESS_MODES_RO: ::core::ffi::c_int = 0xff33 as ::core::ffi::c_int;
pub const LOOKUP_ACCESS_BITS: ::core::ffi::c_int = 0x38ff as ::core::ffi::c_int;
pub const LOOKUP_CHUNK_ZERO_DATA: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const LOOKUP_RO_FILESYSTEM: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const LOOKUP_KEEPCACHE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const LOOKUP_DIRECTMODE: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const LOOKUP_APPENDONLY: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SET_WINATTR_FLAG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SET_MODE_FLAG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SET_UID_FLAG: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SET_GID_FLAG: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MFS_RENAME_STD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_GET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DTYPE_UNKNOWN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SMODE_EXCHANGE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SMODE_TMASK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LABELS_MODE_GLOBAL: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
pub const EATTR_BITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MATTR_ALLOWDATACACHE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MATTR_DIRECTMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const ARCHCTL_GET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_OPENED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_UPDATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_RESERVE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SESFLAG_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SESFLAG_DYNAMICIP: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SESFLAG_ADMIN: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SESFLAG_ATTRBIT: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_ALWAYS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SNAPSHOT_MODE_CPLIKE_ATTR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SNAPSHOT_MODE_DELETE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const OPEN_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPEN_TRUNCATE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPEN_CACHE_CLEARED: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPEN_KEEPCACHE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPEN_DIRECTMODE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPEN_APPENDONLY: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MFS_XATTR_GETA_DATA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_XATTR_LENGTH_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_CSSERV_COMMAND_REMOVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_CSSERV_COMMAND_BACKTOWORK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_CSSERV_COMMAND_MAINTENANCEON: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_CSSERV_COMMAND_MAINTENANCEOFF: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_SESSION_COMMAND_REMOVE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_XATTR_SIZE_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const FULL_DIRECTORY_ADD_CHUNKID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FULL_DIRECTORY_ADD_SYMLINK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SET_ALL_EATTR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SET_ALL_XATTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SET_ALL_FACL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CHUNKOPFLAG_CANMODTIME: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MODULE_TYPE_MASTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFLAG_INVALIDATE_CACHE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ATTR_RECORD_SIZE: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const DISABLE_BIT_CHOWN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DISABLE_BIT_CHMOD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DISABLE_BIT_SYMLINK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DISABLE_BIT_MKFIFO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DISABLE_BIT_MKDEV: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DISABLE_BIT_MKSOCK: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const DISABLE_BIT_MKDIR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const DISABLE_BIT_UNLINK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const DISABLE_BIT_RMDIR: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DISABLE_BIT_RENAME: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const DISABLE_BIT_MOVE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const DISABLE_BIT_LINK: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const DISABLE_BIT_CREATE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const DISABLE_BIT_READDIR: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const DISABLE_BIT_READ: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const DISABLE_BIT_WRITE: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const DISABLE_BIT_TRUNCATE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETLENGTH: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const DISABLE_BIT_APPENDCHUNKS: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const DISABLE_BIT_SNAPSHOT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETTRASH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETSCLASS: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETEATTR: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETXATTR: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETFACL: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const DISABLE_CHOWN: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_CHOWN;
pub const DISABLE_CHMOD: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_CHMOD;
pub const DISABLE_SYMLINK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_SYMLINK;
pub const DISABLE_MKFIFO: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MKFIFO;
pub const DISABLE_MKDEV: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MKDEV;
pub const DISABLE_MKSOCK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MKSOCK;
pub const DISABLE_MKDIR: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MKDIR;
pub const DISABLE_UNLINK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_UNLINK;
pub const DISABLE_RMDIR: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_RMDIR;
pub const DISABLE_RENAME: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_RENAME;
pub const DISABLE_MOVE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MOVE;
pub const DISABLE_LINK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_LINK;
pub const DISABLE_CREATE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_CREATE;
pub const DISABLE_READDIR: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_READDIR;
pub const DISABLE_READ: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_READ;
pub const DISABLE_WRITE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_WRITE;
pub const DISABLE_TRUNCATE: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_TRUNCATE;
pub const DISABLE_SETLENGTH: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETLENGTH;
pub const DISABLE_APPENDCHUNKS: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_APPENDCHUNKS;
pub const DISABLE_SNAPSHOT: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SNAPSHOT;
pub const DISABLE_SETTRASH: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETTRASH;
pub const DISABLE_SETSCLASS: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETSCLASS;
pub const DISABLE_SETEATTR: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETEATTR;
pub const DISABLE_SETXATTR: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETXATTR;
pub const DISABLE_SETFACL: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETFACL;
pub const CLTOMA_MAXPACKETSIZE: ::core::ffi::c_int = 50000000 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ANTOAN_UNKNOWN_COMMAND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTOAN_BAD_COMMAND_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ANTOAN_FORCE_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ANTOAN_GET_VERSION: uint32_t = 10 as uint32_t;
pub const ANTOAN_VERSION: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ANTOMA_SYSLOG: uint32_t = 71 as uint32_t;
pub const ANTOAN_GET_CONFIG: uint32_t = 80 as uint32_t;
pub const ANTOAN_CONFIG_VALUE: ::core::ffi::c_int = PROTO_BASE + 81 as ::core::ffi::c_int;
pub const ANTOAN_GET_CONFIG_FILE: uint32_t = 82 as uint32_t;
pub const ANTOAN_CONFIG_FILE_CONTENT: ::core::ffi::c_int = PROTO_BASE + 83 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_CREATE: uint32_t = 350 as uint32_t;
pub const MATOCL_SCLASS_CREATE: ::core::ffi::c_int = PROTO_BASE + 351 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_CHANGE: uint32_t = 352 as uint32_t;
pub const MATOCL_SCLASS_CHANGE: ::core::ffi::c_int = PROTO_BASE + 353 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_DELETE: uint32_t = 354 as uint32_t;
pub const MATOCL_SCLASS_DELETE: ::core::ffi::c_int = PROTO_BASE + 355 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_DUPLICATE: uint32_t = 356 as uint32_t;
pub const MATOCL_SCLASS_DUPLICATE: ::core::ffi::c_int = PROTO_BASE + 357 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_RENAME: uint32_t = 358 as uint32_t;
pub const MATOCL_SCLASS_RENAME: ::core::ffi::c_int = PROTO_BASE + 359 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_LIST: uint32_t = 360 as uint32_t;
pub const MATOCL_SCLASS_LIST: ::core::ffi::c_int = PROTO_BASE + 361 as ::core::ffi::c_int;
pub const CLTOMA_PATTERN_ADD: uint32_t = 370 as uint32_t;
pub const MATOCL_PATTERN_ADD: ::core::ffi::c_int = PROTO_BASE + 371 as ::core::ffi::c_int;
pub const CLTOMA_PATTERN_DELETE: uint32_t = 372 as uint32_t;
pub const MATOCL_PATTERN_DELETE: ::core::ffi::c_int = PROTO_BASE + 373 as ::core::ffi::c_int;
pub const CLTOMA_PATTERN_LIST: uint32_t = 374 as uint32_t;
pub const MATOCL_PATTERN_LIST: ::core::ffi::c_int = PROTO_BASE + 375 as ::core::ffi::c_int;
pub const CLTOMA_TRASH_LIST: uint32_t = 380 as uint32_t;
pub const MATOCL_TRASH_LIST: ::core::ffi::c_int = PROTO_BASE + 381 as ::core::ffi::c_int;
pub const CLTOMA_TRASH_RECOVER: uint32_t = 382 as uint32_t;
pub const MATOCL_TRASH_RECOVER: ::core::ffi::c_int = PROTO_BASE + 383 as ::core::ffi::c_int;
pub const CLTOMA_TRASH_REMOVE: uint32_t = 384 as uint32_t;
pub const MATOCL_TRASH_REMOVE: ::core::ffi::c_int = PROTO_BASE + 385 as ::core::ffi::c_int;
pub const CLTOMA_SUSTAINED_LIST: uint32_t = 388 as uint32_t;
pub const MATOCL_SUSTAINED_LIST: ::core::ffi::c_int = PROTO_BASE + 389 as ::core::ffi::c_int;
pub const CLTOMA_PATH_LOOKUP: uint32_t = 390 as uint32_t;
pub const MATOCL_PATH_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 391 as ::core::ffi::c_int;
pub const FUSE_REGISTER_BLOB_ACL: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
        *b"DjI1GAQDULI5d2YjA26ypc3ovkhjvhciTQVx3CS4nYgtBoUcsljiVpsErJENHaw0\0",
    )
};
pub const REGISTER_GETRANDOM: ::core::ffi::c_int = 1;
pub const REGISTER_NEWSESSION: ::core::ffi::c_int = 2;
pub const REGISTER_RECONNECT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const REGISTER_NEWMETASESSION: ::core::ffi::c_int = 5;
pub const REGISTER_CLOSESESSION: ::core::ffi::c_int = 6;
pub const CLTOMA_FUSE_REGISTER: uint32_t = 400 as uint32_t;
pub const MATOCL_FUSE_REGISTER: ::core::ffi::c_int = PROTO_BASE + 401 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_STATFS: uint32_t = 402 as uint32_t;
pub const MATOCL_FUSE_STATFS: ::core::ffi::c_int = PROTO_BASE + 403 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_ACCESS: uint32_t = 404 as uint32_t;
pub const MATOCL_FUSE_ACCESS: ::core::ffi::c_int = PROTO_BASE + 405 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_LOOKUP: uint32_t = 406 as uint32_t;
pub const MATOCL_FUSE_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 407 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETATTR: uint32_t = 408 as uint32_t;
pub const MATOCL_FUSE_GETATTR: ::core::ffi::c_int = PROTO_BASE + 409 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETATTR: uint32_t = 410 as uint32_t;
pub const MATOCL_FUSE_SETATTR: ::core::ffi::c_int = PROTO_BASE + 411 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READLINK: uint32_t = 412 as uint32_t;
pub const MATOCL_FUSE_READLINK: ::core::ffi::c_int = PROTO_BASE + 413 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SYMLINK: uint32_t = 414 as uint32_t;
pub const MATOCL_FUSE_SYMLINK: ::core::ffi::c_int = PROTO_BASE + 415 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_MKNOD: uint32_t = 416 as uint32_t;
pub const MATOCL_FUSE_MKNOD: ::core::ffi::c_int = PROTO_BASE + 417 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_MKDIR: uint32_t = 418 as uint32_t;
pub const MATOCL_FUSE_MKDIR: ::core::ffi::c_int = PROTO_BASE + 419 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_UNLINK: uint32_t = 420 as uint32_t;
pub const MATOCL_FUSE_UNLINK: ::core::ffi::c_int = PROTO_BASE + 421 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_RMDIR: uint32_t = 422 as uint32_t;
pub const MATOCL_FUSE_RMDIR: ::core::ffi::c_int = PROTO_BASE + 423 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_RENAME: uint32_t = 424 as uint32_t;
pub const MATOCL_FUSE_RENAME: ::core::ffi::c_int = PROTO_BASE + 425 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_LINK: uint32_t = 426 as uint32_t;
pub const MATOCL_FUSE_LINK: ::core::ffi::c_int = PROTO_BASE + 427 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READDIR: uint32_t = 428 as uint32_t;
pub const MATOCL_FUSE_READDIR: ::core::ffi::c_int = PROTO_BASE + 429 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_OPEN: uint32_t = 430 as uint32_t;
pub const MATOCL_FUSE_OPEN: ::core::ffi::c_int = PROTO_BASE + 431 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READ_CHUNK: uint32_t = 432 as uint32_t;
pub const MATOCL_FUSE_READ_CHUNK: ::core::ffi::c_int = PROTO_BASE + 433 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_WRITE_CHUNK: uint32_t = 434 as uint32_t;
pub const MATOCL_FUSE_WRITE_CHUNK: ::core::ffi::c_int = PROTO_BASE + 435 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_WRITE_CHUNK_END: uint32_t = 436 as uint32_t;
pub const MATOCL_FUSE_WRITE_CHUNK_END: ::core::ffi::c_int = PROTO_BASE + 437 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_APPEND_SLICE: uint32_t = 438 as uint32_t;
pub const MATOCL_FUSE_APPEND_SLICE: ::core::ffi::c_int = PROTO_BASE + 439 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_CHECK: uint32_t = 440 as uint32_t;
pub const MATOCL_FUSE_CHECK: ::core::ffi::c_int = PROTO_BASE + 441 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETTRASHRETENTION: uint32_t = 442 as uint32_t;
pub const MATOCL_FUSE_GETTRASHRETENTION: ::core::ffi::c_int =
    PROTO_BASE + 443 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETTRASHRETENTION: uint32_t = 444 as uint32_t;
pub const MATOCL_FUSE_SETTRASHRETENTION: ::core::ffi::c_int =
    PROTO_BASE + 445 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETSCLASS: uint32_t = 446 as uint32_t;
pub const MATOCL_FUSE_GETSCLASS: ::core::ffi::c_int = PROTO_BASE + 447 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETSCLASS: uint32_t = 448 as uint32_t;
pub const MATOCL_FUSE_SETSCLASS: ::core::ffi::c_int = PROTO_BASE + 449 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETTRASH: uint32_t = 450 as uint32_t;
pub const MATOCL_FUSE_GETTRASH: ::core::ffi::c_int = PROTO_BASE + 451 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETDETACHEDATTR: uint32_t = 452 as uint32_t;
pub const MATOCL_FUSE_GETDETACHEDATTR: ::core::ffi::c_int = PROTO_BASE + 453 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETTRASHPATH: uint32_t = 454 as uint32_t;
pub const MATOCL_FUSE_GETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 455 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETTRASHPATH: uint32_t = 456 as uint32_t;
pub const MATOCL_FUSE_SETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 457 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_UNDEL: uint32_t = 458 as uint32_t;
pub const MATOCL_FUSE_UNDEL: ::core::ffi::c_int = PROTO_BASE + 459 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_PURGE: uint32_t = 460 as uint32_t;
pub const MATOCL_FUSE_PURGE: ::core::ffi::c_int = PROTO_BASE + 461 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETDIRSTATS: uint32_t = 462 as uint32_t;
pub const MATOCL_FUSE_GETDIRSTATS: ::core::ffi::c_int = PROTO_BASE + 463 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_TRUNCATE: uint32_t = 464 as uint32_t;
pub const MATOCL_FUSE_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 465 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_REPAIR: uint32_t = 466 as uint32_t;
pub const MATOCL_FUSE_REPAIR: ::core::ffi::c_int = PROTO_BASE + 467 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SNAPSHOT: uint32_t = 468 as uint32_t;
pub const MATOCL_FUSE_SNAPSHOT: ::core::ffi::c_int = PROTO_BASE + 469 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETSUSTAINED: uint32_t = 470 as uint32_t;
pub const MATOCL_FUSE_GETSUSTAINED: ::core::ffi::c_int = PROTO_BASE + 471 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETEATTR: uint32_t = 472 as uint32_t;
pub const MATOCL_FUSE_GETEATTR: ::core::ffi::c_int = PROTO_BASE + 473 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETEATTR: uint32_t = 474 as uint32_t;
pub const MATOCL_FUSE_SETEATTR: ::core::ffi::c_int = PROTO_BASE + 475 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_QUOTACONTROL: uint32_t = 476 as uint32_t;
pub const MATOCL_FUSE_QUOTACONTROL: ::core::ffi::c_int = PROTO_BASE + 477 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETXATTR: uint32_t = 478 as uint32_t;
pub const MATOCL_FUSE_GETXATTR: ::core::ffi::c_int = PROTO_BASE + 479 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETXATTR: uint32_t = 480 as uint32_t;
pub const MATOCL_FUSE_SETXATTR: ::core::ffi::c_int = PROTO_BASE + 481 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_CREATE: uint32_t = 482 as uint32_t;
pub const MATOCL_FUSE_CREATE: ::core::ffi::c_int = PROTO_BASE + 483 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_PARENTS: uint32_t = 484 as uint32_t;
pub const MATOCL_FUSE_PARENTS: ::core::ffi::c_int = PROTO_BASE + 485 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_PATHS: uint32_t = 486 as uint32_t;
pub const MATOCL_FUSE_PATHS: ::core::ffi::c_int = PROTO_BASE + 487 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETFACL: uint32_t = 488 as uint32_t;
pub const MATOCL_FUSE_GETFACL: ::core::ffi::c_int = PROTO_BASE + 489 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETFACL: uint32_t = 490 as uint32_t;
pub const MATOCL_FUSE_SETFACL: ::core::ffi::c_int = PROTO_BASE + 491 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_FLOCK: uint32_t = 492 as uint32_t;
pub const MATOCL_FUSE_FLOCK: ::core::ffi::c_int = PROTO_BASE + 493 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_POSIX_LOCK: uint32_t = 494 as uint32_t;
pub const MATOCL_FUSE_POSIX_LOCK: ::core::ffi::c_int = PROTO_BASE + 495 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_ARCHCTL: uint32_t = 496 as uint32_t;
pub const MATOCL_FUSE_ARCHCTL: ::core::ffi::c_int = PROTO_BASE + 497 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SUSTAINED_INODES_DEPRECATED: uint32_t = 499 as uint32_t;
pub const CLTOMA_CSERV_LIST: uint32_t = 500 as uint32_t;
pub const MATOCL_CSERV_LIST: ::core::ffi::c_int = PROTO_BASE + 501 as ::core::ffi::c_int;
pub const CLTOAN_MONOTONIC_DATA: uint32_t = 502 as uint32_t;
pub const ANTOCL_MONOTONIC_DATA: ::core::ffi::c_int = PROTO_BASE + 503 as ::core::ffi::c_int;
pub const CLTOAN_CHART: uint32_t = 504 as uint32_t;
pub const ANTOCL_CHART: ::core::ffi::c_int = PROTO_BASE + 505 as ::core::ffi::c_int;
pub const CLTOAN_CHART_DATA: uint32_t = 506 as uint32_t;
pub const ANTOCL_CHART_DATA: ::core::ffi::c_int = PROTO_BASE + 507 as ::core::ffi::c_int;
pub const CLTOMA_SESSION_LIST: uint32_t = 508 as uint32_t;
pub const MATOCL_SESSION_LIST: ::core::ffi::c_int = PROTO_BASE + 509 as ::core::ffi::c_int;
pub const CLTOMA_INFO: uint32_t = 510 as uint32_t;
pub const MATOCL_INFO: ::core::ffi::c_int = PROTO_BASE + 511 as ::core::ffi::c_int;
pub const CLTOMA_FSTEST_INFO: uint32_t = 512 as uint32_t;
pub const MATOCL_FSTEST_INFO: ::core::ffi::c_int = PROTO_BASE + 513 as ::core::ffi::c_int;
pub const CLTOMA_CHUNKSTEST_INFO: uint32_t = 514 as uint32_t;
pub const MATOCL_CHUNKSTEST_INFO: ::core::ffi::c_int = PROTO_BASE + 515 as ::core::ffi::c_int;
pub const CLTOMA_CHUNKS_MATRIX: uint32_t = 516 as uint32_t;
pub const MATOCL_CHUNKS_MATRIX: ::core::ffi::c_int = PROTO_BASE + 517 as ::core::ffi::c_int;
pub const CLTOMA_QUOTA_INFO: uint32_t = 518 as uint32_t;
pub const MATOCL_QUOTA_INFO: ::core::ffi::c_int = PROTO_BASE + 519 as ::core::ffi::c_int;
pub const CLTOMA_EXPORTS_INFO: uint32_t = 520 as uint32_t;
pub const MATOCL_EXPORTS_INFO: ::core::ffi::c_int = PROTO_BASE + 521 as ::core::ffi::c_int;
pub const CLTOMA_MLOG_LIST: uint32_t = 522 as uint32_t;
pub const MATOCL_MLOG_LIST: ::core::ffi::c_int = PROTO_BASE + 523 as ::core::ffi::c_int;
pub const CLTOMA_CSSERV_COMMAND: uint32_t = 524 as uint32_t;
pub const MATOCL_CSSERV_COMMAND: ::core::ffi::c_int = PROTO_BASE + 525 as ::core::ffi::c_int;
pub const CLTOMA_SESSION_COMMAND: uint32_t = 526 as uint32_t;
pub const MATOCL_SESSION_COMMAND: ::core::ffi::c_int = PROTO_BASE + 527 as ::core::ffi::c_int;
pub const CLTOMA_MEMORY_INFO: uint32_t = 528 as uint32_t;
pub const MATOCL_MEMORY_INFO: ::core::ffi::c_int = PROTO_BASE + 529 as ::core::ffi::c_int;
pub const CLTOAN_MODULE_INFO: uint32_t = 530 as uint32_t;
pub const ANTOCL_MODULE_INFO: ::core::ffi::c_int = PROTO_BASE + 531 as ::core::ffi::c_int;
pub const CLTOMA_LIST_OPEN_FILES: uint32_t = 532 as uint32_t;
pub const MATOCL_LIST_OPEN_FILES: ::core::ffi::c_int = PROTO_BASE + 533 as ::core::ffi::c_int;
pub const CLTOMA_LIST_ACQUIRED_LOCKS: uint32_t = 534 as uint32_t;
pub const MATOCL_LIST_ACQUIRED_LOCKS: ::core::ffi::c_int = PROTO_BASE + 535 as ::core::ffi::c_int;
pub const CLTOMA_MASS_RESOLVE_PATHS: uint32_t = 536 as uint32_t;
pub const MATOCL_MASS_RESOLVE_PATHS: ::core::ffi::c_int = PROTO_BASE + 537 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_INFO: uint32_t = 542 as uint32_t;
pub const MATOCL_SCLASS_INFO: ::core::ffi::c_int = PROTO_BASE + 543 as ::core::ffi::c_int;
pub const CLTOMA_MISSING_CHUNKS: uint32_t = 544 as uint32_t;
pub const MATOCL_MISSING_CHUNKS: ::core::ffi::c_int = PROTO_BASE + 545 as ::core::ffi::c_int;
pub const CLTOMA_NODE_INFO: uint32_t = 546 as uint32_t;
pub const MATOCL_NODE_INFO: ::core::ffi::c_int = PROTO_BASE + 547 as ::core::ffi::c_int;
pub const CLTOMA_PATTERN_INFO: uint32_t = 548 as uint32_t;
pub const MATOCL_PATTERN_INFO: ::core::ffi::c_int = PROTO_BASE + 549 as ::core::ffi::c_int;
pub const CLTOMA_INSTANCE_NAME: uint32_t = 550 as uint32_t;
pub const MATOCL_INSTANCE_NAME: ::core::ffi::c_int = PROTO_BASE + 551 as ::core::ffi::c_int;
pub const CLTOMA_FULL_DIRECTORY_DATA: uint32_t = 552 as uint32_t;
pub const MATOCL_FULL_DIRECTORY_DATA: ::core::ffi::c_int = PROTO_BASE + 553 as ::core::ffi::c_int;
pub const CLTOMA_SET_ALL_NODE_ATTRIBUTES: uint32_t = 554 as uint32_t;
pub const MATOCL_SET_ALL_NODE_ATTRIBUTES: ::core::ffi::c_int =
    PROTO_BASE + 555 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SUSTAINED_INODES: uint32_t = 700 as uint32_t;
pub const CLTOMA_FUSE_AMTIME_INODES: uint32_t = 701 as uint32_t;
pub const MATOCL_FUSE_CHUNK_HAS_CHANGED: ::core::ffi::c_int =
    PROTO_BASE + 702 as ::core::ffi::c_int;
pub const MATOCL_FUSE_FLENG_HAS_CHANGED: ::core::ffi::c_int =
    PROTO_BASE + 703 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_TIME_SYNC: uint32_t = 704 as uint32_t;
pub const MATOCL_FUSE_TIME_SYNC: ::core::ffi::c_int = PROTO_BASE + 705 as ::core::ffi::c_int;
pub const MATOCL_FUSE_INVALIDATE_CHUNK_CACHE: ::core::ffi::c_int =
    PROTO_BASE + 706 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_OPDATA: uint32_t = 710 as uint32_t;
pub const CLTOMA_FUSE_WFLAGS: uint32_t = 711 as uint32_t;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const DEFAULT_MASTER_CLIENT_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9421\0") };
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VERSMAJ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VERSMID: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const VERSMIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
pub const VERSSTR: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"4.59.2-1\0") };
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
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
pub const SES_OP_STATFS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SES_OP_GETATTR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SES_OP_SETATTR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SES_OP_LOOKUP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SES_OP_MKDIR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SES_OP_RMDIR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SES_OP_SYMLINK: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SES_OP_READLINK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SES_OP_MKNOD: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SES_OP_UNLINK: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SES_OP_RENAME: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SES_OP_LINK: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SES_OP_READDIR: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SES_OP_OPEN: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SES_OP_READCHUNK: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SES_OP_WRITECHUNK: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SES_OP_READ: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SES_OP_WRITE: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SES_OP_FSYNC: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SES_OP_SNAPSHOT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SES_OP_TRUNCATE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SES_OP_GETXATTR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SES_OP_SETXATTR: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SES_OP_GETFACL: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SES_OP_SETFACL: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const SES_OP_CREATE: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SES_OP_LOCK: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const SES_OP_META: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mfsrealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pptr: *mut ::core::ffi::c_void = realloc(ptr, size);
        if pptr.is_null() {
            free(ptr);
        }
        return pptr;
    }
}
pub const MaxPacketSize: ::core::ffi::c_int = CLTOMA_MAXPACKETSIZE;
static mut matoclservhead: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
static mut lsock: ::core::ffi::c_int = 0;
static mut lsockpdescpos: int32_t = 0;
static mut master_processid: uint64_t = 0;
pub const CHUNKHASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const CHUNK_WAIT_TIMEOUT: ::core::ffi::c_double = 30.0f64;
static mut lwchunkshashhead: [*mut lwchunks; 256] = [::core::ptr::null_mut::<lwchunks>(); 256];
static mut lwchunkshashtail: [*mut *mut lwchunks; 256] =
    [::core::ptr::null_mut::<*mut lwchunks>(); 256];
static mut swchunkshash: [*mut swchunks; 256] = [::core::ptr::null_mut::<swchunks>(); 256];
static mut ListenHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut ListenPort: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut listenip: uint32_t = 0;
static mut listenport: uint16_t = 0;
static mut CreateFirstChunk: uint8_t = 0;
static mut DefaultTimeout: uint32_t = 0;
static mut ForceTimeout: uint32_t = 0;
static mut RestrictIncompatibleClientVersions: uint8_t = 0;
static mut InstanceName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut InstanceNameLeng: uint32_t = 0;
static mut stats_prcvd: uint32_t = 0 as uint32_t;
static mut stats_psent: uint32_t = 0 as uint32_t;
static mut stats_brcvd: uint64_t = 0 as uint64_t;
static mut stats_bsent: uint64_t = 0 as uint64_t;
static mut stats_mounts_bread: uint64_t = 0 as uint64_t;
static mut stats_mounts_bwrite: uint64_t = 0 as uint64_t;
static mut stats_mounts_rcnt: uint32_t = 0 as uint32_t;
static mut stats_mounts_wcnt: uint32_t = 0 as uint32_t;
static mut stats_mounts_fcnt: uint32_t = 0 as uint32_t;
static mut stats_mounts_brcvd: uint64_t = 0 as uint64_t;
static mut stats_mounts_bsent: uint64_t = 0 as uint64_t;
static mut stats_lcnt: uint32_t = 0 as uint32_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_stats(mut stats: *mut uint64_t) {
    unsafe {
        *stats.offset(0 as isize) = stats_prcvd as uint64_t;
        *stats.offset(1 as isize) = stats_psent as uint64_t;
        *stats.offset(2 as isize) = stats_brcvd;
        *stats.offset(3 as isize) = stats_bsent;
        *stats.offset(4 as isize) = stats_mounts_bread;
        *stats.offset(5 as isize) = stats_mounts_bwrite;
        *stats.offset(6 as isize) = stats_mounts_rcnt as uint64_t;
        *stats.offset(7 as isize) = stats_mounts_wcnt as uint64_t;
        *stats.offset(8 as isize) = stats_mounts_fcnt as uint64_t;
        *stats.offset(9 as isize) = stats_mounts_brcvd;
        *stats.offset(10 as isize) = stats_mounts_bsent;
        *stats.offset(11 as isize) = stats_lcnt as uint64_t;
        stats_prcvd = 0 as uint32_t;
        stats_psent = 0 as uint32_t;
        stats_brcvd = 0 as uint64_t;
        stats_bsent = 0 as uint64_t;
        stats_mounts_bread = 0 as uint64_t;
        stats_mounts_bwrite = 0 as uint64_t;
        stats_mounts_rcnt = 0 as uint32_t;
        stats_mounts_wcnt = 0 as uint32_t;
        stats_mounts_fcnt = 0 as uint32_t;
        stats_mounts_brcvd = 0 as uint64_t;
        stats_mounts_bsent = 0 as uint64_t;
        stats_lcnt = 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_create_packet(
    mut eptr: *mut matoclserventry,
    mut r#type: uint32_t,
    mut size: uint32_t,
) -> *mut uint8_t {
    unsafe {
        let mut outpacket: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut psize: uint32_t = 0;
        psize = size.wrapping_add(8 as uint32_t);
        outpacket = malloc((20 as size_t).wrapping_add(psize as size_t)) as *mut out_packetstruct;
        if outpacket.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if outpacket
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut out_packetstruct
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*outpacket).bytesleft = psize;
        ptr = &raw mut (*outpacket).data as *mut uint8_t;
        put32bit(&raw mut ptr, r#type);
        put32bit(&raw mut ptr, size);
        (*outpacket).startptr = &raw mut (*outpacket).data as *mut uint8_t;
        (*outpacket).next = ::core::ptr::null_mut::<out_packetstruct>();
        *(*eptr).outputtail = outpacket;
        (*eptr).outputtail = &raw mut (*outpacket).next as *mut *mut out_packetstruct;
        return ptr;
    }
}
#[inline]
unsafe extern "C" fn matoclserv_fuse_write_chunk_common(
    mut eptr: *mut matoclserventry,
    mut msgid: uint32_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut chunkopflags: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut fleng: uint64_t = 0;
        let mut prevchunkid: uint64_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut opflag: uint8_t = 0;
        let mut swc: *mut swchunks = ::core::ptr::null_mut::<swchunks>();
        let mut lwc: *mut lwchunks = ::core::ptr::null_mut::<lwchunks>();
        let mut i: uint32_t = 0;
        let mut version: uint32_t = 0;
        let mut split: uint8_t = 0;
        let mut count: uint8_t = 0;
        let mut cs_data: [uint8_t; 1400] = [0; 1400];
        if sessions_get_disables((*eptr).sesdata) & DISABLE_WRITE as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_READONLY as uint32_t != 0 {
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        101 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        101 as ::core::ffi::c_int
                    })) as uint32_t
            {
                status = MFS_ERROR_EROFS as uint8_t;
            } else {
                status = MFS_ERROR_IO as uint8_t;
            }
        } else {
            status = fs_writechunk(
                inode,
                indx,
                chunkopflags,
                &raw mut prevchunkid,
                &raw mut chunkid,
                &raw mut fleng,
                &raw mut opflag,
                (*eptr).peerip,
            );
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            if status as ::core::ffi::c_int == MFS_ERROR_LOCKED
                || status as ::core::ffi::c_int == MFS_ERROR_CHUNKBUSY
            {
                i = (prevchunkid & 0xff as uint64_t) as uint32_t;
                lwc = malloc(::core::mem::size_of::<lwchunks>()) as *mut lwchunks;
                if lwc.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        299 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        299 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if lwc
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut lwchunks
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        299 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        299 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*lwc).chunkid = prevchunkid;
                (*lwc).eptr = eptr;
                (*lwc).time = monotonic_seconds();
                (*lwc).msgid = msgid;
                (*lwc).inode = inode;
                (*lwc).indx = indx;
                (*lwc).chunkopflags = chunkopflags;
                (*lwc).r#type = FUSE_WRITE as ::core::ffi::c_int as uint8_t;
                (*lwc).status = status;
                (*lwc).next = ::core::ptr::null_mut::<_lwchunks>();
                *lwchunkshashtail[i as usize] = lwc;
                lwchunkshashtail[i as usize] = &raw mut (*lwc).next as *mut *mut lwchunks;
                return 1 as ::core::ffi::c_int;
            }
            if status as ::core::ffi::c_int == MFS_ERROR_EAGAIN
                && (*eptr).version
                    < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            8 as ::core::ffi::c_int
                        })) as uint32_t
            {
                status = MFS_ERROR_LOCKED as uint8_t;
            }
            ptr =
                matoclserv_create_packet(eptr, MATOCL_FUSE_WRITE_CHUNK as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
            return 0 as ::core::ffi::c_int;
        } else {
            sessions_inc_stats((*eptr).sesdata, SES_OP_WRITECHUNK as uint8_t);
        }
        if opflag != 0 {
            i = (chunkid & 0xff as uint64_t) as uint32_t;
            swc = malloc(::core::mem::size_of::<swchunks>()) as *mut swchunks;
            if swc.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    327 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    327 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if swc
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut swchunks
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    327 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    327 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            (*swc).eptr = eptr;
            (*swc).inode = inode;
            (*swc).indx = indx;
            (*swc).prevchunkid = prevchunkid;
            (*swc).chunkid = chunkid;
            (*swc).msgid = msgid;
            (*swc).fleng = fleng;
            (*swc).r#type = FUSE_WRITE as ::core::ffi::c_int as uint8_t;
            (*swc).next = swchunkshash[i as usize] as *mut _swchunks;
            swchunkshash[i as usize] = swc;
        } else {
            dcm_modify(inode, sessions_get_id((*eptr).sesdata));
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        10 as ::core::ffi::c_int
                    })) as uint32_t
            {
                status = chunk_get_version_and_csdata(
                    2 as uint8_t,
                    chunkid,
                    (*eptr).peerip,
                    &raw mut version,
                    &raw mut count,
                    &raw mut cs_data as *mut uint8_t,
                    &raw mut split,
                );
            } else if (*eptr).version
                >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        32 as ::core::ffi::c_int
                    })) as uint32_t
            {
                status = chunk_get_version_and_csdata(
                    1 as uint8_t,
                    chunkid,
                    (*eptr).peerip,
                    &raw mut version,
                    &raw mut count,
                    &raw mut cs_data as *mut uint8_t,
                    &raw mut split,
                );
            } else {
                status = chunk_get_version_and_csdata(
                    0 as uint8_t,
                    chunkid,
                    (*eptr).peerip,
                    &raw mut version,
                    &raw mut count,
                    &raw mut cs_data as *mut uint8_t,
                    &raw mut split,
                );
            }
            if status as ::core::ffi::c_int == MFS_STATUS_OK && split as ::core::ffi::c_int != 0 {
                status = MFS_ERROR_EAGAIN as uint8_t;
            }
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                    5 as uint32_t,
                );
                put32bit(&raw mut ptr, msgid);
                put8bit(&raw mut ptr, status);
                fs_writeend(
                    0 as uint32_t,
                    0 as uint64_t,
                    chunkid,
                    0 as uint8_t,
                    ::core::ptr::null_mut::<uint8_t>(),
                );
                return 0 as ::core::ffi::c_int;
            }
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        10 as ::core::ffi::c_int
                    })) as uint32_t
            {
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                    (25 as ::core::ffi::c_int
                        + count as ::core::ffi::c_int * 14 as ::core::ffi::c_int)
                        as uint32_t,
                );
            } else if (*eptr).version
                >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        32 as ::core::ffi::c_int
                    })) as uint32_t
            {
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                    (25 as ::core::ffi::c_int
                        + count as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                        as uint32_t,
                );
            } else {
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                    (24 as ::core::ffi::c_int
                        + count as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                        as uint32_t,
                );
            }
            put32bit(&raw mut ptr, msgid);
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        10 as ::core::ffi::c_int
                    })) as uint32_t
            {
                put8bit(&raw mut ptr, 2 as uint8_t);
            } else if (*eptr).version
                >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        32 as ::core::ffi::c_int
                    })) as uint32_t
            {
                put8bit(&raw mut ptr, 1 as uint8_t);
            }
            put64bit(&raw mut ptr, fleng);
            put64bit(&raw mut ptr, chunkid);
            put32bit(&raw mut ptr, version);
            if count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if (*eptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                        (count as ::core::ffi::c_int * 14 as ::core::ffi::c_int) as size_t,
                    );
                } else if (*eptr).version
                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            32 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                        (count as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as size_t,
                    );
                } else {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                        (count as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as size_t,
                    );
                }
            }
            matoclserv_fuse_chunk_has_changed(
                eptr,
                inode,
                indx,
                chunkid,
                version,
                fleng,
                0 as uint8_t,
                0 as uint32_t,
                0 as uint32_t,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn matoclserv_fuse_read_chunk_common(
    mut eptr: *mut matoclserventry,
    mut msgid: uint32_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut chunkopflags: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut fleng: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut lwc: *mut lwchunks = ::core::ptr::null_mut::<lwchunks>();
        let mut i: uint32_t = 0;
        let mut split: uint8_t = 0;
        let mut count: uint8_t = 0;
        let mut cs_data: [uint8_t; 1400] = [0; 1400];
        if sessions_get_disables((*eptr).sesdata) & DISABLE_READ as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_readchunk(
                inode,
                sessions_get_sesflags((*eptr).sesdata),
                indx,
                chunkopflags,
                1 as uint8_t,
                &raw mut chunkid,
                &raw mut fleng,
            );
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            if status as ::core::ffi::c_int == MFS_ERROR_LOCKED
                || status as ::core::ffi::c_int == MFS_ERROR_CHUNKBUSY
            {
                i = (chunkid & 0xff as uint64_t) as uint32_t;
                lwc = malloc(::core::mem::size_of::<lwchunks>()) as *mut lwchunks;
                if lwc.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if lwc
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut lwchunks
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*lwc).chunkid = chunkid;
                (*lwc).eptr = eptr;
                (*lwc).time = monotonic_seconds();
                (*lwc).msgid = msgid;
                (*lwc).inode = inode;
                (*lwc).indx = indx;
                (*lwc).chunkopflags = chunkopflags;
                (*lwc).r#type = FUSE_READ as ::core::ffi::c_int as uint8_t;
                (*lwc).status = status;
                (*lwc).next = ::core::ptr::null_mut::<_lwchunks>();
                *lwchunkshashtail[i as usize] = lwc;
                lwchunkshashtail[i as usize] = &raw mut (*lwc).next as *mut *mut lwchunks;
                return 1 as ::core::ffi::c_int;
            }
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_READ_CHUNK as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
            return 0 as ::core::ffi::c_int;
        } else {
            sessions_inc_stats((*eptr).sesdata, SES_OP_READCHUNK as uint8_t);
            split = 0 as uint8_t;
            if chunkid > 0 as uint64_t {
                if (*eptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    status = chunk_get_version_and_csdata(
                        2 as uint8_t,
                        chunkid,
                        (*eptr).peerip,
                        &raw mut version,
                        &raw mut count,
                        &raw mut cs_data as *mut uint8_t,
                        &raw mut split,
                    );
                } else if (*eptr).version
                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            32 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    status = chunk_get_version_and_csdata(
                        1 as uint8_t,
                        chunkid,
                        (*eptr).peerip,
                        &raw mut version,
                        &raw mut count,
                        &raw mut cs_data as *mut uint8_t,
                        &raw mut split,
                    );
                } else {
                    status = chunk_get_version_and_csdata(
                        0 as uint8_t,
                        chunkid,
                        (*eptr).peerip,
                        &raw mut version,
                        &raw mut count,
                        &raw mut cs_data as *mut uint8_t,
                        &raw mut split,
                    );
                }
                if status as ::core::ffi::c_int == MFS_STATUS_OK
                    && count as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && split as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && version == 0 as uint32_t
                {
                    chunkid = 0 as uint64_t;
                }
            } else {
                version = 0 as uint32_t;
                count = 0 as uint8_t;
            }
        }
        if (*eptr).version
            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
            && split as ::core::ffi::c_int == 8 as ::core::ffi::c_int
            || (*eptr).version
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 26 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
                && split as ::core::ffi::c_int == 4 as ::core::ffi::c_int
        {
            status = MFS_ERROR_IO as uint8_t;
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_READ_CHUNK as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
            return 0 as ::core::ffi::c_int;
        }
        dcm_access(inode, sessions_get_id((*eptr).sesdata));
        if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    10 as ::core::ffi::c_int
                })) as uint32_t
        {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_READ_CHUNK as uint32_t,
                (25 as ::core::ffi::c_int + count as ::core::ffi::c_int * 14 as ::core::ffi::c_int)
                    as uint32_t,
            );
        } else if (*eptr).version
            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    32 as ::core::ffi::c_int
                })) as uint32_t
        {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_READ_CHUNK as uint32_t,
                (25 as ::core::ffi::c_int + count as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                    as uint32_t,
            );
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_READ_CHUNK as uint32_t,
                (24 as ::core::ffi::c_int + count as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                    as uint32_t,
            );
        }
        put32bit(&raw mut ptr, msgid);
        if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    10 as ::core::ffi::c_int
                })) as uint32_t
        {
            if split != 0 {
                put8bit(&raw mut ptr, 3 as uint8_t);
            } else {
                put8bit(&raw mut ptr, 2 as uint8_t);
            }
        } else if (*eptr).version
            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    32 as ::core::ffi::c_int
                })) as uint32_t
        {
            put8bit(&raw mut ptr, 1 as uint8_t);
        }
        put64bit(&raw mut ptr, fleng);
        put64bit(&raw mut ptr, chunkid);
        put32bit(&raw mut ptr, version);
        if count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        10 as ::core::ffi::c_int
                    })) as uint32_t
            {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                    (count as ::core::ffi::c_int * 14 as ::core::ffi::c_int) as size_t,
                );
            } else if (*eptr).version
                >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        32 as ::core::ffi::c_int
                    })) as uint32_t
            {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                    (count as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as size_t,
                );
            } else {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                    (count as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as size_t,
                );
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn matoclserv_fuse_truncate_common(
    mut eptr: *mut matoclserventry,
    mut msgid: uint32_t,
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut auid: uint32_t,
    mut agid: uint32_t,
    mut fleng: uint64_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut indx: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut prevlength: uint64_t = 0;
        let mut prevchunkid: uint64_t = 0;
        let mut disables: uint32_t = 0;
        let mut swc: *mut swchunks = ::core::ptr::null_mut::<swchunks>();
        let mut lwc: *mut lwchunks = ::core::ptr::null_mut::<lwchunks>();
        let mut chunkid: uint64_t = 0;
        chunkid = 0 as uint64_t;
        status = MFS_STATUS_OK as uint8_t;
        disables = sessions_get_disables((*eptr).sesdata);
        if flags as ::core::ffi::c_int & (TRUNCATE_FLAG_RESERVE | TRUNCATE_FLAG_UPDATE) != 0 {
            if disables & DISABLE_WRITE as uint32_t != 0 {
                status = MFS_ERROR_EPERM as uint8_t;
            }
        } else if disables & (DISABLE_TRUNCATE as uint32_t | DISABLE_SETLENGTH as uint32_t)
            == DISABLE_TRUNCATE as uint32_t | DISABLE_SETLENGTH as uint32_t
        {
            status = MFS_ERROR_EPERM as uint8_t;
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            status = fs_try_setlength(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                flags,
                uid,
                gids,
                gid,
                ((if disables & DISABLE_TRUNCATE as uint32_t != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) | (if disables & DISABLE_SETLENGTH as uint32_t != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint8_t,
                fleng,
                &raw mut indx,
                &raw mut prevchunkid,
                &raw mut chunkid,
            );
        }
        if status as ::core::ffi::c_int == MFS_ERROR_DELAYED {
            i = (chunkid & 0xff as uint64_t) as uint32_t;
            swc = malloc(::core::mem::size_of::<swchunks>()) as *mut swchunks;
            if swc.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if swc
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut swchunks
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    521 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*swc).chunkid = chunkid;
            (*swc).prevchunkid = prevchunkid;
            (*swc).eptr = eptr;
            (*swc).msgid = msgid;
            (*swc).inode = inode;
            (*swc).indx = indx;
            (*swc).uid = uid;
            (*swc).gid = *gid.offset(0 as isize);
            (*swc).auid = auid;
            (*swc).agid = agid;
            (*swc).fleng = fleng;
            (*swc).flags = flags;
            (*swc).r#type = FUSE_TRUNCATE as ::core::ffi::c_int as uint8_t;
            (*swc).next = swchunkshash[i as usize] as *mut _swchunks;
            swchunkshash[i as usize] = swc;
            sessions_inc_stats((*eptr).sesdata, SES_OP_TRUNCATE as uint8_t);
            return 0 as ::core::ffi::c_int;
        }
        if status as ::core::ffi::c_int == MFS_ERROR_LOCKED
            || status as ::core::ffi::c_int == MFS_ERROR_CHUNKBUSY
        {
            i = (prevchunkid & 0xff as uint64_t) as uint32_t;
            lwc = malloc(
                ::core::mem::size_of::<lwchunks>()
                    .wrapping_add(::core::mem::size_of::<uint32_t>().wrapping_mul(gids as size_t)),
            ) as *mut lwchunks;
            if lwc.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    544 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    544 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if lwc
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut lwchunks
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    544 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    544 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"lwc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            (*lwc).chunkid = prevchunkid;
            (*lwc).fleng = fleng;
            (*lwc).eptr = eptr;
            (*lwc).status = status;
            (*lwc).time = monotonic_seconds();
            (*lwc).msgid = msgid;
            (*lwc).inode = inode;
            (*lwc).indx = indx;
            (*lwc).uid = uid;
            (*lwc).gids = gids;
            (*lwc).gid = (lwc as *mut uint8_t).offset(::core::mem::size_of::<lwchunks>() as isize)
                as *mut uint32_t;
            memcpy(
                (*lwc).gid as *mut ::core::ffi::c_void,
                gid as *const ::core::ffi::c_void,
                ::core::mem::size_of::<uint32_t>().wrapping_mul(gids as size_t),
            );
            (*lwc).auid = auid;
            (*lwc).agid = agid;
            (*lwc).flags = flags;
            (*lwc).r#type = FUSE_TRUNCATE as ::core::ffi::c_int as uint8_t;
            (*lwc).status = status;
            (*lwc).next = ::core::ptr::null_mut::<_lwchunks>();
            *lwchunkshashtail[i as usize] = lwc;
            lwchunkshashtail[i as usize] = &raw mut (*lwc).next as *mut *mut lwchunks;
            return 1 as ::core::ffi::c_int;
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            status = fs_do_setlength(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                flags,
                uid,
                *gid.offset(0 as isize),
                auid,
                agid,
                fleng,
                &raw mut attr as *mut uint8_t,
                &raw mut prevlength,
            );
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK
            && flags as ::core::ffi::c_int & TRUNCATE_FLAG_UPDATE == 0 as ::core::ffi::c_int
        {
            dcm_modify(inode, sessions_get_id((*eptr).sesdata));
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_TRUNCATE as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        } else {
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        113 as ::core::ffi::c_int
                    })) as uint32_t
                && (*eptr).version
                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                || (*eptr).version
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
            {
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_TRUNCATE as uint32_t,
                    ((*eptr).asize as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as uint32_t,
                );
            } else {
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_TRUNCATE as uint32_t,
                    ((*eptr).asize as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t,
                );
            }
            put32bit(&raw mut ptr, msgid);
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        113 as ::core::ffi::c_int
                    })) as uint32_t
                && (*eptr).version
                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                || (*eptr).version
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
            {
                put64bit(&raw mut ptr, prevlength);
            }
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
            if flags as ::core::ffi::c_int & TRUNCATE_FLAG_RESERVE == 0 as ::core::ffi::c_int {
                matoclserv_fuse_fleng_has_changed(eptr, inode, fleng);
            }
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_TRUNCATE as uint8_t);
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_chunk_unlocked(
    mut chunkid: uint64_t,
    mut cptr: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut lwc: *mut lwchunks = ::core::ptr::null_mut::<lwchunks>();
        let mut plwc: *mut *mut lwchunks = ::core::ptr::null_mut::<*mut lwchunks>();
        let mut locked: uint8_t = 0;
        plwc = (&raw mut lwchunkshashhead as *mut *mut lwchunks)
            .offset((chunkid & 0xff as uint64_t) as isize);
        loop {
            lwc = *plwc;
            if lwc.is_null() {
                break;
            }
            if (*lwc).chunkid == chunkid
                && (*(*lwc).eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            {
                if (*lwc).r#type as ::core::ffi::c_int == FUSE_TRUNCATE as ::core::ffi::c_int {
                    locked = matoclserv_fuse_truncate_common(
                        (*lwc).eptr,
                        (*lwc).msgid,
                        (*lwc).inode,
                        (*lwc).flags,
                        (*lwc).uid,
                        (*lwc).gids,
                        (*lwc).gid,
                        (*lwc).auid,
                        (*lwc).agid,
                        (*lwc).fleng,
                    ) as uint8_t;
                } else if (*lwc).r#type as ::core::ffi::c_int == FUSE_WRITE as ::core::ffi::c_int {
                    locked = matoclserv_fuse_write_chunk_common(
                        (*lwc).eptr,
                        (*lwc).msgid,
                        (*lwc).inode,
                        (*lwc).indx,
                        (*lwc).chunkopflags,
                    ) as uint8_t;
                } else if (*lwc).r#type as ::core::ffi::c_int == FUSE_READ as ::core::ffi::c_int {
                    locked = matoclserv_fuse_read_chunk_common(
                        (*lwc).eptr,
                        (*lwc).msgid,
                        (*lwc).inode,
                        (*lwc).indx,
                        (*lwc).chunkopflags,
                    ) as uint8_t;
                } else {
                    locked = 0 as uint8_t;
                }
                *plwc = (*lwc).next as *mut lwchunks;
                free(lwc as *mut ::core::ffi::c_void);
                if locked as ::core::ffi::c_int != 0 || chunk_locked_or_busy(cptr) != 0 {
                    break;
                }
            } else {
                plwc = &raw mut (*lwc).next as *mut *mut lwchunks;
            }
        }
        if (*plwc).is_null() {
            lwchunkshashtail[(chunkid & 0xff as uint64_t) as usize] = plwc;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_timeout_waiting_ops() {
    unsafe {
        let mut lwc: *mut lwchunks = ::core::ptr::null_mut::<lwchunks>();
        let mut plwc: *mut *mut lwchunks = ::core::ptr::null_mut::<*mut lwchunks>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut curtime: ::core::ffi::c_double = 0.;
        curtime = monotonic_seconds();
        i = 0 as uint32_t;
        while i < CHUNKHASHSIZE as uint32_t {
            plwc = (&raw mut lwchunkshashhead as *mut *mut lwchunks).offset(i as isize);
            loop {
                lwc = *plwc;
                if lwc.is_null() {
                    break;
                }
                if (*lwc).time + CHUNK_WAIT_TIMEOUT < curtime {
                    if (*lwc).r#type as ::core::ffi::c_int == FUSE_TRUNCATE as ::core::ffi::c_int {
                        ptr = matoclserv_create_packet(
                            (*lwc).eptr,
                            MATOCL_FUSE_TRUNCATE as uint32_t,
                            5 as uint32_t,
                        );
                        put32bit(&raw mut ptr, (*lwc).msgid);
                        put8bit(&raw mut ptr, (*lwc).status);
                    } else if (*lwc).r#type as ::core::ffi::c_int
                        == FUSE_WRITE as ::core::ffi::c_int
                    {
                        ptr = matoclserv_create_packet(
                            (*lwc).eptr,
                            MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                            5 as uint32_t,
                        );
                        put32bit(&raw mut ptr, (*lwc).msgid);
                        put8bit(&raw mut ptr, (*lwc).status);
                    } else if (*lwc).r#type as ::core::ffi::c_int == FUSE_READ as ::core::ffi::c_int
                    {
                        ptr = matoclserv_create_packet(
                            (*lwc).eptr,
                            MATOCL_FUSE_READ_CHUNK as uint32_t,
                            5 as uint32_t,
                        );
                        put32bit(&raw mut ptr, (*lwc).msgid);
                        put8bit(&raw mut ptr, (*lwc).status);
                    }
                    *plwc = (*lwc).next as *mut lwchunks;
                    free(lwc as *mut ::core::ffi::c_void);
                } else {
                    plwc = &raw mut (*lwc).next as *mut *mut lwchunks;
                }
            }
            lwchunkshashtail[i as usize] = plwc;
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_chunk_status(mut chunkid: uint64_t, mut status: uint8_t) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gid: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut fleng: uint64_t = 0;
        let mut prevchunkid: uint64_t = 0;
        let mut prevlength: uint64_t = 0;
        let mut offset: uint64_t = 0;
        let mut flags: uint8_t = 0;
        let mut r#type: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut version: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut split: uint8_t = 0;
        let mut count: uint8_t = 0;
        let mut cs_data: [uint8_t; 1400] = [0; 1400];
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut swc: *mut swchunks = ::core::ptr::null_mut::<swchunks>();
        let mut pswc: *mut *mut swchunks = ::core::ptr::null_mut::<*mut swchunks>();
        eptr = ::core::ptr::null_mut::<matoclserventry>();
        prevchunkid = 0 as uint64_t;
        msgid = 0 as uint32_t;
        fleng = 0 as uint64_t;
        r#type = 0 as uint8_t;
        inode = 0 as uint32_t;
        indx = 0 as uint32_t;
        uid = 0 as uint32_t;
        gid = 0 as uint32_t;
        auid = 0 as uint32_t;
        agid = 0 as uint32_t;
        flags = 0 as uint8_t;
        pswc = (&raw mut swchunkshash as *mut *mut swchunks)
            .offset((chunkid & 0xff as uint64_t) as isize);
        loop {
            swc = *pswc;
            if swc.is_null() {
                break;
            }
            if (*swc).chunkid == chunkid {
                eptr = (*swc).eptr;
                prevchunkid = (*swc).prevchunkid;
                msgid = (*swc).msgid;
                fleng = (*swc).fleng;
                r#type = (*swc).r#type;
                flags = (*swc).flags;
                inode = (*swc).inode;
                indx = (*swc).indx;
                uid = (*swc).uid;
                gid = (*swc).gid;
                auid = (*swc).auid;
                agid = (*swc).agid;
                *pswc = (*swc).next as *mut swchunks;
                free(swc as *mut ::core::ffi::c_void);
                break;
            } else {
                pswc = &raw mut (*swc).next as *mut *mut swchunks;
            }
        }
        if eptr.is_null() {
            return;
        }
        if (*eptr).mode as ::core::ffi::c_int != DATA as ::core::ffi::c_int {
            return;
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK
            && r#type as ::core::ffi::c_int != FUSE_CREATE as ::core::ffi::c_int
        {
            dcm_modify(inode, sessions_get_id((*eptr).sesdata));
        }
        match r#type as ::core::ffi::c_int {
            3 => {
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    fs_writeend(
                        inode,
                        0 as uint64_t,
                        chunkid,
                        0 as uint8_t,
                        ::core::ptr::null_mut::<uint8_t>(),
                    );
                }
                return;
            }
            0 => {
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                10 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        status = chunk_get_version_and_csdata(
                            2 as uint8_t,
                            chunkid,
                            (*eptr).peerip,
                            &raw mut version,
                            &raw mut count,
                            &raw mut cs_data as *mut uint8_t,
                            &raw mut split,
                        );
                    } else if (*eptr).version
                        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                32 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        status = chunk_get_version_and_csdata(
                            1 as uint8_t,
                            chunkid,
                            (*eptr).peerip,
                            &raw mut version,
                            &raw mut count,
                            &raw mut cs_data as *mut uint8_t,
                            &raw mut split,
                        );
                    } else {
                        status = chunk_get_version_and_csdata(
                            0 as uint8_t,
                            chunkid,
                            (*eptr).peerip,
                            &raw mut version,
                            &raw mut count,
                            &raw mut cs_data as *mut uint8_t,
                            &raw mut split,
                        );
                    }
                }
                if status as ::core::ffi::c_int == MFS_STATUS_OK && split as ::core::ffi::c_int != 0
                {
                    status = MFS_ERROR_EAGAIN as uint8_t;
                }
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                        5 as uint32_t,
                    );
                    put32bit(&raw mut ptr, msgid);
                    put8bit(&raw mut ptr, status);
                    fs_rollback(inode, indx, prevchunkid, chunkid);
                    return;
                }
                if (*eptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                        (25 as ::core::ffi::c_int
                            + count as ::core::ffi::c_int * 14 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                } else if (*eptr).version
                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            32 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                        (25 as ::core::ffi::c_int
                            + count as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                } else {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_WRITE_CHUNK as uint32_t,
                        (24 as ::core::ffi::c_int
                            + count as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                }
                put32bit(&raw mut ptr, msgid);
                if (*eptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            10 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    put8bit(&raw mut ptr, 2 as uint8_t);
                } else if (*eptr).version
                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            32 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    put8bit(&raw mut ptr, 1 as uint8_t);
                }
                put64bit(&raw mut ptr, fleng);
                put64bit(&raw mut ptr, chunkid);
                put32bit(&raw mut ptr, version);
                if count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                10 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                10 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        memcpy(
                            ptr as *mut ::core::ffi::c_void,
                            &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                            (count as ::core::ffi::c_int * 14 as ::core::ffi::c_int) as size_t,
                        );
                    } else if (*eptr).version
                        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                32 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        memcpy(
                            ptr as *mut ::core::ffi::c_void,
                            &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                            (count as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as size_t,
                        );
                    } else {
                        memcpy(
                            ptr as *mut ::core::ffi::c_void,
                            &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                            (count as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as size_t,
                        );
                    }
                }
                matoclserv_fuse_chunk_has_changed(
                    eptr,
                    inode,
                    indx,
                    chunkid,
                    version,
                    fleng,
                    0 as uint8_t,
                    0 as uint32_t,
                    0 as uint32_t,
                );
                return;
            }
            2 => {
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_TRUNCATE as uint32_t,
                        5 as uint32_t,
                    );
                    put32bit(&raw mut ptr, msgid);
                    put8bit(&raw mut ptr, status);
                    fs_rollback(inode, indx, prevchunkid, chunkid);
                    return;
                }
                fs_end_setlength(chunkid);
                status = fs_do_setlength(
                    sessions_get_rootinode((*eptr).sesdata),
                    sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                    inode,
                    flags,
                    uid,
                    gid,
                    auid,
                    agid,
                    fleng,
                    &raw mut attr as *mut uint8_t,
                    &raw mut prevlength,
                );
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_TRUNCATE as uint32_t,
                        5 as uint32_t,
                    );
                    put32bit(&raw mut ptr, msgid);
                    put8bit(&raw mut ptr, status);
                    return;
                }
                if (*eptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            113 as ::core::ffi::c_int
                        })) as uint32_t
                    && (*eptr).version
                        < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    || (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_TRUNCATE as uint32_t,
                        ((*eptr).asize as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                } else {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_TRUNCATE as uint32_t,
                        ((*eptr).asize as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t,
                    );
                }
                put32bit(&raw mut ptr, msgid);
                if (*eptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            113 as ::core::ffi::c_int
                        })) as uint32_t
                    && (*eptr).version
                        < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    || (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    put64bit(&raw mut ptr, prevlength);
                }
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                    (*eptr).asize as size_t,
                );
                chunk_get_version(chunkid, &raw mut version);
                if flags as ::core::ffi::c_int & TRUNCATE_FLAG_RESERVE != 0 {
                    fleng = fleng.wrapping_add(prevlength);
                }
                offset = indx.wrapping_mul(MFSCHUNKSIZE as uint32_t) as uint64_t;
                if fleng < prevlength {
                    prevlength = fleng;
                }
                if prevlength <= offset {
                    offset = 0 as uint64_t;
                } else {
                    offset = prevlength.wrapping_sub(offset);
                }
                if offset >= MFSCHUNKSIZE as uint64_t {
                    offset = MFSCHUNKSIZE as uint64_t;
                }
                matoclserv_fuse_chunk_has_changed(
                    eptr,
                    inode,
                    indx,
                    chunkid,
                    version,
                    fleng,
                    1 as uint8_t,
                    offset as uint32_t,
                    0 as uint32_t,
                );
                return;
            }
            _ => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"got chunk status, but operation type is unknown\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_cserv_list(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut mode: uint8_t = 0;
        if length != 0 as uint32_t && length != 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_CSERV_LIST - wrong size (%u/0|1)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 1 as uint32_t {
            mode = get8bit(&raw mut data);
        } else {
            mode = 0 as uint8_t;
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_CSERV_LIST as uint32_t,
            csdb_servlist_data(mode, ::core::ptr::null_mut::<uint8_t>(), 0 as uint32_t),
        );
        csdb_servlist_data(mode, ptr, (*eptr).peerip);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_cserv_command(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ip: uint32_t = 0;
        let mut port: uint16_t = 0;
        let mut cmd: uint8_t = 0;
        let mut status: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 6 as uint32_t && length != 7 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_CSSERV_COMMAND - wrong size (%u/6|7)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 7 as uint32_t {
            cmd = get8bit(&raw mut data);
        } else {
            cmd = MFS_CSSERV_COMMAND_REMOVE as uint8_t;
        }
        ip = get32bit(&raw mut data);
        port = get16bit(&raw mut data);
        status = MFS_ERROR_EINVAL as uint8_t;
        if cmd as ::core::ffi::c_int == MFS_CSSERV_COMMAND_REMOVE {
            status = csdb_remove_server(ip, port);
        } else if cmd as ::core::ffi::c_int == MFS_CSSERV_COMMAND_BACKTOWORK {
            status = csdb_back_to_work(ip, port);
        } else if cmd as ::core::ffi::c_int == MFS_CSSERV_COMMAND_MAINTENANCEON {
            status = csdb_maintenance(ip, port, 1 as uint8_t);
        } else if cmd as ::core::ffi::c_int == MFS_CSSERV_COMMAND_MAINTENANCEOFF {
            status = csdb_maintenance(ip, port, 0 as uint8_t);
        }
        if length == 6 as uint32_t {
            matoclserv_create_packet(eptr, MATOCL_CSSERV_COMMAND as uint32_t, 0 as uint32_t);
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_CSSERV_COMMAND as uint32_t, 1 as uint32_t);
            put8bit(&raw mut ptr, status);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_session_list(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut size: uint32_t = 0;
        let mut vmode: uint8_t = 0;
        let mut msgid: uint32_t = 0;
        if length != 0 as uint32_t && length != 1 as uint32_t && length != 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SESSION_LIST - wrong size (%u/0|1|5)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        vmode = 0 as uint8_t;
        msgid = 0 as uint32_t;
        if length >= 1 as uint32_t {
            if length >= 5 as uint32_t {
                msgid = get32bit(&raw mut data);
            }
            vmode = get8bit(&raw mut data);
        }
        size = sessions_datasize(vmode);
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_SESSION_LIST as uint32_t,
            size.wrapping_add(
                (if length >= 5 as uint32_t {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint32_t,
            ),
        );
        if length >= 5 as uint32_t {
            put32bit(&raw mut ptr, msgid);
        }
        sessions_datafill(ptr, vmode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_session_command(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut sessionid: uint32_t = 0;
        let mut cmd: uint8_t = 0;
        let mut status: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SESSION_COMMAND - wrong size (%u/5)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        cmd = get8bit(&raw mut data);
        sessionid = get32bit(&raw mut data);
        if cmd as ::core::ffi::c_int == MFS_SESSION_COMMAND_REMOVE {
            status = sessions_force_remove(sessionid);
        } else {
            status = MFS_ERROR_EINVAL as uint8_t;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_SESSION_COMMAND as uint32_t, 1 as uint32_t);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_chart(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chartid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut l: uint32_t = 0;
        let mut w: uint16_t = 0;
        let mut h: uint16_t = 0;
        if length != 4 as uint32_t && length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOAN_CHART - wrong size (%u/4|8)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chartid = get32bit(&raw mut data);
        if length == 8 as uint32_t {
            w = get16bit(&raw mut data);
            h = get16bit(&raw mut data);
        } else {
            w = 0 as uint16_t;
            h = 0 as uint16_t;
        }
        l = charts_make_png(chartid, w as uint32_t, h as uint32_t);
        ptr = matoclserv_create_packet(eptr, ANTOCL_CHART as uint32_t, l);
        if l > 0 as uint32_t {
            charts_get_png(ptr);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_chart_data(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chartid: uint32_t = 0;
        let mut maxentries: uint32_t = 0;
        let mut multimode: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut l: uint32_t = 0;
        if length != 4 as uint32_t && length != 8 as uint32_t && length != 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOAN_CHART_DATA - wrong size (%u/4|8|9)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        maxentries = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        multimode = 0 as uint8_t;
        chartid = get32bit(&raw mut data);
        if length >= 8 as uint32_t {
            maxentries = get32bit(&raw mut data);
        }
        if length >= 9 as uint32_t {
            multimode = get8bit(&raw mut data);
        }
        l = charts_makedata(
            ::core::ptr::null_mut::<uint8_t>(),
            chartid,
            maxentries,
            multimode,
        );
        ptr = matoclserv_create_packet(eptr, ANTOCL_CHART_DATA as uint32_t, l);
        if l > 0 as uint32_t {
            charts_makedata(ptr, chartid, maxentries, multimode);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_monotonic_data(
    mut eptr: *mut matoclserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut l: uint32_t = 0;
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOAN_MONOTONIC_DATA - wrong size (%u/0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        l = charts_monotonic_data(::core::ptr::null_mut::<uint8_t>());
        ptr = matoclserv_create_packet(eptr, ANTOCL_MONOTONIC_DATA as uint32_t, l);
        if l > 0 as uint32_t {
            charts_monotonic_data(ptr);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_get_version(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0 as uint32_t;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut vstring: [::core::ffi::c_char; 9] = VERSSTR;
        if length != 0 as uint32_t && length != 4 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_VERSION - wrong size (%u/4|0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 4 as uint32_t {
            msgid = get32bit(&raw mut data);
            ptr = matoclserv_create_packet(
                eptr,
                ANTOAN_VERSION as uint32_t,
                ((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(strlen(&raw const vstring as *const ::core::ffi::c_char))
                    as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                ANTOAN_VERSION as uint32_t,
                (4 as size_t).wrapping_add(strlen(&raw const vstring as *const ::core::ffi::c_char))
                    as uint32_t,
            );
        }
        put16bit(&raw mut ptr, VERSMAJ as uint16_t);
        put8bit(&raw mut ptr, VERSMID as uint8_t);
        put8bit(&raw mut ptr, VERSMIN as uint8_t);
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            &raw const vstring as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(&raw const vstring as *const ::core::ffi::c_char),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_get_config(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut name: [::core::ffi::c_char; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut vleng: uint32_t = 0;
        let mut val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_CONFIG - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length != (5 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_CONFIG - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        memcpy(
            &raw mut name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        name[nleng as usize] = 0 as ::core::ffi::c_char;
        if strcmp(
            &raw mut name as *mut ::core::ffi::c_char,
            b"AUTH_CODE\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if cfg_isdefined(&raw mut name as *mut ::core::ffi::c_char) != 0 {
                val = strdup(b"[DEFINED]\0".as_ptr() as *const ::core::ffi::c_char);
            } else {
                val = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        } else {
            val = cfg_getdefaultstr(&raw mut name as *mut ::core::ffi::c_char);
        }
        if !val.is_null() {
            vleng = strlen(val) as uint32_t;
            if vleng > 255 as uint32_t {
                vleng = 255 as uint32_t;
            }
        } else {
            vleng = 0 as uint32_t;
        }
        if msgid == 0 as uint32_t {
            ptr = matoclserv_create_packet(
                eptr,
                ANTOAN_CONFIG_VALUE as uint32_t,
                ((6 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add(vleng),
            );
            put32bit(&raw mut ptr, 0 as uint32_t);
            put8bit(&raw mut ptr, nleng);
            if nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut name as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    nleng as size_t,
                );
                ptr = ptr.offset(nleng as ::core::ffi::c_int as isize);
            }
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                ANTOAN_CONFIG_VALUE as uint32_t,
                (5 as uint32_t).wrapping_add(vleng),
            );
            put32bit(&raw mut ptr, msgid);
        }
        put8bit(&raw mut ptr, vleng as uint8_t);
        if vleng > 0 as uint32_t && !val.is_null() {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                vleng as size_t,
            );
        }
        if !val.is_null() {
            free(val as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_get_config_file(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut name: [::core::ffi::c_char; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut fdata: *mut cfg_buff = ::core::ptr::null_mut::<cfg_buff>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_CONFIG_FILE - wrong size (%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length != (5 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_CONFIG_FILE - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        memcpy(
            &raw mut name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        name[nleng as usize] = 0 as ::core::ffi::c_char;
        if strcmp(
            &raw mut name as *mut ::core::ffi::c_char,
            b"LICENCE_FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            fdata =
                cfg_getdefaultfile(&raw mut name as *mut ::core::ffi::c_char, 65535 as uint32_t);
        } else {
            fdata = ::core::ptr::null_mut::<cfg_buff>();
        }
        if fdata.is_null() {
            ptr = matoclserv_create_packet(
                eptr,
                ANTOAN_CONFIG_FILE_CONTENT as uint32_t,
                5 as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, MFS_ERROR_ENOENT as uint8_t);
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                ANTOAN_CONFIG_FILE_CONTENT as uint32_t,
                (6 as uint32_t).wrapping_add((*fdata).leng),
            );
            put32bit(&raw mut ptr, msgid);
            put16bit(&raw mut ptr, (*fdata).leng as uint16_t);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut (*fdata).data as *mut uint8_t as *const ::core::ffi::c_void,
                (*fdata).leng as size_t,
            );
            free(fdata as *mut ::core::ffi::c_void);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_syslog(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut priority: uint8_t = 0;
        let mut timestamp: uint32_t = 0;
        let mut msgsize: uint16_t = 0;
        if length < 7 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_SYSLOG - wrong size (%u/>=7)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        priority = get8bit(&raw mut data);
        timestamp = get32bit(&raw mut data);
        msgsize = get16bit(&raw mut data);
        if length != (7 as uint32_t).wrapping_add(msgsize as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_SYSLOG - wrong size (%u/7+msgsize(%hu))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                msgsize as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_module_info(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0 as uint32_t;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 0 as uint32_t && length != 4 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOAN_MODULE_INFO - wrong size (%u/4|0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 4 as uint32_t {
            msgid = get32bit(&raw mut data);
            ptr = matoclserv_create_packet(eptr, ANTOCL_MODULE_INFO as uint32_t, 25 as uint32_t);
            put32bit(&raw mut ptr, msgid);
        } else {
            ptr = matoclserv_create_packet(eptr, ANTOCL_MODULE_INFO as uint32_t, 21 as uint32_t);
        }
        put8bit(&raw mut ptr, MODULE_TYPE_MASTER as uint8_t);
        put16bit(&raw mut ptr, VERSMAJ as uint16_t);
        put8bit(&raw mut ptr, VERSMID as uint8_t);
        put8bit(&raw mut ptr, VERSMIN as uint8_t);
        put16bit(&raw mut ptr, 0 as uint16_t);
        put64bit(&raw mut ptr, meta_get_id());
        put32bit(&raw mut ptr, 0 as uint32_t);
        put16bit(&raw mut ptr, 0 as uint16_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_list_open_files(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0 as uint32_t;
        let mut sessionid: uint32_t = 0;
        let mut size: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 4 as uint32_t && length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_LIST_OPEN_FILES - wrong size (%u/4|8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 8 as uint32_t {
            msgid = get32bit(&raw mut data);
        }
        sessionid = get32bit(&raw mut data);
        size = of_lsof(sessionid, ::core::ptr::null_mut::<uint8_t>());
        if length == 8 as uint32_t {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_LIST_OPEN_FILES as uint32_t,
                (4 as uint32_t).wrapping_add(size),
            );
            put32bit(&raw mut ptr, msgid);
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_LIST_OPEN_FILES as uint32_t, size);
        }
        of_lsof(sessionid, ptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_list_acquired_locks(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0 as uint32_t;
        let mut inode: uint32_t = 0;
        let mut psize: uint32_t = 0;
        let mut fsize: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 4 as uint32_t && length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_LIST_ACQUIRED_LOCKS - wrong size (%u/4|8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 8 as uint32_t {
            msgid = get32bit(&raw mut data);
        }
        inode = get32bit(&raw mut data);
        psize = posix_lock_list(inode, ::core::ptr::null_mut::<uint8_t>());
        fsize = flock_list(inode, ::core::ptr::null_mut::<uint8_t>());
        if length == 8 as uint32_t {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_LIST_ACQUIRED_LOCKS as uint32_t,
                (4 as uint32_t).wrapping_add(psize).wrapping_add(fsize),
            );
            put32bit(&raw mut ptr, msgid);
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_LIST_ACQUIRED_LOCKS as uint32_t,
                psize.wrapping_add(fsize),
            );
        }
        posix_lock_list(inode, ptr);
        flock_list(inode, ptr.offset(psize as isize));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_mass_resolve_paths(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        static mut inodetab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut psizetab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut tabsleng: uint32_t = 0 as uint32_t;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut totalsize: uint32_t = 0;
        let mut psize: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length.wrapping_rem(4 as uint32_t) != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_MASS_RESOLVE_PATHS - wrong size (%u/N*4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        length >>= 2 as ::core::ffi::c_int;
        if length > tabsleng {
            if !inodetab.is_null() {
                free(inodetab as *mut ::core::ffi::c_void);
            }
            if !psizetab.is_null() {
                free(psizetab as *mut ::core::ffi::c_void);
            }
            tabsleng = length.wrapping_add(0xff as uint32_t) & 0xffffff00 as uint32_t;
            inodetab = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(tabsleng as size_t))
                as *mut uint32_t;
            if inodetab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1248 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1248 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if inodetab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1248 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1248 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            psizetab = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(tabsleng as size_t))
                as *mut uint32_t;
            if psizetab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"psizetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"psizetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if psizetab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"psizetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"psizetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        j = 0 as uint32_t;
        totalsize = 0 as uint32_t;
        while length > 0 as uint32_t {
            i = get32bit(&raw mut data);
            if i > 0 as uint32_t {
                fs_get_paths_size(MFS_ROOT_ID as uint32_t, i, &raw mut psize);
                *inodetab.offset(j as isize) = i;
                *psizetab.offset(j as isize) = psize;
                j = j.wrapping_add(1);
                totalsize = totalsize.wrapping_add((8 as uint32_t).wrapping_add(psize));
            }
            length = length.wrapping_sub(1);
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_MASS_RESOLVE_PATHS as uint32_t, totalsize);
        i = 0 as uint32_t;
        while i < j {
            put32bit(&raw mut ptr, *inodetab.offset(i as isize));
            put32bit(&raw mut ptr, *psizetab.offset(i as isize));
            fs_get_paths_data(MFS_ROOT_ID as uint32_t, *inodetab.offset(i as isize), ptr);
            ptr = ptr.offset(*psizetab.offset(i as isize) as isize);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_sclass_info(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut fver: uint8_t = 0;
        if length != 0 as uint32_t && length != 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SCLASS_INFO - wrong size (%u/0)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 1 as uint32_t {
            fver = get8bit(&raw mut data);
        } else {
            fver = 0 as uint8_t;
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_SCLASS_INFO as uint32_t,
            sclass_info(::core::ptr::null_mut::<uint8_t>(), fver),
        );
        sclass_info(ptr, fver);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_missing_chunks(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut mode: uint8_t = 0;
        if length != 0 as uint32_t && length != 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_MISSING_CHUNKS - wrong size (%u/0|1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 1 as uint32_t {
            mode = get8bit(&raw mut data);
        } else {
            mode = 0 as uint8_t;
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_MISSING_CHUNKS as uint32_t,
            missing_log_getdata(::core::ptr::null_mut::<uint8_t>(), mode),
        );
        missing_log_getdata(ptr, mode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_node_info(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut inode: uint32_t = 0;
        let mut maxentries: uint32_t = 0;
        let mut continueid: uint64_t = 0;
        let mut msgid: uint32_t = 0;
        let mut eights_mode: uint8_t = 0;
        if length != 16 as uint32_t && length != 20 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_NODE_INFO - wrong size (%u/16|20)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 20 as uint32_t {
            msgid = get32bit(&raw mut data);
        } else {
            msgid = 0 as uint32_t;
        }
        inode = get32bit(&raw mut data);
        maxentries = get32bit(&raw mut data);
        continueid = get64bit(&raw mut data);
        eights_mode = (if (*eptr).version
            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_NODE_INFO as uint32_t,
            fs_node_info(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                eights_mode,
                inode,
                maxentries,
                continueid,
                ::core::ptr::null_mut::<uint8_t>(),
            )
            .wrapping_add(
                (if length == 20 as uint32_t {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint32_t,
            ),
        );
        if length == 20 as uint32_t {
            put32bit(&raw mut ptr, msgid);
        }
        fs_node_info(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            eights_mode,
            inode,
            maxentries,
            continueid,
            ptr,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_full_directory_data(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut neptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut dleng: uint32_t = 0;
        let mut maxentries: uint32_t = 0;
        let mut nedgeid: uint64_t = 0;
        let mut flags: uint8_t = 0;
        let mut userptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*eptr).asize as ::core::ffi::c_int != ATTR_RECORD_SIZE {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FULL_DIRECTORY_DATA - requested attr size not implemented\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length != 20 as uint32_t && length != 21 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FULL_DIRECTORY_DATA - wrong size (%u/20|21)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        maxentries = get32bit(&raw mut data);
        nedgeid = get64bit(&raw mut data);
        if length >= 21 as uint32_t {
            flags = get8bit(&raw mut data);
        } else {
            flags = (FULL_DIRECTORY_ADD_CHUNKID | FULL_DIRECTORY_ADD_SYMLINK) as uint8_t;
        }
        if sessions_get_disables((*eptr).sesdata) & DISABLE_READDIR as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_readdirfull(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                flags,
                maxentries,
                &raw mut nedgeid,
                &raw mut userptr,
                ::core::ptr::null_mut::<uint8_t>(),
                &raw mut dleng,
            );
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FULL_DIRECTORY_DATA as uint32_t,
                5 as uint32_t,
            );
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FULL_DIRECTORY_DATA as uint32_t,
                (12 as uint32_t).wrapping_add(dleng),
            );
        }
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            neptr = ptr;
            put64bit(&raw mut ptr, 0 as uint64_t);
            fs_readdirfull(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                flags,
                maxentries,
                &raw mut nedgeid,
                &raw mut userptr,
                ptr,
                &raw mut dleng,
            );
            put64bit(&raw mut neptr, nedgeid);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_set_all_node_attributes(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut flags: uint8_t = 0;
        let mut disables: uint32_t = 0;
        let mut uid: uint32_t = 0;
        if length < 13 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SET_ALL_NODE_ATTRIBUTES - wrong size (%u/<13)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        uid = get32bit(&raw mut data);
        flags = get8bit(&raw mut data);
        sessions_ugid_remap(
            (*eptr).sesdata,
            &raw mut uid,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        disables = sessions_get_disables((*eptr).sesdata);
        if flags as ::core::ffi::c_int & SET_ALL_EATTR != 0
            && disables & DISABLE_SETEATTR as uint32_t != 0
        {
            status = MFS_ERROR_EPERM as uint8_t;
        } else if flags as ::core::ffi::c_int & SET_ALL_XATTR != 0
            && disables & DISABLE_SETXATTR as uint32_t != 0
        {
            status = MFS_ERROR_EPERM as uint8_t;
        } else if flags as ::core::ffi::c_int & SET_ALL_FACL != 0
            && disables & DISABLE_SETFACL as uint32_t != 0
        {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_set_additional_attributes(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                flags,
                uid,
                data,
                length.wrapping_sub(13 as uint32_t),
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_SET_ALL_NODE_ATTRIBUTES as uint32_t,
            5 as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_info(
    mut eptr: *mut matoclserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut totalspace: uint64_t = 0;
        let mut availspace: uint64_t = 0;
        let mut freespace: uint64_t = 0;
        let mut trspace: uint64_t = 0;
        let mut respace: uint64_t = 0;
        let mut memusage: uint64_t = 0;
        let mut syscpu: uint64_t = 0;
        let mut usercpu: uint64_t = 0;
        let mut trnodes: uint32_t = 0;
        let mut renodes: uint32_t = 0;
        let mut inodes: uint32_t = 0;
        let mut dnodes: uint32_t = 0;
        let mut fnodes: uint32_t = 0;
        let mut chunks: uint32_t = 0;
        let mut copychunks: uint32_t = 0;
        let mut ec8chunks: uint32_t = 0;
        let mut ec4chunks: uint32_t = 0;
        let mut chunkcopies: uint64_t = 0;
        let mut chunkec8parts: uint64_t = 0;
        let mut chunkec4parts: uint64_t = 0;
        let mut chunkhypotheticalcopies: uint64_t = 0;
        let mut lsmetaversion: uint64_t = 0;
        let mut lsstore: uint32_t = 0;
        let mut lstime: uint32_t = 0;
        let mut lsmetachecksum: uint32_t = 0;
        let mut lsstat: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_INFO - wrong size (%u/0)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        meta_info(
            &raw mut lsstore,
            &raw mut lstime,
            &raw mut lsstat,
            &raw mut lsmetaversion,
            &raw mut lsmetachecksum,
        );
        fs_info(
            &raw mut totalspace,
            &raw mut availspace,
            &raw mut freespace,
            &raw mut trspace,
            &raw mut trnodes,
            &raw mut respace,
            &raw mut renodes,
            &raw mut inodes,
            &raw mut dnodes,
            &raw mut fnodes,
        );
        chunk_info(
            &raw mut chunks,
            &raw mut copychunks,
            &raw mut ec8chunks,
            &raw mut ec4chunks,
            &raw mut chunkcopies,
            &raw mut chunkec8parts,
            &raw mut chunkec4parts,
            &raw mut chunkhypotheticalcopies,
        );
        chartsdata_resusage(&raw mut memusage, &raw mut syscpu, &raw mut usercpu);
        ptr = matoclserv_create_packet(eptr, MATOCL_INFO as uint32_t, 205 as uint32_t);
        put16bit(&raw mut ptr, VERSMAJ as uint16_t);
        put8bit(&raw mut ptr, VERSMID as uint8_t);
        put8bit(&raw mut ptr, VERSMIN as uint8_t);
        put64bit(&raw mut ptr, memusage);
        put64bit(&raw mut ptr, syscpu);
        put64bit(&raw mut ptr, usercpu);
        put64bit(&raw mut ptr, totalspace);
        put64bit(&raw mut ptr, availspace);
        put64bit(&raw mut ptr, freespace);
        put64bit(&raw mut ptr, trspace);
        put32bit(&raw mut ptr, trnodes);
        put64bit(&raw mut ptr, respace);
        put32bit(&raw mut ptr, renodes);
        put32bit(&raw mut ptr, inodes);
        put32bit(&raw mut ptr, dnodes);
        put32bit(&raw mut ptr, fnodes);
        put32bit(&raw mut ptr, chunks);
        put32bit(&raw mut ptr, copychunks);
        put32bit(&raw mut ptr, ec8chunks);
        put32bit(&raw mut ptr, ec4chunks);
        put32bit(&raw mut ptr, lsstore);
        put32bit(&raw mut ptr, lstime);
        put8bit(&raw mut ptr, lsstat);
        put8bit(&raw mut ptr, 0xff as uint8_t);
        put8bit(&raw mut ptr, 0xff as uint8_t);
        put8bit(&raw mut ptr, 0xff as uint8_t);
        put8bit(&raw mut ptr, 0xff as uint8_t);
        put32bit(&raw mut ptr, 0 as uint32_t);
        put32bit(&raw mut ptr, 0 as uint32_t);
        put64bit(&raw mut ptr, meta_version());
        put64bit(&raw mut ptr, exports_checksum());
        put64bit(&raw mut ptr, meta_get_id());
        put64bit(&raw mut ptr, lsmetaversion);
        put32bit(&raw mut ptr, lsmetachecksum);
        put64bit(&raw mut ptr, chunkcopies);
        put64bit(&raw mut ptr, chunkec8parts);
        put64bit(&raw mut ptr, chunkec4parts);
        put64bit(&raw mut ptr, chunkhypotheticalcopies);
        put64bit(&raw mut ptr, main_utime());
        put32bit(&raw mut ptr, 0 as uint32_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_memory_info(
    mut eptr: *mut matoclserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut allocated: [uint64_t; 8] = [0; 8];
        let mut used: [uint64_t; 8] = [0; 8];
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_MEMORY_INFO - wrong size (%u/0)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_MEMORY_INFO as uint32_t, 176 as uint32_t);
        chunk_get_memusage(
            &raw mut allocated as *mut uint64_t,
            &raw mut used as *mut uint64_t,
        );
        put64bit(&raw mut ptr, allocated[0 as usize]);
        put64bit(&raw mut ptr, used[0 as usize]);
        put64bit(&raw mut ptr, allocated[1 as usize]);
        put64bit(&raw mut ptr, used[1 as usize]);
        put64bit(&raw mut ptr, allocated[2 as usize]);
        put64bit(&raw mut ptr, used[2 as usize]);
        fs_get_memusage(
            &raw mut allocated as *mut uint64_t,
            &raw mut used as *mut uint64_t,
        );
        put64bit(&raw mut ptr, allocated[0 as usize]);
        put64bit(&raw mut ptr, used[0 as usize]);
        put64bit(&raw mut ptr, allocated[1 as usize]);
        put64bit(&raw mut ptr, used[1 as usize]);
        put64bit(&raw mut ptr, allocated[2 as usize]);
        put64bit(&raw mut ptr, used[2 as usize]);
        put64bit(&raw mut ptr, allocated[3 as usize]);
        put64bit(&raw mut ptr, used[3 as usize]);
        put64bit(&raw mut ptr, allocated[4 as usize]);
        put64bit(&raw mut ptr, used[4 as usize]);
        put64bit(&raw mut ptr, allocated[5 as usize]);
        put64bit(&raw mut ptr, used[5 as usize]);
        put64bit(&raw mut ptr, allocated[6 as usize]);
        put64bit(&raw mut ptr, used[6 as usize]);
        put64bit(&raw mut ptr, allocated[7 as usize]);
        put64bit(&raw mut ptr, used[7 as usize]);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fstest_info(
    mut eptr: *mut matoclserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut loopstart: uint32_t = 0;
        let mut loopend: uint32_t = 0;
        let mut files: uint32_t = 0;
        let mut ugfiles: uint32_t = 0;
        let mut mfiles: uint32_t = 0;
        let mut mtfiles: uint32_t = 0;
        let mut msfiles: uint32_t = 0;
        let mut chunks: uint32_t = 0;
        let mut ugchunks: uint32_t = 0;
        let mut mchunks: uint32_t = 0;
        let mut msgbuffleng: uint32_t = 0;
        let mut msgbuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 0 as uint32_t && length != 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FSTEST_INFO - wrong size (%u/0|1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        fs_test_getdata(
            &raw mut loopstart,
            &raw mut loopend,
            &raw mut files,
            &raw mut ugfiles,
            &raw mut mfiles,
            &raw mut mtfiles,
            &raw mut msfiles,
            &raw mut chunks,
            &raw mut ugchunks,
            &raw mut mchunks,
            &raw mut msgbuff,
            &raw mut msgbuffleng,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FSTEST_INFO as uint32_t,
            msgbuffleng.wrapping_add(
                (if length == 1 as uint32_t {
                    44 as ::core::ffi::c_int
                } else {
                    36 as ::core::ffi::c_int
                }) as uint32_t,
            ),
        );
        put32bit(&raw mut ptr, loopstart);
        put32bit(&raw mut ptr, loopend);
        put32bit(&raw mut ptr, files);
        put32bit(&raw mut ptr, ugfiles);
        put32bit(&raw mut ptr, mfiles);
        if length == 1 as uint32_t {
            put32bit(&raw mut ptr, mtfiles);
            put32bit(&raw mut ptr, msfiles);
        }
        put32bit(&raw mut ptr, chunks);
        put32bit(&raw mut ptr, ugchunks);
        put32bit(&raw mut ptr, mchunks);
        put32bit(&raw mut ptr, msgbuffleng);
        if msgbuffleng > 0 as uint32_t {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                msgbuff as *const ::core::ffi::c_void,
                msgbuffleng as size_t,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_chunkstest_info(
    mut eptr: *mut matoclserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_CHUNKSTEST_INFO - wrong size (%u/0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_CHUNKSTEST_INFO as uint32_t,
            chunk_store_info(::core::ptr::null_mut::<uint8_t>()),
        );
        chunk_store_info(ptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_chunks_matrix(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut mcnt: uint8_t = 0;
        let mut i: uint8_t = 0;
        if length > 2 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_CHUNKS_MATRIX - wrong size (%u/0|1|2)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 1 as uint32_t {
            let mut matrixid: uint8_t = 0;
            matrixid = get8bit(&raw mut data);
            ptr = matoclserv_create_packet(eptr, MATOCL_CHUNKS_MATRIX as uint32_t, 484 as uint32_t);
            chunk_store_chunkcounters(ptr, matrixid, 0 as int16_t);
        } else {
            let mut progressstatus: uint8_t = 0;
            let mut classid: int16_t = 0;
            mcnt = 8 as uint8_t;
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_CHUNKS_MATRIX as uint32_t,
                (1 as ::core::ffi::c_int + mcnt as ::core::ffi::c_int * 484 as ::core::ffi::c_int)
                    as uint32_t,
            );
            progressstatus = chunk_counters_in_progress();
            classid = -1 as int16_t;
            if length == 2 as uint32_t {
                data = data.offset(1 as ::core::ffi::c_int as isize);
                classid = get8bit(&raw mut data) as int16_t;
            }
            put8bit(&raw mut ptr, progressstatus);
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < mcnt as ::core::ffi::c_int {
                chunk_store_chunkcounters(
                    ptr.offset((i as ::core::ffi::c_int * 484 as ::core::ffi::c_int) as isize),
                    i,
                    classid,
                );
                i = i.wrapping_add(1);
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_quota_info(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut ver: uint8_t = 0;
        if length != 0 as uint32_t && length != 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_QUOTA_INFO - wrong size (%u/(0|1))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 0 as uint32_t {
            ver = 0 as uint8_t;
        } else {
            ver = get8bit(&raw mut data);
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_QUOTA_INFO as uint32_t,
            fs_getquotainfo(::core::ptr::null_mut::<uint8_t>(), ver),
        );
        fs_getquotainfo(ptr, ver);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_exports_info(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut vmode: uint8_t = 0;
        if length != 0 as uint32_t && length != 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_EXPORTS_INFO - wrong size (%u/0|1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 0 as uint32_t {
            vmode = 0 as uint8_t;
        } else {
            vmode = get8bit(&raw mut data);
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_EXPORTS_INFO as uint32_t,
            exports_info_size(vmode),
        );
        exports_info_data(vmode, ptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_mlog_list(
    mut eptr: *mut matoclserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_MLOG_LIST - wrong size (%u/0)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_MLOG_LIST as uint32_t,
            matomlserv_mloglist_size(),
        );
        matomlserv_mloglist_data(ptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_instance_name(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0 as uint32_t;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 0 as uint32_t && length != 4 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_INSTANCE_NAME - wrong size (%u/4|0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 4 as uint32_t {
            msgid = get32bit(&raw mut data);
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_INSTANCE_NAME as uint32_t,
                ((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add(InstanceNameLeng),
            );
            put32bit(&raw mut ptr, msgid);
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_INSTANCE_NAME as uint32_t,
                (1 as uint32_t).wrapping_add(InstanceNameLeng),
            );
        }
        put8bit(&raw mut ptr, InstanceNameLeng as uint8_t);
        if InstanceNameLeng > 0 as uint32_t {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                InstanceName as *const ::core::ffi::c_void,
                InstanceNameLeng as size_t,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_register(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut asize: uint32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut status: uint8_t = 0;
        if length < 64 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_REGISTER - wrong size (%u/<64)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if memcmp(
            data as *const ::core::ffi::c_void,
            FUSE_REGISTER_BLOB_ACL.as_ptr() as *const ::core::ffi::c_void,
            64 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            let mut expected_metaid: uint64_t = 0;
            let mut rootinode: uint32_t = 0;
            let mut sesflags: uint8_t = 0;
            let mut umaskval: uint16_t = 0;
            let mut sclassgroups: uint16_t = 0;
            let mut mintrashretention: uint32_t = 0;
            let mut maxtrashretention: uint32_t = 0;
            let mut disables: uint32_t = 0;
            let mut rootuid: uint32_t = 0;
            let mut rootgid: uint32_t = 0;
            let mut mapalluid: uint32_t = 0;
            let mut mapallgid: uint32_t = 0;
            let mut ileng: uint32_t = 0;
            let mut pleng: uint32_t = 0;
            let mut i: uint8_t = 0;
            let mut rcode: uint8_t = 0;
            let mut created: uint8_t = 0;
            if length < 65 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_REGISTER/ACL - wrong size (%u/<65)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            rptr = data.offset(64 as ::core::ffi::c_int as isize);
            rcode = get8bit(&raw mut rptr);
            if (*eptr).registered as ::core::ffi::c_int == NOTREGISTERED as ::core::ffi::c_int
                && rcode as ::core::ffi::c_int == REGISTER_CLOSESESSION
                || (*eptr).registered as ::core::ffi::c_int != NOTREGISTERED as ::core::ffi::c_int
                    && rcode as ::core::ffi::c_int != REGISTER_CLOSESESSION
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_REGISTER/ACL - wrong rcode (%d) for registered status (%d)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    rcode as ::core::ffi::c_int,
                    (*eptr).registered as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            match rcode as ::core::ffi::c_int {
                REGISTER_GETRANDOM => {
                    if length != 65 as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.1 - wrong size (%u/65)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            length,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    wptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_REGISTER as uint32_t,
                        32 as uint32_t,
                    );
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                        (*eptr).passwordrnd[i as usize] = rndu8();
                        i = i.wrapping_add(1);
                    }
                    memcpy(
                        wptr as *mut ::core::ffi::c_void,
                        &raw mut (*eptr).passwordrnd as *mut uint8_t as *const ::core::ffi::c_void,
                        32 as size_t,
                    );
                    return;
                }
                REGISTER_NEWSESSION => {
                    if length < 77 as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.2 - wrong size (%u/>=77)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            length,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    (*eptr).version = get32bit(&raw mut rptr);
                    (*eptr).asize = (if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                93 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                93 as ::core::ffi::c_int
                            })) as uint32_t
                        && (*eptr).version
                            != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                        && (*eptr).version
                            != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        ATTR_RECORD_SIZE
                    } else {
                        35 as ::core::ffi::c_int
                    }) as uint8_t;
                    status = MFS_STATUS_OK as uint8_t;
                    if sclass_ec_version() as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && (*eptr).version
                            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        if RestrictIncompatibleClientVersions != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CLTOMA_FUSE_REGISTER/ACL.2 - client (ip:%s) is too old - erasure coding needs clients at least 4.x\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                (*eptr).strip,
                            );
                            status = MFS_ERROR_EPERM as uint8_t;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"CLTOMA_FUSE_REGISTER/ACL.2 - old client registered - erasure coding needs clients at least 4.x - files in EC format will not be accessible\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    if sclass_ec_version() as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        && (*eptr).version
                            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 26 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        if RestrictIncompatibleClientVersions != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CLTOMA_FUSE_REGISTER/ACL.2 - client (ip:%s) is too old - erasure coding 4+n needs clients at least 4.26.x\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                (*eptr).strip,
                            );
                            status = MFS_ERROR_EPERM as uint8_t;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"CLTOMA_FUSE_REGISTER/ACL.2 - old client registered - erasure coding 4+n needs clients at least 4.26.x - files in EC4 format will not be accessible\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    ileng = get32bit(&raw mut rptr);
                    if ileng > MFS_PATH_MAX as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.2 - info too long (%u)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ileng,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    if length < (77 as uint32_t).wrapping_add(ileng) {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.2 - wrong size (%u/>=77+ileng(%u))\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            length,
                            ileng,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    if !(*eptr).info.is_null() {
                        free((*eptr).info as *mut ::core::ffi::c_void);
                    }
                    (*eptr).ileng = ileng;
                    (*eptr).info = malloc(ileng as size_t) as *mut uint8_t;
                    if (*eptr).info.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1769 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"eptr->info\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1769 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"eptr->info\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*eptr).info
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
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1769 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"eptr->info\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1769 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"eptr->info\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                    memcpy(
                        (*eptr).info as *mut ::core::ffi::c_void,
                        rptr as *const ::core::ffi::c_void,
                        ileng as size_t,
                    );
                    rptr = rptr.offset(ileng as isize);
                    pleng = get32bit(&raw mut rptr);
                    if pleng > MFS_PATH_MAX as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.2 - path too long (%u)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            pleng,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    if length != (77 as uint32_t).wrapping_add(ileng).wrapping_add(pleng)
                        && length
                            != ((77 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_add(ileng)
                                .wrapping_add(pleng)
                        && length
                            != ((77 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_add(ileng)
                                .wrapping_add(pleng)
                        && length
                            != ((77 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(ileng)
                                .wrapping_add(pleng)
                        && length
                            != ((77 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_add(ileng)
                                .wrapping_add(pleng)
                        && length
                            != ((77 as ::core::ffi::c_int
                                + 12 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(ileng)
                                .wrapping_add(pleng)
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.2 - wrong size (%u/77+ileng(%u)+pleng(%u)+[0|4|12]+[0|16])\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            length,
                            ileng,
                            pleng,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    if !(*eptr).path.is_null() {
                        free((*eptr).path as *mut ::core::ffi::c_void);
                    }
                    if pleng > 0 as uint32_t {
                        (*eptr).path = malloc(pleng as size_t) as *mut uint8_t;
                        if (*eptr).path.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1789 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"eptr->path\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1789 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"eptr->path\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if (*eptr).path
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
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1789 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"eptr->path\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1789 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"eptr->path\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            abort();
                        }
                        memcpy(
                            (*eptr).path as *mut ::core::ffi::c_void,
                            rptr as *const ::core::ffi::c_void,
                            pleng as size_t,
                        );
                        rptr = rptr.offset(pleng as isize);
                        if *rptr.offset(-1 as isize) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CLTOMA_FUSE_REGISTER/ACL.2 - received path without ending zero\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                            return;
                        }
                    } else {
                        (*eptr).path = malloc(1 as size_t) as *mut uint8_t;
                        if (*eptr).path.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1799 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"eptr->path\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1799 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"eptr->path\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if (*eptr).path
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
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1799 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"eptr->path\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_1,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1799 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"eptr->path\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_1,
                            );
                            abort();
                        }
                        *(*eptr).path.offset(0 as isize) = 0 as uint8_t;
                    }
                    if length
                        == ((77 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t)
                            .wrapping_add(ileng)
                            .wrapping_add(pleng)
                        || length
                            == ((77 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(ileng)
                                .wrapping_add(pleng)
                    {
                        sessionid = get32bit(&raw mut rptr);
                    } else if length
                        == ((77 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as uint32_t)
                            .wrapping_add(ileng)
                            .wrapping_add(pleng)
                        || length
                            == ((77 as ::core::ffi::c_int
                                + 12 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(ileng)
                                .wrapping_add(pleng)
                    {
                        sessionid = get32bit(&raw mut rptr);
                        expected_metaid = get64bit(&raw mut rptr);
                        if expected_metaid != meta_get_id() {
                            sessionid = 0 as uint32_t;
                        }
                    } else {
                        sessionid = iptosesid_get((*eptr).peerip);
                    }
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        if length
                            >= ((77 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_add(ileng)
                                .wrapping_add(pleng)
                        {
                            (*eptr).usepassword = 1 as uint8_t;
                            memcpy(
                                &raw mut (*eptr).passwordmd5 as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                rptr as *const ::core::ffi::c_void,
                                16 as size_t,
                            );
                            status = exports_check(
                                (*eptr).peerip,
                                (*eptr).version,
                                (*eptr).path,
                                &raw mut (*eptr).passwordrnd as *mut uint8_t as *const uint8_t,
                                rptr as *const uint8_t,
                                &raw mut sesflags,
                                &raw mut umaskval,
                                &raw mut rootuid,
                                &raw mut rootgid,
                                &raw mut mapalluid,
                                &raw mut mapallgid,
                                &raw mut sclassgroups,
                                &raw mut mintrashretention,
                                &raw mut maxtrashretention,
                                &raw mut disables,
                            );
                            if status as ::core::ffi::c_int == MFS_ERROR_BADPASSWORD {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_NOTICE,
                                    b"client from IP:%s attempted a connection with wrong password\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*eptr).strip,
                                );
                            }
                        } else {
                            (*eptr).usepassword = 0 as uint8_t;
                            status = exports_check(
                                (*eptr).peerip,
                                (*eptr).version,
                                (*eptr).path,
                                ::core::ptr::null::<uint8_t>(),
                                ::core::ptr::null::<uint8_t>(),
                                &raw mut sesflags,
                                &raw mut umaskval,
                                &raw mut rootuid,
                                &raw mut rootgid,
                                &raw mut mapalluid,
                                &raw mut mapallgid,
                                &raw mut sclassgroups,
                                &raw mut mintrashretention,
                                &raw mut maxtrashretention,
                                &raw mut disables,
                            );
                            if status as ::core::ffi::c_int == MFS_ERROR_NOPASSWORD {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_NOTICE,
                                    b"client from IP:%s attempted a connection without needed password\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*eptr).strip,
                                );
                            }
                        }
                    }
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        status = fs_getrootinode(&raw mut rootinode, (*eptr).path);
                    }
                    created = 0 as uint8_t;
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        if sessionid != 0 as uint32_t {
                            (*eptr).sesdata = sessions_find_session(sessionid);
                            if (*eptr).sesdata.is_null() {
                                sessionid = 0 as uint32_t;
                            } else {
                                sessionid = sessions_chg_session(
                                    (*eptr).sesdata,
                                    exports_checksum(),
                                    rootinode,
                                    sesflags,
                                    umaskval,
                                    rootuid,
                                    rootgid,
                                    mapalluid,
                                    mapallgid,
                                    sclassgroups,
                                    mintrashretention,
                                    maxtrashretention,
                                    disables,
                                    (*eptr).peerip,
                                    (*eptr).info,
                                    (*eptr).ileng,
                                );
                            }
                        }
                        if sessionid == 0 as uint32_t {
                            (*eptr).sesdata = sessions_new_session(
                                exports_checksum(),
                                rootinode,
                                sesflags,
                                umaskval,
                                rootuid,
                                rootgid,
                                mapalluid,
                                mapallgid,
                                sclassgroups,
                                mintrashretention,
                                maxtrashretention,
                                disables,
                                (*eptr).peerip,
                                (*eptr).info,
                                (*eptr).ileng,
                            );
                            created = 1 as uint8_t;
                        }
                        if (*eptr).sesdata.is_null() {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"can't allocate session record\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                            return;
                        }
                    }
                    asize = 1 as uint32_t;
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        if (*eptr).version
                            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 57 as uint32_t;
                        } else if (*eptr).version
                            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 21 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                            || (*eptr).version
                                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                        112 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                    } else {
                                        112 as ::core::ffi::c_int
                                    })) as uint32_t
                                && (*eptr).version
                                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                        } else {
                                            0 as ::core::ffi::c_int
                                        })) as uint32_t
                        {
                            asize = 49 as uint32_t;
                        } else if (*eptr).version
                            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    72 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    72 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 45 as uint32_t;
                        } else if (*eptr).version
                            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    11 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 43 as uint32_t;
                        } else if (*eptr).version
                            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    26 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    26 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 35 as uint32_t;
                        } else if (*eptr).version
                            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    21 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    21 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 25 as uint32_t;
                        } else if (*eptr).version
                            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 21 as uint32_t;
                        } else {
                            asize = 13 as uint32_t;
                        }
                    }
                    wptr = matoclserv_create_packet(eptr, MATOCL_FUSE_REGISTER as uint32_t, asize);
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        put8bit(&raw mut wptr, status);
                        (*eptr).sesdata = NULL;
                        return;
                    }
                    sessionid = sessions_get_id((*eptr).sesdata);
                    if (*eptr).version
                        == (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                21 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                21 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put32bit(&raw mut wptr, 0 as uint32_t);
                    } else if (*eptr).version
                        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                22 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                22 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put16bit(&raw mut wptr, VERSMAJ as uint16_t);
                        put8bit(&raw mut wptr, VERSMID as uint8_t);
                        put8bit(&raw mut wptr, VERSMIN as uint8_t);
                    }
                    put32bit(&raw mut wptr, sessionid);
                    if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                11 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put64bit(&raw mut wptr, meta_get_id());
                    }
                    put8bit(&raw mut wptr, sesflags);
                    if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                72 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                72 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put16bit(&raw mut wptr, umaskval);
                    }
                    put32bit(&raw mut wptr, rootuid);
                    put32bit(&raw mut wptr, rootgid);
                    if (*eptr).version
                        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put32bit(&raw mut wptr, mapalluid);
                        put32bit(&raw mut wptr, mapallgid);
                    }
                    if (*eptr).version
                        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                26 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                26 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        if (*eptr).version
                            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 57 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            put16bit(&raw mut wptr, sclassgroups);
                        } else {
                            put8bit(&raw mut wptr, 1 as uint8_t);
                            put8bit(&raw mut wptr, 9 as uint8_t);
                        }
                        put32bit(&raw mut wptr, mintrashretention);
                        put32bit(&raw mut wptr, maxtrashretention);
                    }
                    if (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 21 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                        || (*eptr).version
                            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    112 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    112 as ::core::ffi::c_int
                                })) as uint32_t
                            && (*eptr).version
                                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })) as uint32_t
                    {
                        put32bit(&raw mut wptr, disables);
                    }
                    if (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put64bit(&raw mut wptr, master_processid);
                    }
                    sessions_attach_session((*eptr).sesdata, (*eptr).peerip, (*eptr).version);
                    (*eptr).registered = REGISTERED as ::core::ffi::c_int as uint8_t;
                    if created != 0 {
                        if sessionid > 0 as uint32_t && sessionid < 0x80000000 as uint32_t {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_INFO,
                                b"created new sessionid:%u (client ip:%s)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                sessionid,
                                (*eptr).strip,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_INFO,
                                b"created temporary session (id:TMP/%u ; client ip:%s)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                sessionid & 0x7fffffff as uint32_t,
                                (*eptr).strip,
                            );
                        }
                    }
                    return;
                }
                REGISTER_NEWMETASESSION => {
                    if length < 73 as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.5 - wrong size (%u/>=73)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            length,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    (*eptr).version = get32bit(&raw mut rptr);
                    (*eptr).asize = (if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                93 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                93 as ::core::ffi::c_int
                            })) as uint32_t
                        && (*eptr).version
                            != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                        && (*eptr).version
                            != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        ATTR_RECORD_SIZE
                    } else {
                        35 as ::core::ffi::c_int
                    }) as uint8_t;
                    status = MFS_STATUS_OK as uint8_t;
                    if sclass_ec_version() as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && (*eptr).version
                            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        if RestrictIncompatibleClientVersions != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CLTOMA_FUSE_REGISTER/ACL.5 - client (ip:%s) is too old - erasure coding needs clients at least 4.x\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                (*eptr).strip,
                            );
                            status = MFS_ERROR_EPERM as uint8_t;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"CLTOMA_FUSE_REGISTER/ACL.5 - old client registered - erasure coding needs clients at least 4.x - files in EC format will not be accessible\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    if sclass_ec_version() as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        && (*eptr).version
                            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 26 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        if RestrictIncompatibleClientVersions != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CLTOMA_FUSE_REGISTER/ACL.5 - client (ip:%s) is too old - erasure coding 4+n needs clients at least 4.26.x\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                (*eptr).strip,
                            );
                            status = MFS_ERROR_EPERM as uint8_t;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"CLTOMA_FUSE_REGISTER/ACL.5 - old client registered - erasure coding 4+n needs clients at least 4.26.x - files in EC4 format will not be accessible\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    ileng = get32bit(&raw mut rptr);
                    if ileng > MFS_PATH_MAX as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.5 - info too long (%u)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ileng,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    if length != (73 as uint32_t).wrapping_add(ileng)
                        && length
                            != ((73 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_add(ileng)
                        && length
                            != ((73 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_add(ileng)
                        && length
                            != ((73 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(ileng)
                        && length
                            != ((73 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_add(ileng)
                        && length
                            != ((73 as ::core::ffi::c_int
                                + 12 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(ileng)
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.5 - wrong size (%u/73+ileng(%u)+[0|4|12]+[0|16])\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            length,
                            ileng,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    if !(*eptr).info.is_null() {
                        free((*eptr).info as *mut ::core::ffi::c_void);
                    }
                    (*eptr).ileng = ileng;
                    (*eptr).info = malloc(ileng as size_t) as *mut uint8_t;
                    if (*eptr).info.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1970 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"eptr->info\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1970 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"eptr->info\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*eptr).info
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
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1970 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"eptr->info\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1970 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"eptr->info\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_2,
                        );
                        abort();
                    }
                    memcpy(
                        (*eptr).info as *mut ::core::ffi::c_void,
                        rptr as *const ::core::ffi::c_void,
                        ileng as size_t,
                    );
                    rptr = rptr.offset(ileng as isize);
                    if !(*eptr).path.is_null() {
                        free((*eptr).path as *mut ::core::ffi::c_void);
                        (*eptr).path = ::core::ptr::null_mut::<uint8_t>();
                    }
                    if length
                        == ((73 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t)
                            .wrapping_add(ileng)
                        || length
                            == ((73 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(ileng)
                    {
                        sessionid = get32bit(&raw mut rptr);
                    } else if length
                        == ((73 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as uint32_t)
                            .wrapping_add(ileng)
                        || length
                            == ((73 as ::core::ffi::c_int
                                + 12 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int)
                                as uint32_t)
                                .wrapping_add(ileng)
                    {
                        sessionid = get32bit(&raw mut rptr);
                        expected_metaid = get64bit(&raw mut rptr);
                        if expected_metaid != meta_get_id() {
                            sessionid = 0 as uint32_t;
                        }
                    } else {
                        sessionid = iptosesid_get((*eptr).peerip);
                    }
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        if length
                            >= ((73 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_add(ileng)
                        {
                            (*eptr).usepassword = 1 as uint8_t;
                            memcpy(
                                &raw mut (*eptr).passwordmd5 as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                rptr as *const ::core::ffi::c_void,
                                16 as size_t,
                            );
                            status = exports_check(
                                (*eptr).peerip,
                                (*eptr).version,
                                ::core::ptr::null::<uint8_t>(),
                                &raw mut (*eptr).passwordrnd as *mut uint8_t as *const uint8_t,
                                rptr as *const uint8_t,
                                &raw mut sesflags,
                                &raw mut umaskval,
                                &raw mut rootuid,
                                &raw mut rootgid,
                                &raw mut mapalluid,
                                &raw mut mapallgid,
                                &raw mut sclassgroups,
                                &raw mut mintrashretention,
                                &raw mut maxtrashretention,
                                &raw mut disables,
                            );
                            if status as ::core::ffi::c_int == MFS_ERROR_BADPASSWORD {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_NOTICE,
                                    b"client from IP:%s attempted a connection with wrong password\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*eptr).strip,
                                );
                            }
                        } else {
                            (*eptr).usepassword = 0 as uint8_t;
                            status = exports_check(
                                (*eptr).peerip,
                                (*eptr).version,
                                ::core::ptr::null::<uint8_t>(),
                                ::core::ptr::null::<uint8_t>(),
                                ::core::ptr::null::<uint8_t>(),
                                &raw mut sesflags,
                                &raw mut umaskval,
                                &raw mut rootuid,
                                &raw mut rootgid,
                                &raw mut mapalluid,
                                &raw mut mapallgid,
                                &raw mut sclassgroups,
                                &raw mut mintrashretention,
                                &raw mut maxtrashretention,
                                &raw mut disables,
                            );
                            if status as ::core::ffi::c_int == MFS_ERROR_NOPASSWORD {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_NOTICE,
                                    b"client from IP:%s attempted a connection without needed password\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*eptr).strip,
                                );
                            }
                        }
                    }
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        if sessionid != 0 as uint32_t {
                            (*eptr).sesdata = sessions_find_session(sessionid);
                            if (*eptr).sesdata.is_null() {
                                sessionid = 0 as uint32_t;
                            } else {
                                sessionid = sessions_chg_session(
                                    (*eptr).sesdata,
                                    exports_checksum(),
                                    0 as uint32_t,
                                    sesflags,
                                    umaskval,
                                    0 as uint32_t,
                                    0 as uint32_t,
                                    0 as uint32_t,
                                    0 as uint32_t,
                                    sclassgroups,
                                    mintrashretention,
                                    maxtrashretention,
                                    disables,
                                    (*eptr).peerip,
                                    (*eptr).info,
                                    (*eptr).ileng,
                                );
                            }
                        }
                        if sessionid == 0 as uint32_t {
                            (*eptr).sesdata = sessions_new_session(
                                exports_checksum(),
                                0 as uint32_t,
                                sesflags,
                                umaskval,
                                0 as uint32_t,
                                0 as uint32_t,
                                0 as uint32_t,
                                0 as uint32_t,
                                sclassgroups,
                                mintrashretention,
                                maxtrashretention,
                                disables,
                                (*eptr).peerip,
                                (*eptr).info,
                                (*eptr).ileng,
                            );
                        }
                        if (*eptr).sesdata.is_null() {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"can't allocate session record\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                            return;
                        }
                    }
                    asize = 1 as uint32_t;
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        if (*eptr).version
                            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 35 as uint32_t;
                        } else if (*eptr).version
                            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    11 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 27 as uint32_t;
                        } else if (*eptr).version
                            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    26 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    26 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 19 as uint32_t;
                        } else if (*eptr).version
                            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    21 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    21 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            asize = 9 as uint32_t;
                        } else {
                            asize = 5 as uint32_t;
                        }
                    }
                    wptr = matoclserv_create_packet(eptr, MATOCL_FUSE_REGISTER as uint32_t, asize);
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        put8bit(&raw mut wptr, status);
                        (*eptr).sesdata = NULL;
                        return;
                    }
                    sessionid = sessions_get_id((*eptr).sesdata);
                    if (*eptr).version
                        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                21 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                21 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put16bit(&raw mut wptr, VERSMAJ as uint16_t);
                        put8bit(&raw mut wptr, VERSMID as uint8_t);
                        put8bit(&raw mut wptr, VERSMIN as uint8_t);
                    }
                    put32bit(&raw mut wptr, sessionid);
                    if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                11 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put64bit(&raw mut wptr, meta_get_id());
                    }
                    put8bit(&raw mut wptr, sesflags);
                    if (*eptr).version
                        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                26 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                26 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        if (*eptr).version
                            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 57 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            put16bit(&raw mut wptr, sclassgroups);
                        } else {
                            put8bit(&raw mut wptr, 1 as uint8_t);
                            put8bit(&raw mut wptr, 9 as uint8_t);
                        }
                        put32bit(&raw mut wptr, mintrashretention);
                        put32bit(&raw mut wptr, maxtrashretention);
                    }
                    if (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put64bit(&raw mut wptr, master_processid);
                    }
                    sessions_attach_session((*eptr).sesdata, (*eptr).peerip, (*eptr).version);
                    (*eptr).registered = REGISTERED as ::core::ffi::c_int as uint8_t;
                    return;
                }
                REGISTER_RECONNECT => {
                    if length < 73 as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.%hhu - wrong size (%u/73)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            rcode as ::core::ffi::c_int,
                            length,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    sessionid = get32bit(&raw mut rptr);
                    if iptosesid_check((*eptr).peerip) != 0 {
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    (*eptr).version = get32bit(&raw mut rptr);
                    (*eptr).asize = (if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                93 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                93 as ::core::ffi::c_int
                            })) as uint32_t
                        && (*eptr).version
                            != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                        && (*eptr).version
                            != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        ATTR_RECORD_SIZE
                    } else {
                        35 as ::core::ffi::c_int
                    }) as uint8_t;
                    status = MFS_STATUS_OK as uint8_t;
                    if sclass_ec_version() as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                        && (*eptr).version
                            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        if RestrictIncompatibleClientVersions != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CLTOMA_FUSE_REGISTER/ACL.%hhu - client (ip:%s) is too old - erasure coding needs clients at least 4.x\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                rcode as ::core::ffi::c_int,
                                (*eptr).strip,
                            );
                            status = MFS_ERROR_EPERM as uint8_t;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"CLTOMA_FUSE_REGISTER/ACL.%hhu - old client registered - erasure coding needs clients at least 4.x - files in EC format will not be accessible\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                rcode as ::core::ffi::c_int,
                            );
                        }
                    }
                    if sclass_ec_version() as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        && (*eptr).version
                            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 26 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        if RestrictIncompatibleClientVersions != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CLTOMA_FUSE_REGISTER/ACL.%hhu - client (ip:%s) is too old - erasure coding 4+n needs clients at least 4.26.x\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                rcode as ::core::ffi::c_int,
                                (*eptr).strip,
                            );
                            status = MFS_ERROR_EPERM as uint8_t;
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"CLTOMA_FUSE_REGISTER/ACL.%hhu - old client registered - erasure coding 4+n needs clients at least 4.26.x - files in EC4 format will not be accessible\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                rcode as ::core::ffi::c_int,
                            );
                        }
                    }
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        if length >= 81 as uint32_t {
                            expected_metaid = get64bit(&raw mut rptr);
                            if expected_metaid != meta_get_id() {
                                status = MFS_ERROR_BADSESSIONID as uint8_t;
                            }
                        }
                    }
                    if status as ::core::ffi::c_int == MFS_STATUS_OK {
                        (*eptr).sesdata = sessions_find_session(sessionid);
                        if (*eptr).sesdata.is_null()
                            || sessions_get_peerip((*eptr).sesdata) == 0 as uint32_t
                        {
                            status = MFS_ERROR_BADSESSIONID as uint8_t;
                        } else if sessions_get_exportscsum((*eptr).sesdata) != exports_checksum()
                            || sessions_get_sesflags((*eptr).sesdata)
                                & SESFLAG_DYNAMICIP as uint32_t
                                == 0 as uint32_t
                                && (*eptr).peerip != sessions_get_peerip((*eptr).sesdata)
                        {
                            status = MFS_ERROR_EPERM as uint8_t;
                            iptosesid_add((*eptr).peerip, sessionid);
                        }
                    }
                    if rcode as ::core::ffi::c_int == REGISTER_RECONNECT
                        && ((*eptr).version
                            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    95 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    95 as ::core::ffi::c_int
                                })) as uint32_t
                            && (*eptr).version
                                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })) as uint32_t
                            || (*eptr).version
                                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                        7 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                    } else {
                                        7 as ::core::ffi::c_int
                                    })) as uint32_t)
                    {
                        if (*eptr).version
                            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t
                        {
                            wptr = matoclserv_create_packet(
                                eptr,
                                MATOCL_FUSE_REGISTER as uint32_t,
                                13 as uint32_t,
                            );
                            put16bit(&raw mut wptr, VERSMAJ as uint16_t);
                            put8bit(&raw mut wptr, VERSMID as uint8_t);
                            put8bit(&raw mut wptr, VERSMIN as uint8_t);
                            put64bit(&raw mut wptr, master_processid);
                        } else {
                            wptr = matoclserv_create_packet(
                                eptr,
                                MATOCL_FUSE_REGISTER as uint32_t,
                                5 as uint32_t,
                            );
                            put16bit(&raw mut wptr, VERSMAJ as uint16_t);
                            put8bit(&raw mut wptr, VERSMID as uint8_t);
                            put8bit(&raw mut wptr, VERSMIN as uint8_t);
                        }
                    } else {
                        wptr = matoclserv_create_packet(
                            eptr,
                            MATOCL_FUSE_REGISTER as uint32_t,
                            1 as uint32_t,
                        );
                    }
                    put8bit(&raw mut wptr, status);
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        (*eptr).sesdata = NULL;
                        return;
                    }
                    sessions_attach_session((*eptr).sesdata, (*eptr).peerip, (*eptr).version);
                    (*eptr).registered = REGISTERED as ::core::ffi::c_int as uint8_t;
                    return;
                }
                REGISTER_CLOSESESSION => {
                    if length < 69 as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_FUSE_REGISTER/ACL.6 - wrong size (%u/69)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            length,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    sessionid = get32bit(&raw mut rptr);
                    status = MFS_STATUS_OK as uint8_t;
                    if sessionid > 0 as uint32_t && sessionid < 0x80000000 as uint32_t {
                        if sessions_get_id((*eptr).sesdata) != sessionid {
                            status = MFS_ERROR_BADSESSIONID as uint8_t;
                        } else if length >= 77 as uint32_t {
                            expected_metaid = get64bit(&raw mut rptr);
                            if expected_metaid != meta_get_id() {
                                status = MFS_ERROR_BADSESSIONID as uint8_t;
                            }
                        }
                        if status as ::core::ffi::c_int == MFS_STATUS_OK {
                            sessions_close_session((*eptr).sesdata);
                        }
                    }
                    if (*eptr).version
                        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                29 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                29 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        wptr = matoclserv_create_packet(
                            eptr,
                            MATOCL_FUSE_REGISTER as uint32_t,
                            1 as uint32_t,
                        );
                        put8bit(&raw mut wptr, status);
                    }
                    (*eptr).mode = FINISH as ::core::ffi::c_int as uint8_t;
                    return;
                }
                _ => {}
            }
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_REGISTER/ACL - wrong rcode (%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rcode as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_REGISTER - wrong register blob\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_reload_sessions() {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        exports_reload();
        eptr = matoclservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*eptr).registered as ::core::ffi::c_int != NOTREGISTERED as ::core::ffi::c_int
                && !(*eptr).sesdata.is_null()
            {
                if sessions_get_exportscsum((*eptr).sesdata) != exports_checksum() {
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                }
            }
            eptr = (*eptr).next as *mut matoclserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_broadcast_timeout() {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if ForceTimeout > 0 as uint32_t {
            eptr = matoclservhead;
            while !eptr.is_null() {
                if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                    && (*eptr).registered as ::core::ffi::c_int
                        != NOTREGISTERED as ::core::ffi::c_int
                    && (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 12 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    (*eptr).timeout = ForceTimeout as uint16_t;
                    data = matoclserv_create_packet(
                        eptr,
                        ANTOAN_FORCE_TIMEOUT as uint32_t,
                        6 as uint32_t,
                    );
                    put32bit(&raw mut data, 0 as uint32_t);
                    put16bit(&raw mut data, ForceTimeout as uint16_t);
                }
                eptr = (*eptr).next as *mut matoclserventry;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_sustained_inodes(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        static mut inodetab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut inodetabsize: uint32_t = 0 as uint32_t;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        if length & 0x3 as uint32_t != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SUSTAINED_INODES - wrong size (%u/N*4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).sesdata.is_null() {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SUSTAINED_INODES - session doesn't exist\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        length >>= 2 as ::core::ffi::c_int;
        if length > inodetabsize {
            if !inodetab.is_null() {
                free(inodetab as *mut ::core::ffi::c_void);
            }
            inodetabsize = length.wrapping_add(0xff as uint32_t) & 0xffffff00 as uint32_t;
            inodetab =
                malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(inodetabsize as size_t))
                    as *mut uint32_t;
            if inodetab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2242 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2242 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if inodetab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2242 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2242 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        rptr = data;
        j = 0 as uint32_t;
        while length > 0 as uint32_t {
            i = get32bit(&raw mut rptr);
            if i > 0 as uint32_t {
                *inodetab.offset(j as isize) = i;
                j = j.wrapping_add(1);
            }
            length = length.wrapping_sub(1);
        }
        of_sync(sessions_get_id((*eptr).sesdata), inodetab, j);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_amtime_inodes(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        static mut inodetab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut atimetab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut mtimetab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut tabsizes: uint32_t = 0 as uint32_t;
        let mut i: uint32_t = 0;
        let mut a: uint32_t = 0;
        let mut m: uint32_t = 0;
        let mut j: uint32_t = 0;
        if length.wrapping_rem(12 as uint32_t) != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_AMTIME_INODES - wrong size (%u/N*12)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).sesdata.is_null() {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_AMTIME_INODES - session doesn't exist\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        length = length.wrapping_div(12 as uint32_t);
        if length > tabsizes {
            if !inodetab.is_null() {
                free(inodetab as *mut ::core::ffi::c_void);
            }
            if !atimetab.is_null() {
                free(atimetab as *mut ::core::ffi::c_void);
            }
            if !mtimetab.is_null() {
                free(mtimetab as *mut ::core::ffi::c_void);
            }
            tabsizes = length.wrapping_add(0xff as uint32_t) & 0xffffff00 as uint32_t;
            inodetab = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(tabsizes as size_t))
                as *mut uint32_t;
            if inodetab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if inodetab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"inodetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            atimetab = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(tabsizes as size_t))
                as *mut uint32_t;
            if atimetab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"atimetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"atimetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if atimetab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"atimetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"atimetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            mtimetab = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(tabsizes as size_t))
                as *mut uint32_t;
            if mtimetab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"mtimetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"mtimetab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if mtimetab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"mtimetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"mtimetab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
        }
        rptr = data;
        j = 0 as uint32_t;
        while length > 0 as uint32_t {
            i = get32bit(&raw mut rptr);
            a = get32bit(&raw mut rptr);
            m = get32bit(&raw mut rptr);
            if i > 0 as uint32_t && (a > 0 as uint32_t || m > 0 as uint32_t) {
                *inodetab.offset(j as isize) = i;
                *atimetab.offset(j as isize) = a;
                *mtimetab.offset(j as isize) = m;
                j = j.wrapping_add(1);
            }
            length = length.wrapping_sub(1);
        }
        fs_amtime_update(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inodetab,
            atimetab,
            mtimetab,
            j,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_opdata(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut rcnt: uint32_t = 0;
        let mut wcnt: uint32_t = 0;
        let mut fcnt: uint32_t = 0;
        let mut rbyt: uint64_t = 0;
        let mut wbyt: uint64_t = 0;
        let mut rcvdbyt: uint64_t = 0;
        let mut sentbyt: uint64_t = 0;
        if length != 28 as uint32_t && length != 44 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_OPDATA - wrong size (%u/28|44)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).sesdata.is_null() {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_OPDATA - session doesn't exist\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        rptr = data;
        rbyt = get64bit(&raw mut rptr);
        wbyt = get64bit(&raw mut rptr);
        rcnt = get32bit(&raw mut rptr);
        wcnt = get32bit(&raw mut rptr);
        fcnt = get32bit(&raw mut rptr);
        if length == 44 as uint32_t {
            rcvdbyt = get64bit(&raw mut rptr);
            sentbyt = get64bit(&raw mut rptr);
        } else {
            rcvdbyt = 0 as uint64_t;
            sentbyt = 0 as uint64_t;
        }
        stats_mounts_bread = stats_mounts_bread.wrapping_add(rbyt);
        stats_mounts_bwrite = stats_mounts_bwrite.wrapping_add(wbyt);
        stats_mounts_rcnt = stats_mounts_rcnt.wrapping_add(rcnt);
        stats_mounts_wcnt = stats_mounts_wcnt.wrapping_add(wcnt);
        stats_mounts_fcnt = stats_mounts_fcnt.wrapping_add(fcnt);
        stats_mounts_brcvd = stats_mounts_brcvd.wrapping_add(rcvdbyt);
        stats_mounts_bsent = stats_mounts_bsent.wrapping_add(sentbyt);
        sessions_add_stats((*eptr).sesdata, SES_OP_READ as uint8_t, rcnt as uint64_t);
        sessions_add_stats((*eptr).sesdata, SES_OP_WRITE as uint8_t, wcnt as uint64_t);
        sessions_add_stats((*eptr).sesdata, SES_OP_FSYNC as uint8_t, fcnt as uint64_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_wflags(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if length != 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_FLAGS - wrong size (%u/1)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        (*eptr).working_flags = *data.offset(0 as isize);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_time_sync(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut msgid: uint32_t = 0;
        if length != 0 as uint32_t && length != 4 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_TIME_SYNC - wrong size (%u/0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 4 as uint32_t {
            msgid = get32bit(&raw mut data);
        } else {
            msgid = 0 as uint32_t;
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_TIME_SYNC as uint32_t,
            (8 as uint32_t).wrapping_add(length),
        );
        if length == 4 as uint32_t {
            put32bit(&raw mut ptr, msgid);
        }
        put64bit(&raw mut ptr, main_utime());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_gid_storage(mut gids: uint32_t) -> *mut uint32_t {
    unsafe {
        static mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut gidleng: uint32_t = 0 as uint32_t;
        if gids == 0 as uint32_t {
            if !gid.is_null() {
                free(gid as *mut ::core::ffi::c_void);
            }
            gidleng = 0 as uint32_t;
            return ::core::ptr::null_mut::<uint32_t>();
        } else {
            if gidleng < gids {
                gidleng = gids.wrapping_add(255 as uint32_t) & 0xffffff00 as uint32_t;
                if !gid.is_null() {
                    free(gid as *mut ::core::ffi::c_void);
                }
                gid = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(gidleng as size_t))
                    as *mut uint32_t;
                if gid.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"gid\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2404 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"gid\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2404 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"gid\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            return gid;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_path_lookup(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pleng: uint32_t = 0;
        let mut base_inode: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut parent_inode: uint32_t = 0;
        let mut last_inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut name: [uint8_t; 255] = [0; 255];
        let mut nleng: uint8_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 20 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATH_LOOKUP - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        base_inode = get32bit(&raw mut data);
        pleng = get32bit(&raw mut data);
        if length < (20 as uint32_t).wrapping_add(pleng) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATH_LOOKUP - wrong size (%u:pleng=%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                pleng,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        path = data;
        data = data.offset(pleng as isize);
        uid = get32bit(&raw mut data);
        auid = uid;
        gids = get32bit(&raw mut data);
        if gids == 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATH_LOOKUP - group ids missing\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if gids > MFS_GIDS_MAX as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATH_LOOKUP - too many group ids\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length
            != (20 as uint32_t)
                .wrapping_add(pleng)
                .wrapping_add((4 as uint32_t).wrapping_mul(gids))
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATH_LOOKUP - wrong size (%u:pleng=%u:gids=%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                pleng,
                gids,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        gid = matoclserv_gid_storage(gids);
        i = 0 as uint32_t;
        while i < gids {
            *gid.offset(i as isize) = get32bit(&raw mut data);
            i = i.wrapping_add(1);
        }
        agid = *gid.offset(0 as isize);
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        status = fs_path_lookup(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            base_inode,
            pleng,
            path,
            uid,
            gids,
            gid,
            auid,
            agid,
            &raw mut parent_inode,
            &raw mut last_inode,
            &raw mut nleng,
            &raw mut name as *mut uint8_t,
            &raw mut attr as *mut uint8_t,
        );
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_PATH_LOOKUP as uint32_t,
                ((*eptr).asize as ::core::ffi::c_int
                    + 12 as ::core::ffi::c_int
                    + nleng as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
            put32bit(&raw mut ptr, parent_inode);
            put8bit(&raw mut ptr, nleng);
            if nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut name as *mut uint8_t as *const ::core::ffi::c_void,
                    nleng as size_t,
                );
                ptr = ptr.offset(nleng as ::core::ffi::c_int as isize);
            }
            put32bit(&raw mut ptr, last_inode);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_PATH_LOOKUP as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_statfs(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut totalspace: uint64_t = 0;
        let mut availspace: uint64_t = 0;
        let mut freespace: uint64_t = 0;
        let mut trashspace: uint64_t = 0;
        let mut sustainedspace: uint64_t = 0;
        let mut msgid: uint32_t = 0;
        let mut inodes: uint32_t = 0;
        let mut addfreespace: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 4 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_STATFS - wrong size (%u/4)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        addfreespace = (if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    102 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    102 as ::core::ffi::c_int
                })) as uint32_t
            && (*eptr).version
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            || (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 9 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        msgid = get32bit(&raw mut data);
        fs_statfs(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            &raw mut totalspace,
            &raw mut availspace,
            &raw mut freespace,
            &raw mut trashspace,
            &raw mut sustainedspace,
            &raw mut inodes,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_STATFS as uint32_t,
            (if addfreespace as ::core::ffi::c_int != 0 {
                48 as ::core::ffi::c_int
            } else {
                40 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        put64bit(&raw mut ptr, totalspace);
        put64bit(&raw mut ptr, availspace);
        if addfreespace != 0 {
            put64bit(&raw mut ptr, freespace);
        }
        put64bit(&raw mut ptr, trashspace);
        put64bit(&raw mut ptr, sustainedspace);
        put32bit(&raw mut ptr, inodes);
        sessions_inc_stats((*eptr).sesdata, SES_OP_STATFS as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_access(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut modemask: uint16_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length & 1 as uint32_t == 1 as uint32_t {
            if length != 17 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_ACCESS - wrong size (%u/17)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            msgid = get32bit(&raw mut data);
            inode = get32bit(&raw mut data);
            uid = get32bit(&raw mut data);
            gid = matoclserv_gid_storage(1 as uint32_t);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            gids = 1 as uint32_t;
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
            modemask = get8bit(&raw mut data) as uint16_t;
        } else {
            if length < 18 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_ACCESS - wrong size (%u/18+4*N)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            msgid = get32bit(&raw mut data);
            inode = get32bit(&raw mut data);
            uid = get32bit(&raw mut data);
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_ACCESS - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_ACCESS - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length != (18 as uint32_t).wrapping_add(gids.wrapping_mul(4 as uint32_t)) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_ACCESS - wrong size (%u/18+4*N)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
            modemask = get16bit(&raw mut data);
        }
        status = fs_access(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            uid,
            gids,
            gid,
            modemask as ::core::ffi::c_int,
        );
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_ACCESS as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_lookup(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut newinode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut lflags: uint16_t = 0;
        let mut accmode: uint16_t = 0;
        let mut filenode: uint8_t = 0;
        let mut validchunk: uint8_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 17 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_LOOKUP - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length < (17 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_LOOKUP - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name = data;
        data = data.offset(nleng as ::core::ffi::c_int as isize);
        uid = get32bit(&raw mut data);
        auid = uid;
        if length == (17 as uint32_t).wrapping_add(nleng as uint32_t) {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_LOOKUP - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_LOOKUP - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (17 as uint32_t)
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_LOOKUP - wrong size (%u:nleng=%hhu:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        }
        if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    40 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    40 as ::core::ffi::c_int
                })) as uint32_t
        {
            let mut sesflags: uint8_t = sessions_get_sesflags((*eptr).sesdata) as uint8_t;
            status = fs_lookup(
                sessions_get_rootinode((*eptr).sesdata),
                sesflags,
                inode,
                nleng as uint16_t,
                name,
                uid,
                gids,
                gid,
                auid,
                agid,
                &raw mut newinode,
                &raw mut attr as *mut uint8_t,
                1 as uint8_t,
                &raw mut accmode,
                &raw mut filenode,
                &raw mut validchunk,
                &raw mut chunkid,
            );
            if status as ::core::ffi::c_int == MFS_STATUS_OK {
                let mut version: uint32_t = 0;
                let mut split: uint8_t = 0;
                let mut count: uint8_t = 0;
                let mut cs_data: [uint8_t; 1400] = [0; 1400];
                let mut knowflags: uint8_t = (if (*eptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            113 as ::core::ffi::c_int
                        })) as uint32_t
                    && (*eptr).version
                        < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    || (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
                lflags = (accmode as ::core::ffi::c_int & LOOKUP_ACCESS_BITS) as uint16_t;
                split = 0 as uint8_t;
                count = 0 as uint8_t;
                version = 0 as uint32_t;
                if knowflags as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && lflags as ::core::ffi::c_int & LOOKUP_APPENDONLY != 0
                {
                    lflags = (lflags as ::core::ffi::c_int & LOOKUP_ACCESS_MODES_RO) as uint16_t;
                }
                if filenode as ::core::ffi::c_int != 0
                    && lflags as ::core::ffi::c_int & LOOKUP_ACCESS_MODES_IO
                        != 0 as ::core::ffi::c_int
                {
                    if knowflags != 0 {
                        if lflags as ::core::ffi::c_int & LOOKUP_DIRECTMODE
                            == 0 as ::core::ffi::c_int
                        {
                            if dcm_open(newinode, sessions_get_id((*eptr).sesdata)) != 0 {
                                lflags =
                                    (lflags as ::core::ffi::c_int | LOOKUP_KEEPCACHE) as uint16_t;
                            } else if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                                attr[0 as usize] = (attr[0 as usize] as ::core::ffi::c_int
                                    & (0xff as ::core::ffi::c_int ^ MATTR_ALLOWDATACACHE))
                                    as uint8_t;
                            } else {
                                attr[1 as usize] = (attr[1 as usize] as ::core::ffi::c_int
                                    & (0xff as ::core::ffi::c_int
                                        ^ MATTR_ALLOWDATACACHE << 4 as ::core::ffi::c_int))
                                    as uint8_t;
                            }
                        }
                    } else if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT
                        == 0 as ::core::ffi::c_int
                        || attr[0 as usize] as ::core::ffi::c_int & MATTR_DIRECTMODE
                            == 0 as ::core::ffi::c_int
                    {
                        if dcm_open(newinode, sessions_get_id((*eptr).sesdata))
                            == 0 as ::core::ffi::c_int
                        {
                            if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                                attr[0 as usize] = (attr[0 as usize] as ::core::ffi::c_int
                                    & (0xff as ::core::ffi::c_int ^ MATTR_ALLOWDATACACHE))
                                    as uint8_t;
                            } else {
                                attr[1 as usize] = (attr[1 as usize] as ::core::ffi::c_int
                                    & (0xff as ::core::ffi::c_int
                                        ^ MATTR_ALLOWDATACACHE << 4 as ::core::ffi::c_int))
                                    as uint8_t;
                            }
                        }
                    }
                    if validchunk as ::core::ffi::c_int != 0
                        && sessions_get_disables((*eptr).sesdata) & DISABLE_READ as uint32_t
                            == 0 as uint32_t
                    {
                        if chunkid > 0 as uint64_t {
                            if chunk_get_version_and_csdata(
                                2 as uint8_t,
                                chunkid,
                                (*eptr).peerip,
                                &raw mut version,
                                &raw mut count,
                                &raw mut cs_data as *mut uint8_t,
                                &raw mut split,
                            ) as ::core::ffi::c_int
                                == MFS_STATUS_OK
                            {
                                if split as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    || split as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                                        && (*eptr).version
                                            >= (4 as ::core::ffi::c_int
                                                * 0x10000 as ::core::ffi::c_int
                                                + 0 as ::core::ffi::c_int
                                                    * 0x100 as ::core::ffi::c_int
                                                + (if 4 as ::core::ffi::c_int
                                                    > 1 as ::core::ffi::c_int
                                                {
                                                    0 as ::core::ffi::c_int
                                                        * 2 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                }))
                                                as uint32_t
                                    || split as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                                        && (*eptr).version
                                            >= (4 as ::core::ffi::c_int
                                                * 0x10000 as ::core::ffi::c_int
                                                + 26 as ::core::ffi::c_int
                                                    * 0x100 as ::core::ffi::c_int
                                                + (if 4 as ::core::ffi::c_int
                                                    > 1 as ::core::ffi::c_int
                                                {
                                                    0 as ::core::ffi::c_int
                                                        * 2 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                }))
                                                as uint32_t
                                {
                                    lflags = (lflags as ::core::ffi::c_int | LOOKUP_CHUNK_ZERO_DATA)
                                        as uint16_t;
                                }
                                if split as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    && count as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    && version == 0 as uint32_t
                                {
                                    chunkid = 0 as uint64_t;
                                }
                            }
                        } else {
                            version = 0 as uint32_t;
                            count = 0 as uint8_t;
                            lflags =
                                (lflags as ::core::ffi::c_int | LOOKUP_CHUNK_ZERO_DATA) as uint16_t;
                        }
                    }
                }
                if sesflags as ::core::ffi::c_int & SESFLAG_READONLY != 0 {
                    lflags = (lflags as ::core::ffi::c_int | LOOKUP_RO_FILESYSTEM) as uint16_t;
                }
                if lflags as ::core::ffi::c_int & LOOKUP_CHUNK_ZERO_DATA != 0 {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_LOOKUP as uint32_t,
                        ((*eptr).asize as ::core::ffi::c_int
                            + 23 as ::core::ffi::c_int
                            + count as ::core::ffi::c_int * 14 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                    put32bit(&raw mut ptr, msgid);
                    put32bit(&raw mut ptr, newinode);
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                        (*eptr).asize as size_t,
                    );
                    ptr = ptr.offset((*eptr).asize as ::core::ffi::c_int as isize);
                    put16bit(&raw mut ptr, lflags);
                    if split != 0 {
                        put8bit(&raw mut ptr, 3 as uint8_t);
                    } else {
                        put8bit(&raw mut ptr, 2 as uint8_t);
                    }
                    put64bit(&raw mut ptr, chunkid);
                    put32bit(&raw mut ptr, version);
                    if count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        memcpy(
                            ptr as *mut ::core::ffi::c_void,
                            &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                            (count as ::core::ffi::c_int * 14 as ::core::ffi::c_int) as size_t,
                        );
                    }
                } else {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_LOOKUP as uint32_t,
                        ((*eptr).asize as ::core::ffi::c_int + 10 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                    put32bit(&raw mut ptr, msgid);
                    put32bit(&raw mut ptr, newinode);
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                        (*eptr).asize as size_t,
                    );
                    ptr = ptr.offset((*eptr).asize as ::core::ffi::c_int as isize);
                    put16bit(&raw mut ptr, lflags);
                }
            }
        } else {
            status = fs_lookup(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                nleng as uint16_t,
                name,
                uid,
                gids,
                gid,
                auid,
                agid,
                &raw mut newinode,
                &raw mut attr as *mut uint8_t,
                0 as uint8_t,
                ::core::ptr::null_mut::<uint16_t>(),
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
            );
            if status as ::core::ffi::c_int == MFS_ERROR_ENOENT_NOCACHE
                && (*eptr).version
                    < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            25 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            25 as ::core::ffi::c_int
                        })) as uint32_t
            {
                status = MFS_ERROR_ENOENT as uint8_t;
            }
            if status as ::core::ffi::c_int == MFS_STATUS_OK {
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_LOOKUP as uint32_t,
                    ((*eptr).asize as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as uint32_t,
                );
                put32bit(&raw mut ptr, msgid);
                put32bit(&raw mut ptr, newinode);
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                    (*eptr).asize as size_t,
                );
            }
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_LOOKUP as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_LOOKUP as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_getattr(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gid: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut opened: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 8 as uint32_t && length != 16 as uint32_t && length != 17 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETATTR - wrong size (%u/8|16|17)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        if length == 17 as uint32_t {
            opened = get8bit(&raw mut data);
        } else {
            opened = 0 as uint8_t;
        }
        if length >= 16 as uint32_t {
            uid = get32bit(&raw mut data);
            auid = uid;
            gid = get32bit(&raw mut data);
            agid = gid;
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, &raw mut gid);
        } else {
            uid = 12345 as uint32_t;
            auid = uid;
            gid = 12345 as uint32_t;
            agid = gid;
        }
        status = fs_getattr(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            opened,
            uid,
            gid,
            auid,
            agid,
            &raw mut attr as *mut uint8_t,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETATTR as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                (*eptr).asize as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_GETATTR as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_setattr(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut opened: uint8_t = 0;
        let mut setmask: uint16_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut sugidclearmode: uint8_t = 0;
        let mut attrmode: uint16_t = 0;
        let mut attruid: uint32_t = 0;
        let mut attrgid: uint32_t = 0;
        let mut attratime: uint32_t = 0;
        let mut attrmtime: uint32_t = 0;
        let mut disables: uint32_t = 0;
        let mut winattr: uint8_t = 0;
        let mut basesize: uint8_t = 0;
        basesize = (if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    93 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    93 as ::core::ffi::c_int
                })) as uint32_t
            && (*eptr).version
                != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            && (*eptr).version
                != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })) as uint32_t
        {
            38 as ::core::ffi::c_int
        } else {
            37 as ::core::ffi::c_int
        }) as uint8_t;
        if length != 35 as uint32_t && length != 36 as uint32_t && length < basesize as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETATTR - wrong size (%u/35|36|37|37+N*4|38+N*4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        if length >= 37 as uint32_t {
            opened = get8bit(&raw mut data);
        } else {
            opened = 0 as uint8_t;
        }
        uid = get32bit(&raw mut data);
        auid = uid;
        if length <= 37 as uint32_t {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SETATTR - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SETATTR - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length != (basesize as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SETATTR - wrong size (%u:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        setmask = get8bit(&raw mut data) as uint16_t;
        attrmode = get16bit(&raw mut data);
        attruid = get32bit(&raw mut data);
        attrgid = get32bit(&raw mut data);
        attratime = get32bit(&raw mut data);
        attrmtime = get32bit(&raw mut data);
        if basesize as ::core::ffi::c_int == 38 as ::core::ffi::c_int {
            winattr = get8bit(&raw mut data);
        } else {
            winattr = 0 as uint8_t;
        }
        if length >= 36 as uint32_t {
            sugidclearmode = get8bit(&raw mut data);
        } else {
            sugidclearmode = SUGID_CLEAR_MODE_ALWAYS as uint8_t;
        }
        disables = sessions_get_disables((*eptr).sesdata);
        if disables & DISABLE_CHOWN as uint32_t != 0
            && setmask as ::core::ffi::c_int & (SET_UID_FLAG | SET_GID_FLAG) != 0
            || disables & DISABLE_CHMOD as uint32_t != 0
                && setmask as ::core::ffi::c_int & SET_MODE_FLAG != 0
        {
            status = MFS_ERROR_EPERM as uint8_t;
        } else if setmask as ::core::ffi::c_int & SET_WINATTR_FLAG != 0
            && basesize as ::core::ffi::c_int == 37 as ::core::ffi::c_int
        {
            status = MFS_ERROR_EINVAL as uint8_t;
        } else {
            status = fs_setattr(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                opened,
                uid,
                gids,
                gid,
                auid,
                agid,
                setmask as uint8_t,
                attrmode,
                attruid,
                attrgid,
                attratime,
                attrmtime,
                winattr,
                sugidclearmode,
                &raw mut attr as *mut uint8_t,
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_SETATTR as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                (*eptr).asize as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_SETATTR as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_truncate(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut flags: uint8_t = 0;
        let mut fleng: uint64_t = 0;
        if length != 24 as uint32_t && length < 25 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_TRUNCATE - wrong size (%u/24|25+N*4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        flags = 0 as uint8_t;
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        if length >= 25 as uint32_t {
            flags = get8bit(&raw mut data);
        }
        uid = get32bit(&raw mut data);
        auid = uid;
        if length <= 25 as uint32_t {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
            if length == 24 as uint32_t {
                if uid == 0 as uint32_t && *gid.offset(0 as isize) != 0 as uint32_t {
                    flags = TRUNCATE_FLAG_OPENED as uint8_t;
                }
            }
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_TRUNCATE - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_TRUNCATE - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length != (25 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_TRUNCATE - wrong size (%u:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        }
        fleng = get64bit(&raw mut data);
        matoclserv_fuse_truncate_common(
            eptr, msgid, inode, flags, uid, gids, gid, auid, agid, fleng,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_readlink(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut path: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_READLINK - wrong size (%u/8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        status = fs_readlink(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            &raw mut pleng,
            &raw mut path,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_READLINK as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                (8 as uint32_t)
                    .wrapping_add(pleng)
                    .wrapping_add(1 as uint32_t)
            },
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, pleng.wrapping_add(1 as uint32_t));
            if pleng > 0 as uint32_t {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    path as *const ::core::ffi::c_void,
                    pleng as size_t,
                );
            }
            *ptr.offset(pleng as isize) = 0 as uint8_t;
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_READLINK as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_symlink(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut newinode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 21 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SYMLINK - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length < (21 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SYMLINK - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name = data;
        data = data.offset(nleng as ::core::ffi::c_int as isize);
        pleng = get32bit(&raw mut data);
        if length
            < (21 as uint32_t)
                .wrapping_add(nleng as uint32_t)
                .wrapping_add(pleng)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SYMLINK - wrong size (%u:nleng=%hhu:pleng=%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
                pleng,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        path = data;
        data = data.offset(pleng as isize);
        uid = get32bit(&raw mut data);
        auid = uid;
        if length
            == (21 as uint32_t)
                .wrapping_add(nleng as uint32_t)
                .wrapping_add(pleng)
        {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SYMLINK - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SYMLINK - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (21 as uint32_t)
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add(pleng)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SYMLINK - wrong size (%u:nleng=%hhu:pleng=%u:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                    pleng,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        while pleng > 0 as uint32_t
            && *path.offset(pleng.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            pleng = pleng.wrapping_sub(1);
        }
        if sessions_get_disables((*eptr).sesdata) & DISABLE_SYMLINK as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_symlink(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                nleng as uint16_t,
                name,
                pleng,
                path,
                uid,
                gids,
                gid,
                auid,
                agid,
                &raw mut newinode,
                &raw mut attr as *mut uint8_t,
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_SYMLINK as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                (*eptr).asize as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, newinode);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_SYMLINK as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_mknod(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut rdev: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut r#type: uint8_t = 0;
        let mut mode: uint16_t = 0;
        let mut cumask: uint16_t = 0;
        let mut disables: uint32_t = 0;
        let mut newinode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 24 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_MKNOD - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length != (24 as uint32_t).wrapping_add(nleng as uint32_t)
            && length < (26 as uint32_t).wrapping_add(nleng as uint32_t)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_MKNOD - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name = data;
        data = data.offset(nleng as ::core::ffi::c_int as isize);
        r#type = get8bit(&raw mut data);
        mode = get16bit(&raw mut data);
        if length >= (26 as uint32_t).wrapping_add(nleng as uint32_t) {
            cumask = get16bit(&raw mut data);
        } else {
            cumask = 0 as uint16_t;
        }
        cumask = (cumask as ::core::ffi::c_int
            | sessions_get_umask((*eptr).sesdata) as ::core::ffi::c_int)
            as uint16_t;
        uid = get32bit(&raw mut data);
        auid = uid;
        if length <= (26 as uint32_t).wrapping_add(nleng as uint32_t) {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_MKNOD - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_MKNOD - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (26 as uint32_t)
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_MKNOD - wrong size (%u:nleng=%hhu:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        rdev = get32bit(&raw mut data);
        disables = sessions_get_disables((*eptr).sesdata);
        if disables & DISABLE_MKFIFO as uint32_t != 0 && r#type as ::core::ffi::c_int == TYPE_FIFO
            || disables & DISABLE_MKDEV as uint32_t != 0
                && (r#type as ::core::ffi::c_int == TYPE_BLOCKDEV
                    || r#type as ::core::ffi::c_int == TYPE_CHARDEV)
            || disables & DISABLE_MKSOCK as uint32_t != 0
                && r#type as ::core::ffi::c_int == TYPE_SOCKET
            || disables & DISABLE_CREATE as uint32_t != 0
                && r#type as ::core::ffi::c_int == TYPE_FILE
        {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_mknod(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                nleng as uint16_t,
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
                &raw mut newinode,
                &raw mut attr as *mut uint8_t,
                ::core::ptr::null_mut::<uint8_t>(),
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_MKNOD as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                (*eptr).asize as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, newinode);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_MKNOD as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_mkdir(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut mode: uint16_t = 0;
        let mut cumask: uint16_t = 0;
        let mut newinode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut copysgid: uint8_t = 0;
        if length < 19 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_MKDIR - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length != (19 as uint32_t).wrapping_add(nleng as uint32_t)
            && length != (20 as uint32_t).wrapping_add(nleng as uint32_t)
            && length < (22 as uint32_t).wrapping_add(nleng as uint32_t)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_MKDIR - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name = data;
        data = data.offset(nleng as ::core::ffi::c_int as isize);
        mode = get16bit(&raw mut data);
        if length >= (22 as uint32_t).wrapping_add(nleng as uint32_t) {
            cumask = get16bit(&raw mut data);
        } else {
            cumask = 0 as uint16_t;
        }
        cumask = (cumask as ::core::ffi::c_int
            | sessions_get_umask((*eptr).sesdata) as ::core::ffi::c_int)
            as uint16_t;
        uid = get32bit(&raw mut data);
        auid = uid;
        if length <= (22 as uint32_t).wrapping_add(nleng as uint32_t) {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_MKDIR - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_MKDIR - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (22 as uint32_t)
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_MKDIR - wrong size (%u:nleng=%hhu:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        if length > (20 as uint32_t).wrapping_add(nleng as uint32_t) {
            copysgid = get8bit(&raw mut data);
        } else {
            copysgid = 0 as uint8_t;
        }
        if sessions_get_disables((*eptr).sesdata) & DISABLE_MKDIR as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_mkdir(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                nleng as uint16_t,
                name,
                mode,
                cumask,
                uid,
                gids,
                gid,
                auid,
                agid,
                copysgid,
                &raw mut newinode,
                &raw mut attr as *mut uint8_t,
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_MKDIR as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                (*eptr).asize as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, newinode);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_MKDIR as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_unlink(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut uinode: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 17 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_UNLINK - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length < (17 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_UNLINK - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name = data;
        data = data.offset(nleng as ::core::ffi::c_int as isize);
        uid = get32bit(&raw mut data);
        if length == (17 as uint32_t).wrapping_add(nleng as uint32_t) {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_UNLINK - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_UNLINK - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (17 as uint32_t)
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_UNLINK - wrong size (%u:nleng=%hhu:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        if sessions_get_disables((*eptr).sesdata) & DISABLE_UNLINK as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_unlink(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                nleng as uint16_t,
                name,
                uid,
                gids,
                gid,
                &raw mut uinode,
            );
        }
        if ((*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    107 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    107 as ::core::ffi::c_int
                })) as uint32_t
            && (*eptr).version
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            || (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t)
            && status as ::core::ffi::c_int == MFS_STATUS_OK
        {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_UNLINK as uint32_t, 8 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put32bit(&raw mut ptr, uinode);
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_UNLINK as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_UNLINK as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_rmdir(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut uinode: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 17 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_RMDIR - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length < (17 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_RMDIR - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name = data;
        data = data.offset(nleng as ::core::ffi::c_int as isize);
        uid = get32bit(&raw mut data);
        if length == (17 as uint32_t).wrapping_add(nleng as uint32_t) {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_RMDIR - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_RMDIR - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (17 as uint32_t)
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_RMDIR - wrong size (%u:nleng=%hhu:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        if sessions_get_disables((*eptr).sesdata) & DISABLE_RMDIR as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_rmdir(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                nleng as uint16_t,
                name,
                uid,
                gids,
                gid,
                &raw mut uinode,
            );
        }
        if ((*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    107 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    107 as ::core::ffi::c_int
                })) as uint32_t
            && (*eptr).version
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            || (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t)
            && status as ::core::ffi::c_int == MFS_STATUS_OK
        {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_RMDIR as uint32_t, 8 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put32bit(&raw mut ptr, uinode);
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_RMDIR as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_RMDIR as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_rename(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut inode_src: uint32_t = 0;
        let mut inode_dst: uint32_t = 0;
        let mut nleng_src: uint8_t = 0;
        let mut nleng_dst: uint8_t = 0;
        let mut name_src: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut name_dst: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut rmode: uint8_t = 0;
        let mut i: uint32_t = 0;
        let mut disables: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 22 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_RENAME - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode_src = get32bit(&raw mut data);
        nleng_src = get8bit(&raw mut data);
        if length < (22 as uint32_t).wrapping_add(nleng_src as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_RENAME - wrong size (%u:nleng_src=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng_src as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name_src = data;
        data = data.offset(nleng_src as ::core::ffi::c_int as isize);
        inode_dst = get32bit(&raw mut data);
        nleng_dst = get8bit(&raw mut data);
        if length
            < (22 as uint32_t)
                .wrapping_add(nleng_src as uint32_t)
                .wrapping_add(nleng_dst as uint32_t)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_RENAME - wrong size (%u:nleng_src=%hhu:nleng_dst=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng_src as ::core::ffi::c_int,
                nleng_dst as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name_dst = data;
        data = data.offset(nleng_dst as ::core::ffi::c_int as isize);
        uid = get32bit(&raw mut data);
        auid = uid;
        rmode = MFS_RENAME_STD as uint8_t;
        if length
            == (22 as uint32_t)
                .wrapping_add(nleng_src as uint32_t)
                .wrapping_add(nleng_dst as uint32_t)
        {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_RENAME - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_RENAME - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (22 as uint32_t)
                    .wrapping_add(nleng_src as uint32_t)
                    .wrapping_add(nleng_dst as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
                && length
                    != (23 as uint32_t)
                        .wrapping_add(nleng_src as uint32_t)
                        .wrapping_add(nleng_dst as uint32_t)
                        .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_RENAME - wrong size (%u:nleng_src=%hhu:nleng_dst=%hhu:gids=%u)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    length,
                    nleng_src as ::core::ffi::c_int,
                    nleng_dst as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
            if length
                == (23 as uint32_t)
                    .wrapping_add(nleng_src as uint32_t)
                    .wrapping_add(nleng_dst as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                rmode = get8bit(&raw mut data);
            }
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        disables = sessions_get_disables((*eptr).sesdata);
        if disables & (DISABLE_RENAME as uint32_t | DISABLE_MOVE as uint32_t)
            == DISABLE_RENAME as uint32_t | DISABLE_MOVE as uint32_t
            || disables & DISABLE_RENAME as uint32_t != 0
                && (nleng_src as ::core::ffi::c_int != nleng_dst as ::core::ffi::c_int
                    || memcmp(
                        name_src as *const ::core::ffi::c_void,
                        name_dst as *const ::core::ffi::c_void,
                        nleng_src as size_t,
                    ) != 0 as ::core::ffi::c_int)
            || disables & DISABLE_MOVE as uint32_t != 0 && inode_src != inode_dst
        {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_rename(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode_src,
                nleng_src as uint16_t,
                name_src,
                inode_dst,
                nleng_dst as uint16_t,
                name_dst,
                uid,
                gids,
                gid,
                auid,
                agid,
                rmode,
                ((if disables & DISABLE_UNLINK as uint32_t != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) | (if disables & DISABLE_RMDIR as uint32_t != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint8_t,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            );
        }
        if (*eptr).version
            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    21 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    21 as ::core::ffi::c_int
                })) as uint32_t
            && status as ::core::ffi::c_int == MFS_STATUS_OK
        {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_RENAME as uint32_t,
                ((*eptr).asize as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
            put32bit(&raw mut ptr, inode);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_RENAME as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_RENAME as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_link(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut inode_dst: uint32_t = 0;
        let mut nleng_dst: uint8_t = 0;
        let mut name_dst: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut newinode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 21 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_LINK - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        inode_dst = get32bit(&raw mut data);
        nleng_dst = get8bit(&raw mut data);
        if length < (21 as uint32_t).wrapping_add(nleng_dst as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_LINK - wrong size (%u:nleng_dst=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng_dst as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name_dst = data;
        data = data.offset(nleng_dst as ::core::ffi::c_int as isize);
        uid = get32bit(&raw mut data);
        auid = uid;
        if length == (21 as uint32_t).wrapping_add(nleng_dst as uint32_t) {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_LINK - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_LINK - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (21 as uint32_t)
                    .wrapping_add(nleng_dst as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_LINK - wrong size (%u:nleng_dst=%hhu:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng_dst as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        if sessions_get_disables((*eptr).sesdata) & DISABLE_LINK as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_link(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                inode_dst,
                nleng_dst as uint16_t,
                name_dst,
                uid,
                gids,
                gid,
                auid,
                agid,
                &raw mut newinode,
                &raw mut attr as *mut uint8_t,
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_LINK as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                (*eptr).asize as ::core::ffi::c_int + 8 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, newinode);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_LINK as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_readdir(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut flags: uint8_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut dleng: uint32_t = 0;
        let mut maxentries: uint32_t = 0;
        let mut nedgeid: uint64_t = 0;
        let mut attrmode: uint8_t = 0;
        let mut c1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut c2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*eptr).asize as ::core::ffi::c_int == 35 as ::core::ffi::c_int {
            attrmode = 1 as uint8_t;
        } else if (*eptr).asize as ::core::ffi::c_int == ATTR_RECORD_SIZE {
            attrmode = 2 as uint8_t;
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_READDIR - requested attr size not implemented\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length != 16 as uint32_t && length != 17 as uint32_t && length < 29 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_READDIR - wrong size (%u/16|17|29+N*4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        uid = get32bit(&raw mut data);
        auid = uid;
        if length <= 29 as uint32_t {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_READDIR - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_READDIR - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length != (29 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_READDIR - wrong size (%u:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        if length >= 17 as uint32_t {
            flags = get8bit(&raw mut data);
        } else {
            flags = 0 as uint8_t;
        }
        if length >= 29 as uint32_t {
            maxentries = get32bit(&raw mut data);
            nedgeid = get64bit(&raw mut data);
        } else {
            maxentries = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            nedgeid = 0 as uint64_t;
        }
        if sessions_get_disables((*eptr).sesdata) & DISABLE_READDIR as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_readdir_size(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                uid,
                gids,
                gid,
                flags,
                maxentries,
                nedgeid,
                &raw mut c1,
                &raw mut c2,
                &raw mut dleng,
                attrmode,
            );
        }
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_READDIR as uint32_t, 5 as uint32_t);
        } else if length >= 29 as uint32_t {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_READDIR as uint32_t,
                (12 as uint32_t).wrapping_add(dleng),
            );
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_READDIR as uint32_t,
                (4 as uint32_t).wrapping_add(dleng),
            );
        }
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            let mut nedgeidptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
            nedgeidptr = ptr;
            if length >= 29 as uint32_t {
                ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
            }
            fs_readdir_data(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                uid,
                *gid.offset(0 as isize),
                auid,
                agid,
                flags,
                maxentries,
                &raw mut nedgeid,
                c1,
                c2,
                ptr,
                attrmode,
            );
            if length >= 29 as uint32_t {
                put64bit(&raw mut nedgeidptr, nedgeid);
            }
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_READDIR as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_open(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut flags: uint8_t = 0;
        let mut oflags: uint8_t = 0;
        let mut sesflags: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut knowflags: uint8_t = (if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    113 as ::core::ffi::c_int
                })) as uint32_t
            && (*eptr).version
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            || (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if length < 17 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_OPEN - wrong size (%u/17+N*4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        if length == 17 as uint32_t {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            uid = get32bit(&raw mut data);
            auid = uid;
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        } else {
            uid = get32bit(&raw mut data);
            auid = uid;
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_OPEN - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_OPEN - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length != (17 as uint32_t).wrapping_add(gids.wrapping_mul(4 as uint32_t)) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_OPEN - wrong size (%u:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
            sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        }
        flags = get8bit(&raw mut data);
        oflags = 0 as uint8_t;
        sesflags = sessions_get_sesflags((*eptr).sesdata) as uint8_t;
        if flags as ::core::ffi::c_int & OPEN_TRUNCATE != 0
            && sessions_get_disables((*eptr).sesdata) & DISABLE_TRUNCATE as uint32_t != 0
        {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_opencheck(
                sessions_get_rootinode((*eptr).sesdata),
                sesflags,
                inode,
                uid,
                gids,
                gid,
                auid,
                agid,
                flags,
                &raw mut attr as *mut uint8_t,
                &raw mut oflags,
            );
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK
            && knowflags as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && oflags as ::core::ffi::c_int & OPEN_APPENDONLY != 0
            && flags as ::core::ffi::c_int & OPEN_WRITE != 0
        {
            status = MFS_ERROR_EACCES as uint8_t;
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            of_openfile(sessions_get_id((*eptr).sesdata), inode);
            if flags as ::core::ffi::c_int & OPEN_CACHE_CLEARED != 0 {
                dcm_access(inode, sessions_get_id((*eptr).sesdata));
            }
        }
        if (*eptr).version
            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    9 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    9 as ::core::ffi::c_int
                })) as uint32_t
            && status as ::core::ffi::c_int == MFS_STATUS_OK
        {
            if knowflags != 0 {
                if oflags as ::core::ffi::c_int & OPEN_DIRECTMODE == 0 as ::core::ffi::c_int {
                    if dcm_open(inode, sessions_get_id((*eptr).sesdata)) != 0 {
                        oflags = (oflags as ::core::ffi::c_int | OPEN_KEEPCACHE) as uint8_t;
                    } else if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                        attr[0 as usize] = (attr[0 as usize] as ::core::ffi::c_int
                            & (0xff as ::core::ffi::c_int ^ MATTR_ALLOWDATACACHE))
                            as uint8_t;
                    } else {
                        attr[1 as usize] = (attr[1 as usize] as ::core::ffi::c_int
                            & (0xff as ::core::ffi::c_int
                                ^ MATTR_ALLOWDATACACHE << 4 as ::core::ffi::c_int))
                            as uint8_t;
                    }
                }
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_OPEN as uint32_t,
                    ((*eptr).asize as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint32_t,
                );
                put32bit(&raw mut ptr, msgid);
                put8bit(&raw mut ptr, oflags);
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                    (*eptr).asize as size_t,
                );
            } else {
                if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT == 0 as ::core::ffi::c_int
                    || attr[0 as usize] as ::core::ffi::c_int & MATTR_DIRECTMODE
                        == 0 as ::core::ffi::c_int
                {
                    if dcm_open(inode, sessions_get_id((*eptr).sesdata)) == 0 as ::core::ffi::c_int
                    {
                        if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                            attr[0 as usize] = (attr[0 as usize] as ::core::ffi::c_int
                                & (0xff as ::core::ffi::c_int ^ MATTR_ALLOWDATACACHE))
                                as uint8_t;
                        } else {
                            attr[1 as usize] = (attr[1 as usize] as ::core::ffi::c_int
                                & (0xff as ::core::ffi::c_int
                                    ^ MATTR_ALLOWDATACACHE << 4 as ::core::ffi::c_int))
                                as uint8_t;
                        }
                    }
                }
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_OPEN as uint32_t,
                    ((*eptr).asize as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t,
                );
                put32bit(&raw mut ptr, msgid);
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                    (*eptr).asize as size_t,
                );
            }
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_OPEN as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_OPEN as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_create(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut auid: uint32_t = 0;
        let mut agid: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut mode: uint16_t = 0;
        let mut cumask: uint16_t = 0;
        let mut newinode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut oflags: uint8_t = 0;
        let mut sesflags: uint8_t = 0;
        let mut knowflags: uint8_t = (if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    113 as ::core::ffi::c_int
                })) as uint32_t
            && (*eptr).version
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            || (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if length < 19 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_CREATE - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length != (19 as uint32_t).wrapping_add(nleng as uint32_t)
            && length < (21 as uint32_t).wrapping_add(nleng as uint32_t)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_CREATE - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name = data;
        data = data.offset(nleng as ::core::ffi::c_int as isize);
        mode = get16bit(&raw mut data);
        if length >= (21 as uint32_t).wrapping_add(nleng as uint32_t) {
            cumask = get16bit(&raw mut data);
        } else {
            cumask = 0 as uint16_t;
        }
        cumask = (cumask as ::core::ffi::c_int
            | sessions_get_umask((*eptr).sesdata) as ::core::ffi::c_int)
            as uint16_t;
        uid = get32bit(&raw mut data);
        auid = uid;
        if length <= (21 as uint32_t).wrapping_add(nleng as uint32_t) {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
            agid = *gid.offset(0 as isize);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_CREATE - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_CREATE - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (21 as uint32_t)
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_CREATE - wrong size (%u:nleng=%hhu:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            agid = *gid.offset(0 as isize);
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        sesflags = sessions_get_sesflags((*eptr).sesdata) as uint8_t;
        if sessions_get_disables((*eptr).sesdata) & DISABLE_CREATE as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_mknod(
                sessions_get_rootinode((*eptr).sesdata),
                sesflags,
                inode,
                nleng as uint16_t,
                name,
                TYPE_FILE as uint8_t,
                mode,
                cumask,
                uid,
                gids,
                gid,
                auid,
                agid,
                0 as uint32_t,
                &raw mut newinode,
                &raw mut attr as *mut uint8_t,
                &raw mut oflags,
            );
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            if CreateFirstChunk != 0 {
                let mut prevchunkid: uint64_t = 0;
                let mut chunkid: uint64_t = 0;
                let mut fleng: uint64_t = 0;
                let mut opflag: uint8_t = 0;
                let mut swc: *mut swchunks = ::core::ptr::null_mut::<swchunks>();
                if fs_writechunk(
                    newinode,
                    0 as uint32_t,
                    0 as uint8_t,
                    &raw mut prevchunkid,
                    &raw mut chunkid,
                    &raw mut fleng,
                    &raw mut opflag,
                    (*eptr).peerip,
                ) as ::core::ffi::c_int
                    == MFS_STATUS_OK
                {
                    if prevchunkid == 0 as uint64_t {
                    } else {
                        fprintf(
                            stderr,
                            b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3796 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"prevchunkid==0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"chunk created after mknod - prevchunkid should be always zero\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3796 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"prevchunkid==0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"chunk created after mknod - prevchunkid should be always zero\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    };
                    if opflag != 0 {
                        i = (chunkid & 0xff as uint64_t) as uint32_t;
                        swc = malloc(::core::mem::size_of::<swchunks>()) as *mut swchunks;
                        if swc.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                3800 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                3800 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if swc
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut swchunks
                        {
                            let mut _mfs_errorstring: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                3800 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                3800 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"swc\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring,
                            );
                            abort();
                        }
                        (*swc).eptr = eptr;
                        (*swc).inode = newinode;
                        (*swc).indx = 0 as uint32_t;
                        (*swc).prevchunkid = prevchunkid;
                        (*swc).chunkid = chunkid;
                        (*swc).msgid = 0 as uint32_t;
                        (*swc).fleng = fleng;
                        (*swc).r#type = FUSE_CREATE as ::core::ffi::c_int as uint8_t;
                        (*swc).next = swchunkshash[i as usize] as *mut _swchunks;
                        swchunkshash[i as usize] = swc;
                    } else {
                        fs_writeend(
                            newinode,
                            0 as uint64_t,
                            chunkid,
                            0 as uint8_t,
                            ::core::ptr::null_mut::<uint8_t>(),
                        );
                    }
                }
            }
            of_openfile(sessions_get_id((*eptr).sesdata), newinode);
            if knowflags != 0 {
                if oflags as ::core::ffi::c_int & OPEN_DIRECTMODE == 0 as ::core::ffi::c_int {
                    if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                        attr[0 as usize] = (attr[0 as usize] as ::core::ffi::c_int
                            & (0xff as ::core::ffi::c_int ^ MATTR_ALLOWDATACACHE))
                            as uint8_t;
                    } else {
                        attr[1 as usize] = (attr[1 as usize] as ::core::ffi::c_int
                            & (0xff as ::core::ffi::c_int
                                ^ MATTR_ALLOWDATACACHE << 4 as ::core::ffi::c_int))
                            as uint8_t;
                    }
                }
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_CREATE as uint32_t,
                    ((*eptr).asize as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as uint32_t,
                );
                put32bit(&raw mut ptr, msgid);
                put8bit(&raw mut ptr, oflags);
                put32bit(&raw mut ptr, newinode);
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                    (*eptr).asize as size_t,
                );
            } else {
                if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT == 0 as ::core::ffi::c_int
                    || attr[0 as usize] as ::core::ffi::c_int & MATTR_DIRECTMODE
                        == 0 as ::core::ffi::c_int
                {
                    if sesflags as ::core::ffi::c_int & SESFLAG_ATTRBIT != 0 {
                        attr[0 as usize] = (attr[0 as usize] as ::core::ffi::c_int
                            & (0xff as ::core::ffi::c_int ^ MATTR_ALLOWDATACACHE))
                            as uint8_t;
                    } else {
                        attr[1 as usize] = (attr[1 as usize] as ::core::ffi::c_int
                            & (0xff as ::core::ffi::c_int
                                ^ MATTR_ALLOWDATACACHE << 4 as ::core::ffi::c_int))
                            as uint8_t;
                    }
                }
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_CREATE as uint32_t,
                    ((*eptr).asize as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as uint32_t,
                );
                put32bit(&raw mut ptr, msgid);
                put32bit(&raw mut ptr, newinode);
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                    (*eptr).asize as size_t,
                );
            }
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_CREATE as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_CREATE as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_read_chunk(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut chunkopflags: uint8_t = 0;
        let mut msgid: uint32_t = 0;
        if length != 12 as uint32_t && length != 13 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_READ_CHUNK - wrong size (%u/12|13)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        indx = get32bit(&raw mut data);
        if length == 13 as uint32_t {
            chunkopflags = get8bit(&raw mut data);
        } else {
            chunkopflags = CHUNKOPFLAG_CANMODTIME as uint8_t;
        }
        if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    74 as ::core::ffi::c_int
                })) as uint32_t
        {
            chunkopflags =
                (chunkopflags as ::core::ffi::c_int & !CHUNKOPFLAG_CANMODTIME) as uint8_t;
        }
        matoclserv_fuse_read_chunk_common(eptr, msgid, inode, indx, chunkopflags);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_write_chunk(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut chunkopflags: uint8_t = 0;
        if length != 12 as uint32_t && length != 13 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_WRITE_CHUNK - wrong size (%u/12|13)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        indx = get32bit(&raw mut data);
        if length >= 13 as uint32_t {
            chunkopflags = get8bit(&raw mut data);
        } else {
            chunkopflags = CHUNKOPFLAG_CANMODTIME as uint8_t;
        }
        if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    74 as ::core::ffi::c_int
                })) as uint32_t
        {
            chunkopflags =
                (chunkopflags as ::core::ffi::c_int & !CHUNKOPFLAG_CANMODTIME) as uint8_t;
        }
        matoclserv_fuse_write_chunk_common(eptr, msgid, inode, indx, chunkopflags);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_write_chunk_end(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut msgid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut fleng: uint64_t = 0;
        let mut indx: uint32_t = 0;
        let mut version: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut offset: uint32_t = 0;
        let mut size: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut chunkopflags: uint8_t = 0;
        let mut flenghaschanged: uint8_t = 0;
        if length != 24 as uint32_t
            && length != 25 as uint32_t
            && length != 29 as uint32_t
            && length != 37 as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_WRITE_CHUNK_END - wrong size (%u/24|25|29|37)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        chunkid = get64bit(&raw mut data);
        inode = get32bit(&raw mut data);
        if length >= 29 as uint32_t {
            indx = get32bit(&raw mut data);
        } else {
            indx = 0 as uint32_t;
        }
        fleng = get64bit(&raw mut data);
        if length >= 25 as uint32_t {
            chunkopflags = get8bit(&raw mut data);
        } else {
            chunkopflags = CHUNKOPFLAG_CANMODTIME as uint8_t;
        }
        if length >= 37 as uint32_t {
            if (*eptr).version
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 48 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                offset = 0 as uint32_t;
                size = MFSCHUNKSIZE as uint32_t;
                data = data.offset(8 as ::core::ffi::c_int as isize);
            } else {
                offset = get32bit(&raw mut data);
                size = get32bit(&raw mut data);
            }
        } else {
            offset = 0 as uint32_t;
            size = MFSCHUNKSIZE as uint32_t;
        }
        if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    74 as ::core::ffi::c_int
                })) as uint32_t
        {
            chunkopflags =
                (chunkopflags as ::core::ffi::c_int & !CHUNKOPFLAG_CANMODTIME) as uint8_t;
        }
        flenghaschanged = 0 as uint8_t;
        if sessions_get_disables((*eptr).sesdata) & DISABLE_WRITE as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_READONLY as uint32_t != 0 {
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        101 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        101 as ::core::ffi::c_int
                    })) as uint32_t
            {
                status = MFS_ERROR_EROFS as uint8_t;
            } else {
                status = MFS_ERROR_IO as uint8_t;
            }
        } else {
            status = fs_writeend(
                inode,
                fleng,
                chunkid,
                chunkopflags,
                &raw mut flenghaschanged,
            );
        }
        dcm_modify(inode, sessions_get_id((*eptr).sesdata));
        ptr =
            matoclserv_create_packet(eptr, MATOCL_FUSE_WRITE_CHUNK_END as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
        if length >= 29 as uint32_t {
            chunk_get_version(chunkid, &raw mut version);
            matoclserv_fuse_chunk_has_changed(
                eptr,
                inode,
                indx,
                chunkid,
                version,
                fleng,
                0 as uint8_t,
                offset,
                size,
            );
        } else if flenghaschanged != 0 {
            matoclserv_fuse_fleng_has_changed(eptr, inode, fleng);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_chunk_has_changed(
    mut eptr: *mut matoclserventry,
    mut inode: uint32_t,
    mut chindx: uint32_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut fleng: uint64_t,
    mut truncateflag: uint8_t,
    mut offset: uint32_t,
    mut size: uint32_t,
) {
    unsafe {
        let mut xeptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut pver: uint8_t = 0;
        xeptr = matoclservhead;
        while !xeptr.is_null() {
            if xeptr != eptr
                && (*xeptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*xeptr).registered as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
                && !(*xeptr).sesdata.is_null()
                && (*xeptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            74 as ::core::ffi::c_int
                        })) as uint32_t
            {
                if of_isfileopened_by_session(inode, sessions_get_id((*xeptr).sesdata)) != 0 {
                    pver = (if (*xeptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                    ptr = matoclserv_create_packet(
                        xeptr,
                        MATOCL_FUSE_CHUNK_HAS_CHANGED as uint32_t,
                        (if pver as ::core::ffi::c_int != 0 {
                            41 as ::core::ffi::c_int
                        } else {
                            33 as ::core::ffi::c_int
                        }) as uint32_t,
                    );
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    put32bit(&raw mut ptr, inode);
                    put32bit(&raw mut ptr, chindx);
                    put64bit(&raw mut ptr, chunkid);
                    put32bit(&raw mut ptr, version);
                    put64bit(&raw mut ptr, fleng);
                    put8bit(&raw mut ptr, truncateflag);
                    if pver != 0 {
                        put32bit(&raw mut ptr, offset);
                        put32bit(&raw mut ptr, size);
                    }
                    if (*xeptr).working_flags as ::core::ffi::c_int & WFLAG_INVALIDATE_CACHE != 0 {
                        dcm_access(inode, sessions_get_id((*xeptr).sesdata));
                    }
                }
            }
            xeptr = (*xeptr).next as *mut matoclserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_fleng_has_changed(
    mut eptr: *mut matoclserventry,
    mut inode: uint32_t,
    mut fleng: uint64_t,
) {
    unsafe {
        let mut xeptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        xeptr = matoclservhead;
        while !xeptr.is_null() {
            if xeptr != eptr
                && (*xeptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*xeptr).registered as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
                && !(*xeptr).sesdata.is_null()
                && (*xeptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            74 as ::core::ffi::c_int
                        })) as uint32_t
            {
                if of_isfileopened_by_session(inode, sessions_get_id((*xeptr).sesdata)) != 0 {
                    ptr = matoclserv_create_packet(
                        xeptr,
                        MATOCL_FUSE_FLENG_HAS_CHANGED as uint32_t,
                        16 as uint32_t,
                    );
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    put32bit(&raw mut ptr, inode);
                    put64bit(&raw mut ptr, fleng);
                }
            }
            xeptr = (*xeptr).next as *mut matoclserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_invalidate_chunk_cache() {
    unsafe {
        let mut xeptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        xeptr = matoclservhead;
        while !xeptr.is_null() {
            if (*xeptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*xeptr).registered as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
                && !(*xeptr).sesdata.is_null()
                && ((*xeptr).version
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                    || (*xeptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                100 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                100 as ::core::ffi::c_int
                            })) as uint32_t
                        && (*xeptr).version
                            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t)
            {
                ptr = matoclserv_create_packet(
                    xeptr,
                    MATOCL_FUSE_INVALIDATE_CHUNK_CACHE as uint32_t,
                    4 as uint32_t,
                );
                put32bit(&raw mut ptr, 0 as uint32_t);
            }
            xeptr = (*xeptr).next as *mut matoclserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_flock_wake_up(
    mut veptr: *mut ::core::ffi::c_void,
    mut msgid: uint32_t,
    mut status: uint8_t,
) {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        eptr = veptr as *mut matoclserventry;
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_FLOCK as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_flock(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut reqid: uint32_t = 0;
        let mut owner: uint64_t = 0;
        let mut cmd: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 21 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_FLOCK - wrong size (%u/21)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        reqid = get32bit(&raw mut data);
        owner = get64bit(&raw mut data);
        cmd = get8bit(&raw mut data);
        status = flock_locks_cmd(
            eptr as *mut ::core::ffi::c_void,
            sessions_get_id((*eptr).sesdata),
            msgid,
            reqid,
            inode,
            owner,
            cmd,
        );
        sessions_inc_stats((*eptr).sesdata, SES_OP_LOCK as uint8_t);
        stats_lcnt = stats_lcnt.wrapping_add(1);
        if status as ::core::ffi::c_int == MFS_ERROR_WAITING {
            return;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_FLOCK as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_posix_lock_wake_up(
    mut veptr: *mut ::core::ffi::c_void,
    mut msgid: uint32_t,
    mut status: uint8_t,
) {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        eptr = veptr as *mut matoclserventry;
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_POSIX_LOCK as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_posix_lock(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut reqid: uint32_t = 0;
        let mut pid: uint32_t = 0;
        let mut owner: uint64_t = 0;
        let mut start: uint64_t = 0;
        let mut end: uint64_t = 0;
        let mut cmd: uint8_t = 0;
        let mut r#type: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 42 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_POSIX_LOCK - wrong size (%u/42)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        reqid = get32bit(&raw mut data);
        owner = get64bit(&raw mut data);
        pid = get32bit(&raw mut data);
        cmd = get8bit(&raw mut data);
        r#type = get8bit(&raw mut data);
        start = get64bit(&raw mut data);
        end = get64bit(&raw mut data);
        status = posix_lock_cmd(
            eptr as *mut ::core::ffi::c_void,
            sessions_get_id((*eptr).sesdata),
            msgid,
            reqid,
            inode,
            owner,
            cmd,
            &raw mut r#type,
            &raw mut start,
            &raw mut end,
            &raw mut pid,
        );
        sessions_inc_stats((*eptr).sesdata, SES_OP_LOCK as uint8_t);
        stats_lcnt = stats_lcnt.wrapping_add(1);
        if status as ::core::ffi::c_int == MFS_ERROR_WAITING {
            return;
        }
        if cmd as ::core::ffi::c_int == POSIX_LOCK_CMD_GET
            && status as ::core::ffi::c_int == MFS_STATUS_OK
        {
            ptr =
                matoclserv_create_packet(eptr, MATOCL_FUSE_POSIX_LOCK as uint32_t, 25 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put32bit(&raw mut ptr, pid);
            put8bit(&raw mut ptr, r#type);
            put64bit(&raw mut ptr, start);
            put64bit(&raw mut ptr, end);
        } else {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_POSIX_LOCK as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_repair(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut chunksnotchanged: uint32_t = 0;
        let mut chunkserased: uint32_t = 0;
        let mut chunksrepaired: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut flags: uint8_t = 0;
        if length < 16 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_REPAIR - wrong size (%u/(16|17)+N*4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        flags = 0 as uint8_t;
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        uid = get32bit(&raw mut data);
        if length == 16 as uint32_t {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_REPAIR - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_REPAIR - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length != (16 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids))
                && length != (17 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_REPAIR - wrong size (%u:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
            if length == (17 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)) {
                flags = get8bit(&raw mut data);
            }
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        status = fs_repair(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            uid,
            gids,
            gid,
            flags,
            &raw mut chunksnotchanged,
            &raw mut chunkserased,
            &raw mut chunksrepaired,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_REPAIR as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, chunksnotchanged);
            put32bit(&raw mut ptr, chunkserased);
            put32bit(&raw mut ptr, chunksrepaired);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_check(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut chunkmtime: uint32_t = 0;
        let mut basesize: uint32_t = 0;
        let mut cs_data: [uint8_t; 800] = [0; 800];
        let mut count: uint8_t = 0;
        let mut cdescsize: uint8_t = 0;
        let mut i: uint32_t = 0;
        let mut chunkcount: [uint32_t; 2774] = [0; 2774];
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut mode: uint8_t = 0;
        if length != 8 as uint32_t
            && length != 9 as uint32_t
            && length != 12 as uint32_t
            && length != 13 as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_CHECK - wrong size (%u/8|9|12|13)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if length == 13 as uint32_t || length == 9 as uint32_t {
            mode = get8bit(&raw mut data);
        } else {
            mode = 0 as uint8_t;
        }
        inode = get32bit(&raw mut data);
        if length == 12 as uint32_t || length == 13 as uint32_t {
            if mode as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && mode as ::core::ffi::c_int != 1 as ::core::ffi::c_int
                && mode as ::core::ffi::c_int != 4 as ::core::ffi::c_int
            {
                ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_CHECK as uint32_t, 5 as uint32_t);
                put32bit(&raw mut ptr, msgid);
                put8bit(&raw mut ptr, MFS_ERROR_EINVAL as uint8_t);
                return;
            }
            indx = get32bit(&raw mut data);
            status = fs_filechunk(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                indx,
                &raw mut chunkid,
            );
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_CHECK as uint32_t, 5 as uint32_t);
                put32bit(&raw mut ptr, msgid);
                put8bit(&raw mut ptr, status);
                return;
            }
            if chunkid > 0 as uint64_t {
                status = chunk_get_version_and_copies(
                    mode,
                    chunkid,
                    (*eptr).peerip,
                    &raw mut version,
                    &raw mut chunkmtime,
                    &raw mut count,
                    &raw mut cs_data as *mut uint8_t,
                );
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_CHECK as uint32_t,
                        5 as uint32_t,
                    );
                    put32bit(&raw mut ptr, msgid);
                    put8bit(&raw mut ptr, status);
                    return;
                }
            } else {
                version = 0 as uint32_t;
                chunkmtime = 0 as uint32_t;
                count = 0 as uint8_t;
            }
            cdescsize = (if mode as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                8 as ::core::ffi::c_int
            } else {
                7 as ::core::ffi::c_int
            }) as uint8_t;
            basesize = (if mode as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                20 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            }) as uint32_t;
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_CHECK as uint32_t,
                basesize.wrapping_add(
                    (count as ::core::ffi::c_int * cdescsize as ::core::ffi::c_int) as uint32_t,
                ),
            );
            put32bit(&raw mut ptr, msgid);
            put64bit(&raw mut ptr, chunkid);
            put32bit(&raw mut ptr, version);
            if mode as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                put32bit(&raw mut ptr, chunkmtime);
            }
            if count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut cs_data as *mut uint8_t as *const ::core::ffi::c_void,
                    (count as ::core::ffi::c_int * cdescsize as ::core::ffi::c_int) as size_t,
                );
            }
        } else {
            if mode as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && mode as ::core::ffi::c_int != 2 as ::core::ffi::c_int
                && mode as ::core::ffi::c_int != 3 as ::core::ffi::c_int
            {
                ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_CHECK as uint32_t, 5 as uint32_t);
                put32bit(&raw mut ptr, msgid);
                put8bit(&raw mut ptr, MFS_ERROR_EINVAL as uint8_t);
                return;
            }
            status = fs_checkfile(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                mode,
                &raw mut chunkcount as *mut uint32_t,
            );
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_CHECK as uint32_t, 5 as uint32_t);
                put32bit(&raw mut ptr, msgid);
                put8bit(&raw mut ptr, status);
                return;
            }
            if mode as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if mode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                    let mut j: uint8_t = 0;
                    j = 0 as uint8_t;
                    i = 0 as uint32_t;
                    while i < 200 as uint32_t {
                        if chunkcount[i as usize] > 0 as uint32_t {
                            j = j.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                    }
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_CHECK as uint32_t,
                        (4 as ::core::ffi::c_int
                            + 5 as ::core::ffi::c_int * j as ::core::ffi::c_int)
                            as uint32_t,
                    );
                    put32bit(&raw mut ptr, msgid);
                    i = 0 as uint32_t;
                    while i < 200 as uint32_t {
                        if chunkcount[i as usize] > 0 as uint32_t {
                            put8bit(&raw mut ptr, i as uint8_t);
                            put32bit(&raw mut ptr, chunkcount[i as usize]);
                        }
                        i = i.wrapping_add(1);
                    }
                } else {
                    let mut j_0: uint16_t = 0;
                    j_0 = 0 as uint16_t;
                    i = 0 as uint32_t;
                    while i < 2774 as uint32_t {
                        if chunkcount[i as usize] > 0 as uint32_t {
                            j_0 = j_0.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                    }
                    ptr = matoclserv_create_packet(
                        eptr,
                        MATOCL_FUSE_CHECK as uint32_t,
                        (4 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int * j_0 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                    put32bit(&raw mut ptr, msgid);
                    i = 0 as uint32_t;
                    while i < 2774 as uint32_t {
                        if chunkcount[i as usize] > 0 as uint32_t {
                            put16bit(&raw mut ptr, i as uint16_t);
                            put32bit(&raw mut ptr, chunkcount[i as usize]);
                        }
                        i = i.wrapping_add(1);
                    }
                }
            } else if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        30 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        30 as ::core::ffi::c_int
                    })) as uint32_t
            {
                ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_CHECK as uint32_t, 52 as uint32_t);
                put32bit(&raw mut ptr, msgid);
                i = 0 as uint32_t;
                while i < 12 as uint32_t {
                    put32bit(&raw mut ptr, chunkcount[i as usize]);
                    i = i.wrapping_add(1);
                }
            } else if (*eptr).version
                >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        23 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        23 as ::core::ffi::c_int
                    })) as uint32_t
            {
                ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_CHECK as uint32_t, 48 as uint32_t);
                put32bit(&raw mut ptr, msgid);
                i = 0 as uint32_t;
                while i < 11 as uint32_t {
                    put32bit(&raw mut ptr, chunkcount[i as usize]);
                    i = i.wrapping_add(1);
                }
            } else {
                let mut j_1: uint8_t = 0;
                j_1 = 0 as uint8_t;
                i = 0 as uint32_t;
                while i < 11 as uint32_t {
                    if chunkcount[i as usize] > 0 as uint32_t {
                        j_1 = j_1.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                ptr = matoclserv_create_packet(
                    eptr,
                    MATOCL_FUSE_CHECK as uint32_t,
                    (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * j_1 as ::core::ffi::c_int)
                        as uint32_t,
                );
                put32bit(&raw mut ptr, msgid);
                i = 0 as uint32_t;
                while i < 11 as uint32_t {
                    if chunkcount[i as usize] > 0 as uint32_t {
                        put8bit(&raw mut ptr, i as uint8_t);
                        if chunkcount[i as usize] <= 65535 as uint32_t {
                            put16bit(&raw mut ptr, chunkcount[i as usize] as uint16_t);
                        } else {
                            put16bit(&raw mut ptr, 65535 as uint16_t);
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_gettrashretention(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut gmode: uint8_t = 0;
        let mut fptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut dptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut fnodes: uint32_t = 0;
        let mut dnodes: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETTRASHRETENTION - wrong size (%u/9)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        gmode = get8bit(&raw mut data);
        status = fs_gettrashretention_prepare(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            gmode,
            &raw mut fptr,
            &raw mut dptr,
            &raw mut fnodes,
            &raw mut dnodes,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETTRASHRETENTION as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                (12 as uint32_t)
                    .wrapping_add((8 as uint32_t).wrapping_mul(fnodes.wrapping_add(dnodes)))
            },
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, fnodes);
            put32bit(&raw mut ptr, dnodes);
            fs_gettrashretention_store(fptr, dptr, ptr);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_settrashretention(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut trashretention: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut smode: uint8_t = 0;
        let mut changed: uint32_t = 0;
        let mut notchanged: uint32_t = 0;
        let mut notpermitted: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 17 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETTRASHRETENTION - wrong size (%u/17)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        uid = get32bit(&raw mut data);
        sessions_ugid_remap(
            (*eptr).sesdata,
            &raw mut uid,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        trashretention = get32bit(&raw mut data);
        smode = get8bit(&raw mut data);
        if sessions_get_disables((*eptr).sesdata) & DISABLE_SETTRASH as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = sessions_check_trashretention(
                (*eptr).sesdata,
                (smode as ::core::ffi::c_int & SMODE_TMASK) as uint8_t,
                trashretention,
            );
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            status = fs_settrashretention(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                uid,
                trashretention,
                smode,
                &raw mut changed,
                &raw mut notchanged,
                &raw mut notpermitted,
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_SETTRASHRETENTION as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, changed);
            put32bit(&raw mut ptr, notchanged);
            put32bit(&raw mut ptr, notpermitted);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_getsclass(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut fgtab: [uint32_t; 256] = [0; 256];
        let mut dgtab: [uint32_t; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut i: uint16_t = 0;
        let mut r#fn: uint8_t = 0;
        let mut dn: uint8_t = 0;
        let mut gmode: uint8_t = 0;
        let mut psize: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETSCLASS - wrong size (%u/9)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        gmode = get8bit(&raw mut data);
        status = fs_getsclass(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            gmode,
            &raw mut fgtab as *mut uint32_t,
            &raw mut dgtab as *mut uint32_t,
        );
        r#fn = 0 as uint8_t;
        dn = 0 as uint8_t;
        psize = 6 as uint32_t;
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            i = 1 as uint16_t;
            while (i as ::core::ffi::c_int) < MAXSCLASS {
                if (i as ::core::ffi::c_int) < 10 as ::core::ffi::c_int
                    && (*eptr).version
                        < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                75 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    if fgtab[i as usize] != 0 {
                        r#fn = r#fn.wrapping_add(1);
                        psize = psize.wrapping_add(5 as uint32_t);
                    }
                    if dgtab[i as usize] != 0 {
                        dn = dn.wrapping_add(1);
                        psize = psize.wrapping_add(5 as uint32_t);
                    }
                } else if (*eptr).version
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            75 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    if fgtab[i as usize] != 0 {
                        r#fn = r#fn.wrapping_add(1);
                        psize = psize.wrapping_add(
                            (6 as ::core::ffi::c_int
                                + sclass_get_nleng(i as uint8_t) as ::core::ffi::c_int)
                                as uint32_t,
                        );
                    }
                    if dgtab[i as usize] != 0 {
                        dn = dn.wrapping_add(1);
                        psize = psize.wrapping_add(
                            (6 as ::core::ffi::c_int
                                + sclass_get_nleng(i as uint8_t) as ::core::ffi::c_int)
                                as uint32_t,
                        );
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETSCLASS as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                psize
            },
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put8bit(&raw mut ptr, r#fn);
            put8bit(&raw mut ptr, dn);
            i = 1 as uint16_t;
            while (i as ::core::ffi::c_int) < MAXSCLASS {
                if fgtab[i as usize] != 0 {
                    if (i as ::core::ffi::c_int) < 10 as ::core::ffi::c_int
                        && (*eptr).version
                            < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    75 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        put8bit(&raw mut ptr, i as uint8_t);
                        put32bit(&raw mut ptr, fgtab[i as usize]);
                    } else if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                75 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put8bit(&raw mut ptr, 0xff as uint8_t);
                        nleng = sclass_get_nleng(i as uint8_t);
                        put8bit(&raw mut ptr, nleng);
                        memcpy(
                            ptr as *mut ::core::ffi::c_void,
                            sclass_get_name(i as uint8_t) as *const ::core::ffi::c_void,
                            nleng as size_t,
                        );
                        ptr = ptr.offset(nleng as ::core::ffi::c_int as isize);
                        put32bit(&raw mut ptr, fgtab[i as usize]);
                    }
                }
                i = i.wrapping_add(1);
            }
            i = 1 as uint16_t;
            while (i as ::core::ffi::c_int) < MAXSCLASS {
                if dgtab[i as usize] != 0 {
                    if (i as ::core::ffi::c_int) < 10 as ::core::ffi::c_int
                        && (*eptr).version
                            < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    75 as ::core::ffi::c_int
                                })) as uint32_t
                    {
                        put8bit(&raw mut ptr, i as uint8_t);
                        put32bit(&raw mut ptr, dgtab[i as usize]);
                    } else if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                9 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                9 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        put8bit(&raw mut ptr, 0xff as uint8_t);
                        nleng = sclass_get_nleng(i as uint8_t);
                        put8bit(&raw mut ptr, nleng);
                        memcpy(
                            ptr as *mut ::core::ffi::c_void,
                            sclass_get_name(i as uint8_t) as *const ::core::ffi::c_void,
                            nleng as size_t,
                        );
                        ptr = ptr.offset(nleng as ::core::ffi::c_int as isize);
                        put32bit(&raw mut ptr, dgtab[i as usize]);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_setsclass(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut setid: uint8_t = 0;
        let mut smode: uint8_t = 0;
        let mut changed: uint32_t = 0;
        let mut notchanged: uint32_t = 0;
        let mut notpermitted: uint32_t = 0;
        let mut scnleng: uint8_t = 0;
        let mut src_sclassid: uint8_t = 0;
        let mut dst_sclassid: uint8_t = 0;
        let mut pskip: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 14 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETSCLASS - wrong size (%u/<14)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        uid = get32bit(&raw mut data);
        sessions_ugid_remap(
            (*eptr).sesdata,
            &raw mut uid,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        setid = get8bit(&raw mut data);
        smode = get8bit(&raw mut data);
        if setid as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
            if smode as ::core::ffi::c_int & SMODE_TMASK == SMODE_EXCHANGE {
                if length < 15 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_SETSCLASS - wrong size (%u/<15)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                scnleng = get8bit(&raw mut data);
                src_sclassid = sclass_find_by_name(scnleng, data);
                data = data.offset(scnleng as ::core::ffi::c_int as isize);
                pskip = (15 as ::core::ffi::c_int + scnleng as ::core::ffi::c_int) as uint32_t;
            } else {
                pskip = 14 as uint32_t;
                src_sclassid = 0x1 as uint8_t;
            }
            if length < pskip.wrapping_add(1 as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SETSCLASS - wrong size (%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            scnleng = get8bit(&raw mut data);
            dst_sclassid = sclass_find_by_name(scnleng, data);
            data = data.offset(scnleng as ::core::ffi::c_int as isize);
            if length
                < pskip
                    .wrapping_add(1 as uint32_t)
                    .wrapping_add(scnleng as uint32_t)
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SETSCLASS - wrong size (%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if sessions_get_disables((*eptr).sesdata) & DISABLE_SETSCLASS as uint32_t != 0 {
                status = MFS_ERROR_EPERM as uint8_t;
            } else if src_sclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || dst_sclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                status = MFS_ERROR_NOSUCHCLASS as uint8_t;
            } else {
                status = sessions_check_sclass(
                    (*eptr).sesdata,
                    (smode as ::core::ffi::c_int & SMODE_TMASK) as uint8_t,
                    dst_sclassid,
                );
            }
        } else if sessions_get_disables((*eptr).sesdata) & DISABLE_SETSCLASS as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else if (setid as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
            || setid as ::core::ffi::c_int > 9 as ::core::ffi::c_int
        {
            status = MFS_ERROR_EINVAL as uint8_t;
        } else {
            src_sclassid = setid;
            dst_sclassid = setid;
            if length != 14 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SETSCLASS (classic version) - wrong size (%u/14)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            status = sessions_check_sclass(
                (*eptr).sesdata,
                (smode as ::core::ffi::c_int & SMODE_TMASK) as uint8_t,
                setid,
            );
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            status = fs_setsclass(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                uid,
                src_sclassid,
                dst_sclassid,
                smode,
                &raw mut changed,
                &raw mut notchanged,
                &raw mut notpermitted,
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_SETSCLASS as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, changed);
            put32bit(&raw mut ptr, notchanged);
            put32bit(&raw mut ptr, notpermitted);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_geteattr(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut feattrtab: [uint32_t; 256] = [0; 256];
        let mut deattrtab: [uint32_t; 256] = [0; 256];
        let mut gmode: uint8_t = 0;
        let mut i: uint16_t = 0;
        let mut r#fn: uint16_t = 0;
        let mut dn: uint16_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut knowflags: uint8_t = (if (*eptr).version
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    113 as ::core::ffi::c_int
                })) as uint32_t
            && (*eptr).version
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            || (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if length != 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETEATTR - wrong size (%u/9)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        gmode = get8bit(&raw mut data);
        status = fs_geteattr(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            gmode,
            &raw mut feattrtab as *mut uint32_t,
            &raw mut deattrtab as *mut uint32_t,
        );
        if (*eptr).version
            < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    30 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    30 as ::core::ffi::c_int
                })) as uint32_t
        {
            i = 16 as uint16_t;
            while (i as ::core::ffi::c_int) < (1 as ::core::ffi::c_int) << EATTR_BITS {
                feattrtab[(i as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize] =
                    feattrtab[(i as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize]
                        .wrapping_add(feattrtab[i as usize]);
                feattrtab[i as usize] = 0 as uint32_t;
                deattrtab[(i as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize] =
                    deattrtab[(i as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize]
                        .wrapping_add(deattrtab[i as usize]);
                deattrtab[i as usize] = 0 as uint32_t;
                i = i.wrapping_add(1);
            }
        } else if knowflags as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = 32 as uint16_t;
            while (i as ::core::ffi::c_int) < (1 as ::core::ffi::c_int) << EATTR_BITS {
                feattrtab[(i as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as usize] =
                    feattrtab[(i as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as usize]
                        .wrapping_add(feattrtab[i as usize]);
                feattrtab[i as usize] = 0 as uint32_t;
                deattrtab[(i as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as usize] =
                    deattrtab[(i as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as usize]
                        .wrapping_add(deattrtab[i as usize]);
                deattrtab[i as usize] = 0 as uint32_t;
                i = i.wrapping_add(1);
            }
        }
        r#fn = 0 as uint16_t;
        dn = 0 as uint16_t;
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < (1 as ::core::ffi::c_int) << EATTR_BITS {
                if feattrtab[i as usize] != 0
                    && (r#fn as ::core::ffi::c_int) < 255 as ::core::ffi::c_int
                {
                    r#fn = r#fn.wrapping_add(1);
                } else {
                    feattrtab[i as usize] = 0 as uint32_t;
                }
                if deattrtab[i as usize] != 0
                    && (dn as ::core::ffi::c_int) < 255 as ::core::ffi::c_int
                {
                    dn = dn.wrapping_add(1);
                } else {
                    deattrtab[i as usize] = 0 as uint32_t;
                }
                i = i.wrapping_add(1);
            }
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETEATTR as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                6 as ::core::ffi::c_int
                    + 5 as ::core::ffi::c_int
                        * (r#fn as ::core::ffi::c_int + dn as ::core::ffi::c_int)
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put8bit(&raw mut ptr, r#fn as uint8_t);
            put8bit(&raw mut ptr, dn as uint8_t);
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < (1 as ::core::ffi::c_int) << EATTR_BITS {
                if feattrtab[i as usize] != 0 {
                    put8bit(&raw mut ptr, i as uint8_t);
                    put32bit(&raw mut ptr, feattrtab[i as usize]);
                }
                i = i.wrapping_add(1);
            }
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < (1 as ::core::ffi::c_int) << EATTR_BITS {
                if deattrtab[i as usize] != 0 {
                    put8bit(&raw mut ptr, i as uint8_t);
                    put32bit(&raw mut ptr, deattrtab[i as usize]);
                }
                i = i.wrapping_add(1);
            }
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_seteattr(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut eattr: uint8_t = 0;
        let mut smode: uint8_t = 0;
        let mut changed: uint32_t = 0;
        let mut notchanged: uint32_t = 0;
        let mut notpermitted: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 14 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETEATTR - wrong size (%u/14)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        uid = get32bit(&raw mut data);
        sessions_ugid_remap(
            (*eptr).sesdata,
            &raw mut uid,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        eattr = get8bit(&raw mut data);
        smode = get8bit(&raw mut data);
        if sessions_get_disables((*eptr).sesdata) & DISABLE_SETEATTR as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_seteattr(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                uid,
                eattr,
                smode,
                &raw mut changed,
                &raw mut notchanged,
                &raw mut notpermitted,
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_SETEATTR as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, changed);
            put32bit(&raw mut ptr, notchanged);
            put32bit(&raw mut ptr, notpermitted);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_parents(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut pcount: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_PARENTS - wrong size (%u/8)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        status = fs_get_parents_count(
            sessions_get_rootinode((*eptr).sesdata),
            inode,
            &raw mut pcount,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_PARENTS as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_PARENTS as uint32_t,
                (4 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(pcount)),
            );
            put32bit(&raw mut ptr, msgid);
            fs_get_parents_data(sessions_get_rootinode((*eptr).sesdata), inode, ptr);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_paths(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut psize: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_PATHS - wrong size (%u/8)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        status = fs_get_paths_size(
            sessions_get_rootinode((*eptr).sesdata),
            inode,
            &raw mut psize,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_PATHS as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_PATHS as uint32_t,
                (4 as uint32_t).wrapping_add(psize),
            );
            put32bit(&raw mut ptr, msgid);
            fs_get_paths_data(sessions_get_rootinode((*eptr).sesdata), inode, ptr);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_getxattr(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut opened: uint8_t = 0;
        let mut mode: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut anleng: uint8_t = 0;
        let mut attrname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        if length < 19 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETXATTR - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        opened = 0 as uint8_t;
        gid = ::core::ptr::null_mut::<uint32_t>();
        uid = 0 as uint32_t;
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        if (*eptr).version
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            opened = get8bit(&raw mut data);
            uid = get32bit(&raw mut data);
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
        }
        anleng = get8bit(&raw mut data);
        attrname = data;
        data = data.offset(anleng as ::core::ffi::c_int as isize);
        if length < (19 as uint32_t).wrapping_add(anleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETXATTR - wrong size (%u:anleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                anleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        mode = get8bit(&raw mut data);
        if (*eptr).version
            >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            opened = get8bit(&raw mut data);
            uid = get32bit(&raw mut data);
            if length == (19 as uint32_t).wrapping_add(anleng as uint32_t) {
                gids = 1 as uint32_t;
                gid = matoclserv_gid_storage(gids);
                *gid.offset(0 as isize) = get32bit(&raw mut data);
            } else {
                gids = get32bit(&raw mut data);
                if gids == 0 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_GETXATTR - group ids missing\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if gids > MFS_GIDS_MAX as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_GETXATTR - too many group ids\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if length
                    != (19 as uint32_t)
                        .wrapping_add(anleng as uint32_t)
                        .wrapping_add((4 as uint32_t).wrapping_mul(gids))
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_GETXATTR - wrong size (%u:anleng=%hhu:gids=%u)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                        anleng as ::core::ffi::c_int,
                        gids,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                gid = matoclserv_gid_storage(gids);
                i = 0 as uint32_t;
                while i < gids {
                    *gid.offset(i as isize) = get32bit(&raw mut data);
                    i = i.wrapping_add(1);
                }
            }
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        if mode as ::core::ffi::c_int != MFS_XATTR_GETA_DATA
            && mode as ::core::ffi::c_int != MFS_XATTR_LENGTH_ONLY
        {
            ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_GETXATTR as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, MFS_ERROR_EINVAL as uint8_t);
        } else if anleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut xanode: *mut ::core::ffi::c_void =
                ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut xasize: uint32_t = 0;
            status = fs_listxattr_leng(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                opened,
                uid,
                gids,
                gid,
                &raw mut xanode,
                &raw mut xasize,
            );
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_GETXATTR as uint32_t,
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    5 as uint32_t
                } else {
                    (8 as uint32_t).wrapping_add(
                        if mode as ::core::ffi::c_int == MFS_XATTR_GETA_DATA {
                            xasize
                        } else {
                            0 as uint32_t
                        },
                    )
                },
            );
            put32bit(&raw mut ptr, msgid);
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                put8bit(&raw mut ptr, status);
            } else {
                put32bit(&raw mut ptr, xasize);
                if mode as ::core::ffi::c_int == MFS_XATTR_GETA_DATA && xasize > 0 as uint32_t {
                    fs_listxattr_data(xanode, ptr);
                }
            }
        } else {
            let mut attrvalue: *const uint8_t = ::core::ptr::null::<uint8_t>();
            let mut avleng: uint32_t = 0;
            status = fs_getxattr(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                opened,
                uid,
                gids,
                gid,
                anleng,
                attrname,
                &raw mut avleng,
                &raw mut attrvalue,
            );
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_GETXATTR as uint32_t,
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    5 as uint32_t
                } else {
                    (8 as uint32_t).wrapping_add(
                        if mode as ::core::ffi::c_int == MFS_XATTR_GETA_DATA {
                            avleng
                        } else {
                            0 as uint32_t
                        },
                    )
                },
            );
            put32bit(&raw mut ptr, msgid);
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                put8bit(&raw mut ptr, status);
            } else {
                put32bit(&raw mut ptr, avleng);
                if mode as ::core::ffi::c_int == MFS_XATTR_GETA_DATA && avleng > 0 as uint32_t {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        attrvalue as *const ::core::ffi::c_void,
                        avleng as size_t,
                    );
                }
            }
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_GETXATTR as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_setxattr(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut attrname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut attrvalue: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut opened: uint8_t = 0;
        let mut anleng: uint8_t = 0;
        let mut avleng: uint32_t = 0;
        let mut mode: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 23 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETXATTR - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        opened = 0 as uint8_t;
        uid = 0 as uint32_t;
        gid = ::core::ptr::null_mut::<uint32_t>();
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        if (*eptr).version
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            opened = get8bit(&raw mut data);
            uid = get32bit(&raw mut data);
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
        }
        anleng = get8bit(&raw mut data);
        if length < (23 as uint32_t).wrapping_add(anleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETXATTR - wrong size (%u:anleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                anleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        attrname = data;
        data = data.offset(anleng as ::core::ffi::c_int as isize);
        avleng = get32bit(&raw mut data);
        if avleng > MFS_XATTR_SIZE_MAX as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETXATTR - xattr value too long\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length
            < (23 as uint32_t)
                .wrapping_add(anleng as uint32_t)
                .wrapping_add(avleng)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETXATTR - wrong size (%u:anleng=%hhu:avleng=%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                anleng as ::core::ffi::c_int,
                avleng,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        attrvalue = data;
        data = data.offset(avleng as isize);
        mode = get8bit(&raw mut data);
        if (*eptr).version
            >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            opened = get8bit(&raw mut data);
            uid = get32bit(&raw mut data);
            if length
                == (23 as uint32_t)
                    .wrapping_add(anleng as uint32_t)
                    .wrapping_add(avleng)
            {
                gids = 1 as uint32_t;
                gid = matoclserv_gid_storage(gids);
                *gid.offset(0 as isize) = get32bit(&raw mut data);
            } else {
                gids = get32bit(&raw mut data);
                if gids == 0 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_SETXATTR - group ids missing\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if gids > MFS_GIDS_MAX as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_SETXATTR - too many group ids\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if length
                    != (23 as uint32_t)
                        .wrapping_add(anleng as uint32_t)
                        .wrapping_add(avleng)
                        .wrapping_add((4 as uint32_t).wrapping_mul(gids))
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_SETXATTR - wrong size (%u:anleng=%hhu:avleng=%u:gids=%u)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        length,
                        anleng as ::core::ffi::c_int,
                        avleng,
                        gids,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                gid = matoclserv_gid_storage(gids);
                i = 0 as uint32_t;
                while i < gids {
                    *gid.offset(i as isize) = get32bit(&raw mut data);
                    i = i.wrapping_add(1);
                }
            }
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        if sessions_get_disables((*eptr).sesdata) & DISABLE_SETXATTR as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_setxattr(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                opened,
                uid,
                gids,
                gid,
                anleng,
                attrname,
                avleng,
                attrvalue,
                mode,
            );
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_SETXATTR as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
        sessions_inc_stats((*eptr).sesdata, SES_OP_SETXATTR as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_getfacl(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut acltype: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut c: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut userperm: uint16_t = 0;
        let mut groupperm: uint16_t = 0;
        let mut otherperm: uint16_t = 0;
        let mut mask: uint16_t = 0;
        let mut namedusers: uint16_t = 0;
        let mut namedgroups: uint16_t = 0;
        let mut aclleng: uint32_t = 0;
        if length < 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETFACL - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        acltype = get8bit(&raw mut data);
        if length > 9 as uint32_t {
            if length < 18 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_GETFACL - wrong size (%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            } else if length > 18 as uint32_t {
                let mut gids: uint32_t = 0;
                data = data.offset(5 as ::core::ffi::c_int as isize);
                gids = get32bit(&raw mut data);
                if gids == 0 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_GETFACL - group ids missing\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if length != (18 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)) {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_FUSE_GETFACL - wrong size (%u:gids=%u)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                        gids,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
            }
        }
        status = fs_getfacl_size(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            acltype,
            &raw mut c,
            &raw mut aclleng,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETFACL as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                (16 as uint32_t).wrapping_add(aclleng)
            },
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        } else {
            fs_getfacl_data(
                c,
                &raw mut userperm,
                &raw mut groupperm,
                &raw mut otherperm,
                &raw mut mask,
                &raw mut namedusers,
                &raw mut namedgroups,
                ptr.offset(16 as ::core::ffi::c_int as isize),
            );
            put32bit(&raw mut ptr, msgid);
            put16bit(&raw mut ptr, userperm);
            put16bit(&raw mut ptr, groupperm);
            put16bit(&raw mut ptr, otherperm);
            put16bit(&raw mut ptr, mask);
            put16bit(&raw mut ptr, namedusers);
            put16bit(&raw mut ptr, namedgroups);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_GETFACL as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_setfacl(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut acltype: uint8_t = 0;
        let mut userperm: uint16_t = 0;
        let mut groupperm: uint16_t = 0;
        let mut otherperm: uint16_t = 0;
        let mut mask: uint16_t = 0;
        let mut namedusers: uint16_t = 0;
        let mut namedgroups: uint16_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 25 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETFACL - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        uid = get32bit(&raw mut data);
        sessions_ugid_remap(
            (*eptr).sesdata,
            &raw mut uid,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        acltype = get8bit(&raw mut data);
        userperm = get16bit(&raw mut data);
        groupperm = get16bit(&raw mut data);
        otherperm = get16bit(&raw mut data);
        mask = get16bit(&raw mut data);
        namedusers = get16bit(&raw mut data);
        namedgroups = get16bit(&raw mut data);
        if length
            != ((namedusers as ::core::ffi::c_int + namedgroups as ::core::ffi::c_int) as uint32_t)
                .wrapping_mul(6 as uint32_t)
                .wrapping_add(25 as uint32_t)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETFACL - wrong size (%u:namedusers=%hu:namedgroups=%hu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                namedusers as ::core::ffi::c_int,
                namedgroups as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if sessions_get_disables((*eptr).sesdata) & DISABLE_SETFACL as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_setfacl(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                uid,
                acltype,
                userperm,
                groupperm,
                otherperm,
                mask,
                namedusers,
                namedgroups,
                data,
            );
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_SETFACL as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
        sessions_inc_stats((*eptr).sesdata, SES_OP_SETFACL as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_append_slice(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut inode_src: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut slice_from: uint32_t = 0;
        let mut slice_to: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut fleng: uint64_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut flags: uint8_t = 0;
        if length < 20 as uint32_t
            || length & 1 as uint32_t == 1 as uint32_t && length < 29 as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_APPEND_SLICE - wrong size (%u/20+|29+)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if length & 1 as uint32_t != 0 {
            flags = get8bit(&raw mut data);
        } else {
            flags = 0 as uint8_t;
        }
        inode = get32bit(&raw mut data);
        inode_src = get32bit(&raw mut data);
        if length & 1 as uint32_t != 0 {
            slice_from = get32bit(&raw mut data);
            slice_to = get32bit(&raw mut data);
        } else {
            slice_from = 0 as uint32_t;
            slice_to = 0 as uint32_t;
        }
        uid = get32bit(&raw mut data);
        if length == 20 as uint32_t {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_APPEND_SLICE - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_APPEND_SLICE - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length != (20 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids))
                && length != (29 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_APPEND_SLICE - wrong size (%u:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        if sessions_get_disables((*eptr).sesdata) & DISABLE_APPENDCHUNKS as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        } else {
            status = fs_append_slice(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                flags,
                inode,
                inode_src,
                slice_from,
                slice_to,
                uid,
                gids,
                gid,
                &raw mut fleng,
            );
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            matoclserv_fuse_fleng_has_changed(
                ::core::ptr::null_mut::<matoclserventry>(),
                inode,
                fleng,
            );
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_APPEND_SLICE as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
        sessions_inc_stats((*eptr).sesdata, SES_OP_SNAPSHOT as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_snapshot(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut inode_dst: uint32_t = 0;
        let mut nleng_dst: uint8_t = 0;
        let mut name_dst: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut i: uint32_t = 0;
        let mut smode: uint8_t = 0;
        let mut requmask: uint16_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 22 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SNAPSHOT - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        inode_dst = get32bit(&raw mut data);
        nleng_dst = get8bit(&raw mut data);
        if length != (22 as uint32_t).wrapping_add(nleng_dst as uint32_t)
            && length < (24 as uint32_t).wrapping_add(nleng_dst as uint32_t)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SNAPSHOT - wrong size (%u:nleng_dst=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng_dst as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        name_dst = data;
        data = data.offset(nleng_dst as ::core::ffi::c_int as isize);
        uid = get32bit(&raw mut data);
        if length <= (24 as uint32_t).wrapping_add(nleng_dst as uint32_t) {
            gids = 1 as uint32_t;
            gid = matoclserv_gid_storage(gids);
            *gid.offset(0 as isize) = get32bit(&raw mut data);
        } else {
            gids = get32bit(&raw mut data);
            if gids == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SNAPSHOT - group ids missing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if gids > MFS_GIDS_MAX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SNAPSHOT - too many group ids\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if length
                != (24 as uint32_t)
                    .wrapping_add(nleng_dst as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_SNAPSHOT - wrong size (%u:nleng_dst=%hhu:gids=%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng_dst as ::core::ffi::c_int,
                    gids,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gid = matoclserv_gid_storage(gids);
            i = 0 as uint32_t;
            while i < gids {
                *gid.offset(i as isize) = get32bit(&raw mut data);
                i = i.wrapping_add(1);
            }
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        smode = get8bit(&raw mut data);
        if length >= (24 as uint32_t).wrapping_add(nleng_dst as uint32_t) {
            requmask = get16bit(&raw mut data);
        } else {
            smode = (smode as ::core::ffi::c_int & !SNAPSHOT_MODE_CPLIKE_ATTR) as uint8_t;
            requmask = 0 as uint16_t;
        }
        requmask = (requmask as ::core::ffi::c_int
            | sessions_get_umask((*eptr).sesdata) as ::core::ffi::c_int)
            as uint16_t;
        status = MFS_STATUS_OK as uint8_t;
        if smode as ::core::ffi::c_int & SNAPSHOT_MODE_DELETE != 0 {
            if sessions_get_disables((*eptr).sesdata)
                & (DISABLE_UNLINK as uint32_t | DISABLE_RMDIR as uint32_t)
                != 0
            {
                status = MFS_ERROR_EPERM as uint8_t;
            }
        } else if sessions_get_disables((*eptr).sesdata) & DISABLE_SNAPSHOT as uint32_t != 0 {
            status = MFS_ERROR_EPERM as uint8_t;
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            status = fs_snapshot(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                inode_dst,
                nleng_dst as uint16_t,
                name_dst,
                uid,
                gids,
                gid,
                smode,
                requmask,
            );
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_SNAPSHOT as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
        sessions_inc_stats((*eptr).sesdata, SES_OP_SNAPSHOT as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_quotacontrol(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut flags: uint8_t = 0;
        let mut del: uint8_t = 0;
        let mut sinodes: uint32_t = 0;
        let mut hinodes: uint32_t = 0;
        let mut curinodes: uint32_t = 0;
        let mut slength: uint64_t = 0;
        let mut ssize: uint64_t = 0;
        let mut srealsize: uint64_t = 0;
        let mut hlength: uint64_t = 0;
        let mut hsize: uint64_t = 0;
        let mut hrealsize: uint64_t = 0;
        let mut curlength: uint64_t = 0;
        let mut cursize: uint64_t = 0;
        let mut currealsize: uint64_t = 0;
        let mut msgid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut graceperiod: uint32_t = 0;
        let mut defaultgp: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 65 as uint32_t && length != 69 as uint32_t && length != 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_QUOTACONTROL - wrong size (%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        flags = get8bit(&raw mut data);
        if length == 65 as uint32_t || length == 69 as uint32_t {
            if length == 69 as uint32_t {
                graceperiod = get32bit(&raw mut data);
            } else {
                graceperiod = 0 as uint32_t;
            }
            sinodes = get32bit(&raw mut data);
            slength = get64bit(&raw mut data);
            ssize = get64bit(&raw mut data);
            srealsize = get64bit(&raw mut data);
            hinodes = get32bit(&raw mut data);
            hlength = get64bit(&raw mut data);
            hsize = get64bit(&raw mut data);
            hrealsize = get64bit(&raw mut data);
            del = 0 as uint8_t;
        } else {
            graceperiod = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            sinodes = 0 as uint32_t;
            slength = 0 as uint64_t;
            ssize = 0 as uint64_t;
            srealsize = 0 as uint64_t;
            hinodes = 0 as uint32_t;
            hlength = 0 as uint64_t;
            hsize = 0 as uint64_t;
            hrealsize = 0 as uint64_t;
            del = 1 as uint8_t;
        }
        if flags as ::core::ffi::c_int != 0
            && sessions_is_root_remapped((*eptr).sesdata) as ::core::ffi::c_int != 0
        {
            status = MFS_ERROR_EACCES as uint8_t;
        } else {
            status = fs_quotacontrol(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                del,
                &raw mut flags,
                &raw mut defaultgp,
                &raw mut graceperiod,
                &raw mut sinodes,
                &raw mut slength,
                &raw mut ssize,
                &raw mut srealsize,
                &raw mut hinodes,
                &raw mut hlength,
                &raw mut hsize,
                &raw mut hrealsize,
                &raw mut curinodes,
                &raw mut curlength,
                &raw mut cursize,
                &raw mut currealsize,
            );
        }
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_QUOTACONTROL as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else if (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 51 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                94 as ::core::ffi::c_int
            } else if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        9 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        9 as ::core::ffi::c_int
                    })) as uint32_t
            {
                93 as ::core::ffi::c_int
            } else {
                89 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put8bit(&raw mut ptr, flags);
            if (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 51 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                put8bit(&raw mut ptr, defaultgp);
            }
            if (*eptr).version
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        9 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        9 as ::core::ffi::c_int
                    })) as uint32_t
            {
                put32bit(&raw mut ptr, graceperiod);
            }
            put32bit(&raw mut ptr, sinodes);
            put64bit(&raw mut ptr, slength);
            put64bit(&raw mut ptr, ssize);
            put64bit(&raw mut ptr, srealsize);
            put32bit(&raw mut ptr, hinodes);
            put64bit(&raw mut ptr, hlength);
            put64bit(&raw mut ptr, hsize);
            put64bit(&raw mut ptr, hrealsize);
            put32bit(&raw mut ptr, curinodes);
            put64bit(&raw mut ptr, curlength);
            put64bit(&raw mut ptr, cursize);
            put64bit(&raw mut ptr, currealsize);
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_archctl(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut cmd: uint8_t = 0;
        let mut status: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 13 as uint32_t && length != 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_ARCHCTL - wrong size (%u/9|13)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        cmd = get8bit(&raw mut data);
        if cmd as ::core::ffi::c_int == ARCHCTL_GET {
            let mut archinodes: uint32_t = 0;
            let mut partinodes: uint32_t = 0;
            let mut notarchinodes: uint32_t = 0;
            let mut archchunks: uint64_t = 0;
            let mut notarchchunks: uint64_t = 0;
            if length != 9 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_ARCHCTL (GET) - wrong size (%u/9)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            status = fs_archget(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                inode,
                &raw mut archchunks,
                &raw mut notarchchunks,
                &raw mut archinodes,
                &raw mut partinodes,
                &raw mut notarchinodes,
            );
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_ARCHCTL as uint32_t,
                (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    5 as ::core::ffi::c_int
                } else {
                    32 as ::core::ffi::c_int
                }) as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                put8bit(&raw mut ptr, status);
            } else {
                put64bit(&raw mut ptr, archchunks);
                put64bit(&raw mut ptr, notarchchunks);
                put32bit(&raw mut ptr, archinodes);
                put32bit(&raw mut ptr, partinodes);
                put32bit(&raw mut ptr, notarchinodes);
            }
        } else {
            let mut uid: uint32_t = 0;
            let mut changed: uint64_t = 0;
            let mut notchanged: uint64_t = 0;
            let mut notpermitted: uint32_t = 0;
            if length != 13 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_FUSE_ARCHCTL (SET/CLR) - wrong size (%u/13)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            uid = get32bit(&raw mut data);
            sessions_ugid_remap(
                (*eptr).sesdata,
                &raw mut uid,
                ::core::ptr::null_mut::<uint32_t>(),
            );
            if sessions_get_disables((*eptr).sesdata) & DISABLE_SETEATTR as uint32_t != 0 {
                status = MFS_ERROR_EPERM as uint8_t;
            } else {
                status = fs_archchg(
                    sessions_get_rootinode((*eptr).sesdata),
                    sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                    inode,
                    uid,
                    cmd,
                    &raw mut changed,
                    &raw mut notchanged,
                    &raw mut notpermitted,
                );
            }
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_FUSE_ARCHCTL as uint32_t,
                (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    5 as ::core::ffi::c_int
                } else {
                    24 as ::core::ffi::c_int
                }) as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                put8bit(&raw mut ptr, status);
            } else {
                put64bit(&raw mut ptr, changed);
                put64bit(&raw mut ptr, notchanged);
                put32bit(&raw mut ptr, notpermitted);
            }
        }
        sessions_inc_stats((*eptr).sesdata, SES_OP_META as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_sclass_create(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut dleng: uint8_t = 0;
        let mut constleng: uint8_t = 0;
        let mut fver: uint8_t = 0;
        let mut i: uint8_t = 0;
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
        let mut labels_mode: uint8_t = 0;
        let mut export_group: uint8_t = 0;
        let mut priority: uint32_t = 0;
        let mut arch_mode: uint8_t = 0;
        let mut arch_delay: uint16_t = 0;
        let mut min_trashretention: uint16_t = 0;
        let mut arch_min_size: uint64_t = 0;
        let mut admin_only: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut desc: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SCLASS_CREATE - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_ADMIN as uint32_t != 0 {
            nleng = get8bit(&raw mut data);
            name = data;
            data = data.offset(nleng as ::core::ffi::c_int as isize);
            if length < (6 as uint32_t).wrapping_add(nleng as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_SCLASS_CREATE - wrong size (%u:nleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            fver = get8bit(&raw mut data);
            if fver as ::core::ffi::c_int <= 6 as ::core::ffi::c_int {
                constleng = (if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                    53 as ::core::ffi::c_int
                } else if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                    47 as ::core::ffi::c_int
                } else if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                    43 as ::core::ffi::c_int
                } else if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                    35 as ::core::ffi::c_int
                } else if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                    34 as ::core::ffi::c_int
                } else {
                    13 as ::core::ffi::c_int
                }) as uint8_t;
                if length
                    < (constleng as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_SCLASS_CREATE/%hhu - wrong size (%u:nleng=%hhu)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        fver as ::core::ffi::c_int,
                        length,
                        nleng as ::core::ffi::c_int,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
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
                if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                    dleng = get8bit(&raw mut data);
                } else {
                    dleng = 0 as uint8_t;
                }
                if length
                    < (constleng as ::core::ffi::c_int
                        + nleng as ::core::ffi::c_int
                        + dleng as ::core::ffi::c_int) as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_SCLASS_CREATE/%hhu - wrong size (%u:nleng=%hhu:dleng=%hhu)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        fver as ::core::ffi::c_int,
                        length,
                        nleng as ::core::ffi::c_int,
                        dleng as ::core::ffi::c_int,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                    desc = data;
                    data = data.offset(dleng as ::core::ffi::c_int as isize);
                    priority = get32bit(&raw mut data);
                    export_group = get8bit(&raw mut data);
                } else {
                    desc = name;
                    priority = 0 as uint32_t;
                    export_group = 0 as uint8_t;
                }
                admin_only = get8bit(&raw mut data);
                labels_mode = get8bit(&raw mut data);
                if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                    arch_mode = get8bit(&raw mut data);
                } else {
                    arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
                }
                arch_delay = get16bit(&raw mut data);
                if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                    arch_min_size = get64bit(&raw mut data);
                } else {
                    arch_min_size = 0 as uint64_t;
                }
                if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                    min_trashretention = get16bit(&raw mut data);
                    arch.ec_data_chksum_parts = get8bit(&raw mut data);
                    trash.ec_data_chksum_parts = get8bit(&raw mut data);
                    if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                        create.labels_mode = get8bit(&raw mut data);
                        keep.labels_mode = get8bit(&raw mut data);
                        arch.labels_mode = get8bit(&raw mut data);
                        trash.labels_mode = get8bit(&raw mut data);
                    } else {
                        create.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
                        keep.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
                        arch.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
                        trash.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
                    }
                    create.uniqmask = get32bit(&raw mut data);
                    keep.uniqmask = get32bit(&raw mut data);
                    arch.uniqmask = get32bit(&raw mut data);
                    trash.uniqmask = get32bit(&raw mut data);
                } else {
                    min_trashretention = 0 as uint16_t;
                }
                create.labelscnt = get8bit(&raw mut data);
                keep.labelscnt = get8bit(&raw mut data);
                arch.labelscnt = get8bit(&raw mut data);
                if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                    trash.labelscnt = get8bit(&raw mut data);
                    if create.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                        || keep.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                        || arch.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                        || trash.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_SCLASS_CREATE/%hhu - wrong label count C=%hhu;K=%hhu;A=%hhu;T=%hhu\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            fver as ::core::ffi::c_int,
                            create.labelscnt as ::core::ffi::c_int,
                            keep.labelscnt as ::core::ffi::c_int,
                            arch.labelscnt as ::core::ffi::c_int,
                            trash.labelscnt as ::core::ffi::c_int,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    if length
                        != (constleng as ::core::ffi::c_int
                            + nleng as ::core::ffi::c_int
                            + dleng as ::core::ffi::c_int
                            + (create.labelscnt as ::core::ffi::c_int
                                + keep.labelscnt as ::core::ffi::c_int
                                + arch.labelscnt as ::core::ffi::c_int
                                + trash.labelscnt as ::core::ffi::c_int)
                                * SCLASS_EXPR_MAX_SIZE) as uint32_t
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_SCLASS_CREATE/%hhu - wrong size (%u:nleng=%hhu:dleng=%hhu:labels C=%hhu;K=%hhu;A=%hhu;T=%hhu)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            fver as ::core::ffi::c_int,
                            length,
                            nleng as ::core::ffi::c_int,
                            dleng as ::core::ffi::c_int,
                            create.labelscnt as ::core::ffi::c_int,
                            keep.labelscnt as ::core::ffi::c_int,
                            arch.labelscnt as ::core::ffi::c_int,
                            trash.labelscnt as ::core::ffi::c_int,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < create.labelscnt as ::core::ffi::c_int {
                        memcpy(
                            &raw mut *(&raw mut create.labelexpr as *mut [uint8_t; 128])
                                .offset(i as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            SCLASS_EXPR_MAX_SIZE as size_t,
                        );
                        data = data.offset(SCLASS_EXPR_MAX_SIZE as isize);
                        i = i.wrapping_add(1);
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int {
                        memcpy(
                            &raw mut *(&raw mut keep.labelexpr as *mut [uint8_t; 128])
                                .offset(i as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            SCLASS_EXPR_MAX_SIZE as size_t,
                        );
                        data = data.offset(SCLASS_EXPR_MAX_SIZE as isize);
                        i = i.wrapping_add(1);
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < arch.labelscnt as ::core::ffi::c_int {
                        memcpy(
                            &raw mut *(&raw mut arch.labelexpr as *mut [uint8_t; 128])
                                .offset(i as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            SCLASS_EXPR_MAX_SIZE as size_t,
                        );
                        data = data.offset(SCLASS_EXPR_MAX_SIZE as isize);
                        i = i.wrapping_add(1);
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < trash.labelscnt as ::core::ffi::c_int {
                        memcpy(
                            &raw mut *(&raw mut trash.labelexpr as *mut [uint8_t; 128])
                                .offset(i as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            SCLASS_EXPR_MAX_SIZE as size_t,
                        );
                        data = data.offset(SCLASS_EXPR_MAX_SIZE as isize);
                        i = i.wrapping_add(1);
                    }
                } else {
                    if create.labelscnt as ::core::ffi::c_int > 9 as ::core::ffi::c_int
                        || keep.labelscnt as ::core::ffi::c_int > 9 as ::core::ffi::c_int
                        || arch.labelscnt as ::core::ffi::c_int > 9 as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_SCLASS_CREATE/%hhu - wrong label count C=%hhu;K=%hhu;A=%hhu\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            fver as ::core::ffi::c_int,
                            create.labelscnt as ::core::ffi::c_int,
                            keep.labelscnt as ::core::ffi::c_int,
                            arch.labelscnt as ::core::ffi::c_int,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    arch_delay =
                        (arch_delay as ::core::ffi::c_int * 24 as ::core::ffi::c_int) as uint16_t;
                    trash.labelscnt = 0 as uint8_t;
                    if length
                        != ((constleng as ::core::ffi::c_int + nleng as ::core::ffi::c_int)
                            as uint32_t)
                            .wrapping_add(
                                ((create.labelscnt as ::core::ffi::c_int
                                    + keep.labelscnt as ::core::ffi::c_int
                                    + arch.labelscnt as ::core::ffi::c_int)
                                    as uint32_t)
                                    .wrapping_mul(4 as uint32_t)
                                    .wrapping_mul(MASKORGROUP as uint32_t),
                            )
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_SCLASS_CREATE/%hhu - wrong size (%u:nleng=%hhu:labels C=%hhu;K=%hhu;A=%hhu)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            fver as ::core::ffi::c_int,
                            length,
                            nleng as ::core::ffi::c_int,
                            create.labelscnt as ::core::ffi::c_int,
                            keep.labelscnt as ::core::ffi::c_int,
                            arch.labelscnt as ::core::ffi::c_int,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int)
                        < create.labelscnt as ::core::ffi::c_int * MASKORGROUP
                    {
                        old_labelmasks[i as usize] = get32bit(&raw mut data);
                        i = i.wrapping_add(1);
                    }
                    sclass_maskorgroup_to_labelexpr(
                        &raw mut create.labelexpr as *mut [uint8_t; 128],
                        &raw mut old_labelmasks as *mut uint32_t,
                        create.labelscnt,
                    );
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int)
                        < keep.labelscnt as ::core::ffi::c_int * MASKORGROUP
                    {
                        old_labelmasks[i as usize] = get32bit(&raw mut data);
                        i = i.wrapping_add(1);
                    }
                    sclass_maskorgroup_to_labelexpr(
                        &raw mut keep.labelexpr as *mut [uint8_t; 128],
                        &raw mut old_labelmasks as *mut uint32_t,
                        keep.labelscnt,
                    );
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int)
                        < arch.labelscnt as ::core::ffi::c_int * MASKORGROUP
                    {
                        old_labelmasks[i as usize] = get32bit(&raw mut data);
                        i = i.wrapping_add(1);
                    }
                    sclass_maskorgroup_to_labelexpr(
                        &raw mut arch.labelexpr as *mut [uint8_t; 128],
                        &raw mut old_labelmasks as *mut uint32_t,
                        arch.labelscnt,
                    );
                }
                if arch.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                    > 1 as ::core::ffi::c_int
                    || trash.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                        > 1 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"CLTOMA_SCLASS_CREATE - redundancy levels > 1 supported only in pro version\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    status = MFS_ERROR_EINVAL as uint8_t;
                } else {
                    status = sclass_create_entry(
                        nleng,
                        name,
                        dleng,
                        desc,
                        priority,
                        export_group,
                        admin_only,
                        labels_mode,
                        arch_mode,
                        arch_delay,
                        arch_min_size,
                        min_trashretention,
                        &raw mut create,
                        &raw mut keep,
                        &raw mut arch,
                        &raw mut trash,
                    );
                }
            } else {
                status = MFS_ERROR_EINVAL as uint8_t;
            }
        } else {
            status = MFS_ERROR_EPERM_NOTADMIN as uint8_t;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_SCLASS_CREATE as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_sclass_change(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut dleng: uint8_t = 0;
        let mut constleng: uint8_t = 0;
        let mut fver: uint8_t = 0;
        let mut i: uint8_t = 0;
        let mut chgmask: uint16_t = 0;
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
        let mut create_labelmasks: [uint32_t; 36] = [0; 36];
        let mut keep_labelmasks: [uint32_t; 36] = [0; 36];
        let mut arch_labelmasks: [uint32_t; 36] = [0; 36];
        let mut labels_mode: uint8_t = 0;
        let mut export_group: uint8_t = 0;
        let mut priority: uint32_t = 0;
        let mut arch_mode: uint8_t = 0;
        let mut arch_delay: uint16_t = 0;
        let mut arch_min_size: uint64_t = 0;
        let mut min_trashretention: uint16_t = 0;
        let mut admin_only: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut desc: [uint8_t; 256] = [0; 256];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        labels_mode = 0 as uint8_t;
        arch_mode = 0 as uint8_t;
        arch_delay = 0 as uint16_t;
        arch_min_size = 0 as uint64_t;
        min_trashretention = 0 as uint16_t;
        admin_only = 0 as uint8_t;
        export_group = 0 as uint8_t;
        dleng = 0 as uint8_t;
        priority = 0 as uint32_t;
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SCLASS_CHANGE - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        name = data;
        data = data.offset(nleng as ::core::ffi::c_int as isize);
        if length < (6 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SCLASS_CHANGE - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        fver = get8bit(&raw mut data);
        if fver as ::core::ffi::c_int <= 6 as ::core::ffi::c_int {
            constleng = (if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                55 as ::core::ffi::c_int
            } else if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                49 as ::core::ffi::c_int
            } else if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                45 as ::core::ffi::c_int
            } else if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                37 as ::core::ffi::c_int
            } else if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                36 as ::core::ffi::c_int
            } else {
                15 as ::core::ffi::c_int
            }) as uint8_t;
            if length < (constleng as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_SCLASS_CHANGE/%hhu - wrong size (%u:nleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    fver as ::core::ffi::c_int,
                    length,
                    nleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
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
            chgmask = get16bit(&raw mut data);
            if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_ADMIN as uint32_t != 0
                || chgmask as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                    dleng = get8bit(&raw mut data);
                } else {
                    dleng = 0 as uint8_t;
                }
                if length
                    < (constleng as ::core::ffi::c_int
                        + nleng as ::core::ffi::c_int
                        + dleng as ::core::ffi::c_int) as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CLTOMA_SCLASS_CHANGE/%hhu - wrong size (%u:nleng=%hhu:dleng=%hhu)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        fver as ::core::ffi::c_int,
                        length,
                        nleng as ::core::ffi::c_int,
                        dleng as ::core::ffi::c_int,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                    memcpy(
                        &raw mut desc as *mut uint8_t as *mut ::core::ffi::c_void,
                        data as *const ::core::ffi::c_void,
                        dleng as size_t,
                    );
                    data = data.offset(dleng as ::core::ffi::c_int as isize);
                    priority = get32bit(&raw mut data);
                    export_group = get8bit(&raw mut data);
                } else {
                    dleng = 0 as uint8_t;
                    priority = 0 as uint32_t;
                    export_group = 0 as uint8_t;
                }
                admin_only = get8bit(&raw mut data);
                labels_mode = get8bit(&raw mut data);
                if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                    arch_mode = get8bit(&raw mut data);
                } else {
                    arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
                }
                arch_delay = get16bit(&raw mut data);
                if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                    arch_min_size = get64bit(&raw mut data);
                } else {
                    arch_min_size = 0 as uint64_t;
                }
                if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                    min_trashretention = get16bit(&raw mut data);
                    arch.ec_data_chksum_parts = get8bit(&raw mut data);
                    trash.ec_data_chksum_parts = get8bit(&raw mut data);
                    if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                        create.labels_mode = get8bit(&raw mut data);
                        keep.labels_mode = get8bit(&raw mut data);
                        arch.labels_mode = get8bit(&raw mut data);
                        trash.labels_mode = get8bit(&raw mut data);
                    } else {
                        create.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
                        keep.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
                        arch.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
                        trash.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
                    }
                    create.uniqmask = get32bit(&raw mut data);
                    keep.uniqmask = get32bit(&raw mut data);
                    arch.uniqmask = get32bit(&raw mut data);
                    trash.uniqmask = get32bit(&raw mut data);
                }
                create.labelscnt = get8bit(&raw mut data);
                keep.labelscnt = get8bit(&raw mut data);
                arch.labelscnt = get8bit(&raw mut data);
                if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                    trash.labelscnt = get8bit(&raw mut data);
                    if create.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                        || keep.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                        || arch.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                        || trash.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_SCLASS_CHANGE/%hhu - wrong label count C=%hhu;K=%hhu;A=%hhu;T=%hhu\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            fver as ::core::ffi::c_int,
                            create.labelscnt as ::core::ffi::c_int,
                            keep.labelscnt as ::core::ffi::c_int,
                            arch.labelscnt as ::core::ffi::c_int,
                            trash.labelscnt as ::core::ffi::c_int,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    if length
                        != (constleng as ::core::ffi::c_int
                            + nleng as ::core::ffi::c_int
                            + dleng as ::core::ffi::c_int
                            + (create.labelscnt as ::core::ffi::c_int
                                + keep.labelscnt as ::core::ffi::c_int
                                + arch.labelscnt as ::core::ffi::c_int
                                + trash.labelscnt as ::core::ffi::c_int)
                                * SCLASS_EXPR_MAX_SIZE) as uint32_t
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_SCLASS_CHANGE/%hhu - wrong size (%u:nleng=%hhu:dleng=%hhu:labels C=%hhu;K=%hhu;A=%hhu;T=%hhu)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            fver as ::core::ffi::c_int,
                            length,
                            nleng as ::core::ffi::c_int,
                            dleng as ::core::ffi::c_int,
                            create.labelscnt as ::core::ffi::c_int,
                            keep.labelscnt as ::core::ffi::c_int,
                            arch.labelscnt as ::core::ffi::c_int,
                            trash.labelscnt as ::core::ffi::c_int,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < create.labelscnt as ::core::ffi::c_int {
                        memcpy(
                            &raw mut *(&raw mut create.labelexpr as *mut [uint8_t; 128])
                                .offset(i as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            SCLASS_EXPR_MAX_SIZE as size_t,
                        );
                        data = data.offset(SCLASS_EXPR_MAX_SIZE as isize);
                        i = i.wrapping_add(1);
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int {
                        memcpy(
                            &raw mut *(&raw mut keep.labelexpr as *mut [uint8_t; 128])
                                .offset(i as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            SCLASS_EXPR_MAX_SIZE as size_t,
                        );
                        data = data.offset(SCLASS_EXPR_MAX_SIZE as isize);
                        i = i.wrapping_add(1);
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < arch.labelscnt as ::core::ffi::c_int {
                        memcpy(
                            &raw mut *(&raw mut arch.labelexpr as *mut [uint8_t; 128])
                                .offset(i as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            SCLASS_EXPR_MAX_SIZE as size_t,
                        );
                        data = data.offset(SCLASS_EXPR_MAX_SIZE as isize);
                        i = i.wrapping_add(1);
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int) < trash.labelscnt as ::core::ffi::c_int {
                        memcpy(
                            &raw mut *(&raw mut trash.labelexpr as *mut [uint8_t; 128])
                                .offset(i as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            data as *const ::core::ffi::c_void,
                            SCLASS_EXPR_MAX_SIZE as size_t,
                        );
                        data = data.offset(SCLASS_EXPR_MAX_SIZE as isize);
                        i = i.wrapping_add(1);
                    }
                } else {
                    if create.labelscnt as ::core::ffi::c_int > 9 as ::core::ffi::c_int
                        || keep.labelscnt as ::core::ffi::c_int > 9 as ::core::ffi::c_int
                        || arch.labelscnt as ::core::ffi::c_int > 9 as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_SCLASS_CHANGE/%hhu - wrong label count C=%hhu;K=%hhu;A=%hhu\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            fver as ::core::ffi::c_int,
                            create.labelscnt as ::core::ffi::c_int,
                            keep.labelscnt as ::core::ffi::c_int,
                            arch.labelscnt as ::core::ffi::c_int,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    arch_delay =
                        (arch_delay as ::core::ffi::c_int * 24 as ::core::ffi::c_int) as uint16_t;
                    if length
                        != ((constleng as ::core::ffi::c_int + nleng as ::core::ffi::c_int)
                            as uint32_t)
                            .wrapping_add(
                                ((create.labelscnt as ::core::ffi::c_int
                                    + keep.labelscnt as ::core::ffi::c_int
                                    + arch.labelscnt as ::core::ffi::c_int)
                                    as uint32_t)
                                    .wrapping_mul(4 as uint32_t)
                                    .wrapping_mul(MASKORGROUP as uint32_t),
                            )
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"CLTOMA_SCLASS_CHANGE/%hhu - wrong size (%u:nleng=%hhu:labels C=%hhu;K=%hhu;A=%hhu)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            fver as ::core::ffi::c_int,
                            length,
                            nleng as ::core::ffi::c_int,
                            create.labelscnt as ::core::ffi::c_int,
                            keep.labelscnt as ::core::ffi::c_int,
                            arch.labelscnt as ::core::ffi::c_int,
                        );
                        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int)
                        < create.labelscnt as ::core::ffi::c_int * MASKORGROUP
                    {
                        create_labelmasks[i as usize] = get32bit(&raw mut data);
                        i = i.wrapping_add(1);
                    }
                    sclass_maskorgroup_to_labelexpr(
                        &raw mut create.labelexpr as *mut [uint8_t; 128],
                        &raw mut create_labelmasks as *mut uint32_t,
                        create.labelscnt,
                    );
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int)
                        < keep.labelscnt as ::core::ffi::c_int * MASKORGROUP
                    {
                        keep_labelmasks[i as usize] = get32bit(&raw mut data);
                        i = i.wrapping_add(1);
                    }
                    sclass_maskorgroup_to_labelexpr(
                        &raw mut keep.labelexpr as *mut [uint8_t; 128],
                        &raw mut keep_labelmasks as *mut uint32_t,
                        keep.labelscnt,
                    );
                    i = 0 as uint8_t;
                    while (i as ::core::ffi::c_int)
                        < arch.labelscnt as ::core::ffi::c_int * MASKORGROUP
                    {
                        arch_labelmasks[i as usize] = get32bit(&raw mut data);
                        i = i.wrapping_add(1);
                    }
                    sclass_maskorgroup_to_labelexpr(
                        &raw mut arch.labelexpr as *mut [uint8_t; 128],
                        &raw mut arch_labelmasks as *mut uint32_t,
                        arch.labelscnt,
                    );
                }
                if arch.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                    > 1 as ::core::ffi::c_int
                    || trash.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                        > 1 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"CLTOMA_SCLASS_CHANGE - redundancy levels > 1 supported only in pro version\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    status = MFS_ERROR_EINVAL as uint8_t;
                } else {
                    status = sclass_change_entry(
                        nleng,
                        name,
                        chgmask,
                        &raw mut dleng,
                        &raw mut desc as *mut uint8_t,
                        &raw mut priority,
                        &raw mut export_group,
                        &raw mut admin_only,
                        &raw mut labels_mode,
                        &raw mut arch_mode,
                        &raw mut arch_delay,
                        &raw mut arch_min_size,
                        &raw mut min_trashretention,
                        &raw mut create,
                        &raw mut keep,
                        &raw mut arch,
                        &raw mut trash,
                    );
                }
            } else {
                status = MFS_ERROR_EPERM_NOTADMIN as uint8_t;
            }
        } else {
            status = MFS_ERROR_EINVAL as uint8_t;
        }
        if fver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if status as ::core::ffi::c_int == MFS_STATUS_OK {
                if trash.labelscnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    status = MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
                if sclass_labelexpr_to_maskorgroup(
                    &raw mut create_labelmasks as *mut uint32_t,
                    &raw mut create.labelexpr as *mut [uint8_t; 128],
                    create.labelscnt,
                ) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    status = MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
                if sclass_labelexpr_to_maskorgroup(
                    &raw mut keep_labelmasks as *mut uint32_t,
                    &raw mut keep.labelexpr as *mut [uint8_t; 128],
                    keep.labelscnt,
                ) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    status = MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
                if sclass_labelexpr_to_maskorgroup(
                    &raw mut arch_labelmasks as *mut uint32_t,
                    &raw mut arch.labelexpr as *mut [uint8_t; 128],
                    arch.labelscnt,
                ) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    status = MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
                if arch_delay as ::core::ffi::c_int % 24 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    status = MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
                if (arch.ec_data_chksum_parts as ::core::ffi::c_int
                    | trash.ec_data_chksum_parts as ::core::ffi::c_int)
                    as uint32_t
                    | create.uniqmask
                    | keep.uniqmask
                    | arch.uniqmask
                    | trash.uniqmask
                    | min_trashretention as uint32_t
                    > 0 as uint32_t
                {
                    status = MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
            }
            constleng = 12 as uint8_t;
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_SCLASS_CHANGE as uint32_t,
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    5 as uint32_t
                } else {
                    (constleng as uint32_t).wrapping_add(
                        (4 as uint32_t)
                            .wrapping_mul(MASKORGROUP as uint32_t)
                            .wrapping_mul(
                                (create.labelscnt as ::core::ffi::c_int
                                    + keep.labelscnt as ::core::ffi::c_int
                                    + arch.labelscnt as ::core::ffi::c_int)
                                    as uint32_t,
                            ),
                    )
                },
            );
        } else {
            constleng = (if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                52 as ::core::ffi::c_int
            } else if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                46 as ::core::ffi::c_int
            } else if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                42 as ::core::ffi::c_int
            } else if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                34 as ::core::ffi::c_int
            } else {
                33 as ::core::ffi::c_int
            }) as uint8_t;
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_SCLASS_CHANGE as uint32_t,
                (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    5 as ::core::ffi::c_int
                } else {
                    constleng as ::core::ffi::c_int
                        + dleng as ::core::ffi::c_int
                        + SCLASS_EXPR_MAX_SIZE
                            * (create.labelscnt as ::core::ffi::c_int
                                + keep.labelscnt as ::core::ffi::c_int
                                + arch.labelscnt as ::core::ffi::c_int
                                + trash.labelscnt as ::core::ffi::c_int)
                }) as uint32_t,
            );
        }
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put8bit(&raw mut ptr, fver);
            if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                put8bit(&raw mut ptr, dleng);
                if dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut desc as *mut uint8_t as *const ::core::ffi::c_void,
                        dleng as size_t,
                    );
                    ptr = ptr.offset(dleng as ::core::ffi::c_int as isize);
                }
                put32bit(&raw mut ptr, priority);
                put8bit(&raw mut ptr, export_group);
            }
            put8bit(&raw mut ptr, admin_only);
            put8bit(&raw mut ptr, labels_mode);
            if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                put8bit(&raw mut ptr, arch_mode);
            }
            if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                put16bit(&raw mut ptr, arch_delay);
                if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                    put64bit(&raw mut ptr, arch_min_size);
                }
                put16bit(&raw mut ptr, min_trashretention);
                put8bit(&raw mut ptr, arch.ec_data_chksum_parts);
                put8bit(&raw mut ptr, trash.ec_data_chksum_parts);
                if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                    put8bit(&raw mut ptr, create.labels_mode);
                    put8bit(&raw mut ptr, keep.labels_mode);
                    put8bit(&raw mut ptr, arch.labels_mode);
                    put8bit(&raw mut ptr, trash.labels_mode);
                }
                put32bit(&raw mut ptr, create.uniqmask);
                put32bit(&raw mut ptr, keep.uniqmask);
                put32bit(&raw mut ptr, arch.uniqmask);
                put32bit(&raw mut ptr, trash.uniqmask);
            } else {
                put16bit(
                    &raw mut ptr,
                    (arch_delay as ::core::ffi::c_int / 24 as ::core::ffi::c_int) as uint16_t,
                );
            }
            put8bit(&raw mut ptr, create.labelscnt);
            put8bit(&raw mut ptr, keep.labelscnt);
            put8bit(&raw mut ptr, arch.labelscnt);
            if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                put8bit(&raw mut ptr, trash.labelscnt);
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < create.labelscnt as ::core::ffi::c_int {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut create.labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    ptr = ptr.offset(SCLASS_EXPR_MAX_SIZE as isize);
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut keep.labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    ptr = ptr.offset(SCLASS_EXPR_MAX_SIZE as isize);
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < arch.labelscnt as ::core::ffi::c_int {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut arch.labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    ptr = ptr.offset(SCLASS_EXPR_MAX_SIZE as isize);
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < trash.labelscnt as ::core::ffi::c_int {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut trash.labelexpr as *mut [uint8_t; 128])
                            .offset(i as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    ptr = ptr.offset(SCLASS_EXPR_MAX_SIZE as isize);
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int)
                    < create.labelscnt as ::core::ffi::c_int * MASKORGROUP
                {
                    put32bit(&raw mut ptr, create_labelmasks[i as usize]);
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int * MASKORGROUP
                {
                    put32bit(&raw mut ptr, keep_labelmasks[i as usize]);
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < arch.labelscnt as ::core::ffi::c_int * MASKORGROUP
                {
                    put32bit(&raw mut ptr, arch_labelmasks[i as usize]);
                    i = i.wrapping_add(1);
                }
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_sclass_delete(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SCLASS_DELETE - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_ADMIN as uint32_t != 0 {
            nleng = get8bit(&raw mut data);
            name = data;
            data = data.offset(nleng as ::core::ffi::c_int as isize);
            if length != (5 as uint32_t).wrapping_add(nleng as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_SCLASS_DELETE - wrong size (%u:nleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    nleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            status = sclass_delete_entry(nleng, name);
        } else {
            status = MFS_ERROR_EPERM_NOTADMIN as uint8_t;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_SCLASS_DELETE as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_sclass_duplicate(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut snleng: uint8_t = 0;
        let mut dnleng: uint8_t = 0;
        let mut sname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut dname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SCLASS_DUPLICATE - wrong size (%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_ADMIN as uint32_t != 0 {
            snleng = get8bit(&raw mut data);
            sname = data;
            data = data.offset(snleng as ::core::ffi::c_int as isize);
            if length < (6 as uint32_t).wrapping_add(snleng as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_SCLASS_DUPLICATE - wrong size (%u:snleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    snleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            dnleng = get8bit(&raw mut data);
            dname = data;
            data = data.offset(dnleng as ::core::ffi::c_int as isize);
            if length
                != (6 as uint32_t)
                    .wrapping_add(snleng as uint32_t)
                    .wrapping_add(dnleng as uint32_t)
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_SCLASS_DUPLICATE - wrong size (%u:snleng=%hhu:dnleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    snleng as ::core::ffi::c_int,
                    dnleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            status = sclass_duplicate_entry(snleng, sname, dnleng, dname);
        } else {
            status = MFS_ERROR_EPERM_NOTADMIN as uint8_t;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_SCLASS_DUPLICATE as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_sclass_rename(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut snleng: uint8_t = 0;
        let mut dnleng: uint8_t = 0;
        let mut sname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut dname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SCLASS_RENAME - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_ADMIN as uint32_t != 0 {
            snleng = get8bit(&raw mut data);
            sname = data;
            data = data.offset(snleng as ::core::ffi::c_int as isize);
            if length < (6 as uint32_t).wrapping_add(snleng as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_SCLASS_RENAME - wrong size (%u:snleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    snleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            dnleng = get8bit(&raw mut data);
            dname = data;
            data = data.offset(dnleng as ::core::ffi::c_int as isize);
            if length
                != (6 as uint32_t)
                    .wrapping_add(snleng as uint32_t)
                    .wrapping_add(dnleng as uint32_t)
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_SCLASS_RENAME - wrong size (%u:snleng=%hhu:dnleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    snleng as ::core::ffi::c_int,
                    dnleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            status = sclass_rename_entry(snleng, sname, dnleng, dname);
        } else {
            status = MFS_ERROR_EPERM_NOTADMIN as uint8_t;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_SCLASS_RENAME as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_sclass_list(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rsize: uint32_t = 0;
        let mut fver: uint8_t = 0;
        if length != 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SCLASS_LIST - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        fver = get8bit(&raw mut data);
        rsize = sclass_list_entries(::core::ptr::null_mut::<uint8_t>(), fver);
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_SCLASS_LIST as uint32_t,
            (4 as uint32_t).wrapping_add(rsize),
        );
        put32bit(&raw mut ptr, msgid);
        sclass_list_entries(ptr, fver);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_pattern_add(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut fver: uint8_t = 0;
        let mut gnleng: uint8_t = 0;
        let mut scnleng: uint8_t = 0;
        let mut gname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut scname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut euid: uint32_t = 0;
        let mut egid: uint32_t = 0;
        let mut trashretention: uint16_t = 0;
        let mut priority: uint8_t = 0;
        let mut omask: uint8_t = 0;
        let mut seteattr: uint8_t = 0;
        let mut clreattr: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 21 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATTERN_ADD - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_ADMIN as uint32_t != 0 {
            fver = get8bit(&raw mut data);
            if fver as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_PATTERN_ADD - wrong packet version (%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    fver as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gnleng = get8bit(&raw mut data);
            if length < (21 as uint32_t).wrapping_add(gnleng as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_PATTERN_ADD - wrong size (%u:gnleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gnleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gname = data;
            data = data.offset(gnleng as ::core::ffi::c_int as isize);
            euid = get32bit(&raw mut data);
            egid = get32bit(&raw mut data);
            priority = get8bit(&raw mut data);
            omask = get8bit(&raw mut data);
            scnleng = get8bit(&raw mut data);
            if length
                != (21 as uint32_t)
                    .wrapping_add(gnleng as uint32_t)
                    .wrapping_add(scnleng as uint32_t)
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_PATTERN_ADD - wrong size (%u:gnleng=%hhu:scnleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gnleng as ::core::ffi::c_int,
                    scnleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            scname = data;
            data = data.offset(scnleng as ::core::ffi::c_int as isize);
            trashretention = get16bit(&raw mut data);
            seteattr = get8bit(&raw mut data);
            clreattr = get8bit(&raw mut data);
            status = patterns_add(
                gnleng,
                gname,
                euid,
                egid,
                priority,
                omask,
                scnleng,
                scname,
                trashretention,
                seteattr,
                clreattr,
            );
        } else {
            status = MFS_ERROR_EPERM_NOTADMIN as uint8_t;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_PATTERN_ADD as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_pattern_delete(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut fver: uint8_t = 0;
        let mut gnleng: uint8_t = 0;
        let mut gname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut euid: uint32_t = 0;
        let mut egid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 14 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATTERN_DELETE - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if sessions_get_sesflags((*eptr).sesdata) & SESFLAG_ADMIN as uint32_t != 0 {
            fver = get8bit(&raw mut data);
            if fver as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_PATTERN_DELETE - wrong packet version (%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    fver as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gnleng = get8bit(&raw mut data);
            if length != (14 as uint32_t).wrapping_add(gnleng as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_PATTERN_DELETE - wrong size (%u:gnleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gnleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gname = data;
            data = data.offset(gnleng as ::core::ffi::c_int as isize);
            euid = get32bit(&raw mut data);
            egid = get32bit(&raw mut data);
            status = patterns_delete(gnleng, gname, euid, egid);
        } else {
            status = MFS_ERROR_EPERM_NOTADMIN as uint8_t;
        }
        ptr = matoclserv_create_packet(eptr, MATOCL_PATTERN_DELETE as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_pattern_list(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut fver: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut lsize: uint32_t = 0;
        if length != 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATTERN_LIST - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        fver = get8bit(&raw mut data);
        if fver as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATTERN_LIST - wrong packet version (%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fver as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        lsize = patterns_list(::core::ptr::null_mut::<uint8_t>());
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_PATTERN_LIST as uint32_t,
            (4 as uint32_t).wrapping_add(lsize),
        );
        put32bit(&raw mut ptr, msgid);
        patterns_list(ptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_pattern_info(
    mut eptr: *mut matoclserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut lsize: uint32_t = 0;
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_PATTERN_INFO - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        lsize = patterns_list(::core::ptr::null_mut::<uint8_t>());
        ptr = matoclserv_create_packet(eptr, MATOCL_PATTERN_INFO as uint32_t, lsize);
        patterns_list(ptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_trash_list(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut partno: uint8_t = 0;
        let mut format: uint8_t = 0;
        let mut uid: uint32_t = 0;
        let mut mints: uint32_t = 0;
        let mut maxts: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut dleng: uint32_t = 0;
        let mut gnleng: uint8_t = 0;
        let mut gname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 18 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_TRASH_LIST - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        partno = get8bit(&raw mut data);
        format = get8bit(&raw mut data);
        uid = get32bit(&raw mut data);
        sessions_ugid_remap(
            (*eptr).sesdata,
            &raw mut uid,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        mints = get32bit(&raw mut data);
        maxts = get32bit(&raw mut data);
        if length > 18 as uint32_t {
            gnleng = get8bit(&raw mut data);
            if length != (19 as uint32_t).wrapping_add(gnleng as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_TRASH_LIST - wrong size (%u;gnleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gnleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gname = data;
            data = data.offset(gnleng as ::core::ffi::c_int as isize);
        } else {
            gnleng = 0 as uint8_t;
            gname = ::core::ptr::null::<uint8_t>();
        }
        dleng = fs_listtrash(
            partno,
            format,
            uid,
            mints,
            maxts,
            gnleng,
            gname,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        status = MFS_STATUS_OK as uint8_t;
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_TRASH_LIST as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                (4 as uint32_t).wrapping_add(dleng)
            },
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            fs_listtrash(partno, format, uid, mints, maxts, gnleng, gname, ptr);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_trash_recover(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut proot: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut cumask: uint16_t = 0;
        let mut uid: uint32_t = 0;
        let mut gids: uint32_t = 0;
        let mut gid: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut copysgid: uint8_t = 0;
        let mut i: uint32_t = 0;
        let mut used_pleng: uint32_t = 0;
        let mut used_path: [uint8_t; 1024] = [0; 1024];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 27 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_TRASH_RECOVER - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        proot = get32bit(&raw mut data);
        pleng = get32bit(&raw mut data);
        if length < (27 as uint32_t).wrapping_add(pleng) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_TRASH_RECOVER - wrong size (%u:pleng=%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                pleng,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        path = data;
        data = data.offset(pleng as isize);
        cumask = get16bit(&raw mut data);
        uid = get32bit(&raw mut data);
        gids = get32bit(&raw mut data);
        if gids == 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_TRASH_RECOVER - group ids missing\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if gids > MFS_GIDS_MAX as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_TRASH_RECOVER - too many group ids\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length
            != (27 as uint32_t)
                .wrapping_add(pleng)
                .wrapping_add((4 as uint32_t).wrapping_mul(gids))
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_TRASH_RECOVER - wrong size (%u:pleng=%u:gids=%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                pleng,
                gids,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        gid = matoclserv_gid_storage(gids);
        i = 0 as uint32_t;
        while i < gids {
            *gid.offset(i as isize) = get32bit(&raw mut data);
            i = i.wrapping_add(1);
        }
        sessions_ugid_remap((*eptr).sesdata, &raw mut uid, gid);
        copysgid = get8bit(&raw mut data);
        status = fs_trash_recover(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            proot,
            pleng,
            path,
            cumask,
            uid,
            gids,
            gid,
            copysgid,
            &raw mut used_pleng,
            &raw mut used_path as *mut uint8_t,
        );
        if status as ::core::ffi::c_int != MFS_STATUS_OK || used_pleng == 0 as uint32_t {
            ptr = matoclserv_create_packet(eptr, MATOCL_TRASH_RECOVER as uint32_t, 5 as uint32_t);
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, status);
        } else {
            ptr = matoclserv_create_packet(
                eptr,
                MATOCL_TRASH_RECOVER as uint32_t,
                (8 as uint32_t).wrapping_add(used_pleng),
            );
            put32bit(&raw mut ptr, msgid);
            put32bit(&raw mut ptr, used_pleng);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut used_path as *mut uint8_t as *const ::core::ffi::c_void,
                used_pleng as size_t,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_trash_remove(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut uid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 12 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_TRASH_REMOVE - wrong size (%u/12)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        uid = get32bit(&raw mut data);
        sessions_ugid_remap(
            (*eptr).sesdata,
            &raw mut uid,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        status = fs_trash_remove(
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            uid,
        );
        ptr = matoclserv_create_packet(eptr, MATOCL_TRASH_REMOVE as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_sustained_list(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut partno: uint8_t = 0;
        let mut uid: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut dleng: uint32_t = 0;
        let mut gnleng: uint8_t = 0;
        let mut gname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_SUSTAINED_LIST - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        partno = get8bit(&raw mut data);
        uid = get32bit(&raw mut data);
        sessions_ugid_remap(
            (*eptr).sesdata,
            &raw mut uid,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        if length > 9 as uint32_t {
            gnleng = get8bit(&raw mut data);
            if length != (10 as uint32_t).wrapping_add(gnleng as uint32_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOMA_SUSTAINED_LIST - wrong size (%u;gnleng=%hhu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                    gnleng as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            gname = data;
            data = data.offset(gnleng as ::core::ffi::c_int as isize);
        } else {
            gnleng = 0 as uint8_t;
            gname = ::core::ptr::null::<uint8_t>();
        }
        dleng = fs_listsustained(
            partno,
            uid,
            gnleng,
            gname,
            ::core::ptr::null_mut::<uint8_t>(),
        );
        status = MFS_STATUS_OK as uint8_t;
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_SUSTAINED_LIST as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                (4 as uint32_t).wrapping_add(dleng)
            },
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            fs_listsustained(partno, uid, gnleng, gname, ptr);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_getdirstats_old(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut inodes: uint32_t = 0;
        let mut files: uint32_t = 0;
        let mut dirs: uint32_t = 0;
        let mut chunks: uint32_t = 0;
        let mut leng: uint64_t = 0;
        let mut size: uint64_t = 0;
        let mut rsize: uint64_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETDIRSTATS - wrong size (%u/8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        status = fs_get_dir_stats(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            &raw mut inodes,
            &raw mut dirs,
            &raw mut files,
            &raw mut chunks,
            &raw mut leng,
            &raw mut size,
            &raw mut rsize,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETDIRSTATS as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                60 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, inodes);
            put32bit(&raw mut ptr, dirs);
            put32bit(&raw mut ptr, files);
            put32bit(&raw mut ptr, 0 as uint32_t);
            put32bit(&raw mut ptr, 0 as uint32_t);
            put32bit(&raw mut ptr, chunks);
            put32bit(&raw mut ptr, 0 as uint32_t);
            put32bit(&raw mut ptr, 0 as uint32_t);
            put64bit(&raw mut ptr, leng);
            put64bit(&raw mut ptr, size);
            put64bit(&raw mut ptr, rsize);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_getdirstats(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut inodes: uint32_t = 0;
        let mut files: uint32_t = 0;
        let mut dirs: uint32_t = 0;
        let mut chunks: uint32_t = 0;
        let mut leng: uint64_t = 0;
        let mut size: uint64_t = 0;
        let mut rsize: uint64_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETDIRSTATS - wrong size (%u/8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        status = fs_get_dir_stats(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            &raw mut inodes,
            &raw mut dirs,
            &raw mut files,
            &raw mut chunks,
            &raw mut leng,
            &raw mut size,
            &raw mut rsize,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETDIRSTATS as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                44 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, inodes);
            put32bit(&raw mut ptr, dirs);
            put32bit(&raw mut ptr, files);
            put32bit(&raw mut ptr, chunks);
            put64bit(&raw mut ptr, leng);
            put64bit(&raw mut ptr, size);
            put64bit(&raw mut ptr, rsize);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_gettrash(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut dleng: uint32_t = 0;
        let mut tid: uint32_t = 0;
        if length != 4 as uint32_t && length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETTRASH - wrong size (%u/4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        if length == 8 as uint32_t {
            tid = get32bit(&raw mut data);
        } else {
            tid = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        }
        status = fs_readtrash_size(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            tid,
            &raw mut dleng,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETTRASH as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                (4 as uint32_t).wrapping_add(dleng)
            },
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            fs_readtrash_data(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                tid,
                ptr,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_getdetachedattr(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut msgid: uint32_t = 0;
        let mut dtype: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length < 8 as uint32_t || length > 9 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETDETACHEDATTR - wrong size (%u/8,9)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        if length == 9 as uint32_t {
            dtype = get8bit(&raw mut data);
        } else {
            dtype = DTYPE_UNKNOWN as uint8_t;
        }
        status = fs_getdetachedattr(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            &raw mut attr as *mut uint8_t,
            dtype,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETDETACHEDATTR as uint32_t,
            (if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as ::core::ffi::c_int
            } else {
                (*eptr).asize as ::core::ffi::c_int + 4 as ::core::ffi::c_int
            }) as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut attr as *mut uint8_t as *const ::core::ffi::c_void,
                (*eptr).asize as size_t,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_gettrashpath(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETTRASHPATH - wrong size (%u/8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        status = fs_gettrashpath(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            &raw mut pleng,
            &raw mut path,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETTRASHPATH as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                (8 as uint32_t)
                    .wrapping_add(pleng)
                    .wrapping_add(1 as uint32_t)
            },
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            put32bit(&raw mut ptr, pleng.wrapping_add(1 as uint32_t));
            if pleng > 0 as uint32_t {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    path as *const ::core::ffi::c_void,
                    pleng as size_t,
                );
            }
            *ptr.offset(pleng as isize) = 0 as uint8_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_settrashpath(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pleng: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 12 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETTRASHPATH - wrong size (%u/>=12)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        pleng = get32bit(&raw mut data);
        if length != (12 as uint32_t).wrapping_add(pleng) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_SETTRASHPATH - wrong size (%u/%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                (12 as uint32_t).wrapping_add(pleng),
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        path = data;
        data = data.offset(pleng as isize);
        while pleng > 0 as uint32_t
            && *path.offset(pleng.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            pleng = pleng.wrapping_sub(1);
        }
        status = fs_settrashpath(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
            pleng,
            path,
        );
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_SETTRASHPATH as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_undel(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_UNDEL - wrong size (%u/8)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        status = fs_undel(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
        );
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_UNDEL as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_purge(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut inode: uint32_t = 0;
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_PURGE - wrong size (%u/8)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        inode = get32bit(&raw mut data);
        status = fs_purge(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            inode,
        );
        ptr = matoclserv_create_packet(eptr, MATOCL_FUSE_PURGE as uint32_t, 5 as uint32_t);
        put32bit(&raw mut ptr, msgid);
        put8bit(&raw mut ptr, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_fuse_getsustained(
    mut eptr: *mut matoclserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut status: uint8_t = 0;
        let mut dleng: uint32_t = 0;
        if length != 4 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOMA_FUSE_GETSUSTAINED - wrong size (%u/4)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        status = fs_readsustained_size(
            sessions_get_rootinode((*eptr).sesdata),
            sessions_get_sesflags((*eptr).sesdata) as uint8_t,
            &raw mut dleng,
        );
        ptr = matoclserv_create_packet(
            eptr,
            MATOCL_FUSE_GETSUSTAINED as uint32_t,
            if status as ::core::ffi::c_int != MFS_STATUS_OK {
                5 as uint32_t
            } else {
                (4 as uint32_t).wrapping_add(dleng)
            },
        );
        put32bit(&raw mut ptr, msgid);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            put8bit(&raw mut ptr, status);
        } else {
            fs_readsustained_data(
                sessions_get_rootinode((*eptr).sesdata),
                sessions_get_sesflags((*eptr).sesdata) as uint8_t,
                ptr,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_get_min_cl_version() -> uint32_t {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut minver: uint32_t = 0 as uint32_t;
        eptr = matoclservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*eptr).registered as ::core::ffi::c_int != NOTREGISTERED as ::core::ffi::c_int
                && !(*eptr).sesdata.is_null()
            {
                if minver == 0 as uint32_t || (*eptr).version < minver {
                    minver = (*eptr).version;
                }
            }
            eptr = (*eptr).next as *mut matoclserventry;
        }
        return minver;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_beforedisconnect(mut eptr: *mut matoclserventry) {
    unsafe {
        let mut swc: *mut swchunks = ::core::ptr::null_mut::<swchunks>();
        let mut pswc: *mut *mut swchunks = ::core::ptr::null_mut::<*mut swchunks>();
        let mut lwc: *mut lwchunks = ::core::ptr::null_mut::<lwchunks>();
        let mut plwc: *mut *mut lwchunks = ::core::ptr::null_mut::<*mut lwchunks>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < CHUNKHASHSIZE as uint32_t {
            pswc = (&raw mut swchunkshash as *mut *mut swchunks).offset(i as isize);
            loop {
                swc = *pswc;
                if swc.is_null() {
                    break;
                }
                if (*swc).eptr == eptr {
                    fs_rollback(
                        (*swc).inode,
                        (*swc).indx,
                        (*swc).prevchunkid,
                        (*swc).chunkid,
                    );
                    *pswc = (*swc).next as *mut swchunks;
                    free(swc as *mut ::core::ffi::c_void);
                } else {
                    pswc = &raw mut (*swc).next as *mut *mut swchunks;
                }
            }
            plwc = (&raw mut lwchunkshashhead as *mut *mut lwchunks).offset(i as isize);
            loop {
                lwc = *plwc;
                if lwc.is_null() {
                    break;
                }
                if (*lwc).eptr == eptr {
                    *plwc = (*lwc).next as *mut lwchunks;
                    free(lwc as *mut ::core::ffi::c_void);
                } else {
                    plwc = &raw mut (*lwc).next as *mut *mut lwchunks;
                }
            }
            lwchunkshashtail[i as usize] = plwc;
            i = i.wrapping_add(1);
        }
        if !(*eptr).path.is_null() {
            free((*eptr).path as *mut ::core::ffi::c_void);
            (*eptr).path = ::core::ptr::null_mut::<uint8_t>();
        }
        if !(*eptr).info.is_null() {
            free((*eptr).info as *mut ::core::ffi::c_void);
            (*eptr).info = ::core::ptr::null_mut::<uint8_t>();
        }
        if !(*eptr).strip.is_null() {
            free((*eptr).strip as *mut ::core::ffi::c_void);
            (*eptr).strip = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        sessions_disconnection((*eptr).sesdata);
        posix_lock_disconnected(eptr as *mut ::core::ffi::c_void);
        flock_disconnected(eptr as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_gotpacket(
    mut eptr: *mut matoclserventry,
    mut r#type: uint32_t,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if r#type == ANTOAN_NOP as uint32_t {
            return;
        }
        if r#type == ANTOAN_UNKNOWN_COMMAND as uint32_t {
            return;
        }
        if r#type == ANTOAN_BAD_COMMAND_SIZE as uint32_t {
            return;
        }
        if (*eptr).registered as ::core::ffi::c_int == NOTREGISTERED as ::core::ffi::c_int {
            match r#type {
                10 => {
                    matoclserv_get_version(eptr, data, length);
                }
                400 => {
                    matoclserv_fuse_register(eptr, data, length);
                }
                500 => {
                    matoclserv_cserv_list(eptr, data, length);
                }
                508 => {
                    matoclserv_session_list(eptr, data, length);
                }
                504 => {
                    matoclserv_chart(eptr, data, length);
                }
                506 => {
                    matoclserv_chart_data(eptr, data, length);
                }
                502 => {
                    matoclserv_monotonic_data(eptr, data, length);
                }
                510 => {
                    matoclserv_info(eptr, data, length);
                }
                512 => {
                    matoclserv_fstest_info(eptr, data, length);
                }
                514 => {
                    matoclserv_chunkstest_info(eptr, data, length);
                }
                516 => {
                    matoclserv_chunks_matrix(eptr, data, length);
                }
                518 => {
                    matoclserv_quota_info(eptr, data, length);
                }
                520 => {
                    matoclserv_exports_info(eptr, data, length);
                }
                522 => {
                    matoclserv_mlog_list(eptr, data, length);
                }
                524 => {
                    matoclserv_cserv_command(eptr, data, length);
                }
                526 => {
                    matoclserv_session_command(eptr, data, length);
                }
                528 => {
                    matoclserv_memory_info(eptr, data, length);
                }
                530 => {
                    matoclserv_module_info(eptr, data, length);
                }
                532 => {
                    matoclserv_list_open_files(eptr, data, length);
                }
                534 => {
                    matoclserv_list_acquired_locks(eptr, data, length);
                }
                536 => {
                    matoclserv_mass_resolve_paths(eptr, data, length);
                }
                542 => {
                    matoclserv_sclass_info(eptr, data, length);
                }
                548 => {
                    matoclserv_pattern_info(eptr, data, length);
                }
                544 => {
                    matoclserv_missing_chunks(eptr, data, length);
                }
                550 => {
                    matoclserv_instance_name(eptr, data, length);
                }
                _ => {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"main master server module: got unknown message from unregistered (type:%u)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        r#type,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                }
            }
        } else if (*eptr).registered as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int {
            if (*eptr).sesdata.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"registered connection without sesdata !!!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            match r#type {
                10 => {
                    matoclserv_get_version(eptr, data, length);
                }
                80 => {
                    matoclserv_get_config(eptr, data, length);
                }
                82 => {
                    matoclserv_get_config_file(eptr, data, length);
                }
                71 => {
                    matoclserv_syslog(eptr, data, length);
                }
                400 => {
                    matoclserv_fuse_register(eptr, data, length);
                }
                499 | 700 => {
                    matoclserv_fuse_sustained_inodes(eptr, data, length);
                }
                701 => {
                    matoclserv_fuse_amtime_inodes(eptr, data, length);
                }
                710 => {
                    matoclserv_fuse_opdata(eptr, data, length);
                }
                711 => {
                    matoclserv_fuse_wflags(eptr, data, length);
                }
                704 => {
                    matoclserv_fuse_time_sync(eptr, data, length);
                }
                390 => {
                    matoclserv_path_lookup(eptr, data, length);
                }
                402 => {
                    matoclserv_fuse_statfs(eptr, data, length);
                }
                404 => {
                    matoclserv_fuse_access(eptr, data, length);
                }
                406 => {
                    matoclserv_fuse_lookup(eptr, data, length);
                }
                408 => {
                    matoclserv_fuse_getattr(eptr, data, length);
                }
                410 => {
                    matoclserv_fuse_setattr(eptr, data, length);
                }
                412 => {
                    matoclserv_fuse_readlink(eptr, data, length);
                }
                414 => {
                    matoclserv_fuse_symlink(eptr, data, length);
                }
                416 => {
                    matoclserv_fuse_mknod(eptr, data, length);
                }
                418 => {
                    matoclserv_fuse_mkdir(eptr, data, length);
                }
                420 => {
                    matoclserv_fuse_unlink(eptr, data, length);
                }
                422 => {
                    matoclserv_fuse_rmdir(eptr, data, length);
                }
                424 => {
                    matoclserv_fuse_rename(eptr, data, length);
                }
                426 => {
                    matoclserv_fuse_link(eptr, data, length);
                }
                428 => {
                    matoclserv_fuse_readdir(eptr, data, length);
                }
                430 => {
                    matoclserv_fuse_open(eptr, data, length);
                }
                482 => {
                    matoclserv_fuse_create(eptr, data, length);
                }
                432 => {
                    matoclserv_fuse_read_chunk(eptr, data, length);
                }
                434 => {
                    matoclserv_fuse_write_chunk(eptr, data, length);
                }
                436 => {
                    matoclserv_fuse_write_chunk_end(eptr, data, length);
                }
                492 => {
                    matoclserv_fuse_flock(eptr, data, length);
                }
                494 => {
                    matoclserv_fuse_posix_lock(eptr, data, length);
                }
                450 => {
                    matoclserv_fuse_gettrash(eptr, data, length);
                }
                452 => {
                    matoclserv_fuse_getdetachedattr(eptr, data, length);
                }
                454 => {
                    matoclserv_fuse_gettrashpath(eptr, data, length);
                }
                456 => {
                    matoclserv_fuse_settrashpath(eptr, data, length);
                }
                458 => {
                    matoclserv_fuse_undel(eptr, data, length);
                }
                460 => {
                    matoclserv_fuse_purge(eptr, data, length);
                }
                470 => {
                    matoclserv_fuse_getsustained(eptr, data, length);
                }
                440 => {
                    matoclserv_fuse_check(eptr, data, length);
                }
                442 => {
                    matoclserv_fuse_gettrashretention(eptr, data, length);
                }
                444 => {
                    matoclserv_fuse_settrashretention(eptr, data, length);
                }
                446 => {
                    matoclserv_fuse_getsclass(eptr, data, length);
                }
                448 => {
                    matoclserv_fuse_setsclass(eptr, data, length);
                }
                438 => {
                    matoclserv_fuse_append_slice(eptr, data, length);
                }
                462 => {
                    matoclserv_fuse_getdirstats(eptr, data, length);
                }
                464 => {
                    matoclserv_fuse_truncate(eptr, data, length);
                }
                466 => {
                    matoclserv_fuse_repair(eptr, data, length);
                }
                468 => {
                    matoclserv_fuse_snapshot(eptr, data, length);
                }
                472 => {
                    matoclserv_fuse_geteattr(eptr, data, length);
                }
                474 => {
                    matoclserv_fuse_seteattr(eptr, data, length);
                }
                484 => {
                    matoclserv_fuse_parents(eptr, data, length);
                }
                486 => {
                    matoclserv_fuse_paths(eptr, data, length);
                }
                478 => {
                    matoclserv_fuse_getxattr(eptr, data, length);
                }
                480 => {
                    matoclserv_fuse_setxattr(eptr, data, length);
                }
                488 => {
                    matoclserv_fuse_getfacl(eptr, data, length);
                }
                490 => {
                    matoclserv_fuse_setfacl(eptr, data, length);
                }
                476 => {
                    matoclserv_fuse_quotacontrol(eptr, data, length);
                }
                496 => {
                    matoclserv_fuse_archctl(eptr, data, length);
                }
                350 => {
                    matoclserv_sclass_create(eptr, data, length);
                }
                352 => {
                    matoclserv_sclass_change(eptr, data, length);
                }
                354 => {
                    matoclserv_sclass_delete(eptr, data, length);
                }
                356 => {
                    matoclserv_sclass_duplicate(eptr, data, length);
                }
                358 => {
                    matoclserv_sclass_rename(eptr, data, length);
                }
                360 => {
                    matoclserv_sclass_list(eptr, data, length);
                }
                370 => {
                    matoclserv_pattern_add(eptr, data, length);
                }
                372 => {
                    matoclserv_pattern_delete(eptr, data, length);
                }
                374 => {
                    matoclserv_pattern_list(eptr, data, length);
                }
                380 => {
                    matoclserv_trash_list(eptr, data, length);
                }
                382 => {
                    matoclserv_trash_recover(eptr, data, length);
                }
                384 => {
                    matoclserv_trash_remove(eptr, data, length);
                }
                388 => {
                    matoclserv_sustained_list(eptr, data, length);
                }
                500 => {
                    matoclserv_cserv_list(eptr, data, length);
                }
                508 => {
                    matoclserv_session_list(eptr, data, length);
                }
                504 => {
                    matoclserv_chart(eptr, data, length);
                }
                506 => {
                    matoclserv_chart_data(eptr, data, length);
                }
                502 => {
                    matoclserv_monotonic_data(eptr, data, length);
                }
                510 => {
                    matoclserv_info(eptr, data, length);
                }
                512 => {
                    matoclserv_fstest_info(eptr, data, length);
                }
                514 => {
                    matoclserv_chunkstest_info(eptr, data, length);
                }
                516 => {
                    matoclserv_chunks_matrix(eptr, data, length);
                }
                518 => {
                    matoclserv_quota_info(eptr, data, length);
                }
                520 => {
                    matoclserv_exports_info(eptr, data, length);
                }
                522 => {
                    matoclserv_mlog_list(eptr, data, length);
                }
                524 => {
                    matoclserv_cserv_command(eptr, data, length);
                }
                526 => {
                    matoclserv_session_command(eptr, data, length);
                }
                528 => {
                    matoclserv_memory_info(eptr, data, length);
                }
                530 => {
                    matoclserv_module_info(eptr, data, length);
                }
                532 => {
                    matoclserv_list_open_files(eptr, data, length);
                }
                534 => {
                    matoclserv_list_acquired_locks(eptr, data, length);
                }
                536 => {
                    matoclserv_mass_resolve_paths(eptr, data, length);
                }
                542 => {
                    matoclserv_sclass_info(eptr, data, length);
                }
                544 => {
                    matoclserv_missing_chunks(eptr, data, length);
                }
                546 => {
                    matoclserv_node_info(eptr, data, length);
                }
                552 => {
                    matoclserv_full_directory_data(eptr, data, length);
                }
                554 => {
                    matoclserv_set_all_node_attributes(eptr, data, length);
                }
                550 => {
                    matoclserv_instance_name(eptr, data, length);
                }
                _ => {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"main master server module: got unknown message from mfsmount (type:%u)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        r#type,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_read(
    mut eptr: *mut matoclserventry,
    mut now: ::core::ffi::c_double,
) {
    unsafe {
        let mut i: int32_t = 0;
        let mut r#type: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut rbleng: uint32_t = 0;
        let mut rbpos: uint32_t = 0;
        let mut err: uint8_t = 0;
        let mut hup: uint8_t = 0;
        let mut errmsg: uint8_t = 0;
        static mut readbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut readbuffsize: uint32_t = 0 as uint32_t;
        if eptr.is_null() {
            if !readbuff.is_null() {
                free(readbuff as *mut ::core::ffi::c_void);
            }
            readbuff = ::core::ptr::null_mut::<uint8_t>();
            readbuffsize = 0 as uint32_t;
            return;
        }
        if readbuffsize == 0 as uint32_t {
            readbuffsize = 65536 as uint32_t;
            readbuff = malloc(readbuffsize as size_t) as *mut uint8_t;
            if readbuff.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7034 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7034 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if readbuff
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7034 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    7034 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        rbleng = 0 as uint32_t;
        err = 0 as uint8_t;
        hup = 0 as uint8_t;
        errmsg = 0 as uint8_t;
        loop {
            i = read(
                (*eptr).sock,
                readbuff.offset(rbleng as isize) as *mut ::core::ffi::c_void,
                readbuffsize.wrapping_sub(rbleng) as size_t,
            ) as int32_t;
            if i == 0 as int32_t {
                hup = 1 as uint8_t;
                break;
            } else if i < 0 as int32_t {
                if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    err = 1 as uint8_t;
                    errmsg = 1 as uint8_t;
                }
                break;
            } else {
                stats_brcvd = stats_brcvd.wrapping_add(i as uint64_t);
                rbleng = rbleng.wrapping_add(i as uint32_t);
                if rbleng != readbuffsize {
                    break;
                }
                readbuffsize = readbuffsize.wrapping_mul(2 as uint32_t);
                readbuff = mfsrealloc(readbuff as *mut ::core::ffi::c_void, readbuffsize as size_t)
                    as *mut uint8_t;
                if readbuff.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7058 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7058 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if readbuff
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
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7058 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7058 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    abort();
                }
            }
        }
        if rbleng > 0 as uint32_t {
            (*eptr).lastread = now;
        }
        rbpos = 0 as uint32_t;
        while rbpos < rbleng {
            if rbleng.wrapping_sub(rbpos) >= (*eptr).input_bytesleft {
                memcpy(
                    (*eptr).input_startptr as *mut ::core::ffi::c_void,
                    readbuff.offset(rbpos as isize) as *const ::core::ffi::c_void,
                    (*eptr).input_bytesleft as size_t,
                );
                i = (*eptr).input_bytesleft as int32_t;
            } else {
                memcpy(
                    (*eptr).input_startptr as *mut ::core::ffi::c_void,
                    readbuff.offset(rbpos as isize) as *const ::core::ffi::c_void,
                    rbleng.wrapping_sub(rbpos) as size_t,
                );
                i = rbleng.wrapping_sub(rbpos) as int32_t;
            }
            rbpos = rbpos.wrapping_add(i as uint32_t);
            (*eptr).input_startptr = (*eptr).input_startptr.offset(i as isize);
            (*eptr).input_bytesleft = (*eptr).input_bytesleft.wrapping_sub(i as uint32_t);
            if (*eptr).input_bytesleft > 0 as uint32_t {
                break;
            }
            if (*eptr).input_packet.is_null() {
                ptr = &raw mut (*eptr).input_hdr as *mut uint8_t;
                r#type = get32bit(&raw mut ptr);
                leng = get32bit(&raw mut ptr);
                if leng > MaxPacketSize as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"main master server module: packet too long (%u/%u) ; command:%u\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        leng,
                        MaxPacketSize,
                        r#type,
                    );
                    (*eptr).input_end = 1 as uint8_t;
                    return;
                }
                stats_prcvd = stats_prcvd.wrapping_add(1);
                (*eptr).input_packet =
                    malloc((16 as size_t).wrapping_add(leng as size_t)) as *mut in_packetstruct;
                if (*eptr).input_packet.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7099 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7099 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*eptr).input_packet
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut in_packetstruct
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7099 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7099 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    abort();
                }
                (*(*eptr).input_packet).next = ::core::ptr::null_mut::<in_packetstruct>();
                (*(*eptr).input_packet).r#type = r#type;
                (*(*eptr).input_packet).leng = leng;
                (*eptr).input_startptr = &raw mut (*(*eptr).input_packet).data as *mut uint8_t;
                (*eptr).input_bytesleft = leng;
            }
            if (*eptr).input_bytesleft > 0 as uint32_t {
                continue;
            }
            if !(*eptr).input_packet.is_null() {
                *(*eptr).inputtail = (*eptr).input_packet;
                (*eptr).inputtail =
                    &raw mut (*(*eptr).input_packet).next as *mut *mut in_packetstruct;
                (*eptr).input_packet = ::core::ptr::null_mut::<in_packetstruct>();
                (*eptr).input_bytesleft = 8 as uint32_t;
                (*eptr).input_startptr = &raw mut (*eptr).input_hdr as *mut uint8_t;
            }
        }
        if hup != 0 {
            if (*eptr).registered as ::core::ffi::c_int != NOTREGISTERED as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"connection with client (ip:%s) has been closed by peer\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).strip,
                );
            }
            (*eptr).input_end = 1 as uint8_t;
        } else if err != 0 {
            if errmsg != 0 {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"main master server module: client (ip:%s) read error\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).strip,
                );
            }
            (*eptr).input_end = 1 as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_parse(mut eptr: *mut matoclserventry) {
    unsafe {
        let mut ipack: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut starttime: uint64_t = 0;
        let mut currtime: uint64_t = 0;
        starttime = monotonic_useconds();
        currtime = starttime;
        while (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && {
                ipack = (*eptr).inputhead;
                !ipack.is_null()
            }
            && starttime.wrapping_add(10000 as uint64_t) > currtime
        {
            matoclserv_gotpacket(
                eptr,
                (*ipack).r#type,
                &raw mut (*ipack).data as *mut uint8_t,
                (*ipack).leng,
            );
            (*eptr).inputhead = (*ipack).next as *mut in_packetstruct;
            free(ipack as *mut ::core::ffi::c_void);
            if (*eptr).inputhead.is_null() {
                (*eptr).inputtail = &raw mut (*eptr).inputhead;
            } else {
                currtime = monotonic_useconds();
            }
        }
        if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && (*eptr).inputhead.is_null()
            && (*eptr).input_end as ::core::ffi::c_int != 0
        {
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_write(
    mut eptr: *mut matoclserventry,
    mut now: ::core::ffi::c_double,
) {
    unsafe {
        let mut opack: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut i: int32_t = 0;
        let mut iovtab: [iovec; 100] = [iovec {
            iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            iov_len: 0,
        }; 100];
        let mut iovdata: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut left: uint32_t = 0;
        loop {
            leng = 0 as uint32_t;
            iovdata = 0 as uint32_t;
            opack = (*eptr).outputhead;
            while iovdata < 100 as uint32_t && !opack.is_null() {
                iovtab[iovdata as usize].iov_base = (*opack).startptr as *mut ::core::ffi::c_void;
                iovtab[iovdata as usize].iov_len = (*opack).bytesleft as size_t;
                leng = leng.wrapping_add((*opack).bytesleft);
                iovdata = iovdata.wrapping_add(1);
                opack = (*opack).next as *mut out_packetstruct;
            }
            if iovdata == 0 as uint32_t {
                return;
            }
            i = writev(
                (*eptr).sock,
                &raw mut iovtab as *mut iovec,
                iovdata as ::core::ffi::c_int,
            ) as int32_t;
            if i < 0 as int32_t {
                if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"main master server module: client (ip:%s) write error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*eptr).strip,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                }
                return;
            }
            if i > 0 as int32_t {
                (*eptr).lastwrite = now;
            }
            stats_bsent = stats_bsent.wrapping_add(i as uint64_t);
            left = i as uint32_t;
            while left > 0 as uint32_t && !(*eptr).outputhead.is_null() {
                opack = (*eptr).outputhead;
                if (*opack).bytesleft > left {
                    (*opack).startptr = (*opack).startptr.offset(left as isize);
                    (*opack).bytesleft = (*opack).bytesleft.wrapping_sub(left);
                    left = 0 as uint32_t;
                } else {
                    left = left.wrapping_sub((*opack).bytesleft);
                    (*eptr).outputhead = (*opack).next as *mut out_packetstruct;
                    if (*eptr).outputhead.is_null() {
                        (*eptr).outputtail = &raw mut (*eptr).outputhead;
                    }
                    free(opack as *mut ::core::ffi::c_void);
                    stats_psent = stats_psent.wrapping_add(1);
                }
            }
            if (i as uint32_t) < leng {
                return;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_desc(mut pdesc: *mut pollfd, mut ndesc: *mut uint32_t) {
    unsafe {
        let mut pos: uint32_t = *ndesc;
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        (*pdesc.offset(pos as isize)).fd = lsock;
        (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
        lsockpdescpos = pos as int32_t;
        pos = pos.wrapping_add(1);
        eptr = matoclservhead;
        while !eptr.is_null() {
            (*pdesc.offset(pos as isize)).fd = (*eptr).sock;
            (*pdesc.offset(pos as isize)).events = 0 as ::core::ffi::c_short;
            (*eptr).pdescpos = pos as int32_t;
            if (*eptr).input_end as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*pdesc.offset(pos as isize)).events =
                    ((*pdesc.offset(pos as isize)).events as ::core::ffi::c_int | POLLIN)
                        as ::core::ffi::c_short;
            }
            if !(*eptr).outputhead.is_null() {
                (*pdesc.offset(pos as isize)).events =
                    ((*pdesc.offset(pos as isize)).events as ::core::ffi::c_int | POLLOUT)
                        as ::core::ffi::c_short;
            }
            pos = pos.wrapping_add(1);
            eptr = (*eptr).next as *mut matoclserventry;
        }
        *ndesc = pos;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_disconnection_loop() {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut kptr: *mut *mut matoclserventry = ::core::ptr::null_mut::<*mut matoclserventry>();
        let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        kptr = &raw mut matoclservhead;
        loop {
            eptr = *kptr;
            if eptr.is_null() {
                break;
            }
            if (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int {
                matoclserv_beforedisconnect(eptr);
                tcpclose((*eptr).sock);
                if !(*eptr).input_packet.is_null() {
                    free((*eptr).input_packet as *mut ::core::ffi::c_void);
                }
                ipptr = (*eptr).inputhead;
                while !ipptr.is_null() {
                    ipaptr = ipptr;
                    ipptr = (*ipptr).next as *mut in_packetstruct;
                    free(ipaptr as *mut ::core::ffi::c_void);
                }
                opptr = (*eptr).outputhead;
                while !opptr.is_null() {
                    opaptr = opptr;
                    opptr = (*opptr).next as *mut out_packetstruct;
                    free(opaptr as *mut ::core::ffi::c_void);
                }
                *kptr = (*eptr).next as *mut matoclserventry;
                free(eptr as *mut ::core::ffi::c_void);
            } else {
                kptr = &raw mut (*eptr).next as *mut *mut matoclserventry;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_serve(mut pdesc: *mut pollfd) {
    unsafe {
        let mut now: ::core::ffi::c_double = 0.;
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut ns: ::core::ffi::c_int = 0;
        static mut lastaction: ::core::ffi::c_double = 0.0f64;
        let mut timeoutadd: ::core::ffi::c_double = 0.;
        now = monotonic_seconds();
        if lastaction > 0.0f64 {
            timeoutadd = now - lastaction;
            if timeoutadd > 1.0f64 {
                eptr = matoclservhead;
                while !eptr.is_null() {
                    (*eptr).lastread += timeoutadd;
                    eptr = (*eptr).next as *mut matoclserventry;
                }
            }
        }
        lastaction = now;
        if lsockpdescpos >= 0 as int32_t
            && (*pdesc.offset(lsockpdescpos as isize)).revents as ::core::ffi::c_int & POLLIN != 0
        {
            ns = tcpaccept(lsock);
            if ns < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"main master server module: accept error\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                tcpnonblock(ns);
                tcpnodelay(ns);
                eptr = malloc(::core::mem::size_of::<matoclserventry>()) as *mut matoclserventry;
                if eptr.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if eptr
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut matoclserventry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matoclserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        7337 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*eptr).next = matoclservhead as *mut matoclserventry;
                matoclservhead = eptr;
                (*eptr).sock = ns;
                (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
                tcpgetpeer(
                    ns,
                    &raw mut (*eptr).peerip,
                    ::core::ptr::null_mut::<uint16_t>(),
                );
                (*eptr).strip = univallocstrip((*eptr).peerip);
                (*eptr).registered = NOTREGISTERED as ::core::ffi::c_int as uint8_t;
                (*eptr).version = 0 as uint32_t;
                (*eptr).asize = 0 as uint8_t;
                (*eptr).mode = DATA as ::core::ffi::c_int as uint8_t;
                (*eptr).lastread = now;
                (*eptr).lastwrite = now;
                (*eptr).input_bytesleft = 8 as uint32_t;
                (*eptr).input_startptr = &raw mut (*eptr).input_hdr as *mut uint8_t;
                (*eptr).input_end = 0 as uint8_t;
                (*eptr).input_packet = ::core::ptr::null_mut::<in_packetstruct>();
                (*eptr).inputhead = ::core::ptr::null_mut::<in_packetstruct>();
                (*eptr).inputtail = &raw mut (*eptr).inputhead;
                (*eptr).outputhead = ::core::ptr::null_mut::<out_packetstruct>();
                (*eptr).outputtail = &raw mut (*eptr).outputhead;
                if ForceTimeout > 0 as uint32_t {
                    (*eptr).timeout = ForceTimeout as uint16_t;
                } else {
                    (*eptr).timeout = DefaultTimeout as uint16_t;
                }
                (*eptr).path = ::core::ptr::null_mut::<uint8_t>();
                (*eptr).info = ::core::ptr::null_mut::<uint8_t>();
                (*eptr).ileng = 0 as uint32_t;
                (*eptr).usepassword = 0 as uint8_t;
                (*eptr).working_flags = 0 as uint8_t;
                (*eptr).sesdata = NULL;
                memset(
                    &raw mut (*eptr).passwordrnd as *mut uint8_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    32 as size_t,
                );
            }
        }
        eptr = matoclservhead;
        while !eptr.is_null() {
            if (*eptr).pdescpos >= 0 as int32_t {
                if (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLERR | POLLIN)
                    == POLLIN
                    && (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                {
                    matoclserv_read(eptr, now);
                }
                if (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLERR | POLLHUP)
                    != 0
                {
                    (*eptr).input_end = 1 as uint8_t;
                }
            }
            matoclserv_parse(eptr);
            eptr = (*eptr).next as *mut matoclserventry;
        }
        eptr = matoclservhead;
        while !eptr.is_null() {
            if (*eptr).lastwrite + 1.0f64 < now && (*eptr).outputhead.is_null() {
                let mut ptr: *mut uint8_t =
                    matoclserv_create_packet(eptr, ANTOAN_NOP as uint32_t, 4 as uint32_t);
                *(ptr as *mut uint32_t) = 0 as uint32_t;
            }
            if (*eptr).pdescpos >= 0 as int32_t {
                if ((*pdesc.offset((*eptr).pdescpos as isize)).events as ::core::ffi::c_int
                    & POLLOUT
                    == 0 as ::core::ffi::c_int
                    && !(*eptr).outputhead.is_null()
                    || (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                        & POLLOUT
                        != 0)
                    && (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                {
                    matoclserv_write(eptr, now);
                }
            }
            if ((*eptr).lastread + (*eptr).timeout as ::core::ffi::c_int as ::core::ffi::c_double)
                < now
            {
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
            if (*eptr).mode as ::core::ffi::c_int == FINISH as ::core::ffi::c_int
                && (*eptr).outputhead.is_null()
            {
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
            eptr = (*eptr).next as *mut matoclserventry;
        }
        matoclserv_disconnection_loop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_keep_alive() {
    unsafe {
        let mut now: ::core::ffi::c_double = 0.;
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        now = monotonic_seconds();
        eptr = matoclservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*eptr).input_end as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                matoclserv_read(eptr, now);
            }
            eptr = (*eptr).next as *mut matoclserventry;
        }
        eptr = matoclservhead;
        while !eptr.is_null() {
            if (*eptr).lastwrite + 1.0f64 < now && (*eptr).outputhead.is_null() {
                let mut ptr: *mut uint8_t =
                    matoclserv_create_packet(eptr, ANTOAN_NOP as uint32_t, 4 as uint32_t);
                *(ptr as *mut uint32_t) = 0 as uint32_t;
            }
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && !(*eptr).outputhead.is_null()
            {
                matoclserv_write(eptr, now);
            }
            eptr = (*eptr).next as *mut matoclserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_close_lsock() {
    unsafe {
        if lsock >= 0 as ::core::ffi::c_int {
            close(lsock);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_term() {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut eaptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut swc: *mut swchunks = ::core::ptr::null_mut::<swchunks>();
        let mut swcn: *mut swchunks = ::core::ptr::null_mut::<swchunks>();
        let mut lwc: *mut lwchunks = ::core::ptr::null_mut::<lwchunks>();
        let mut lwcn: *mut lwchunks = ::core::ptr::null_mut::<lwchunks>();
        let mut i: uint32_t = 0;
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"main master server module: closing %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        tcpclose(lsock);
        eptr = matoclservhead;
        while !eptr.is_null() {
            if !(*eptr).input_packet.is_null() {
                free((*eptr).input_packet as *mut ::core::ffi::c_void);
            }
            ipptr = (*eptr).inputhead;
            while !ipptr.is_null() {
                ipaptr = ipptr;
                ipptr = (*ipptr).next as *mut in_packetstruct;
                free(ipaptr as *mut ::core::ffi::c_void);
            }
            opptr = (*eptr).outputhead;
            while !opptr.is_null() {
                opaptr = opptr;
                opptr = (*opptr).next as *mut out_packetstruct;
                free(opaptr as *mut ::core::ffi::c_void);
            }
            eaptr = eptr;
            eptr = (*eptr).next as *mut matoclserventry;
            free(eaptr as *mut ::core::ffi::c_void);
        }
        matoclservhead = ::core::ptr::null_mut::<matoclserventry>();
        i = 0 as uint32_t;
        while i < CHUNKHASHSIZE as uint32_t {
            swc = swchunkshash[i as usize];
            while !swc.is_null() {
                swcn = (*swc).next as *mut swchunks;
                free(swc as *mut ::core::ffi::c_void);
                swc = swcn;
            }
            lwc = lwchunkshashhead[i as usize];
            while !lwc.is_null() {
                lwcn = (*lwc).next as *mut lwchunks;
                free(lwc as *mut ::core::ffi::c_void);
                lwc = lwcn;
            }
            swchunkshash[i as usize] = ::core::ptr::null_mut::<swchunks>();
            lwchunkshashhead[i as usize] = ::core::ptr::null_mut::<lwchunks>();
            lwchunkshashtail[i as usize] = ::core::ptr::null_mut::<*mut lwchunks>();
            i = i.wrapping_add(1);
        }
        matoclserv_read(::core::ptr::null_mut::<matoclserventry>(), 0.0f64);
        matoclserv_gid_storage(0 as uint32_t);
        free(ListenHost as *mut ::core::ffi::c_void);
        free(ListenPort as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_no_more_pending_jobs() -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        let mut i: uint32_t = 0;
        eptr = matoclservhead;
        while !eptr.is_null() {
            if !(*eptr).outputhead.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            eptr = (*eptr).next as *mut matoclserventry;
        }
        i = 0 as uint32_t;
        while i < CHUNKHASHSIZE as uint32_t {
            if !swchunkshash[i as usize].is_null() {
                return 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_disconnect_all() {
    unsafe {
        let mut eptr: *mut matoclserventry = ::core::ptr::null_mut::<matoclserventry>();
        eptr = matoclservhead;
        while !eptr.is_null() {
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            eptr = (*eptr).next as *mut matoclserventry;
        }
        matoclserv_disconnection_loop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_reload_common() {
    unsafe {
        if !InstanceName.is_null() {
            free(InstanceName as *mut ::core::ffi::c_void);
        }
        InstanceName = cfg_getstr(
            b"INSTANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        InstanceNameLeng = strlen(InstanceName) as uint32_t;
        if InstanceNameLeng > 255 as uint32_t {
            InstanceNameLeng = 255 as uint32_t;
        }
        RestrictIncompatibleClientVersions = (if cfg_getuint32(
            b"RESTRICT_INCOMPATIBLE_CLIENT_VERSIONS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as uint32_t,
        ) != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        DefaultTimeout = cfg_getuint32(
            b"MATOCL_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            10 as uint32_t,
        );
        if DefaultTimeout > 65535 as uint32_t {
            DefaultTimeout = 65535 as uint32_t;
        } else if DefaultTimeout < 10 as uint32_t {
            DefaultTimeout = 10 as uint32_t;
        }
        ForceTimeout = cfg_getuint32(
            b"MATOCL_FORCE_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint32_t,
        );
        if ForceTimeout > 0 as uint32_t && ForceTimeout < 10 as uint32_t {
            ForceTimeout = 10 as uint32_t;
        }
        if ForceTimeout > 65535 as uint32_t {
            ForceTimeout = 65535 as uint32_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_reload() {
    unsafe {
        let mut oldListenHost: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oldListenPort: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oldlistenip: uint32_t = 0;
        let mut oldlistenport: uint16_t = 0;
        let mut newlsock: ::core::ffi::c_int = 0;
        matoclserv_reload_sessions();
        matoclserv_reload_common();
        oldListenHost = ListenHost;
        oldListenPort = ListenPort;
        oldlistenip = listenip;
        oldlistenport = listenport;
        if cfg_isdefined(b"MATOCL_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char) != 0
            || cfg_isdefined(b"MATOCL_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char) != 0
            || !(cfg_isdefined(b"MATOCU_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char) != 0
                || cfg_isdefined(b"MATOCU_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0)
        {
            ListenHost = cfg_getstr(
                b"MATOCL_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
                b"*\0".as_ptr() as *const ::core::ffi::c_char,
            );
            ListenPort = cfg_getstr(
                b"MATOCL_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
                DEFAULT_MASTER_CLIENT_PORT.as_ptr(),
            );
        } else {
            ListenHost = cfg_getstr(
                b"MATOCU_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
                b"*\0".as_ptr() as *const ::core::ffi::c_char,
            );
            ListenPort = cfg_getstr(
                b"MATOCU_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
                DEFAULT_MASTER_CLIENT_PORT.as_ptr(),
            );
        }
        if strcmp(oldListenHost, ListenHost) == 0 as ::core::ffi::c_int
            && strcmp(oldListenPort, ListenPort) == 0 as ::core::ffi::c_int
        {
            free(oldListenHost as *mut ::core::ffi::c_void);
            free(oldListenPort as *mut ::core::ffi::c_void);
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"main master server module: socket address hasn't changed (%s:%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            return;
        }
        newlsock = tcpsocket();
        if newlsock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"main master server module: socket address has changed, but can't create new socket\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            free(ListenHost as *mut ::core::ffi::c_void);
            free(ListenPort as *mut ::core::ffi::c_void);
            ListenHost = oldListenHost;
            ListenPort = oldListenPort;
            return;
        }
        tcpnonblock(newlsock);
        tcpnodelay(newlsock);
        tcpreuseaddr(newlsock);
        if tcpresolve(
            ListenHost,
            ListenPort,
            &raw mut listenip,
            &raw mut listenport,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"main master server module: socket address has changed, but can't be resolved (%s:%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            free(ListenHost as *mut ::core::ffi::c_void);
            free(ListenPort as *mut ::core::ffi::c_void);
            ListenHost = oldListenHost;
            ListenPort = oldListenPort;
            listenip = oldlistenip;
            listenport = oldlistenport;
            tcpclose(newlsock);
            return;
        }
        if tcpnumlisten(newlsock, listenip, listenport, 100 as uint16_t) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"main master server module: socket address has changed, but can't listen on socket (%s:%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            free(ListenHost as *mut ::core::ffi::c_void);
            free(ListenPort as *mut ::core::ffi::c_void);
            ListenHost = oldListenHost;
            ListenPort = oldListenPort;
            listenip = oldlistenip;
            listenport = oldlistenport;
            tcpclose(newlsock);
            return;
        }
        if tcpsetacceptfilter(newlsock) < 0 as ::core::ffi::c_int && *__errno_location() != ENOTSUP
        {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"main master server module: can't set accept filter\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"main master server module: socket address has changed, now listen on %s:%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        free(oldListenHost as *mut ::core::ffi::c_void);
        free(oldListenPort as *mut ::core::ffi::c_void);
        tcpclose(lsock);
        lsock = newlsock;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matoclserv_init() -> ::core::ffi::c_int {
    unsafe {
        InstanceName = ::core::ptr::null_mut::<::core::ffi::c_char>();
        InstanceNameLeng = 0 as uint32_t;
        master_processid = time(::core::ptr::null_mut::<time_t>()) as uint64_t;
        master_processid <<= 32 as ::core::ffi::c_int;
        master_processid |= random() as uint64_t;
        matoclserv_reload_common();
        if cfg_isdefined(b"MATOCL_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char) != 0
            || cfg_isdefined(b"MATOCL_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char) != 0
            || !(cfg_isdefined(b"MATOCU_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char) != 0
                || cfg_isdefined(b"MATOCU_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0)
        {
            ListenHost = cfg_getstr(
                b"MATOCL_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
                b"*\0".as_ptr() as *const ::core::ffi::c_char,
            );
            ListenPort = cfg_getstr(
                b"MATOCL_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
                DEFAULT_MASTER_CLIENT_PORT.as_ptr(),
            );
        } else {
            fprintf(
                stderr,
                b"change MATOCU_LISTEN_* option names to MATOCL_LISTEN_* !!!\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            ListenHost = cfg_getstr(
                b"MATOCU_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
                b"*\0".as_ptr() as *const ::core::ffi::c_char,
            );
            ListenPort = cfg_getstr(
                b"MATOCU_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
                DEFAULT_MASTER_CLIENT_PORT.as_ptr(),
            );
        }
        CreateFirstChunk = 0 as uint8_t;
        lsock = tcpsocket();
        if lsock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"main master server module: can't create socket\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        tcpnonblock(lsock);
        tcpnodelay(lsock);
        tcpreuseaddr(lsock);
        if tcpresolve(
            ListenHost,
            ListenPort,
            &raw mut listenip,
            &raw mut listenport,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"main master server module: can't resolve %s:%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpnumlisten(lsock, listenip, listenport, 100 as uint16_t) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"main master server module: can't listen on %s:%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpsetacceptfilter(lsock) < 0 as ::core::ffi::c_int && *__errno_location() != ENOTSUP {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"main master server module: can't set accept filter\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"main master server module: listen on %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        matoclservhead = ::core::ptr::null_mut::<matoclserventry>();
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(matoclserv_timeout_waiting_ops as unsafe extern "C" fn() -> ()),
            b"matoclserv_timeout_waiting_ops\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_reload_register_fname(
            Some(matoclserv_reload as unsafe extern "C" fn() -> ()),
            b"matoclserv_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_destruct_register_fname(
            Some(matoclserv_term as unsafe extern "C" fn() -> ()),
            b"matoclserv_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_poll_register_fname(
            Some(matoclserv_desc as unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()),
            Some(matoclserv_serve as unsafe extern "C" fn(*mut pollfd) -> ()),
            b"matoclserv_desc\0".as_ptr() as *const ::core::ffi::c_char,
            b"matoclserv_serve\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_keepalive_register_fname(
            Some(matoclserv_keep_alive as unsafe extern "C" fn() -> ()),
            b"matoclserv_keep_alive\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            10 as uint32_t,
            0 as uint32_t,
            Some(matoclserv_broadcast_timeout as unsafe extern "C" fn() -> ()),
            b"matoclserv_broadcast_timeout\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
