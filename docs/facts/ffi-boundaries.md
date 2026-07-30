# FFI boundary inventory

P0 facts artifact. The **genuine** C-library boundaries of the transpiled
tree — where Rust code crosses into a system C library or is called back
from one. Everything else that looks like FFI (`extern "C" fn`, `#[no_mangle]`)
is a machine-translate artifact of c2rust preserving C's translation-unit
ABI inside a single process: 2,997 `#[no_mangle]` and 5,151 `extern "C"`
occurrences tree-wide, of which the table below is the complete genuine set.
Migration consequence ([porting.md](porting.md) type map): intra-crate
`extern "C" fn` becomes plain `fn`; only this page's rows keep the C ABI.

Evidence commands are reproducible; numbers as of commit `2e49c26`.

## Genuine boundaries

| Crate | C library | Direction | Sites | Evidence |
| --- | --- | --- | --- | --- |
| `mfsmount` | libfuse3 (≥3.17; CI builds 3.18) | Rust→C: session setup, `fuse_session_new_versioned` | `mfsmount/src/fuse_client/mfsmount.rs:93` (extern decl), `:1125` (`fuse_session_new_fn` compat wrapper), `:1138` (call) | `grep -n fuse_session_new_versioned mfsmount/src/fuse_client/mfsmount.rs` |
| `mfsmount` | libfuse3 | **C→Rust callbacks**: the two `fuse_lowlevel_ops` vtables | `mfsmount.rs:1311` (`mfs_oper`), `:1197` (`mfs_meta_oper`), struct def `:741` | every fn pointer in these tables is a re-entrant C→Rust entry point — aliasing-hunt class C |
| `mfsmaster` | zlib (`-l z`) | Rust→C: metadata (de)compression | `mfsmaster/build.rs` link line; users: `mfsmaster/src/mfsmaster/bio.rs`, `metadata.rs`, `bgsaver.rs` | `grep -l 'deflate\|inflate\|zlib' mfsmaster/src/mfsmaster/*.rs` |
| `mfschunkserver` | zlib (`-l z`) | Rust→C: chunk CRC helpers | `mfschunkserver/build.rs`; `mfscommon/crc.rs`, `mfschunkserver/replicator.rs`, `hddspacemgr.rs` | same grep |
| `mfsnetdump` | libpcap (`-l pcap`) | Rust→C: capture open/read | `mfsnetdump/src/mfsnetdump.rs:60–76` (`pcap_lookupnet`, `pcap_open_live`, `pcap_open_offline`, …) | `grep -n pcap_ mfsnetdump/src/mfsnetdump.rs` |
| `mfsmetalogger`, `mfsgui`, `mfsbdev` | — | none (libc only) | — | `build.rs` files carry no `rustc-link-lib` |

libc itself (malloc, sockets, poll, pthread) is omnipresent but is not a
migration boundary — it is replaced by std facilities module-by-module per
the porting type map.

## Kernel interfaces (not C-ABI FFI, listed for completeness)

- `mfsmount` ↔ Linux FUSE device (`/dev/fuse`) via libfuse3 — covered above.
- `mfsbdev` ↔ NBD kernel protocol via socket pair + `ioctl` — plain libc
  calls inside `mfsbdev/src/mfsclient/mfsio*.rs`; no external library.

## Re-entrancy note (feeds aliasing-hunt class C)

The libfuse3 row is the only boundary where C code calls **into** Rust on
threads we do not control (libfuse worker threads, depending on session
config). Every `fuse_lowlevel_ops` callback receives `userdata` as
`void*` — the `void*` callback-context pattern. P0 grep target:
`fuse_lowlevel_ops` field initializers at `mfsmount.rs:1197–1400` and every
`userdata` deref reachable from them.
