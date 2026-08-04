#![allow(
    clashing_extern_declarations,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

use core::ffi::{c_char, c_int};

pub type runfn = Option<unsafe extern "C" fn() -> c_int>;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InitEntry {
    pub function: runfn,
    pub name: *mut c_char,
}

const fn c_chars<const N: usize>(bytes: [u8; N]) -> [c_char; N] {
    let mut output = [0; N];
    let mut index = 0;
    while index < N {
        output[index] = bytes[index] as c_char;
        index += 1;
    }
    output
}

#[unsafe(no_mangle)]
pub static mut id: [c_char; 72] =
    c_chars(*b"@(#) version: 4.59.2-1, build: 2106, written by Jakub Kruszona-Zawadzki\0");

#[unsafe(no_mangle)]
pub static mut RunTab: [InitEntry; 2] = [
    InitEntry {
        function: Some(
            plfsmetalogger::src::mfsmetalogger::masterconn::masterconn_init
                as unsafe extern "C" fn() -> c_int,
        ),
        name: c"connection with master".as_ptr() as *mut c_char,
    },
    InitEntry {
        function: None,
        name: c"****".as_ptr() as *mut c_char,
    },
];

#[unsafe(no_mangle)]
pub static mut LateRunTab: [InitEntry; 1] = [InitEntry {
    function: None,
    name: c"****".as_ptr() as *mut c_char,
}];

#[unsafe(no_mangle)]
pub static mut RestoreRunTab: [InitEntry; 1] = [InitEntry {
    function: None,
    name: c"****".as_ptr() as *mut c_char,
}];

const APP_NAME: &str = "mfsmetalogger";
const DATA_PATH: &str = "/usr/local/var/mfs";
const DEFAULT_CONFIG: &str = "/usr/local/etc/mfs/mfsmetalogger.cfg";
const FALLBACK_CONFIG: &str = "/usr/local/etc/mfsmetalogger.cfg";
const INFO_FILE: &str = ".mfsmetalogger_info.txt";
const SILENT_SIGCHLD: bool = false;

#[path = "../../../plfscommon/src/p2_daemon_main.rs"]
mod daemon_main;

fn main() {
    if !daemon_main::configure(daemon_main::DaemonConfig {
        app_name: APP_NAME,
        data_path: DATA_PATH,
        default_config: DEFAULT_CONFIG,
        fallback_config: FALLBACK_CONFIG,
        info_file: INFO_FILE,
        silent_sigchld: SILENT_SIGCHLD,
        init: plfsmetalogger::src::mfsmetalogger::masterconn::masterconn_init,
        init_name: c"connection with master",
    }) {
        std::process::exit(1);
    }
    std::process::exit(daemon_main::run_from_env());
}
