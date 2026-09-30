//! plfsmetalogger entry point — mfsmetalogger/init.h over the shared
//! daemon runtime (`plfscommon::daemon`, port of mfscommon/main.c).

use plfscommon::daemon::{RunFn, Spec, run};
use plfsmetalogger::src::mfsmetalogger::masterconn::masterconn_init;

static RUN_TAB: [(RunFn, &str); 1] = [(masterconn_init, "connection with master")];

fn main() {
    std::process::exit(run(&Spec {
        appname: "mfsmetalogger",
        maxfiles: 4096,
        pthreads: false,
        ionice: false,
        silent_sigchld: false,
        run_tab: &RUN_TAB,
        late_run_tab: &[],
        restore_run_tab: &[],
        module_options: &[],
        module_synopsis: "",
        module_desc: "",
    }))
}
