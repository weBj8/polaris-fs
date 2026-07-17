//! The MDS worker: owns the `Mds` on a dedicated thread and executes jobs
//! sent by the fuser dispatch threads.
//!
//! Why: `Mds` is `!Send` (the extent store's io_uring ring holds raw
//! pointers), but `fuser::Filesystem` requires `Send + Sync`. Rather than
//! an unsafe wrapper, the MDS is *opened on* its worker thread and never
//! crosses a thread boundary — only Send request/response values do. This
//! is also the exact seam P6 turns into network RPC, and it keeps the
//! single-writer property the MDS assumes.

use std::sync::mpsc::{self, Sender};
use std::thread::JoinHandle;

use porfs_mds::{Mds, MdsError};

/// One unit of work: a closure running against the MDS on the worker thread.
type Job = Box<dyn FnOnce(&mut Mds) + Send>;

/// Handle to the MDS worker thread. Dropping it closes the job channel and
/// joins the thread, so the MDS (redb lock, device file) is fully released
/// before the drop returns — remount-after-unmount is race-free.
pub(crate) struct Worker {
    tx: Option<Sender<Job>>,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    /// Spawn the worker thread and open the MDS on it. `open` runs entirely
    /// on the worker thread, so the `!Send` MDS never moves between threads.
    /// Open errors are relayed back to the caller.
    pub(crate) fn spawn(
        open: impl FnOnce() -> Result<Mds, MdsError> + Send + 'static,
    ) -> Result<Self, MdsError> {
        let (job_tx, job_rx) = mpsc::channel::<Job>();
        let (init_tx, init_rx) = mpsc::channel::<Result<(), MdsError>>();
        let handle = std::thread::spawn(move || {
            let mut mds = match open() {
                Ok(mds) => {
                    if init_tx.send(Ok(())).is_err() {
                        return; // the caller gave up waiting
                    }
                    mds
                }
                Err(err) => {
                    let _ = init_tx.send(Err(err));
                    return;
                }
            };
            for job in job_rx {
                job(&mut mds);
            }
        });
        // A dead channel here means the worker thread panicked before
        // reporting the open outcome.
        init_rx
            .recv()
            .map_err(|_| MdsError::Corrupt("mds worker thread died during open".to_string()))??;
        Ok(Self {
            tx: Some(job_tx),
            handle: Some(handle),
        })
    }

    /// Run `f` against the MDS on the worker thread and return its result.
    /// `None` means the worker is gone (only possible mid-teardown) or died
    /// while executing — the caller maps it to EIO.
    pub(crate) fn call<R: Send + 'static>(
        &self,
        f: impl FnOnce(&mut Mds) -> R + Send + 'static,
    ) -> Option<R> {
        let (reply_tx, reply_rx) = mpsc::channel::<R>();
        let job = Box::new(move |mds: &mut Mds| {
            // If the caller gave up waiting, dropping the result is fine.
            let _ = reply_tx.send(f(mds));
        });
        self.tx.as_ref()?.send(job).ok()?;
        reply_rx.recv().ok()
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        // Closing the channel ends the worker loop; joining makes the MDS
        // teardown (clean-unmount superblock write, redb lock release)
        // synchronous with the filesystem drop.
        self.tx.take();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
