//! Allocation bitmaps (contract §3): one bit per slot, L-map then S-map as
//! the persisted payload, each copy followed by crc32c of the payload and
//! zero padding to `bitmap_copy_bytes`. Bits are a hint only — slot headers
//! are the truth and the maps are rebuilt at every open.

use crate::index::SlotClass;

/// In-memory pair of allocation bitmaps (LSB-first bit order, u64 words).
#[derive(Debug, Clone)]
pub(crate) struct Bitmaps {
    l: Vec<u64>,
    s: Vec<u64>,
    l_count: u64,
    s_count: u64,
}

impl Bitmaps {
    pub(crate) fn new(l_count: u64, s_count: u64) -> Self {
        Self {
            l: vec![0; words(l_count)],
            s: vec![0; words(s_count)],
            l_count,
            s_count,
        }
    }

    fn bits(&self, class: SlotClass) -> (&[u64], u64) {
        match class {
            SlotClass::L => (&self.l, self.l_count),
            SlotClass::S => (&self.s, self.s_count),
        }
    }

    fn bits_mut(&mut self, class: SlotClass) -> &mut [u64] {
        match class {
            SlotClass::L => &mut self.l,
            SlotClass::S => &mut self.s,
        }
    }

    pub(crate) fn test(&self, class: SlotClass, idx: u64) -> bool {
        let (bits, count) = self.bits(class);
        idx < count && bits[idx as usize / 64] & (1 << (idx % 64)) != 0
    }

    pub(crate) fn set(&mut self, class: SlotClass, idx: u64) {
        self.bits_mut(class)[idx as usize / 64] |= 1 << (idx % 64);
    }

    pub(crate) fn clear(&mut self, class: SlotClass, idx: u64) {
        self.bits_mut(class)[idx as usize / 64] &= !(1 << (idx % 64));
    }

    /// First clear bit at or after `start`, wrapping once (contract §6:
    /// bitmap scan from a random offset).
    pub(crate) fn find_free_from(&self, class: SlotClass, start: u64) -> Option<u64> {
        let (_, count) = self.bits(class);
        if count == 0 {
            return None;
        }
        let start = start % count;
        for k in 0..count {
            let idx = (start + k) % count;
            if !self.test(class, idx) {
                return Some(idx);
            }
        }
        None
    }

    pub(crate) fn free_count(&self, class: SlotClass) -> u64 {
        let (bits, count) = self.bits(class);
        let used: u64 = bits.iter().map(|w| u64::from(w.count_ones())).sum();
        count - used.min(count)
    }

    /// Persisted payload length: per class `ceil(count/8)` padded to 8 bytes.
    pub(crate) fn payload_len(l_count: u64, s_count: u64) -> usize {
        words(l_count) * 8 + words(s_count) * 8
    }

    /// Payload bytes + trailing u32 crc32c of the payload (no copy padding).
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(Self::payload_len(self.l_count, self.s_count) + 4);
        for w in &self.l {
            out.extend_from_slice(&w.to_le_bytes());
        }
        for w in &self.s {
            out.extend_from_slice(&w.to_le_bytes());
        }
        let crc = crc32fast::hash(&out);
        out.extend_from_slice(&crc.to_le_bytes());
        out
    }

    /// Inverse of [`Bitmaps::encode`]; `None` on wrong length or bad crc.
    /// Used by tests to verify what was flushed; production opens rebuild
    /// from headers instead.
    #[cfg(test)]
    pub(crate) fn decode(buf: &[u8], l_count: u64, s_count: u64) -> Option<Self> {
        let payload = Self::payload_len(l_count, s_count);
        if buf.len() != payload + 4 {
            return None;
        }
        let (body, crc) = buf.split_at(payload);
        let want = u32::from_le_bytes(crc.try_into().ok()?);
        if crc32fast::hash(body) != want {
            return None;
        }
        let read_words = |n: usize, from: usize| -> Vec<u64> {
            (0..n)
                .map(|i| {
                    u64::from_le_bytes(
                        body[from + i * 8..from + (i + 1) * 8]
                            .try_into()
                            .expect("word"),
                    )
                })
                .collect()
        };
        let l = read_words(words(l_count), 0);
        let s = read_words(words(s_count), words(l_count) * 8);
        Some(Self {
            l,
            s,
            l_count,
            s_count,
        })
    }
}

fn words(count: u64) -> usize {
    count.div_ceil(64) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_test_clear() {
        let mut b = Bitmaps::new(100, 10);
        assert!(!b.test(SlotClass::L, 99));
        b.set(SlotClass::L, 99);
        assert!(b.test(SlotClass::L, 99));
        assert!(!b.test(SlotClass::L, 98));
        b.clear(SlotClass::L, 99);
        assert!(!b.test(SlotClass::L, 99));
        assert!(!b.test(SlotClass::L, 100));
        b.set(SlotClass::S, 9);
        assert!(b.test(SlotClass::S, 9));
        assert!(!b.test(SlotClass::L, 9));
    }

    #[test]
    fn find_free_wraps() {
        let mut b = Bitmaps::new(4, 0);
        for i in 0..4 {
            b.set(SlotClass::L, i);
        }
        b.clear(SlotClass::L, 1);
        assert_eq!(b.find_free_from(SlotClass::L, 2), Some(1));
        b.set(SlotClass::L, 1);
        assert_eq!(b.find_free_from(SlotClass::L, 0), None);
        assert_eq!(Bitmaps::new(0, 0).find_free_from(SlotClass::S, 0), None);
    }

    #[test]
    fn free_count_tracks() {
        let mut b = Bitmaps::new(10, 3);
        assert_eq!(b.free_count(SlotClass::L), 10);
        assert_eq!(b.free_count(SlotClass::S), 3);
        b.set(SlotClass::L, 0);
        b.set(SlotClass::L, 9);
        assert_eq!(b.free_count(SlotClass::L), 8);
    }

    #[test]
    fn encode_decode_roundtrip() {
        let mut b = Bitmaps::new(100, 37);
        b.set(SlotClass::L, 0);
        b.set(SlotClass::L, 99);
        b.set(SlotClass::S, 36);
        let buf = b.encode();
        assert_eq!(buf.len(), Bitmaps::payload_len(100, 37) + 4);
        let got = Bitmaps::decode(&buf, 100, 37).expect("decode");
        assert!(got.test(SlotClass::L, 0));
        assert!(got.test(SlotClass::L, 99));
        assert!(got.test(SlotClass::S, 36));
        assert!(!got.test(SlotClass::S, 35));
        assert_eq!(got.free_count(SlotClass::L), 98);
    }

    #[test]
    fn decode_rejects_bad_crc_and_len() {
        let b = Bitmaps::new(8, 8);
        let mut buf = b.encode();
        assert!(Bitmaps::decode(&buf[..buf.len() - 1], 8, 8).is_none());
        let last = buf.len() - 1;
        buf[last] ^= 0x01;
        assert!(Bitmaps::decode(&buf, 8, 8).is_none());
    }
}
