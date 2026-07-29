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
    unsafe fn job_stats(maxjobscnt: *mut uint32_t);
    unsafe fn csserv_stats(bin: *mut uint64_t, bout: *mut uint64_t);
    unsafe fn mainserv_stats(
        bin: *mut uint64_t,
        bout: *mut uint64_t,
        hlopr: *mut uint32_t,
        hlopw: *mut uint32_t,
    );
    unsafe fn masterconn_stats(bin: *mut uint64_t, bout: *mut uint64_t);
    unsafe fn hdd_stats(
        br: *mut uint64_t,
        bw: *mut uint64_t,
        opr: *mut uint32_t,
        opw: *mut uint32_t,
        dbr: *mut uint32_t,
        dbw: *mut uint32_t,
        dopr: *mut uint32_t,
        dopw: *mut uint32_t,
        movl: *mut uint32_t,
        movh: *mut uint32_t,
        rtime: *mut uint64_t,
        wtime: *mut uint64_t,
    );
    unsafe fn hdd_op_stats(
        op_create: *mut uint32_t,
        op_delete: *mut uint32_t,
        op_version: *mut uint32_t,
        op_duplicate: *mut uint32_t,
        op_truncate: *mut uint32_t,
        op_duptrunc: *mut uint32_t,
        op_test: *mut uint32_t,
        op_split: *mut uint32_t,
    );
    unsafe fn hdd_get_chart_data(
        copychunkcount: *mut uint32_t,
        ec4chunkcount: *mut uint32_t,
        ec8chunkcount: *mut uint32_t,
        hddok: *mut uint32_t,
        hddmfr: *mut uint32_t,
        hdddmg: *mut uint32_t,
        usagediff: *mut uint32_t,
    );
    unsafe fn hdd_get_space(
        usedspace: *mut uint64_t,
        totalspace: *mut uint64_t,
        chunkcount: *mut uint32_t,
        tdusedspace: *mut uint64_t,
        tdtotalspace: *mut uint64_t,
        tdchunkcount: *mut uint32_t,
    );
    unsafe fn replicator_stats(bin: *mut uint64_t, bout: *mut uint64_t, repl: *mut uint32_t);
    unsafe fn cpu_init();
    unsafe fn cpu_used(scpu: *mut uint64_t, ucpu: *mut uint64_t);
    unsafe fn mem_used(rss: *mut uint64_t, virt: *mut uint64_t) -> uint8_t;
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
pub const CHARTS_FILENAME: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"csstats.mfs\0") };
pub const CHARTS_UCPU: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHARTS_SCPU: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHARTS_MASTERIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHARTS_MASTEROUT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CHARTS_CSREPIN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHARTS_CSREPOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CHARTS_CSSERVIN: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CHARTS_CSSERVOUT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CHARTS_HDRBYTESR: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CHARTS_HDRBYTESW: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const CHARTS_HDRLLOPR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CHARTS_HDRLLOPW: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const CHARTS_DATABYTESR: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const CHARTS_DATABYTESW: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const CHARTS_DATALLOPR: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const CHARTS_DATALLOPW: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const CHARTS_HLOPR: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const CHARTS_HLOPW: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const CHARTS_RTIME: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const CHARTS_WTIME: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const CHARTS_REPL: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const CHARTS_CREATE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const CHARTS_DELETE: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const CHARTS_VERSION: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const CHARTS_DUPLICATE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const CHARTS_TRUNCATE: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const CHARTS_DUPTRUNC: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const CHARTS_TEST: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const CHARTS_LOAD: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const CHARTS_MEMORY_RSS: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const CHARTS_MEMORY_VIRT: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const CHARTS_MOVELS: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const CHARTS_MOVEHS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const CHARTS_CHANGE: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const CHARTS_SPLIT: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const CHARTS_USPACE: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const CHARTS_TSPACE: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const CHARTS_CHCOUNT: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const CHARTS_TDUSPACE: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const CHARTS_TDTSPACE: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const CHARTS_TDCHCOUNT: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const CHARTS_COPYCHUNKS: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const CHARTS_EC4CHUNKS: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const CHARTS_EC8CHUNKS: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const CHARTS_HDD_OK: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const CHARTS_HDD_MFR: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const CHARTS_HDD_DMG: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const CHARTS_USAGE_DIFF: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const CHARTS: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
static mut calcdefs: [uint32_t; 15] = [
    1000 as uint32_t,
    0 as uint32_t,
    30 as uint32_t,
    29 as uint32_t,
    1002 as uint32_t,
    1004 as uint32_t,
    CHARTS_OP_END as uint32_t,
    1000 as uint32_t,
    0 as uint32_t,
    36 as uint32_t,
    35 as uint32_t,
    1002 as uint32_t,
    1004 as uint32_t,
    CHARTS_OP_END as uint32_t,
    CHARTS_DEFS_END as uint32_t,
];
static mut statdefs: [statdef; 49] = [statdef {
    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    statid: 0,
    mode: 0,
    percent: 0,
    scale: 0,
    multiplier: 0,
    divisor: 0,
}; 49];
static mut estatdefs: [estatdef; 13] = [estatdef {
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
}; 13];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chartsdata_refresh() {
    unsafe {
        let mut data: [uint64_t; 48] = [0; 48];
        let mut bin: uint64_t = 0;
        let mut bout: uint64_t = 0;
        let mut i: uint32_t = 0;
        let mut opr: uint32_t = 0;
        let mut opw: uint32_t = 0;
        let mut dbr: uint32_t = 0;
        let mut dbw: uint32_t = 0;
        let mut dopr: uint32_t = 0;
        let mut dopw: uint32_t = 0;
        let mut movl: uint32_t = 0;
        let mut movh: uint32_t = 0;
        let mut repl: uint32_t = 0;
        let mut op_cr: uint32_t = 0;
        let mut op_de: uint32_t = 0;
        let mut op_ve: uint32_t = 0;
        let mut op_du: uint32_t = 0;
        let mut op_tr: uint32_t = 0;
        let mut op_dt: uint32_t = 0;
        let mut op_te: uint32_t = 0;
        let mut op_sp: uint32_t = 0;
        let mut jobs: uint32_t = 0;
        let mut scpu: uint64_t = 0;
        let mut ucpu: uint64_t = 0;
        let mut rss: uint64_t = 0;
        let mut virt: uint64_t = 0;
        let mut uspace: uint64_t = 0;
        let mut tspace: uint64_t = 0;
        let mut tduspace: uint64_t = 0;
        let mut tdtspace: uint64_t = 0;
        let mut chcount: uint32_t = 0;
        let mut tdchcount: uint32_t = 0;
        let mut copychunks: uint32_t = 0;
        let mut ec4chunks: uint32_t = 0;
        let mut ec8chunks: uint32_t = 0;
        let mut hddok: uint32_t = 0;
        let mut hddmfr: uint32_t = 0;
        let mut hdddmg: uint32_t = 0;
        let mut usagediff: uint32_t = 0;
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
        masterconn_stats(
            (&raw mut data as *mut uint64_t).offset(CHARTS_MASTERIN as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_MASTEROUT as isize),
        );
        job_stats(&raw mut jobs);
        data[CHARTS_LOAD as usize] = jobs as uint64_t;
        csserv_stats(
            (&raw mut data as *mut uint64_t).offset(CHARTS_CSSERVIN as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_CSSERVOUT as isize),
        );
        mainserv_stats(&raw mut bin, &raw mut bout, &raw mut opr, &raw mut opw);
        data[CHARTS_CSSERVIN as usize] = data[CHARTS_CSSERVIN as usize].wrapping_add(bin);
        data[CHARTS_CSSERVOUT as usize] = data[CHARTS_CSSERVOUT as usize].wrapping_add(bout);
        data[CHARTS_HLOPR as usize] = opr as uint64_t;
        data[CHARTS_HLOPW as usize] = opw as uint64_t;
        hdd_stats(
            &raw mut bin,
            &raw mut bout,
            &raw mut opr,
            &raw mut opw,
            &raw mut dbr,
            &raw mut dbw,
            &raw mut dopr,
            &raw mut dopw,
            &raw mut movl,
            &raw mut movh,
            (&raw mut data as *mut uint64_t).offset(CHARTS_RTIME as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_WTIME as isize),
        );
        data[CHARTS_HDRBYTESR as usize] = bin;
        data[CHARTS_HDRBYTESW as usize] = bout;
        data[CHARTS_HDRLLOPR as usize] = opr as uint64_t;
        data[CHARTS_HDRLLOPW as usize] = opw as uint64_t;
        data[CHARTS_DATABYTESR as usize] = dbr as uint64_t;
        data[CHARTS_DATABYTESW as usize] = dbw as uint64_t;
        data[CHARTS_DATALLOPR as usize] = dopr as uint64_t;
        data[CHARTS_DATALLOPW as usize] = dopw as uint64_t;
        data[CHARTS_MOVELS as usize] = movl as uint64_t;
        data[CHARTS_MOVEHS as usize] = movh as uint64_t;
        replicator_stats(
            (&raw mut data as *mut uint64_t).offset(CHARTS_CSREPIN as isize),
            (&raw mut data as *mut uint64_t).offset(CHARTS_CSREPOUT as isize),
            &raw mut repl,
        );
        data[CHARTS_REPL as usize] = repl as uint64_t;
        hdd_op_stats(
            &raw mut op_cr,
            &raw mut op_de,
            &raw mut op_ve,
            &raw mut op_du,
            &raw mut op_tr,
            &raw mut op_dt,
            &raw mut op_te,
            &raw mut op_sp,
        );
        data[CHARTS_CREATE as usize] = op_cr as uint64_t;
        data[CHARTS_DELETE as usize] = op_de as uint64_t;
        data[CHARTS_VERSION as usize] = op_ve as uint64_t;
        data[CHARTS_DUPLICATE as usize] = op_du as uint64_t;
        data[CHARTS_TRUNCATE as usize] = op_tr as uint64_t;
        data[CHARTS_DUPTRUNC as usize] = op_dt as uint64_t;
        data[CHARTS_TEST as usize] = op_te as uint64_t;
        data[CHARTS_SPLIT as usize] = op_sp as uint64_t;
        data[CHARTS_CHANGE as usize] = op_ve
            .wrapping_add(op_du)
            .wrapping_add(op_tr)
            .wrapping_add(op_dt)
            .wrapping_add(op_sp) as uint64_t;
        hdd_get_space(
            &raw mut uspace,
            &raw mut tspace,
            &raw mut chcount,
            &raw mut tduspace,
            &raw mut tdtspace,
            &raw mut tdchcount,
        );
        data[CHARTS_USPACE as usize] = uspace;
        data[CHARTS_TSPACE as usize] = tspace;
        data[CHARTS_CHCOUNT as usize] = chcount as uint64_t;
        data[CHARTS_TDUSPACE as usize] = tduspace;
        data[CHARTS_TDTSPACE as usize] = tdtspace;
        data[CHARTS_TDCHCOUNT as usize] = tdchcount as uint64_t;
        hdd_get_chart_data(
            &raw mut copychunks,
            &raw mut ec4chunks,
            &raw mut ec8chunks,
            &raw mut hddok,
            &raw mut hddmfr,
            &raw mut hdddmg,
            &raw mut usagediff,
        );
        data[CHARTS_COPYCHUNKS as usize] = copychunks as uint64_t;
        data[CHARTS_EC4CHUNKS as usize] = ec4chunks as uint64_t;
        data[CHARTS_EC8CHUNKS as usize] = ec8chunks as uint64_t;
        data[CHARTS_HDD_OK as usize] = hddok as uint64_t;
        data[CHARTS_HDD_MFR as usize] = hddmfr as uint64_t;
        data[CHARTS_HDD_DMG as usize] = hdddmg as uint64_t;
        data[CHARTS_USAGE_DIFF as usize] = usagediff as uint64_t;
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
                name: b"masterin\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"masterout\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"csrepin\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"csrepout\0".as_ptr() as *const ::core::ffi::c_char
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
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"csservin\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"csservout\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"hdrbytesr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"hdrbytesw\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"hdrllopr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"hdrllopw\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"databytesr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"databytesw\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _statdef {
                name: b"datallopr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"datallopw\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"hlopr\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"hlopw\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"rtime\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MICRO as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 60000 as uint16_t,
            },
            _statdef {
                name: b"wtime\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MICRO as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 60000 as uint16_t,
            },
            _statdef {
                name: b"repl\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
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
                name: b"create\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
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
                name: b"version\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('V' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"duplicate\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t)
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
                name: b"truncate\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"duptrunc\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"test\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t)
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
                name: b"load\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('A' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
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
                name: b"movels\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"movehs\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('H' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"change\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('G' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"split\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('S' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('L' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_ADD as uint8_t,
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
                name: b"chunkcount\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"tdusedspace\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"tdtotalspace\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('S' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"tdchunkcount\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('T' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"copychunks\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('P' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('Y' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"ec4chunks\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('4' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"ec8chunks\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('E' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('8' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"hddok\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('K' as ::core::ffi::c_int as uint8_t as uint32_t),
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _statdef {
                name: b"hddmfr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
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
                name: b"hdddmg\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('M' as ::core::ffi::c_int as uint8_t as uint32_t),
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
                name: b"bwin\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('I' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (4 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (6 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _estatdef {
                name: b"bwout\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('U' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (5 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (7 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 8000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _estatdef {
                name: b"hddread\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (8 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (12 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _estatdef {
                name: b"hddwrite\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('B' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (9 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (13 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_MILI as uint8_t,
                multiplier: 1000 as uint16_t,
                divisor: 60 as uint16_t,
            },
            _estatdef {
                name: b"hddopsr\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('R' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (10 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (14 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"hddopsw\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('W' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (11 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (15 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
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
                c2src: (29 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"move\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                statid: ('M' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('O' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('V' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('E' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (31 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (32 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: CHARTS_NONE as uint32_t,
                mode: CHARTS_MODE_ADD as uint8_t,
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
                c2src: (35 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
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
                c1src: (41 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (42 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: (43 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                mode: CHARTS_MODE_MAX as uint8_t,
                percent: 0 as uint8_t,
                scale: CHARTS_SCALE_NONE as uint8_t,
                multiplier: 1 as uint16_t,
                divisor: 1 as uint16_t,
            },
            _estatdef {
                name: b"hddcnt\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
                statid: ('H' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('D' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('C' as ::core::ffi::c_int as uint8_t as uint32_t)
                    .wrapping_mul(256 as uint32_t)
                    .wrapping_add('N' as ::core::ffi::c_int as uint8_t as uint32_t),
                c1src: (44 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c2src: (45 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
                c3src: (46 as ::core::ffi::c_int + CHARTS_DIRECT_START) as uint32_t,
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
