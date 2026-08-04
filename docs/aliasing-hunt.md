# Aliasing-mutability hunt — protocol and punch-list

*Adapts `NOALIAS_HUNT_REPORT.md` and `NOALIAS_SUSPECTS.md` from the Bun PR
#30412 audit set. Bun hunted LLVM `noalias` miscompiles: `&mut self` promising
exclusivity that the compiler exploited across re-entrant JS callbacks. Our
analogue is broader and more direct: the transpiled tree forms `&mut`
references to `static mut` globals and through `void*` callback contexts —
two live `&mut` to one allocation is language-level UB **regardless of what
LLVM does with it today**.*

**Precedent (ours, not Bun's):** the groups-cache use-after-free in `plfsmount`
(commit `35f00f4`) — a cached pointer held across a re-entrant operation that
freed it. Same fault family this hunt targets: pointer/reference held across
an invalidation point.

---

## Executive summary (seed state — P0 populates)

| Stage | Count |
| --- | --- |
| Candidates enumerated (pattern match) | 1,786 `static mut` sites + `plfsclient` cache modules + callback-context `void*` sites (ungrepped) |
| Survived adversarial triage | — P0 |
| Miri/ASM-verified `PROVEN` | — P0+ |
| `NOT_CACHED` (UB, not currently exploited) | — P0+ |
| `INCONCLUSIVE` | — P0+ |

**Seed finding:** 1 known-shipped instance (VC-09, `35f00f4`). The hunt
exists because one instance already escaped smoke testing into production.

---

## The hunt protocol (binding)

Four stages, in order. No stage may be skipped; stages 2–4 are where Bun's
equivalent hunt earned its rigor.

### Stage 1 — Enumerate candidates (pattern match)

Grep classes, each producing a candidate list with `file:line`:

| Class | Pattern | Seed count |
| --- | --- | --- |
| A: `static mut` access | `static mut` declarations + every `&`/`&mut`/`addr_of!` use | 1,786 decls (VC-02) |
| B: cache-pointer-across-callback | `plfsclient/*cache*.rs`, `fdcache.rs`, `sustained_*.rs`, `csdb.rs` — pointer/index held across `mastercomm` round-trips | ~14 modules (VC-06, M2) |
| C: `void*` callback contexts | event-loop registrations casting `*mut c_void` to concrete ctx | ungrepped (P0) |
| D: worker-thread shared state | `lwthread.rs`, `workers.rs`, `pcqueue.rs`, `squeue.rs` + their clients | 4 modules + clients |

### Stage 2 — Adversarial triage (2-vote, default-deny)

Two independent verifiers per candidate. The prompt is literal: **"Default
confirmed=false unless you verify against the transpiled source at
file:line."** A candidate survives only with an aliasing/invalidation
argument citing both the `&mut`-forming site and the second-access site. A
third verifier breaks ties.

### Stage 3 — Proof

- **Miri** for everything reachable: a Miri failure is `PROVEN` with no ASM
  needed (Miri detects the aliasing violation directly).
- **Release-mode x86-64 disassembly** for hot-path survivors Miri can't
  reach (FUSE callbacks): prove whether a `self.*`/global field is cached in
  a callee-saved register across an opaque re-entrant call — the exact
  technique in Bun's report (cache → opaque call → stale store without
  reload).

### Stage 4 — Classification and fix

| Class | Meaning | Action |
| --- | --- | --- |
| `PROVEN` | UB demonstrated (Miri trace or ASM) | fix before phase closes; reproducer becomes a regression test |
| `NOT_CACHED` | UB, not exploited by current codegen | **still punch-list** — "do not rely on current codegen" (Bun's exact wording); fixed by the systemic conversion, then re-audited |
| `INCONCLUSIVE` | cannot prove either way | IOU with named consumer phase |

**Systemic fix first** (methodology #4): the `OWNERSHIP.tsv`-driven
`static mut` conversion ([divergence-audit.md](divergence-audit.md) H1)
retires class A wholesale. **Then re-run the per-site audit against the
converted code** (methodology correction #4) — Bun fixed 22 of 23 and
shipped the 23rd; the re-audit is what catches ours.

---

## SUSPECT register

*Format kept from `NOALIAS_SUSPECTS.md`: sites that survive triage but are
not (yet) proven. One inlining-heuristic change away from exploitable.
Populated during P0–P4; seeded below with the class-B module list.*

### Class B — `plfsclient` cache modules (audit against the `getgroups.rs` fix pattern)

| Crate | Module | Pattern to check |
| --- | --- | --- |
| plfsmount | `fuse_client/dirattrcache.rs` | entry pointer held across master round-trip |
| plfsmount | `fuse_client/negentrycache.rs` | same |
| plfsmount | `fuse_client/symlinkcache.rs` | same |
| plfsmount | `fuse_client/xattrcache.rs` | same |
| plfsmount | `fuse_client/fdcache.rs` | same |
| plfsmount | `fuse_client/dentry_invalidator.rs` | invalidation vs. held references |
| plfsmount | `fuse_client/masterproxy.rs` | callback re-entrancy |
| shared (post-dedup) | `plfsclient/chunksdatacache.rs` | same |
| shared | `plfsclient/csdb.rs` | same |
| shared | `plfsclient/mastercomm.rs` | re-added `_Atomic` counters (VC-07) + callback contexts |
| plfsbdev | `plfsclient/mfsioint_lookupcache.rs` | same |

### Class D — worker-thread boundaries

| Crate | Module | Pattern to check |
| --- | --- | --- |
| plfschunkserver | `plfscommon/lwthread.rs`, `plfscommon/pcqueue.rs` | producer/consumer shared state, `&mut` across queue handoff |
| plfsbdev | `plfscommon/workers.rs`, `plfscommon/squeue.rs` | same |
| plfsmaster | worker modules touching metadata | `static mut` metadata accessed from >1 thread |

*(Classes A and C are generated, not hand-listed: A = the OWNERSHIP.tsv rows;
C = P0 grep for `*mut c_void` context registrations.)*

### Metalogger and GUI re-audit — closed 2026-08-04

`plfsmetalogger` and `plfsgui` no longer retain mutable raw-pointer ownership
in config lists, callback registries, protocol queues, request routes, or CGI
child tracking. Opaque callback handles remain integer/raw values only at the
daemon ABI seam; Rust-owned state never dereferences them. The six active
libc process globals (`stdout`, `stderr`, `optarg` in each daemon) are classified
`FFI`, and active ownership rows for these daemons contain no `UNKNOWN`.

---

## Record-keeping

- Every `PROVEN` item gets: Miri trace or ASM excerpt (as in Bun's report —
  cache instruction → opaque call → stale store), the fix commit, and the
  regression test. Items are struck through when closed, never deleted.
- Every fix passes the two-gate rule: reproducer green + independent
  "what new problem does this fix introduce?" review.
- The register and this protocol live in the repo permanently. Deleting the
  hunt record is how a methodology becomes unfalsifiable (Bun deleted theirs
  at merge; we keep ours).
