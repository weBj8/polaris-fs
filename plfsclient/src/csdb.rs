//! Chunkserver operation counters.
//!
//! Native Rust state replaces the C hash table, intrusive lists, malloc'd
//! mutex, and internal C ABI. Counts intentionally use wrapping arithmetic.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Default)]
struct Counts {
    read: u32,
    write: u32,
}

static COUNTS: LazyLock<Mutex<HashMap<(u32, u16), Counts>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn init() {
    COUNTS.lock().unwrap().clear();
}

pub fn term() {
    COUNTS.lock().unwrap().clear();
}

pub fn get_read(ip: u32, port: u16) -> u32 {
    COUNTS
        .lock()
        .unwrap()
        .get(&(ip, port))
        .map_or(0, |counts| counts.read)
}

pub fn get_write(ip: u32, port: u16) -> u32 {
    COUNTS
        .lock()
        .unwrap()
        .get(&(ip, port))
        .map_or(0, |counts| counts.write)
}

pub fn get_operations(ip: u32, port: u16) -> u32 {
    COUNTS
        .lock()
        .unwrap()
        .get(&(ip, port))
        .map_or(0, |counts| counts.read.wrapping_add(counts.write))
}

pub fn read_inc(ip: u32, port: u16) {
    let mut counts = COUNTS.lock().unwrap();
    let entry = counts.entry((ip, port)).or_default();
    entry.read = entry.read.wrapping_add(1);
}

pub fn read_dec(ip: u32, port: u16) {
    if let Some(entry) = COUNTS.lock().unwrap().get_mut(&(ip, port)) {
        entry.read = entry.read.wrapping_sub(1);
    }
}

pub fn write_inc(ip: u32, port: u16) {
    let mut counts = COUNTS.lock().unwrap();
    let entry = counts.entry((ip, port)).or_default();
    entry.write = entry.write.wrapping_add(1);
}

pub fn write_dec(ip: u32, port: u16) {
    if let Some(entry) = COUNTS.lock().unwrap().get_mut(&(ip, port)) {
        entry.write = entry.write.wrapping_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counters_are_independent() {
        init();
        read_inc(1, 2);
        read_inc(1, 2);
        write_inc(1, 2);
        write_inc(1, 3);
        assert_eq!(get_read(1, 2), 2);
        assert_eq!(get_write(1, 2), 1);
        assert_eq!(get_operations(1, 2), 3);
        assert_eq!(get_operations(1, 3), 1);
        read_dec(1, 2);
        write_dec(1, 2);
        assert_eq!(get_operations(1, 2), 1);
        term();
        assert_eq!(get_operations(1, 2), 0);
    }

    #[test]
    fn missing_decrement_is_noop() {
        init();
        read_dec(9, 9);
        write_dec(9, 9);
        assert_eq!(get_operations(9, 9), 0);
    }
}
