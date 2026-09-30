//! plfsmaster entry point — mfsmaster/init.h and its Makefile flags
//! (MFSMAXFILES=16384) over the shared daemon runtime
//! (`plfscommon::daemon`, port of mfscommon/main.c).

use plfscommon::daemon::{RunFn, Spec, run};
use plfscommon::random::rnd_init;
use plfsmaster::src::mfscommon::globengine::glob_cache_init;
use plfsmaster::src::mfsmaster::{
    bgsaver::bgsaver_init,
    changelog::changelog_init,
    chartsdata::chartsdata_init,
    datacachemgr::dcm_init,
    exports::exports_init,
    matoclserv::matoclserv_init,
    matocsserv::matocsserv_init,
    matomlserv::matomlserv_init,
    metadata::{meta_allowautorestore, meta_incverboselevel, meta_init, meta_restore, meta_setignoreflag},
    missinglog::missing_log_init,
    multilan::multilan_init,
    topology::topology_init,
};

unsafe extern "C" fn rnd_init_c() -> core::ffi::c_int {
    rnd_init()
}
unsafe extern "C" fn glob_cache_init_c() -> core::ffi::c_int {
    glob_cache_init()
}

static RUN_TAB: [(RunFn, &str); 14] = [
    (rnd_init_c, "random generator"),
    (bgsaver_init, "bgsaver"),
    (glob_cache_init_c, "glob engine"),
    (multilan_init, "multilan map"),
    (changelog_init, "change log"),
    (missing_log_init, "missing chunks/files log"), // has to be before 'fs_init'
    (dcm_init, "data cache manager"), // has to be before 'fs_init' and 'matoclserv_init'
    (exports_init, "exports manager"),
    (topology_init, "net topology module"),
    (meta_init, "metadata manager"),
    (chartsdata_init, "charts module"),
    (matomlserv_init, "communication with metalogger"),
    (matocsserv_init, "communication with chunkserver"),
    (matoclserv_init, "communication with clients"),
];
static RESTORE_RUN_TAB: [(RunFn, &str); 2] =
    [(dcm_init, "data cache manager"), (meta_restore, "metadata restore")];
static MODULE_OPTIONS: [(u8, unsafe extern "C" fn()); 3] =
    [(b'i', meta_setignoreflag), (b'a', meta_allowautorestore), (b'x', meta_incverboselevel)];

fn main() {
    std::process::exit(run(&Spec {
        appname: "mfsmaster",
        maxfiles: 16384,
        pthreads: false,
        ionice: false,
        silent_sigchld: false,
        run_tab: &RUN_TAB,
        late_run_tab: &[],
        restore_run_tab: &RESTORE_RUN_TAB,
        module_options: &MODULE_OPTIONS,
        module_synopsis: "[-i] [-a] [-x [-x]] ",
        module_desc: "-i : ignore some metadata structure errors (attach orphans to root, ignore names without inode, etc.). DO NOT USE unless you are absoluttely sure that there are no other options to restore your metadata.\n-a : automatically restore metadata from change logs\n-x : produce more verbose output\n-xx : even more verbose output\n",
    }))
}
