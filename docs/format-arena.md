# ChunkArena On-Disk Format — v2 (binding contract)

Status: **v2, binding**. Change this document and bump `FORMAT_VERSION` BEFORE
changing code. Source of truth: `docs/design.md` §6; where the design doc is
ambiguous this document is the resolution. All multi-byte integers are
**little-endian** unless stated otherwise. `crc32c` = CRC-32C (Castagnoli), as in
`crc32fast`.

**v2 changes vs v1** (bounded boot, review P0-3; slot-class fallback, review
P1-5): a checkpoint region between the bitmap copies and the slot region with a
`CLEAN_CLOSE` superblock flag — a clean close persists the in-memory index +
bitmaps, and the next open loads them instead of scanning every slot header
(dirty shutdown or any validation failure falls back to the v1 full scan); and
S-class Puts fall back to L-slots before reporting OutOfSpace. v2 and v1 arenas
are mutually unreadable (`Error::UnsupportedVersion`); upgrade = re-mkfs
(docs/runbook.md).

## 1. Region layout

```
┌────────────┬────────────┬───────────────┬──────────────────┬────────────────────┐
│ Superblock │ Superblock │ Bitmap region │ Checkpoint (v2)  │ Slot region        │
│ A(4K) off0 │ B(4K) 4096 │ (2 copies+crc)│ index + bitmaps  │ L-slots, then S    │
└────────────┴────────────┴───────────────┴──────────────────┴────────────────────┘
```

A ChunkArena is a single regular file (sparse image) or block device. All offsets
below are derivable from superblock fields; readers MUST use the superblock values,
not recomputed constants.

| Region | Offset | Size |
|---|---|---|
| Superblock A | 0 | 4096 |
| Superblock B | 4096 | 4096 |
| Bitmap copy A | `bitmap_region_offset` (= 8192) | `bitmap_copy_bytes` |
| Bitmap copy B | `bitmap_region_offset + bitmap_copy_bytes` | `bitmap_copy_bytes` |
| Checkpoint (v2) | `bitmap_region_offset + 2*bitmap_copy_bytes` | `checkpoint_bytes` = max(1 MiB, `total_bytes`/1024), 4096-aligned |
| L-slot i (0-based) | `slot_region_offset + i * l_slot_size` | `l_slot_size` |
| S-slot j (0-based) | `slot_region_offset + l_slot_count*l_slot_size + j * s_slot_size` | `s_slot_size` |

## 2. Superblock (4096 bytes)

| Offset | Size | Field |
|---|---|---|
| 0 | 8 | magic = ASCII `GFARENA1` |
| 8 | 4 | `format_version` u32 = **2** |
| 12 | 4 | `flags` u32: bit0 `PUNCH_OK`, bit1 `DISCARD_OK`, bit2 `BLOCK_DEVICE`, bit3 `CLEAN_CLOSE` (v2, §3.5) |
| 16 | 4 | `l_slot_size` u32 = 1048576 (1 MiB) |
| 20 | 4 | `s_slot_size` u32 = 65536 (64 KiB) |
| 24 | 8 | `l_slot_count` u64 |
| 32 | 8 | `s_slot_count` u64 |
| 40 | 8 | `bitmap_region_offset` u64 (= 8192) |
| 48 | 8 | `bitmap_copy_bytes` u64 (per copy incl. crc, padded to 4096) |
| 56 | 8 | `slot_region_offset` u64 (1 MiB aligned) |
| 64 | 8 | `total_bytes` u64 (image/device size at mkfs) |
| 72 | 16 | `arena_uuid` (16 bytes, RFC 4122 layout) |
| 88 | 8 | `created_at` u64 (unix seconds) |
| 96 | 8 | reserved, zero |
| 104 | 4 | `crc32c` u32 over superblock bytes [0, 104) |
| 108 | 3988 | zero padding |

- Written once at mkfs; A and B are identical copies. Read A; on magic/crc failure
  read B; both bad ⇒ `Error::BadSuperblock`. In v2 the only post-mkfs update is
  `CLEAN_CLOSE` (§3.5) — always written to both copies.
- `PUNCH_OK`: sparse-image backend supports `fallocate(FALLOC_FL_PUNCH_HOLE |
  FALLOC_FL_KEEP_SIZE)` (probed at mkfs, §6).
- `DISCARD_OK`: block-device backend supports `BLKDISCARD` (probed at mkfs).
- `BLOCK_DEVICE`: backend is a block device (vs. regular sparse file).

## 3. Bitmap region

Bit i of the L-bitmap = 1 ⇒ L-slot i allocated (**hint only**); S-bitmap likewise.
Bitmap payload = L-bitmap (`ceil(l_slot_count/8)` bytes, padded to 8) followed by
S-bitmap (same rule). Each copy = payload followed by u32 crc32c of the payload,
padded with zeros to `bitmap_copy_bytes` (multiple of 4096).

- The **slot header is the source of truth; the bitmap is a hint**. Bitmaps are
  flushed lazily (clean close, and opportunistically); they are fully rebuilt from
  slot headers at every open that takes the scan path (§5).

## 3.5 Checkpoint region (v2)

`checkpoint_bytes` = max(1 MiB, `total_bytes` / 1024), 4096-aligned; located
immediately after bitmap copy B (offset and size derivable from the superblock,
like every other region). Header (64 bytes):

| Offset | Size | Field |
|---|---|---|
| 0 | 8 | magic = ASCII `GFCPNT01` |
| 8 | 8 | `gen` u64 (close counter; diagnostics) |
| 16 | 8 | `entry_count` u64 |
| 24 | 4 | `bitmap_len` u32 (bitmap payload bytes that follow) |
| 28 | 4 | `payload_crc32c` u32 over `bitmap_len` bitmap bytes + `entry_count`×48 entry bytes |
| 32 | 4 | `header_crc32c` u32 over header bytes [0, 32) |
| 36 | 28 | reserved, zero |

Entry (48 bytes): `chunk_id`(16) · `version` u64 · `payload_len` u64 ·
`payload_crc32c` u32 · `class` u8 (0 = S, 1 = L) · reserved(3) · `slot_no` u64.

**Clean close**: write header + bitmap payload + all index entries into the
region, fdatasync, then set `CLEAN_CLOSE` in BOTH superblocks and fdatasync
again. If the payload does not fit the region, close skips the checkpoint (the
flag stays clear).

**Fast boot**: `format_version` = 2 with `CLEAN_CLOSE` set ⇒ read and validate
the checkpoint (magic, both crcs, payload fits the region); on success verify
16 pseudo-random slots against the loaded index (any mismatch ⇒ treat the
checkpoint as absent), then CLEAR `CLEAN_CLOSE` in both superblocks
(+fdatasync) BEFORE accepting writes and skip the header scan entirely. Any
validation failure ⇒ the v1 full scan. The flag ordering is the safety
property: the checkpoint is durable before the flag is set, and the flag is
cleared before any new write — a crash anywhere lands on the scan path.

## 4. Slot header (first 64 bytes of each slot)

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | magic u32 = `0x47534631` (ASCII "GFS1") |
| 4 | 2 | `header_version` u16 = 1 |
| 6 | 2 | `flags` u16: bit0 `SEALED` |
| 8 | 16 | `chunk_id` (16 bytes, UUIDv7 RFC 4122 layout; nil UUID forbidden) |
| 24 | 8 | `version` u64 (caller metadata; immutable per chunk_id) |
| 32 | 8 | `payload_len` u64 (0 ..= slot payload capacity) |
| 40 | 4 | `payload_crc32c` u32 (of payload bytes only) |
| 44 | 4 | `header_crc32c` u32 (of header bytes [0, 44)) |
| 48 | 16 | reserved, zero |

- Payload occupies `[slot_offset + 64, slot_offset + 64 + payload_len)`.
- Payload capacity: L-slot = `l_slot_size - 64` = 1,048,512; S-slot = `s_slot_size - 64`
  = 65,472.
- A header is **valid** iff: magic matches, `header_version` == 1, reserved bytes
  are zero (ignore — v1 writers zero them; readers do not check), `header_crc32c`
  matches, `payload_len` ≤ capacity, `chunk_id` ≠ nil.

## 5. Invariants & reconciliation (headers are truth)

At `open`, either load the v2 checkpoint (§3.5 fast boot) or — dirty shutdown,
missing/invalid checkpoint — scan every slot header sequentially and rebuild
state:

| On-disk observation | Conclusion |
|---|---|
| valid header, `SEALED` set | slot **allocated**; insert into index; set bitmap bit |
| valid header, `SEALED` clear | interrupted/reserved Put ⇒ slot **free**; clear bit |
| invalid header / zeroed slot | slot **free**; clear bit |

After the scan both bitmap copies are rewritten from the scan result (+crc, synced).
Duplicate `chunk_id` in two allocated slots (should never happen): the slot with the
higher (class, slot_no) wins, the other is treated as free and its bit cleared —
and the occurrence is logged as corruption evidence.

## 6. Operations

**mkfs**(path, total_bytes, l_fraction = 0.90, l_slot_size = 1 MiB, s_slot_size =
64 KiB): create a sparse file of `total_bytes` (or use the block device as-is);
compute geometry by fixpoint iteration (counts → bitmap size → `slot_region_offset`
→ counts; ≤ 3 iterations, counts only shrink) with `l_fraction` of the slot-region
bytes assigned to L-slots; zero + crc both bitmap copies; probe capabilities
(`PUNCH_OK` by an actual punch on the file, `DISCARD_OK` by an actual BLKDISCARD on
a sacrificial range at the device end — device only); write superblock B then A;
fdatasync. The slot region is NOT touched (stays sparse holes).

**Put**(chunk_id, version, payload):
1. Choose slot class: `payload_len` ≤ S capacity ⇒ S-slot, else L-slot (must fit L
   capacity, else `Error::Oversize`). Class-full fallback (v2): no free S-slot ⇒
   try an L-slot instead (an S payload in one L-slot); no free L-slot either ⇒
   `Error::OutOfSpace`. (The reverse — an L payload in an S-slot — is impossible
   by capacity.)
2. Find a free slot (bitmap scan starting at a random offset).
3. `pwrite` payload at slot+64.
4. `pwrite` the complete 64-byte header **with `SEALED` set**, one write, then
   `fdatasync`. (Single-write atomic publish; a torn header fails crc ⇒ free.)
5. Insert into the in-memory index; set the bitmap bit (lazy flush — reconciliation
   at boot is the safety net).
- Idempotency: `chunk_id` already present with same `version` and same
  `payload_crc32c` ⇒ success no-op. Same `chunk_id` with any different
  (version, crc) ⇒ `Error::AlreadyExists`. Chunk ids are UUIDv7 — CoW layers always
  allocate a fresh id; the arena never mutates a sealed slot.

**Get**(chunk_id) ⇒ (version, payload): in-memory index lookup, one `pread` of
64+payload_len, verify header valid + SEALED + payload crc; mismatch ⇒
`Error::Corrupt`. Unknown id ⇒ `Error::NotFound`.

**Stat**(chunk_id) ⇒ Option<{version, payload_len, payload_crc32c}>: index only, no
disk I/O.

**Delete**(chunk_id, version) ⇒ bool:
1. Index lookup: absent, or stored version ≠ requested ⇒ return false (idempotent;
   protects GC vs re-Put races).
2. Remove from index; clear bitmap bit (lazy flush).
3. Space reclamation: `PUNCH_OK` ⇒ punch `[slot_offset, slot_offset+slot_size)`;
   `DISCARD_OK` ⇒ BLKDISCARD same range; neither ⇒ bitmap-only free (monthly
   `sparsify` pass is ops tooling, out of scope here).
- No fsync on the delete path: a crash may resurrect the chunk at boot (header still
  intact); higher layers re-issue the exact-version Delete (GC is idempotent).

**List**() ⇒ iterator over the in-memory index: (chunk_id, version, payload_len,
payload_crc32c, class, slot_no). No disk I/O.

## 7. Crash consistency (design doc §6.5, normative here)

- Interrupted Put ⇒ un-SEALED or crc-bad header ⇒ slot freed at boot; the client's
  idempotent re-Put repairs.
- Bitmap corruption/loss ⇒ rebuilt from headers at every boot.
- Torn header write ⇒ crc mismatch ⇒ slot free. Payload is never trusted without a
  valid SEALED header.
- No journal: allocation is single-slot atomic publish (header last).
- Hole punch vs crash: delete is index-first; worst case a deleted chunk reappears
  and is re-deleted by the idempotent GC path. Space accounting is always
  reconstructible from headers.

## 8. Versioning

`format_version` = 1. Readers reject other versions with `Error::UnsupportedVersion`.
Reserved fields are written as zero; readers must not require them to be zero
(forward compatibility).
