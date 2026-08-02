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
`chunksdatacache`, `chunkrwlock`, and `extrapackets`, plus mount
`dirattrcache`, `fdcache`, and stats/params file state. Mount callers no
longer declare or call the old `dcache_*`, `fdcache_*`, `inoleng_*`, or
`stats_term` internal ABI symbols. Directory cache blobs are owned copies;
its registry is `Weak<DirCache>` and its indexes are ordinary Rust maps.
`fdcache::acquire` returns an owned `FdEntry`, and `.stats/.params` handles
use `Arc<Mutex<Sinfo>>` with `Vec<u8>` buffers.

Verification for this wave:

- `cargo test --release -p plfsclient`: 12 passed;
- `cargo test --release -p plfsmount --lib`: 72 passed;
- `cargo test --release -p plfsmount --bin plfsmount`: 2 passed;
- `cargo test --release -p plfsbdev`: build and 0 tests passed;
- required-FUSE-path workspace release build: passed;
- `cargo fmt --all` and `git diff --check`: passed.

Residual mount work is explicit: `mfs_fuse` still contains the generated
`dirbuf`/`finfo` pthread lock and condition-variable protocol, `getgroups`
still exposes its groups blob at the FUSE/C boundary, and `pcqueue` plus
some FUSE metadata handles retain boundary-shaped ownership. These remain
next migration waves; this commit does not claim raw/pthread global zero.

## Completed thread wave

The first native-thread wave now uses `std::thread::Builder`, `JoinHandle`,
`Mutex`, `RwLock`/`Condvar` where needed, and standard sleep primitives in:

- `plfscommon::lwthread` (native worker spawn with inherited daemon signal mask);
- `plfscommon::delayrun` and `plfscommon::conncache`;
- mount dentry invalidation and sustained inode/parent/stats workers;
- mount symlink and xattr caches (Rust mutexes; xattr values use `Arc`).

POSIX signal-mask calls remain at the system boundary. These modules no longer
use pthread start-routine or pthread join protocols internally. Remaining
pthread occurrences are tracked as the next lock/data-path waves, especially
`oplog`, `negentrycache`, `mfs_meta_fuse`, and `plfsclient` request workers.
