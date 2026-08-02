# Rust-native plfsmount migration audit

Status: active. This document tracks the migration of `plfsmount` and the
workspace modules it actually depends on (`plfscommon` and `plfsclient`).
Behavior and exported daemon/FUSE contracts remain frozen during migration.

## Boundary rule

Raw pointers, `extern "C"`, and libc-style ownership are allowed only at a
thin boundary with a named external owner:

- libfuse callbacks and opaque FUSE request/session objects;
- `/dev/fuse`, kernel, libc, POSIX, and other system calls;
- published C ABI symbols that an external daemon or test harness consumes.

Rust modules inside the dependency chain must call Rust functions directly and
exchange Rust types. Internal objects must use ownership/borrowing, `Box`,
`Arc`, `Mutex`/`RwLock`, collections, and RAII. An ABI wrapper must not be the
interface between two Rust modules.

## Baseline at start

Collected from the tracked source trees before this migration wave:

| tree | files containing `extern "C"` | pointer tokens (`*mut`/`*const`) | `c_void`/`void*` tokens | `from_raw` sites |
| --- | ---: | ---: | ---: | ---: |
| `plfsmount` | 20 | 5,976 | 474 | 11 |
| `plfsclient` | 12 | 10,550 | 541 | 0 |
| `plfscommon` | 16 | 771 | 172 | 0 |

These are inventory counts, not completion metrics: they include external FFI
and generated declarations. Every remaining occurrence must be classified in
the boundary ledger before completion.

## Migration waves

1. **Boundary inventory:** map every internal `extern` declaration and raw
   ownership token to its defining module and callers.
2. **Rust-native shared primitives:** migrate the `plfscommon` modules used by
   mount (`pcqueue`, `delayrun`, clocks, logging, sockets, parsers) and expose
   direct Rust APIs.
3. **Client core:** migrate the `plfsclient` modules used by mount (I/O,
   caches, master communication, locks, stats) to direct Rust APIs and typed
   ownership.
4. **Mount caches and FUSE glue:** keep only the unavoidable FUSE callback
   adapter raw at the edge; make cache state and file/dir handles typed inside.
5. **Bootstrap and link graph:** remove internal C ABI declarations from the
   mount binary and call the Rust modules directly; verify release LTO symbols.
6. **Audit and verification:** rerun the inventory, update the external-boundary
   ledger, run tests/build/smoke/gates, and inspect the final diff.

## Known completed fixes carried into this wave

- `dirattrcache` registry walks hold the registry lock while using cache
  objects, preventing the confirmed `NodeIndex` use-after-free.
- meta-FUSE open/release paths preserve raw values across `fh` clearing and
  synchronize directory teardown with readdir.

Those fixes are compatibility evidence, not a substitute for the full
Rust-native migration.

## Current wave evidence

Typed direct Rust APIs now cover `plfsclient::inoleng`, `stats`, `csdb`,
`chunksdatacache`, `chunkrwlock`, `extrapackets`, and `pcqueue`, plus mount
`dirattrcache`, `dirbuf`, `fdcache`, and stats/params file state. Mount callers
no longer declare or call the old `dcache_*`, `dirbuf_*`, `fdcache_*`,
`inoleng_*`, or `stats_term` internal ABI symbols. Directory cache blobs are
owned copies; its registry is `Weak<DirCache>` and its indexes are ordinary
Rust maps. Directory read handles are opaque `u64` tokens resolving to
`Arc<DirSlot>`; credentials, supplementary groups, master response blocks,
cursor state, and cache handles are owned behind Rust `Mutex`/`Condvar` state.
`fdcache::acquire` returns an owned `FdEntry`, `.stats/.params` handles use
`Arc<Mutex<Sinfo>>` with `Vec<u8>` buffers, and file handles use a generation-
checked `Arc<FileInfo>` registry with Rust state guards and open waiters.

Verification for this wave:

- `cargo test --release -p plfsclient`: 16 passed;
- `cargo test --release -p plfsmount --lib`: 91 passed after moving pcqueue tests to plfsclient;
- `cargo test --release -p plfsmount --bin plfsmount`: 2 passed;
- `cargo test --release -p plfsbdev`: build and 0 tests passed;
- required-FUSE-path workspace release build: passed;
- `cargo fmt --all` and `git diff --check`: passed;
- CI run `30742916264`: gates, workspace tests, release build, cluster smoke,
  container build, and GHCR publish all passed;
- deployed image `7e4742d21cb9` digest
  `sha256:3df8ce060e99eac1bd1505961765cc219c00f644a89e5b65721f5e414186c7eb`.

The dirbuf wave removed the raw table/free-list, variable-tail malloc blocks,
per-slot pthread lock/condition protocol, and the internal C ABI exports
`dirbuf_cleardata`, `mfs_readdir_readmore`, and `mfs_readdir_next`. Workspace
search found no consumers of those internal translation artifacts; true
libfuse callback symbols and signatures remain unchanged. Release now removes
the token before close, holds in-flight state through `Arc`, serializes complete
readdir replies, waits for an active fetch, and rejects stale handles. It also
fixes the supplementary-group leak and validates a nonempty master response
pointer before copying at the FFI boundary.

Residual mount work is explicit: some FUSE metadata handles retain boundary-
shaped ownership, and readdata/writedata remain shared raw backends. Their job
queues now use typed Rust ownership directly; worker pthread state remains for a
later wave. Supplementary groups now use `Arc<[u32]>` ownership from the cache
through every `mfs_fuse` caller; only immediate backend FFI borrows remain.
The finfo registry/state is now typed Rust ownership; its `ReadDataHandle` and
`WriteDataHandle` are narrow RAII adapters whose Drop methods call the existing
backend end functions. This does not claim shared IO migration complete: raw
readdata/writedata APIs and their callback-facing resource semantics remain a
residual risk until those backends receive their own safe-core migration.

## Completed thread wave

The first native-thread wave now uses `std::thread::Builder`, `JoinHandle`,
`Mutex`, `RwLock`/`Condvar` where needed, and standard sleep primitives in:

- `plfscommon::lwthread` (native worker spawn with inherited daemon signal mask);
- `plfscommon::delayrun` and `plfscommon::conncache`;
- mount dentry invalidation and sustained inode/parent/stats workers;
- mount symlink and xattr caches (Rust mutexes; xattr values use `Arc`).

POSIX signal-mask calls remain at the system boundary. These modules no longer
use pthread start-routine or pthread join protocols internally. `mfs_fuse` has
no remaining pthread mutex/condition or pthread TLS protocol; ACL scratch data
uses Rust thread-local ownership. Shared IO backends and the separate bdev data
path retain pthread/raw state for later waves.
