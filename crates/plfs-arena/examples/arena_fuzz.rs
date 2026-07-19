//! kill -9 fuzz harness for ChunkArena crash consistency (contract §5/§7).
//!
//! `write <arena> <acklog> <seed>`: mkfs a fresh arena, then loop a
//! deterministic op stream (seeded) of Put/PutAgain/Delete. Every op outcome
//! is checked against a model prediction (divergence = contract violation =
//! panic). Acked ops append an fsynced line to the acklog AFTER the op
//! returned: `PUT <hexid> <version>` / `DEL <hexid> <version>`.
//!
//! `verify <arena> <acklog> <seed>`: reopen (boot scan + reconcile), replay
//! the same deterministic op stream to fold the acklog, then assert:
//!   - every live (acked PUT, not acked DEL) chunk is present byte-exact;
//!   - every allocated chunk traces to an acked PUT with identical
//!     version+payload (no false-allocated slots);
//!   - delete-acked chunks may be present (resurrect, contract §6/§7) but
//!     must be byte-exact;
//!   - at most one inconsistency explained by the kill gap (op acked in
//!     memory but its acklog line lost, or op killed in flight): the gap op
//!     is exactly the next acking op after the last acklog line.
//!
//! Payloads are regenerated from (chunk_id, version) via a xorshift
//! expander, so the verifier needs no payload storage. The op stream and
//! model are fully deterministic in `seed`, so writer and verifier agree
//! op-for-op (including OutOfSpace/AlreadyExists/NoOp/false outcomes).

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use plfs_arena::{Arena, ArenaError, ChunkId, MkfsConfig, PutOutcome, SlotClass};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const TOTAL: u64 = 48 << 20;
const ID_POOL: usize = 64;
const MAX_PAYLOAD: usize = 100_000;

fn config() -> MkfsConfig {
    MkfsConfig::new(TOTAL)
}

// ---------- deterministic payload expander ----------

/// Payload bytes for (chunk_id, version): xorshift64* stream seeded from the
/// id and version; the first stream word also fixes the length.
fn payload_for(id: &ChunkId, version: u64) -> Vec<u8> {
    let b = id.as_bytes();
    let s0 = u64::from_le_bytes(b[..8].try_into().expect("8"));
    let s1 = u64::from_le_bytes(b[8..].try_into().expect("8"));
    let mut state = s0 ^ s1.rotate_left(31) ^ version.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    if state == 0 {
        state = 0x1234_5678_9ABC_DEF0;
    }
    let mut next = move || {
        state ^= state >> 12;
        state <<= 25;
        state ^= state >> 27;
        state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    };
    let len = (next() % (MAX_PAYLOAD as u64 + 1)) as usize;
    let mut out = vec![0u8; len];
    for chunk in out.chunks_mut(8) {
        let x = next().to_le_bytes();
        chunk.copy_from_slice(&x[..chunk.len()]);
    }
    out
}

// ---------- deterministic op stream ----------

#[derive(Debug, Clone, Copy)]
enum FuzzOp {
    Put { idx: usize, version: u64 },
    PutAgain { idx: usize },
    Delete { idx: usize, exact: bool },
}

struct OpGen {
    rng: StdRng,
    next_version: u64,
}

impl OpGen {
    fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            next_version: 1,
        }
    }

    fn next_op(&mut self) -> FuzzOp {
        let idx = self.rng.random_range(0..ID_POOL);
        match self.rng.random_range(0..10u8) {
            0..=5 => {
                let version = self.next_version;
                self.next_version += 1;
                FuzzOp::Put { idx, version }
            }
            6..=7 => FuzzOp::PutAgain { idx },
            _ => FuzzOp::Delete {
                idx,
                exact: self.rng.random_bool(0.75),
            },
        }
    }
}

fn make_ids(seed: u64) -> Vec<ChunkId> {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x1D5E_ED5E_ED5E_ED5E);
    (0..ID_POOL)
        .map(|_| {
            let mut b = [0u8; 16];
            rng.fill(&mut b);
            b[0] |= 1; // never nil
            ChunkId::from(b)
        })
        .collect()
}

// ---------- model (deterministic outcome prediction) ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Stored,
    NoOp,
    AlreadyExists,
    OutOfSpace,
    Deleted,
    NotDeleted,
    Skip,
}

impl Outcome {
    /// Outcomes that produce an acklog line when they complete.
    fn acks(self) -> bool {
        matches!(self, Outcome::Stored | Outcome::Deleted)
    }
}

struct Model {
    live: HashMap<usize, (u64, SlotClass)>,
    l_free: u64,
    s_free: u64,
    s_cap: usize,
}

impl Model {
    fn new(l_count: u64, s_count: u64, s_cap: usize) -> Self {
        Self {
            live: HashMap::new(),
            l_free: l_count,
            s_free: s_count,
            s_cap,
        }
    }

    fn class_of(&self, id: &ChunkId, version: u64) -> SlotClass {
        let len = payload_for(id, version).len();
        if len <= self.s_cap {
            SlotClass::S
        } else {
            SlotClass::L
        }
    }

    fn predict(&self, op: &FuzzOp, ids: &[ChunkId]) -> Outcome {
        match *op {
            FuzzOp::Put { idx, version } => {
                if self.live.contains_key(&idx) {
                    return Outcome::AlreadyExists;
                }
                let free = match self.class_of(&ids[idx], version) {
                    SlotClass::L => self.l_free,
                    SlotClass::S => self.s_free,
                };
                if free == 0 {
                    Outcome::OutOfSpace
                } else {
                    Outcome::Stored
                }
            }
            FuzzOp::PutAgain { idx } => {
                if self.live.contains_key(&idx) {
                    Outcome::NoOp
                } else {
                    Outcome::Skip
                }
            }
            FuzzOp::Delete { idx, exact } => match self.live.get(&idx) {
                Some(_) if exact => Outcome::Deleted,
                _ => Outcome::NotDeleted,
            },
        }
    }

    fn commit(&mut self, op: &FuzzOp, outcome: Outcome, ids: &[ChunkId]) {
        match (*op, outcome) {
            (FuzzOp::Put { idx, version }, Outcome::Stored) => {
                let class = self.class_of(&ids[idx], version);
                match class {
                    SlotClass::L => self.l_free -= 1,
                    SlotClass::S => self.s_free -= 1,
                }
                self.live.insert(idx, (version, class));
            }
            (FuzzOp::Delete { idx, .. }, Outcome::Deleted) => {
                let (_, class) = self.live.remove(&idx).expect("live at predict");
                match class {
                    SlotClass::L => self.l_free += 1,
                    SlotClass::S => self.s_free += 1,
                }
            }
            _ => {}
        }
    }
}

// ---------- acklog ----------

fn hex_of(id: &ChunkId) -> String {
    let mut s = String::with_capacity(32);
    for b in id.as_bytes() {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn id_from_hex(s: &str) -> Option<ChunkId> {
    let mut b = [0u8; 16];
    for (i, w) in (0..16).zip(s.as_bytes().chunks_exact(2)) {
        b[i] = u8::from_str_radix(std::str::from_utf8(w).ok()?, 16).ok()?;
    }
    Some(ChunkId::from(b))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AckKind {
    Put,
    Del,
}

type AckLine = (AckKind, ChunkId, u64);

fn parse_acklog(bytes: &[u8]) -> Result<Vec<AckLine>, String> {
    // A kill can leave a torn final line; only newline-terminated lines count.
    let end = bytes
        .iter()
        .rposition(|b| *b == b'\n')
        .map(|p| p + 1)
        .unwrap_or(0);
    let text = std::str::from_utf8(&bytes[..end]).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for line in text.lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        if t.len() != 3 {
            return Err(format!("bad acklog line: {line:?}"));
        }
        let kind = match t[0] {
            "PUT" => AckKind::Put,
            "DEL" => AckKind::Del,
            other => return Err(format!("bad acklog op: {other}")),
        };
        let id = id_from_hex(t[1]).ok_or_else(|| format!("bad id hex: {}", t[1]))?;
        let version: u64 = t[2].parse().map_err(|_| format!("bad version: {}", t[2]))?;
        out.push((kind, id, version));
    }
    Ok(out)
}

fn write_line(ack: &mut File, line: &str) {
    ack.write_all(line.as_bytes()).expect("acklog write");
    ack.sync_data().expect("acklog fsync");
}

// ---------- writer ----------

fn cmd_write(arena_path: &Path, ack_path: &Path, seed: u64) {
    let _ = std::fs::remove_file(arena_path);
    let report = Arena::mkfs(arena_path, &config()).expect("mkfs");
    let mut arena = Arena::open(arena_path).expect("open");
    let s_cap = report.geometry.capacity(SlotClass::S) as usize;
    let mut model = Model::new(
        report.geometry.l_slot_count,
        report.geometry.s_slot_count,
        s_cap,
    );
    let ids = make_ids(seed);
    let mut opgen = OpGen::new(seed);
    let mut slp = StdRng::seed_from_u64(seed ^ 0x5EED_5EED_5EED_5EED);
    let mut ack = File::create(ack_path).expect("acklog create");
    loop {
        let op = opgen.next_op();
        let predicted = model.predict(&op, &ids);
        match op {
            FuzzOp::Put { idx, version } => {
                let id = ids[idx];
                let payload = payload_for(&id, version);
                match arena.put(id, version, &payload) {
                    Ok(PutOutcome::Stored) => assert!(
                        predicted == Outcome::Stored,
                        "model/arena divergence: predicted {predicted:?}, arena Stored"
                    ),
                    Ok(PutOutcome::NoOp) => {
                        panic!("contract violation: fresh-version put returned NoOp")
                    }
                    Err(ArenaError::AlreadyExists(_)) => assert!(
                        predicted == Outcome::AlreadyExists,
                        "model/arena divergence: predicted {predicted:?}, arena AlreadyExists"
                    ),
                    Err(ArenaError::OutOfSpace { .. }) => assert!(
                        predicted == Outcome::OutOfSpace,
                        "model/arena divergence: predicted {predicted:?}, arena OutOfSpace"
                    ),
                    Err(e) => panic!("put failed unexpectedly: {e}"),
                }
                model.commit(&op, predicted, &ids);
                if predicted == Outcome::Stored {
                    write_line(&mut ack, &format!("PUT {} {version}\n", hex_of(&id)));
                }
            }
            FuzzOp::PutAgain { idx } => {
                if let Some(&(version, _)) = model.live.get(&idx) {
                    let id = ids[idx];
                    let payload = payload_for(&id, version);
                    assert_eq!(
                        arena.put(id, version, &payload).expect("putagain"),
                        PutOutcome::NoOp,
                        "contract violation: idempotent re-put was not NoOp"
                    );
                }
            }
            FuzzOp::Delete { idx, exact } => {
                let req = match (model.live.get(&idx), exact) {
                    (Some(&(v, _)), true) => v,
                    (Some(&(v, _)), false) => v.wrapping_add(1),
                    (None, _) => 0,
                };
                let deleted = arena.delete(&ids[idx], req).expect("delete");
                assert_eq!(
                    deleted,
                    predicted == Outcome::Deleted,
                    "model/arena divergence on delete: predicted {predicted:?}, arena {deleted}"
                );
                model.commit(&op, predicted, &ids);
                if deleted {
                    write_line(&mut ack, &format!("DEL {} {req}\n", hex_of(&ids[idx])));
                }
            }
        }
        std::thread::sleep(Duration::from_micros(slp.random_range(0..500)));
    }
}

// ---------- verifier ----------

fn fail(msg: &str) -> ExitCode {
    println!("VERIFY_FAIL: {msg}");
    ExitCode::from(1)
}

fn cmd_verify(arena_path: &Path, ack_path: &Path, seed: u64) -> ExitCode {
    let bytes = std::fs::read(ack_path).unwrap_or_default();
    let lines = match parse_acklog(&bytes) {
        Ok(lines) => lines,
        Err(e) => return fail(&format!("acklog parse: {e}")),
    };
    let mut arena = match Arena::open(arena_path) {
        Ok(arena) => arena,
        Err(e) => {
            if lines.is_empty() {
                println!("VERIFY_OK live=0 present=0 (killed during mkfs: {e})");
                return ExitCode::SUCCESS;
            }
            return fail(&format!(
                "arena open failed ({e}) but {} ops were acked",
                lines.len()
            ));
        }
    };
    let geom = *arena.geometry();
    let mut model = Model::new(
        geom.l_slot_count,
        geom.s_slot_count,
        geom.capacity(SlotClass::S) as usize,
    );
    let ids = make_ids(seed);
    let mut opgen = OpGen::new(seed);
    let mut acked_puts: HashSet<(ChunkId, u64)> = HashSet::new();
    let mut acked_dels: HashSet<(ChunkId, u64)> = HashSet::new();
    let mut gap_put: Option<(ChunkId, u64)> = None;
    let mut gap_del: Option<(ChunkId, u64)> = None;
    // Replay the deterministic op stream; fold consumed acklog lines; the
    // first acking op without a line is the tolerated kill-gap op.
    let mut line_idx = 0usize;
    loop {
        let op = opgen.next_op();
        let outcome = model.predict(&op, &ids);
        if outcome.acks() && line_idx >= lines.len() {
            match op {
                FuzzOp::Put { idx, version } => gap_put = Some((ids[idx], version)),
                FuzzOp::Delete { idx, .. } => {
                    let (v, _) = model.live[&idx];
                    gap_del = Some((ids[idx], v));
                }
                FuzzOp::PutAgain { .. } => unreachable!("NoOp never acks"),
            }
            break;
        }
        match op {
            FuzzOp::Put { idx, version } if outcome == Outcome::Stored => {
                let want = (AckKind::Put, ids[idx], version);
                if lines[line_idx] != want {
                    return fail(&format!(
                        "acklog/op-stream mismatch at line {line_idx}: want {want:?}, got {:?}",
                        lines[line_idx]
                    ));
                }
                acked_puts.insert((ids[idx], version));
                line_idx += 1;
            }
            FuzzOp::Delete { idx, .. } if outcome == Outcome::Deleted => {
                let (v, _) = model.live[&idx];
                let want = (AckKind::Del, ids[idx], v);
                if lines[line_idx] != want {
                    return fail(&format!(
                        "acklog/op-stream mismatch at line {line_idx}: want {want:?}, got {:?}",
                        lines[line_idx]
                    ));
                }
                acked_dels.insert((ids[idx], v));
                line_idx += 1;
            }
            _ => {}
        }
        model.commit(&op, outcome, &ids);
    }
    // Gate (a): every live chunk present byte-exact (one gap delete may be
    // missing; a resurrected older version may legally shadow it, §5/§7).
    let mut shadowed = 0usize;
    for (idx, (v_live, _)) in model.live.clone() {
        let id = ids[idx];
        match arena.stat(&id) {
            Some(m) if m.version == v_live => {
                let (_, got) = match arena.get(&id) {
                    Ok(r) => r,
                    Err(e) => return fail(&format!("live chunk {id} unreadable: {e}")),
                };
                if got != payload_for(&id, v_live) {
                    return fail(&format!("live chunk {id} v{v_live} payload mismatch"));
                }
            }
            Some(m) if acked_dels.contains(&(id, m.version)) => {
                // Older, delete-acked version resurrected and won duplicate
                // arbitration; the newer live version stays durable on disk
                // but shadowed. Must be byte-exact at its own version and the
                // duplicate must be logged as corruption evidence (§5).
                let (_, got) = match arena.get(&id) {
                    Ok(r) => r,
                    Err(e) => return fail(&format!("shadowing chunk {id} unreadable: {e}")),
                };
                if got != payload_for(&id, m.version) {
                    return fail(&format!(
                        "shadowing chunk {id} v{} payload mismatch",
                        m.version
                    ));
                }
                if arena.corruption_evidence().is_empty() {
                    return fail(&format!("duplicate chunk {id} not logged as corruption"));
                }
                shadowed += 1;
            }
            _ if gap_del == Some((id, v_live)) => {}
            other => {
                return fail(&format!(
                    "live chunk {id} v{v_live} lost (stat={other:?}, no gap explanation)"
                ));
            }
        }
    }
    // Gate (b): every allocated chunk traces to an acked PUT (or the one gap
    // put) with identical payload; resurrected delete-acked chunks are fine.
    let present = arena.list();
    let mut resurrected = 0usize;
    for meta in &present {
        let key = (meta.chunk_id, meta.version);
        let (_, got) = match arena.get(&meta.chunk_id) {
            Ok(r) => r,
            Err(e) => return fail(&format!("allocated chunk {meta:?} unreadable: {e}")),
        };
        if got != payload_for(&meta.chunk_id, meta.version) {
            return fail(&format!("allocated chunk {meta:?} payload mismatch"));
        }
        if acked_dels.contains(&key) {
            resurrected += 1;
        }
        if acked_puts.contains(&key) || gap_put == Some(key) {
            continue;
        }
        return fail(&format!(
            "false-allocated slot: {meta:?} was never put-acked"
        ));
    }
    println!(
        "VERIFY_OK live={} present={} resurrected={} shadowed={} gap_put={} gap_del={}",
        model.live.len(),
        present.len(),
        resurrected,
        shadowed,
        gap_put.is_some(),
        gap_del.is_some(),
    );
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("usage: {} write|verify <arena> <acklog> <seed>", args[0]);
        return ExitCode::from(2);
    }
    let seed: u64 = match args[4].parse() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("bad seed: {}", args[4]);
            return ExitCode::from(2);
        }
    };
    let arena = Path::new(&args[2]);
    let ack = Path::new(&args[3]);
    match args[1].as_str() {
        "write" => {
            cmd_write(arena, ack, seed);
            ExitCode::SUCCESS
        }
        "verify" => cmd_verify(arena, ack, seed),
        other => {
            eprintln!("unknown subcommand: {other}");
            ExitCode::from(2)
        }
    }
}
