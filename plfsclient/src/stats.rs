//! Hierarchical runtime counters.
//!
//! Native Rust tree replaces malloc'd intrusive nodes, pthread locking, and
//! `void *` handles. Nodes are stable `Arc` values shared by producers.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex, Weak};

pub type StatsHandle = Arc<StatsNode>;

pub struct StatsNode {
    counter: AtomicU64,
    print: bool,
    absolute: bool,
    fullname: String,
    parent: Weak<StatsNode>,
    children: Mutex<Vec<StatsHandle>>,
}

static ROOTS: LazyLock<Mutex<Vec<StatsHandle>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn subnode(
    parent: Option<&StatsHandle>,
    name: &str,
    absolute: bool,
    print: bool,
) -> StatsHandle {
    let children: &Mutex<Vec<StatsHandle>> = match parent {
        Some(node) => &node.children,
        None => &ROOTS,
    };
    let mut children = children.lock().unwrap();
    if let Some(node) = children.iter().find(|node| {
        node.fullname
            .rsplit_once('.')
            .map_or(node.fullname.as_str(), |(_, name)| name)
            == name
    }) {
        return Arc::clone(node);
    }
    let fullname = parent.map_or_else(
        || name.to_owned(),
        |parent| format!("{}.{}", parent.fullname, name),
    );
    let node = Arc::new(StatsNode {
        counter: AtomicU64::new(0),
        print,
        absolute,
        fullname,
        parent: parent.map_or_else(Weak::new, Arc::downgrade),
        children: Mutex::new(Vec::new()),
    });
    // C inserted at list head; preserve traversal/output order.
    children.insert(0, Arc::clone(&node));
    node
}

fn ancestors(node: &StatsHandle) -> impl Iterator<Item = StatsHandle> {
    std::iter::successors(Some(Arc::clone(node)), |node| node.parent.upgrade())
}

pub fn counter_add(node: &StatsHandle, delta: u64) {
    for node in ancestors(node) {
        node.counter.fetch_add(delta, Ordering::Relaxed);
        if node.absolute {
            break;
        }
    }
}

pub fn counter_sub(node: &StatsHandle, delta: u64) {
    for node in ancestors(node) {
        node.counter.fetch_sub(delta, Ordering::Relaxed);
        if node.absolute {
            break;
        }
    }
}

pub fn counter_inc(node: &StatsHandle) {
    counter_add(node, 1);
}

pub fn counter_dec(node: &StatsHandle) {
    counter_sub(node, 1);
}

pub fn counter_set(node: &StatsHandle, value: u64) {
    if node.absolute {
        node.counter.store(value, Ordering::Relaxed);
    }
}

fn reset(node: &StatsHandle) {
    if !node.absolute {
        node.counter.store(0, Ordering::Relaxed);
    }
    for child in node.children.lock().unwrap().clone() {
        reset(&child);
    }
}

pub fn reset_all() {
    for root in ROOTS.lock().unwrap().clone() {
        reset(&root);
    }
}

fn append_values(node: &StatsHandle, out: &mut Vec<u8>) {
    if node.print {
        let value = node.counter.load(Ordering::Relaxed);
        if node.absolute {
            out.extend_from_slice(format!("{}: [{}]\n", node.fullname, value).as_bytes());
        } else {
            out.extend_from_slice(format!("{}: {}\n", node.fullname, value).as_bytes());
        }
    }
    for child in node.children.lock().unwrap().clone() {
        append_values(&child, out);
    }
}

pub fn show_all() -> Vec<u8> {
    let mut out = Vec::new();
    for root in ROOTS.lock().unwrap().clone() {
        append_values(&root, &mut out);
    }
    out
}

pub fn term() {
    ROOTS.lock().unwrap().clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    #[test]
    fn hierarchy_propagation_reset_and_output() {
        let _test = TEST_LOCK.lock().unwrap();
        term();
        let root = subnode(None, "root", false, false);
        let leaf = subnode(Some(&root), "leaf", false, true);
        counter_add(&leaf, 3);
        assert_eq!(show_all(), b"root.leaf: 3\n");
        reset_all();
        assert_eq!(show_all(), b"root.leaf: 0\n");
    }

    #[test]
    fn absolute_counter_stops_propagation() {
        let _test = TEST_LOCK.lock().unwrap();
        term();
        let root = subnode(None, "root", false, true);
        let gauge = subnode(Some(&root), "gauge", true, true);
        counter_set(&gauge, 7);
        counter_inc(&gauge);
        assert_eq!(show_all(), b"root: 0\nroot.gauge: [8]\n");
    }

    #[test]
    fn duplicate_subnode_returns_same_handle() {
        let _test = TEST_LOCK.lock().unwrap();
        term();
        let first = subnode(None, "same", false, true);
        let second = subnode(None, "same", true, false);
        assert!(Arc::ptr_eq(&first, &second));
    }
}
