#![deny(unsafe_code)]

use plfsclient::inoleng;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

const SLOT_MASK: u32 = 0x00ff_ffff;
const MAX_SLOTS: u32 = SLOT_MASK;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoMode {
    ReadWrite,
    ReadOnly,
    ReadAppend,
}

impl IoMode {
    pub fn writable(self) -> bool {
        matches!(self, Self::ReadWrite | Self::ReadAppend)
    }

    pub fn append_only(self) -> bool {
        self == Self::ReadAppend
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LockOwner {
    pub owner: u64,
    pub pid: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenState {
    pub waiting: usize,
    pub in_master: bool,
    pub status: i32,
}

pub struct FileState<R, W> {
    pub mode: IoMode,
    pub use_locks: u8,
    pub open: OpenState,
    pub read_data: Option<R>,
    pub write_data: Option<W>,
    pub posix_owners: Vec<LockOwner>,
    pub flock_owners: Vec<LockOwner>,
}

impl<R, W> FileState<R, W> {
    pub fn remember_posix_owner(&mut self, owner: LockOwner) {
        if !self
            .posix_owners
            .iter()
            .any(|entry| entry.owner == owner.owner)
        {
            self.posix_owners.insert(0, owner);
        }
    }

    pub fn remember_flock_owner(&mut self, owner: u64) {
        if !self.flock_owners.iter().any(|entry| entry.owner == owner) {
            self.flock_owners.insert(0, LockOwner { owner, pid: 0 });
        }
    }

    pub fn take_lock_owners(
        &mut self,
        current_flock_owner: u64,
    ) -> (Vec<LockOwner>, Vec<LockOwner>) {
        let posix = std::mem::take(&mut self.posix_owners);
        let flock = std::mem::take(&mut self.flock_owners)
            .into_iter()
            .filter(|entry| entry.owner != current_flock_owner)
            .collect();
        (posix, flock)
    }

    pub fn remove_posix_owners_for_pid(&mut self, pid: i32, current_owner: u64) -> Vec<LockOwner> {
        let mut removed = Vec::new();
        self.posix_owners.retain(|entry| {
            if entry.pid == pid || entry.owner == current_owner {
                if entry.pid == pid && entry.owner != current_owner {
                    removed.push(*entry);
                }
                false
            } else {
                true
            }
        });
        removed
    }
}

pub struct FileInfo<R, W> {
    pub inode: u32,
    pub length: Option<inoleng::Handle>,
    pub created: f64,
    state: Mutex<FileState<R, W>>,
    open_changed: Condvar,
}

impl<R, W> FileInfo<R, W> {
    pub fn new(
        inode: u32,
        length: Option<inoleng::Handle>,
        created: f64,
        mode: IoMode,
        open_in_master: bool,
    ) -> Self {
        Self {
            inode,
            length,
            created,
            state: Mutex::new(FileState {
                mode,
                use_locks: 0,
                open: OpenState {
                    waiting: 0,
                    in_master: open_in_master,
                    status: 0,
                },
                read_data: None,
                write_data: None,
                posix_owners: Vec::new(),
                flock_owners: Vec::new(),
            }),
            open_changed: Condvar::new(),
        }
    }

    pub fn state(&self) -> MutexGuard<'_, FileState<R, W>> {
        self.state.lock().unwrap()
    }

    pub fn wait_for_open(&self) -> i32 {
        let mut state = self.state();
        while !state.open.in_master {
            state.open.waiting += 1;
            state = self.open_changed.wait(state).unwrap();
            state.open.waiting -= 1;
        }
        state.open.status
    }

    pub fn wait_for_open_locked<'a>(
        &self,
        mut state: MutexGuard<'a, FileState<R, W>>,
    ) -> MutexGuard<'a, FileState<R, W>> {
        while !state.open.in_master {
            state.open.waiting += 1;
            state = self.open_changed.wait(state).unwrap();
            state.open.waiting -= 1;
        }
        state
    }

    pub fn complete_open(&self, status: i32) {
        let mut state = self.state();
        state.open.status = status;
        state.open.in_master = true;
        self.open_changed.notify_all();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileHandle(u32);

impl FileHandle {
    pub fn from_u32(value: u32) -> Option<Self> {
        (value != 0 && value & 0xff00_0000 != 0).then_some(Self(value))
    }

    pub fn from_fuse(value: u64) -> Option<Self> {
        u32::try_from(value).ok().and_then(Self::from_u32)
    }

    pub fn get(self) -> u32 {
        self.0
    }

    fn slot(self) -> u32 {
        self.0 & SLOT_MASK
    }

    fn generation(self) -> u8 {
        (self.0 >> 24) as u8
    }
}

struct Slot<R, W> {
    generation: u8,
    value: Option<Arc<FileInfo<R, W>>>,
}

pub struct Removed<R, W> {
    slot: u32,
    pub file: Arc<FileInfo<R, W>>,
}

pub struct Registry<R, W> {
    slots: Vec<Slot<R, W>>,
    free: Vec<u32>,
}

impl<R, W> Default for Registry<R, W> {
    fn default() -> Self {
        Self {
            slots: vec![Slot {
                generation: 0,
                value: None,
            }],
            free: Vec::new(),
        }
    }
}

impl<R, W> Registry<R, W> {
    pub fn insert(&mut self, file: FileInfo<R, W>) -> FileHandle {
        let slot = if let Some(slot) = self.free.pop() {
            slot
        } else {
            let slot = u32::try_from(self.slots.len()).expect("file handle table too big");
            assert!(slot <= MAX_SLOTS, "file handle table too big");
            self.slots.push(Slot {
                generation: 0,
                value: None,
            });
            slot
        };
        let entry = &mut self.slots[slot as usize];
        entry.generation = entry.generation.wrapping_add(1);
        if entry.generation == 0 {
            entry.generation = 1;
        }
        entry.value = Some(Arc::new(file));
        FileHandle((u32::from(entry.generation) << 24) | slot)
    }

    pub fn get(&self, handle: FileHandle) -> Option<Arc<FileInfo<R, W>>> {
        let entry = self.slots.get(handle.slot() as usize)?;
        (entry.generation == handle.generation())
            .then(|| entry.value.as_ref().map(Arc::clone))
            .flatten()
    }

    pub fn remove(&mut self, handle: FileHandle) -> Option<Removed<R, W>> {
        let entry = self.slots.get_mut(handle.slot() as usize)?;
        if entry.generation != handle.generation() {
            return None;
        }
        entry.value.take().map(|file| Removed {
            slot: handle.slot(),
            file,
        })
    }

    pub fn recycle(&mut self, removed: Removed<R, W>) -> Arc<FileInfo<R, W>> {
        let entry = &self.slots[removed.slot as usize];
        debug_assert!(entry.value.is_none());
        // Retiring generation 255 prevents an ancient handle becoming valid again.
        if entry.generation != u8::MAX {
            self.free.push(removed.slot);
        }
        removed.file
    }

    pub fn take_all(&mut self) -> Vec<Arc<FileInfo<R, W>>> {
        self.free.clear();
        self.slots
            .iter_mut()
            .filter_map(|slot| slot.value.take())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    fn info(inode: u32) -> FileInfo<(), ()> {
        FileInfo::new(inode, None, 0.0, IoMode::ReadWrite, true)
    }

    #[test]
    fn generation_rejects_stale_handles_and_wrap_skips_zero() {
        let mut registry = Registry::default();
        let stale = registry.insert(info(1));
        let mut current = stale;
        for expected in 2..=255u8 {
            let removed = registry.remove(current).unwrap();
            registry.recycle(removed);
            current = registry.insert(info(1));
            assert_eq!(current.generation(), expected);
            assert!(registry.get(stale).is_none());
        }
        let retired_slot = current.slot();
        let removed = registry.remove(current).unwrap();
        registry.recycle(removed);
        current = registry.insert(info(1));
        assert_ne!(current.slot(), retired_slot);
        assert_eq!(current.generation(), 1);
        assert!(registry.get(stale).is_none());
    }

    #[test]
    fn arc_lookup_survives_registry_removal() {
        let mut registry = Registry::default();
        let handle = registry.insert(info(9));
        let lookup = registry.get(handle).unwrap();
        let removed = registry.remove(handle).unwrap();
        assert!(registry.get(handle).is_none());
        assert_eq!(lookup.inode, 9);
        registry.recycle(removed);
        assert_eq!(lookup.inode, 9);
    }

    #[test]
    fn async_open_waiters_observe_success_and_failure() {
        for status in [0, libc::EACCES] {
            let file = Arc::new(FileInfo::<(), ()>::new(
                2,
                None,
                0.0,
                IoMode::ReadOnly,
                false,
            ));
            let waiter = {
                let file = Arc::clone(&file);
                thread::spawn(move || file.wait_for_open())
            };
            while file.state().open.waiting == 0 {
                thread::yield_now();
            }
            file.complete_open(status);
            assert_eq!(waiter.join().unwrap(), status);
        }
    }

    #[test]
    fn release_removes_before_cleanup_and_recycles_afterward() {
        let mut registry = Registry::default();
        let handle = registry.insert(info(3));
        let removed = registry.remove(handle).unwrap();
        assert!(registry.get(handle).is_none());
        let next = registry.insert(info(4));
        assert_ne!(next.slot(), handle.slot());
        registry.recycle(removed);
        let reused = registry.insert(info(5));
        assert_eq!(reused.slot(), handle.slot());
    }

    #[test]
    fn resources_drop_exactly_once() {
        struct Counted(Arc<AtomicUsize>);
        impl Drop for Counted {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::Relaxed);
            }
        }
        let drops = Arc::new(AtomicUsize::new(0));
        let file = FileInfo::<Counted, ()>::new(4, None, 0.0, IoMode::ReadWrite, true);
        file.state().read_data = Some(Counted(Arc::clone(&drops)));
        drop(file);
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn lock_owner_snapshot_preserves_order_and_excludes_current_flock() {
        let file = info(6);
        let mut state = file.state();
        state.remember_posix_owner(LockOwner { owner: 1, pid: 7 });
        state.remember_posix_owner(LockOwner { owner: 2, pid: 7 });
        state.remember_posix_owner(LockOwner { owner: 1, pid: 7 });
        state.remember_flock_owner(3);
        state.remember_flock_owner(4);
        let (posix, flock) = state.take_lock_owners(4);
        assert_eq!(
            posix.iter().map(|entry| entry.owner).collect::<Vec<_>>(),
            [2, 1]
        );
        assert_eq!(
            flock.iter().map(|entry| entry.owner).collect::<Vec<_>>(),
            [3]
        );
    }

    #[test]
    fn same_inode_length_handle_is_retained() {
        plfsclient::inoleng::init();
        let first = plfsclient::inoleng::acquire(77);
        let file =
            FileInfo::<(), ()>::new(77, Some(Arc::clone(&first)), 0.0, IoMode::ReadWrite, true);
        drop(first);
        plfsclient::inoleng::set_length(file.length.as_ref().unwrap(), 99);
        let second = plfsclient::inoleng::acquire(77);
        assert_eq!(plfsclient::inoleng::get_length(&second), 99);
    }
}
