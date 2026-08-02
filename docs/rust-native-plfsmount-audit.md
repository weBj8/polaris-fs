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
Meta-FUSE directory and trash-path handles are opaque `u64` tokens resolving
to `Arc<DirBuf>`/`Arc<PathBuf>` in never-reissued registries; release removes
the token while in-flight readdir/read/write calls hold strong refs, and stale
tokens fail with `EBADF` instead of dereferencing freed memory.

Verification for this wave:

- `cargo test --release -p plfsclient`: 16 passed;
- `cargo test --release -p plfsmount --lib`: 95 passed after the meta-FUSE handle registry wave;
- `cargo test --release -p plfsmount --bin plfsmount`: 2 passed;
- `cargo test --release -p plfsbdev`: build and 0 tests passed;
- required-FUSE-path workspace release build: passed;
- `cargo fmt --all` and `git diff --check`: passed;
- CI run `30742916264`: gates, workspace tests, release build, cluster smoke,
  container build, and GHCR publish all passed;
- deployed image `45ff7343310a` digest
  `sha256:18e10073afd52a9d3160c4e32404bc8f626f45cb2fee5d07a4d1178d1b451556`
  (plfsmount-only update; other services not restarted; post-deploy concurrent
  read/write smoke passed);
- redeployed image `782bb278208d` digest
  `sha256:64ca97aad72cd845b97dc689baf8e599394aaf8f89975e7d58aa69f9249982f9`
  after CI run `30768186487` (plfsmount-only; stale FUSE mount unmounted on
  host first; concurrent 4-way read/write smoke passed).

The dirbuf wave removed the raw table/free-list, variable-tail malloc blocks,
per-slot pthread lock/condition protocol, and the internal C ABI exports
`dirbuf_cleardata`, `mfs_readdir_readmore`, and `mfs_readdir_next`. Workspace
search found no consumers of those internal translation artifacts; true
libfuse callback symbols and signatures remain unchanged. Release now removes
the token before close, holds in-flight state through `Arc`, serializes complete
readdir replies, waits for an active fetch, and rejects stale handles. It also
fixes the supplementary-group leak and validates a nonempty master response
pointer before copying at the FFI boundary.

Residual mount work is explicit: readdata/writedata remain shared raw
backends, but their synchronization is now Rust-owned. Their job
queues use typed Rust ownership directly; worker-thread lifecycle (spawn,
counters, termination wait/join) uses `std::thread` plus a Rust
`Mutex`/`Condvar` pool per module; the global inode/hash locks and all
per-inodedata mutex/cond pairs use `std::sync::Mutex`/`Condvar` with
same-thread guard-slot helpers (1:1 site parity verified against C).
mastercomm global locks and per-thread request record mutex/cond pairs
likewise use Rust guard-slot helpers; timedwaits are relative monotonic
(documented divergence from CLOCK_REALTIME abstime). Its nop/receive threads
use `std::thread` JoinHandles and the per-thread record lives in a Rust
thread-local whose Drop runs the C key-destructor protocol. readdata's
ranges scratch storage is a thread-local owned `Vec`. Supplementary groups now use `Arc<[u32]>` ownership from the cache
through every `mfs_fuse` caller; only immediate backend FFI borrows remain.
The finfo registry/state is now typed Rust ownership; its `ReadDataHandle` and
`WriteDataHandle` are narrow RAII adapters whose Drop methods call the existing
backend end functions. Remaining mount-reachable raw state is narrow:
malloc-owned structures are gone from readdata/writedata/mastercomm (all
structs and buffers are `Box`/arena/thread-local owned; raw links remain only
inside intrusive lists whose endpoints are owned), the sole surviving libc
allocation pair is the iovec array behind the fixed 3-arg `read_data_free_buff`
ABI (count not recoverable at free time), and the callback-facing raw FUSE APIs
stay at the libfuse boundary. The separate bdev data path (NBD daemon, not
reachable from plfsmount runtime) keeps its own transpiled copies for later
waves. Shared IO safe-core migration of structure internals (turning the
intrusive lists into typed collections) is the remaining deep work.

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
uses Rust thread-local ownership. readdata/writedata/mastercomm no longer use
pthread mutexes, condition variables, thread-spawn/join, or pthread-key TLS;
their structs and buffers are Box/arena/thread-local owned, and the iovec
array is now a leaked `Box<[iovec]>` consumed by the fixed 3-arg
`read_data_free_buff` ABI.

## plfsbdev (NBD daemon) wave

The same ownership migration was completed across the plfsbdev runtime
closure (`squeue`/`workers`/NBD session threads, `mfsioint` synchronization
and buffers, `mfsioint_lookupcache`, `plfsbdev` daemon allocations, `mfsio`,
`strerr`):

- `SQueue<T>` typed queue with RAII job wrappers; dead transpiled pcqueue copy
  deleted; NBD control/send/recv threads and the worker pool use std threads
  with JoinHandle/Mutex/Condvar.
- `mfsioint` fdtab/usemask are owned Vecs with C doubling growth; per-file
  locks and rwcond are std Mutex/Condvar with guard-slot emulation
  (index/pointer-match misuse panics); dbuff is a leaked Box with
  length-tracked free; lookup-cache bucket locks are a const-init std Mutex
  array with entry Condvars; daemon config strings and packet buffers are
  CString/Vec/Box owned.
- Verified libc retention with in-code comments: variable-size protocol
  structs (`nbdrequest`, `mfsacl`), `getline`/`__getdelim` internally
  realloc'd buffers, and the `mfs_set_defaults` strdup ↔ takeover pairing.
- C quirk frozen: `password_read` rejects passwords of exactly one
  non-newline character (C strip-loop decrements before testing).
- plfsbdev has no runtime NBD test harness; verification is `cargo test -p
  plfsbdev` (16 unit tests), clean rebuild, and gates. Baseline re-frozen to
  `wrapping_ops plfsbdev ge 186`.

## External boundary ledger (mount runtime closure)

Verified libc/libfuse ownership protocols intentionally kept, with in-code
annotations:

- `plfsmount.rs` mfsopts five string fields (masterhost, masterport, bindhost,
  proxyhost, subfolder): libfuse `fuse_opt_parse` writes `%s` templates into
  struct offsets and free+strdup's them on repeated options (verified against
  libfuse3 disassembly; a CString conversion reproduced a double-free).
- `plfscommon::processname` environ copy: process bootstrap (annotated
  boundary, AGENTS.md pattern).
- `plfsclient::readdata` iovec array: fixed 3-arg `read_data_free_buff` ABI
  cannot recover the count at free time.
- libfuse/libc/POSIX syscall surface (fuse_*, sockets, sigmask, tcp*).
- `mfsopts.password`: CString::into_raw intentional leak matching C.
- plfscommon::charts is not in the mount closure (CGI-only); the separate
  plfsbdev NBD daemon keeps its own transpiled copies.
