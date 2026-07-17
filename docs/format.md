# PolarisFS on-disk format v2 (FORMAT_VERSION = 2)

> Contract file: change this document and bump the version BEFORE changing code.
> All multi-byte integers are little-endian. A device is a single regular file or
> block device. All I/O is 4KiB (BLOCK) aligned.
> v2 changes (introduced by P2): DATA_START 1MiB→64MiB; the former reserved region
> becomes the checkpoint area (checkpoint slots); group-commit and durability-horizon
> semantics are now explicit. v1(v0) devices are rejected, not migrated (v0 never
> left the dev box).

## 1. Device layout

```
[0          .. 4KiB)   superblock copy A
[4KiB       .. 8KiB)   superblock copy B
[8KiB       .. 32MiB)  checkpoint slot A (capacity 32MiB-8KiB)
[32MiB      .. 64MiB)  checkpoint slot B (capacity 32MiB)
[64MiB(DATA_START) ..) extent log region: extent records appended in order
```

## 2. Superblock (exactly 4KiB)

| Offset | Size | Field | Description |
|---|---|---|---|
| 0    | 8  | magic        | `b"PORFS_SB"` |
| 8    | 4  | format_version | u32, = 2 |
| 12   | 4  | flags        | u32; bit0 = clean_unmount (set on clean unmount, cleared on mount) |
| 16   | 8  | device_size  | u64, total device bytes |
| 24   | 8  | data_start   | u64, = 64MiB |
| 32   | 8  | tail         | u64, byte offset of the next append |
| 40   | 8  | extent_count | u64, live extents (excludes tombstones) |
| 48   | 8  | sync_seq     | u64, +1 per sync; the copy with the larger sync_seq wins |
| 56   | 16 | uuid         | filesystem UUID |
| 72   | 8  | created_at   | unix seconds |
| 80   | 8  | last_sync_at | unix seconds |
| 88   | 4004 | reserved   | all zero |
| 4092 | 4  | sb_crc32c    | u32, CRC32C over [0..4092) |

Open rules: validate each copy independently (magic + version + CRC); take the
valid copy with the larger sync_seq; two valid copies with different tails are
normal (the later-written copy may not have landed); both invalid → refuse to mount.
Write rules: on sync write **copy B first, then copy A** (A is authoritative),
bumping sync_seq.

## 3. Extent records

header (64 bytes) + data (data_len bytes) + zero padding to a 4KiB boundary.
A single record's data_len ≤ EXTENT_DATA_MAX = 4MiB. The whole unit
(header+data+pad) is called disk_len and must be a multiple of 4KiB.

header layout (64B):

| Offset | Size | Field | Description |
|---|---|---|---|
| 0  | 4 | magic          | u32 = 0x4558_5431 ("EXT1" LE) |
| 4  | 2 | header_len     | u16 = 64 |
| 6  | 2 | flags          | u16; bit0 = tombstone (logical delete) |
| 8  | 8 | extent_id      | u64, monotonically increasing, device-unique |
| 16 | 8 | inode          | u64, owning inode (pass-through only as of P1) |
| 24 | 8 | logical_offset | u64, logical byte offset inside the file |
| 32 | 4 | data_len       | u32, payload bytes (excludes padding) |
| 36 | 4 | data_crc32c    | u32, over data[0..data_len) |
| 40 | 4 | disk_len       | u32, total bytes of header+data+pad (multiple of 4KiB) |
| 44 | 16 | reserved       | all zero |
| 60 | 4 | header_crc32c  | u32, over header[0..60) |

## 4. Checkpoint area (checkpoint slots)

A checkpoint is a consistent snapshot of the in-memory index (extent_id → location)
at a given log position, so mounting does not require a full scan.
Two slots are written ping-pong; a slot contains a 64B header + an entry array
(40B each; payload zero-padded up to a 4KiB multiple).

checkpoint header (64B):

| Offset | Size | Field | Description |
|---|---|---|---|
| 0  | 4 | magic           | u32 = 0x3150_4B43 ("CKP1" LE) |
| 4  | 2 | header_len      | u16 = 64 |
| 6  | 2 | flags           | u16 = 0 |
| 8  | 8 | slot_gen        | u64, monotonically increasing across both slots; larger wins |
| 16 | 8 | covered_tail    | u64, log offset this checkpoint's index covers (scanning resumes there) |
| 24 | 8 | next_extent_id  | u64 |
| 32 | 8 | extent_count    | u64 (live count) |
| 40 | 8 | live_bytes      | u64 |
| 48 | 4 | entry_count     | u32 |
| 52 | 4 | payload_crc32c  | u32, over the payload (entry_count×40B, excludes padding) |
| 56 | 4 | reserved        | all zero |
| 60 | 4 | header_crc32c   | u32, over header[0..60) |

entry (40B): `extent_id u64, offset u64, disk_len u32, data_len u32, inode u64, logical_offset u64`.
Only **live** entries are stored (tombstoned extents are dropped at checkpoint time).

Write protocol: on `checkpoint()`, slot_gen = newest valid gen + 1, written into the
**older** slot (ping-pong), as one 4KiB-aligned write + fdatasync. Read protocol:
validate each slot independently (magic/header_crc/payload_crc/entry_count×40 ≤ slot
capacity); take the larger slot_gen.
**Capacity fallback**: if live entries × 40B + 64B exceed slot capacity, no
checkpoint is written (mount falls back to a full scan — always correct).

## 5. Open (mount) flow v2

1. Read the superblock (larger sync_seq wins) → tail, data_start.
2. Read checkpoints: if valid → load the index, resume scanning the log from
   covered_tail to tail (applying appends/tombstones); if invalid or absent →
   full scan from DATA_START.
3. During the scan, **stop at the first bad record**: truncate tail to that
   record's start (log-structured salvage: everything past the bad point is
   treated as unconfirmed and dropped).

## 6. Group commit and the durability horizon

In the log-structured design **the extent log IS the WAL**: an append hits the
device immediately (one self-contained 4KiB-aligned block) but only counts as
"confirmed" after `sync()` (= fdatasync of the data region + dual-superblock update).

- **Durability horizon** `confirmed_id`: the maximum extent_id written as of the
  last successful sync; extents with id ≤ confirmed_id MUST be readable after a
  crash (zero confirmed-write loss); ids above may exist or be salvaged away
  (callers must not rely on them).
- **Group-commit policy** (implementation side, not format):
  `set_commit_policy(max_pending)` — auto-sync once max_pending appends have
  accumulated since the last sync (one fdatasync amortizes a whole batch of
  writes = group commit).
- **Auto-checkpoint**: a checkpoint is written automatically every CKPT_INTERVAL
  (=8) syncs (amortizing index-serialization cost).

## 7. Write-path invariants

- Appends always happen at tail; within one sync batch, header+data travel in the
  same 4KiB-aligned write I/O.
- Confirmation order: extent data fdatasync succeeds → only then "confirmed" →
  only then is the superblock tail advanced. The inversion (superblock pointing
  at un-flushed extents) must never occur; scan salvage bounds the worst case at
  losing unconfirmed writes.
- O_DIRECT preferred; filesystems that don't support it (e.g. tmpfs) fall back to
  buffered I/O, declared in the open log.

## 8. Constants

```
BLOCK_SIZE       = 4096
DATA_START       = 64 MiB
CKPT_SLOT_A_OFF  = 8 KiB
CKPT_SLOT_B_OFF  = 32 MiB
CKPT_SLOT_CAP    = 32 MiB - 8 KiB   (same capacity definition for both slots)
CKPT_INTERVAL    = 8                (auto-checkpoint every 8 syncs)
EXTENT_DATA_MAX  = 4 MiB
SB_MAGIC         = b"PORFS_SB"
EXT_MAGIC        = 0x4558_5431
CKPT_MAGIC       = 0x3150_4B43
FORMAT_VERSION   = 2
```

Planned for v3 (not in v2): free-space extent table (for GC, introduced by P27).
