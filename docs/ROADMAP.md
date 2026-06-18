# openWEPP Engine Roadmap

Status: living — **canonical**, **forward-only planning queue**
Last updated: 2026-06-16
Audience: all contributors
Owner: maintainers (Claude Code maintains this document)

This is the single authoritative roadmap for the openWEPP simulation engine — a
**forward-looking planning queue only.** Completed work is **not** recorded here; it
lives in the [work-packages execution log](work-packages/README.md). When a rung
closes, it is **removed** from this queue and its detail moves to that log. Other
locations (ADRs, backlog, agent memory) point here; when they disagree, this document
wins. See [§ Keeping this current](#keeping-this-current).

---

## How to read this queue

openWEPP is built **architecture-first** with **top-down science contracts** as the
correctness authority
([ADR-0011](decisions/0011-architecture-first-top-down-science-contracts.md)). Legacy
WEPP is a **flag, not a target**
([ADR-0017](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)).
Defects are closed with **Defect-Closure ExecPlans**
([ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md),
[defect_closure_execplans.md](defect_closure_execplans.md)).

**The ordering principle — closure, not magnitude, not comparator-match.** Each item's
acceptance target is **closure** (does it *conserve* — water/mass balance, bounds),
**not** whether forcing magnitude is physically perfect and **not** whether it matches
a legacy binary. Conservation is independent of magnitude, so the structure is closed
first; **magnitude/physics fidelity is judged last**, against an already-closed and
routed system, so magnitude error is never aliased with structural error. Every item
adds **one mechanism** on an already-closed foundation. Boundaries are **closure gates,
not calendar phases**.

**Current position:** single-OFE water-balance closure, frost
activation/conservation, single-OFE frost-depth heat-flow parity,
MOFE hillslope inter-OFE water-routing closure, the **openWEPP-native
totalwatsed3 CLI + closure** (WSHED01, the WBVAL06/6a deferral, closed
2026-06-14), and the **MOFE >10-OFE far-point demonstration** (FARPOINT01,
closed 2026-06-16 — openWEPP's three identities close at 19 OFEs past the legacy
ceiling; the frost `watbtm` double-count it surfaced was closed contract-first)
are closed. The next active mechanism is **per-OFE runoff magnitude
adjudication**; on the perf track, PERFHO01 attributed the ~80–110× high-OFE gap,
**PERFOPT01** landed its first optimization (complete 2026-06-16: ~1.15×,
bit-identical), and **PERFHO02** characterized the residual as hydrology
symbol-access/guard work plus secondary writeback-application overhead. Operator
set a **≤10× (≤5×)** target; **PERFARCH01** completed the indexed runtime-surface
design and ADR-0022 (ratified 2026-06-16). **PERFIDX01** (Stage 1) is complete — frozen registry + invariants proven
bit-identical. Its completeness audit found the *bounded* symbol universe is
~1.7M for H2637 (vs the ~6K assumed; ~3.6K actually used), so the dense-store
representation needs an **ADR-0022 refinement** (compact/sparse/partitioned)
before Stage 2 (`PERFIDX02`).
(Completed-rung detail and commits: [work-packages execution log](work-packages/README.md).)

---

## Queue

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| 1 | **Per-OFE runoff magnitude adjudication** | Decide if per-OFE runoff vs legacy (FARPOINT01: openWEPP 71% vs legacy 55.5% of precip on H2637) is expected Stage-2 divergence or a defect | A per-term verdict (expected vs defect-shaped follow-on) | ⏭️ **Next** (`MOFE-MAGPARITY01`) |
| 2 | **Indexed runtime-surface — read-side migration done; re-measure next** | Two dominant levers: the per-lane/day clone and the per-access `format!`+map-lookup. **`PERFIDX03B`** (✅) removed the **clone** via `std::mem::take` move. **`PERFIDX04`** (✅) removed the **lookup** (resolve-once `SymbolId` hot tables + indexed read-mirror), −24% on H2637. **`PERFIDX05`** (⏸️ HELD) attempted the *write/guard* side and **regressed −5.7%** — the dual-write cost (logical + mirror) exceeds the id saving; **structural ceiling** of the read-mirror design. Next: **`PERFIDX06`** re-measures vs ≤10× **before** any further write-side work | Bit-identical; read levers realized; actual legacy ratio measured | ⏸️ **`PERFIDX05` HELD** (code discarded, record kept) — bit-identical (Claude reproduced OFE2) but H2637 **−5.3–5.8%**; the prefix→range trap correctly stopped the one paying scan (decomposition overflow). ▶️ **`PERFIDX06` scaffolded, Codex-ready** (re-measure + actual legacy ratio + ≤10× verdict; prior arithmetic implies **~56–75×** still — measure like-for-like to confirm and decide reachable-incrementally vs redesign). ✅ `PERFIDX04` stands as the endpoint (H2637 673 s). *Irrigation stays deferred → `backlog/20260617-…`.* |
| 3 | **MOFE line-count split** | Behavior-preserving split of the 3 files that crossed 2000 lines | Each under 2000 WARN; bit-identical outputs | ▶️ follow-on (`REFACTOR022`) |
| 4 | **Stage-2 physics-magnitude** | Fidelity of deferred magnitudes vs external authority | Magnitude correctness, judged against the closed + routed balance with comparator as flag | ⏸️ **Deferred** |

(MOFE01 + FARPOINT01 closed hillslope water-routing closure through 19 OFEs; the
remaining items are separate follow-on mechanisms.)

---

### 1. Per-OFE runoff magnitude adjudication ⏭️ (next)

FARPOINT01 closed the >10-OFE routing **conservation** but surfaced a
**magnitude** divergence: on H2637 openWEPP routes 71 % of precip to the outlet
vs legacy's 55.5 % (both bounded; legacy with_ui is the q-cap-broken 127.7 %).
Adjudicate whether the per-OFE runoff magnitude is expected Stage-2 divergence or
a defect-shaped follow-on, judged against the already-closed routed balance
(comparator a flag, ADR-0017). Package: `MOFE-MAGPARITY01`.

### 2. Indexed runtime-surface — storage representation ▶️ (gated on ADR-0022 refinement)

PERFHO01 *(complete 2026-06-16)* characterized the ~80–110× H2637 wall-clock gap:
CPU-bound (`977.99/978.55` user s), **not** I/O or parquet, scaling
roughly linear-to-modestly-superlinear in OFE count (`b≈1.12`) — a large
**constant per-OFE-day cost**, not an explosive exponent. GDB-sampled dominant
cost: per-OFE-day symbol-keyed `BTreeMap<BoundarySymbol, BoundaryValue>`
runtime-surface clone/insert/remove/lookup + success-path writeback validation
(`11/15` samples); the WB13-string lead was tested and found **not** dominant.
Verdict: **not acceptable as-is**. **PERFOPT01** *(complete 2026-06-16)* landed
the first optimization — removed the per-OFE-day report-to-persistent-state +
climate-overlay surface clones and made writeback validation detail lazy — for
**~1.15×** on H2637 (`978.55→849.86 s`), **bit-identical** (`anchor_mismatches = 0`,
independently re-confirmed against a separate pre-opt baseline) and
determinism-preserving. The residual now sits in hydrology/transfer guards +
remaining lane-surface clone/drop → **`PERFHO02`** (next characterization). The
band's upper reaches (the `BTreeMap` data-structure replacement) were deliberately
not attempted under the strict bit-identity gate. Packages:
`work-packages/20260616-perf-high-ofe-hillslope-characterization-001/`,
`work-packages/20260616-perfopt01-runtime-surface-map-churn-001/`.

**PERFHO02** *(complete 2026-06-16)* sampled the optimized H2637 path and found
the residual in hydrology typed-symbol lookup/dynamic symbol construction/frost,
decomposition, and PL guard work (`13/20` GDB samples), with secondary
`apply_kernel_writeback` sort/allocation/insertion (`4/20`). After
`kernel.perf_event_paranoid=0` became visible, `perf record` confirmed the same
direction (`execute_persistent_scheduler_kernel_lifecycle` `96.24 %` children,
`apply_kernel_writeback` `12.46 %`, `compute_active_frost_coupling` `12.35 %`).
Output writers again had no sampled dominance.

**Decision (operator, 2026-06-16):** target **≤10× (≤5×)** vs legacy. Incremental
PERFOPT passes are Amdahl-capped well above 10× (PERFOPT01 = 1.15×; the cost is
distributed across every per-OFE-day `String`-keyed access, not a few excisable
functions). **PERFARCH01** completed the design and feasibility work:
`work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/`.
It chose a frozen run-scoped `SymbolRegistry`, sorted-order `SymbolId`, and
dense indexed state/flux storage, with proposed
[ADR-0022](decisions/0022-indexed-runtime-surface-representation.md). Prototype
storage operations were 109.85× faster for clone, 219.16× faster for pre-resolved
lookup, and 115.77× faster for update batches. <=10× is plausible if staged
implementation migrates about 89-90% of current elapsed time out of string-keyed
surface mechanics; <=5× remains aspirational.

**PERFIDX01** *(complete 2026-06-16)* landed the frozen registry + invariants
(sorted-id, equality, completeness with 0 post-freeze unknowns, bit-identical) —
but its completeness audit surfaced that the *bounded* symbol universe is
**~1.7M for H2637** (vs the ~6K the dense-store premise assumed; ~3.6K actually
used), RSS nearly doubling. Lookups stay O(1) at any size, but a dense
`Vec<Option>` indexed by the global 1.7M-`SymbolId` would make the per-OFE clone
(the *dominant* cost) **larger and slower** than the BTreeMap it replaces. **ADR-0022 Amendment 1** (2026-06-16) refined this: the store is sized to the
working set (sparse `Vec<(SymbolId, value)>` primary; dense global-`SymbolId` `Vec`
rejected), with the global registry/sorted-id unchanged. **PERFIDX02** *(complete 2026-06-16)* **cleared the gate**: on real H2637 surfaces
(4,087 present entries) the sparse `Vec<(SymbolId, value)>` clone is **54–70× faster**
than `BTreeMap::clone` (Codex caught + fixed an LLVM clone-elision bench artifact),
the registry tightened **1.7M→44,746** (reachable `ncut`/`ncycle` bound, 0 unknowns),
the shadow equals the BTreeMap (mismatch 0), and outputs are bit-identical (shadow
dormant in production). The dominant lever (clone) is proven; the *total* ≤10× still
awaits the Stage-6 re-measure. **PERFIDX03** *(executed-hold 2026-06-17)* attempted
the flip: the diverse-management registry gate passed (0 unknowns), but the flip
**regressed OFE5 +41.9%** (27.01→38.34 s) — the compat seam clones the sparse store
then **exports it back to a full `BTreeMap` per lane/day** for the kernel, and that
export dwarfs the clone win. Codex held (disabled the flip; no-flip 26.80 s); the
uncommitted code was **discarded** and the record kept. This proves the flip and the
read-side migration are *coupled*: the flip can't win while the seam reads via a full
export. **PERFIDX03B** *(complete 2026-06-17)* closed the blocker differently than a
re-flip: instead of making the indexed store authoritative for reads, it eliminated
the per-lane/day **clone** with `std::mem::take` **move semantics** — the logical
surface is moved into execution, refilled from the report, and the indexed mirror is
refreshed afterward as Stage-4 groundwork. OFE5 **38.34→25.45 s** (−5.1% vs the 26.82
baseline); the full anchor ran and passed (H2637 both UI + 1–5 ladder); outputs are
bit-identical. (The `pass.parquet` byte difference was disproved as a regression:
the same baseline binary emits 3 distinct `pass.parquet` hashes across identical runs
— pre-existing parquet-*container* non-determinism — and the decoded rows are
identical including order.) The win is honestly modest because the **read** seam still
resolves `BoundarySymbol` and the mirror is maintained but not yet consumed.
**PERFIDX04** *(complete 2026-06-17)* closed that second lever: resolve-once
`HotSymbolTables` (built once from the frozen registry) plus an indexed read-mirror
**carried beside** the logical surface and **dual-applied** on each accepted writeback
and same-day OFE transfer (in-place mutation — *no* full-map export, so the PERFIDX03
trap stays closed), migrating the hot read families (climate, frost, WB18/19, PL,
MOFE hourly; **irrigation excluded** — deferred/inert). The mirror is a
**non-authoritative read shadow**; the logical `BTreeMap` remains the commit authority,
so bit-identity is the proof the by-`SymbolId` reads equal the by-`BoundarySymbol` reads.
Result: **H2637 −24.3% / −25.2%** (888.92→673.29 / 894.98→669.75 s), OFE5 −14.3%
(OFE1 −4.4%, setup unamortized on a trivial single-OFE run); full anchor bit-identical
(Claude independently reproduced OFE2, which exercises the transfer-sync path); the
profiler shows hot `format!` collapsed to 0.01% self with id-table helpers in its place.
**PERFIDX05** *(HELD 2026-06-18)* attempted the **write/guard** side (apply-by-`SymbolId`,
consumer-boundary id-sets, failure-path tests) and came back **bit-identical but
−5.3–5.8% on H2637**. Root cause is **structural**, not incompleteness: every
writeback/transfer/guard must **dual-write** the authoritative logical `BTreeMap` *and*
the indexed mirror, and that cost exceeds the id-lookup saving on the write side (reads
won in PERFIDX04 precisely because a read touches only the mirror). The one scan whose
removal could have paid — the decomposition overflow scan — is blocked by the
prefix→range interloper-proof the package correctly declined to force. So the
write/guard-side migration is net-negative under the read-mirror design, and **PERFIDX04
appears to have captured most of the win available under it**. Per operator decision the
PERFIDX05 code was **discarded** (record kept) and the program **pivots to `PERFIDX06`**:
re-measure the actual legacy ratio on the PERFIDX04 endpoint and decide the ≤10× (≤5×)
verdict *before* any further write-side investment. If more is needed, the next lever is a
deliberate redesign choice (decomposition-scan-with-proof *iff* it beats dual-write, or
indexed-authoritative without the PERFIDX03 export seam) — not "finish Stage 5 as
specified," whose premise PERFIDX05 undercuts.
*(Irrigation stays deferred — `backlog/20260617-irrigation-management-gated-activation.md`;
it runs only when the management declares it and is out of scope for the perf migration.)*

### 4. Stage-2 Physics-Magnitude ⏸️ (deferred, judged last)

Fidelity questions deferred by the closure-not-magnitude principle until the structure is
closed and routed so the comparator can attribute error cleanly. They do **not** block
items 1–2.

| Item | What | Provenance | Backlog |
|---|---|---|---|
| Snow magnitude | `snowd.for` melt/settling/density/partition equation fidelity (CRM Ch. 3.7) | legacy physics adjudication | [backlog/20260605-snow-code-deferred-science-review.md](backlog/20260605-snow-code-deferred-science-review.md) (Stage 2) |

(Frost depth was a Stage-2 candidate; the FDMC01 verdict + the settle-vertical-before-routing
principle promoted it to active queue item 1 on 2026-06-07.)

---

## Keeping this current

This is a forward-only queue; it only works if it stays true.

1. **This file is canonical, and forward-only.** It contains what is *next* and
   *deferred* — never completed work. The [work-packages execution log](work-packages/README.md),
   ADRs, backlog, and agent memory reference it; they do not redefine the queue.
2. **When an item closes, remove it from the queue.** Move its detail/commits to the
   execution log and update the "Current position" line and `Last updated`. This is
   part of the closing package's documentation, not a separate chore. Do not let
   completed items accumulate here.
3. **A strategic decision is not bound until it lands here and in the active handoff.**
   A roadmap living only in an ADR or memory has no hook into the execution chain — to
   redirect work you must change both this file *and* what the active work-package
   handoff names as its next item. (Lesson from the HPHYS0314–0320 arc, where the
   snow-comparator relay continued for seven packages after the strategy had already
   changed elsewhere.)
4. **Deferred ≠ forgotten.** Stage-2 items carry a backlog pointer; promote a
   defect-shaped backlog entry when an item is deferred.

## Authority pointers

- [ADR-0011](decisions/0011-architecture-first-top-down-science-contracts.md) — architecture-first, top-down contracts, comparator-tier policy
- [ADR-0017](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) — comparator is a flag, not a target
- [ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md) — Defect-Closure ExecPlan conversion rule
- [defect_closure_execplans.md](defect_closure_execplans.md) — DC-ExecPlan authoring
- [specifications/science-contracts/README.md](specifications/science-contracts/README.md) — `SC-*` contract authority
- [work-packages/README.md](work-packages/README.md) — completed-work execution log
