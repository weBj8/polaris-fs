// Shared daemon lifecycle: process bootstrap, libc polling, signal installation,
// C variadics, and callback invocation form annotated ABI boundary. Registry
// ownership, timer scheduling, event ordering, and termination state live in `imp`.
// allow: SIZE_OK - one include unit preserves daemon-local exported C symbols.

use core::ffi::{c_char, c_double, c_int, c_void};
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub type FILE = libc::FILE;
pub type pollfd = libc::pollfd;
pub type pid_t = libc::pid_t;

const MFSMAXFILES: usize = 4096;
const MFSLOG_SYSLOG: c_int = 0;
const MFSLOG_SYSLOG_STDERR: c_int = 2;
const MFSLOG_ERRNO_SYSLOG_STDERR: c_int = 3;
const MFSLOG_INFO: c_int = 1;
const MFSLOG_NOTICE: c_int = 2;
const MFSLOG_WARNING: c_int = 3;
const MFSLOG_ERR: c_int = 4;

const RM_RESTART: u8 = 0;
const RM_START: u8 = 1;
const RM_STOP: u8 = 2;
const RM_RELOAD: u8 = 3;
const RM_INFO: u8 = 4;
const RM_TEST: u8 = 5;
const RM_KILL: u8 = 6;
const RM_TRY_RESTART: u8 = 7;
const RM_RESTORE: u8 = 8;

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn cfg_load(fname: *const c_char, logundefined: c_int) -> c_int;
    fn cfg_dangerous_options() -> c_int;
    fn cfg_reload() -> c_int;
    fn cfg_info(fd: *mut FILE);
    fn cfg_term();
    fn cfg_getstr(name: *const c_char, def: *const c_char) -> *mut c_char;
    fn cfg_getnum(name: *const c_char, def: c_int) -> c_int;
    fn cfg_getuint32(name: *const c_char, def: u32) -> u32;
    fn cfg_getint32(name: *const c_char, def: i32) -> i32;
    fn cfg_getdouble(name: *const c_char, def: c_double) -> c_double;

    safe fn strerr_init();
    safe fn strerr_term();
    safe fn mycrc32_init();
    safe fn monotonic_method() -> *const c_char;
    safe fn monotonic_speed() -> u32;
    fn processname_init(argc: c_int, argv: *mut *mut c_char);

    fn mfs_log_str_to_pri(value: *const c_char) -> c_int;
    fn mfs_log(mode: c_int, priority: c_int, format: *const c_char, ...);
    safe fn mfs_log_set_min_level(priority: c_int);
    safe fn mfs_log_set_elevate_to(priority: c_int);
    safe fn mfs_log_detach_stderr();
    fn mfs_log_term();
    fn mfs_log_init(identity: *const c_char, daemon: c_int) -> c_int;
}

type VoidCallback = unsafe extern "C" fn();
type GateCallback = unsafe extern "C" fn() -> c_int;
type InfoCallback = unsafe extern "C" fn(*mut FILE);
type PollDescCallback = unsafe extern "C" fn(*mut pollfd, *mut u32);
type PollServeCallback = unsafe extern "C" fn(*mut pollfd);
type ChildCallback = unsafe extern "C" fn(pid_t, c_int);

pub struct DaemonConfig {
    pub app_name: &'static str,
    pub data_path: &'static str,
    pub default_config: &'static str,
    pub fallback_config: &'static str,
    pub info_file: &'static str,
    pub silent_sigchld: bool,
    pub init: unsafe extern "C" fn() -> c_int,
    pub init_name: &'static CStr,
}

static DAEMON_CONFIG: OnceLock<DaemonConfig> = OnceLock::new();

pub fn configure(config: DaemonConfig) -> bool {
    DAEMON_CONFIG.set(config).is_ok()
}

pub fn run_from_env() -> c_int {
    let mut arguments = std::env::args()
        .map(|argument| {
            CString::new(argument)
                .unwrap_or_else(|_| std::process::exit(1))
                .into_bytes_with_nul()
        })
        .collect::<Vec<_>>();
    let mut pointers = arguments
        .iter_mut()
        .map(|argument| argument.as_mut_ptr().cast())
        .collect::<Vec<_>>();
    pointers.push(core::ptr::null_mut());
    // SAFETY: Category 8 (FFI). CString storage outlives `run`; argv is NUL-terminated.
    unsafe { run((pointers.len() - 1) as c_int, pointers.as_mut_ptr()) }
}

fn daemon_config() -> &'static DaemonConfig {
    match DAEMON_CONFIG.get() {
        Some(config) => config,
        None => std::process::abort(),
    }
}

#[derive(Clone, Copy)]
enum DaemonSignal {
    Reload,
    Info,
    Terminate,
    Kill,
}

impl DaemonSignal {
    const fn as_raw(self) -> c_int {
        match self {
            Self::Reload => libc::SIGHUP,
            Self::Info => libc::SIGUSR1,
            Self::Terminate => libc::SIGTERM,
            Self::Kill => libc::SIGKILL,
        }
    }
}

fn send_signal(pid: pid_t, signal: DaemonSignal) -> bool {
    // SAFETY: Category 8 (FFI). Signal enum restricts values to supported daemon controls.
    unsafe { libc::kill(pid, signal.as_raw()) == 0 }
}

fn config_string(name: &CStr, default: &CStr) -> *mut c_char {
    // SAFETY: Category 8 (FFI). CStr arguments guarantee live NUL-terminated inputs.
    unsafe { cfg_getstr(name.as_ptr(), default.as_ptr()) }
}

#[derive(Clone, Copy)]
enum ResourceLimit {
    OpenFiles,
    Core,
    #[cfg(target_os = "linux")]
    MemoryLock,
}

fn set_resource_limit(
    resource: ResourceLimit,
    current: libc::rlim_t,
    maximum: libc::rlim_t,
) -> c_int {
    let resource = match resource {
        ResourceLimit::OpenFiles => libc::RLIMIT_NOFILE,
        ResourceLimit::Core => libc::RLIMIT_CORE,
        #[cfg(target_os = "linux")]
        ResourceLimit::MemoryLock => libc::RLIMIT_MEMLOCK,
    };
    let limit = libc::rlimit {
        rlim_cur: current,
        rlim_max: maximum,
    };
    // SAFETY: Category 8 (FFI). Enum selects a supported resource; rlimit fields are initialized.
    unsafe { libc::setrlimit(resource, &limit) }
}

#[cfg(target_os = "linux")]
fn memory_lock_limit() -> Option<libc::rlimit> {
    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    // SAFETY: Category 8 (FFI). `limit` is initialized writable RLIMIT_MEMLOCK storage.
    if unsafe { libc::getrlimit(libc::RLIMIT_MEMLOCK, &mut limit) } < 0 {
        None
    } else {
        Some(limit)
    }
}

#[cfg(target_os = "linux")]
fn lock_all_memory() -> c_int {
    // SAFETY: Category 8 (FFI). Flags are documented Linux mlockall options.
    unsafe { libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE) }
}

pub mod imp {
    #![deny(unsafe_code)]

    use super::{
        ChildCallback, GateCallback, InfoCallback, PollDescCallback, PollServeCallback,
        VoidCallback,
    };
    use std::sync::{Mutex, MutexGuard};

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub enum ExitState {
        #[default]
        Running,
        WaitingMayExit,
        WaitingCanExit,
        Exited,
    }

    #[derive(Default)]
    pub struct Lifecycle {
        state: ExitState,
    }

    impl Lifecycle {
        pub fn request_termination(&mut self) {
            if self.state == ExitState::Running {
                self.state = ExitState::WaitingMayExit;
            }
        }

        pub fn advance(&mut self, may_exit: bool, can_exit: bool) -> ExitState {
            match self.state {
                ExitState::Running => {}
                ExitState::WaitingMayExit if may_exit => {
                    self.state = ExitState::WaitingCanExit;
                }
                ExitState::WaitingMayExit => {}
                ExitState::WaitingCanExit if can_exit => {
                    self.state = ExitState::Exited;
                }
                ExitState::WaitingCanExit | ExitState::Exited => {}
            }
            self.state
        }

        pub const fn state(&self) -> ExitState {
            self.state
        }
    }

    pub struct TimerSchedule {
        pub next_event: u64,
        pub interval: u64,
        pub offset: u64,
    }

    pub struct Timer {
        pub schedule: Mutex<TimerSchedule>,
        pub callback: Option<VoidCallback>,
    }

    impl Timer {
        pub fn new(now: u64, interval: u64, offset: u64, callback: Option<VoidCallback>) -> Self {
            Self {
                schedule: Mutex::new(TimerSchedule {
                    next_event: next_event(now, interval, offset),
                    interval,
                    offset,
                }),
                callback,
            }
        }

        pub fn change(&self, now: u64, interval: u64, offset: u64) {
            let mut schedule = lock(&self.schedule);
            schedule.next_event = next_event(now, interval, offset);
            schedule.interval = interval;
            schedule.offset = offset;
        }
    }

    pub struct PollRegistration {
        pub desc: Option<PollDescCallback>,
        pub serve: Option<PollServeCallback>,
    }

    pub struct ChildRegistration {
        pub pid: libc::pid_t,
        pub callback: Option<ChildCallback>,
    }

    pub struct Registry {
        pub destruct: Vec<Option<VoidCallback>>,
        pub may_exit: Vec<Option<GateCallback>>,
        pub want_exit: Vec<Option<VoidCallback>>,
        pub can_exit: Vec<Option<GateCallback>>,
        pub reload: Vec<Option<VoidCallback>>,
        pub info: Vec<Option<InfoCallback>>,
        pub keepalive: Vec<Option<VoidCallback>>,
        pub polls: Vec<PollRegistration>,
        pub each_loop: Vec<Option<VoidCallback>>,
        pub children: Vec<ChildRegistration>,
        pub timers: Vec<Box<Timer>>,
    }

    impl Registry {
        pub const fn new() -> Self {
            Self {
                destruct: Vec::new(),
                may_exit: Vec::new(),
                want_exit: Vec::new(),
                can_exit: Vec::new(),
                reload: Vec::new(),
                info: Vec::new(),
                keepalive: Vec::new(),
                polls: Vec::new(),
                each_loop: Vec::new(),
                children: Vec::new(),
                timers: Vec::new(),
            }
        }

        pub fn due_timers(&self, now: u64, previous: u64) -> Vec<VoidCallback> {
            let mut due = Vec::new();
            for timer in self.timers.iter().rev() {
                let mut schedule = lock(&timer.schedule);
                repair_clock_jump(&mut schedule, now, previous);
                let mut count = 0;
                while now >= schedule.next_event && count < 10 {
                    if let Some(callback) = timer.callback {
                        due.push(callback);
                    }
                    schedule.next_event = schedule.next_event.wrapping_add(schedule.interval);
                    count += 1;
                }
                if now >= schedule.next_event {
                    schedule.next_event = next_event_after(now, schedule.interval, schedule.offset);
                }
            }
            due
        }

        pub fn take_children(&mut self, pid: libc::pid_t) -> Vec<ChildCallback> {
            let mut callbacks = Vec::new();
            let mut index = self.children.len();
            while index > 0 {
                index -= 1;
                if self.children[index].pid == pid {
                    if let Some(callback) = self.children.remove(index).callback {
                        callbacks.push(callback);
                    }
                }
            }
            callbacks
        }
    }

    pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
        match mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    pub const fn valid_timer(interval: u64, offset: u64) -> bool {
        interval != 0 && offset < interval
    }

    pub fn handoff_lock_owner(
        owner: libc::pid_t,
        new_owner: libc::pid_t,
        signal: impl FnOnce(libc::pid_t) -> bool,
    ) -> Result<libc::pid_t, ()> {
        if new_owner == owner || signal(new_owner) {
            Ok(new_owner)
        } else {
            Err(())
        }
    }

    fn next_event(now: u64, interval: u64, offset: u64) -> u64 {
        let mut next = (now / interval).wrapping_mul(interval).wrapping_add(offset);
        while next < now {
            next = next.wrapping_add(interval);
        }
        next
    }

    fn next_event_after(now: u64, interval: u64, offset: u64) -> u64 {
        let mut next = (now / interval).wrapping_mul(interval).wrapping_add(offset);
        while now >= next {
            next = next.wrapping_add(interval);
        }
        next
    }

    fn repair_clock_jump(schedule: &mut TimerSchedule, now: u64, previous: u64) {
        if now < previous {
            let remaining = schedule
                .next_event
                .wrapping_sub(previous)
                .min(schedule.interval);
            schedule.next_event = (now / schedule.interval)
                .wrapping_mul(schedule.interval)
                .wrapping_add(schedule.offset);
            while schedule.next_event <= now.wrapping_add(remaining) {
                schedule.next_event = schedule.next_event.wrapping_add(schedule.interval);
            }
        } else if now > previous.wrapping_add(5_000_000) {
            schedule.next_event = next_event_after(now, schedule.interval, schedule.offset);
        }
    }
}

static REGISTRY: Mutex<imp::Registry> = Mutex::new(imp::Registry::new());
static NOW: AtomicU32 = AtomicU32::new(0);
static START_TIME: AtomicU32 = AtomicU32::new(0);
static USEC_NOW: AtomicU64 = AtomicU64::new(0);
static SIGNAL_READ: AtomicI32 = AtomicI32::new(-1);
static SIGNAL_WRITE: AtomicI32 = AtomicI32::new(-1);
static FIRST_RELOAD: AtomicBool = AtomicBool::new(true);
static LOCK_FD: AtomicI32 = AtomicI32::new(-1);
static DEBUG: Mutex<(f64, u64)> = Mutex::new((0.0, 0));

fn registry() -> MutexGuard<'static, imp::Registry> {
    imp::lock(&REGISTRY)
}

fn epoch_duration() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
}

fn refresh_time() -> u64 {
    let duration = epoch_duration();
    let micros = duration.as_secs().wrapping_mul(1_000_000) + u64::from(duration.subsec_micros());
    USEC_NOW.store(micros, Ordering::Relaxed);
    NOW.store(duration.as_secs() as u32, Ordering::Relaxed);
    micros
}

fn reverse_callbacks<T: Copy>(callbacks: &[Option<T>]) -> Vec<T> {
    callbacks
        .iter()
        .rev()
        .filter_map(|callback| *callback)
        .collect()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_destruct_register_fname(
    callback: Option<VoidCallback>,
    _name: *const c_char,
) {
    registry().destruct.push(callback);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_mayexit_register_fname(
    callback: Option<GateCallback>,
    _name: *const c_char,
) {
    registry().may_exit.push(callback);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_wantexit_register_fname(
    callback: Option<VoidCallback>,
    _name: *const c_char,
) {
    registry().want_exit.push(callback);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_canexit_register_fname(
    callback: Option<GateCallback>,
    _name: *const c_char,
) {
    registry().can_exit.push(callback);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_reload_register_fname(
    callback: Option<VoidCallback>,
    _name: *const c_char,
) {
    registry().reload.push(callback);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_info_register_fname(
    callback: Option<InfoCallback>,
    _name: *const c_char,
) {
    registry().info.push(callback);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_keepalive_register_fname(
    callback: Option<VoidCallback>,
    _name: *const c_char,
) {
    registry().keepalive.push(callback);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_poll_register_fname(
    desc: Option<PollDescCallback>,
    serve: Option<PollServeCallback>,
    _desc_name: *const c_char,
    _serve_name: *const c_char,
) {
    registry().polls.push(imp::PollRegistration { desc, serve });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_eachloop_register_fname(
    callback: Option<VoidCallback>,
    _name: *const c_char,
) {
    registry().each_loop.push(callback);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_chld_register_fname(
    pid: pid_t,
    callback: Option<ChildCallback>,
    _name: *const c_char,
) {
    registry()
        .children
        .push(imp::ChildRegistration { pid, callback });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_msectime_register_fname(
    milliseconds: u32,
    offset: u32,
    callback: Option<VoidCallback>,
    _name: *const c_char,
) -> *mut c_void {
    let interval = u64::from(milliseconds).wrapping_mul(1_000);
    let offset = u64::from(offset).wrapping_mul(1_000);
    if !imp::valid_timer(interval, offset) {
        return core::ptr::null_mut();
    }
    let timer = Box::new(imp::Timer::new(
        USEC_NOW.load(Ordering::Relaxed),
        interval,
        offset,
        callback,
    ));
    let handle = (&*timer as *const imp::Timer).cast_mut().cast::<c_void>();
    registry().timers.push(timer);
    handle
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_msectime_change(
    handle: *mut c_void,
    milliseconds: u32,
    offset: u32,
) -> c_int {
    let interval = u64::from(milliseconds).wrapping_mul(1_000);
    let offset = u64::from(offset).wrapping_mul(1_000);
    if handle.is_null() || !imp::valid_timer(interval, offset) {
        return -1;
    }
    // SAFETY: Category 8 (FFI). Handle comes from `main_msectime_register_fname`;
    // registry owns its stable Box until process teardown, matching C contract.
    let timer = unsafe { &*handle.cast::<imp::Timer>() };
    timer.change(USEC_NOW.load(Ordering::Relaxed), interval, offset);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_time_register_fname(
    seconds: u32,
    offset: u32,
    callback: Option<VoidCallback>,
    name: *const c_char,
) -> *mut c_void {
    // SAFETY: Category 8 (FFI). Forwarded callback/name retain original ABI contract.
    unsafe {
        main_msectime_register_fname(
            seconds.wrapping_mul(1_000),
            offset.wrapping_mul(1_000),
            callback,
            name,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_time_change(handle: *mut c_void, seconds: u32, offset: u32) -> c_int {
    // SAFETY: Category 8 (FFI). Forwarded timer handle retains original ABI contract.
    unsafe {
        main_msectime_change(
            handle,
            seconds.wrapping_mul(1_000),
            offset.wrapping_mul(1_000),
        )
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_all_registered_entries() {
    *registry() = imp::Registry::new();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn canexit() -> c_int {
    // SAFETY: Category 8 (FFI). Registration contract supplies live C ABI callbacks.
    unsafe { all_gates(reverse_callbacks(&registry().can_exit)) as c_int }
}

#[unsafe(no_mangle)]
pub extern "C" fn main_time_refresh() -> u32 {
    refresh_time();
    NOW.load(Ordering::Relaxed)
}

#[unsafe(no_mangle)]
pub extern "C" fn main_time() -> u32 {
    NOW.load(Ordering::Relaxed)
}

#[unsafe(no_mangle)]
pub extern "C" fn main_utime() -> u64 {
    let duration = epoch_duration();
    duration.as_secs().wrapping_mul(1_000_000) + u64::from(duration.subsec_micros())
}

#[unsafe(no_mangle)]
pub extern "C" fn main_start_time() -> u32 {
    START_TIME.load(Ordering::Relaxed)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_keep_alive() {
    refresh_time();
    // SAFETY: Category 8 (FFI). Registration contract supplies live C ABI callbacks.
    unsafe { call_void(reverse_callbacks(&registry().keepalive)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_reload() {
    // SAFETY: Category 8 (FFI). Static C strings satisfy config getter ABI.
    let trigger = unsafe { cfg_getdouble(c"LONG_CALL_TRIGGER_MS".as_ptr(), 0.0) } / 1_000.0;
    // SAFETY: Category 8 (FFI). Static C string satisfies config getter ABI.
    let sleep = unsafe { cfg_getuint32(c"DEBUG_LOOP_SLEEP_MS".as_ptr(), 0) } as u64 * 1_000;
    *imp::lock(&DEBUG) = (trigger, sleep);

    // SAFETY: Category 8 (FFI). Returned strings are valid until freed by libc allocator.
    let minimum = config_string(c"SYSLOG_MIN_LEVEL", c"INFO");
    // SAFETY: Category 8 (FFI). Returned strings are valid until freed by libc allocator.
    let elevate = config_string(c"SYSLOG_ELEVATE_TO", c"NOTICE");
    // SAFETY: Category 8 (FFI). cfg returned non-null NUL-terminated values.
    let minimum_value = unsafe { mfs_log_str_to_pri(minimum) };
    // SAFETY: Category 8 (FFI). cfg returned non-null NUL-terminated values.
    let elevate_value = unsafe { mfs_log_str_to_pri(elevate) };
    let first = FIRST_RELOAD.swap(false, Ordering::Relaxed);
    if minimum_value >= 0 {
        mfs_log_set_min_level(minimum_value);
    } else if first {
        log_message(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"error parsing SYSLOG_MIN_LEVEL option - using INFO",
        );
        // SAFETY: Category 8 (FFI). Constant is valid logging priority.
        mfs_log_set_min_level(MFSLOG_INFO);
    } else {
        log_message(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            c"error parsing SYSLOG_MIN_LEVEL option - left unchanged",
        );
    }
    if elevate_value >= 0 {
        mfs_log_set_elevate_to(elevate_value);
    } else if first {
        log_message(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"error parsing SYSLOG_ELEVATE_TO option - using NOTICE",
        );
        // SAFETY: Category 8 (FFI). Constant is valid logging priority.
        mfs_log_set_elevate_to(MFSLOG_NOTICE);
    } else {
        log_message(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            c"error parsing SYSLOG_ELEVATE_TO option - left unchanged",
        );
    }
    // SAFETY: Category 12 (allocation). cfg_getstr transfers malloc ownership to caller once.
    unsafe {
        libc::free(minimum.cast());
        libc::free(elevate.cast());
    }
}

unsafe fn call_void(callbacks: Vec<VoidCallback>) {
    for callback in callbacks {
        // SAFETY: Category 8 (FFI). Registration contract supplies valid C ABI function pointers.
        unsafe { callback() };
    }
}

unsafe fn all_gates(callbacks: Vec<GateCallback>) -> bool {
    for callback in callbacks {
        // SAFETY: Category 8 (FFI). Registration contract supplies valid C ABI function pointers.
        if unsafe { callback() } == 0 {
            return false;
        }
    }
    true
}

unsafe fn handle_reload() {
    // SAFETY: Category 8 (FFI). Config subsystem is initialized for event-loop lifetime.
    unsafe { cfg_reload() };
    // SAFETY: Category 8 (FFI). Same initialized config/logging lifetime.
    unsafe { main_reload() };
    let callbacks = reverse_callbacks(&registry().reload);
    // SAFETY: Callback pointers were registered through C ABI and remain live until teardown.
    unsafe { call_void(callbacks) };
}

fn log_message(mode: c_int, priority: c_int, message: &CStr) {
    // SAFETY: Category 8 (FFI). `message` is NUL-terminated and format has no operands.
    unsafe { mfs_log(mode, priority, message.as_ptr()) };
}

unsafe fn handle_children() {
    loop {
        let mut status = 0;
        // SAFETY: Category 8 (FFI). `status` is valid writable storage for waitpid.
        let pid = unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) };
        if pid <= 0 {
            break;
        }
        let callbacks = registry().take_children(pid);
        for callback in callbacks {
            // SAFETY: Category 8 (FFI). Callback was registered for this pid and ABI.
            unsafe { callback(pid, status) };
        }
    }
}

unsafe fn handle_info() {
    let file_name = CString::new(daemon_config().info_file).unwrap_or_default();
    // SAFETY: Category 8 (FFI). Both C strings are NUL-terminated and live through fclose.
    let file = unsafe { libc::fopen(file_name.as_ptr(), c"w".as_ptr()) };
    if file.is_null() {
        log_message(MFSLOG_SYSLOG, MFSLOG_WARNING, c"can't create info file");
        return;
    }
    // SAFETY: Category 8 (FFI). `file` is a valid writable FILE and formats match arguments.
    unsafe {
        libc::fprintf(
            file,
            c"[general]\nversion: 4.59.2-1\nbuild: 2106\ntimestamp: %ld\n\n".as_ptr(),
            libc::time(core::ptr::null_mut()),
        );
        cfg_info(file);
    }
    let callbacks = reverse_callbacks(&registry().info);
    for callback in callbacks {
        // SAFETY: Category 8 (FFI). FILE remains open; callback ABI accepts it for call duration.
        unsafe { callback(file) };
    }
    // SAFETY: Category 12 (resource ownership). `file` was opened once above and closes once here.
    unsafe { libc::fclose(file) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop() -> c_int {
    let mut descriptors = vec![
        pollfd {
            fd: -1,
            events: 0,
            revents: 0
        };
        MFSMAXFILES
    ];
    let mut previous = 0;
    let mut lifecycle = imp::Lifecycle::default();
    let mut deferred = 0u8;
    let mut status = 0;

    while lifecycle.state() != imp::ExitState::Exited {
        let sleep = imp::lock(&DEBUG).1;
        if sleep != 0 {
            std::thread::sleep(Duration::from_micros(sleep));
        }
        let mut count = 1u32;
        descriptors[0] = pollfd {
            fd: SIGNAL_READ.load(Ordering::Relaxed),
            events: libc::POLLIN,
            revents: 0,
        };
        let polls: Vec<(Option<PollDescCallback>, Option<PollServeCallback>)> = registry()
            .polls
            .iter()
            .rev()
            .map(|entry| (entry.desc, entry.serve))
            .collect();
        for (desc, _) in &polls {
            if let Some(callback) = desc {
                // SAFETY: Category 8 (FFI). 4096 initialized pollfd slots match C MFSMAXFILES contract.
                unsafe { callback(descriptors.as_mut_ptr(), &mut count) };
            }
        }
        if count as usize > descriptors.len() {
            return 1;
        }
        // SAFETY: Category 8 (FFI). Slice owns `count` initialized pollfd entries for poll duration.
        let ready = unsafe { libc::poll(descriptors.as_mut_ptr(), count as libc::nfds_t, 10) };
        let now = refresh_time();
        if ready < 0 {
            let error = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
            if error == libc::EAGAIN {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }
            if error != libc::EINTR {
                break;
            }
        } else if ready > 0 {
            if descriptors[0].revents & libc::POLLIN != 0 {
                let mut signal = 0u8;
                // SAFETY: Category 8 (FFI). Signal pipe fd is open and byte storage is valid.
                if unsafe { libc::read(descriptors[0].fd, (&mut signal as *mut u8).cast(), 1) } == 1
                {
                    match signal {
                        1 => {
                            log_message(MFSLOG_SYSLOG, MFSLOG_NOTICE, c"terminate signal received");
                            lifecycle.request_termination();
                        }
                        2 => {
                            log_message(MFSLOG_SYSLOG, MFSLOG_INFO, c"reloading config files");
                            deferred = 1;
                        }
                        3 => {
                            if !daemon_config().silent_sigchld {
                                log_message(MFSLOG_SYSLOG, MFSLOG_INFO, c"child finished");
                            }
                            deferred = 2;
                        }
                        4 => {
                            log_message(MFSLOG_SYSLOG, MFSLOG_INFO, c"log extra info");
                            deferred = 3;
                        }
                        5 => log_message(
                            MFSLOG_SYSLOG,
                            MFSLOG_NOTICE,
                            c"unexpected alarm/prof signal received - ignoring",
                        ),
                        6 => {
                            log_message(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                c"internal terminate request",
                            );
                            lifecycle.request_termination();
                            status = 1;
                        }
                        _ => {}
                    }
                }
            }
            for (_, serve) in &polls {
                if let Some(callback) = serve {
                    // SAFETY: Category 8 (FFI). Descriptor buffer remains initialized after poll.
                    unsafe { callback(descriptors.as_mut_ptr()) };
                }
            }
        }
        let each_loop = reverse_callbacks(&registry().each_loop);
        // SAFETY: Callback pointers were registered through C ABI and remain live until teardown.
        unsafe { call_void(each_loop) };
        let due = registry().due_timers(now, previous);
        // SAFETY: Timer callback pointers were registered through C ABI and remain live until teardown.
        unsafe { call_void(due) };
        previous = now;

        match deferred {
            1 => unsafe { handle_reload() },
            2 => unsafe { handle_children() },
            3 => unsafe { handle_info() },
            _ => {}
        }
        deferred = 0;

        if lifecycle.state() == imp::ExitState::WaitingMayExit {
            let may_exit = reverse_callbacks(&registry().may_exit);
            // SAFETY: Gate callbacks were registered through C ABI and remain live until teardown.
            if unsafe { all_gates(may_exit) } {
                let want_exit = reverse_callbacks(&registry().want_exit);
                // SAFETY: Callback pointers were registered through C ABI and remain live until teardown.
                unsafe { call_void(want_exit) };
                lifecycle.advance(true, false);
            }
        }
        if lifecycle.state() == imp::ExitState::WaitingCanExit {
            let can_exit = reverse_callbacks(&registry().can_exit);
            // SAFETY: Gate callbacks were registered through C ABI and remain live until teardown.
            lifecycle.advance(true, unsafe { all_gates(can_exit) });
        }
    }
    status
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize() -> c_int {
    let config = daemon_config();
    main_time_refresh();
    // SAFETY: Category 8 (FFI). Daemon root supplies its static init callback and diagnostic name.
    if unsafe { (config.init)() } < 0 {
        // SAFETY: Category 8 (FFI). Static format and configured C string match variadic operands.
        unsafe {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                c"init: %s failed !!!".as_ptr(),
                config.init_name.as_ptr(),
            )
        };
        0
    } else {
        1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore() -> c_int {
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_late() -> c_int {
    let now = main_time_refresh();
    START_TIME.store(now, Ordering::Relaxed);
    1
}

fn write_signal(value: u8) {
    let fd = SIGNAL_WRITE.load(Ordering::Relaxed);
    if fd >= 0 {
        // SAFETY: Category 8 (FFI). Atomic fd is published only after pipe succeeds; one-byte write is async-signal-safe.
        unsafe { libc::write(fd, (&value as *const u8).cast(), 1) };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn termhandle(_signal: c_int) {
    write_signal(1);
}

#[unsafe(no_mangle)]
pub extern "C" fn reloadhandle(_signal: c_int) {
    write_signal(2);
}

#[unsafe(no_mangle)]
pub extern "C" fn chldhandle(_signal: c_int) {
    write_signal(3);
}

#[unsafe(no_mangle)]
pub extern "C" fn infohandle(_signal: c_int) {
    write_signal(4);
}

#[unsafe(no_mangle)]
pub extern "C" fn alarmhandle(_signal: c_int) {
    write_signal(5);
}

unsafe fn install(signals: &[c_int], handler: usize) {
    // SAFETY: Category 4 (initialization). Zero is valid initial state for libc sigaction before fields are assigned.
    let mut action: libc::sigaction = unsafe { core::mem::zeroed() };
    action.sa_flags = libc::SA_RESTART;
    action.sa_sigaction = handler;
    // SAFETY: Category 8 (FFI). `action.sa_mask` is valid writable sigset storage.
    unsafe { libc::sigemptyset(&mut action.sa_mask) };
    for signal in signals {
        // SAFETY: Category 8 (FFI). Handler uses C ABI and lives for process lifetime.
        unsafe { libc::sigaction(*signal, &action, core::ptr::null_mut()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_signal_handlers(daemon: c_int) {
    let mut pipe_fds = [-1; 2];
    // SAFETY: Category 8 (FFI). Two-element array is valid writable storage for pipe.
    if unsafe { libc::pipe(pipe_fds.as_mut_ptr()) } != 0 {
        std::process::abort();
    }
    SIGNAL_READ.store(pipe_fds[0], Ordering::Relaxed);
    SIGNAL_WRITE.store(pipe_fds[1], Ordering::Relaxed);
    // SAFETY: Handlers are extern C functions with static lifetime.
    unsafe {
        install(&[libc::SIGTERM], termhandle as *const () as usize);
        install(&[libc::SIGHUP], reloadhandle as *const () as usize);
        install(&[libc::SIGUSR1], infohandle as *const () as usize);
        install(
            &[libc::SIGALRM, libc::SIGVTALRM, libc::SIGPROF],
            alarmhandle as *const () as usize,
        );
        install(&[libc::SIGCHLD], chldhandle as *const () as usize);
        install(
            &[
                libc::SIGQUIT,
                libc::SIGPIPE,
                libc::SIGTSTP,
                libc::SIGTTIN,
                libc::SIGTTOU,
                libc::SIGUSR2,
            ],
            libc::SIG_IGN,
        );
        install(
            &[libc::SIGINT],
            if daemon != 0 {
                libc::SIG_IGN
            } else {
                termhandle as *const () as usize
            },
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_syslog_wrapper(priority: c_int, message: *const c_char) {
    // SAFETY: Category 8 (FFI). Caller supplies valid NUL-terminated message; `%s` prevents format injection.
    unsafe { libc::syslog(priority, c"%s".as_ptr(), message) };
}

#[unsafe(no_mangle)]
pub extern "C" fn main_exit() {
    write_signal(6);
}

#[unsafe(no_mangle)]
pub extern "C" fn signal_cleanup() {
    for fd in [
        SIGNAL_READ.swap(-1, Ordering::Relaxed),
        SIGNAL_WRITE.swap(-1, Ordering::Relaxed),
    ] {
        if fd >= 0 {
            // SAFETY: Category 12 (resource ownership). Atomic swap gives this call sole close ownership.
            unsafe { libc::close(fd) };
        }
    }
}

// Process/bootstrap boundary: libc account lookup, fd locking, daemonization,
// getopt, and variadic logging require raw OS ABI. Safe lifecycle state never enters here.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn changeugid() {
    // SAFETY: Category 8 (FFI). Config subsystem is initialized and returns malloc-owned C strings.
    if unsafe { libc::geteuid() } != 0 {
        return;
    }
    let user = config_string(c"WORKING_USER", c"nobody");
    let group = config_string(c"WORKING_GROUP", c"");
    let user_bytes = if user.is_null() {
        &[][..]
    } else {
        unsafe { CStr::from_ptr(user) }.to_bytes()
    };
    let group_bytes = if group.is_null() {
        &[][..]
    } else {
        unsafe { CStr::from_ptr(group) }.to_bytes()
    };
    let mut gid = u32::MAX;
    if let Some(number) = group_bytes.strip_prefix(b"#") {
        gid = std::str::from_utf8(number)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(u32::MAX);
    } else if !group_bytes.is_empty() {
        // SAFETY: Category 8 (FFI). `group` is a valid NUL-terminated cfg string.
        let entry = unsafe { libc::getgrnam(group) };
        if entry.is_null() {
            std::process::exit(1);
        }
        // SAFETY: Category 8 (FFI). libc returned non-null group entry.
        gid = unsafe { (*entry).gr_gid };
    }
    let uid;
    if let Some(number) = user_bytes.strip_prefix(b"#") {
        uid = std::str::from_utf8(number)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(u32::MAX);
        if gid == u32::MAX {
            // SAFETY: Category 8 (FFI). libc account database owns returned entry.
            let entry = unsafe { libc::getpwuid(uid) };
            if entry.is_null() {
                std::process::exit(1);
            }
            gid = unsafe { (*entry).pw_gid };
        }
    } else {
        // SAFETY: Category 8 (FFI). `user` is a valid NUL-terminated cfg string.
        let entry = unsafe { libc::getpwnam(user) };
        if entry.is_null() {
            std::process::exit(1);
        }
        uid = unsafe { (*entry).pw_uid };
        if gid == u32::MAX {
            gid = unsafe { (*entry).pw_gid };
        }
    }
    // SAFETY: Category 12 (allocation). Each cfg string is freed exactly once after lookup.
    unsafe {
        libc::free(user.cast());
        libc::free(group.cast());
    }
    // SAFETY: Category 8 (FFI). Resolved uid/gid values are passed directly to POSIX identity calls.
    if unsafe { libc::setgid(gid) } < 0
        || unsafe { libc::setgroups(0, core::ptr::null()) } < 0
        || unsafe { libc::setuid(uid) } < 0
    {
        std::process::exit(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mylock(fd: c_int) -> pid_t {
    let mut lock = libc::flock {
        l_type: libc::F_WRLCK as i16,
        l_whence: libc::SEEK_SET as i16,
        l_start: 0,
        l_len: 0,
        l_pid: unsafe { libc::getpid() },
    };
    loop {
        // SAFETY: Category 8 (FFI). `lock` has libc layout and valid writable lifetime.
        if unsafe { libc::fcntl(fd, libc::F_SETLK, &mut lock) } >= 0 {
            return 0;
        }
        let error = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
        if error != libc::EAGAIN && error != libc::EWOULDBLOCK {
            return -1;
        }
        if unsafe { libc::fcntl(fd, libc::F_GETLK, &mut lock) } < 0 {
            return -1;
        }
        if lock.l_type != libc::F_UNLCK as i16 {
            return lock.l_pid;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wdunlock() {
    let fd = LOCK_FD.swap(-1, Ordering::Relaxed);
    if fd >= 0 {
        // SAFETY: Category 12 (resource ownership). Atomic swap transfers sole close ownership.
        unsafe { libc::close(fd) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wdlock(run_mode: u8, timeout: u32) -> u8 {
    let name = CString::new(format!(".{}.lock", daemon_config().app_name)).unwrap_or_default();
    // SAFETY: Category 8 (FFI). Path is NUL-terminated; mode is supplied for O_CREAT.
    let fd = unsafe { libc::open(name.as_ptr(), libc::O_WRONLY | libc::O_CREAT, 0o666) };
    if fd < 0 {
        return 1;
    }
    LOCK_FD.store(fd, Ordering::Relaxed);
    // SAFETY: Category 8 (FFI). fd is open lock file.
    let mut owner = unsafe { mylock(fd) };
    if owner < 0 {
        return 1;
    }
    if owner > 0 {
        match run_mode {
            RM_TEST => return 0,
            RM_START => return 1,
            RM_RELOAD => return u8::from(!send_signal(owner, DaemonSignal::Reload)),
            RM_INFO => return u8::from(!send_signal(owner, DaemonSignal::Info)),
            _ => {}
        }
        let signal = if run_mode == RM_KILL {
            DaemonSignal::Kill
        } else {
            DaemonSignal::Terminate
        };
        if !send_signal(owner, signal) {
            return 1;
        }
        let mut elapsed = 0;
        loop {
            let new_owner = unsafe { mylock(fd) };
            if new_owner == 0 {
                return 0;
            }
            if new_owner < 0 {
                return 1;
            }
            elapsed += 1;
            if elapsed >= timeout {
                return 1;
            }
            owner = match imp::handoff_lock_owner(owner, new_owner, |pid| send_signal(pid, signal))
            {
                Ok(owner) => owner,
                Err(()) => return 1,
            };
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    match run_mode {
        RM_START | RM_RESTART => {
            let pid = format!("{}\n", unsafe { libc::getpid() });
            // SAFETY: Category 8 (FFI). fd is owned lock file; byte slice is valid for write duration.
            unsafe {
                libc::ftruncate(fd, 0);
                libc::write(fd, pid.as_ptr().cast(), pid.len());
            }
            0
        }
        RM_TRY_RESTART | RM_RELOAD | RM_INFO | RM_TEST => 1,
        RM_STOP | RM_KILL => 0,
        _ => 0,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn makedaemon() {
    let mut channel = [-1; 2];
    // SAFETY: Category 8 (FFI). Two-element array is valid pipe storage.
    if unsafe { libc::pipe(channel.as_mut_ptr()) } < 0 {
        std::process::exit(1);
    }
    // SAFETY: Category 8 (FFI). fork is confined to bootstrap before Rust worker threads start.
    let first = unsafe { libc::fork() };
    if first < 0 {
        std::process::exit(1);
    }
    if first > 0 {
        let mut child_status = 0;
        unsafe { libc::waitpid(first, &mut child_status, 0) };
        if child_status != 0 {
            std::process::exit(1);
        }
        unsafe { libc::close(channel[1]) };
        let mut buffer = [0u8; 1000];
        loop {
            let read = unsafe { libc::read(channel[0], buffer.as_mut_ptr().cast(), buffer.len()) };
            if read <= 0 {
                break;
            }
            if buffer[read as usize - 1] == 0 {
                std::process::exit(1);
            }
            unsafe { libc::write(libc::STDERR_FILENO, buffer.as_ptr().cast(), read as usize) };
        }
        std::process::exit(0);
    }
    unsafe {
        libc::setsid();
        libc::setpgid(0, libc::getpid());
    }
    let second = unsafe { libc::fork() };
    if second != 0 {
        std::process::exit(if second < 0 { 1 } else { 0 });
    }
    unsafe { set_signal_handlers(1) };
    let null = unsafe { libc::open(c"/dev/null".as_ptr(), libc::O_RDWR) };
    unsafe {
        libc::dup2(null, libc::STDIN_FILENO);
        libc::dup2(null, libc::STDOUT_FILENO);
        libc::dup2(channel[1], libc::STDERR_FILENO);
        libc::close(channel[0]);
        libc::close(channel[1]);
        libc::close(null);
        mfs_log_detach_stderr();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn close_msg_channel() {
    // SAFETY: Category 8 (FFI). `/dev/null` path is static and dup2 targets process stderr.
    let null = unsafe { libc::open(c"/dev/null".as_ptr(), libc::O_RDWR) };
    if null >= 0 {
        unsafe {
            libc::dup2(null, libc::STDERR_FILENO);
            libc::close(null);
        }
    }
}

fn emit_daemon_failure_status() {
    // SAFETY: Category 8 (FFI). libc owns the live stderr stream during daemon bootstrap.
    unsafe { libc::fputc(0, stderr) };
    // SAFETY: Category 8 (FFI). Flushing the same live stderr stream publishes the status byte.
    unsafe { libc::fflush(stderr) };
    close_msg_channel();
}

fn notify_daemon_failure(daemon: c_int) {
    if daemon != 0 {
        emit_daemon_failure_status();
    }
}

#[cfg(target_os = "linux")]
fn lock_process_memory() {
    let Some(limit) = memory_lock_limit() else {
        log_message(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"error getting memory lock limits",
        );
        return;
    };
    if limit.rlim_cur != libc::RLIM_INFINITY
        && limit.rlim_max == libc::RLIM_INFINITY
        && set_resource_limit(
            ResourceLimit::MemoryLock,
            libc::RLIM_INFINITY,
            libc::RLIM_INFINITY,
        ) < 0
    {
        log_message(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"error setting memory lock limit to unlimited",
        );
    }
    let Some(limit) = memory_lock_limit() else {
        log_message(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"error getting memory lock limits",
        );
        return;
    };
    if limit.rlim_cur != libc::RLIM_INFINITY {
        log_message(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"can't set memory lock limit to unlimited",
        );
    } else if lock_all_memory() < 0 {
        log_message(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            c"memory lock error",
        );
    } else {
        log_message(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            c"process memory was successfully locked in RAM",
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn createpath(path: *const c_char) {
    if path.is_null() {
        return;
    }
    // SAFETY: Category 8 (FFI). Caller contract supplies NUL-terminated path.
    let bytes = unsafe { CStr::from_ptr(path) }.to_bytes();
    if let Some(parent) = Path::new(OsStr::from_bytes(bytes)).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usage(application: *const c_char) {
    // SAFETY: Category 8 (FFI). Caller supplies argv[0]; format and operand match.
    unsafe {
        libc::printf(
            c"usage: %s [-vhfdun] [-t locktimeout] [-c cfgfile] [start|stop|restart|reload|info|test|kill|restore]\n\n-v : print version number and exit\n-h : print this info and exit\n-f : run in foreground\n-d : run with dangerous options (names: DANGEROUS_*)\n-u : log undefined config variables\n-n : do not attempt to increase limit of core dump size\n-t locktimeout : how long wait for lockfile\n-c cfgfile : use given config file\n".as_ptr(),
            application,
        );
    }
    std::process::exit(1);
}

fn command_mode(command: &[u8]) -> Option<u8> {
    if command.eq_ignore_ascii_case(b"start") {
        Some(RM_START)
    } else if command.eq_ignore_ascii_case(b"stop") {
        Some(RM_STOP)
    } else if command.eq_ignore_ascii_case(b"restart") {
        Some(RM_RESTART)
    } else if command.eq_ignore_ascii_case(b"try-restart") {
        Some(RM_TRY_RESTART)
    } else if command.eq_ignore_ascii_case(b"reload") {
        Some(RM_RELOAD)
    } else if command.eq_ignore_ascii_case(b"info") {
        Some(RM_INFO)
    } else if command.eq_ignore_ascii_case(b"test") || command.eq_ignore_ascii_case(b"status") {
        Some(RM_TEST)
    } else if command.eq_ignore_ascii_case(b"kill") {
        Some(RM_KILL)
    } else if command.eq_ignore_ascii_case(b"restore") {
        Some(RM_RESTORE)
    } else {
        None
    }
}

unsafe fn cleanup(log_identity: *mut c_char) {
    signal_cleanup();
    // SAFETY: Category 8 (FFI). Subsystems were initialized once by run and terminate once here.
    unsafe {
        cfg_term();
        strerr_term();
        wdunlock();
        mfs_log_term();
        libc::free(log_identity.cast());
    }
}

pub unsafe fn run(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // SAFETY: Category 8 (FFI). Process entry supplies conventional argc/argv storage.
    let application = unsafe { *argv };
    strerr_init();
    mycrc32_init();
    let daemon_config = daemon_config();
    let mut config = CString::new(daemon_config.default_config).unwrap_or_default();
    if std::fs::metadata(daemon_config.default_config).is_err()
        && std::fs::metadata(daemon_config.fallback_config).is_ok()
    {
        config = CString::new(daemon_config.fallback_config).unwrap_or_default();
    }
    let mut run_mode = RM_START;
    let mut daemon = 1;
    let mut dangerous = false;
    let mut user_config = false;
    let mut log_undefined = 0;
    let mut force_core = true;
    let mut lock_timeout = 1_800u32;
    // SAFETY: Category 8 (FFI). Resetting getopt state is required for this one process parse.
    unsafe { optind = 1 };
    loop {
        // SAFETY: Category 8 (FFI). argc/argv and static option string obey getopt contract.
        let option = unsafe { libc::getopt(argc, argv, c"nuvfdc:t:h?".as_ptr()) };
        if option == -1 {
            break;
        }
        match option as u8 {
            b'v' => {
                // SAFETY: Category 8 (FFI). Static printf format and operands match.
                unsafe {
                    libc::printf(
                        c"version: %s ; build: %s\n".as_ptr(),
                        c"4.59.2-1".as_ptr(),
                        c"2106".as_ptr(),
                    )
                };
                return 0;
            }
            b'f' => daemon = 0,
            b'd' => dangerous = true,
            b't' => {
                // SAFETY: Category 8 (FFI). getopt sets optarg for option requiring value.
                let value = unsafe { CStr::from_ptr(optarg) }.to_bytes();
                lock_timeout = std::str::from_utf8(value)
                    .ok()
                    .and_then(|text| text.parse().ok())
                    .unwrap_or(0);
            }
            b'c' => {
                // SAFETY: Category 8 (FFI). getopt sets optarg for option requiring value.
                config = unsafe { CStr::from_ptr(optarg) }.to_owned();
                user_config = true;
            }
            b'u' => log_undefined = 1,
            b'n' => force_core = false,
            _ => unsafe { usage(application) },
        }
    }
    let option_index = unsafe { optind };
    let remaining = argc - option_index;
    if remaining == 1 {
        // SAFETY: Category 10 (bounds). optind points at sole remaining argv element.
        let command = unsafe { CStr::from_ptr(*argv.add(option_index as usize)) }.to_bytes();
        run_mode = match command_mode(command) {
            Some(mode) => mode,
            None => {
                unsafe { usage(application) };
                return 1;
            }
        };
    } else if remaining != 0 {
        unsafe { usage(application) };
    }
    // SAFETY: Category 8 (FFI). Config path is owned NUL-terminated string.
    if unsafe { cfg_load(config.as_ptr(), log_undefined) } == 0 && user_config {
        return 1;
    }
    if unsafe { cfg_dangerous_options() } != 0
        && !dangerous
        && matches!(run_mode, RM_RELOAD | RM_START | RM_RESTART | RM_TRY_RESTART)
    {
        return 1;
    }
    if matches!(run_mode, RM_START | RM_RESTART | RM_TRY_RESTART) {
        if daemon != 0 {
            unsafe { makedaemon() };
        } else {
            unsafe { set_signal_handlers(0) };
        }
    }
    unsafe { processname_init(argc, argv) };
    let app_default = CString::new(daemon_config.app_name).unwrap_or_default();
    let log_identity = config_string(c"SYSLOG_IDENT", &app_default);
    unsafe { mfs_log_init(log_identity, daemon) };
    unsafe { main_reload() };

    #[cfg(target_os = "linux")]
    let mut lock_memory = false;
    if matches!(run_mode, RM_START | RM_RESTART | RM_TRY_RESTART) {
        set_resource_limit(
            ResourceLimit::OpenFiles,
            MFSMAXFILES as libc::rlim_t,
            MFSMAXFILES as libc::rlim_t,
        );
        #[cfg(target_os = "linux")]
        {
            // SAFETY: Category 8 (FFI). Static C string satisfies config getter ABI.
            lock_memory = unsafe { cfg_getnum(c"LOCK_MEMORY".as_ptr(), 0) } != 0;
            if lock_memory {
                set_resource_limit(
                    ResourceLimit::MemoryLock,
                    libc::RLIM_INFINITY,
                    libc::RLIM_INFINITY,
                );
            }
        }
        // SAFETY: Category 8 (FFI). Static C string satisfies config getter ABI.
        let nice_level = unsafe { cfg_getint32(c"NICE_LEVEL".as_ptr(), -19) };
        // SAFETY: Category 8 (FFI). Current process id and configured priority are valid inputs.
        unsafe { libc::setpriority(libc::PRIO_PROCESS, libc::getpid() as _, nice_level) };
    }
    unsafe { changeugid() };
    let data_path = CString::new(daemon_config.data_path).unwrap_or_default();
    let workdir = config_string(c"DATA_PATH", &data_path);
    if workdir.is_null() || unsafe { libc::chdir(workdir) } < 0 {
        notify_daemon_failure(daemon);
        unsafe { libc::free(workdir.cast()) };
        unsafe { cleanup(log_identity) };
        return 1;
    }
    unsafe { libc::free(workdir.cast()) };
    unsafe { libc::umask(cfg_getuint32(c"FILE_UMASK".as_ptr(), 0o27) & 0o77) };
    let lock_result = unsafe { wdlock(run_mode, lock_timeout) };
    if lock_result != 0 {
        notify_daemon_failure(daemon);
        unsafe { cleanup(log_identity) };
        return c_int::from(lock_result);
    }
    if run_mode == RM_RESTORE {
        let result = if unsafe { restore() } == 0 { 1 } else { 0 };
        unsafe { cleanup(log_identity) };
        return result;
    }
    if matches!(run_mode, RM_STOP | RM_KILL | RM_RELOAD | RM_INFO | RM_TEST) {
        unsafe { cleanup(log_identity) };
        return 0;
    }
    #[cfg(target_os = "linux")]
    if lock_memory {
        // SAFETY: Category 8 (FFI). Called after config dispatch and wdlock during single-thread bootstrap.
        lock_process_memory();
    }
    if force_core {
        set_resource_limit(
            ResourceLimit::Core,
            libc::RLIM_INFINITY,
            libc::RLIM_INFINITY,
        );
    }
    unsafe {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            c"monotonic clock function: %s".as_ptr(),
            monotonic_method(),
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            c"monotonic clock speed: %u ops / 10 mili seconds".as_ptr(),
            monotonic_speed(),
        );
    }
    let status = if unsafe { initialize() } != 0 {
        if daemon != 0 {
            close_msg_channel();
        }
        if unsafe { initialize_late() } != 0 {
            unsafe { mainloop() }
        } else {
            1
        }
    } else {
        notify_daemon_failure(daemon);
        1
    };
    let destructors = reverse_callbacks(&registry().destruct);
    unsafe { call_void(destructors) };
    free_all_registered_entries();
    unsafe { cleanup(log_identity) };
    status
}

#[cfg(test)]
mod tests {
    use super::imp::{self, ExitState, Lifecycle, Registry, Timer};
    use super::{DaemonSignal, VoidCallback, reverse_callbacks};

    #[test]
    fn callback_stack_is_lifo_and_shutdown_is_two_phase() {
        unsafe extern "C" fn first() {}
        unsafe extern "C" fn second() {}

        let callbacks =
            reverse_callbacks(&[Some(first as VoidCallback), Some(second as VoidCallback)]);
        assert_eq!(callbacks[0] as *const (), second as *const ());
        assert_eq!(callbacks[1] as *const (), first as *const ());

        let mut lifecycle = Lifecycle::default();
        lifecycle.request_termination();
        assert_eq!(lifecycle.advance(false, true), ExitState::WaitingMayExit);
        assert_eq!(lifecycle.advance(true, false), ExitState::WaitingCanExit);
        assert_eq!(lifecycle.advance(true, true), ExitState::Exited);
    }

    #[test]
    fn timer_catch_up_is_capped_and_lifo() {
        unsafe extern "C" fn first() {}
        unsafe extern "C" fn second() {}

        let mut registry = Registry::new();
        registry
            .timers
            .push(Box::new(Timer::new(0, 10, 0, Some(first))));
        registry
            .timers
            .push(Box::new(Timer::new(0, 10, 0, Some(second))));
        let callbacks = registry.due_timers(100, 90);
        assert_eq!(callbacks.len(), 20);
        assert_eq!(callbacks[0] as *const (), second as *const ());
        assert_eq!(callbacks[10] as *const (), first as *const ());
    }

    #[test]
    fn replacement_lock_owner_is_signaled_and_tracked() {
        let mut signaled = Vec::new();

        let owner = imp::handoff_lock_owner(17, 42, |pid| {
            signaled.push(pid);
            true
        });

        assert_eq!(owner, Ok(42));
        assert_eq!(signaled, [42]);
    }

    #[test]
    fn daemon_signal_domain_maps_to_expected_libc_values() {
        assert_eq!(DaemonSignal::Reload.as_raw(), libc::SIGHUP);
        assert_eq!(DaemonSignal::Info.as_raw(), libc::SIGUSR1);
        assert_eq!(DaemonSignal::Terminate.as_raw(), libc::SIGTERM);
        assert_eq!(DaemonSignal::Kill.as_raw(), libc::SIGKILL);
    }
}
