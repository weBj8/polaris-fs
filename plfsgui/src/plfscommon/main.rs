//! plfsgui entry point — mfsgui/init.h over the shared daemon runtime
//! (`plfscommon::daemon`, port of mfscommon/main.c).

use plfscommon::daemon::{RunFn, Spec, run};
use plfsgui::src::mfsgui::mfsgui::mfscgiserv_init;

static RUN_TAB: [(RunFn, &str); 1] = [(mfscgiserv_init, "cgi_server")];

fn main() {
    std::process::exit(run(&Spec {
        appname: "mfsgui",
        maxfiles: 4096,
        pthreads: false,
        ionice: false,
        silent_sigchld: true,
        run_tab: &RUN_TAB,
        late_run_tab: &[],
        restore_run_tab: &[],
        module_options: &[],
        module_synopsis: "",
        module_desc: "",
    }))
}
