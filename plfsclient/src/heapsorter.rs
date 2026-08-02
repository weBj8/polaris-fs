//! Min-heap of u32 inode numbers — safe Rust rewrite.
//!
//! Original: MooseFS mfsclient/heapsorter.c. Sole consumer is
//! mastercomm::fs_send_open_inodes. Growth matches C: 1024 initial,
//! doubling. heap_pop on empty returns 0 (C semantics).
//!
//! Static mutable state guarded by the caller's af_lock (mastercomm
//! holds AF_LOCK across every call).

static mut HEAP: Vec<u32> = Vec::new();
static mut HEAPELEMENTS: u32 = 0;

fn heap() -> &'static mut Vec<u32> {
    // SAFETY: every caller holds mastercomm AF_LOCK; single-threaded access.
    unsafe { &mut *(&raw mut HEAP) }
}

fn elements() -> u32 {
    // SAFETY: as above.
    unsafe { *(&raw const HEAPELEMENTS) }
}

fn set_elements(v: u32) {
    // SAFETY: as above.
    unsafe {
        *(&raw mut HEAPELEMENTS) = v;
    }
}

fn heap_sort_down() {
    let heap = heap();
    let heapelements = elements();
    let mut pos: u32 = 0;
    while pos < heapelements {
        let l = pos.wrapping_mul(2).wrapping_add(1);
        let r = l.wrapping_add(1);
        if l >= heapelements {
            return;
        }
        let mut m = l;
        if r < heapelements && heap[r as usize] < heap[l as usize] {
            m = r;
        }
        if heap[pos as usize] <= heap[m as usize] {
            return;
        }
        heap.swap(pos as usize, m as usize);
        pos = m;
    }
}

fn heap_sort_up() {
    let heap = heap();
    let mut pos = elements().wrapping_sub(1);
    while pos > 0 {
        let p = pos.wrapping_sub(1).wrapping_div(2);
        if heap[pos as usize] >= heap[p as usize] {
            return;
        }
        heap.swap(pos as usize, p as usize);
        pos = p;
    }
}

pub fn heap_cleanup() {
    set_elements(0);
}

pub fn heap_push(element: u32) {
    if elements() as usize >= heap().len() {
        // C: malloc(1024) then doubling mfsrealloc; OOM aborts.
        let newsize = if heap().is_empty() {
            1024
        } else {
            heap().len() << 1
        };
        heap().resize(newsize, 0);
    }
    let pos = elements() as usize;
    heap()[pos] = element;
    set_elements(elements().wrapping_add(1));
    heap_sort_up();
}

pub fn heap_pop() -> u32 {
    let heap = heap();
    let mut heapelements = elements();
    if heapelements == 0 {
        return 0;
    }
    let element = heap[0];
    heapelements = heapelements.wrapping_sub(1);
    set_elements(heapelements);
    if heapelements > 0 {
        heap[0] = heap[heapelements as usize];
        heap_sort_down();
    }
    element
}

pub fn heap_elements() -> u32 {
    elements()
}

pub fn heap_term() {
    *heap() = Vec::new();
    set_elements(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pops_in_sorted_order() {
        heap_cleanup();
        for v in [9u32, 3, 7, 1, 8, 2, 5] {
            heap_push(v);
        }
        assert_eq!(heap_elements(), 7);
        let mut out = Vec::new();
        while heap_elements() > 0 {
            out.push(heap_pop());
        }
        assert_eq!(out, [1, 2, 3, 5, 7, 8, 9]);
        assert_eq!(heap_pop(), 0);
        heap_term();
        assert_eq!(heap_elements(), 0);
    }
}
