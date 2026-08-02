//! Rust-owned directory read buffers.
//!
//! Handles are opaque, never-reissued `u64` tokens. Registry lookups clone an
//! `Arc`, so release cannot recycle storage beneath an in-flight read. Slots
//! own credentials, supplementary groups, response bytes, and cache handles;
//! this also closes the C implementation's supplementary-group leak.

#![deny(unsafe_code)]

use crate::src::fuse_client::dirattrcache;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock};

pub const EDGEID_MAX: u64 = 0x7fff_ffff_ffff_ffff;
pub const MFS_ERROR_EACCES: u8 = 4;
pub const MFS_ERROR_IO: u8 = 22;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credentials {
    pub pid: i32,
    pub uid: u32,
    pub gid: u32,
    pub groups: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataFormat {
    Empty,
    Basic,
    Attrs,
}

impl DataFormat {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Empty | Self::Basic => 0,
            Self::Attrs => 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FetchRequest {
    pub inode: u32,
    pub credentials: Credentials,
    pub edge: u64,
    pub format: DataFormat,
}

#[derive(Debug)]
pub struct FetchReply {
    pub status: u8,
    pub edge: u64,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub bytes: Arc<[u8]>,
    pub start: usize,
    pub format: DataFormat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Record<'a> {
    pub name: &'a [u8],
    pub inode: u32,
    pub data: &'a [u8],
    pub len: usize,
}

impl Record<'_> {
    pub fn next_offset(self, off: i64) -> i64 {
        off.wrapping_add(self.len as i64)
    }
}

pub fn parse_record(bytes: &[u8], format: DataFormat, attr_size: usize) -> Option<Record<'_>> {
    let &name_len = bytes.first()?;
    let data_len = match format {
        DataFormat::Basic => 1,
        DataFormat::Attrs => attr_size,
        DataFormat::Empty => return None,
    };
    let inode_at = 1usize.wrapping_add(name_len as usize);
    let record_len = inode_at.wrapping_add(4).wrapping_add(data_len);
    if bytes.len() < record_len {
        return None;
    }
    Some(Record {
        name: &bytes[1..inode_at],
        inode: u32::from_be_bytes(bytes[inode_at..inode_at + 4].try_into().ok()?),
        data: &bytes[inode_at + 4..record_len],
        len: record_len,
    })
}

struct DataBlock {
    off: i64,
    bytes: Arc<[u8]>,
}

impl DataBlock {
    fn end(&self) -> i64 {
        self.off.wrapping_add(self.bytes.len() as i64)
    }
}

struct DirState {
    inode: u32,
    credentials: Credentials,
    format: DataFormat,
    edge: u64,
    blocks: Vec<DataBlock>,
    cursor: Option<usize>,
    cache: Option<dirattrcache::Handle>,
    busy: bool,
    busy_off: Option<i64>,
    last_fetch: Option<(i64, u8)>,
    closing: bool,
}

pub struct DirSlot {
    operation: Mutex<()>,
    state: Mutex<DirState>,
    wake: Condvar,
}

impl DirSlot {
    pub fn new(inode: u32, credentials: Credentials, sustained: bool) -> Self {
        Self {
            operation: Mutex::new(()),
            state: Mutex::new(DirState {
                inode,
                credentials,
                format: if sustained {
                    DataFormat::Empty
                } else {
                    DataFormat::Basic
                },
                edge: if sustained { EDGEID_MAX } else { 0 },
                blocks: Vec::new(),
                cursor: None,
                cache: None,
                busy: false,
                busy_off: None,
                last_fetch: None,
                closing: false,
            }),
            wake: Condvar::new(),
        }
    }

    fn lock(&self) -> MutexGuard<'_, DirState> {
        self.state.lock().unwrap()
    }

    /// Serialize one complete FUSE readdir reply, matching the C callback's
    /// per-directory lock scope while `next` may unlock around the master RPC.
    pub fn begin_operation(&self) -> MutexGuard<'_, ()> {
        self.operation.lock().unwrap()
    }

    fn clear_data(state: &mut DirState) -> Option<dirattrcache::Handle> {
        state.blocks.clear();
        state.cursor = None;
        state.edge = 0;
        state.cache.take()
    }

    fn find_block(state: &mut DirState, off: i64) -> Option<usize> {
        if let Some(index) = state.cursor {
            let block = &state.blocks[index];
            if off >= block.off && off < block.end() {
                return Some(index);
            }
        }
        let found = state
            .blocks
            .iter()
            .position(|block| off >= block.off && off < block.end());
        state.cursor = found;
        found
    }

    pub fn next<F>(
        &self,
        requested_attrs: bool,
        off: i64,
        use_dir_cache: bool,
        attr_size: u8,
        mut fetch: F,
    ) -> Result<Option<Chunk>, u8>
    where
        F: FnMut(FetchRequest) -> FetchReply,
    {
        let mut reset_done = false;
        let mut waited_for_fetch = false;
        loop {
            let mut state = self.lock();
            while state.busy {
                waited_for_fetch |= state.busy_off == Some(off);
                state = self.wake.wait(state).unwrap();
            }
            if state.closing || state.format == DataFormat::Empty {
                return Ok(None);
            }

            if waited_for_fetch {
                if let Some((fetch_off, status)) = state.last_fetch {
                    if fetch_off == off {
                        if status != 0 {
                            return Err(status);
                        }
                        reset_done |= off == 0;
                    }
                }
                waited_for_fetch = false;
            }

            if off == 0 && !reset_done {
                drop(Self::clear_data(&mut state));
                state.last_fetch = None;
                reset_done = true;
            }

            if let Some(index) = Self::find_block(&mut state, off) {
                let block = &state.blocks[index];
                return Ok(Some(Chunk {
                    bytes: Arc::clone(&block.bytes),
                    start: off.wrapping_sub(block.off) as usize,
                    format: state.format,
                }));
            }

            let at_end = state.blocks.last().is_some_and(|block| off == block.end());
            let should_fetch =
                (reset_done && state.blocks.is_empty()) || (at_end && state.edge != EDGEID_MAX);
            if !should_fetch {
                return Ok(None);
            }

            let format = if state.edge == 0 {
                if use_dir_cache || requested_attrs {
                    DataFormat::Attrs
                } else {
                    DataFormat::Basic
                }
            } else {
                state.format
            };
            let request = FetchRequest {
                inode: state.inode,
                credentials: state.credentials.clone(),
                edge: state.edge,
                format,
            };
            state.busy = true;
            state.busy_off = Some(off);
            state.last_fetch = None;
            drop(state);

            let mut reply = fetch(request.clone());
            let mut final_format = format;
            if reply.status == MFS_ERROR_EACCES
                && request.edge == 0
                && format == DataFormat::Attrs
                && !requested_attrs
            {
                final_format = DataFormat::Basic;
                reply = fetch(FetchRequest {
                    edge: 0,
                    format: final_format,
                    ..request
                });
            }

            // A successful empty nonterminal page must advance its edge. Without
            // this guard, a malformed server response would spin in this call.
            if reply.status == 0
                && reply.bytes.is_empty()
                && reply.edge != EDGEID_MAX
                && reply.edge == request.edge
            {
                reply.status = MFS_ERROR_IO;
            }

            let mut state = self.lock();
            state.busy = false;
            state.busy_off = None;
            state.last_fetch = Some((off, reply.status));
            if !state.closing {
                state.format = final_format;
                state.edge = reply.edge;
                if reply.status == 0 {
                    let bytes: Arc<[u8]> = reply.bytes.into();
                    if use_dir_cache && final_format == DataFormat::Attrs {
                        let credentials = state.credentials.clone();
                        let inode = state.inode;
                        let cache = state.cache.get_or_insert_with(|| {
                            dirattrcache::new(
                                credentials.pid,
                                credentials.uid,
                                credentials.gid,
                                inode,
                                attr_size,
                            )
                        });
                        dirattrcache::append(cache, &bytes);
                    }
                    state.blocks.push(DataBlock { off, bytes });
                    state.cursor = Some(state.blocks.len() - 1);
                }
            }
            self.wake.notify_all();
            if state.closing {
                return Ok(None);
            }
            if reply.status != 0 {
                return Err(reply.status);
            }
        }
    }

    pub fn close(&self) {
        let _operation = self.begin_operation();
        let cache = {
            let mut state = self.lock();
            state.closing = true;
            while state.busy {
                state = self.wake.wait(state).unwrap();
            }
            state.credentials.groups.clear();
            Self::clear_data(&mut state)
        };
        drop(cache);
        self.wake.notify_all();
    }

    #[cfg(test)]
    fn owned_groups(&self) -> Vec<u32> {
        self.lock().credentials.groups.clone()
    }

    #[cfg(test)]
    fn is_closing(&self) -> bool {
        self.lock().closing
    }
}

struct RegistryState {
    active: HashMap<u64, Arc<DirSlot>>,
    next: u64,
}

impl Default for RegistryState {
    fn default() -> Self {
        Self {
            active: HashMap::new(),
            next: 1,
        }
    }
}

#[derive(Default)]
struct Registry {
    state: Mutex<RegistryState>,
}

impl Registry {
    fn insert(&self, slot: Arc<DirSlot>) -> u64 {
        let mut state = self.state.lock().unwrap();
        let token = state.next;
        state.next = token.checked_add(1).expect("dir handle space exhausted");
        state.active.insert(token, slot);
        token
    }

    fn get(&self, token: u64) -> Option<Arc<DirSlot>> {
        self.state.lock().unwrap().active.get(&token).cloned()
    }

    fn remove(&self, token: u64) -> Option<Arc<DirSlot>> {
        self.state.lock().unwrap().active.remove(&token)
    }

    fn drain(&self) -> Vec<Arc<DirSlot>> {
        self.state
            .lock()
            .unwrap()
            .active
            .drain()
            .map(|(_, slot)| slot)
            .collect()
    }
}

static REGISTRY: OnceLock<Registry> = OnceLock::new();

fn registry() -> &'static Registry {
    REGISTRY.get_or_init(Registry::default)
}

pub fn insert(slot: Arc<DirSlot>) -> u64 {
    registry().insert(slot)
}

pub fn get(token: u64) -> Option<Arc<DirSlot>> {
    registry().get(token)
}

pub fn release(token: u64) -> bool {
    let Some(slot) = registry().remove(token) else {
        return false;
    };
    slot.close();
    true
}

pub fn free_all() {
    for slot in registry().drain() {
        slot.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Barrier, mpsc};
    use std::thread;
    use std::time::Duration;

    fn credentials() -> Credentials {
        Credentials {
            pid: 3,
            uid: 4,
            gid: 5,
            groups: vec![5, 6, 7],
        }
    }

    #[test]
    fn stale_token_never_resolves_or_reuses() {
        let first = insert(Arc::new(DirSlot::new(1, credentials(), false)));
        assert!(release(first));
        assert!(get(first).is_none());
        let second = insert(Arc::new(DirSlot::new(2, credentials(), false)));
        assert_ne!(first, second);
        assert!(get(first).is_none());
        assert!(release(second));
    }

    #[test]
    fn local_registry_exhausts_only_at_u64_limit() {
        let registry = Registry {
            state: Mutex::new(RegistryState {
                active: HashMap::new(),
                next: u64::MAX - 1,
            }),
        };
        let token = registry.insert(Arc::new(DirSlot::new(1, credentials(), false)));
        assert_eq!(token, u64::MAX - 1);
        assert!(registry.remove(token).is_some());
        assert!(
            std::panic::catch_unwind(|| {
                registry.insert(Arc::new(DirSlot::new(2, credentials(), false)))
            })
            .is_err()
        );
    }

    #[test]
    fn in_flight_arc_survives_registry_removal() {
        let token = insert(Arc::new(DirSlot::new(1, credentials(), false)));
        let held = get(token).unwrap();
        assert!(release(token));
        assert!(get(token).is_none());
        assert_eq!(held.owned_groups(), Vec::<u32>::new());
    }

    #[test]
    fn fetch_contention_wakes_on_success_and_error() {
        for status in [0, 9] {
            let slot = Arc::new(DirSlot::new(1, credentials(), false));
            let calls = Arc::new(AtomicUsize::new(0));
            let barrier = Arc::new(Barrier::new(2));
            let first_slot = Arc::clone(&slot);
            let first_calls = Arc::clone(&calls);
            let first_barrier = Arc::clone(&barrier);
            let first = thread::spawn(move || {
                first_slot.next(false, 0, false, 36, |_| {
                    first_calls.fetch_add(1, Ordering::SeqCst);
                    first_barrier.wait();
                    thread::sleep(Duration::from_millis(20));
                    FetchReply {
                        status,
                        edge: EDGEID_MAX,
                        bytes: vec![1],
                    }
                })
            });
            barrier.wait();
            let second_calls = Arc::clone(&calls);
            let second = slot.next(false, 0, false, 36, |_| {
                second_calls.fetch_add(1, Ordering::SeqCst);
                FetchReply {
                    status,
                    edge: EDGEID_MAX,
                    bytes: vec![1],
                }
            });
            let first = first.join().unwrap();
            if status == 0 {
                assert!(first.unwrap().is_some());
                assert!(second.unwrap().is_some());
            } else {
                assert_eq!(first.unwrap_err(), status);
                assert_eq!(second.unwrap_err(), status);
            }
            assert_eq!(calls.load(Ordering::SeqCst), 1);
        }
    }

    #[test]
    fn close_during_delayed_fetch_waits_and_discards_response() {
        let slot = Arc::new(DirSlot::new(1, credentials(), false));
        let (started_tx, started_rx) = mpsc::channel();
        let (finish_tx, finish_rx) = mpsc::channel();
        let reader_slot = Arc::clone(&slot);
        let reader = thread::spawn(move || {
            reader_slot.next(false, 0, false, 36, |_| {
                started_tx.send(()).unwrap();
                finish_rx.recv().unwrap();
                FetchReply {
                    status: 0,
                    edge: EDGEID_MAX,
                    bytes: vec![1, 2],
                }
            })
        });
        started_rx.recv().unwrap();
        let closer_slot = Arc::clone(&slot);
        let closer = thread::spawn(move || closer_slot.close());
        while !slot.is_closing() {
            thread::yield_now();
        }
        finish_tx.send(()).unwrap();
        closer.join().unwrap();
        assert!(reader.join().unwrap().unwrap().is_none());
        assert_eq!(slot.owned_groups(), Vec::<u32>::new());
    }

    #[test]
    fn offsets_reset_exact_end_and_holes() {
        let slot = DirSlot::new(1, credentials(), false);
        let calls = AtomicUsize::new(0);
        let fetch = |_: FetchRequest| {
            let call = calls.fetch_add(1, Ordering::SeqCst);
            FetchReply {
                status: 0,
                edge: if call == 0 { 7 } else { EDGEID_MAX },
                bytes: if call == 0 { vec![1, 2] } else { vec![3] },
            }
        };
        assert_eq!(
            slot.next(false, 0, false, 36, fetch)
                .unwrap()
                .unwrap()
                .start,
            0
        );
        assert!(
            slot.next(false, 99, false, 36, |_| unreachable!())
                .unwrap()
                .is_none()
        );
        assert_eq!(
            slot.next(false, 2, false, 36, fetch)
                .unwrap()
                .unwrap()
                .bytes
                .as_ref(),
            &[3]
        );
        assert!(
            slot.next(false, 3, false, 36, |_| unreachable!())
                .unwrap()
                .is_none()
        );
        assert!(
            slot.next(false, 0, false, 36, |_| FetchReply {
                status: 0,
                edge: EDGEID_MAX,
                bytes: vec![9]
            })
            .unwrap()
            .is_some()
        );
    }

    #[test]
    fn empty_nonterminal_page_fetches_until_data() {
        let slot = DirSlot::new(1, credentials(), false);
        let calls = AtomicUsize::new(0);
        let chunk = slot
            .next(false, 0, false, 36, |request| {
                let call = calls.fetch_add(1, Ordering::SeqCst);
                if call == 0 {
                    assert_eq!(request.edge, 0);
                    FetchReply {
                        status: 0,
                        edge: 7,
                        bytes: Vec::new(),
                    }
                } else {
                    assert_eq!(request.edge, 7);
                    FetchReply {
                        status: 0,
                        edge: EDGEID_MAX,
                        bytes: vec![1],
                    }
                }
            })
            .unwrap()
            .unwrap();
        assert_eq!(chunk.bytes.as_ref(), &[1]);
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn unchanged_edge_empty_page_returns_io() {
        let slot = DirSlot::new(1, credentials(), false);
        assert_eq!(
            slot.next(false, 0, false, 36, |request| FetchReply {
                status: 0,
                edge: request.edge,
                bytes: Vec::new(),
            })
            .unwrap_err(),
            MFS_ERROR_IO
        );
    }

    #[test]
    fn format_selection_and_eacces_fallback() {
        let slot = DirSlot::new(1, credentials(), false);
        let formats = Mutex::new(Vec::new());
        let chunk = slot
            .next(false, 0, true, 36, |request| {
                formats.lock().unwrap().push(request.format);
                if request.format == DataFormat::Attrs {
                    FetchReply {
                        status: MFS_ERROR_EACCES,
                        edge: 99,
                        bytes: Vec::new(),
                    }
                } else {
                    FetchReply {
                        status: 0,
                        edge: EDGEID_MAX,
                        bytes: vec![1],
                    }
                }
            })
            .unwrap()
            .unwrap();
        assert_eq!(chunk.format, DataFormat::Basic);
        assert_eq!(
            *formats.lock().unwrap(),
            vec![DataFormat::Attrs, DataFormat::Basic]
        );

        let plus_slot = DirSlot::new(2, credentials(), false);
        let calls = AtomicUsize::new(0);
        assert_eq!(
            plus_slot
                .next(true, 0, false, 36, |_| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    FetchReply {
                        status: MFS_ERROR_EACCES,
                        edge: 1,
                        bytes: Vec::new(),
                    }
                })
                .unwrap_err(),
            MFS_ERROR_EACCES
        );
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        let continuation_slot = DirSlot::new(3, credentials(), false);
        assert!(
            continuation_slot
                .next(false, 0, true, 36, |_| FetchReply {
                    status: 0,
                    edge: 7,
                    bytes: vec![1],
                })
                .unwrap()
                .is_some()
        );
        let edges = Mutex::new(Vec::new());
        assert_eq!(
            continuation_slot
                .next(false, 1, true, 36, |request| {
                    edges.lock().unwrap().push(request.edge);
                    FetchReply {
                        status: MFS_ERROR_EACCES,
                        edge: 99,
                        bytes: Vec::new(),
                    }
                })
                .unwrap_err(),
            MFS_ERROR_EACCES
        );
        assert_eq!(*edges.lock().unwrap(), vec![7]);
    }

    #[test]
    fn parser_rejects_truncation_and_preserves_cookie_math() {
        let basic = [3, b'f', b'o', b'o', 0, 0, 0, 0, 2];
        assert!(parse_record(&basic[..8], DataFormat::Basic, 36).is_none());
        let deleted = parse_record(&basic, DataFormat::Basic, 36).unwrap();
        assert_eq!(deleted.name, b"foo");
        assert_eq!(deleted.inode, 0);
        assert_eq!(deleted.data, &[2]);
        assert_eq!(deleted.len, 9);
        assert_eq!(deleted.next_offset(i64::MAX - 4), i64::MIN + 4);

        let mut attrs = vec![1, b'x', 0, 0, 0, 7];
        attrs.extend_from_slice(&[9; 36]);
        let record = parse_record(&attrs, DataFormat::Attrs, 36).unwrap();
        assert_eq!(record.inode, 7);
        assert_eq!(record.len, 42);
        assert_eq!(record.next_offset(11), 53);
        assert!(parse_record(&attrs[..41], DataFormat::Attrs, 36).is_none());
        assert!(parse_record(&attrs, DataFormat::Empty, 36).is_none());
    }

    #[test]
    fn operations_are_serialized() {
        let slot = Arc::new(DirSlot::new(1, credentials(), false));
        let held = slot.begin_operation();
        let (tx, rx) = mpsc::channel();
        let waiting = Arc::clone(&slot);
        let thread = thread::spawn(move || {
            let _operation = waiting.begin_operation();
            tx.send(()).unwrap();
        });
        assert!(rx.recv_timeout(Duration::from_millis(20)).is_err());
        drop(held);
        rx.recv_timeout(Duration::from_secs(1)).unwrap();
        thread.join().unwrap();
    }

    #[test]
    fn supplementary_groups_are_owned() {
        let mut groups = vec![7, 8];
        let slot = DirSlot::new(
            1,
            Credentials {
                pid: 1,
                uid: 2,
                gid: 3,
                groups: groups.clone(),
            },
            false,
        );
        groups[0] = 99;
        assert_eq!(slot.owned_groups(), vec![7, 8]);
    }
}
