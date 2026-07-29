extern "C" {
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> ::core::ffi::c_int;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> ::core::ffi::c_int;
}
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __itimer_which = ::core::ffi::c_uint;
pub const ITIMER_PROF: __itimer_which = 2;
pub const ITIMER_VIRTUAL: __itimer_which = 1;
pub const ITIMER_REAL: __itimer_which = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
pub type __itimer_which_t = __itimer_which;
pub type __rusage_who = ::core::ffi::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2Rust_Unnamed_12,
    pub c2rust_unnamed_0: C2Rust_Unnamed_11,
    pub c2rust_unnamed_1: C2Rust_Unnamed_10,
    pub c2rust_unnamed_2: C2Rust_Unnamed_9,
    pub c2rust_unnamed_3: C2Rust_Unnamed_8,
    pub c2rust_unnamed_4: C2Rust_Unnamed_7,
    pub c2rust_unnamed_5: C2Rust_Unnamed_6,
    pub c2rust_unnamed_6: C2Rust_Unnamed_5,
    pub c2rust_unnamed_7: C2Rust_Unnamed_4,
    pub c2rust_unnamed_8: C2Rust_Unnamed_3,
    pub c2rust_unnamed_9: C2Rust_Unnamed_2,
    pub c2rust_unnamed_10: C2Rust_Unnamed_1,
    pub c2rust_unnamed_11: C2Rust_Unnamed_0,
    pub c2rust_unnamed_12: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub ru_nivcsw: ::core::ffi::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub ru_nvcsw: ::core::ffi::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub ru_nsignals: ::core::ffi::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub ru_msgrcv: ::core::ffi::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub ru_msgsnd: ::core::ffi::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub ru_oublock: ::core::ffi::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub ru_inblock: ::core::ffi::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub ru_nswap: ::core::ffi::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub ru_majflt: ::core::ffi::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub ru_minflt: ::core::ffi::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub ru_isrss: ::core::ffi::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub ru_idrss: ::core::ffi::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub ru_ixrss: ::core::ffi::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub ru_maxrss: ::core::ffi::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = __rusage_who;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub const MAXITIMER: ::core::ffi::c_int = 999 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut it_set: itimerval = itimerval {
    it_interval: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
    it_value: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
};
static mut addusec: uint32_t = 0;
static mut addsec: uint32_t = 0;
static mut lastautime: uint64_t = 0;
static mut lastastime: uint64_t = 0;
static mut lastget: uint64_t = 0;
#[no_mangle]
pub unsafe extern "C" fn cpu_init() {
    let mut rc: itimerval = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };
    let mut uc: itimerval = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };
    let mut pc: itimerval = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };
    let mut rus: rusage = rusage {
        ru_utime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        c2rust_unnamed: C2Rust_Unnamed_12 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2Rust_Unnamed_11 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2Rust_Unnamed_10 { ru_idrss: 0 },
        c2rust_unnamed_2: C2Rust_Unnamed_9 { ru_isrss: 0 },
        c2rust_unnamed_3: C2Rust_Unnamed_8 { ru_minflt: 0 },
        c2rust_unnamed_4: C2Rust_Unnamed_7 { ru_majflt: 0 },
        c2rust_unnamed_5: C2Rust_Unnamed_6 { ru_nswap: 0 },
        c2rust_unnamed_6: C2Rust_Unnamed_5 { ru_inblock: 0 },
        c2rust_unnamed_7: C2Rust_Unnamed_4 { ru_oublock: 0 },
        c2rust_unnamed_8: C2Rust_Unnamed_3 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2Rust_Unnamed_2 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2Rust_Unnamed_1 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2Rust_Unnamed_0 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2Rust_Unnamed { ru_nivcsw: 0 },
    };
    let mut tod: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&raw mut tod, NULL);
    lastget = tod.tv_sec as uint64_t;
    lastget =
        (lastget as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong) as uint64_t;
    lastget = lastget.wrapping_add(tod.tv_usec as uint64_t);
    addsec = 0 as uint32_t;
    addusec = 0 as uint32_t;
    it_set.it_interval.tv_sec = 0 as __time_t;
    it_set.it_interval.tv_usec = 0 as __suseconds_t;
    it_set.it_value.tv_sec = MAXITIMER as __time_t;
    it_set.it_value.tv_usec = 999999 as __suseconds_t;
    setitimer(ITIMER_REAL, &raw mut it_set, &raw mut rc);
    setitimer(ITIMER_VIRTUAL, &raw mut it_set, &raw mut uc);
    setitimer(ITIMER_PROF, &raw mut it_set, &raw mut pc);
    getrusage(RUSAGE_SELF, &raw mut rus);
    lastautime = rus.ru_utime.tv_sec as uint64_t;
    lastautime = (lastautime as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong)
        as uint64_t;
    lastautime = lastautime.wrapping_add(rus.ru_utime.tv_usec as uint64_t);
    lastastime = rus.ru_stime.tv_sec as uint64_t;
    lastastime = (lastastime as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong)
        as uint64_t;
    lastastime = lastastime.wrapping_add(rus.ru_stime.tv_usec as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn cpu_used(mut scpu: *mut uint64_t, mut ucpu: *mut uint64_t) {
    let mut rc: itimerval = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };
    let mut uc: itimerval = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };
    let mut pc: itimerval = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };
    let mut ucusec: uint64_t = 0;
    let mut pcusec: uint64_t = 0;
    let mut rus: rusage = rusage {
        ru_utime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        c2rust_unnamed: C2Rust_Unnamed_12 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2Rust_Unnamed_11 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2Rust_Unnamed_10 { ru_idrss: 0 },
        c2rust_unnamed_2: C2Rust_Unnamed_9 { ru_isrss: 0 },
        c2rust_unnamed_3: C2Rust_Unnamed_8 { ru_minflt: 0 },
        c2rust_unnamed_4: C2Rust_Unnamed_7 { ru_majflt: 0 },
        c2rust_unnamed_5: C2Rust_Unnamed_6 { ru_nswap: 0 },
        c2rust_unnamed_6: C2Rust_Unnamed_5 { ru_inblock: 0 },
        c2rust_unnamed_7: C2Rust_Unnamed_4 { ru_oublock: 0 },
        c2rust_unnamed_8: C2Rust_Unnamed_3 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2Rust_Unnamed_2 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2Rust_Unnamed_1 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2Rust_Unnamed_0 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2Rust_Unnamed { ru_nivcsw: 0 },
    };
    let mut autime: uint64_t = 0;
    let mut astime: uint64_t = 0;
    let mut rdiff: uint64_t = 0;
    let mut now: uint64_t = 0;
    let mut tod: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut systime: uint64_t = 0;
    let mut usertime: uint64_t = 0;
    gettimeofday(&raw mut tod, NULL);
    now = tod.tv_sec as uint64_t;
    now = (now as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong) as uint64_t;
    now = now.wrapping_add(tod.tv_usec as uint64_t);
    if now > lastget {
        rdiff = now.wrapping_sub(lastget);
        lastget = now;
    } else {
        rdiff = 0 as uint64_t;
    }
    setitimer(ITIMER_REAL, &raw mut it_set, &raw mut rc);
    setitimer(ITIMER_VIRTUAL, &raw mut it_set, &raw mut uc);
    setitimer(ITIMER_PROF, &raw mut it_set, &raw mut pc);
    rc.it_value.tv_sec = MAXITIMER as __time_t - rc.it_value.tv_sec;
    rc.it_value.tv_usec = 999999 as __suseconds_t - rc.it_value.tv_usec;
    uc.it_value.tv_sec = MAXITIMER as __time_t - uc.it_value.tv_sec;
    uc.it_value.tv_usec = 999999 as __suseconds_t - uc.it_value.tv_usec;
    pc.it_value.tv_sec = MAXITIMER as __time_t - pc.it_value.tv_sec;
    pc.it_value.tv_usec = 999999 as __suseconds_t - pc.it_value.tv_usec;
    addsec = (addsec as __time_t + rc.it_value.tv_sec) as uint32_t;
    addusec = (addusec as __suseconds_t + rc.it_value.tv_usec) as uint32_t;
    while addusec > 1000000 as uint32_t {
        addusec = addusec.wrapping_sub(1000000 as uint32_t);
        addsec = addsec.wrapping_add(1);
    }
    if rc.it_value.tv_sec >= 0 as __time_t && rc.it_value.tv_usec >= 0 as __suseconds_t {
        rdiff = rc.it_value.tv_sec as uint64_t;
        rdiff = (rdiff as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong)
            as uint64_t;
        rdiff = rdiff.wrapping_add(rc.it_value.tv_usec as uint64_t);
    }
    if uc.it_value.tv_sec >= 0 as __time_t
        && pc.it_value.tv_sec >= 0 as __time_t
        && uc.it_value.tv_usec >= 0 as __suseconds_t
        && pc.it_value.tv_usec >= 0 as __suseconds_t
    {
        ucusec = uc.it_value.tv_sec as uint64_t;
        ucusec = (ucusec as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong)
            as uint64_t;
        ucusec = ucusec.wrapping_add(uc.it_value.tv_usec as uint64_t);
        pcusec = pc.it_value.tv_sec as uint64_t;
        pcusec = (pcusec as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong)
            as uint64_t;
        pcusec = pcusec.wrapping_add(pc.it_value.tv_usec as uint64_t);
    } else {
        ucusec = 0 as uint64_t;
        pcusec = 0 as uint64_t;
    }
    if pcusec > ucusec {
        pcusec = pcusec.wrapping_sub(ucusec);
    } else {
        pcusec = 0 as uint64_t;
    }
    usertime = ucusec;
    systime = pcusec;
    getrusage(RUSAGE_SELF, &raw mut rus);
    autime = rus.ru_utime.tv_sec as uint64_t;
    autime =
        (autime as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong) as uint64_t;
    autime = autime.wrapping_add(rus.ru_utime.tv_usec as uint64_t);
    astime = rus.ru_stime.tv_sec as uint64_t;
    astime =
        (astime as ::core::ffi::c_ulong).wrapping_mul(1000000 as ::core::ffi::c_ulong) as uint64_t;
    astime = astime.wrapping_add(rus.ru_stime.tv_usec as uint64_t);
    if autime > lastautime {
        usertime = autime.wrapping_sub(lastautime);
        lastautime = autime;
    }
    if astime > lastastime {
        systime = astime.wrapping_sub(lastastime);
        lastastime = astime;
    }
    if rdiff > 0 as uint64_t {
        *scpu = systime
            .wrapping_mul(1000000000 as uint64_t)
            .wrapping_div(rdiff);
        *ucpu = usertime
            .wrapping_mul(1000000000 as uint64_t)
            .wrapping_div(rdiff);
    } else {
        *scpu = 0 as uint64_t;
        *ucpu = 0 as uint64_t;
    };
}
