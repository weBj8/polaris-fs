pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum _bio {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
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
    unsafe fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_skip(b: *mut bio, len: uint64_t);
    unsafe fn patterns_sclass_delete(scid: uint8_t);
    unsafe fn chunk_sclass_inc_counters(
        sclassid: uint8_t,
        flags: uint8_t,
        rlevel: uint8_t,
        counters: *mut uint64_t,
    );
    unsafe fn chunk_sclass_has_chunks(sclassid: uint8_t) -> uint8_t;
    unsafe fn chunk_labelset_can_be_fulfilled(sm: *mut storagemode) -> uint8_t;
    unsafe fn chunk_labelset_fix_matching_servers(sm: *mut storagemode);
    unsafe fn matocsserv_servers_matches_labelexpr(labelexpr: *const uint8_t) -> uint16_t;
    unsafe fn matocsserv_servers_count() -> uint16_t;
    unsafe fn matocsserv_get_min_cs_version() -> uint32_t;
    unsafe fn matoclserv_get_min_cl_version() -> uint32_t;
    unsafe fn meta_version_inc() -> uint64_t;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn changelog(format: *const ::core::ffi::c_char, ...);
    unsafe fn changelog_escape_name(
        nleng: uint32_t,
        name: *const uint8_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn main_time() -> uint32_t;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
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
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
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
pub type storageclass = _storageclass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _storageclass {
    pub nleng: uint8_t,
    pub dleng: uint8_t,
    pub name: [uint8_t; 256],
    pub desc: [uint8_t; 256],
    pub priority: uint32_t,
    pub export_group: uint8_t,
    pub admin_only: uint8_t,
    pub arch_mode: uint8_t,
    pub labels_mode: uint8_t,
    pub create: storagemode,
    pub keep: storagemode,
    pub arch: storagemode,
    pub trash: storagemode,
    pub arch_delay: uint16_t,
    pub min_trashretention: uint16_t,
    pub arch_min_size: uint64_t,
    pub files: uint32_t,
    pub directories: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MASKORGROUP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAXSCLASS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const SCLASS_EXPR_MAX_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MAXSCLASSNAMELENG: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAXSCLASSDESCLENG: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAXLABELSCNT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_ERROR_CLASSEXISTS: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const MFS_ERROR_CLASSLIMITREACH: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSUCHCLASS: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const MFS_ERROR_CLASSINUSE: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const MFS_ERROR_INCOMPATVERSION: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SCLASS_CHG_ADMIN_ONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SCLASS_CHG_LABELS_MODE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SCLASS_CHG_CREATE_MASKS: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SCLASS_CHG_KEEP_MASKS: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SCLASS_CHG_ARCH_MASKS: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SCLASS_CHG_ARCH_DELAY: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SCLASS_CHG_TRASH_MASKS: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SCLASS_CHG_MIN_TRASHRETENTION: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SCLASS_CHG_ARCH_MODE: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const SCLASS_CHG_ARCH_MIN_SIZE: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const SCLASS_CHG_PRIORITY: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const SCLASS_CHG_DESCRIPTION: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const SCLASS_CHG_EXPORT_GROUP: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const SCLASS_ARCH_MODE_CTIME: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SCLASS_EXPR_TYPE_MASK: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
pub const SCLASS_EXPR_VALUE_MASK: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
pub const SCLASS_EXPR_SYMBOL: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
pub const SCLASS_EXPR_SYMBOL_ANY: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
pub const SCLASS_EXPR_OP_AND: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SCLASS_EXPR_OP_OR: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const TYPE_FILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TYPE_DIRECTORY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TYPE_TRASH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TYPE_SUSTAINED: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const LABELS_MODE_LOOSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LABELS_MODE_STD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LABELS_MODE_STRICT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LABELS_MODE_GLOBAL: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
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
pub const CHLOGSTRSIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int
    * MAXLABELSCNT
    * (SCLASS_EXPR_MAX_SIZE * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    + 1 as ::core::ffi::c_int;
pub const MAX_EC_LEVEL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const REDUCED_EC_LEVEL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COMPAT_ECMODE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut tmp_storagemode: storagemode = storagemode {
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
static mut sclasstab: [storageclass; 256] = [storageclass {
    nleng: 0,
    dleng: 0,
    name: [0; 256],
    desc: [0; 256],
    priority: 0,
    export_group: 0,
    admin_only: 0,
    arch_mode: 0,
    labels_mode: 0,
    create: storagemode {
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
    },
    keep: storagemode {
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
    },
    arch: storagemode {
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
    },
    trash: storagemode {
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
    },
    arch_delay: 0,
    min_trashretention: 0,
    arch_min_size: 0,
    files: 0,
    directories: 0,
}; 256];
static mut firstneverused: uint32_t = 0 as uint32_t;
static mut ec_current_version: uint8_t = 0 as uint8_t;
static mut MaxECRedundancyLevel: uint8_t = 1 as uint8_t;
static mut DefaultECMODE: uint8_t = 8 as uint8_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_maskorgroup_to_labelexpr(
    mut labelexpr: *mut [uint8_t; 128],
    mut labelmasks: *mut uint32_t,
    mut labelscnt: uint8_t,
) {
    unsafe {
        let mut mask: uint32_t = 0;
        let mut b: uint8_t = 0;
        let mut ands: uint8_t = 0;
        let mut ors: uint8_t = 0;
        let mut i: uint8_t = 0;
        let mut exprpos: uint8_t = 0;
        if labelscnt as ::core::ffi::c_int > MAXLABELSCNT {
            labelscnt = MAXLABELSCNT as uint8_t;
        }
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < labelscnt as ::core::ffi::c_int {
            memset(
                &raw mut *labelexpr.offset(i as isize) as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                SCLASS_EXPR_MAX_SIZE as size_t,
            );
            exprpos = 0 as uint8_t;
            ors = 0 as uint8_t;
            while (ors as ::core::ffi::c_int) < 4 as ::core::ffi::c_int
                && *labelmasks.offset(
                    (i as ::core::ffi::c_int * MASKORGROUP + ors as ::core::ffi::c_int) as isize,
                ) != 0 as uint32_t
            {
                ands = 0 as uint8_t;
                b = 0 as uint8_t;
                mask = 1 as uint32_t;
                while (b as ::core::ffi::c_int) < 26 as ::core::ffi::c_int {
                    if *labelmasks.offset(
                        (i as ::core::ffi::c_int * MASKORGROUP + ors as ::core::ffi::c_int)
                            as isize,
                    ) & mask
                        != 0
                    {
                        let c2rust_fresh0 = exprpos;
                        exprpos = exprpos.wrapping_add(1);
                        (*labelexpr.offset(i as isize))[c2rust_fresh0 as usize] =
                            (SCLASS_EXPR_SYMBOL + b as ::core::ffi::c_int) as uint8_t;
                        ands = ands.wrapping_add(1);
                    }
                    b = b.wrapping_add(1);
                    mask <<= 1 as ::core::ffi::c_int;
                }
                if ands as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    let c2rust_fresh1 = exprpos;
                    exprpos = exprpos.wrapping_add(1);
                    (*labelexpr.offset(i as isize))[c2rust_fresh1 as usize] = (SCLASS_EXPR_OP_AND
                        + (ands as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
                        as uint8_t;
                }
                ors = ors.wrapping_add(1);
            }
            if ors as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                let c2rust_fresh2 = exprpos;
                exprpos = exprpos.wrapping_add(1);
                (*labelexpr.offset(i as isize))[c2rust_fresh2 as usize] = (SCLASS_EXPR_OP_OR
                    + (ors as ::core::ffi::c_int - 2 as ::core::ffi::c_int))
                    as uint8_t;
            }
            let c2rust_fresh3 = exprpos;
            exprpos = exprpos.wrapping_add(1);
            (*labelexpr.offset(i as isize))[c2rust_fresh3 as usize] = 0 as uint8_t;
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_labelexpr_to_maskorgroup(
    mut labelmasks: *mut uint32_t,
    mut labelexpr: *mut [uint8_t; 128],
    mut labelscnt: uint8_t,
) -> uint8_t {
    unsafe {
        let mut mask: uint32_t = 0;
        let mut i: uint8_t = 0;
        let mut ors: uint8_t = 0;
        let mut ands: uint8_t = 0;
        let mut exprpos: uint8_t = 0;
        if labelscnt as ::core::ffi::c_int > MAXLABELSCNT {
            return 0 as uint8_t;
        }
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < labelscnt as ::core::ffi::c_int {
            exprpos = 0 as uint8_t;
            while (exprpos as ::core::ffi::c_int) < SCLASS_EXPR_MAX_SIZE
                && (*labelexpr.offset(i as isize))[exprpos as usize] as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                exprpos = exprpos.wrapping_add(1);
            }
            if exprpos as ::core::ffi::c_int >= SCLASS_EXPR_MAX_SIZE {
                return 0 as uint8_t;
            }
            memset(
                labelmasks.offset((i as ::core::ffi::c_int * MASKORGROUP) as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<uint32_t>().wrapping_mul(MASKORGROUP as size_t),
            );
            if exprpos as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if (*labelexpr.offset(i as isize))
                    [(exprpos as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    & SCLASS_EXPR_TYPE_MASK
                    == SCLASS_EXPR_OP_OR
                {
                    exprpos = exprpos.wrapping_sub(1);
                    ors = (((*labelexpr.offset(i as isize))[exprpos as usize]
                        as ::core::ffi::c_int
                        & SCLASS_EXPR_VALUE_MASK)
                        + 2 as ::core::ffi::c_int) as uint8_t;
                } else {
                    ors = 1 as uint8_t;
                }
                while ors as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    ors = ors.wrapping_sub(1);
                    if exprpos as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        return 0 as uint8_t;
                    }
                    if (*labelexpr.offset(i as isize))
                        [(exprpos as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        & SCLASS_EXPR_TYPE_MASK
                        == SCLASS_EXPR_OP_AND
                    {
                        exprpos = exprpos.wrapping_sub(1);
                        ands = (((*labelexpr.offset(i as isize))[exprpos as usize]
                            as ::core::ffi::c_int
                            & SCLASS_EXPR_VALUE_MASK)
                            + 2 as ::core::ffi::c_int) as uint8_t;
                    } else {
                        ands = 1 as uint8_t;
                    }
                    mask = 0 as uint32_t;
                    while ands as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        ands = ands.wrapping_sub(1);
                        if exprpos as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            return 0 as uint8_t;
                        }
                        exprpos = exprpos.wrapping_sub(1);
                        if (*labelexpr.offset(i as isize))[exprpos as usize] as ::core::ffi::c_int
                            & SCLASS_EXPR_TYPE_MASK
                            == SCLASS_EXPR_SYMBOL
                        {
                            mask |= ((1 as ::core::ffi::c_int)
                                << ((*labelexpr.offset(i as isize))[exprpos as usize]
                                    as ::core::ffi::c_int
                                    & SCLASS_EXPR_VALUE_MASK))
                                as uint32_t;
                        } else {
                            return 0 as uint8_t;
                        }
                    }
                    *labelmasks.offset(
                        (i as ::core::ffi::c_int * MASKORGROUP + ors as ::core::ffi::c_int)
                            as isize,
                    ) = mask;
                }
                if exprpos as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    return 0 as uint8_t;
                }
            }
            i = i.wrapping_add(1);
        }
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_ec_version() -> uint8_t {
    unsafe {
        return ec_current_version;
    }
}
unsafe extern "C" fn sclass_check_ec(mut ec_new_version: uint8_t) -> uint8_t {
    unsafe {
        if ec_current_version as ::core::ffi::c_int >= ec_new_version as ::core::ffi::c_int {
            return 1 as uint8_t;
        }
        if ec_new_version as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            if matocsserv_get_min_cs_version()
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                return 0 as uint8_t;
            }
            if matoclserv_get_min_cl_version()
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                return 0 as uint8_t;
            }
        }
        if ec_new_version as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
            if matocsserv_get_min_cs_version()
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 26 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                return 0 as uint8_t;
            }
            if matoclserv_get_min_cl_version()
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 26 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                return 0 as uint8_t;
            }
        }
        changelog(
            b"%u|SCECVERSION(%u)\0".as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            ec_new_version as ::core::ffi::c_int,
        );
        ec_current_version = ec_new_version;
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_mr_ec_version(mut ec_new_version: uint8_t) -> uint8_t {
    unsafe {
        if ec_current_version as ::core::ffi::c_int >= ec_new_version as ::core::ffi::c_int {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        meta_version_inc();
        ec_current_version = ec_new_version;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn sclass_name_check(mut nleng: uint8_t, mut name: *const uint8_t) -> uint8_t {
    unsafe {
        let mut i: uint8_t = 0;
        if nleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as uint8_t;
        }
        if *name.offset(0 as isize) as ::core::ffi::c_int == 32 as ::core::ffi::c_int {
            return 0 as uint8_t;
        }
        if *name.offset((nleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            == 32 as ::core::ffi::c_int
        {
            return 0 as uint8_t;
        }
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < nleng as ::core::ffi::c_int {
            if (*name.offset(i as isize) as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                return 0 as uint8_t;
            }
            i = i.wrapping_add(1);
        }
        return 1 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn sclass_make_changelog_mode(
    mut m: *mut storagemode,
    mut buff: *mut ::core::ffi::c_char,
    mut maxleng: uint32_t,
) -> uint32_t {
    unsafe {
        let mut leng: uint32_t = 0 as uint32_t;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        i = 0 as uint32_t;
        while i < (*m).labelscnt as uint32_t {
            j = 0 as uint32_t;
            while j < SCLASS_EXPR_MAX_SIZE as uint32_t
                && (*m).labelexpr[i as usize][j as usize] as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                if leng < maxleng {
                    leng = leng.wrapping_add(snprintf(
                        buff.offset(leng as isize),
                        maxleng.wrapping_sub(leng) as size_t,
                        b"%02hhX\0".as_ptr() as *const ::core::ffi::c_char,
                        (*m).labelexpr[i as usize][j as usize] as ::core::ffi::c_int,
                    ) as uint32_t);
                }
                j = j.wrapping_add(1);
            }
            if leng < maxleng {
                let c2rust_fresh4 = leng;
                leng = leng.wrapping_add(1);
                *buff.offset(c2rust_fresh4 as isize) = ',' as ::core::ffi::c_char;
            }
            i = i.wrapping_add(1);
        }
        return leng;
    }
}
#[inline]
unsafe extern "C" fn sclass_make_changelog(mut sclassid: uint16_t, mut new_flag: uint8_t) {
    unsafe {
        let mut chlogstr: [::core::ffi::c_char; 9253] = [0; 9253];
        let mut chlogstrleng: ::core::ffi::c_int = 0;
        chlogstrleng = 0 as ::core::ffi::c_int;
        chlogstrleng = (chlogstrleng as uint32_t).wrapping_add(sclass_make_changelog_mode(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).create,
            (&raw mut chlogstr as *mut ::core::ffi::c_char).offset(chlogstrleng as isize),
            (CHLOGSTRSIZE - chlogstrleng) as uint32_t,
        )) as ::core::ffi::c_int;
        chlogstrleng = (chlogstrleng as uint32_t).wrapping_add(sclass_make_changelog_mode(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).keep,
            (&raw mut chlogstr as *mut ::core::ffi::c_char).offset(chlogstrleng as isize),
            (CHLOGSTRSIZE - chlogstrleng) as uint32_t,
        )) as ::core::ffi::c_int;
        chlogstrleng = (chlogstrleng as uint32_t).wrapping_add(sclass_make_changelog_mode(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).arch,
            (&raw mut chlogstr as *mut ::core::ffi::c_char).offset(chlogstrleng as isize),
            (CHLOGSTRSIZE - chlogstrleng) as uint32_t,
        )) as ::core::ffi::c_int;
        chlogstrleng = (chlogstrleng as uint32_t).wrapping_add(sclass_make_changelog_mode(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).trash,
            (&raw mut chlogstr as *mut ::core::ffi::c_char).offset(chlogstrleng as isize),
            (CHLOGSTRSIZE - chlogstrleng) as uint32_t,
        )) as ::core::ffi::c_int;
        if chlogstrleng > 0 as ::core::ffi::c_int {
            chlogstr[(chlogstrleng - 1 as ::core::ffi::c_int) as usize] =
                '\0' as ::core::ffi::c_char;
        } else {
            chlogstr[0 as usize] = '-' as ::core::ffi::c_char;
            chlogstr[1 as usize] = '\0' as ::core::ffi::c_char;
        }
        changelog(
            b"%u|SCSET(%s,%hhu,1,%s,%u,%hhu,C(%hhu:%u:%hhu),K(%hhu:%u:%hhu),A(%hhu:%u:%hhu:%hhu),T(%hhu:%u:%hhu:%hhu),%hhu,(%hhu:%hu:%lu),%hu,%hhu,%s):%hu\0"
                .as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            changelog_escape_name(
                sclasstab[sclassid as usize].nleng as uint32_t,
                &raw mut (*(&raw mut sclasstab as *mut storageclass)
                    .offset(sclassid as isize))
                    .name as *mut uint8_t,
            ),
            new_flag as ::core::ffi::c_int,
            changelog_escape_name(
                sclasstab[sclassid as usize].dleng as uint32_t,
                &raw mut (*(&raw mut sclasstab as *mut storageclass)
                    .offset(sclassid as isize))
                    .desc as *mut uint8_t,
            ),
            sclasstab[sclassid as usize].priority,
            sclasstab[sclassid as usize].export_group as ::core::ffi::c_int,
            sclasstab[sclassid as usize].create.labelscnt as ::core::ffi::c_int,
            sclasstab[sclassid as usize].create.uniqmask,
            sclasstab[sclassid as usize].create.labels_mode as ::core::ffi::c_int,
            sclasstab[sclassid as usize].keep.labelscnt as ::core::ffi::c_int,
            sclasstab[sclassid as usize].keep.uniqmask,
            sclasstab[sclassid as usize].keep.labels_mode as ::core::ffi::c_int,
            sclasstab[sclassid as usize].arch.labelscnt as ::core::ffi::c_int,
            sclasstab[sclassid as usize].arch.uniqmask,
            sclasstab[sclassid as usize].arch.ec_data_chksum_parts as ::core::ffi::c_int,
            sclasstab[sclassid as usize].arch.labels_mode as ::core::ffi::c_int,
            sclasstab[sclassid as usize].trash.labelscnt as ::core::ffi::c_int,
            sclasstab[sclassid as usize].trash.uniqmask,
            sclasstab[sclassid as usize].trash.ec_data_chksum_parts
                as ::core::ffi::c_int,
            sclasstab[sclassid as usize].trash.labels_mode as ::core::ffi::c_int,
            sclasstab[sclassid as usize].labels_mode as ::core::ffi::c_int,
            sclasstab[sclassid as usize].arch_mode as ::core::ffi::c_int,
            sclasstab[sclassid as usize].arch_delay as ::core::ffi::c_int,
            sclasstab[sclassid as usize].arch_min_size,
            sclasstab[sclassid as usize].min_trashretention as ::core::ffi::c_int,
            sclasstab[sclassid as usize].admin_only as ::core::ffi::c_int,
            &raw mut chlogstr as *mut ::core::ffi::c_char,
            sclassid as ::core::ffi::c_int,
        );
    }
}
#[inline]
unsafe extern "C" fn sclass_mode_fix_has_labels(mut m: *mut storagemode) {
    unsafe {
        let mut i: uint32_t = 0;
        if (*m).uniqmask != 0 {
            (*m).has_labels = 1 as uint8_t;
        } else {
            (*m).has_labels = 0 as uint8_t;
            i = 0 as uint32_t;
            while i < (*m).labelscnt as uint32_t {
                if !((*m).labelexpr[i as usize][0 as usize] as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    || (*m).labelexpr[i as usize][0 as usize] as ::core::ffi::c_int
                        == SCLASS_EXPR_SYMBOL_ANY
                        && (*m).labelexpr[i as usize][1 as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int)
                {
                    (*m).has_labels = 1 as uint8_t;
                    return;
                }
                i = i.wrapping_add(1);
            }
        };
    }
}
#[inline]
unsafe extern "C" fn sclass_fix_has_labels_fields(mut sclassid: uint8_t) {
    unsafe {
        sclass_mode_fix_has_labels(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).create,
        );
        sclass_mode_fix_has_labels(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).keep,
        );
        sclass_mode_fix_has_labels(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).arch,
        );
        sclass_mode_fix_has_labels(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).trash,
        );
    }
}
unsafe extern "C" fn sclass_fix_matching_servers_fields() {
    unsafe {
        let mut i: uint32_t = 0;
        chunk_labelset_fix_matching_servers(::core::ptr::null_mut::<storagemode>());
        i = 1 as uint32_t;
        while i < firstneverused {
            if sclasstab[i as usize].nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                chunk_labelset_fix_matching_servers(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).create,
                );
                chunk_labelset_fix_matching_servers(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).keep,
                );
                chunk_labelset_fix_matching_servers(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).arch,
                );
                chunk_labelset_fix_matching_servers(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).trash,
                );
            }
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_create_entry(
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut dleng: uint8_t,
    mut desc: *const uint8_t,
    mut priority: uint32_t,
    mut export_group: uint8_t,
    mut admin_only: uint8_t,
    mut labels_mode: uint8_t,
    mut arch_mode: uint8_t,
    mut arch_delay: uint16_t,
    mut arch_min_size: uint64_t,
    mut min_trashretention: uint16_t,
    mut create: *mut storagemode,
    mut keep: *mut storagemode,
    mut arch: *mut storagemode,
    mut trash: *mut storagemode,
) -> uint8_t {
    unsafe {
        let mut sclassid: uint32_t = 0;
        let mut fsclassid: uint32_t = 0;
        if sclass_name_check(nleng, name) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && sclass_name_check(dleng, desc) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        fsclassid = 0 as uint32_t;
        sclassid = 1 as uint32_t;
        while sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == nleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    nleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_CLASSEXISTS as uint8_t;
            }
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && fsclassid == 0 as uint32_t
            {
                fsclassid = sclassid;
            }
            sclassid = sclassid.wrapping_add(1);
        }
        if fsclassid == 0 as uint32_t {
            if firstneverused == MAXSCLASS as uint32_t {
                return MFS_ERROR_CLASSLIMITREACH as uint8_t;
            }
            fsclassid = firstneverused;
            firstneverused = firstneverused.wrapping_add(1);
        }
        if (*create).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            || (*keep).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            || (*create).labelscnt as ::core::ffi::c_int > MAXLABELSCNT
            || (*keep).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*keep).labelscnt as ::core::ffi::c_int > MAXLABELSCNT
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (*arch).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if sclass_check_ec(1 as uint8_t) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return MFS_ERROR_INCOMPATVERSION as uint8_t;
            }
            if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int != 0 {
                if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    != 4 as ::core::ffi::c_int
                    && (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        != 8 as ::core::ffi::c_int
                {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
            } else {
                (*arch).ec_data_chksum_parts = ((*arch).ec_data_chksum_parts as ::core::ffi::c_int
                    | (DefaultECMODE as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as uint8_t;
            }
            if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                == 4 as ::core::ffi::c_int
            {
                if sclass_check_ec(2 as uint8_t) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
            }
            if (*arch).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*arch).labelscnt as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                || (*arch).ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                    > MaxECRedundancyLevel as ::core::ffi::c_int
            {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        } else if (*arch).labelscnt as ::core::ffi::c_int > MAXLABELSCNT {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (*trash).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if sclass_check_ec(1 as uint8_t) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return MFS_ERROR_INCOMPATVERSION as uint8_t;
            }
            if (*trash).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int != 0 {
                if (*trash).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    != 4 as ::core::ffi::c_int
                    && (*trash).ec_data_chksum_parts as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int
                        != 8 as ::core::ffi::c_int
                {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
            } else {
                (*trash).ec_data_chksum_parts = ((*trash).ec_data_chksum_parts
                    as ::core::ffi::c_int
                    | (DefaultECMODE as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                    as uint8_t;
            }
            if (*trash).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                == 4 as ::core::ffi::c_int
            {
                if sclass_check_ec(2 as uint8_t) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
            }
            if (*trash).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*trash).labelscnt as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                || (*trash).ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                    > MaxECRedundancyLevel as ::core::ffi::c_int
            {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        } else if (*trash).labelscnt as ::core::ffi::c_int > MAXLABELSCNT {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if arch_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || arch_mode as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        sclasstab[fsclassid as usize].nleng = nleng;
        memcpy(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fsclassid as isize)).name
                as *mut uint8_t as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        sclasstab[fsclassid as usize].dleng = dleng;
        if dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            memcpy(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fsclassid as isize))
                    .desc as *mut uint8_t as *mut ::core::ffi::c_void,
                desc as *const ::core::ffi::c_void,
                dleng as size_t,
            );
        }
        sclasstab[fsclassid as usize].priority = priority;
        sclasstab[fsclassid as usize].export_group = export_group;
        sclasstab[fsclassid as usize].admin_only = admin_only;
        sclasstab[fsclassid as usize].labels_mode = labels_mode;
        sclasstab[fsclassid as usize].arch_mode = arch_mode;
        sclasstab[fsclassid as usize].create = *create;
        sclasstab[fsclassid as usize].keep = *keep;
        sclasstab[fsclassid as usize].arch = *arch;
        sclasstab[fsclassid as usize].trash = *trash;
        sclasstab[fsclassid as usize].arch_delay = arch_delay;
        sclasstab[fsclassid as usize].min_trashretention = min_trashretention;
        sclasstab[fsclassid as usize].arch_min_size = arch_min_size;
        sclass_fix_has_labels_fields(fsclassid as uint8_t);
        sclass_make_changelog(fsclassid as uint16_t, 1 as uint8_t);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_change_entry(
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut chgmask: uint16_t,
    mut dleng: *mut uint8_t,
    mut desc: *mut uint8_t,
    mut priority: *mut uint32_t,
    mut export_group: *mut uint8_t,
    mut admin_only: *mut uint8_t,
    mut labels_mode: *mut uint8_t,
    mut arch_mode: *mut uint8_t,
    mut arch_delay: *mut uint16_t,
    mut arch_min_size: *mut uint64_t,
    mut min_trashretention: *mut uint16_t,
    mut create: *mut storagemode,
    mut keep: *mut storagemode,
    mut arch: *mut storagemode,
    mut trash: *mut storagemode,
) -> uint8_t {
    unsafe {
        let mut sclassid: uint32_t = 0;
        let mut fsclassid: uint32_t = 0;
        if sclass_name_check(nleng, name) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        fsclassid = 0 as uint32_t;
        sclassid = 1 as uint32_t;
        while fsclassid == 0 as uint32_t && sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == nleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    nleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                fsclassid = sclassid;
            }
            sclassid = sclassid.wrapping_add(1);
        }
        if fsclassid == 0 as uint32_t {
            return MFS_ERROR_NOSUCHCLASS as uint8_t;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_CREATE_MASKS != 0
            && ((*create).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                || (*create).labelscnt as ::core::ffi::c_int > MAXLABELSCNT)
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_KEEP_MASKS != 0
            && ((*keep).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                || (*keep).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*keep).labelscnt as ::core::ffi::c_int > MAXLABELSCNT)
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_ARCH_MASKS != 0 {
            if (*arch).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if sclass_check_ec(1 as uint8_t) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
                if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    != 0
                {
                    if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        != 4 as ::core::ffi::c_int
                        && (*arch).ec_data_chksum_parts as ::core::ffi::c_int
                            >> 4 as ::core::ffi::c_int
                            != 8 as ::core::ffi::c_int
                    {
                        return MFS_ERROR_EINVAL as uint8_t;
                    }
                } else {
                    (*arch).ec_data_chksum_parts = ((*arch).ec_data_chksum_parts
                        as ::core::ffi::c_int
                        | (DefaultECMODE as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        as uint8_t;
                }
                if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    == 4 as ::core::ffi::c_int
                {
                    if sclass_check_ec(2 as uint8_t) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        return MFS_ERROR_INCOMPATVERSION as uint8_t;
                    }
                }
                if (*arch).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || (*arch).labelscnt as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                    || (*arch).ec_data_chksum_parts as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int
                        > MaxECRedundancyLevel as ::core::ffi::c_int
                {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
            } else if (*arch).labelscnt as ::core::ffi::c_int > MAXLABELSCNT {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_TRASH_MASKS != 0 {
            if (*trash).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if sclass_check_ec(1 as uint8_t) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return MFS_ERROR_INCOMPATVERSION as uint8_t;
                }
                if (*trash).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    != 0
                {
                    if (*trash).ec_data_chksum_parts as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int
                        != 4 as ::core::ffi::c_int
                        && (*trash).ec_data_chksum_parts as ::core::ffi::c_int
                            >> 4 as ::core::ffi::c_int
                            != 8 as ::core::ffi::c_int
                    {
                        return MFS_ERROR_EINVAL as uint8_t;
                    }
                } else {
                    (*trash).ec_data_chksum_parts = ((*trash).ec_data_chksum_parts
                        as ::core::ffi::c_int
                        | (DefaultECMODE as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
                        as uint8_t;
                }
                if (*trash).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    == 4 as ::core::ffi::c_int
                {
                    if sclass_check_ec(2 as uint8_t) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        return MFS_ERROR_INCOMPATVERSION as uint8_t;
                    }
                }
                if (*trash).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || (*trash).labelscnt as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                    || (*trash).ec_data_chksum_parts as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int
                        > MaxECRedundancyLevel as ::core::ffi::c_int
                {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
            } else if (*trash).labelscnt as ::core::ffi::c_int > MAXLABELSCNT {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_ARCH_MODE != 0
            && (*arch_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || *arch_mode as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int)
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_DESCRIPTION != 0 {
            if *dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && sclass_name_check(*dleng, desc) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_EINVAL as uint8_t;
            }
            sclasstab[fsclassid as usize].dleng = *dleng;
            if *dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                memcpy(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fsclassid as isize))
                        .desc as *mut uint8_t as *mut ::core::ffi::c_void,
                    desc as *const ::core::ffi::c_void,
                    *dleng as size_t,
                );
            }
        } else {
            *dleng = sclasstab[fsclassid as usize].dleng;
            if *dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                memcpy(
                    desc as *mut ::core::ffi::c_void,
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fsclassid as isize))
                        .desc as *mut uint8_t as *const ::core::ffi::c_void,
                    *dleng as size_t,
                );
            }
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_PRIORITY != 0 {
            sclasstab[fsclassid as usize].priority = *priority;
        } else {
            *priority = sclasstab[fsclassid as usize].priority;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_EXPORT_GROUP != 0 {
            sclasstab[fsclassid as usize].export_group = *export_group;
        } else {
            *export_group = sclasstab[fsclassid as usize].export_group;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_ADMIN_ONLY != 0 {
            sclasstab[fsclassid as usize].admin_only = *admin_only;
        } else {
            *admin_only = sclasstab[fsclassid as usize].admin_only;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_LABELS_MODE != 0 {
            sclasstab[fsclassid as usize].labels_mode = *labels_mode;
        } else {
            *labels_mode = sclasstab[fsclassid as usize].labels_mode;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_ARCH_MODE != 0 {
            sclasstab[fsclassid as usize].arch_mode = *arch_mode;
        } else {
            *arch_mode = sclasstab[fsclassid as usize].arch_mode;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_CREATE_MASKS != 0 {
            sclasstab[fsclassid as usize].create = *create;
        } else {
            *create = sclasstab[fsclassid as usize].create;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_KEEP_MASKS != 0 {
            sclasstab[fsclassid as usize].keep = *keep;
        } else {
            *keep = sclasstab[fsclassid as usize].keep;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_ARCH_MASKS != 0 {
            sclasstab[fsclassid as usize].arch = *arch;
        } else {
            *arch = sclasstab[fsclassid as usize].arch;
            if (*arch).ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                > MaxECRedundancyLevel as ::core::ffi::c_int
            {
                (*arch).ec_data_chksum_parts = ((*arch).ec_data_chksum_parts as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                    as uint8_t;
            }
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_TRASH_MASKS != 0 {
            sclasstab[fsclassid as usize].trash = *trash;
        } else {
            *trash = sclasstab[fsclassid as usize].trash;
            if (*trash).ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                > MaxECRedundancyLevel as ::core::ffi::c_int
            {
                (*trash).ec_data_chksum_parts = ((*trash).ec_data_chksum_parts
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                    as uint8_t;
            }
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_ARCH_DELAY != 0 {
            sclasstab[fsclassid as usize].arch_delay = *arch_delay;
        } else {
            *arch_delay = sclasstab[fsclassid as usize].arch_delay;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_MIN_TRASHRETENTION != 0 {
            sclasstab[fsclassid as usize].min_trashretention = *min_trashretention;
        } else {
            *min_trashretention = sclasstab[fsclassid as usize].min_trashretention;
        }
        if chgmask as ::core::ffi::c_int & SCLASS_CHG_ARCH_MIN_SIZE != 0 {
            sclasstab[fsclassid as usize].arch_min_size = *arch_min_size;
        } else {
            *arch_min_size = sclasstab[fsclassid as usize].arch_min_size;
        }
        if chgmask as ::core::ffi::c_int
            & (SCLASS_CHG_CREATE_MASKS
                | SCLASS_CHG_KEEP_MASKS
                | SCLASS_CHG_ARCH_MASKS
                | SCLASS_CHG_TRASH_MASKS)
            != 0
        {
            sclass_fix_has_labels_fields(fsclassid as uint8_t);
        }
        if chgmask as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            sclass_make_changelog(fsclassid as uint16_t, 0 as uint8_t);
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_mr_set_entry(
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut esclassid: uint16_t,
    mut new_flag: uint8_t,
    mut dleng: uint8_t,
    mut desc: *const uint8_t,
    mut priority: uint32_t,
    mut export_group: uint8_t,
    mut admin_only: uint8_t,
    mut labels_mode: uint8_t,
    mut arch_mode: uint8_t,
    mut arch_delay: uint16_t,
    mut arch_min_size: uint64_t,
    mut min_trashretention: uint16_t,
    mut create: *mut storagemode,
    mut keep: *mut storagemode,
    mut arch: *mut storagemode,
    mut trash: *mut storagemode,
) -> uint8_t {
    unsafe {
        let mut sclassid: uint32_t = 0;
        let mut fsclassid: uint32_t = 0;
        if sclass_name_check(nleng, name) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && sclass_name_check(dleng, desc) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        fsclassid = 0 as uint32_t;
        sclassid = 1 as uint32_t;
        while sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == nleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    nleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                if new_flag != 0 {
                    return MFS_ERROR_CLASSEXISTS as uint8_t;
                } else {
                    fsclassid = sclassid;
                    break;
                }
            } else {
                if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && fsclassid == 0 as uint32_t
                    && new_flag as ::core::ffi::c_int != 0
                {
                    fsclassid = sclassid;
                }
                sclassid = sclassid.wrapping_add(1);
            }
        }
        if fsclassid == 0 as uint32_t {
            if new_flag != 0 {
                if firstneverused == MAXSCLASS as uint32_t {
                    return MFS_ERROR_CLASSLIMITREACH as uint8_t;
                }
                fsclassid = firstneverused;
                firstneverused = firstneverused.wrapping_add(1);
            } else {
                return MFS_ERROR_NOSUCHCLASS as uint8_t;
            }
        }
        if fsclassid != esclassid as uint32_t {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        if (*create).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            || (*keep).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            || (*create).labelscnt as ::core::ffi::c_int > MAXLABELSCNT
            || (*keep).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*keep).labelscnt as ::core::ffi::c_int > MAXLABELSCNT
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (*arch).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if (ec_current_version as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int != 0 {
                if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    != 4 as ::core::ffi::c_int
                    && (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        != 8 as ::core::ffi::c_int
                {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
            } else {
                (*arch).ec_data_chksum_parts = ((*arch).ec_data_chksum_parts as ::core::ffi::c_int
                    | COMPAT_ECMODE << 4 as ::core::ffi::c_int)
                    as uint8_t;
            }
            if (*arch).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                == 4 as ::core::ffi::c_int
                && (ec_current_version as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
            {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            if (*arch).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*arch).labelscnt as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                || (*arch).ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                    > MAX_EC_LEVEL
            {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        } else if (*arch).labelscnt as ::core::ffi::c_int > MAXLABELSCNT {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if (*trash).ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if (ec_current_version as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            if (*trash).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int != 0 {
                if (*trash).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    != 4 as ::core::ffi::c_int
                    && (*trash).ec_data_chksum_parts as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int
                        != 8 as ::core::ffi::c_int
                {
                    return MFS_ERROR_EINVAL as uint8_t;
                }
            } else {
                (*trash).ec_data_chksum_parts =
                    ((*trash).ec_data_chksum_parts as ::core::ffi::c_int
                        | COMPAT_ECMODE << 4 as ::core::ffi::c_int) as uint8_t;
            }
            if (*trash).ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                == 4 as ::core::ffi::c_int
                && (ec_current_version as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
            {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            if (*trash).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*trash).labelscnt as ::core::ffi::c_int > 2 as ::core::ffi::c_int
                || (*trash).ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                    > MAX_EC_LEVEL
            {
                return MFS_ERROR_EINVAL as uint8_t;
            }
        } else if (*trash).labelscnt as ::core::ffi::c_int > MAXLABELSCNT {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if arch_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || arch_mode as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if new_flag != 0 {
            sclasstab[fsclassid as usize].nleng = nleng;
            memcpy(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fsclassid as isize))
                    .name as *mut uint8_t as *mut ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                nleng as size_t,
            );
        }
        sclasstab[fsclassid as usize].dleng = dleng;
        if dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            memcpy(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fsclassid as isize))
                    .desc as *mut uint8_t as *mut ::core::ffi::c_void,
                desc as *const ::core::ffi::c_void,
                dleng as size_t,
            );
        }
        sclasstab[fsclassid as usize].priority = priority;
        sclasstab[fsclassid as usize].export_group = export_group;
        sclasstab[fsclassid as usize].admin_only = admin_only;
        sclasstab[fsclassid as usize].labels_mode = labels_mode;
        sclasstab[fsclassid as usize].arch_mode = arch_mode;
        sclasstab[fsclassid as usize].create = *create;
        sclasstab[fsclassid as usize].keep = *keep;
        sclasstab[fsclassid as usize].arch = *arch;
        sclasstab[fsclassid as usize].trash = *trash;
        sclasstab[fsclassid as usize].arch_delay = arch_delay;
        sclasstab[fsclassid as usize].min_trashretention = min_trashretention;
        sclasstab[fsclassid as usize].arch_min_size = arch_min_size;
        sclass_fix_has_labels_fields(fsclassid as uint8_t);
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn sclass_univ_duplicate_entry(
    mut oldnleng: uint8_t,
    mut oldname: *const uint8_t,
    mut newnleng: uint8_t,
    mut newname: *const uint8_t,
    mut essclassid: uint16_t,
    mut edsclassid: uint16_t,
) -> uint8_t {
    unsafe {
        let mut sclassid: uint32_t = 0;
        let mut fssclassid: uint32_t = 0;
        let mut fdsclassid: uint32_t = 0;
        if sclass_name_check(oldnleng, oldname) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || sclass_name_check(newnleng, newname) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || essclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && edsclassid as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            || edsclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && essclassid as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        fssclassid = 0 as uint32_t;
        sclassid = 1 as uint32_t;
        while fssclassid == 0 as uint32_t && sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == oldnleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    oldname as *const ::core::ffi::c_void,
                    oldnleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                fssclassid = sclassid;
            }
            sclassid = sclassid.wrapping_add(1);
        }
        if fssclassid == 0 as uint32_t {
            return MFS_ERROR_NOSUCHCLASS as uint8_t;
        }
        if essclassid as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && fssclassid != essclassid as uint32_t
        {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        fdsclassid = 0 as uint32_t;
        sclassid = 1 as uint32_t;
        while sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == newnleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    newname as *const ::core::ffi::c_void,
                    newnleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_CLASSEXISTS as uint8_t;
            }
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && fdsclassid == 0 as uint32_t
            {
                fdsclassid = sclassid;
            }
            sclassid = sclassid.wrapping_add(1);
        }
        if fdsclassid == 0 as uint32_t {
            if firstneverused == MAXSCLASS as uint32_t {
                return MFS_ERROR_CLASSLIMITREACH as uint8_t;
            }
            fdsclassid = firstneverused;
            firstneverused = firstneverused.wrapping_add(1);
        }
        if edsclassid as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && fdsclassid != edsclassid as uint32_t
        {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        sclasstab[fdsclassid as usize].nleng = newnleng;
        memcpy(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fdsclassid as isize)).name
                as *mut uint8_t as *mut ::core::ffi::c_void,
            newname as *const ::core::ffi::c_void,
            newnleng as size_t,
        );
        sclasstab[fdsclassid as usize].dleng = sclasstab[fssclassid as usize].dleng;
        if sclasstab[fssclassid as usize].dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            memcpy(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fdsclassid as isize))
                    .desc as *mut uint8_t as *mut ::core::ffi::c_void,
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fssclassid as isize))
                    .desc as *mut uint8_t as *const ::core::ffi::c_void,
                sclasstab[fssclassid as usize].dleng as size_t,
            );
        }
        sclasstab[fdsclassid as usize].priority = sclasstab[fssclassid as usize].priority;
        sclasstab[fdsclassid as usize].export_group = sclasstab[fssclassid as usize].export_group;
        sclasstab[fdsclassid as usize].admin_only = sclasstab[fssclassid as usize].admin_only;
        sclasstab[fdsclassid as usize].labels_mode = sclasstab[fssclassid as usize].labels_mode;
        sclasstab[fdsclassid as usize].arch_mode = sclasstab[fssclassid as usize].arch_mode;
        sclasstab[fdsclassid as usize].create = sclasstab[fssclassid as usize].create;
        sclasstab[fdsclassid as usize].keep = sclasstab[fssclassid as usize].keep;
        sclasstab[fdsclassid as usize].arch = sclasstab[fssclassid as usize].arch;
        sclasstab[fdsclassid as usize].trash = sclasstab[fssclassid as usize].trash;
        sclasstab[fdsclassid as usize].arch_delay = sclasstab[fssclassid as usize].arch_delay;
        sclasstab[fdsclassid as usize].min_trashretention =
            sclasstab[fssclassid as usize].min_trashretention;
        sclasstab[fdsclassid as usize].arch_min_size = sclasstab[fssclassid as usize].arch_min_size;
        if essclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && edsclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            changelog(
                b"%u|SCDUP(%s,%s):%u,%u\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                changelog_escape_name(oldnleng as uint32_t, oldname),
                changelog_escape_name(newnleng as uint32_t, newname),
                fssclassid,
                fdsclassid,
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_mr_duplicate_entry(
    mut oldnleng: uint8_t,
    mut oldname: *const uint8_t,
    mut newnleng: uint8_t,
    mut newname: *const uint8_t,
    mut essclassid: uint16_t,
    mut edsclassid: uint16_t,
) -> uint8_t {
    unsafe {
        return sclass_univ_duplicate_entry(
            oldnleng, oldname, newnleng, newname, essclassid, edsclassid,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_duplicate_entry(
    mut oldnleng: uint8_t,
    mut oldname: *const uint8_t,
    mut newnleng: uint8_t,
    mut newname: *const uint8_t,
) -> uint8_t {
    unsafe {
        return sclass_univ_duplicate_entry(
            oldnleng,
            oldname,
            newnleng,
            newname,
            0 as uint16_t,
            0 as uint16_t,
        );
    }
}
#[inline]
unsafe extern "C" fn sclass_univ_rename_entry(
    mut oldnleng: uint8_t,
    mut oldname: *const uint8_t,
    mut newnleng: uint8_t,
    mut newname: *const uint8_t,
    mut esclassid: uint16_t,
) -> uint8_t {
    unsafe {
        let mut sclassid: uint32_t = 0;
        let mut fsclassid: uint32_t = 0;
        if sclass_name_check(oldnleng, oldname) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || sclass_name_check(newnleng, newname) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        fsclassid = 0 as uint32_t;
        sclassid = 1 as uint32_t;
        while fsclassid == 0 as uint32_t && sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == oldnleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    oldname as *const ::core::ffi::c_void,
                    oldnleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                fsclassid = sclassid;
            }
            sclassid = sclassid.wrapping_add(1);
        }
        if fsclassid == 0 as uint32_t {
            return MFS_ERROR_NOSUCHCLASS as uint8_t;
        }
        if esclassid as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && fsclassid != esclassid as uint32_t
        {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        sclassid = 1 as uint32_t;
        while sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == newnleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    newname as *const ::core::ffi::c_void,
                    newnleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_CLASSEXISTS as uint8_t;
            }
            sclassid = sclassid.wrapping_add(1);
        }
        sclasstab[fsclassid as usize].nleng = newnleng;
        memcpy(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(fsclassid as isize)).name
                as *mut uint8_t as *mut ::core::ffi::c_void,
            newname as *const ::core::ffi::c_void,
            newnleng as size_t,
        );
        if esclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|SCREN(%s,%s):%u\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                changelog_escape_name(oldnleng as uint32_t, oldname),
                changelog_escape_name(newnleng as uint32_t, newname),
                fsclassid,
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_mr_rename_entry(
    mut oldnleng: uint8_t,
    mut oldname: *const uint8_t,
    mut newnleng: uint8_t,
    mut newname: *const uint8_t,
    mut esclassid: uint16_t,
) -> uint8_t {
    unsafe {
        return sclass_univ_rename_entry(oldnleng, oldname, newnleng, newname, esclassid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_rename_entry(
    mut oldnleng: uint8_t,
    mut oldname: *const uint8_t,
    mut newnleng: uint8_t,
    mut newname: *const uint8_t,
) -> uint8_t {
    unsafe {
        return sclass_univ_rename_entry(oldnleng, oldname, newnleng, newname, 0 as uint16_t);
    }
}
#[inline]
unsafe extern "C" fn sclass_univ_delete_entry(
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut esclassid: uint16_t,
) -> uint8_t {
    unsafe {
        let mut sclassid: uint32_t = 0;
        let mut fsclassid: uint32_t = 0;
        if sclass_name_check(nleng, name) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        fsclassid = 0 as uint32_t;
        sclassid = 1 as uint32_t;
        while fsclassid == 0 as uint32_t && sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == nleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    nleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                fsclassid = sclassid;
            }
            sclassid = sclassid.wrapping_add(1);
        }
        if fsclassid == 0 as uint32_t {
            return MFS_ERROR_NOSUCHCLASS as uint8_t;
        }
        if sclasstab[fsclassid as usize].files > 0 as uint32_t
            || sclasstab[fsclassid as usize].directories > 0 as uint32_t
        {
            return MFS_ERROR_CLASSINUSE as uint8_t;
        }
        if esclassid as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && fsclassid != esclassid as uint32_t
        {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        patterns_sclass_delete(fsclassid as uint8_t);
        sclasstab[fsclassid as usize].nleng = 0 as uint8_t;
        if esclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            changelog(
                b"%u|SCDEL(%s):%u\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                changelog_escape_name(nleng as uint32_t, name),
                fsclassid,
            );
        } else {
            meta_version_inc();
        }
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_mr_delete_entry(
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut esclassid: uint16_t,
) -> uint8_t {
    unsafe {
        return sclass_univ_delete_entry(nleng, name, esclassid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_delete_entry(
    mut nleng: uint8_t,
    mut name: *const uint8_t,
) -> uint8_t {
    unsafe {
        return sclass_univ_delete_entry(nleng, name, 0 as uint16_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_list_entries(
    mut buff: *mut uint8_t,
    mut fver: uint8_t,
) -> uint32_t {
    unsafe {
        let mut sclassid: uint32_t = 0;
        let mut ret: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut og: uint32_t = 0;
        let mut err: uint8_t = 0;
        ret = 0 as uint32_t;
        sclassid = 1 as uint32_t;
        while sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if buff.is_null() {
                    if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                        ret = ret.wrapping_add(1);
                    }
                    ret = ret.wrapping_add(
                        (sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as uint32_t,
                    );
                    if fver as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                        ret = ret.wrapping_add(
                            ((sclasstab[sclassid as usize].create.labelscnt as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].keep.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].arch.labelscnt
                                    as ::core::ffi::c_int)
                                * 4 as ::core::ffi::c_int
                                * MASKORGROUP
                                + 7 as ::core::ffi::c_int) as uint32_t,
                        );
                    } else if fver as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                        ret = ret.wrapping_add(
                            ((sclasstab[sclassid as usize].create.labelscnt as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].keep.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].arch.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].trash.labelscnt
                                    as ::core::ffi::c_int)
                                * SCLASS_EXPR_MAX_SIZE
                                + 28 as ::core::ffi::c_int) as uint32_t,
                        );
                    } else if fver as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                        ret = ret.wrapping_add(
                            ((sclasstab[sclassid as usize].create.labelscnt as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].keep.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].arch.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].trash.labelscnt
                                    as ::core::ffi::c_int)
                                * SCLASS_EXPR_MAX_SIZE
                                + 29 as ::core::ffi::c_int) as uint32_t,
                        );
                    } else if fver as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
                        ret = ret.wrapping_add(
                            ((sclasstab[sclassid as usize].create.labelscnt as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].keep.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].arch.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].trash.labelscnt
                                    as ::core::ffi::c_int)
                                * SCLASS_EXPR_MAX_SIZE
                                + 37 as ::core::ffi::c_int) as uint32_t,
                        );
                    } else if fver as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
                        ret = ret.wrapping_add(
                            ((sclasstab[sclassid as usize].create.labelscnt as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].keep.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].arch.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].trash.labelscnt
                                    as ::core::ffi::c_int)
                                * SCLASS_EXPR_MAX_SIZE
                                + 41 as ::core::ffi::c_int) as uint32_t,
                        );
                    } else if fver as ::core::ffi::c_int == 6 as ::core::ffi::c_int {
                        ret = ret.wrapping_add(
                            ((sclasstab[sclassid as usize].create.labelscnt as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].keep.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].arch.labelscnt
                                    as ::core::ffi::c_int
                                + sclasstab[sclassid as usize].trash.labelscnt
                                    as ::core::ffi::c_int)
                                * SCLASS_EXPR_MAX_SIZE
                                + 47 as ::core::ffi::c_int) as uint32_t,
                        );
                        ret = ret.wrapping_add(sclasstab[sclassid as usize].dleng as uint32_t);
                    }
                } else {
                    if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                        put8bit(&raw mut buff, sclassid as uint8_t);
                    }
                    put8bit(&raw mut buff, sclasstab[sclassid as usize].nleng);
                    memcpy(
                        buff as *mut ::core::ffi::c_void,
                        &raw mut (*(&raw mut sclasstab as *mut storageclass)
                            .offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                        sclasstab[sclassid as usize].nleng as size_t,
                    );
                    buff = buff
                        .offset(sclasstab[sclassid as usize].nleng as ::core::ffi::c_int as isize);
                    if fver as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                        let mut create_labelmasks: [uint32_t; 36] = [0; 36];
                        let mut keep_labelmasks: [uint32_t; 36] = [0; 36];
                        let mut arch_labelmasks: [uint32_t; 36] = [0; 36];
                        memset(
                            &raw mut create_labelmasks as *mut uint32_t as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<uint32_t>()
                                .wrapping_mul(MAXLABELSCNT as size_t)
                                .wrapping_mul(MASKORGROUP as size_t),
                        );
                        memset(
                            &raw mut keep_labelmasks as *mut uint32_t as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<uint32_t>()
                                .wrapping_mul(MAXLABELSCNT as size_t)
                                .wrapping_mul(MASKORGROUP as size_t),
                        );
                        memset(
                            &raw mut arch_labelmasks as *mut uint32_t as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<uint32_t>()
                                .wrapping_mul(MAXLABELSCNT as size_t)
                                .wrapping_mul(MASKORGROUP as size_t),
                        );
                        err = 0 as uint8_t;
                        if sclasstab[sclassid as usize].arch_delay as ::core::ffi::c_int
                            % 24 as ::core::ffi::c_int
                            != 0
                        {
                            err = 1 as uint8_t;
                        }
                        if err as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && sclass_labelexpr_to_maskorgroup(
                                &raw mut create_labelmasks as *mut uint32_t,
                                &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(sclassid as isize))
                                .create
                                .labelexpr as *mut [uint8_t; 128],
                                sclasstab[sclassid as usize].create.labelscnt,
                            ) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            err = 1 as uint8_t;
                            memset(
                                &raw mut create_labelmasks as *mut uint32_t
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<uint32_t>()
                                    .wrapping_mul(MAXLABELSCNT as size_t)
                                    .wrapping_mul(MASKORGROUP as size_t),
                            );
                        }
                        if err as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && sclass_labelexpr_to_maskorgroup(
                                &raw mut keep_labelmasks as *mut uint32_t,
                                &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(sclassid as isize))
                                .keep
                                .labelexpr as *mut [uint8_t; 128],
                                sclasstab[sclassid as usize].keep.labelscnt,
                            ) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            err = 1 as uint8_t;
                            memset(
                                &raw mut keep_labelmasks as *mut uint32_t
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<uint32_t>()
                                    .wrapping_mul(MAXLABELSCNT as size_t)
                                    .wrapping_mul(MASKORGROUP as size_t),
                            );
                        }
                        if err as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && sclass_labelexpr_to_maskorgroup(
                                &raw mut arch_labelmasks as *mut uint32_t,
                                &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(sclassid as isize))
                                .arch
                                .labelexpr as *mut [uint8_t; 128],
                                sclasstab[sclassid as usize].arch.labelscnt,
                            ) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            err = 1 as uint8_t;
                            memset(
                                &raw mut arch_labelmasks as *mut uint32_t
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<uint32_t>()
                                    .wrapping_mul(MAXLABELSCNT as size_t)
                                    .wrapping_mul(MASKORGROUP as size_t),
                            );
                        }
                        if sclasstab[sclassid as usize].trash.labelscnt as ::core::ffi::c_int
                            > 0 as ::core::ffi::c_int
                            || sclasstab[sclassid as usize].trash.ec_data_chksum_parts
                                as ::core::ffi::c_int
                                != 0
                            || sclasstab[sclassid as usize].arch.ec_data_chksum_parts
                                as ::core::ffi::c_int
                                != 0
                        {
                            err = 1 as uint8_t;
                        }
                        if sclasstab[sclassid as usize].create.uniqmask
                            | sclasstab[sclassid as usize].keep.uniqmask
                            | sclasstab[sclassid as usize].arch.uniqmask
                            | sclasstab[sclassid as usize].trash.uniqmask
                            > 0 as uint32_t
                        {
                            err = 1 as uint8_t;
                        }
                        if err != 0 {
                            memset(
                                buff.offset(
                                    -(sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                                        as isize),
                                ) as *mut ::core::ffi::c_void,
                                '*' as ::core::ffi::c_int,
                                sclasstab[sclassid as usize].nleng as size_t,
                            );
                        }
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].admin_only);
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].labels_mode);
                        put16bit(
                            &raw mut buff,
                            (sclasstab[sclassid as usize].arch_delay as ::core::ffi::c_int
                                / 24 as ::core::ffi::c_int) as uint16_t,
                        );
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].create.labelscnt);
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].keep.labelscnt);
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].arch.labelscnt);
                        i = 0 as uint32_t;
                        while i < sclasstab[sclassid as usize].create.labelscnt as uint32_t {
                            og = 0 as uint32_t;
                            while og < MASKORGROUP as uint32_t {
                                put32bit(
                                    &raw mut buff,
                                    create_labelmasks[i
                                        .wrapping_mul(MASKORGROUP as uint32_t)
                                        .wrapping_add(og)
                                        as usize],
                                );
                                og = og.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                        i = 0 as uint32_t;
                        while i < sclasstab[sclassid as usize].keep.labelscnt as uint32_t {
                            og = 0 as uint32_t;
                            while og < MASKORGROUP as uint32_t {
                                put32bit(
                                    &raw mut buff,
                                    keep_labelmasks[i
                                        .wrapping_mul(MASKORGROUP as uint32_t)
                                        .wrapping_add(og)
                                        as usize],
                                );
                                og = og.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                        i = 0 as uint32_t;
                        while i < sclasstab[sclassid as usize].arch.labelscnt as uint32_t {
                            og = 0 as uint32_t;
                            while og < MASKORGROUP as uint32_t {
                                put32bit(
                                    &raw mut buff,
                                    arch_labelmasks[i
                                        .wrapping_mul(MASKORGROUP as uint32_t)
                                        .wrapping_add(og)
                                        as usize],
                                );
                                og = og.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                    } else if fver as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                        || fver as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                        || fver as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                        || fver as ::core::ffi::c_int == 5 as ::core::ffi::c_int
                        || fver as ::core::ffi::c_int == 6 as ::core::ffi::c_int
                    {
                        if fver as ::core::ffi::c_int >= 6 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, sclasstab[sclassid as usize].dleng);
                            if sclasstab[sclassid as usize].dleng as ::core::ffi::c_int
                                > 0 as ::core::ffi::c_int
                            {
                                memcpy(
                                    buff as *mut ::core::ffi::c_void,
                                    &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                        .offset(sclassid as isize))
                                    .desc as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    sclasstab[sclassid as usize].dleng as size_t,
                                );
                                buff = buff.offset(
                                    sclasstab[sclassid as usize].dleng as ::core::ffi::c_int
                                        as isize,
                                );
                            }
                            put32bit(&raw mut buff, sclasstab[sclassid as usize].priority);
                            put8bit(&raw mut buff, sclasstab[sclassid as usize].export_group);
                        }
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].admin_only);
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].labels_mode);
                        if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, sclasstab[sclassid as usize].arch_mode);
                        }
                        put16bit(&raw mut buff, sclasstab[sclassid as usize].arch_delay);
                        if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            put64bit(&raw mut buff, sclasstab[sclassid as usize].arch_min_size);
                        }
                        put16bit(
                            &raw mut buff,
                            sclasstab[sclassid as usize].min_trashretention,
                        );
                        if sclasstab[sclassid as usize].arch.ec_data_chksum_parts
                            as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int
                            > MaxECRedundancyLevel as ::core::ffi::c_int
                        {
                            put8bit(
                                &raw mut buff,
                                (sclasstab[sclassid as usize].arch.ec_data_chksum_parts
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                                    as uint8_t,
                            );
                        } else {
                            put8bit(
                                &raw mut buff,
                                sclasstab[sclassid as usize].arch.ec_data_chksum_parts,
                            );
                        }
                        if sclasstab[sclassid as usize].trash.ec_data_chksum_parts
                            as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int
                            > MaxECRedundancyLevel as ::core::ffi::c_int
                        {
                            put8bit(
                                &raw mut buff,
                                (sclasstab[sclassid as usize].trash.ec_data_chksum_parts
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                                    as uint8_t,
                            );
                        } else {
                            put8bit(
                                &raw mut buff,
                                sclasstab[sclassid as usize].trash.ec_data_chksum_parts,
                            );
                        }
                        if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                            put8bit(
                                &raw mut buff,
                                sclasstab[sclassid as usize].create.labels_mode,
                            );
                            put8bit(&raw mut buff, sclasstab[sclassid as usize].keep.labels_mode);
                            put8bit(&raw mut buff, sclasstab[sclassid as usize].arch.labels_mode);
                            put8bit(
                                &raw mut buff,
                                sclasstab[sclassid as usize].trash.labels_mode,
                            );
                        }
                        put32bit(&raw mut buff, sclasstab[sclassid as usize].create.uniqmask);
                        put32bit(&raw mut buff, sclasstab[sclassid as usize].keep.uniqmask);
                        put32bit(&raw mut buff, sclasstab[sclassid as usize].arch.uniqmask);
                        put32bit(&raw mut buff, sclasstab[sclassid as usize].trash.uniqmask);
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].create.labelscnt);
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].keep.labelscnt);
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].arch.labelscnt);
                        put8bit(&raw mut buff, sclasstab[sclassid as usize].trash.labelscnt);
                        i = 0 as uint32_t;
                        while i < sclasstab[sclassid as usize].create.labelscnt as uint32_t {
                            memcpy(
                                buff as *mut ::core::ffi::c_void,
                                &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(sclassid as isize))
                                .create
                                .labelexpr
                                    as *mut [uint8_t; 128])
                                    .offset(i as isize)
                                    as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                SCLASS_EXPR_MAX_SIZE as size_t,
                            );
                            buff = buff.offset(SCLASS_EXPR_MAX_SIZE as isize);
                            i = i.wrapping_add(1);
                        }
                        i = 0 as uint32_t;
                        while i < sclasstab[sclassid as usize].keep.labelscnt as uint32_t {
                            memcpy(
                                buff as *mut ::core::ffi::c_void,
                                &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(sclassid as isize))
                                .keep
                                .labelexpr
                                    as *mut [uint8_t; 128])
                                    .offset(i as isize)
                                    as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                SCLASS_EXPR_MAX_SIZE as size_t,
                            );
                            buff = buff.offset(SCLASS_EXPR_MAX_SIZE as isize);
                            i = i.wrapping_add(1);
                        }
                        i = 0 as uint32_t;
                        while i < sclasstab[sclassid as usize].arch.labelscnt as uint32_t {
                            memcpy(
                                buff as *mut ::core::ffi::c_void,
                                &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(sclassid as isize))
                                .arch
                                .labelexpr
                                    as *mut [uint8_t; 128])
                                    .offset(i as isize)
                                    as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                SCLASS_EXPR_MAX_SIZE as size_t,
                            );
                            buff = buff.offset(SCLASS_EXPR_MAX_SIZE as isize);
                            i = i.wrapping_add(1);
                        }
                        i = 0 as uint32_t;
                        while i < sclasstab[sclassid as usize].trash.labelscnt as uint32_t {
                            memcpy(
                                buff as *mut ::core::ffi::c_void,
                                &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(sclassid as isize))
                                .trash
                                .labelexpr
                                    as *mut [uint8_t; 128])
                                    .offset(i as isize)
                                    as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                SCLASS_EXPR_MAX_SIZE as size_t,
                            );
                            buff = buff.offset(SCLASS_EXPR_MAX_SIZE as isize);
                            i = i.wrapping_add(1);
                        }
                    }
                }
            }
            sclassid = sclassid.wrapping_add(1);
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_find_by_name(
    mut nleng: uint8_t,
    mut name: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut sclassid: uint32_t = 0;
        sclassid = 1 as uint32_t;
        while sclassid < firstneverused {
            if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                == nleng as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                        .name as *mut uint8_t as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    nleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return sclassid as uint8_t;
            }
            sclassid = sclassid.wrapping_add(1);
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_nleng(mut sclassid: uint8_t) -> uint8_t {
    unsafe {
        return sclasstab[sclassid as usize].nleng;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_name(mut sclassid: uint8_t) -> *const uint8_t {
    unsafe {
        return &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize)).name
            as *mut uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_incref(mut sclassid: uint16_t, mut r#type: uint8_t) {
    unsafe {
        if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
            sclasstab[sclassid as usize].directories =
                sclasstab[sclassid as usize].directories.wrapping_add(1);
        } else if r#type as ::core::ffi::c_int == TYPE_FILE
            || r#type as ::core::ffi::c_int == TYPE_TRASH
            || r#type as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            sclasstab[sclassid as usize].files = sclasstab[sclassid as usize].files.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_decref(mut sclassid: uint16_t, mut r#type: uint8_t) {
    unsafe {
        if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
            sclasstab[sclassid as usize].directories =
                sclasstab[sclassid as usize].directories.wrapping_sub(1);
        } else if r#type as ::core::ffi::c_int == TYPE_FILE
            || r#type as ::core::ffi::c_int == TYPE_TRASH
            || r#type as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            sclasstab[sclassid as usize].files = sclasstab[sclassid as usize].files.wrapping_sub(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_labels_mode(
    mut sclassid: uint16_t,
    mut sm: *mut storagemode,
) -> uint8_t {
    unsafe {
        if (*sm).labels_mode as ::core::ffi::c_int == LABELS_MODE_LOOSE
            || (*sm).labels_mode as ::core::ffi::c_int == LABELS_MODE_STD
            || (*sm).labels_mode as ::core::ffi::c_int == LABELS_MODE_STRICT
        {
            return (*sm).labels_mode;
        }
        return sclasstab[sclassid as usize].labels_mode;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_arch_mode(mut sclassid: uint16_t) -> uint8_t {
    unsafe {
        return sclasstab[sclassid as usize].arch_mode;
    }
}
#[inline]
unsafe extern "C" fn sclass_get_keeparch_max_goal_equivalent(mut sclassid: uint16_t) -> uint8_t {
    unsafe {
        let mut ec_data_chksum_parts: uint8_t = 0;
        ec_data_chksum_parts = (sclasstab[sclassid as usize].arch.ec_data_chksum_parts
            as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int) as uint8_t;
        if ec_data_chksum_parts as ::core::ffi::c_int > MaxECRedundancyLevel as ::core::ffi::c_int {
            ec_data_chksum_parts = MaxECRedundancyLevel;
        }
        if ec_data_chksum_parts != 0 {
            if ec_data_chksum_parts as ::core::ffi::c_int
                >= sclasstab[sclassid as usize].keep.labelscnt as ::core::ffi::c_int
            {
                return (ec_data_chksum_parts as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as uint8_t;
            } else {
                return sclasstab[sclassid as usize].keep.labelscnt;
            }
        } else if sclasstab[sclassid as usize].arch.labelscnt as ::core::ffi::c_int
            > sclasstab[sclassid as usize].keep.labelscnt as ::core::ffi::c_int
        {
            return sclasstab[sclassid as usize].arch.labelscnt;
        } else {
            return sclasstab[sclassid as usize].keep.labelscnt;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_keeparch_maxstorage_eights(mut sclassid: uint16_t) -> uint8_t {
    unsafe {
        let mut res: uint8_t = 0;
        let mut ec_data_parts: uint8_t = 0;
        let mut ec_chksum_parts: uint8_t = 0;
        if sclasstab[sclassid as usize].arch.ec_data_chksum_parts != 0 {
            ec_data_parts = (sclasstab[sclassid as usize].arch.ec_data_chksum_parts
                as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as uint8_t;
            ec_chksum_parts = (sclasstab[sclassid as usize].arch.ec_data_chksum_parts
                as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t;
            if ec_chksum_parts as ::core::ffi::c_int > MaxECRedundancyLevel as ::core::ffi::c_int {
                res = (MaxECRedundancyLevel as ::core::ffi::c_int
                    + ec_data_parts as ::core::ffi::c_int) as uint8_t;
            } else {
                res = (ec_data_parts as ::core::ffi::c_int + ec_chksum_parts as ::core::ffi::c_int)
                    as uint8_t;
            }
            res = (res as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t;
            res = (res as ::core::ffi::c_int / ec_data_parts as ::core::ffi::c_int) as uint8_t;
        } else {
            res = (sclasstab[sclassid as usize].arch.labelscnt as ::core::ffi::c_int
                * 8 as ::core::ffi::c_int) as uint8_t;
        }
        if (res as ::core::ffi::c_int)
            < sclasstab[sclassid as usize].keep.labelscnt as ::core::ffi::c_int
                * 8 as ::core::ffi::c_int
        {
            res = (sclasstab[sclassid as usize].keep.labelscnt as ::core::ffi::c_int
                * 8 as ::core::ffi::c_int) as uint8_t;
        }
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_keeparch_storage_eights(
    mut sclassid: uint16_t,
    mut keepflag: uint8_t,
) -> uint8_t {
    unsafe {
        let mut ec_data_parts: uint8_t = 0;
        let mut ec_chksum_parts: uint8_t = 0;
        if keepflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (sclasstab[sclassid as usize].arch.labelscnt as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
                || sclasstab[sclassid as usize].arch.ec_data_chksum_parts as ::core::ffi::c_int
                    != 0)
        {
            if sclasstab[sclassid as usize].arch.ec_data_chksum_parts != 0 {
                ec_data_parts = (sclasstab[sclassid as usize].arch.ec_data_chksum_parts
                    as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int) as uint8_t;
                ec_chksum_parts = (sclasstab[sclassid as usize].arch.ec_data_chksum_parts
                    as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int) as uint8_t;
                if ec_chksum_parts as ::core::ffi::c_int
                    > MaxECRedundancyLevel as ::core::ffi::c_int
                {
                    return ((MaxECRedundancyLevel as ::core::ffi::c_int
                        + ec_data_parts as ::core::ffi::c_int)
                        * 8 as ::core::ffi::c_int
                        / ec_data_parts as ::core::ffi::c_int)
                        as uint8_t;
                } else {
                    return ((ec_chksum_parts as ::core::ffi::c_int
                        + ec_data_parts as ::core::ffi::c_int)
                        * 8 as ::core::ffi::c_int
                        / ec_data_parts as ::core::ffi::c_int)
                        as uint8_t;
                }
            } else {
                return (sclasstab[sclassid as usize].arch.labelscnt as ::core::ffi::c_int
                    * 8 as ::core::ffi::c_int) as uint8_t;
            }
        }
        return (sclasstab[sclassid as usize].keep.labelscnt as ::core::ffi::c_int
            * 8 as ::core::ffi::c_int) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_create_storagemode(mut sclassid: uint16_t) -> *mut storagemode {
    unsafe {
        if sclasstab[sclassid as usize].create.labelscnt as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            return &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                .create;
        } else {
            return &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
                .keep;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_keeparch_storagemode(
    mut sclassid: uint16_t,
    mut flags: uint8_t,
) -> *mut storagemode {
    unsafe {
        if flags as ::core::ffi::c_int & 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (sclasstab[sclassid as usize].trash.labelscnt as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
                || sclasstab[sclassid as usize].trash.ec_data_chksum_parts as ::core::ffi::c_int
                    != 0)
        {
            if sclasstab[sclassid as usize].trash.ec_data_chksum_parts as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                > MaxECRedundancyLevel as ::core::ffi::c_int
            {
                tmp_storagemode = sclasstab[sclassid as usize].trash;
                tmp_storagemode.ec_data_chksum_parts = (tmp_storagemode.ec_data_chksum_parts
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                    as uint8_t;
                return &raw mut tmp_storagemode;
            } else {
                return &raw mut (*(&raw mut sclasstab as *mut storageclass)
                    .offset(sclassid as isize))
                .trash;
            }
        } else if flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (sclasstab[sclassid as usize].arch.labelscnt as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
                || sclasstab[sclassid as usize].arch.ec_data_chksum_parts as ::core::ffi::c_int
                    != 0)
        {
            if sclasstab[sclassid as usize].arch.ec_data_chksum_parts as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int
                > MaxECRedundancyLevel as ::core::ffi::c_int
            {
                tmp_storagemode = sclasstab[sclassid as usize].arch;
                tmp_storagemode.ec_data_chksum_parts = (tmp_storagemode.ec_data_chksum_parts
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                    as uint8_t;
                return &raw mut tmp_storagemode;
            } else {
                return &raw mut (*(&raw mut sclasstab as *mut storageclass)
                    .offset(sclassid as isize))
                .arch;
            }
        }
        return &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(sclassid as isize))
            .keep;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_joining_priority(mut sclassid: uint16_t) -> uint64_t {
    unsafe {
        let mut ret: uint64_t = 0;
        ret = sclasstab[sclassid as usize].priority as uint64_t;
        ret <<= 4 as ::core::ffi::c_int;
        ret = ret.wrapping_add(sclass_get_keeparch_max_goal_equivalent(sclassid) as uint64_t);
        ret <<= 1 as ::core::ffi::c_int;
        if sclasstab[sclassid as usize].arch.ec_data_chksum_parts as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int
            != 0
        {
            ret = ret.wrapping_add(1 as uint64_t);
        }
        ret <<= 1 as ::core::ffi::c_int;
        if sclasstab[sclassid as usize].keep.has_labels as ::core::ffi::c_int
            | sclasstab[sclassid as usize].arch.has_labels as ::core::ffi::c_int
            != 0
        {
            ret = ret.wrapping_add(1 as uint64_t);
        }
        ret <<= 1 as ::core::ffi::c_int;
        if sclasstab[sclassid as usize].trash.ec_data_chksum_parts as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int
            != 0
        {
            ret = ret.wrapping_add(1 as uint64_t);
        }
        ret <<= 1 as ::core::ffi::c_int;
        if sclasstab[sclassid as usize].trash.has_labels != 0 {
            ret = ret.wrapping_add(1 as uint64_t);
        }
        ret <<= 8 as ::core::ffi::c_int;
        ret = ret.wrapping_add(sclassid as uint64_t);
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_export_group(mut sclassid: uint16_t) -> uint8_t {
    unsafe {
        return sclasstab[sclassid as usize].export_group;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_is_admin_only(mut sclassid: uint16_t) -> uint8_t {
    unsafe {
        return sclasstab[sclassid as usize].admin_only;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_min_trashretention(mut sclassid: uint16_t) -> uint16_t {
    unsafe {
        return sclasstab[sclassid as usize].min_trashretention;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_arch_delay(mut sclassid: uint16_t) -> uint16_t {
    unsafe {
        return sclasstab[sclassid as usize].arch_delay;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_get_arch_min_size(mut sclassid: uint16_t) -> uint64_t {
    unsafe {
        return sclasstab[sclassid as usize].arch_min_size;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_calc_goal_equivalent(mut sm: *mut storagemode) -> uint8_t {
    unsafe {
        if (*sm).ec_data_chksum_parts != 0 {
            if (*sm).ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                > MaxECRedundancyLevel as ::core::ffi::c_int
            {
                return (MaxECRedundancyLevel as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as uint8_t;
            } else {
                return (((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int) as uint8_t;
            }
        } else {
            return (*sm).labelscnt;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_info(mut buff: *mut uint8_t, mut fver: uint8_t) -> uint32_t {
    unsafe {
        let mut leng: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut keepcnt: [uint64_t; 6] = [0; 6];
        let mut archcnt: [uint64_t; 6] = [0; 6];
        let mut trashcnt: [uint64_t; 6] = [0; 6];
        let mut krlevel: uint8_t = 0;
        let mut arlevel: uint8_t = 0;
        let mut trlevel: uint8_t = 0;
        if buff.is_null() {
            if fver as ::core::ffi::c_int == 128 as ::core::ffi::c_int {
                leng = (2 as ::core::ffi::c_int + 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as uint32_t;
            } else {
                leng = 2 as uint32_t;
            }
            i = 1 as uint32_t;
            while i < firstneverused {
                if sclasstab[i as usize].nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    if fver as ::core::ffi::c_int == 128 as ::core::ffi::c_int {
                        leng = leng.wrapping_add(
                            (2 as ::core::ffi::c_int
                                + sclasstab[i as usize].nleng as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as uint32_t,
                        );
                    } else {
                        if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                            leng = leng.wrapping_add(1);
                        }
                        if fver as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
                            leng = leng.wrapping_add(
                                (3 as ::core::ffi::c_int * 24 as ::core::ffi::c_int) as uint32_t,
                            );
                        }
                        if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                            leng = leng.wrapping_add(8 as uint32_t);
                        }
                        if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            leng = leng.wrapping_add(4 as uint32_t);
                        }
                        if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                            leng = leng.wrapping_add(6 as uint32_t);
                        }
                        leng = leng.wrapping_add(
                            (42 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 24 as ::core::ffi::c_int)
                                as uint32_t,
                        );
                        leng = leng.wrapping_add(sclasstab[i as usize].nleng as uint32_t);
                        leng = leng.wrapping_add(sclasstab[i as usize].dleng as uint32_t);
                        leng = leng.wrapping_add(
                            (sclasstab[i as usize].create.labelscnt as uint32_t).wrapping_mul(
                                (SCLASS_EXPR_MAX_SIZE + 2 as ::core::ffi::c_int) as uint32_t,
                            ),
                        );
                        leng = leng.wrapping_add(
                            (sclasstab[i as usize].keep.labelscnt as uint32_t).wrapping_mul(
                                (SCLASS_EXPR_MAX_SIZE + 2 as ::core::ffi::c_int) as uint32_t,
                            ),
                        );
                        leng = leng.wrapping_add(
                            (sclasstab[i as usize].arch.labelscnt as uint32_t).wrapping_mul(
                                (SCLASS_EXPR_MAX_SIZE + 2 as ::core::ffi::c_int) as uint32_t,
                            ),
                        );
                        leng = leng.wrapping_add(
                            (sclasstab[i as usize].trash.labelscnt as uint32_t).wrapping_mul(
                                (SCLASS_EXPR_MAX_SIZE + 2 as ::core::ffi::c_int) as uint32_t,
                            ),
                        );
                    }
                }
                i = i.wrapping_add(1);
            }
            return leng;
        } else {
            if fver as ::core::ffi::c_int == 128 as ::core::ffi::c_int {
                put8bit(&raw mut buff, 0 as uint8_t);
                put8bit(&raw mut buff, 9 as uint8_t);
                memcpy(
                    buff as *mut ::core::ffi::c_void,
                    b"(deleted)\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    9 as size_t,
                );
                buff = buff.offset(9 as ::core::ffi::c_int as isize);
                put8bit(&raw mut buff, chunk_sclass_has_chunks(0 as uint8_t));
                i = 1 as uint32_t;
                while i < firstneverused {
                    if sclasstab[i as usize].nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        put8bit(&raw mut buff, i as uint8_t);
                        put8bit(&raw mut buff, sclasstab[i as usize].nleng);
                        memcpy(
                            buff as *mut ::core::ffi::c_void,
                            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize))
                                .name as *mut uint8_t
                                as *const ::core::ffi::c_void,
                            sclasstab[i as usize].nleng as size_t,
                        );
                        buff =
                            buff.offset(sclasstab[i as usize].nleng as ::core::ffi::c_int as isize);
                        put8bit(&raw mut buff, chunk_sclass_has_chunks(i as uint8_t));
                    }
                    i = i.wrapping_add(1);
                }
            } else {
                chunk_labelset_can_be_fulfilled(::core::ptr::null_mut::<storagemode>());
                put16bit(&raw mut buff, matocsserv_servers_count());
                i = 1 as uint32_t;
                while i < firstneverused {
                    if sclasstab[i as usize].nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        put8bit(&raw mut buff, i as uint8_t);
                        put8bit(&raw mut buff, sclasstab[i as usize].nleng);
                        memcpy(
                            buff as *mut ::core::ffi::c_void,
                            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize))
                                .name as *mut uint8_t
                                as *const ::core::ffi::c_void,
                            sclasstab[i as usize].nleng as size_t,
                        );
                        buff =
                            buff.offset(sclasstab[i as usize].nleng as ::core::ffi::c_int as isize);
                        if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, sclasstab[i as usize].dleng);
                            if sclasstab[i as usize].dleng as ::core::ffi::c_int
                                > 0 as ::core::ffi::c_int
                            {
                                memcpy(
                                    buff as *mut ::core::ffi::c_void,
                                    &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                        .offset(i as isize))
                                    .desc as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    sclasstab[i as usize].dleng as size_t,
                                );
                                buff = buff.offset(
                                    sclasstab[i as usize].dleng as ::core::ffi::c_int as isize,
                                );
                            }
                        }
                        put32bit(&raw mut buff, sclasstab[i as usize].files);
                        put32bit(&raw mut buff, sclasstab[i as usize].directories);
                        krlevel = sclass_calc_goal_equivalent(
                            &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                .offset(i as isize))
                            .keep,
                        );
                        arlevel = sclass_calc_goal_equivalent(
                            &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                .offset(i as isize))
                            .arch,
                        );
                        trlevel = sclass_calc_goal_equivalent(
                            &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                .offset(i as isize))
                            .trash,
                        );
                        j = 0 as uint32_t;
                        while j < 6 as uint32_t {
                            keepcnt[j as usize] = 0 as uint64_t;
                            archcnt[j as usize] = 0 as uint64_t;
                            trashcnt[j as usize] = 0 as uint64_t;
                            j = j.wrapping_add(1);
                        }
                        if arlevel as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && trlevel as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                0 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                1 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                2 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                3 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                        } else if arlevel as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                0 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                1 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                2 as uint8_t,
                                trlevel,
                                &raw mut trashcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                3 as uint8_t,
                                trlevel,
                                &raw mut trashcnt as *mut uint64_t,
                            );
                        } else if trlevel as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                0 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                1 as uint8_t,
                                arlevel,
                                &raw mut archcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                2 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                3 as uint8_t,
                                arlevel,
                                &raw mut archcnt as *mut uint64_t,
                            );
                        } else {
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                0 as uint8_t,
                                krlevel,
                                &raw mut keepcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                1 as uint8_t,
                                arlevel,
                                &raw mut archcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                2 as uint8_t,
                                trlevel,
                                &raw mut trashcnt as *mut uint64_t,
                            );
                            chunk_sclass_inc_counters(
                                i as uint8_t,
                                3 as uint8_t,
                                trlevel,
                                &raw mut trashcnt as *mut uint64_t,
                            );
                        }
                        if fver as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
                            j = 0 as uint32_t;
                            while j < 6 as uint32_t {
                                put64bit(&raw mut buff, keepcnt[j as usize]);
                                put64bit(&raw mut buff, archcnt[j as usize]);
                                put64bit(&raw mut buff, trashcnt[j as usize]);
                                j = j.wrapping_add(1);
                            }
                        } else {
                            j = 0 as uint32_t;
                            while j < 3 as uint32_t {
                                put64bit(
                                    &raw mut buff,
                                    keepcnt[(2 as uint32_t).wrapping_mul(j) as usize].wrapping_add(
                                        keepcnt[(2 as uint32_t)
                                            .wrapping_mul(j)
                                            .wrapping_add(1 as uint32_t)
                                            as usize],
                                    ),
                                );
                                put64bit(
                                    &raw mut buff,
                                    archcnt[(2 as uint32_t).wrapping_mul(j) as usize].wrapping_add(
                                        archcnt[(2 as uint32_t)
                                            .wrapping_mul(j)
                                            .wrapping_add(1 as uint32_t)
                                            as usize],
                                    ),
                                );
                                put64bit(
                                    &raw mut buff,
                                    trashcnt[(2 as uint32_t).wrapping_mul(j) as usize]
                                        .wrapping_add(
                                            trashcnt[(2 as uint32_t)
                                                .wrapping_mul(j)
                                                .wrapping_add(1 as uint32_t)
                                                as usize],
                                        ),
                                );
                                j = j.wrapping_add(1);
                            }
                        }
                        if fver as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                            put32bit(&raw mut buff, sclasstab[i as usize].priority);
                            put8bit(&raw mut buff, sclasstab[i as usize].export_group);
                        }
                        put8bit(&raw mut buff, sclasstab[i as usize].admin_only);
                        put8bit(&raw mut buff, sclasstab[i as usize].labels_mode);
                        if fver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, sclasstab[i as usize].arch_mode);
                        }
                        put16bit(&raw mut buff, sclasstab[i as usize].arch_delay);
                        if fver as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                            put64bit(&raw mut buff, sclasstab[i as usize].arch_min_size);
                        }
                        put16bit(&raw mut buff, sclasstab[i as usize].min_trashretention);
                        put8bit(
                            &raw mut buff,
                            chunk_labelset_can_be_fulfilled(
                                &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(i as isize))
                                .create,
                            ),
                        );
                        put8bit(&raw mut buff, sclasstab[i as usize].create.labelscnt);
                        put32bit(&raw mut buff, sclasstab[i as usize].create.uniqmask);
                        if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, sclasstab[i as usize].create.labels_mode);
                        }
                        put8bit(
                            &raw mut buff,
                            chunk_labelset_can_be_fulfilled(
                                &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(i as isize))
                                .keep,
                            ),
                        );
                        put8bit(&raw mut buff, sclasstab[i as usize].keep.labelscnt);
                        put32bit(&raw mut buff, sclasstab[i as usize].keep.uniqmask);
                        if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, sclasstab[i as usize].keep.labels_mode);
                        }
                        if sclasstab[i as usize].arch.ec_data_chksum_parts as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int
                            > MaxECRedundancyLevel as ::core::ffi::c_int
                        {
                            tmp_storagemode = sclasstab[i as usize].arch;
                            tmp_storagemode.ec_data_chksum_parts =
                                (tmp_storagemode.ec_data_chksum_parts as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                                    as uint8_t;
                            put8bit(
                                &raw mut buff,
                                chunk_labelset_can_be_fulfilled(&raw mut tmp_storagemode),
                            );
                        } else {
                            put8bit(
                                &raw mut buff,
                                chunk_labelset_can_be_fulfilled(
                                    &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                        .offset(i as isize))
                                    .arch,
                                ),
                            );
                        }
                        put8bit(&raw mut buff, sclasstab[i as usize].arch.labelscnt);
                        if sclasstab[i as usize].arch.ec_data_chksum_parts as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int
                            > MaxECRedundancyLevel as ::core::ffi::c_int
                        {
                            put8bit(
                                &raw mut buff,
                                (sclasstab[i as usize].arch.ec_data_chksum_parts
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                                    as uint8_t,
                            );
                        } else {
                            put8bit(
                                &raw mut buff,
                                sclasstab[i as usize].arch.ec_data_chksum_parts,
                            );
                        }
                        put32bit(&raw mut buff, sclasstab[i as usize].arch.uniqmask);
                        if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, sclasstab[i as usize].arch.labels_mode);
                        }
                        if sclasstab[i as usize].trash.ec_data_chksum_parts as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int
                            > MaxECRedundancyLevel as ::core::ffi::c_int
                        {
                            tmp_storagemode = sclasstab[i as usize].trash;
                            tmp_storagemode.ec_data_chksum_parts =
                                (tmp_storagemode.ec_data_chksum_parts as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                                    as uint8_t;
                            put8bit(
                                &raw mut buff,
                                chunk_labelset_can_be_fulfilled(&raw mut tmp_storagemode),
                            );
                        } else {
                            put8bit(
                                &raw mut buff,
                                chunk_labelset_can_be_fulfilled(
                                    &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                        .offset(i as isize))
                                    .trash,
                                ),
                            );
                        }
                        put8bit(&raw mut buff, sclasstab[i as usize].trash.labelscnt);
                        if sclasstab[i as usize].trash.ec_data_chksum_parts as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int
                            > MaxECRedundancyLevel as ::core::ffi::c_int
                        {
                            put8bit(
                                &raw mut buff,
                                (sclasstab[i as usize].trash.ec_data_chksum_parts
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    | MaxECRedundancyLevel as ::core::ffi::c_int)
                                    as uint8_t,
                            );
                        } else {
                            put8bit(
                                &raw mut buff,
                                sclasstab[i as usize].trash.ec_data_chksum_parts,
                            );
                        }
                        put32bit(&raw mut buff, sclasstab[i as usize].trash.uniqmask);
                        if fver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, sclasstab[i as usize].trash.labels_mode);
                        }
                        j = 0 as uint32_t;
                        while j < sclasstab[i as usize].create.labelscnt as uint32_t {
                            memcpy(
                                buff as *mut ::core::ffi::c_void,
                                &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(i as isize))
                                .create
                                .labelexpr
                                    as *mut [uint8_t; 128])
                                    .offset(j as isize)
                                    as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                SCLASS_EXPR_MAX_SIZE as size_t,
                            );
                            buff = buff.offset(SCLASS_EXPR_MAX_SIZE as isize);
                            put16bit(
                                &raw mut buff,
                                matocsserv_servers_matches_labelexpr(
                                    &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                        .offset(i as isize))
                                    .create
                                    .labelexpr
                                        as *mut [uint8_t; 128])
                                        .offset(j as isize)
                                        as *mut uint8_t
                                        as *const uint8_t,
                                ),
                            );
                            j = j.wrapping_add(1);
                        }
                        j = 0 as uint32_t;
                        while j < sclasstab[i as usize].keep.labelscnt as uint32_t {
                            memcpy(
                                buff as *mut ::core::ffi::c_void,
                                &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(i as isize))
                                .keep
                                .labelexpr
                                    as *mut [uint8_t; 128])
                                    .offset(j as isize)
                                    as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                SCLASS_EXPR_MAX_SIZE as size_t,
                            );
                            buff = buff.offset(SCLASS_EXPR_MAX_SIZE as isize);
                            put16bit(
                                &raw mut buff,
                                matocsserv_servers_matches_labelexpr(
                                    &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                        .offset(i as isize))
                                    .keep
                                    .labelexpr
                                        as *mut [uint8_t; 128])
                                        .offset(j as isize)
                                        as *mut uint8_t
                                        as *const uint8_t,
                                ),
                            );
                            j = j.wrapping_add(1);
                        }
                        j = 0 as uint32_t;
                        while j < sclasstab[i as usize].arch.labelscnt as uint32_t {
                            memcpy(
                                buff as *mut ::core::ffi::c_void,
                                &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(i as isize))
                                .arch
                                .labelexpr
                                    as *mut [uint8_t; 128])
                                    .offset(j as isize)
                                    as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                SCLASS_EXPR_MAX_SIZE as size_t,
                            );
                            buff = buff.offset(SCLASS_EXPR_MAX_SIZE as isize);
                            put16bit(
                                &raw mut buff,
                                matocsserv_servers_matches_labelexpr(
                                    &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                        .offset(i as isize))
                                    .arch
                                    .labelexpr
                                        as *mut [uint8_t; 128])
                                        .offset(j as isize)
                                        as *mut uint8_t
                                        as *const uint8_t,
                                ),
                            );
                            j = j.wrapping_add(1);
                        }
                        j = 0 as uint32_t;
                        while j < sclasstab[i as usize].trash.labelscnt as uint32_t {
                            memcpy(
                                buff as *mut ::core::ffi::c_void,
                                &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                    .offset(i as isize))
                                .trash
                                .labelexpr
                                    as *mut [uint8_t; 128])
                                    .offset(j as isize)
                                    as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                SCLASS_EXPR_MAX_SIZE as size_t,
                            );
                            buff = buff.offset(SCLASS_EXPR_MAX_SIZE as isize);
                            put16bit(
                                &raw mut buff,
                                matocsserv_servers_matches_labelexpr(
                                    &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                                        .offset(i as isize))
                                    .trash
                                    .labelexpr
                                        as *mut [uint8_t; 128])
                                        .offset(j as isize)
                                        as *mut uint8_t
                                        as *const uint8_t,
                                ),
                            );
                            j = j.wrapping_add(1);
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            return 0 as uint32_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_store(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut databuff: [uint8_t; 5170] = [0; 5170];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint16_t = 0;
        let mut j: uint16_t = 0;
        let mut wsize: int32_t = 0;
        if fd.is_null() {
            return 0x1c as uint8_t;
        }
        ptr = &raw mut databuff as *mut uint8_t;
        put16bit(&raw mut ptr, SCLASS_EXPR_MAX_SIZE as uint16_t);
        put8bit(&raw mut ptr, ec_current_version);
        if bio_write(
            fd,
            &raw mut databuff as *mut uint8_t as *const ::core::ffi::c_void,
            3 as uint64_t,
        ) != 3 as int64_t
        {
            return 0xff as uint8_t;
        }
        i = 1 as uint16_t;
        while (i as uint32_t) < firstneverused {
            if sclasstab[i as usize].nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                ptr = &raw mut databuff as *mut uint8_t;
                put16bit(&raw mut ptr, i);
                put8bit(&raw mut ptr, sclasstab[i as usize].nleng);
                put8bit(&raw mut ptr, sclasstab[i as usize].dleng);
                put32bit(&raw mut ptr, sclasstab[i as usize].priority);
                put8bit(&raw mut ptr, sclasstab[i as usize].export_group);
                put8bit(&raw mut ptr, sclasstab[i as usize].admin_only);
                put8bit(&raw mut ptr, sclasstab[i as usize].labels_mode);
                put8bit(&raw mut ptr, sclasstab[i as usize].arch_mode);
                put16bit(&raw mut ptr, sclasstab[i as usize].arch_delay);
                put64bit(&raw mut ptr, sclasstab[i as usize].arch_min_size);
                put16bit(&raw mut ptr, sclasstab[i as usize].min_trashretention);
                put8bit(&raw mut ptr, sclasstab[i as usize].create.labelscnt);
                put32bit(&raw mut ptr, sclasstab[i as usize].create.uniqmask);
                put8bit(&raw mut ptr, sclasstab[i as usize].create.labels_mode);
                put8bit(&raw mut ptr, sclasstab[i as usize].keep.labelscnt);
                put32bit(&raw mut ptr, sclasstab[i as usize].keep.uniqmask);
                put8bit(&raw mut ptr, sclasstab[i as usize].keep.labels_mode);
                put8bit(&raw mut ptr, sclasstab[i as usize].arch.labelscnt);
                put8bit(
                    &raw mut ptr,
                    sclasstab[i as usize].arch.ec_data_chksum_parts,
                );
                put32bit(&raw mut ptr, sclasstab[i as usize].arch.uniqmask);
                put8bit(&raw mut ptr, sclasstab[i as usize].arch.labels_mode);
                put8bit(&raw mut ptr, sclasstab[i as usize].trash.labelscnt);
                put8bit(
                    &raw mut ptr,
                    sclasstab[i as usize].trash.ec_data_chksum_parts,
                );
                put32bit(&raw mut ptr, sclasstab[i as usize].trash.uniqmask);
                put8bit(&raw mut ptr, sclasstab[i as usize].trash.labels_mode);
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).name
                        as *mut uint8_t as *const ::core::ffi::c_void,
                    sclasstab[i as usize].nleng as size_t,
                );
                ptr = ptr.offset(sclasstab[i as usize].nleng as ::core::ffi::c_int as isize);
                if sclasstab[i as usize].dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize))
                            .desc as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        sclasstab[i as usize].dleng as size_t,
                    );
                    ptr = ptr.offset(sclasstab[i as usize].dleng as ::core::ffi::c_int as isize);
                }
                j = 0 as uint16_t;
                while (j as ::core::ffi::c_int)
                    < sclasstab[i as usize].create.labelscnt as ::core::ffi::c_int
                {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                            .offset(i as isize))
                        .create
                        .labelexpr as *mut [uint8_t; 128])
                            .offset(j as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    ptr = ptr.offset(SCLASS_EXPR_MAX_SIZE as isize);
                    j = j.wrapping_add(1);
                }
                j = 0 as uint16_t;
                while (j as ::core::ffi::c_int)
                    < sclasstab[i as usize].keep.labelscnt as ::core::ffi::c_int
                {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                            .offset(i as isize))
                        .keep
                        .labelexpr as *mut [uint8_t; 128])
                            .offset(j as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    ptr = ptr.offset(SCLASS_EXPR_MAX_SIZE as isize);
                    j = j.wrapping_add(1);
                }
                j = 0 as uint16_t;
                while (j as ::core::ffi::c_int)
                    < sclasstab[i as usize].arch.labelscnt as ::core::ffi::c_int
                {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                            .offset(i as isize))
                        .arch
                        .labelexpr as *mut [uint8_t; 128])
                            .offset(j as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    ptr = ptr.offset(SCLASS_EXPR_MAX_SIZE as isize);
                    j = j.wrapping_add(1);
                }
                j = 0 as uint16_t;
                while (j as ::core::ffi::c_int)
                    < sclasstab[i as usize].trash.labelscnt as ::core::ffi::c_int
                {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut (*(&raw mut sclasstab as *mut storageclass)
                            .offset(i as isize))
                        .trash
                        .labelexpr as *mut [uint8_t; 128])
                            .offset(j as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        SCLASS_EXPR_MAX_SIZE as size_t,
                    );
                    ptr = ptr.offset(SCLASS_EXPR_MAX_SIZE as isize);
                    j = j.wrapping_add(1);
                }
                wsize = (50 as ::core::ffi::c_int
                    + sclasstab[i as usize].nleng as ::core::ffi::c_int
                    + sclasstab[i as usize].dleng as ::core::ffi::c_int
                    + (sclasstab[i as usize].create.labelscnt as ::core::ffi::c_int
                        + sclasstab[i as usize].keep.labelscnt as ::core::ffi::c_int
                        + sclasstab[i as usize].arch.labelscnt as ::core::ffi::c_int
                        + sclasstab[i as usize].trash.labelscnt as ::core::ffi::c_int)
                        * SCLASS_EXPR_MAX_SIZE) as int32_t;
                if bio_write(
                    fd,
                    &raw mut databuff as *mut uint8_t as *const ::core::ffi::c_void,
                    wsize as uint64_t,
                ) != wsize as int64_t
                {
                    return 0xff as uint8_t;
                }
            }
            i = i.wrapping_add(1);
        }
        ptr = &raw mut databuff as *mut uint8_t;
        put16bit(&raw mut ptr, 0 as uint16_t);
        if bio_write(
            fd,
            &raw mut databuff as *mut uint8_t as *const ::core::ffi::c_void,
            2 as uint64_t,
        ) != 2 as int64_t
        {
            return 0xff as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_load(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut databuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut hdrbuff: [uint8_t; 3] = [0; 3];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut chunkcount: uint32_t = 0;
        let mut sclassid: uint16_t = 0;
        let mut arch_delay: uint16_t = 0;
        let mut min_trashretention: uint16_t = 0;
        let mut exprsize: uint16_t = 0;
        let mut lexprsize: uint16_t = 0;
        let mut labels_mode: uint8_t = 0;
        let mut arch_mode: uint8_t = 0;
        let mut arch_min_size: uint64_t = 0;
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
        let mut smptr: *mut storagemode = ::core::ptr::null_mut::<storagemode>();
        let mut descrleng: uint8_t = 0;
        let mut nleng: uint8_t = 0;
        let mut dleng: uint8_t = 0;
        let mut priority: uint32_t = 0;
        let mut export_group: uint8_t = 0;
        let mut admin_only: uint8_t = 0;
        let mut name: [uint8_t; 256] = [0; 256];
        let mut desc: [uint8_t; 256] = [0; 256];
        let mut i: uint16_t = 0;
        let mut j: uint16_t = 0;
        let mut orgroup: uint8_t = 0;
        let mut hdrleng: uint8_t = 0;
        let mut labelmasks: [uint32_t; 36] = [0; 36];
        if (mver as ::core::ffi::c_int) < 0x16 as ::core::ffi::c_int {
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < 26 as ::core::ffi::c_int {
                if bio_read(
                    fd,
                    &raw mut descrleng as *mut ::core::ffi::c_void,
                    1 as uint64_t,
                ) != 1 as int64_t
                {
                    let mut err: ::core::ffi::c_int = *__errno_location();
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    *__errno_location() = err;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: read error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if descrleng as ::core::ffi::c_int > 128 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: description too long\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                bio_skip(fd, descrleng as uint64_t);
                i = i.wrapping_add(1);
            }
        }
        if mver as ::core::ffi::c_int >= 0x17 as ::core::ffi::c_int {
            let mut psize: uint8_t = 0;
            psize = (if mver as ::core::ffi::c_int >= 0x1c as ::core::ffi::c_int {
                48 as ::core::ffi::c_int
            } else if mver as ::core::ffi::c_int >= 0x1b as ::core::ffi::c_int {
                42 as ::core::ffi::c_int
            } else if mver as ::core::ffi::c_int >= 0x1a as ::core::ffi::c_int {
                38 as ::core::ffi::c_int
            } else if mver as ::core::ffi::c_int >= 0x18 as ::core::ffi::c_int {
                30 as ::core::ffi::c_int
            } else {
                29 as ::core::ffi::c_int
            }) as uint8_t;
            if bio_read(
                fd,
                &raw mut hdrbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                3 as uint64_t,
            ) != 3 as int64_t
            {
                let mut err_0: ::core::ffi::c_int = *__errno_location();
                fputc('\n' as ::core::ffi::c_int, stderr);
                *__errno_location() = err_0;
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading storage class: read error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut hdrbuff as *mut uint8_t;
            exprsize = get16bit(&raw mut ptr);
            if exprsize as ::core::ffi::c_int > SCLASS_EXPR_MAX_SIZE {
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"loading storage class data: label expressions too long - ignore\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: label expressions too long\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
            if exprsize as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"loading storage class data: label expressions size is zero !!! - ignoring\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: label expressions size is zero !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
            ec_current_version = get8bit(&raw mut ptr);
            databuff = malloc(psize as size_t) as *mut uint8_t;
            if databuff.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/storageclass.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/storageclass.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if databuff
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/storageclass.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/storageclass.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            loop {
                if bio_read(fd, databuff as *mut ::core::ffi::c_void, 2 as uint64_t) != 2 as int64_t
                {
                    let mut err_1: ::core::ffi::c_int = *__errno_location();
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    *__errno_location() = err_1;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: read error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    databuff = ::core::ptr::null_mut::<uint8_t>();
                    return -1 as ::core::ffi::c_int;
                }
                ptr = databuff;
                sclassid = get16bit(&raw mut ptr);
                if sclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                if bio_read(fd, databuff as *mut ::core::ffi::c_void, psize as uint64_t)
                    != psize as int64_t
                {
                    let mut err_2: ::core::ffi::c_int = *__errno_location();
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    *__errno_location() = err_2;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: read error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    databuff = ::core::ptr::null_mut::<uint8_t>();
                    return -1 as ::core::ffi::c_int;
                }
                ptr = databuff;
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
                nleng = get8bit(&raw mut ptr);
                if mver as ::core::ffi::c_int >= 0x1c as ::core::ffi::c_int {
                    dleng = get8bit(&raw mut ptr);
                    priority = get32bit(&raw mut ptr);
                    export_group = get8bit(&raw mut ptr);
                } else {
                    dleng = 0 as uint8_t;
                    priority = 0 as uint32_t;
                    if (sclassid as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                        export_group = sclassid as uint8_t;
                    } else {
                        export_group = 0 as uint8_t;
                    }
                }
                admin_only = get8bit(&raw mut ptr);
                labels_mode = get8bit(&raw mut ptr);
                if mver as ::core::ffi::c_int >= 0x18 as ::core::ffi::c_int {
                    arch_mode = get8bit(&raw mut ptr);
                    if arch_mode as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        arch_mode = (arch_mode as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                            as uint8_t;
                    }
                    if arch_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
                    }
                } else {
                    arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
                }
                arch_delay = get16bit(&raw mut ptr);
                if mver as ::core::ffi::c_int >= 0x1a as ::core::ffi::c_int {
                    arch_min_size = get64bit(&raw mut ptr);
                } else {
                    arch_min_size = 0 as uint64_t;
                }
                min_trashretention = get16bit(&raw mut ptr);
                create.labelscnt = get8bit(&raw mut ptr);
                create.uniqmask = get32bit(&raw mut ptr);
                if mver as ::core::ffi::c_int >= 0x1b as ::core::ffi::c_int {
                    create.labels_mode = get8bit(&raw mut ptr);
                }
                keep.labelscnt = get8bit(&raw mut ptr);
                keep.uniqmask = get32bit(&raw mut ptr);
                if mver as ::core::ffi::c_int >= 0x1b as ::core::ffi::c_int {
                    keep.labels_mode = get8bit(&raw mut ptr);
                }
                arch.labelscnt = get8bit(&raw mut ptr);
                arch.ec_data_chksum_parts = get8bit(&raw mut ptr);
                arch.uniqmask = get32bit(&raw mut ptr);
                if mver as ::core::ffi::c_int >= 0x1b as ::core::ffi::c_int {
                    arch.labels_mode = get8bit(&raw mut ptr);
                }
                trash.labelscnt = get8bit(&raw mut ptr);
                trash.ec_data_chksum_parts = get8bit(&raw mut ptr);
                trash.uniqmask = get32bit(&raw mut ptr);
                if mver as ::core::ffi::c_int >= 0x1b as ::core::ffi::c_int {
                    trash.labels_mode = get8bit(&raw mut ptr);
                }
                if (mver as ::core::ffi::c_int) < 0x19 as ::core::ffi::c_int {
                    if arch.ec_data_chksum_parts as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        arch.ec_data_chksum_parts = arch.ec_data_chksum_parts.wrapping_sub(1);
                    }
                    if trash.ec_data_chksum_parts as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        trash.ec_data_chksum_parts = trash.ec_data_chksum_parts.wrapping_sub(1);
                    }
                }
                if bio_read(
                    fd,
                    &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
                    nleng as uint64_t,
                ) != nleng as int64_t
                {
                    let mut err_3: ::core::ffi::c_int = *__errno_location();
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    *__errno_location() = err_3;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: read error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    databuff = ::core::ptr::null_mut::<uint8_t>();
                    return -1 as ::core::ffi::c_int;
                }
                if nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (mver as ::core::ffi::c_int) < 0x1c as ::core::ffi::c_int
                {
                    i = 0 as uint16_t;
                    while (i as ::core::ffi::c_int) < nleng as ::core::ffi::c_int {
                        if (name[i as usize] as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                            name[i as usize] = 32 as uint8_t;
                        }
                        i = i.wrapping_add(1);
                    }
                    if name[0 as usize] as ::core::ffi::c_int == 32 as ::core::ffi::c_int {
                        name[0 as usize] = '_' as uint8_t;
                    }
                    if name[(nleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        == 32 as ::core::ffi::c_int
                    {
                        name[(nleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] =
                            '_' as uint8_t;
                    }
                }
                if dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    if bio_read(
                        fd,
                        &raw mut desc as *mut uint8_t as *mut ::core::ffi::c_void,
                        dleng as uint64_t,
                    ) != dleng as int64_t
                    {
                        let mut err_4: ::core::ffi::c_int = *__errno_location();
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        *__errno_location() = err_4;
                        mfs_log(
                            MFSLOG_ERRNO_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: read error\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        free(databuff as *mut ::core::ffi::c_void);
                        databuff = ::core::ptr::null_mut::<uint8_t>();
                        return -1 as ::core::ffi::c_int;
                    }
                }
                if keep.labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || keep.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                    || create.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                    || arch.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                    || trash.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                    || create.ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    || keep.ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    || arch.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                        > MAX_EC_LEVEL
                    || trash.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                        > MAX_EC_LEVEL
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: sclassid: %hu - data format error (labelscnt,ec_data_chksum_parts) for create: (%hhu,0x%02hhX) ; keep: (%hhu,0x%02hhX) ; arch: (%hhu,0x%02hhX) ; trash: (%hhu,0x%02hhX)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        sclassid as ::core::ffi::c_int,
                        create.labelscnt as ::core::ffi::c_int,
                        create.ec_data_chksum_parts as ::core::ffi::c_int,
                        keep.labelscnt as ::core::ffi::c_int,
                        keep.ec_data_chksum_parts as ::core::ffi::c_int,
                        arch.labelscnt as ::core::ffi::c_int,
                        arch.ec_data_chksum_parts as ::core::ffi::c_int,
                        trash.labelscnt as ::core::ffi::c_int,
                        trash.ec_data_chksum_parts as ::core::ffi::c_int,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    databuff = ::core::ptr::null_mut::<uint8_t>();
                    return -1 as ::core::ffi::c_int;
                }
                if arch.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && arch.ec_data_chksum_parts as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                        != 0
                    || trash.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && trash.ec_data_chksum_parts as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            != 0
                {
                    if ignoreflag == 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: sclassid: %hu - data format error (ec with data parts and no checksums) - use '-i' to ignore\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                        );
                        free(databuff as *mut ::core::ffi::c_void);
                        databuff = ::core::ptr::null_mut::<uint8_t>();
                        return -1 as ::core::ffi::c_int;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading storage class data: sclassid: %hu - data format error (ec with data parts and no checksums) - ignoring\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                        );
                    }
                    if arch.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && arch.ec_data_chksum_parts as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            != 0
                    {
                        arch.ec_data_chksum_parts = 0 as uint8_t;
                    }
                    if trash.ec_data_chksum_parts as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        && trash.ec_data_chksum_parts as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            != 0
                    {
                        trash.ec_data_chksum_parts = 0 as uint8_t;
                    }
                }
                if arch.ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    if arch.ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        != 0
                    {
                        if arch.ec_data_chksum_parts as ::core::ffi::c_int
                            >> 4 as ::core::ffi::c_int
                            != 4 as ::core::ffi::c_int
                            && arch.ec_data_chksum_parts as ::core::ffi::c_int
                                >> 4 as ::core::ffi::c_int
                                != 8 as ::core::ffi::c_int
                        {
                            if ignoreflag == 0 as ::core::ffi::c_int {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"loading storage class data: sclassid: %hu - data format error (number of data parts in arch mode: %u) - use '-i' to ignore\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    sclassid as ::core::ffi::c_int,
                                    arch.ec_data_chksum_parts as ::core::ffi::c_int
                                        >> 4 as ::core::ffi::c_int,
                                );
                                free(databuff as *mut ::core::ffi::c_void);
                                databuff = ::core::ptr::null_mut::<uint8_t>();
                                return -1 as ::core::ffi::c_int;
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_NOTICE,
                                    b"loading storage class data: sclassid: %hu - data format error (number of data parts in arch mode: %u) - ignoring\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    sclassid as ::core::ffi::c_int,
                                    arch.ec_data_chksum_parts as ::core::ffi::c_int
                                        >> 4 as ::core::ffi::c_int,
                                );
                            }
                            arch.ec_data_chksum_parts = (arch.ec_data_chksum_parts
                                as ::core::ffi::c_int
                                & 0xf as ::core::ffi::c_int
                                | COMPAT_ECMODE << 4 as ::core::ffi::c_int)
                                as uint8_t;
                        }
                    } else {
                        arch.ec_data_chksum_parts = (arch.ec_data_chksum_parts
                            as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int
                            | COMPAT_ECMODE << 4 as ::core::ffi::c_int)
                            as uint8_t;
                    }
                }
                if trash.ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    if trash.ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        != 0
                    {
                        if trash.ec_data_chksum_parts as ::core::ffi::c_int
                            >> 4 as ::core::ffi::c_int
                            != 4 as ::core::ffi::c_int
                            && trash.ec_data_chksum_parts as ::core::ffi::c_int
                                >> 4 as ::core::ffi::c_int
                                != 8 as ::core::ffi::c_int
                        {
                            if ignoreflag == 0 as ::core::ffi::c_int {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"loading storage class data: sclassid: %hu - data format error (number of data parts in trash mode: %u) - use '-i' to ignore\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    sclassid as ::core::ffi::c_int,
                                    trash.ec_data_chksum_parts as ::core::ffi::c_int
                                        >> 4 as ::core::ffi::c_int,
                                );
                                free(databuff as *mut ::core::ffi::c_void);
                                databuff = ::core::ptr::null_mut::<uint8_t>();
                                return -1 as ::core::ffi::c_int;
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_NOTICE,
                                    b"loading storage class data: sclassid: %hu - data format error (number of data parts in trash mode: %u) - ignoring\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    sclassid as ::core::ffi::c_int,
                                    trash.ec_data_chksum_parts as ::core::ffi::c_int
                                        >> 4 as ::core::ffi::c_int,
                                );
                            }
                            trash.ec_data_chksum_parts = (trash.ec_data_chksum_parts
                                as ::core::ffi::c_int
                                & 0xf as ::core::ffi::c_int
                                | COMPAT_ECMODE << 4 as ::core::ffi::c_int)
                                as uint8_t;
                        }
                    } else {
                        trash.ec_data_chksum_parts = (trash.ec_data_chksum_parts
                            as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int
                            | COMPAT_ECMODE << 4 as ::core::ffi::c_int)
                            as uint8_t;
                    }
                }
                if (arch.ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    || trash.ec_data_chksum_parts as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
                    && (ec_current_version as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
                {
                    ec_current_version = 1 as uint8_t;
                    if ignoreflag == 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: sclassid: %hu - data format error (arch.ec_data_chksum_parts: 0x%02hhX ; trash.ec_data_chksum_parts: 0x%02hhX ; ec_current_version: %u)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                            arch.ec_data_chksum_parts as ::core::ffi::c_int,
                            trash.ec_data_chksum_parts as ::core::ffi::c_int,
                            ec_current_version as ::core::ffi::c_int,
                        );
                        free(databuff as *mut ::core::ffi::c_void);
                        databuff = ::core::ptr::null_mut::<uint8_t>();
                        return -1 as ::core::ffi::c_int;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading storage class data: sclassid: %hu - data format error (arch.ec_data_chksum_parts: 0x%02hhX ; trash.ec_data_chksum_parts: 0x%02hhX ; ec_current_version: %u) - ignoring\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                            arch.ec_data_chksum_parts as ::core::ffi::c_int,
                            trash.ec_data_chksum_parts as ::core::ffi::c_int,
                            ec_current_version as ::core::ffi::c_int,
                        );
                    }
                }
                if (arch.ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    == 4 as ::core::ffi::c_int
                    || trash.ec_data_chksum_parts as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        == 4 as ::core::ffi::c_int)
                    && (ec_current_version as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
                {
                    ec_current_version = 2 as uint8_t;
                    if ignoreflag == 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: sclassid: %hu - data format error (arch.ec_data_chksum_parts: 0x%02hhX ; trash.ec_data_chksum_parts: 0x%02hhX ; ec_current_version: %u)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                            arch.ec_data_chksum_parts as ::core::ffi::c_int,
                            trash.ec_data_chksum_parts as ::core::ffi::c_int,
                            ec_current_version as ::core::ffi::c_int,
                        );
                        free(databuff as *mut ::core::ffi::c_void);
                        databuff = ::core::ptr::null_mut::<uint8_t>();
                        return -1 as ::core::ffi::c_int;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading storage class data: sclassid: %hu - data format error (arch.ec_data_chksum_parts: 0x%02hhX ; trash.ec_data_chksum_parts: 0x%02hhX ; ec_current_version: %u) - ignoring\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                            arch.ec_data_chksum_parts as ::core::ffi::c_int,
                            trash.ec_data_chksum_parts as ::core::ffi::c_int,
                            ec_current_version as ::core::ffi::c_int,
                        );
                    }
                }
                if sclassid as ::core::ffi::c_int >= MAXSCLASS {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading storage class data: bad sclassid (%hu) - ignore\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                        );
                        bio_skip(
                            fd,
                            ((create.labelscnt as ::core::ffi::c_int
                                + keep.labelscnt as ::core::ffi::c_int
                                + arch.labelscnt as ::core::ffi::c_int
                                + trash.labelscnt as ::core::ffi::c_int)
                                * exprsize as ::core::ffi::c_int)
                                as uint64_t,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: bad sclassid (%hu)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                        );
                        free(databuff as *mut ::core::ffi::c_void);
                        databuff = ::core::ptr::null_mut::<uint8_t>();
                        return -1 as ::core::ffi::c_int;
                    }
                } else if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                    > 0 as ::core::ffi::c_int
                {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading storage class data: repeated sclassid (%hu) - ignore\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                        );
                        bio_skip(
                            fd,
                            ((create.labelscnt as ::core::ffi::c_int
                                + keep.labelscnt as ::core::ffi::c_int
                                + arch.labelscnt as ::core::ffi::c_int
                                + trash.labelscnt as ::core::ffi::c_int)
                                * exprsize as ::core::ffi::c_int)
                                as uint64_t,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: repeated sclassid (%hu)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                        );
                        free(databuff as *mut ::core::ffi::c_void);
                        databuff = ::core::ptr::null_mut::<uint8_t>();
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    if exprsize as ::core::ffi::c_int > SCLASS_EXPR_MAX_SIZE {
                        lexprsize = SCLASS_EXPR_MAX_SIZE as uint16_t;
                    } else {
                        lexprsize = exprsize;
                    }
                    smptr = ::core::ptr::null_mut::<storagemode>();
                    i = 0 as uint16_t;
                    while (i as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
                        match i as ::core::ffi::c_int {
                            0 => {
                                smptr = &raw mut create;
                            }
                            1 => {
                                smptr = &raw mut keep;
                            }
                            2 => {
                                smptr = &raw mut arch;
                            }
                            3 => {
                                smptr = &raw mut trash;
                            }
                            _ => {}
                        }
                        j = 0 as uint16_t;
                        while (j as ::core::ffi::c_int) < (*smptr).labelscnt as ::core::ffi::c_int {
                            if bio_read(
                                fd,
                                &raw mut *(&raw mut (*smptr).labelexpr as *mut [uint8_t; 128])
                                    .offset(j as isize)
                                    as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                lexprsize as uint64_t,
                            ) != lexprsize as int64_t
                            {
                                let mut err_5: ::core::ffi::c_int = *__errno_location();
                                fputc('\n' as ::core::ffi::c_int, stderr);
                                *__errno_location() = err_5;
                                mfs_log(
                                    MFSLOG_ERRNO_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"loading storage class data: read error\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                free(databuff as *mut ::core::ffi::c_void);
                                databuff = ::core::ptr::null_mut::<uint8_t>();
                                return -1 as ::core::ffi::c_int;
                            }
                            if exprsize as ::core::ffi::c_int > SCLASS_EXPR_MAX_SIZE {
                                bio_skip(
                                    fd,
                                    (exprsize as ::core::ffi::c_int - SCLASS_EXPR_MAX_SIZE)
                                        as uint64_t,
                                );
                            }
                            j = j.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                    }
                    if arch.ec_data_chksum_parts as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if arch.labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            arch.labelscnt = 1 as uint8_t;
                            arch.labelexpr[0 as usize][0 as usize] = 0 as uint8_t;
                        }
                        if arch.labelscnt as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
                            if ignoreflag != 0 {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_NOTICE,
                                    b"loading storage class data: sclassid: %hu - bad labelscnt in archive definition (%u -> 2)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    sclassid as ::core::ffi::c_int,
                                    arch.labelscnt as ::core::ffi::c_int,
                                );
                                arch.labelscnt = 2 as uint8_t;
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"loading storage class data: sclassid %hu - bad labelscnt in archive definition (%u)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    sclassid as ::core::ffi::c_int,
                                    arch.labelscnt as ::core::ffi::c_int,
                                );
                                free(databuff as *mut ::core::ffi::c_void);
                                databuff = ::core::ptr::null_mut::<uint8_t>();
                                return -1 as ::core::ffi::c_int;
                            }
                        }
                    }
                    if trash.ec_data_chksum_parts as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if trash.labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            trash.labelscnt = 1 as uint8_t;
                            trash.labelexpr[0 as usize][0 as usize] = 0 as uint8_t;
                        }
                        if trash.labelscnt as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
                            if ignoreflag != 0 {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_NOTICE,
                                    b"loading storage class data: sclassid: %hu - bad labelscnt in trash definition (%u -> 2)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    sclassid as ::core::ffi::c_int,
                                    trash.labelscnt as ::core::ffi::c_int,
                                );
                                trash.labelscnt = 2 as uint8_t;
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"loading storage class data: sclassid %hu - bad labelscnt in trash definition (%u)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    sclassid as ::core::ffi::c_int,
                                    trash.labelscnt as ::core::ffi::c_int,
                                );
                                free(databuff as *mut ::core::ffi::c_void);
                                databuff = ::core::ptr::null_mut::<uint8_t>();
                                return -1 as ::core::ffi::c_int;
                            }
                        }
                    }
                    sclasstab[sclassid as usize].nleng = nleng;
                    memcpy(
                        &raw mut (*(&raw mut sclasstab as *mut storageclass)
                            .offset(sclassid as isize))
                        .name as *mut uint8_t as *mut ::core::ffi::c_void,
                        &raw mut name as *mut uint8_t as *const ::core::ffi::c_void,
                        nleng as size_t,
                    );
                    sclasstab[sclassid as usize].dleng = dleng;
                    if dleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        memcpy(
                            &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                .offset(sclassid as isize))
                            .desc as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            &raw mut desc as *mut uint8_t as *const ::core::ffi::c_void,
                            dleng as size_t,
                        );
                    }
                    sclasstab[sclassid as usize].priority = priority;
                    sclasstab[sclassid as usize].export_group = export_group;
                    sclasstab[sclassid as usize].admin_only = admin_only;
                    sclasstab[sclassid as usize].labels_mode = labels_mode;
                    sclasstab[sclassid as usize].arch_mode = arch_mode;
                    sclasstab[sclassid as usize].create = create;
                    sclasstab[sclassid as usize].keep = keep;
                    sclasstab[sclassid as usize].arch = arch;
                    sclasstab[sclassid as usize].trash = trash;
                    sclasstab[sclassid as usize].arch_delay = arch_delay;
                    sclasstab[sclassid as usize].min_trashretention = min_trashretention;
                    sclasstab[sclassid as usize].arch_min_size = arch_min_size;
                    sclasstab[sclassid as usize].files = 0 as uint32_t;
                    sclasstab[sclassid as usize].directories = 0 as uint32_t;
                    sclass_fix_has_labels_fields(sclassid as uint8_t);
                    if (mver as ::core::ffi::c_int) < 0x1b as ::core::ffi::c_int
                        && labels_mode as ::core::ffi::c_int != LABELS_MODE_LOOSE
                    {
                        if arch.labelscnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                            && arch.ec_data_chksum_parts as ::core::ffi::c_int != 0
                            && sclasstab[sclassid as usize].arch.has_labels as ::core::ffi::c_int
                                != 0
                        {
                            sclasstab[sclassid as usize].arch.labels_mode =
                                LABELS_MODE_LOOSE as uint8_t;
                        }
                        if trash.labelscnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                            && trash.ec_data_chksum_parts as ::core::ffi::c_int != 0
                            && sclasstab[sclassid as usize].trash.has_labels as ::core::ffi::c_int
                                != 0
                        {
                            sclasstab[sclassid as usize].trash.labels_mode =
                                LABELS_MODE_LOOSE as uint8_t;
                        }
                    }
                    if sclassid as uint32_t >= firstneverused {
                        firstneverused =
                            (sclassid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t;
                    }
                }
            }
        } else {
            if mver as ::core::ffi::c_int == 0x10 as ::core::ffi::c_int {
                orgroup = 1 as uint8_t;
            } else {
                if bio_read(
                    fd,
                    &raw mut orgroup as *mut ::core::ffi::c_void,
                    1 as uint64_t,
                ) != 1 as int64_t
                {
                    let mut err_6: ::core::ffi::c_int = *__errno_location();
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    *__errno_location() = err_6;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class: read error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                if orgroup as ::core::ffi::c_int > MASKORGROUP {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading storage class data: too many or-groups - ignore\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: too many or-groups\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
            if (orgroup as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading storage class data: zero or-groups !!!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            databuff = malloc(
                (3 as uint32_t)
                    .wrapping_mul(9 as uint32_t)
                    .wrapping_mul(4 as uint32_t)
                    .wrapping_mul(orgroup as uint32_t) as size_t,
            ) as *mut uint8_t;
            if databuff.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/storageclass.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1956 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/storageclass.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1956 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if databuff
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
                    b"/tmp/moosefs-ref/mfsmaster/storageclass.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1956 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/storageclass.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1956 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            hdrleng = (if mver as ::core::ffi::c_int == 0x12 as ::core::ffi::c_int {
                11 as ::core::ffi::c_int
            } else if mver as ::core::ffi::c_int <= 0x13 as ::core::ffi::c_int {
                3 as ::core::ffi::c_int
            } else if mver as ::core::ffi::c_int <= 0x14 as ::core::ffi::c_int {
                5 as ::core::ffi::c_int
            } else if mver as ::core::ffi::c_int <= 0x15 as ::core::ffi::c_int {
                8 as ::core::ffi::c_int
            } else {
                10 as ::core::ffi::c_int
            }) as uint8_t;
            loop {
                if bio_read(
                    fd,
                    databuff as *mut ::core::ffi::c_void,
                    hdrleng as uint64_t,
                ) != hdrleng as int64_t
                {
                    let mut err_7: ::core::ffi::c_int = *__errno_location();
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    *__errno_location() = err_7;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: read error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    databuff = ::core::ptr::null_mut::<uint8_t>();
                    return -1 as ::core::ffi::c_int;
                }
                ptr = databuff;
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
                sclassid = get16bit(&raw mut ptr);
                if mver as ::core::ffi::c_int > 0x15 as ::core::ffi::c_int {
                    nleng = get8bit(&raw mut ptr);
                    admin_only = get8bit(&raw mut ptr);
                    labels_mode = get8bit(&raw mut ptr);
                    arch_delay = get16bit(&raw mut ptr);
                    create.labelscnt = get8bit(&raw mut ptr);
                    keep.labelscnt = get8bit(&raw mut ptr);
                    arch.labelscnt = get8bit(&raw mut ptr);
                    chunkcount = 0 as uint32_t;
                } else if mver as ::core::ffi::c_int > 0x14 as ::core::ffi::c_int {
                    nleng = 0 as uint8_t;
                    admin_only = 0 as uint8_t;
                    labels_mode = get8bit(&raw mut ptr);
                    arch_delay = get16bit(&raw mut ptr);
                    create.labelscnt = get8bit(&raw mut ptr);
                    keep.labelscnt = get8bit(&raw mut ptr);
                    arch.labelscnt = get8bit(&raw mut ptr);
                    chunkcount = 0 as uint32_t;
                } else if mver as ::core::ffi::c_int > 0x13 as ::core::ffi::c_int {
                    nleng = 0 as uint8_t;
                    admin_only = 0 as uint8_t;
                    labels_mode = get8bit(&raw mut ptr);
                    create.labelscnt = get8bit(&raw mut ptr);
                    keep.labelscnt = get8bit(&raw mut ptr);
                    arch.labelscnt = 0 as uint8_t;
                    arch_delay = 0 as uint16_t;
                    chunkcount = 0 as uint32_t;
                } else {
                    nleng = 0 as uint8_t;
                    admin_only = 0 as uint8_t;
                    create.labelscnt = 0 as uint8_t;
                    keep.labelscnt = get8bit(&raw mut ptr);
                    arch.labelscnt = 0 as uint8_t;
                    labels_mode = LABELS_MODE_STD as uint8_t;
                    arch_delay = 0 as uint16_t;
                    if mver as ::core::ffi::c_int == 0x12 as ::core::ffi::c_int {
                        chunkcount = get32bit(&raw mut ptr);
                        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
                    } else {
                        chunkcount = 0 as uint32_t;
                    }
                }
                if nleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    nleng = snprintf(
                        &raw mut name as *mut uint8_t as *mut ::core::ffi::c_char,
                        MAXSCLASSNAMELENG as size_t,
                        b"sclass_%u\0".as_ptr() as *const ::core::ffi::c_char,
                        sclassid as uint32_t,
                    ) as uint8_t;
                } else if bio_read(
                    fd,
                    &raw mut name as *mut uint8_t as *mut ::core::ffi::c_void,
                    nleng as uint64_t,
                ) != nleng as int64_t
                {
                    let mut err_8: ::core::ffi::c_int = *__errno_location();
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    *__errno_location() = err_8;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: read error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    databuff = ::core::ptr::null_mut::<uint8_t>();
                    return -1 as ::core::ffi::c_int;
                }
                if sclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && create.labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && keep.labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && arch.labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && chunkcount == 0 as uint32_t
                    && arch_delay as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    break;
                }
                if create.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                    || keep.labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || keep.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                    || arch.labelscnt as ::core::ffi::c_int > MAXLABELSCNT
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading storage class data: data format error (sclassid: %hu ; labels_mode: %hhu ; create.labelscnt: %hhu ; keep.labelscnt: %hhu ; arch.labelscnt: %hhu ; arch_delay: %hu)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        sclassid as ::core::ffi::c_int,
                        labels_mode as ::core::ffi::c_int,
                        create.labelscnt as ::core::ffi::c_int,
                        keep.labelscnt as ::core::ffi::c_int,
                        arch.labelscnt as ::core::ffi::c_int,
                        arch_delay as ::core::ffi::c_int,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    databuff = ::core::ptr::null_mut::<uint8_t>();
                    return -1 as ::core::ffi::c_int;
                }
                if sclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || sclassid as ::core::ffi::c_int >= MAXSCLASS
                    || nleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading storage class data: bad sclassid (%hu) - ignore\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                        );
                        if mver as ::core::ffi::c_int > 0x14 as ::core::ffi::c_int {
                            bio_skip(
                                fd,
                                ((create.labelscnt as ::core::ffi::c_int
                                    + keep.labelscnt as ::core::ffi::c_int
                                    + arch.labelscnt as ::core::ffi::c_int)
                                    * 4 as ::core::ffi::c_int
                                    * orgroup as ::core::ffi::c_int)
                                    as uint64_t,
                            );
                        } else if mver as ::core::ffi::c_int > 0x13 as ::core::ffi::c_int {
                            bio_skip(
                                fd,
                                ((create.labelscnt as ::core::ffi::c_int
                                    + keep.labelscnt as ::core::ffi::c_int)
                                    * 4 as ::core::ffi::c_int
                                    * orgroup as ::core::ffi::c_int)
                                    as uint64_t,
                            );
                        } else {
                            bio_skip(
                                fd,
                                (keep.labelscnt as ::core::ffi::c_int
                                    * 4 as ::core::ffi::c_int
                                    * orgroup as ::core::ffi::c_int)
                                    as uint64_t,
                            );
                        }
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: bad sclassid (%hu)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            sclassid as ::core::ffi::c_int,
                        );
                        free(databuff as *mut ::core::ffi::c_void);
                        databuff = ::core::ptr::null_mut::<uint8_t>();
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    if mver as ::core::ffi::c_int > 0x14 as ::core::ffi::c_int {
                        if bio_read(
                            fd,
                            databuff as *mut ::core::ffi::c_void,
                            ((create.labelscnt as ::core::ffi::c_int
                                + keep.labelscnt as ::core::ffi::c_int
                                + arch.labelscnt as ::core::ffi::c_int)
                                * 4 as ::core::ffi::c_int
                                * orgroup as ::core::ffi::c_int)
                                as uint64_t,
                        ) != ((create.labelscnt as ::core::ffi::c_int
                            + keep.labelscnt as ::core::ffi::c_int
                            + arch.labelscnt as ::core::ffi::c_int)
                            * 4 as ::core::ffi::c_int
                            * orgroup as ::core::ffi::c_int)
                            as int64_t
                        {
                            let mut err_9: ::core::ffi::c_int = *__errno_location();
                            fputc('\n' as ::core::ffi::c_int, stderr);
                            *__errno_location() = err_9;
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_ERR,
                                b"loading storage class data: read error\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            free(databuff as *mut ::core::ffi::c_void);
                            databuff = ::core::ptr::null_mut::<uint8_t>();
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if mver as ::core::ffi::c_int > 0x13 as ::core::ffi::c_int {
                        if bio_read(
                            fd,
                            databuff as *mut ::core::ffi::c_void,
                            ((create.labelscnt as ::core::ffi::c_int
                                + keep.labelscnt as ::core::ffi::c_int)
                                * 4 as ::core::ffi::c_int
                                * orgroup as ::core::ffi::c_int)
                                as uint64_t,
                        ) != ((create.labelscnt as ::core::ffi::c_int
                            + keep.labelscnt as ::core::ffi::c_int)
                            * 4 as ::core::ffi::c_int
                            * orgroup as ::core::ffi::c_int)
                            as int64_t
                        {
                            let mut err_10: ::core::ffi::c_int = *__errno_location();
                            fputc('\n' as ::core::ffi::c_int, stderr);
                            *__errno_location() = err_10;
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_ERR,
                                b"loading storage class data: read error\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            free(databuff as *mut ::core::ffi::c_void);
                            databuff = ::core::ptr::null_mut::<uint8_t>();
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if bio_read(
                        fd,
                        databuff as *mut ::core::ffi::c_void,
                        (keep.labelscnt as ::core::ffi::c_int
                            * 4 as ::core::ffi::c_int
                            * orgroup as ::core::ffi::c_int) as uint64_t,
                    ) != (keep.labelscnt as ::core::ffi::c_int
                        * 4 as ::core::ffi::c_int
                        * orgroup as ::core::ffi::c_int)
                        as int64_t
                    {
                        let mut err_11: ::core::ffi::c_int = *__errno_location();
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        *__errno_location() = err_11;
                        mfs_log(
                            MFSLOG_ERRNO_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading storage class data: read error\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        free(databuff as *mut ::core::ffi::c_void);
                        databuff = ::core::ptr::null_mut::<uint8_t>();
                        return -1 as ::core::ffi::c_int;
                    }
                    if sclasstab[sclassid as usize].nleng as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                    {
                        if ignoreflag != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_NOTICE,
                                b"loading storage class data: repeated sclassid - ignore\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            if chunkcount > 0 as uint32_t {
                                bio_skip(fd, chunkcount.wrapping_mul(8 as uint32_t) as uint64_t);
                            }
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_ERR,
                                b"loading storage class data: repeated sclassid\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            free(databuff as *mut ::core::ffi::c_void);
                            databuff = ::core::ptr::null_mut::<uint8_t>();
                            return -1 as ::core::ffi::c_int;
                        }
                    } else {
                        ptr = databuff;
                        i = 0 as uint16_t;
                        while (i as ::core::ffi::c_int) < create.labelscnt as ::core::ffi::c_int {
                            j = 0 as uint16_t;
                            while (j as ::core::ffi::c_int) < MASKORGROUP {
                                if (j as ::core::ffi::c_int) < orgroup as ::core::ffi::c_int {
                                    labelmasks[(i as ::core::ffi::c_int * MASKORGROUP
                                        + j as ::core::ffi::c_int)
                                        as usize] = get32bit(&raw mut ptr);
                                } else {
                                    labelmasks[(i as ::core::ffi::c_int * MASKORGROUP
                                        + j as ::core::ffi::c_int)
                                        as usize] = 0 as uint32_t;
                                }
                                j = j.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                        sclass_maskorgroup_to_labelexpr(
                            &raw mut create.labelexpr as *mut [uint8_t; 128],
                            &raw mut labelmasks as *mut uint32_t,
                            create.labelscnt,
                        );
                        i = 0 as uint16_t;
                        while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int {
                            j = 0 as uint16_t;
                            while (j as ::core::ffi::c_int) < MASKORGROUP {
                                if (j as ::core::ffi::c_int) < orgroup as ::core::ffi::c_int {
                                    labelmasks[(i as ::core::ffi::c_int * MASKORGROUP
                                        + j as ::core::ffi::c_int)
                                        as usize] = get32bit(&raw mut ptr);
                                } else {
                                    labelmasks[(i as ::core::ffi::c_int * MASKORGROUP
                                        + j as ::core::ffi::c_int)
                                        as usize] = 0 as uint32_t;
                                }
                                j = j.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                        sclass_maskorgroup_to_labelexpr(
                            &raw mut keep.labelexpr as *mut [uint8_t; 128],
                            &raw mut labelmasks as *mut uint32_t,
                            keep.labelscnt,
                        );
                        i = 0 as uint16_t;
                        while (i as ::core::ffi::c_int) < arch.labelscnt as ::core::ffi::c_int {
                            j = 0 as uint16_t;
                            while (j as ::core::ffi::c_int) < MASKORGROUP {
                                if (j as ::core::ffi::c_int) < orgroup as ::core::ffi::c_int {
                                    labelmasks[(i as ::core::ffi::c_int * MASKORGROUP
                                        + j as ::core::ffi::c_int)
                                        as usize] = get32bit(&raw mut ptr);
                                } else {
                                    labelmasks[(i as ::core::ffi::c_int * MASKORGROUP
                                        + j as ::core::ffi::c_int)
                                        as usize] = 0 as uint32_t;
                                }
                                j = j.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                        sclass_maskorgroup_to_labelexpr(
                            &raw mut arch.labelexpr as *mut [uint8_t; 128],
                            &raw mut labelmasks as *mut uint32_t,
                            arch.labelscnt,
                        );
                        if create.labelscnt as ::core::ffi::c_int
                            == keep.labelscnt as ::core::ffi::c_int
                        {
                            j = 0 as uint16_t;
                            i = 0 as uint16_t;
                            while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int
                                && j as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                if memcmp(
                                    &raw mut *(&raw mut create.labelexpr as *mut [uint8_t; 128])
                                        .offset(i as isize)
                                        as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    &raw mut *(&raw mut keep.labelexpr as *mut [uint8_t; 128])
                                        .offset(i as isize)
                                        as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    SCLASS_EXPR_MAX_SIZE as size_t,
                                ) != 0 as ::core::ffi::c_int
                                {
                                    j = 1 as uint16_t;
                                }
                                i = i.wrapping_add(1);
                            }
                            if j as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                memset(
                                    &raw mut create as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    ::core::mem::size_of::<storagemode>(),
                                );
                            }
                        }
                        if arch.labelscnt as ::core::ffi::c_int
                            == keep.labelscnt as ::core::ffi::c_int
                        {
                            j = 0 as uint16_t;
                            i = 0 as uint16_t;
                            while (i as ::core::ffi::c_int) < keep.labelscnt as ::core::ffi::c_int
                                && j as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                if memcmp(
                                    &raw mut *(&raw mut arch.labelexpr as *mut [uint8_t; 128])
                                        .offset(i as isize)
                                        as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    &raw mut *(&raw mut keep.labelexpr as *mut [uint8_t; 128])
                                        .offset(i as isize)
                                        as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    SCLASS_EXPR_MAX_SIZE as size_t,
                                ) != 0 as ::core::ffi::c_int
                                {
                                    j = 1 as uint16_t;
                                }
                                i = i.wrapping_add(1);
                            }
                            if j as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                memset(
                                    &raw mut arch as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    ::core::mem::size_of::<storagemode>(),
                                );
                            }
                        }
                        sclasstab[sclassid as usize].nleng = nleng;
                        memcpy(
                            &raw mut (*(&raw mut sclasstab as *mut storageclass)
                                .offset(sclassid as isize))
                            .name as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            &raw mut name as *mut uint8_t as *const ::core::ffi::c_void,
                            nleng as size_t,
                        );
                        sclasstab[sclassid as usize].dleng = 0 as uint8_t;
                        sclasstab[sclassid as usize].priority = 0 as uint32_t;
                        if (sclassid as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                            sclasstab[sclassid as usize].export_group = sclassid as uint8_t;
                        } else {
                            sclasstab[sclassid as usize].export_group = 0 as uint8_t;
                        }
                        sclasstab[sclassid as usize].admin_only = admin_only;
                        sclasstab[sclassid as usize].labels_mode = labels_mode;
                        sclasstab[sclassid as usize].arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
                        sclasstab[sclassid as usize].create = create;
                        sclasstab[sclassid as usize].keep = keep;
                        sclasstab[sclassid as usize].arch = arch;
                        sclasstab[sclassid as usize].trash = trash;
                        sclasstab[sclassid as usize].arch_delay = (arch_delay as ::core::ffi::c_int
                            * 24 as ::core::ffi::c_int)
                            as uint16_t;
                        sclasstab[sclassid as usize].min_trashretention = 0 as uint16_t;
                        sclasstab[sclassid as usize].files = 0 as uint32_t;
                        sclasstab[sclassid as usize].directories = 0 as uint32_t;
                        sclass_fix_has_labels_fields(sclassid as uint8_t);
                        if sclassid as uint32_t >= firstneverused {
                            firstneverused = (sclassid as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as uint32_t;
                        }
                        if chunkcount > 0 as uint32_t {
                            bio_skip(fd, chunkcount.wrapping_mul(8 as uint32_t) as uint64_t);
                        }
                    }
                }
            }
        }
        free(databuff as *mut ::core::ffi::c_void);
        databuff = ::core::ptr::null_mut::<uint8_t>();
        return 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_new() {
    unsafe {
        sclasstab[1 as usize].nleng = snprintf(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(1 as isize)).name
                as *mut uint8_t as *mut ::core::ffi::c_char,
            MAXSCLASSNAMELENG as size_t,
            b"2CP\0".as_ptr() as *const ::core::ffi::c_char,
        ) as uint8_t;
        sclasstab[1 as usize].dleng = snprintf(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(1 as isize)).desc
                as *mut uint8_t as *mut ::core::ffi::c_char,
            MAXSCLASSDESCLENG as size_t,
            b"2 copies\0".as_ptr() as *const ::core::ffi::c_char,
        ) as uint8_t;
        sclasstab[1 as usize].keep.labelscnt = 2 as uint8_t;
        sclasstab[2 as usize].nleng = snprintf(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(2 as isize)).name
                as *mut uint8_t as *mut ::core::ffi::c_char,
            MAXSCLASSNAMELENG as size_t,
            b"3CP\0".as_ptr() as *const ::core::ffi::c_char,
        ) as uint8_t;
        sclasstab[2 as usize].dleng = snprintf(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(2 as isize)).desc
                as *mut uint8_t as *mut ::core::ffi::c_char,
            MAXSCLASSDESCLENG as size_t,
            b"3 copies\0".as_ptr() as *const ::core::ffi::c_char,
        ) as uint8_t;
        sclasstab[2 as usize].keep.labelscnt = 3 as uint8_t;
        sclasstab[3 as usize].nleng = snprintf(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(3 as isize)).name
                as *mut uint8_t as *mut ::core::ffi::c_char,
            MAXSCLASSNAMELENG as size_t,
            b"EC4+1\0".as_ptr() as *const ::core::ffi::c_char,
        ) as uint8_t;
        sclasstab[3 as usize].dleng = snprintf(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(3 as isize)).desc
                as *mut uint8_t as *mut ::core::ffi::c_char,
            MAXSCLASSDESCLENG as size_t,
            b"2 copies in KEEP state, 4 data parts plus 1 checksum part in ARCHIVE state\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) as uint8_t;
        sclasstab[3 as usize].arch_delay = 24 as uint16_t;
        sclasstab[3 as usize].keep.labelscnt = 2 as uint8_t;
        sclasstab[3 as usize].arch.labelscnt = 1 as uint8_t;
        sclasstab[3 as usize].arch.ec_data_chksum_parts = 0x41 as uint8_t;
        sclasstab[3 as usize].arch_min_size =
            (512 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as uint64_t;
        sclasstab[4 as usize].nleng = snprintf(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(4 as isize)).name
                as *mut uint8_t as *mut ::core::ffi::c_char,
            MAXSCLASSNAMELENG as size_t,
            b"EC8+1\0".as_ptr() as *const ::core::ffi::c_char,
        ) as uint8_t;
        sclasstab[4 as usize].dleng = snprintf(
            &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(4 as isize)).desc
                as *mut uint8_t as *mut ::core::ffi::c_char,
            MAXSCLASSDESCLENG as size_t,
            b"2 copies in KEEP state, 8 data parts plus 1 checksum part in ARCHIVE state\0".as_ptr()
                as *const ::core::ffi::c_char,
        ) as uint8_t;
        sclasstab[4 as usize].arch_delay = 24 as uint16_t;
        sclasstab[4 as usize].keep.labelscnt = 2 as uint8_t;
        sclasstab[4 as usize].arch.labelscnt = 1 as uint8_t;
        sclasstab[4 as usize].arch.ec_data_chksum_parts = 0x81 as uint8_t;
        sclasstab[4 as usize].arch_min_size =
            (512 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as uint64_t;
        firstneverused = 5 as uint32_t;
        ec_current_version = 2 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_cleanup() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < MAXSCLASS as uint32_t {
            sclasstab[i as usize].nleng = 0 as uint8_t;
            sclasstab[i as usize].dleng = 0 as uint8_t;
            sclasstab[i as usize].name[0 as usize] = 0 as uint8_t;
            sclasstab[i as usize].desc[0 as usize] = 0 as uint8_t;
            sclasstab[i as usize].priority = 0 as uint32_t;
            sclasstab[i as usize].export_group = 0 as uint8_t;
            sclasstab[i as usize].admin_only = 0 as uint8_t;
            sclasstab[i as usize].labels_mode = LABELS_MODE_STD as uint8_t;
            sclasstab[i as usize].arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
            sclasstab[i as usize].arch_delay = 0 as uint16_t;
            sclasstab[i as usize].min_trashretention = 0 as uint16_t;
            sclasstab[i as usize].arch_min_size = 0 as uint64_t;
            sclasstab[i as usize].files = 0 as uint32_t;
            sclasstab[i as usize].directories = 0 as uint32_t;
            memset(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).create
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<storagemode>(),
            );
            memset(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).keep
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<storagemode>(),
            );
            memset(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).arch
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<storagemode>(),
            );
            memset(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).trash
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<storagemode>(),
            );
            sclasstab[i as usize].create.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
            sclasstab[i as usize].keep.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
            sclasstab[i as usize].arch.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
            sclasstab[i as usize].trash.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
            i = i.wrapping_add(1);
        }
        firstneverused = 1 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_reload() {
    unsafe {
        let mut ecmode: uint32_t = 0;
        ecmode = cfg_getuint32(
            b"DEFAULT_EC_DATA_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
            8 as uint32_t,
        );
        if ecmode == 4 as uint32_t || ecmode == 8 as uint32_t {
            DefaultECMODE = ecmode as uint8_t;
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"wrong value for DEFAULT_EC_DATA_PARTS option - only 4 and 8 are currently supported (using: %u)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                DefaultECMODE as ::core::ffi::c_int,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sclass_init() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        MaxECRedundancyLevel = REDUCED_EC_LEVEL as uint8_t;
        sclass_reload();
        i = 0 as uint32_t;
        while i < MAXSCLASS as uint32_t {
            sclasstab[i as usize].nleng = 0 as uint8_t;
            sclasstab[i as usize].dleng = 0 as uint8_t;
            sclasstab[i as usize].name[0 as usize] = 0 as uint8_t;
            sclasstab[i as usize].desc[0 as usize] = 0 as uint8_t;
            sclasstab[i as usize].priority = 0 as uint32_t;
            sclasstab[i as usize].export_group = 0 as uint8_t;
            sclasstab[i as usize].admin_only = 0 as uint8_t;
            sclasstab[i as usize].labels_mode = LABELS_MODE_STD as uint8_t;
            sclasstab[i as usize].arch_mode = SCLASS_ARCH_MODE_CTIME as uint8_t;
            sclasstab[i as usize].arch_delay = 0 as uint16_t;
            sclasstab[i as usize].min_trashretention = 0 as uint16_t;
            sclasstab[i as usize].arch_min_size = 0 as uint64_t;
            sclasstab[i as usize].files = 0 as uint32_t;
            sclasstab[i as usize].directories = 0 as uint32_t;
            memset(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).create
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<storagemode>(),
            );
            memset(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).keep
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<storagemode>(),
            );
            memset(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).arch
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<storagemode>(),
            );
            memset(
                &raw mut (*(&raw mut sclasstab as *mut storageclass).offset(i as isize)).trash
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<storagemode>(),
            );
            sclasstab[i as usize].create.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
            sclasstab[i as usize].keep.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
            sclasstab[i as usize].arch.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
            sclasstab[i as usize].trash.labels_mode = LABELS_MODE_GLOBAL as uint8_t;
            i = i.wrapping_add(1);
        }
        firstneverused = 1 as uint32_t;
        main_reload_register_fname(
            Some(sclass_reload as unsafe extern "C" fn() -> ()),
            b"sclass_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(sclass_fix_matching_servers_fields as unsafe extern "C" fn() -> ()),
            b"sclass_fix_matching_servers_fields\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
