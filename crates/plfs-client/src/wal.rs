//! Write-ahead log (design doc §7.2): a segmented append-only log with a
//! per-record crc32c, group-commit fsync, and prefix-consistent replay.
//!
//! On-disk layout per segment file (`seg_<id:016>.wal`, ids increase by 1):
//!
//! ```text
//! header (16 B): magic u32 "PWAL" | format_version u32 = 1 | segment_id u64
//! record*: payload_len u32 | payload_crc32c u32 | payload | zero pad to 8 B
//!          (payload_len == 0 ⇒ segment terminator: end of records)
//! ```
//!
//! Records are 8-byte aligned and never cross segments. Durability
//! invariants (this is what makes replay prefix-consistent):
//!
//! - `sync()` fsyncs the active segment: every record appended before the
//!   call is durable when it returns (group commit — the caller batches).
//! - Segment switch writes the terminator, fsyncs the OLD segment, then
//!   creates the new one (header + file fsync + directory fsync). So a torn
//!   tail can only ever exist in the LAST segment, and everything torn off
//!   was not yet acknowledged.
//! - Replay reads segments in id order, verifies each record crc, and stops
//!   at the first torn/garbage byte: the file is truncated there; if that
//!   segment is not the last, later segments are deleted (they can only
//!   contain unacknowledged records by the switch invariant). The result is
//!   always a prefix of the appended record sequence containing every
//!   acknowledged record.
//!
//! `truncate_prefix` deletes whole segments below a position; the client
//! calls it after the corresponding records are committed to metadata
//! (§7.2). A crash can theoretically resurrect a deleted segment file
//! (unlink not yet durable): replay is append-order and the client's apply
//! must be idempotent.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

const SEGMENT_MAGIC: u32 = 0x5057_414C; // "PWAL"
const FORMAT_VERSION: u32 = 1;
const HEADER_LEN: u64 = 16;
const RECORD_HEADER: u64 = 8;

/// Default segment length (design doc §7.2: 64 MiB segments).
pub const DEFAULT_SEGMENT_LEN: u64 = 64 << 20;

/// Position after a record (segment id, byte offset within the segment).
/// Orders lexicographically; everything `<=` a durable position survived a
/// crash once the corresponding `sync()` returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecordPos {
    /// Segment id.
    pub segment: u64,
    /// Byte offset one past the record's end within the segment.
    pub offset: u64,
}

/// A record recovered by replay, in append order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayRecord {
    /// Position after this record.
    pub pos: RecordPos,
    /// Opaque payload as appended.
    pub payload: Vec<u8>,
}

/// WAL errors.
#[derive(Debug, thiserror::Error)]
pub enum WalError {
    /// Zero-length payloads are reserved (a zero length on disk is the
    /// segment terminator).
    #[error("empty payload records are not supported")]
    EmptyPayload,
    /// The record cannot ever fit in one segment.
    #[error("record of {len} bytes exceeds usable segment capacity {max}")]
    Oversize {
        /// Offered payload length.
        len: usize,
        /// Usable per-segment capacity.
        max: u64,
    },
    /// A segment file's name does not parse.
    #[error("wal directory contains invalid segment file name: {0}")]
    BadSegmentName(String),
    /// Underlying I/O failure.
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

/// A segmented write-ahead log. Single-writer (`&mut self`); the client's
/// flusher thread owns it.
pub struct Wal {
    dir: PathBuf,
    segment_len: u64,
    active: u64,
    file: File,
    offset: u64,
    durable: RecordPos,
}

impl Wal {
    /// Open (or create) the WAL in `dir`, replaying every segment: torn
    /// tails are truncated away, later segments after a torn one are
    /// deleted, and the surviving records are returned in append order.
    pub fn open(
        dir: impl AsRef<Path>,
        segment_len: u64,
    ) -> Result<(Self, Vec<ReplayRecord>), WalError> {
        if segment_len < HEADER_LEN + RECORD_HEADER + 8 || !segment_len.is_multiple_of(8) {
            return Err(WalError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "segment_len must be a multiple of 8 and hold a minimal record",
            )));
        }
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir)?;
        let mut segments = list_segments(&dir)?;
        let mut records = Vec::new();
        let mut torn: Option<(u64, u64)> = None; // (segment id, truncate-at)
        for (i, &id) in segments.iter().enumerate() {
            let path = segment_path(&dir, id);
            let file = File::open(&path)?;
            let file_len = file.metadata()?.len();
            match replay_segment(&file, id, file_len, &mut records)? {
                SegmentEnd::Clean => {}
                SegmentEnd::Torn(at) => {
                    torn = Some((id, at));
                    // Recovery rule: a torn segment is the crash point;
                    // everything after it was never acknowledged (switch
                    // invariant) and is deleted.
                    for &later in &segments[i + 1..] {
                        std::fs::remove_file(segment_path(&dir, later))?;
                    }
                    segments.truncate(i + 1);
                    break;
                }
            }
        }
        if let Some((id, at)) = torn {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(segment_path(&dir, id))?;
            file.set_len(at)?;
            file.sync_data()?;
            sync_dir(&dir)?;
        }
        let (active, offset) = match (torn, segments.last()) {
            (Some((id, at)), _) => (id, at),
            (None, Some(&last)) => (last, std::fs::metadata(segment_path(&dir, last))?.len()),
            (None, None) => {
                let id = 1;
                create_segment(&dir, id, segment_len)?;
                segments.push(id);
                (id, HEADER_LEN)
            }
        };
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(segment_path(&dir, active))?;
        file.sync_data()?;
        let wal = Wal {
            dir,
            segment_len,
            active,
            file,
            offset,
            durable: RecordPos {
                segment: active,
                offset,
            },
        };
        Ok((wal, records))
    }

    /// Append a record (not yet durable; see [`Wal::sync`]). Returns the
    /// position one past the record.
    pub fn append(&mut self, payload: &[u8]) -> Result<RecordPos, WalError> {
        if payload.is_empty() {
            return Err(WalError::EmptyPayload);
        }
        let record_len = align8(RECORD_HEADER + payload.len() as u64);
        let usable = self.segment_len - HEADER_LEN;
        if record_len > usable {
            return Err(WalError::Oversize {
                len: payload.len(),
                max: usable - RECORD_HEADER,
            });
        }
        if self.offset + record_len > self.segment_len {
            self.seal_and_advance()?;
        }
        let mut buf = Vec::with_capacity(record_len as usize);
        buf.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        buf.extend_from_slice(&crc32fast::hash(payload).to_le_bytes());
        buf.extend_from_slice(payload);
        buf.resize(record_len as usize, 0);
        self.file.write_all_at(&buf, self.offset)?;
        self.offset += record_len;
        Ok(RecordPos {
            segment: self.active,
            offset: self.offset,
        })
    }

    /// Group commit: fsync the active segment. Every record appended before
    /// this call is durable once it returns; the returned position is the
    /// new durability horizon.
    pub fn sync(&mut self) -> Result<RecordPos, WalError> {
        self.file.sync_data()?;
        self.durable = RecordPos {
            segment: self.active,
            offset: self.offset,
        };
        Ok(self.durable)
    }

    /// The durability horizon: everything `<=` this position was covered by
    /// a completed `sync()` (or a segment switch, which fsyncs the old
    /// segment).
    pub fn durable_pos(&self) -> RecordPos {
        self.durable
    }

    /// Current append position (the position the next record will get).
    pub fn append_pos(&self) -> RecordPos {
        RecordPos {
            segment: self.active,
            offset: self.offset,
        }
    }

    /// Delete whole segments strictly below `pos.segment` (the client calls
    /// this after those records are committed to metadata, §7.2). The
    /// active segment is never deleted.
    pub fn truncate_prefix(&mut self, pos: RecordPos) -> Result<(), WalError> {
        for id in list_segments(&self.dir)? {
            if id < pos.segment && id != self.active {
                std::fs::remove_file(segment_path(&self.dir, id))?;
            }
        }
        sync_dir(&self.dir)
    }

    /// Write the terminator into the active segment, fsync it, then create
    /// the next segment. This ordering is the switch invariant: the old
    /// segment is fully durable before the new one exists.
    fn seal_and_advance(&mut self) -> Result<(), WalError> {
        let remaining = self.segment_len - self.offset;
        if remaining >= RECORD_HEADER {
            self.file
                .write_all_at(&[0u8; RECORD_HEADER as usize], self.offset)?;
        }
        self.file.sync_data()?;
        let next = self.active + 1;
        create_segment(&self.dir, next, self.segment_len)?;
        self.file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(segment_path(&self.dir, next))?;
        self.active = next;
        self.offset = HEADER_LEN;
        self.durable = RecordPos {
            segment: next,
            offset: HEADER_LEN,
        };
        Ok(())
    }
}

fn align8(n: u64) -> u64 {
    n.div_ceil(8) * 8
}

fn segment_path(dir: &Path, id: u64) -> PathBuf {
    dir.join(format!("seg_{id:016}.wal"))
}

fn list_segments(dir: &Path) -> Result<Vec<u64>, WalError> {
    let mut ids = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if let Some(id) = name
            .strip_prefix("seg_")
            .and_then(|s| s.strip_suffix(".wal"))
            .map(str::parse::<u64>)
        {
            ids.push(id.map_err(|_| WalError::BadSegmentName(name.to_string()))?);
        }
    }
    ids.sort_unstable();
    Ok(ids)
}

fn sync_dir(dir: &Path) -> Result<(), WalError> {
    File::open(dir)?.sync_all()?;
    Ok(())
}

/// Create a segment file with its 16-byte header; fsync file + directory so
/// an existing segment always has a valid header after a crash.
fn create_segment(dir: &Path, id: u64, _segment_len: u64) -> Result<(), WalError> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(segment_path(dir, id))?;
    let mut header = Vec::with_capacity(HEADER_LEN as usize);
    header.extend_from_slice(&SEGMENT_MAGIC.to_le_bytes());
    header.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    header.extend_from_slice(&id.to_le_bytes());
    file.write_all_at(&header, 0)?;
    file.sync_data()?;
    sync_dir(dir)
}

enum SegmentEnd {
    Clean,
    Torn(u64),
}

/// Read one segment, appending valid records to `out`. `Clean` = records
/// ended at a boundary or terminator; `Torn(at)` = garbage/torn bytes at
/// `at` (the file must be truncated there).
fn replay_segment(
    file: &File,
    id: u64,
    file_len: u64,
    out: &mut Vec<ReplayRecord>,
) -> Result<SegmentEnd, WalError> {
    let mut header = [0u8; HEADER_LEN as usize];
    if file_len < HEADER_LEN || file.read_exact_at(&mut header, 0).is_err() {
        return Ok(SegmentEnd::Torn(0));
    }
    let magic = u32::from_le_bytes(header[0..4].try_into().expect("4"));
    let version = u32::from_le_bytes(header[4..8].try_into().expect("4"));
    let seg_id = u64::from_le_bytes(header[8..16].try_into().expect("8"));
    if magic != SEGMENT_MAGIC || version != FORMAT_VERSION || seg_id != id {
        return Ok(SegmentEnd::Torn(0));
    }
    let mut off = HEADER_LEN;
    loop {
        if off == file_len {
            return Ok(SegmentEnd::Clean);
        }
        let mut rh = [0u8; RECORD_HEADER as usize];
        if file.read_exact_at(&mut rh, off).is_err() {
            return Ok(SegmentEnd::Torn(off));
        }
        let len = u32::from_le_bytes(rh[0..4].try_into().expect("4"));
        let crc = u32::from_le_bytes(rh[4..8].try_into().expect("4"));
        if len == 0 {
            return Ok(SegmentEnd::Clean); // terminator
        }
        let record_len = align8(RECORD_HEADER + u64::from(len));
        if off + record_len > file_len {
            return Ok(SegmentEnd::Torn(off));
        }
        let mut payload = vec![0u8; len as usize];
        file.read_exact_at(&mut payload, off + RECORD_HEADER)?;
        if crc32fast::hash(&payload) != crc {
            return Ok(SegmentEnd::Torn(off));
        }
        off += record_len;
        out.push(ReplayRecord {
            pos: RecordPos {
                segment: id,
                offset: off,
            },
            payload,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir(name: &str) -> PathBuf {
        let base = std::env::var("CARGO_TARGET_TMPDIR").unwrap_or_else(|_| {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/tmp").to_string()
        });
        let dir = PathBuf::from(base).join(format!("plfs-wal-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("tmpdir");
        dir
    }

    fn payloads(wal: &mut Wal, specs: &[(u8, usize)]) -> Vec<Vec<u8>> {
        specs
            .iter()
            .map(|&(fill, len)| {
                let p = vec![fill; len];
                wal.append(&p).expect("append");
                p
            })
            .collect()
    }

    #[test]
    fn fresh_open_then_roundtrip() {
        let dir = tmpdir("fresh");
        let (mut wal, records) = Wal::open(&dir, 4096).expect("open");
        assert!(records.is_empty());
        let want = payloads(&mut wal, &[(1, 10), (2, 100), (3, 1)]);
        wal.sync().expect("sync");
        drop(wal);
        let (_wal, records) = Wal::open(&dir, 4096).expect("reopen");
        let got: Vec<Vec<u8>> = records.into_iter().map(|r| r.payload).collect();
        assert_eq!(got, want);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn segment_switch_keeps_order() {
        let dir = tmpdir("switch");
        let (mut wal, _) = Wal::open(&dir, 4096).expect("open");
        // usable = 4096-16; records of ~500 B ⇒ several switches.
        let specs: Vec<(u8, usize)> = (0..40u8).map(|i| (i, 480 + i as usize)).collect();
        let want = payloads(&mut wal, &specs);
        wal.sync().expect("sync");
        drop(wal);
        assert!(list_segments(&dir).expect("list").len() > 3);
        let (_wal, records) = Wal::open(&dir, 4096).expect("reopen");
        let got: Vec<Vec<u8>> = records.into_iter().map(|r| r.payload).collect();
        assert_eq!(got, want);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn torn_mid_record_is_discarded() {
        let dir = tmpdir("torn");
        let (mut wal, _) = Wal::open(&dir, 4096).expect("open");
        let want = payloads(&mut wal, &[(1, 50), (2, 50), (3, 50)]);
        wal.sync().expect("sync");
        let last = wal.append(&[9u8; 50]).expect("append unsynced");
        drop(wal);
        // Tear inside the unsynced record: only a prefix of it survives.
        let path = segment_path(&dir, last.segment);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("open");
        file.set_len(last.offset - 10).expect("truncate");
        drop(file);
        let (_wal, records) = Wal::open(&dir, 4096).expect("reopen");
        let got: Vec<Vec<u8>> = records.into_iter().map(|r| r.payload).collect();
        assert_eq!(got, want);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn garbage_tail_is_discarded() {
        let dir = tmpdir("garbage");
        let (mut wal, _) = Wal::open(&dir, 4096).expect("open");
        let want = payloads(&mut wal, &[(7, 30); 4]);
        wal.sync().expect("sync");
        let pos = wal.append_pos();
        drop(wal);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(segment_path(&dir, pos.segment))
            .expect("open");
        file.write_all_at(&[0xAB; 37], pos.offset).expect("garbage");
        drop(file);
        let (_wal, records) = Wal::open(&dir, 4096).expect("reopen");
        let got: Vec<Vec<u8>> = records.into_iter().map(|r| r.payload).collect();
        assert_eq!(got, want);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn truncate_prefix_deletes_old_segments() {
        let dir = tmpdir("truncate");
        let (mut wal, _) = Wal::open(&dir, 2048).expect("open");
        let mut second_segment_pos = None;
        for i in 0..30u8 {
            let pos = wal.append(&[i; 400]).expect("append");
            if second_segment_pos.is_none() && pos.segment == 2 {
                second_segment_pos = Some(pos);
            }
            if i % 5 == 0 {
                wal.sync().expect("sync");
            }
        }
        wal.sync().expect("sync");
        let pos = second_segment_pos.expect("reached segment 2");
        wal.truncate_prefix(pos).expect("truncate");
        let ids = list_segments(&dir).expect("list");
        assert!(!ids.contains(&1), "segment 1 must be deleted: {ids:?}");
        drop(wal);
        let (_wal, records) = Wal::open(&dir, 2048).expect("reopen");
        assert!(records.iter().all(|r| r.pos.segment >= 2));
        assert!(!records.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn durable_pos_tracks_sync_and_switch() {
        let dir = tmpdir("durable");
        let (mut wal, _) = Wal::open(&dir, 2048).expect("open");
        let p1 = wal.append(&[1; 100]).expect("append");
        assert!(wal.durable_pos() < p1, "unsynced record is not durable");
        wal.sync().expect("sync");
        assert!(wal.durable_pos() >= p1, "synced record is durable");
        // Force a switch: everything before the new segment becomes durable.
        while wal.append_pos().segment == 1 {
            wal.append(&[2; 400]).expect("append");
        }
        assert_eq!(
            wal.durable_pos(),
            RecordPos {
                segment: 2,
                offset: HEADER_LEN
            }
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn rejects_empty_and_oversize() {
        let dir = tmpdir("reject");
        let (mut wal, _) = Wal::open(&dir, 2048).expect("open");
        assert!(matches!(wal.append(&[]), Err(WalError::EmptyPayload)));
        let big = vec![0u8; 4096];
        assert!(matches!(wal.append(&big), Err(WalError::Oversize { .. })));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn bad_header_on_nonlast_segment_discards_tail() {
        let dir = tmpdir("badhdr");
        let (mut wal, _) = Wal::open(&dir, 2048).expect("open");
        while wal.append_pos().segment == 1 {
            wal.append(&[3; 400]).expect("append");
        }
        wal.append(&[4; 50]).expect("append");
        wal.sync().expect("sync");
        drop(wal);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(segment_path(&dir, 1))
            .expect("open seg 1");
        file.write_all_at(&[0xFF; 4], 0).expect("corrupt magic");
        drop(file);
        let (_wal, records) = Wal::open(&dir, 2048).expect("reopen");
        assert!(records.is_empty(), "torn first segment discards the tail");
        assert!(!segment_path(&dir, 2).exists(), "later segments deleted");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
