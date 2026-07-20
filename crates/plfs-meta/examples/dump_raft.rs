//! Dump a meta node's raft log state (last_log parts + row count) from its
//! meta.redb — diagnostic for replication issues.

use plfs_meta::MetaState;
use plfs_meta::raft::RedbRaftStore;

fn main() {
    for dir in std::env::args().skip(1) {
        let path = std::path::PathBuf::from(&dir).join("meta.redb");
        match MetaState::open(&path) {
            Ok(state) => match RedbRaftStore::new(&state) {
                Ok(store) => match store.debug_log_state() {
                    Ok((last, count)) => println!("{dir}: last_log={last:?} raft_log_rows={count}"),
                    Err(e) => println!("{dir}: debug_log_state error: {e}"),
                },
                Err(e) => println!("{dir}: store error: {e}"),
            },
            Err(e) => println!("{dir}: open error: {e}"),
        }
    }
}
