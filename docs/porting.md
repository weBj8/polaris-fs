# c2rust-unsafe → safe Rust porting guide

*Adapts `PORTING.md` from the Bun PR #30412 audit set. That document governed
translating one Zig file to a Rust draft; this one governs converting one
c2rust-transpiled module into safe, idiomatic Rust.*

You are migrating one transpiled `.rs` module to safe Rust. Read this whole
document before writing any code. The goal of a migration unit is a module
that (a) compiles with `#![forbid(unsafe_code)]` or the tightest feasible
boundary, (b) behaves bit-identically to the transpiled original on the
protocol, disk, and FUSE surfaces, and (c) passes the default-deny verifier.

## Ground rules

- **Migrate in place, same file path.** `mfsmaster/src/mfsmaster/matocsserv.rs`
  stays `matocsserv.rs`. Reviewers diff old↔new side-by-side; do not
  restructure the tree while migrating.
- **Do not invent crate layouts.** The 7-crate layout
  ([rewrite-plan.md](rewrite-plan.md)) is fixed per phase. New shared code
  goes in the crate's own `src/`; cross-crate dedup is P6 work, not mid-phase.
- **No new dependencies without PR-level justification.** std first. Existing
  deps (`libc`, `c2rust-bitfields`, the crate's current set) second. No async
  runtime, no `tokio`/`futures` — the daemons are poll-loop state machines.
- **No behavior change.** If the transpiled code wraps on overflow, the safe
  code calls `wrapping_*` explicitly. If it relies on C integer promotion,
  the safe code reproduces the promotion. Surprising behavior is preserved
  and explained in a claim, never "fixed" silently. Suspected upstream bugs
  get a `divergence-audit.md` entry, not a drive-by fix.
- **`unsafe` retreats to boundaries, it does not vanish by fiat.** FFI to
  libfuse3/zlib/pcap and unavoidable raw-pointer seams keep `unsafe` blocks,
  each annotated `// SAFETY: <why the invariant holds>`. Everything else is
  safe code.
- **Leave `// MIGRATION-IOU: blocked_on: <crate>::<item>` for anything you
  cannot migrate confidently.** Don't guess. Flagging beats wrong code.
  Every IOU gets a ledger entry in [verified-claims.md](verified-claims.md).
- **Leave `// PERF(port): <idiom> — re-measure after migration`** wherever the
  transpiled code used a perf-relevant idiom (manual buffer management,
  intrusive lists, pointer arithmetic over slices) and the safe version uses
  the plain idiomatic form. Hot paths (chunk I/O, metadata ops, FUSE data
  path) are re-measured before the phase closes.
- **Match the transpiled structure.** Same fn names (already snake_case from
  c2rust), same field order, same control flow. Exception — borrow-checker
  reshaping: when faithful structure yields overlapping `&mut`, capture
  scalars into locals, drop the borrow, re-borrow. Do NOT reach for raw
  pointers to silence borrowck; leave
  `// PORT NOTE: reshaped for borrowck` so diff readers aren't confused.
- **Release semantics everywhere.** Tests and gates run `--release` or with
  `overflow-checks = off`. Never "fix" a debug-mode overflow panic by
  changing arithmetic; reproduce C's wrapping explicitly.

## Ownership pre-classification (mandatory)

Before migrating any struct or global, look it up in
`docs/facts/OWNERSHIP.tsv` (columns: `crate · file · item · transpiled_type ·
class · safe_type · evidence`). Use the `safe_type` column verbatim. Classes:

| Class | Safe form | When |
| --- | --- | --- |
| `OWNED` | `Box<T>` / `Vec<T>` / `String` | single owner, freed where allocated's owner dies |
| `SHARED` | `Rc<T>` / `Arc<T>` (+ `RefCell`/`Mutex` as needed) | shared ownership, e.g. chunk server registry entries |
| `BORROW` | `&'a T` / `&'a mut T` with explicit lifetime | non-owning reference with provable scope |
| `STATIC` | `&'static T` or `OnceLock<T>` / `LazyLock<T>` | lives for process duration |
| `THREAD` | thread-local (`thread_local!`) | per-thread state (worker threads in chunkserver/master) |
| `FFI` | raw pointer stays, confined to boundary | crosses to libfuse3/zlib/pcap |
| `UNKNOWN` | `Option<NonNull<T>>` + `MIGRATION-IOU` | could not classify — blocks phase exit if still UNKNOWN at phase end |

The TSV is pre-computed cross-file analysis; trust it over local guessing.
If you believe a row is wrong, fix the TSV (with evidence) and re-grep its
consumers — do not override it locally.

## Type map — c2rust idiom → safe Rust

| Transpiled form | Safe form | Notes |
| --- | --- | --- |
| `static mut X: T` | consult TSV: `AtomicXxx` for scalars, `Mutex<T>`/`RwLock<T>` for structs, `thread_local!` for per-thread, `OnceLock` for init-once | 1,898 sites; this is the single biggest class. See aliasing-hunt for the aliasing risk of `&mut X` formed from `static mut` |
| `*mut T` / `*const T` struct field | TSV `safe_type` column | never put a lifetime on the struct until the TSV says `BORROW` |
| `*mut T` fn param (intra-crate) | `&mut T` / `Option<&mut T>` if null is a live value | null checks at the boundary become `Option` |
| `*mut c_void` callback context | concrete `*mut Ctx` at the FFI seam, `&mut Ctx` inside the callback | the event loop's `void* arg` pattern; type the context |
| `extern "C" fn` used only intra-crate | plain `fn` | the C ABI seam is a transpile artifact unless a symbol is genuinely exported (check `#[no_mangle]` and build.rs link lines) |
| `#[no_mangle] pub extern "C" fn` called from C libs (fuse3 callbacks) | stays `extern "C"`, body becomes safe wrapper around safe internals | FFI boundary; `// SAFETY:` on the body |
| C arrays `[T; N]` + index arithmetic | `[T; N]` with slice ops; `Vec<T>` if the C code realloc'd | pointer-walking loops become iterators or explicit indices |
| `libc::malloc/free` pairs | `Box`/`Vec`; if the allocation crosses FFI, `Box::into_raw`/`from_raw` at the boundary only | pairing must be visible in one function |
| sentinel values (`-1`, `NULL`, `0` as error) | `Option<T>` / `Result<T, E>` at module boundary; sentinel preserved at the protocol/disk boundary | wire format is frozen — sentinels on the wire stay sentinels |
| `c2rust_bitfields::BitfieldStruct` | stays (vendored derive, `vendor/c2rust-bitfields-derive`) until P6; new code uses explicit masks | the derive patch is load-bearing (`Ident::new_raw`); do not "simplify" bitfields mid-phase |
| variadic fns (`VaList`, `mfslog`, `oplog`, `changelog`) | safe wrapper with fixed signature per call-site family; `c_variadic` confined to the leaf that formats | VaList sites: `mfscommon/mfslog.rs` (all crates), `mfsclient/oplog.rs`, `mfsclient/mfsio.rs`, `mfsmaster/changelog.rs`, `mfsgui/mfsgui.rs` |
| `core_intrinsics` uses (`likely`/`unlikely`/`abort` etc.) | `std::hint::likely`/`unlikely` where stable, else drop the hint; `std::process::abort` | removal of `core_intrinsics` is a per-module win toward P6 stable |
| `wrapping_add` etc. | keep `wrapping_*` | 11,498 sites are machine-inserted C-semantics; do not "clean up" to panicking or `+` — that changes overflow behavior |
| `transmute` | `#[repr(C)]` + field access, or `bytemuck`-free explicit byte conversion | every remaining `transmute` needs a `// SAFETY:` citing the layout invariant, plus a `const _: () = assert!(size_of …)` gate |

## Marker conventions (all CI-grepped)

| Marker | Meaning | Gate |
| --- | --- | --- |
| `// SAFETY: <why>` | on every remaining `unsafe` block/fn | CI fails on bare `unsafe {` without a SAFETY comment within 2 lines |
| `// MIGRATION-IOU: blocked_on: X::Y` | explicit IOU | CI counts; ledger required; phase exit = 0 or transferred |
| `// PORT NOTE: reshaped for borrowck` | control flow differs from transpiled original | review checklist |
| `// PERF(port): <idiom>` | perf-relevant change, re-measure | hot-path phases re-measure before close |
| `#![forbid(unsafe_code)]` | module fully migrated | permanent; removing it requires PR justification |
| `// ponytail: <what was skipped>` | deliberate simplification with named ceiling | survives review only with the upgrade path named |

## Definition of done — one module

1. Compiles with the tightest unsafe boundary achieved.
2. Default-deny verifier: cites transpiled `file:line` + migrated `file:line`
   for every behavior-bearing change; `confirmed` only with evidence.
3. Behavioral snapshot diff empty (protocol trace / FUSE ops / on-disk bytes,
   per the module's surface).
4. Miri clean (module-level) where the module is fully safe.
5. unsafe counter decreased by the module's baseline; no new bare `unsafe`.
6. TSV rows touched: no `UNKNOWN` left behind; IOUs ledgered.
7. Merge commit, not squash; PR links the claims and audit items it closes
   (items are **struck through**, never deleted).
