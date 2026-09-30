//! I/O priority helpers — port of mfscommon/ionice.c (Linux ioprio path).
//!
//! Boundary: `ioprio_set` has no libc wrapper and no std equivalent, so it
//! is issued with `syscall(2)`; `setpriority` is a plain libc call. Return
//! values are ignored exactly as in C.

use crate::mfslog::{MFSLOG_INFO, MFSLOG_SYSLOG, log_bytes_errno};

const IOPRIO_CLASS_RT: libc::c_int = 1;
const IOPRIO_CLASS_BE: libc::c_int = 2;
const IOPRIO_CLASS_IDLE: libc::c_int = 3;
const IOPRIO_WHO_PROCESS: libc::c_int = 1;
const IOPRIO_CLASS_SHIFT: libc::c_int = 13;

/// C `IOPRIO_PRIO_VALUE(class, data)`.
const fn prio_value(class: libc::c_int, data: libc::c_int) -> libc::c_int {
    (class << IOPRIO_CLASS_SHIFT) | data
}

fn ioprio_set(which: libc::c_int, who: libc::c_int, ioprio: libc::c_int) {
    // SAFETY: ioprio_set takes three integer arguments and touches no
    // caller memory.
    unsafe { libc::syscall(libc::SYS_ioprio_set, which, who, ioprio) };
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn ionice_test() {
    log_bytes_errno(
        MFSLOG_SYSLOG,
        MFSLOG_INFO,
        b"using linux ioprio (ionice) syscalls for I/O priority",
        0,
    );
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn ionice_high() {
    ioprio_set(IOPRIO_WHO_PROCESS, 0, prio_value(IOPRIO_CLASS_RT, 0));
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn ionice_medium() {
    ioprio_set(IOPRIO_WHO_PROCESS, 0, prio_value(IOPRIO_CLASS_BE, 0));
}

// SAFETY: exported by symbol for C-ABI consumers; body is safe.
#[unsafe(no_mangle)]
pub extern "C" fn ionice_low() {
    // SAFETY: setpriority on the calling process; integer arguments only.
    unsafe { libc::setpriority(libc::PRIO_PROCESS, 0, 19) };
    ioprio_set(IOPRIO_WHO_PROCESS, 0, prio_value(IOPRIO_CLASS_IDLE, 0));
}

#[cfg(test)]
mod tests {
    #[test]
    fn prio_values_match_c_macros() {
        assert_eq!(super::prio_value(1, 0), 0x2000);
        assert_eq!(super::prio_value(2, 0), 0x4000);
        assert_eq!(super::prio_value(3, 0), 0x6000);
    }
}
