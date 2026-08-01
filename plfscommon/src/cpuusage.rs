//! CPU usage sampling (getrusage + itimer deltas), migrated to safe Rust (P1).
//! The c2rust union soup is replaced by libc's plain `rusage`/`itimerval`;
//! mutable globals moved into one Mutex-protected state (called at most once
//! per second from daemon main loops — contention is a non-issue).
//!
//! Semantics preserved, including the original's itimer bookkeeping.

pub type uint32_t = u32;
pub type uint64_t = u64;
pub const MAXITIMER: ::core::ffi::c_long = 999;

use libc::{getrusage, gettimeofday, itimerval, rusage, setitimer, timeval};
use std::sync::Mutex;

struct CpuState {
    it_set: itimerval,
    addusec: uint32_t,
    addsec: uint32_t,
    lastautime: uint64_t,
    lastastime: uint64_t,
    lastget: uint64_t,
}

const ZERO_TV: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};

static STATE: Mutex<CpuState> = Mutex::new(CpuState {
    it_set: itimerval {
        it_interval: ZERO_TV,
        it_value: ZERO_TV,
    },
    addusec: 0,
    addsec: 0,
    lastautime: 0,
    lastastime: 0,
    lastget: 0,
});

#[deny(unsafe_code)]
mod imp {
    use super::*;

    fn zero_itimerval() -> itimerval {
        itimerval {
            it_interval: ZERO_TV,
            it_value: ZERO_TV,
        }
    }

    #[allow(unsafe_code)]
    pub fn now_usec() -> uint64_t {
        let mut tod = ZERO_TV;
        // SAFETY: tod is valid; null tz (as the original).
        unsafe { gettimeofday(&mut tod, std::ptr::null_mut()) };
        (tod.tv_sec as uint64_t)
            .wrapping_mul(1_000_000)
            .wrapping_add(tod.tv_usec as uint64_t)
    }

    /// current (rc, uc, pc) itimer remainders after re-arming to it_set
    #[allow(unsafe_code)]
    pub fn rearm(it_set: &itimerval) -> (itimerval, itimerval, itimerval) {
        let mut rc = zero_itimerval();
        let mut uc = zero_itimerval();
        let mut pc = zero_itimerval();
        // SAFETY: all pointers valid; it_set lives across the calls.
        unsafe {
            setitimer(libc::ITIMER_REAL, it_set, &mut rc);
            setitimer(libc::ITIMER_VIRTUAL, it_set, &mut uc);
            setitimer(libc::ITIMER_PROF, it_set, &mut pc);
        }
        (rc, uc, pc)
    }

    /// (utime_usec, stime_usec) for RUSAGE_SELF
    #[allow(unsafe_code)]
    pub fn rusage_usec() -> (uint64_t, uint64_t) {
        // libc's rusage is all-public-fields; zero value is a valid rusage.
        let mut rus: rusage = unsafe { std::mem::zeroed() };
        // SAFETY: rus valid for write.
        unsafe { getrusage(libc::RUSAGE_SELF, &mut rus) };
        (
            (rus.ru_utime.tv_sec as uint64_t)
                .wrapping_mul(1_000_000)
                .wrapping_add(rus.ru_utime.tv_usec as uint64_t),
            (rus.ru_stime.tv_sec as uint64_t)
                .wrapping_mul(1_000_000)
                .wrapping_add(rus.ru_stime.tv_usec as uint64_t),
        )
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cpu_init() {
    let mut st = STATE.lock().unwrap();
    st.lastget = imp::now_usec();
    st.addsec = 0;
    st.addusec = 0;
    st.it_set.it_interval.tv_sec = 0;
    st.it_set.it_interval.tv_usec = 0;
    st.it_set.it_value.tv_sec = MAXITIMER;
    st.it_set.it_value.tv_usec = 999999;
    let _ = imp::rearm(&st.it_set);
    let (u, s) = imp::rusage_usec();
    st.lastautime = u;
    st.lastastime = s;
}

/// # Safety
/// `scpu` and `ucpu` must be valid writable `u64` pointers (C caller contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_used(mut scpu: *mut uint64_t, mut ucpu: *mut uint64_t) {
    let mut st = STATE.lock().unwrap();
    let now = imp::now_usec();
    let mut rdiff: uint64_t;
    if now > st.lastget {
        rdiff = now.wrapping_sub(st.lastget);
        st.lastget = now;
    } else {
        rdiff = 0;
    }
    let (mut rc, mut uc, mut pc) = imp::rearm(&st.it_set);
    rc.it_value.tv_sec = MAXITIMER - rc.it_value.tv_sec;
    rc.it_value.tv_usec = 999999 - rc.it_value.tv_usec;
    uc.it_value.tv_sec = MAXITIMER - uc.it_value.tv_sec;
    uc.it_value.tv_usec = 999999 - uc.it_value.tv_usec;
    pc.it_value.tv_sec = MAXITIMER - pc.it_value.tv_sec;
    pc.it_value.tv_usec = 999999 - pc.it_value.tv_usec;
    st.addsec = (st.addsec as ::core::ffi::c_long + rc.it_value.tv_sec) as uint32_t;
    st.addusec = (st.addusec as ::core::ffi::c_long + rc.it_value.tv_usec) as uint32_t;
    while st.addusec > 1_000_000 {
        st.addusec = st.addusec.wrapping_sub(1_000_000);
        st.addsec = st.addsec.wrapping_add(1);
    }
    if rc.it_value.tv_sec >= 0 && rc.it_value.tv_usec >= 0 {
        rdiff = (rc.it_value.tv_sec as uint64_t)
            .wrapping_mul(1_000_000)
            .wrapping_add(rc.it_value.tv_usec as uint64_t);
    }
    let ucusec: uint64_t;
    let pcusec0: uint64_t;
    if uc.it_value.tv_sec >= 0
        && pc.it_value.tv_sec >= 0
        && uc.it_value.tv_usec >= 0
        && pc.it_value.tv_usec >= 0
    {
        ucusec = (uc.it_value.tv_sec as uint64_t)
            .wrapping_mul(1_000_000)
            .wrapping_add(uc.it_value.tv_usec as uint64_t);
        pcusec0 = (pc.it_value.tv_sec as uint64_t)
            .wrapping_mul(1_000_000)
            .wrapping_add(pc.it_value.tv_usec as uint64_t);
    } else {
        ucusec = 0;
        pcusec0 = 0;
    }
    let pcusec = if pcusec0 > ucusec {
        pcusec0.wrapping_sub(ucusec)
    } else {
        0
    };
    let mut usertime = ucusec;
    let mut systime = pcusec;
    let (autime, astime) = imp::rusage_usec();
    if autime > st.lastautime {
        usertime = autime.wrapping_sub(st.lastautime);
        st.lastautime = autime;
    }
    if astime > st.lastastime {
        systime = astime.wrapping_sub(st.lastastime);
        st.lastastime = astime;
    }
    let (s, u) = if rdiff > 0 {
        (
            systime.wrapping_mul(1_000_000_000).wrapping_div(rdiff),
            usertime.wrapping_mul(1_000_000_000).wrapping_div(rdiff),
        )
    } else {
        (0, 0)
    };
    // SAFETY: per fn contract.
    unsafe {
        *scpu = s;
        *ucpu = u;
    }
}
