unsafe extern "C" {
    unsafe fn charts_add(data: *mut uint64_t, datats: uint32_t);
    unsafe fn charts_store();
    unsafe fn charts_init(
        calcs: *const uint32_t,
        stats: *const statdef,
        estats: *const estatdef,
        filename: *const ::core::ffi::c_char,
        mode: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn charts_term();
    unsafe fn main_destruct_register_fname(
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
    unsafe fn chunk_stats(chunkops: *mut uint32_t);
    unsafe fn chunk_chart_data(
        copychunks: *mut uint64_t,
        ec8chunks: *mut uint64_t,
        ec4chunks: *mut uint64_t,
        regendangered: *mut uint64_t,
        regundergoal: *mut uint64_t,
        allendangered: *mut uint64_t,
        allundergoal: *mut uint64_t,
    );
    unsafe fn fs_stats(stats: *mut uint32_t);
    unsafe fn fs_charts_data(file_objects: *mut uint32_t, meta_objects: *mut uint32_t);
    unsafe fn matoclserv_stats(stats: *mut uint64_t);
    unsafe fn mem_used(rss_0: *mut uint64_t, virt_0: *mut uint64_t) -> uint8_t;
    unsafe fn cpu_init();
    unsafe fn cpu_used(scpu_0: *mut uint64_t, ucpu_0: *mut uint64_t);
    unsafe fn matocsserv_getusagediff() -> uint32_t;
    unsafe fn matocsserv_getspace(
        totalspace: *mut uint64_t,
        availspace: *mut uint64_t,
        freespace: *mut uint64_t,
    );
    unsafe fn csdb_get_server_counters(
        servers_ptr: *mut uint32_t,
        disconnected_servers_ptr: *mut uint32_t,
        disconnected_servers_in_maintenance_ptr: *mut uint32_t,
    );
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _statdef {
    pub name: *mut ::core::ffi::c_char,
    pub statid: uint32_t,
    pub mode: uint8_t,
    pub percent: uint8_t,
    pub scale: uint8_t,
    pub multiplier: uint16_t,
    pub divisor: uint16_t,
}
pub type statdef = _statdef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _estatdef {
    pub name: *mut ::core::ffi::c_char,
    pub statid: uint32_t,
    pub c1src: uint32_t,
    pub c2src: uint32_t,
    pub c3src: uint32_t,
    pub mode: uint8_t,
    pub percent: uint8_t,
    pub scale: uint8_t,
    pub multiplier: uint16_t,
    pub divisor: uint16_t,
}
pub type estatdef = _estatdef;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const CHARTS_MODE_ADD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHARTS_MODE_MAX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHARTS_SCALE_MICRO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHARTS_SCALE_MILI: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHARTS_SCALE_NONE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHARTS_OP_END: ::core::ffi::c_int = 1999 as ::core::ffi::c_int;
pub const CHARTS_DEFS_END: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const CHARTS_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHARTS_DIRECT_START: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHARTS_CALC_START: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const CHARTS_FILENAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"stats.mfs\0") };
pub const CHARTS_UCPU: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHARTS_SCPU: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHARTS_DELCHUNK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHARTS_REPLCHUNK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CHARTS_STATFS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHARTS_MEMORY_RSS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const CHARTS_PACKETSRCVD: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const CHARTS_PACKETSSENT: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const CHARTS_BYTESRCVD: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const CHARTS_BYTESSENT: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const CHARTS_MEMORY_VIRT: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const CHARTS_USED_SPACE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const CHARTS_TOTAL_SPACE: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const CHARTS_CREATECHUNK: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const CHARTS_CHANGECHUNK: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const CHARTS_DELETECHUNK_OK: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const CHARTS_DELETECHUNK_ERR: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const CHARTS_REPLICATECHUNK_OK: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const CHARTS_REPLICATECHUNK_ERR: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const CHARTS_CREATECHUNK_OK: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const CHARTS_CREATECHUNK_ERR: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const CHARTS_CHANGECHUNK_OK: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const CHARTS_CHANGECHUNK_ERR: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const CHARTS_SPLITCHUNK_OK: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const CHARTS_SPLITCHUNK_ERR: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const CHARTS_FILE_OBJECTS: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const CHARTS_META_OBJECTS: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const CHARTS_EC8_CHUNKS: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const CHARTS_EC4_CHUNKS: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const CHARTS_COPY_CHUNKS: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const CHARTS_REG_ENDANGERED: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const CHARTS_REG_UNDERGOAL: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const CHARTS_ALL_ENDANGERED: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const CHARTS_ALL_UNDERGOAL: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const CHARTS_BYTESREAD: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const CHARTS_BYTESWRITE: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const CHARTS_READ: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const CHARTS_WRITE: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const CHARTS_FSYNC: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const CHARTS_LOCK: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const CHARTS_SNAPSHOT: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const CHARTS_DELAY: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const CHARTS_ALL_SERVERS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const CHARTS_MDISC_SERVERS: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const CHARTS_DISC_SERVERS: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const CHARTS_USAGE_DIFF: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const CHARTS_MOUNTS_BYTES_RECEIVED: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const CHARTS_MOUNTS_BYTES_SENT: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
pub const CHARTS: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
pub const CHUNK_OP_DELETE_TRY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHUNK_OP_REPLICATE_TRY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHUNK_OP_CREATE_TRY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHUNK_OP_CHANGE_TRY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CHUNK_OP_SPLIT_TRY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_OP_DELETE_OK: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CHUNK_OP_REPLICATE_OK: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CHUNK_OP_CREATE_OK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CHUNK_OP_CHANGE_OK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CHUNK_OP_SPLIT_OK: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const CHUNK_OP_DELETE_ERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CHUNK_OP_REPLICATE_ERR: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const CHUNK_OP_CREATE_ERR: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const CHUNK_OP_CHANGE_ERR: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const CHUNK_OP_SPLIT_ERR: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
static mut calcdefs: [uint32_t; 29] = [
    1000 as uint32_t,
    0 as uint32_t,
    25 as uint32_t,
    20 as uint32_t,
    1002 as uint32_t,
    1004 as uint32_t,
    CHARTS_OP_END as uint32_t,
    1000 as uint32_t,
    0 as uint32_t,
    27 as uint32_t,
    26 as uint32_t,
    1002 as uint32_t,
    1004 as uint32_t,
    CHARTS_OP_END as uint32_t,
    1000 as uint32_t,
    0 as uint32_t,
    64 as uint32_t,
    66 as uint32_t,
    1002 as uint32_t,
    1004 as uint32_t,
    CHARTS_OP_END as uint32_t,
    1000 as uint32_t,
    0 as uint32_t,
    66 as uint32_t,
    65 as uint32_t,
    1002 as uint32_t,
    1004 as uint32_t,
    CHARTS_OP_END as uint32_t,
    CHARTS_DEFS_END as uint32_t,
];
static mut statdefs: [statdef; 71] = [statdef {
    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    statid: 0,
    mode: 0,
    percent: 0,
    scale: 0,
    multiplier: 0,
    divisor: 0,
}; 71];
static mut estatdefs: [estatdef; 14] = [estatdef {
    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    statid: 0,
    c1src: 0,
    c2src: 0,
    c3src: 0,
    mode: 0,
    percent: 0,
    scale: 0,
    multiplier: 0,
    divisor: 0,
}; 14];
static mut rss: uint64_t = 0;
static mut virt: uint64_t = 0;
static mut scpu: uint64_t = 0;
static mut ucpu: uint64_t = 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chartsdata_resusage(
    mut mem: *mut uint64_t,
    mut syscpu: *mut uint64_t,
    mut usrcpu: *mut uint64_t,
) {
    unsafe {
        *mem = rss;
        *syscpu = scpu;
        *usrcpu = ucpu;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chartsdata_refresh() {
    unsafe {
        let mut data: [uint64_t; 70] = [0; 70];
        let mut fsdata: [uint32_t; 24] = [0; 24];
        let mut fobj: uint32_t = 0;
        let mut mobj: uint32_t = 0;
        let mut cldata: [uint64_t; 12] = [0; 12];
        let mut chunkops: [uint32_t; 15] = [0; 15];
        let mut i: uint32_t = 0;
        let mut total: uint64_t = 0;
        let mut avail: uint64_t = 0;
        let mut servers: uint32_t = 0;
        let mut disc_servers: uint32_t = 0;
        let mut mdisc_servers: uint32_t = 0;
        i = 0 as uint32_t;
        while i < CHARTS as uint32_t {
            data[i as usize] = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
            i = i.wrapping_add(1);
        }
        cpu_used(&raw mut scpu, &raw mut ucpu);
        if scpu > 0 as uint64_t || ucpu > 0 as uint64_t {
            data[CHARTS_UCPU as usize] = ucpu
                .wrapping_mul(6 as uint64_t)
                .wrapping_div(100 as uint64_t);
            data[CHARTS_SCPU as usize] = scpu
                .wrapping_mul(6 as uint64_t)
                .wrapping_div(100 as uint64_t);
        }
        if mem_used(&raw mut rss, &raw mut virt) != 0 {
            data[CHARTS_MEMORY_RSS as usize] = rss;
            data[CHARTS_MEMORY_VIRT as usize] = virt;
        }
        chunk_stats(&raw mut chunkops as *mut uint32_t);
        data[CHARTS_DELCHUNK as usize] = chunkops[CHUNK_OP_DELETE_TRY as usize] as uint64_t;
        data[CHARTS_REPLCHUNK as usize] = chunkops[CHUNK_OP_REPLICATE_TRY as usize] as uint64_t;
        data[CHARTS_CREATECHUNK as usize] = chunkops[CHUNK_OP_CREATE_TRY as usize] as uint64_t;
        data[CHARTS_CHANGECHUNK as usize] = chunkops[CHUNK_OP_CHANGE_TRY as usize]
            .wrapping_add(chunkops[CHUNK_OP_SPLIT_TRY as usize])
            as uint64_t;
        data[CHARTS_DELETECHUNK_OK as usize] = chunkops[CHUNK_OP_DELETE_OK as usize] as uint64_t;
        data[CHARTS_REPLICATECHUNK_OK as usize] =
            chunkops[CHUNK_OP_REPLICATE_OK as usize] as uint64_t;
        data[CHARTS_CREATECHUNK_OK as usize] = chunkops[CHUNK_OP_CREATE_OK as usize] as uint64_t;
        data[CHARTS_CHANGECHUNK_OK as usize] = chunkops[CHUNK_OP_CHANGE_OK as usize] as uint64_t;
        data[CHARTS_SPLITCHUNK_OK as usize] = chunkops[CHUNK_OP_SPLIT_OK as usize] as uint64_t;
        data[CHARTS_DELETECHUNK_ERR as usize] = chunkops[CHUNK_OP_DELETE_ERR as usize] as uint64_t;
        data[CHARTS_REPLICATECHUNK_ERR as usize] =
            chunkops[CHUNK_OP_REPLICATE_ERR as usize] as uint64_t;
        data[CHARTS_CREATECHUNK_ERR as usize] = chunkops[CHUNK_OP_CREATE_ERR as usize] as uint64_t;
        data[CHARTS_CHANGECHUNK_ERR as usize] = chunkops[CHUNK_OP_CHANGE_ERR as usize] as uint64_t;
        data[CHARTS_SPLITCHUNK_ERR as usize] = chunkops[CHUNK_OP_SPLIT_ERR as usize] as uint64_t;
        fs_stats(&raw mut fsdata as *mut uint32_t);
        i = 0 as uint32_t;
        while i < 16 as uint32_t {
            data[(CHARTS_STATFS as uint32_t).wrapping_add(i) as usize] =
                fsdata[i as usize] as uint64_t;
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < 8 as uint32_t {
            data[(CHARTS_SNAPSHOT as uint32_t).wrapping_add(i) as usize] =
                fsdata[(16 as uint32_t).wrapping_add(i) as usize] as uint64_t;
            i = i.wrapping_add(1);
        }
        matoclserv_stats(&raw mut cldata as *mut uint64_t);
        data[CHARTS_PACKETSRCVD as usize] = cldata[0 as usize];
        data[CHARTS_PACKETSSENT as usize] = cldata[1 as usize];
        data[CHARTS_BYTESRCVD as usize] = cldata[2 as usize];
        data[CHARTS_BYTESSENT as usize] = cldata[3 as usize];
        data[CHARTS_BYTESREAD as usize] = cldata[4 as usize];
        data[CHARTS_BYTESWRITE as usize] = cldata[5 as usize];
        data[CHARTS_READ as usize] = cldata[6 as usize];
        data[CHARTS_WRITE as usize] = cldata[7 as usize];
        data[CHARTS_FSYNC as usize] = cldata[8 as usize];
        data[CHARTS_MOUNTS_BYTES_RECEIVED as usize] = cldata[9 as usize];
        data[CHARTS_MOUNTS_BYTES_SENT as usize] = cldata[10 as usize];
        data[CHARTS_LOCK as usize] = cldata[11 as usize];
        matocsserv_getspace(
            &raw mut total,
            &raw mut avail,
            ::core::ptr::null_mut::<uint64_t>(),
        );
        data[CHARTS_USED_SPACE as usize] = total.wrapping_sub(avail);
        data[CHARTS_TOTAL_SPACE as usize] = total;
        fs_charts_data(&raw mut fobj, &raw mut mobj);
        data[CHARTS_FILE_OBJECTS as usize] = fobj as uint64_t;
        data[CHARTS_META_OBJECTS as usize] = mobj as uint64_t;
        chunk_chart_data(
            (&raw mut data as *mut uint64_t).offset(CHARTS_COPY_CHUNKS as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_EC8_CHUNKS as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_EC4_CHUNKS as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_REG_ENDANGERED as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_REG_UNDERGOAL as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_ALL_ENDANGERED as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_ALL_UNDERGOAL as isize),
        );
        data[CHARTS_DELAY as usize] = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
        csdb_get_server_counters(
            &raw mut servers,
            &raw mut disc_servers,
            &raw mut mdisc_servers,
        );
        data[CHARTS_ALL_SERVERS as usize] = servers as uint64_t;
        data[CHARTS_MDISC_SERVERS as usize] = mdisc_servers as uint64_t;
        data[CHARTS_DISC_SERVERS as usize] = disc_servers as uint64_t;
        data[CHARTS_USAGE_DIFF as usize] = matocsserv_getusagediff() as uint64_t;
        charts_add(
            &raw mut data as *mut uint64_t,
            main_time().wrapping_sub(60 as uint32_t),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chartsdata_term() {
    unsafe {
        chartsdata_refresh();
        charts_store();
        charts_term();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chartsdata_store() {
    unsafe {
        charts_store();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chartsdata_init() -> ::core::ffi::c_int {
    unsafe {
        cpu_init();
        ucpu = 0 as uint64_t;
        scpu = ucpu;
        mem_used(&raw mut rss, &raw mut virt);
        main_time_register_fname(
            60 as uint32_t,
            0 as uint32_t,
            Some(chartsdata_refresh as unsafe extern "C" fn() -> ()),
            b"chartsdata_refresh\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            3600 as uint32_t,
            30 as uint32_t,
            Some(chartsdata_store as unsafe extern "C" fn() -> ()),
            b"chartsdata_store\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_destruct_register_fname(
            Some(chartsdata_term as unsafe extern "C" fn() -> ()),
            b"chartsdata_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return charts_init(
            &raw const calcdefs as *const uint32_t,
            &raw const statdefs as *const statdef,
            &raw const estatdefs as *const estatdef,
            CHARTS_FILENAME.as_ptr(),
            0 as uint8_t,
        );
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        statdefs = [
            _statdef {
                name: b"ucpu\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 1 as uint8_t,
                scale: CHARTS_SCALE_MICRO as uint8_t,
                multiplier: 100 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"scpu\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 1 as uint8_t,
                scale: CHARTS_SCALE_MICRO as uint8_t,
                multiplier: 100 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"delete\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"replicate\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"statfs\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('F' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"getattr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('G' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"setattr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"lookup\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"mkdir\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"rmdir\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"symlink\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"readlink\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"mknod\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"unlink\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"rename\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"link\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"readdir\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"open\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"readchunk\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"writechunk\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('W' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"memoryrss\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"prcvd\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"psent\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"brcvd\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"bsent\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"memoryvirt\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"usedspace\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"totalspace\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"create\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"change\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"delete_ok\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"delete_err\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"replicate_ok\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"replicate_err\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"create_ok\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"create_err\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"change_ok\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"change_err\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"split_ok\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"split_err\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"fileobjects\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('F' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"metaobjects\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"chunksec8\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('8' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"chunksec4\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('4' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"chunkscopy\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('Y' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"chregdanger\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"chregunder\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"challdanger\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"challunder\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"bytesread\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('Y' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"byteswrite\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('Y' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"read\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"write\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('W' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"fsync\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('F' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"lock\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"snapshot\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"truncate\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"getxattr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('G' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('X' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"setxattr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('X' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"getfacl\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('G' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('F' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"setfacl\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('F' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"fcreate\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"meta\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"delay\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('Y' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"servers\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"mdservers\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"dservers\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"udiff\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('F' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"mountbytrcvd\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('Y' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"mountbytsent\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('Y' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                statid: 0 as uint32_t,
                mode: 0 as uint8_t,
                percent: 0 as uint8_t,
                scale: 0 as uint8_t,
                multiplier: 0 as uint16_t,
                divisor: 0 as uint16_t,
            },
        ];
        estatdefs = [
            _estatdef {
                name: b"cpu\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (0 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (1 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 1 as uint8_t,
                scale: CHARTS_SCALE_MICRO as uint8_t,
                multiplier: 100 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _estatdef {
                name: b"mem\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('M' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (0 as ::core::ffi::c_int + CHARTS_CALC_START) as uint32_t,
                c2src: (20 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"space\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (1 as ::core::ffi::c_int + CHARTS_CALC_START) as uint32_t,
                c2src: (26 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"delete_stat\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (30 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (31 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"replicate_stat\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (32 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (33 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"create_stat\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (34 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (35 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"change_stat\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (36 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (37 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"split_stat\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (38 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (39 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"objects\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('J' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (40 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (41 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"chunks\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (44 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (43 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: (42 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"regunder\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (46 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (45 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"allunder\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (48 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (47 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"cservers\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (2 as ::core::ffi::c_int + CHARTS_CALC_START) as uint32_t,
                c2src: (65 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: (3 as ::core::ffi::c_int + CHARTS_CALC_START) as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                statid: 0 as uint32_t,
                c1src: CHARTS_NONE as uint32_t,
                c2src: CHARTS_NONE as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: 0 as uint8_t,
                percent: 0 as uint8_t,
                scale: 0 as uint8_t,
                multiplier: 0 as uint16_t,
                divisor: 0 as uint16_t,
            },
        ];
    }
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XIB"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
