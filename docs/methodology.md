# Methodology — lessons from Bun PR #30412, applied to the polaris-fs migration

*Condensed from Michael Lady's five-round adversarial investigation of Bun PR
#30412 (the AI-orchestrated Zig→Rust port of a ~1M-line codebase, squash-merged
2026-05-14). Source gists:
[lessons](https://gist.github.com/michaellady/7e63223d5d72d9ad18a03efa1f376aae),
[full investigation](https://gist.github.com/michaellady/7d552137fb1e37ab9bf637e450016c25).
This document restates the lessons as **binding rules for the polaris-fs
migration** (c2rust-transpiled MooseFS 4.59.2, ~498K LOC of unsafe Rust, 7
crates, 8,727 unsafe sites). Where a lesson names a Bun artifact, the
polaris-fs counterpart is given inline.*

> **Sourcing convention (kept from the original).** Claims labeled
> *(bun — source)* are verified facts about Bun's PR from the investigation.
> Claims labeled *(polaris — file:line)* are verified against this repo and
> cross-referenced in [verified-claims.md](verified-claims.md). Bridging
> claims are marked *(inference)*.

---

## What worked in the Bun port — we adopt all seven

### 1. Fact-first, plan-second

Bun extracted an adversarially-verified fact corpus (1,355 lines, 200+ claims,
each cited to `file:line`, surviving 3-vote review) **before** writing the
architecture plan. The plan was a *derived* artifact of agreed-upon source
facts. When agents disagreed mid-port, the tiebreak referenced an immutable
peer-verified fact instead of re-litigating from source *(bun —
rust-rewrite-verified-claims.md)*.

**polaris-fs rule:** [verified-claims.md](verified-claims.md) is written and
reviewed before any phase plan that depends on it. Each claim uses the
four-field schema `FACT (source-cited) → RUST (translation) → PERF/GATE
(testable target) → SRC (file:line)`. No claim without a `SRC` line.

### 1b. Pre-classification before code generation

Bun pre-classified every Zig pointer field into an ownership class in a
2,253-row `LIFETIMES.tsv`, with a per-row evidence citation, and hardcoded a
grep of the TSV into the implement-agent prompt. Lifetime decisions were
pre-decided, not re-derived per agent *(bun — `docs/LIFETIMES.tsv`, deleted
pre-merge)*. This was the only *specification-as-data* artifact in the whole
methodology.

**polaris-fs rule:** Phase P0 produces `docs/facts/OWNERSHIP.tsv` — every
`static mut` (1,898 sites *(polaris — grep, see verified-claims VC-02)*) and
every struct field holding a raw pointer, classified into
`OWNED → Box`, `SHARED → Rc/Arc`, `BORROW → &'a`, `STATIC → &'static`,
`FFI → raw ptr stays`, `UNKNOWN → Option<NonNull<T>> + IOU`. The porting
prompt for every module greps this TSV; local guessing is forbidden.

### 2. Default-deny on verification

Bun's tiebreak prompt was literal: *"Default confirmed=false unless you verify
against .zig."* Verifiers defaulted to **refuted** unless they could cite both
source `file:line` AND target `file:line` AND an observable divergence
*(bun — phase-b2-cycle.workflow.js:107)*. Cost asymmetry: a false-positive
flag wastes one agent-round; a false-negative ships a bug.

**polaris-fs rule:** every migration-acceptance prompt and review checklist
contains the literal sentence: **"Default confirmed=false unless you verify
against the transpiled source at file:line."** The verifier must prove the
migrated code is equivalent, not fail to prove it broken.

### 3. ASM-verified UB hunting

Bun didn't stop at "lint flags 277 suspicious patterns." It went to
release-mode x86-64 disassembly to *prove* 23 miscompiles, and classified 46
more as `NOT_CACHED` — "safe in current codegen, still UB, one
inlining-heuristic change away from miscompiling" *(bun —
NOALIAS_HUNT_REPORT.md)*.

**polaris-fs rule:** the aliasing hunt ([aliasing-hunt.md](aliasing-hunt.md))
follows the same funnel: pattern-match candidates → 2-vote adversarial triage
→ release-mode ASM confirmation for survivors → `PROVEN` / `NOT_CACHED` /
`INCONCLUSIVE` classification. `NOT_CACHED` sites are still punch-list items;
current codegen is not a defense.

### 4. Systemic fix at the codegen/type layer, not just per-site patches

For the `noalias` class, Bun flipped the codegen default (`&self` + interior
mutability for all host functions) rather than patching 69 sites individually
*(bun — generate-classes.ts, R-2 Phase 2/3, verified on origin/main)*.

**polaris-fs rule:** every bug class gets a systemic fix first — a type
wrapper, a macro, a lint, or a crate-wide convention (see
[divergence-audit.md](divergence-audit.md) for the class list: `static mut`
globals, wrapping arithmetic, `&mut` from raw pointers, variadic logging).
Per-site patches come second.

### 5. Compile-time gates at scale

The merged Bun tree contains 148 `const _: () = assert!(...)` layout asserts —
~5 lines each, zero runtime cost, each blocking a whole class of
cross-language drift *(bun — git grep on origin/main)*.

**polaris-fs rule:** the cheapest gate that catches a class of bug ships
repeatedly. Standing gates (all in CI, all blocking):

- **unsafe-site counter** per crate: grep count may only decrease (baseline in
  [rewrite-plan.md](rewrite-plan.md) §0).
- `#![forbid(unsafe_code)]` at the top of every fully migrated module —
  24 characters, permanently prevents regression.
- `MIGRATION-IOU` comment counter: reported per PR; must trend to zero within
  a phase.
- Behavioral smoke: one-command cluster test (master + chunkserver + FUSE
  mount, md5-verified write/read) on every PR.
- Miri on migrated safe modules; ASAN/TSAN builds for the full cluster
  (remembering: release mode or `overflow-checks = off` — the transpiled code
  relies on wrapping arithmetic *(polaris — README.md "Release mode is
  required")*).

### 6. Bounded repair loops with explicit caps and zero leftover

Every Bun phase had a round cap (12–100). An agent that couldn't fix something
wrote `todo!("blocked_on: X::Y")` — an explicit IOU consumed by a later phase.
At merge, unresolved IOUs numbered **zero** *(bun — git grep)*. "Agent runs in
a loop forever" is the #1 failure mode of multi-agent orchestration; the
recipe is hard cap + IOU mechanism + a phase that consumes the IOUs.

**polaris-fs rule:** every phase in [rewrite-plan.md](rewrite-plan.md) §7 has
a hard cap. A site that can't be migrated within budget gets
`// MIGRATION-IOU: blocked_on: <crate>::<item>` and a ledger entry. A phase
does not close with unconsumed IOUs unless each is explicitly transferred and
recorded.

### 7. Branch ecosystem as orchestration substrate — preserved

Bun's PR was actually 32 `claude/*` sub-branches, 87 merge points, 5,512
first-parent commits, and 49 self-correcting Revert commits *(bun — git log
--first-parent)*. The Reverts are evidence the methodology caught and rolled
back its own bad merges.

**polaris-fs rule:** migration work happens on per-module branches merged with
**merge commits, never squash**. The branch graph is a load-bearing
methodology artifact.

---

## What didn't work in the Bun port — we adopt the corrections

### 1. Spec enforcement decayed toward merge

Bun's PORTING.md prescribed a `PORT STATUS:` trailer in every `.rs` file; the
merged tree satisfied it in 1 of 1,448 files. The spec said so; no gate
enforced it *(bun — git grep origin/main)*.

**Correction:** every convention in [porting.md](porting.md) that says "must"
has a corresponding CI check. A convention without a gate is a wish. If it's
not worth gating, it doesn't say "must."

### 2. `git pull -X ours` as synchronization primitive

Dozens of agents resolving conflicts by "prefer the puller" can silently drop
concurrent edits on shared files, with no record of what was dropped *(bun —
phase-d/e workflow files)*.

**Correction:** shared infrastructure files (`mfscommon/`, the duplicated
`mfsclient/` tree, workspace-level `build.rs`, `OWNERSHIP.tsv`) are owned by
one lane at a time. Module partitions are by file; files that can't be
partitioned are serialized, not merged-blind.

### 3. No semantic-equivalence gate; tests were a moving target

Bun's build-green gate (`cargo check` = 0 errors) plus the existing test suite
as oracle meant: behavior classes not covered by tests were invisible, and
three commits in the final 48h *recalibrated tests to the new implementation*
*(bun — e99311e584, 591214fd67, 78d74e84dc)*.

**Correction:** the oracle is the **old binary's behavior**, snapshotted
before migration: protocol traces (master↔chunkserver↔client), FUSE operation
results, and on-disk artifacts (chunk files, metadata image) captured from the
unmigrated daemons. Per-module flip is gated on the snapshot diff being empty.
`cargo build` green is necessary, never sufficient.

### 4. The 23 PROVEN miscompiles got 22 fixes; one instance escaped

`handle_reading` in Bun's SSL wrapper shipped with `&mut self` and no launder
while its three sibling methods — listed in the same audit cluster with
identical prescribed fix language — all got the fix *(bun — direct
re-verification on origin/main)*. The systemic codegen fix landed; the
per-site list was not re-run against the new code.

**Correction — the single most important rule in this document:** after every
systemic/class-level fix lands, **re-run the per-site audit against the new
code**. The re-audit list is regenerated from the new tree, not reused from
the old one. Instances escape; at least one will be in a path the systemic
change doesn't cover. Budget the re-audit pass at the same weight as the fix.

### 5. Agents find well, repair poorly — three auto-fixes were themselves buggy

Bun's post-merge security PR (36 findings) dropped 3 agent-authored auto-fixes
because each introduced a new bug of an adjacent class (an SSL-leak fix that
introduced a UAF; a dedup that stored a non-`'static` view in a `'static`
field; a precheck that added dead gating) *(bun — PR #30722 body)*. A human
reviewer caught all three.

**Correction:** trust agents to find, distrust agents to repair. Every fix
commit passes two gates: (a) the reproducer of the original problem turns
green; (b) an independent reviewer (separate agent context or human) answers
"what new problem does this fix introduce?" — an empty answer does not merge.

### 6. The audits were deleted and the PR was squash-merged

Bun deleted all six audit documents and the TSV in the commit immediately
before merge, then squash-merged 5,512 commits into one line. The most
rigorous AI orchestration examined left no preserved trail of its own rigor —
the methodology became unfalsifiable *(bun — 10cfa0b237, 23427dbc12)*.

**Correction:** nothing in `docs/` is ever deleted. Closed items are struck
through, not removed. Migration PRs merge with merge commits. The branch
graph, the audit docs, the IOU ledger, and the closed-item history are what
make this methodology peer-reviewable.

### 7. No human approving review at >1M-LOC scale

The investigation's closing opinion: one scoped human review of the audit
punch-list would likely have caught the escaped `handle_reading` miscompile
before merge rather than 8 days after *(bun — investigation, opinion-labeled)*.

**Correction:** each phase closes with a scoped human review of that phase's
punch-list (divergence audit + aliasing hunt items touched), not of the whole
diff. The review is of the *audit artifacts*, which is what makes it cheap
enough to actually happen.

---

## The meta-lesson

*The methodology works at the class level and leaks at the instance level.*

Every Bun failure — the escaped miscompile, the 36 post-merge findings, the 3
buggy auto-fixes — is the long tail of class-level work. The audit caught the
class; the codegen fix addressed the class; the per-site pass closed 22 of 23
instances. polaris-fs has already produced its own instance-level evidence:
the groups-cache use-after-free in `mfsmount` (fixed in commit `35f00f4`) was
a latent MooseFS bug that survived machine translation, behavioral smoke
testing, and real usage until it crashed `opendir` for root.

The fix is not "less AI" and not "less rigor." It is class-level rigor —
fact corpus, ownership pre-classification, systemic fixes, cheap gates —
**plus** an equally-funded per-instance re-audit pass that runs after each
systemic fix lands. That second investment is the one the Bun team under-made.
We make it.
