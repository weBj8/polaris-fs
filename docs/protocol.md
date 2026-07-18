# PolarisFS wire protocol v2 (PROTOCOL_VERSION = 2)

> Contract file: change this document and bump the version BEFORE changing
> code. Covers the network protocol only; the on-disk extent-device format
> is a separate contract (`format.md`). Introduced by P7.

## 1. Transport and framing

- TCP, one connection per client–chunkserver pair.
- Frames are length-prefixed: `u32 LE payload_len` + `payload_len` bytes.
- `payload_len` ≤ MAX_FRAME = **8 MiB** (an extent payload of up to
  `EXTENT_DATA_MAX` = 4 MiB plus envelope slack). A peer sending a larger
  frame is disconnected.
- The payload is one `Request` or `Response` value encoded with **bincode 2
  (`config::standard()`, serde)**: enums as `u32` variant index followed by
  fields in declaration order, integers little-endian, `Vec<u8>`/`String` as
  `u64` length + bytes. Field order and types below are the contract; serde
  attribute games that change the encoding are contract changes.

## 2. Messages

`Request`:

| # | Variant | Fields | Meaning |
|---|---|---|---|
| 0 | `Hello` | `protocol_version u16`, `client_nonce u64` | First frame on a new connection. |
| 1 | `WriteExtent` | `write_id u128`, `inode u64`, `logical_offset u64`, `data Vec<u8>`, `next Option<SocketAddr>` | Append one extent record; when `next` is present, chain-replicate it to that secondary. |
| 2 | `ReadExtent` | `extent_id u64` | Read one extent back, CRC32C-verified. |
| 3 | `Tombstone` | `extent_id u64` | Logically delete one extent (idempotent). |
| 4 | `Sync` | `next Option<SocketAddr>` | Durability barrier (group commit); when `next` is present, confirm the secondary too. |
| 5 | `Stats` | — | Store counters. |

`Response`:

| # | Variant | Fields | Answers |
|---|---|---|---|
| 0 | `HelloAck` | `protocol_version u16`, `server_nonce u64`, `started_unix u64` | Hello; `VersionMismatch` error + close if unsupported. |
| 1 | `WriteAck` | `extent_id u64`, `replica_extent_id Option<u64>` | WriteExtent. |
| 2 | `ReadAck` | `data Vec<u8>` | ReadExtent. |
| 3 | `TombstoneAck` | — | Tombstone. |
| 4 | `SyncAck` | `confirmed_id u64`, `replica_confirmed_id Option<u64>` | Sync. |
| 5 | `StatsAck` | `device_size u64`, `tail u64`, `live_bytes u64`, `extent_count u64`, `confirmed_id u64` | Stats. |
| 6 | `Error` | `code ErrorCode`, `message String` | Any request. |

`ErrorCode`: 0 `BadRequest` · 1 `NotFound` · 2 `Corrupt` (CRC mismatch or
salvage failure) · 3 `StoreFull` · 4 `VersionMismatch` · 5 `Internal` · 6
`ReplicaUnavailable`.

## 3. Op semantics

- **Hello** must precede all other ops; anything else first → `BadRequest`
  and close. Nonces are for debugging only (no auth in v1 — static
  membership on a trusted network).
- **WriteExtent** is *idempotent per server lifetime*: the server remembers
  `write_id → extent_id`; a retry with the same `write_id` returns the
  original `extent_id` without appending again. The dedup map is in memory
  and **lost on server restart** (documented limitation; see §4). Rules:
  `data` non-empty and ≤ 4 MiB, else `BadRequest`; device full → `StoreFull`.
  A `WriteAck` means the extent is appended to the log device but is **not
  yet durable** — durability requires Sync. With `next`, the primary forwards
  the identical write to the secondary and returns `WriteAck` only after both
  servers append it. The forwarded request always has `next = None`, so a
  two-node chain cannot loop.
- **ReadExtent** verifies the extent's CRC32C before returning data;
  mismatch → `Corrupt`. Unknown or tombstoned id → `NotFound`.
- **Tombstone** is idempotent: tombstoning an already-tombstoned extent is
  a no-op (`TombstoneAck`); unknown id → `NotFound`.
- **Sync** performs the store's group commit (data fdatasync + dual
  superblock). After `SyncAck { confirmed_id }`, every extent with
  `extent_id ≤ confirmed_id` MUST be readable after a crash (same
  durability horizon as `format.md` §6). With `next`, the primary returns
  `SyncAck` only after both nodes confirm their durability horizons. A
  secondary failure returns `ReplicaUnavailable`; callers must not treat the
  local append as confirmed.

## 4. Reconnect semantics (normative)

- The client connects lazily (first op, or after a disconnect) with
  exponential backoff: 50 ms → ×2 → capped at 2 s, no retry limit (a caller
  cancels via its own timeout).
- On a connection failure, an in-flight op gets **one transparent retry**
  over a fresh connection — safe because every op is idempotent (§3) — and
  only fails to the caller when the retry also dies on the wire. Connect
  attempts themselves back off without a limit, so ops issued while the
  server is down stay pending until it returns or the caller times out;
  ops never hang without the connection actually being attempted.
- Op-level retry safety after a reconnect: **WriteExtent** retries are safe
  within the same server lifetime (write_id dedup, §3). Across a server
  restart a retried write MAY append a duplicate extent with a new
  extent_id; both copies are valid immutable records, and the layering rule
  for P8+ is: the metadata layer records at most one winning extent_id per
  logical position, so duplicates are harmless garbage (reclaimed by GC,
  P27). **Read/Tombstone/Sync/Stats are naturally idempotent.**
- Pipelining: multiple requests may be in flight; responses carry the
  request's position implicitly by protocol — v1 processes ops on the
  server in arrival order on a single store thread, so responses are in
  order. (Request ids were deliberately left out of v1; add them with a
  version bump if the store ever executes concurrently.)

## 5. Placement topology

Topology is static client-side membership input, not a wire message: each
chunkserver endpoint is tagged with its rack and chassis. The P10 replicated
placement policy ranks members with rendezvous hashing, then selects the
highest-ranked secondary in a different rack from the primary. A replicated
layout therefore requires at least two racks; deployment configuration must
provide truthful physical topology.

## 6. Evolution

Unknown variant indices → `BadRequest` + close. New ops are added by
bumping PROTOCOL_VERSION and documenting them here first; `Hello` version
negotiation lets mixed-version peers fail fast.
