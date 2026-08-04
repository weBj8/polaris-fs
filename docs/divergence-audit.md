# C → transpiled-Rust divergence audit

*Adapts `ZIG_RUST_DIVERGENCE_AUDIT.md` from the Bun PR #30412 audit set. That
audit catalogued structural divergences introduced by hand-porting Zig to
Rust. This one catalogues divergences introduced by **machine translation**
(c2rust 0.22.1) plus the semantic gaps between C and Rust that the transpile
preserves deliberately. Format kept: summary table → category notes →
risk-rated items with evidence, symptom, test coverage, recommended fix.*

## Summary

**Divergence classes: 4** where C/c2rust semantics do not map 1:1 to safe
Rust. Item counts are initial (P0 expands this audit into the full
punch-list).

| Risk | Count | dropped-definitions | keyword-mangling | aliasing-mutability | UB-preserving-semantics |
| --- | ---: | ---: | ---: | ---: | ---: |
| HIGH | 1 | | | 1 | |
| MEDIUM | 3 | 1 | | 1 | 1 |
| LOW | 3 | | 1 | | 2 |
| **Total** | **7** | **1** | **1** | **2** | **3** |

### Category notes

- **dropped-definitions** — c2rust drops definitions it cannot translate
  (e.g. `_Atomic` file statics inside `#if HAVE_ATOMICS`) while keeping
  their uses. Compiles only because a human re-added them; the class generalizes:
  any transpile regeneration must re-check for silently dropped items.
- **keyword-mangling** — C identifiers that are Rust keywords (`type`) are
  mangled `r#type` by c2rust; the stock bitfields derive panics on them.
  Mechanical, but load-bearing for regeneration.
- **aliasing-mutability** — the transpile forms `&mut` references and raw
  pointers to `static mut` globals and through callback `void*` contexts.
  Two live `&mut` to one allocation is UB **today**, whether or not current
  LLVM exploits it. This is our analogue of Bun's `noalias` class.
- **UB-preserving-semantics** — c2rust translates C operations into
  semantics-preserving-but-unsafe Rust (raw-pointer arithmetic, `transmute`,
  wrapping arithmetic). "Works in release" is not "defined behavior."

### Seeded items (P0 expands to the full audit)

| # | Category | Site | Divergence | Risk |
| --- | --- | --- | --- | --- |
| D1 | aliasing-mutability | 1,786 `static mut` sites tree-wide | `&mut`/`&` formed to `static mut` globals across the event loop; any two overlapping `&mut` = UB | HIGH (class) |
| D2 | dropped-definitions | 6 `_Atomic` statics, `plfsmount`+`plfsbdev` `mastercomm.rs`/`readdata.rs`/`writedata.rs` | definitions dropped by c2rust, re-added by hand; regression risk on every regeneration | MEDIUM |
| D3 | aliasing-mutability | `plfsclient` cache modules (`getgroups.rs` fixed at `35f00f4`; sibling caches un-audited) | re-entrant callback invalidating cached pointers — one latent UAF already shipped | MEDIUM (class) |
| D4 | UB-preserving-semantics | 11,498 `wrapping_*` sites | machine-inserted C overflow semantics; any "cleanup" to plain `+`/`-` silently changes overflow behavior | MEDIUM (anti-fix class) |
| D5 | keyword-mangling | `vendor/c2rust-bitfields-derive-0.22.1` patch | stock derive panics on `r#type`; vendored patch is load-bearing | LOW |
| D6 | UB-preserving-semantics | `transmute` sites (ungrepped at P0) | layout invariants unasserted; Bun shipped 148 layout asserts, we ship ~0 | LOW (class) |
| D7 | UB-preserving-semantics | variadic leaves (`mfslog`, `oplog`, `mfsio`, `changelog`, `plfsgui`) | `VaList` misuse is UB; nightly `c_variadic` API drift already bit once (`arg` → `next_arg` rename) | LOW |

Metalogger/GUI re-audit (2026-08-04): class-A mutable globals moved into
owned/atomic/mutex/thread-local state. Callback registries
and packet/request ownership no longer use intrusive raw-pointer globals.
`mfscgiserv_printf` remains the single GUI variadic leaf; fixed-signature logic
stays outside it. C-required wrapping remains explicit and is frozen at 15
(metalogger), 13 (GUI), and 854 (shared `plfscommon` sources).

---

## H1. `static mut` aliasing across the poll loop — the systemic class

**C original:** MooseFS daemons are single-threaded poll loops with module-
level globals (`static` in C). C permits taking multiple pointers to one
global freely; data races are impossible only because the loop is
single-threaded — and worker threads (chunkserver disk I/O, master
`lsmatoma` workers) break even that assumption.

**Transpiled Rust:** `static mut X: T` with `&mut X` / `&X` formed at use
sites, plus raw pointers stashed in callback contexts. Rust's rules: forming
two live `&mut X` (or `&mut` + `&`) to the same `static mut` is
**instant UB**, regardless of threads. With 1,786 sites *(verified-claims
VC-02)*, exhaustive per-site proof is not feasible by inspection — this is a
class, and gets a class fix.

**Symptom:** none observable today under the current toolchain — the Bun
lesson applies verbatim: "safe in current codegen" is one inlining-heuristic
change away from miscompiling, and Miri flags it immediately.

**Test coverage:** ❌ none — no Miri run, no ASM audit has been performed on
this tree. P0 adds the Miri gate.

**Recommended fix (systemic, per methodology #4):** classify all 1,786 in
`docs/facts/OWNERSHIP.tsv` (P0), then convert by class: scalars →
`AtomicXxx` (relaxed for stats counters, see VC-07); structs → `Mutex`/
`RwLock` or thread-confined `thread_local!` where the worker-thread split
allows; init-once → `OnceLock`. **Then re-run the per-site audit against the
converted code** (methodology correction #4): any `&mut` formed to a
remaining `static mut` after the class fix is a punch-list item, one by one.

---

## M1. Dropped `_Atomic` statics — regeneration trap

See verified-claims VC-07. The six re-added statics are the known instances;
the class fix is a regeneration checklist item: after any future
re-transpile, diff the symbol table against the previous tree for
used-but-undefined statics. **Gate:** build + `nm`-level check in the
regeneration runbook (`README.md` §Regenerating), not in per-PR CI.

## M2. Re-entrant callback invalidation in `plfsclient` caches

**Evidence:** commit `35f00f4` — groups-cache UAF crashing `opendir` for
root. The `plfsclient/` tree holds a family of similar caches
(`dirattrcache.rs`, `negentrycache.rs`, `symlinkcache.rs`, `xattrcache.rs`,
`chunksdatacache.rs`, `fdcache.rs`, `sustained_*`) following the same
pattern: cache pointer held across a network round-trip that can run
callbacks invalidating the cache.

**Symptom:** crash or stale data under re-entrant load; one instance shipped.

**Recommended fix:** P4 audits every `plfsclient` cache module against the
`getgroups.rs` fix pattern (VC-09). Class fix: cache handles become
generation-checked or `Rc`-owned so a held handle keeps the entry alive.

## M3. `wrapping_*` anti-fix class

11,498 machine-inserted `wrapping_*` ops encode C overflow semantics. The
risk is not the status quo — it is a future "cleanup" PR replacing them with
plain arithmetic (panics in debug, wraps in release: behavior divergence on
both). **Gate:** CI lint rejecting the *removal* of `wrapping_` without a
claim entry justifying the site's non-overflowing range.

---

## LOW items

| # | Site | Divergence | Recommended fix |
| --- | --- | --- | --- |
| L1 (D5) | `vendor/c2rust-bitfields-derive-0.22.1` | stock derive panics on `r#type`; vendored patch (`Ident::new_raw`) is load-bearing | P6: upstream the patch; until then never bump the dep |
| L2 (D6) | `transmute` sites tree-wide | layout invariants unasserted | adopt Bun's pattern: `const _: () = assert!(size_of::<T>() == N)` at every FFI-crossing or transmuted type; cheap, mechanical, compounding |
| L3 (D7) | variadic leaves (VC-10) | `VaList` misuse = UB; nightly API drift | confine `c_variadic` to the leaves; fixed-signature safe wrappers for all call sites; drop feature at Rust 1.99 |

---

## Working this audit

- This table is a **punch-list**: P0 expands it from the seed items to the
  full per-site audit (starting with the aliasing-hunt candidates).
- Closed items are **struck through** with the closing commit noted, never
  deleted ([methodology.md](methodology.md) correction #6).
- Every fix lands with (a) its reproducer green and (b) an independent
  "what new problem does this fix introduce?" review ([methodology.md]
  correction #5).
