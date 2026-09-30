//! plfschunkserver entry point — mfschunkserver/init.h and its Makefile
//! flags (MFSMAXFILES=16384, _USE_PTHREADS, USE_IONICE) over the shared
//! daemon runtime (`plfscommon::daemon`, port of mfscommon/main.c).

use plfschunkserver::src::mfschunkserver::{
    bgjobs::job_init,
    chartsdata::chartsdata_init,
    csserv::csserv_init,
    hddspacemgr::{hdd_init, hdd_late_init, hdd_restore},
    mainserv::mainserv_init,
    masterconn::masterconn_init,
};
use plfscommon::daemon::{RunFn, Spec, run};
use plfscommon::random::rnd_init;

unsafe extern "C" fn rnd_init_c() -> core::ffi::c_int {
    rnd_init()
}

static RUN_TAB: [(RunFn, &str); 7] = [
    (rnd_init_c, "random generator"),
    (hdd_init, "hdd space manager"),
    (mainserv_init, "main server threads"),
    (job_init, "jobs manager"),
    (csserv_init, "main server acceptor"), // it has to be before "masterconn"
    (masterconn_init, "master connection module"),
    (chartsdata_init, "charts module"),
];
static LATE_RUN_TAB: [(RunFn, &str); 1] = [(hdd_late_init, "hdd space manager - threads")];
static RESTORE_RUN_TAB: [(RunFn, &str); 1] = [(hdd_restore, "hdd space restore")];

fn main() {
    std::process::exit(run(&Spec {
        appname: "mfschunkserver",
        maxfiles: 16384,
        pthreads: true,
        ionice: true,
        silent_sigchld: false,
        run_tab: &RUN_TAB,
        late_run_tab: &LATE_RUN_TAB,
        restore_run_tab: &RESTORE_RUN_TAB,
        module_options: &[],
        module_synopsis: "",
        module_desc: "",
    }))
}
