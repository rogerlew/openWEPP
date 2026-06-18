# openWEPP Engine Roadmap

Status: living — **canonical**, **forward-only planning queue**
Last updated: 2026-06-18
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
are closed. **MAGPARITY01** completed the per-OFE runoff magnitude adjudication:
the H2637 71% `runvol` is not an `INV-RUNOFFPART-028`, area-scaling, or export
defect. **STAGE2-LATQCC-H2637-MAGNITUDE** then traced WB19 per-substep operands
and found no equation, withdrawal, conductivity-override, active-depth, or
`drfc` formula defect; REFINTENT001 landed the ratified `ksatadj`
source-intent saturation-fraction algorithm (a correct defect closure) — **but it
is byte-inert on H2637 (`ksatadj = 0` there), so it does NOT close the FARPOINT01
71% flag.** STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE then proved base `ksat` is
byte-live and found a source-intent defect in vertical `wb18_perc_ssc` 200 mm
normalization. BASECOND01 closed that defect: vertical `ssc` is now harmonic
below the top 200 mm interval while hourly H2637 `wb19_lateral_ssh` remains
arithmetic from `ksat*anisotropy`. The H2637 no-UI rerun was aggregate-inert
(`runvol_pct_precip` remained `71.0036550031206`), so the remaining 71% flag
must be re-disposed after source-intent conductivity closure, not chased by
making `ui_ssh` harmonic. On
the perf track, PERFOPT01, PERFIDX03B, and PERFIDX04 captured
the read-side clone/lookup levers; PERFIDX05 was held because write/guard id work
is dual-write-bound under the read-mirror design; PERFIDX06 measured the
PERFIDX04 endpoint at **73.12×** legacy no-UI on H2637. The next perf mechanism
is not more narrow id-table work; it is a scoped hot-path state-representation
redesign decision. PERFARRAY02 executed that scoped request/accessor pilot and closed
NO-GO: identity passed, but H2637 array-native cost was `817.810 us/OFE-day`,
above the `386 us/OFE-day` <=10x budget; ADR-0023 should not be ratified from that
evidence.
(Completed-rung detail and commits: [work-packages execution log](work-packages/README.md).)

---

## Queue

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| 1 | **Per-OFE runoff magnitude adjudication** | Decide if per-OFE runoff vs legacy (FARPOINT01: openWEPP 71% vs legacy 55.5% of precip on H2637) is expected Stage-2 divergence or a defect | A per-term verdict (expected vs defect-shaped follow-on) | ✅ **`MOFE-MAGPARITY01` complete 2026-06-18** — no `INV-RUNOFFPART-028`, area-scaling, closure, or export defect; 71% `runvol` decomposes to routed lateral/subsurface magnitude. Follow-on is Stage-2 `latqcc`/WB19 magnitude, not a fix. |
| 2 | **Monolith line-count split** | Behavior-preserving split by domain responsibility of the WARN-band files (measured 2026-06-18: **10 files >2000, 0 over the 3000 required threshold** — advisory hygiene, not the stale "3 files") | Target tier (4 files >2500) under 2000 WARN; bit-identical outputs | ✅ **`REFACTOR022` complete 2026-06-18** — the four target-tier files were split under 2000 lines, true pre-refactor HEAD identity passed with `anchor_mismatches = 0`, and the 2000-2500 tier remains deferred advisory WARN work. |
| 3 | **Stage-2 physics-magnitude** | Fidelity of deferred magnitudes vs external authority | Magnitude correctness, judged against the closed + routed balance with comparator as flag | ✅ **`STAGE2-LATQCC-H2637-MAGNITUDE` complete 2026-06-18** — WB19 `latqcc` equation and operand-bound checks passed on selected H2637 high-magnitude rows; no openWEPP defect or defect-closure handoff. Verdict: `CONTRACT-GAP`; closing it routes to item 4 (reference-implementation-intent authority), not an external benchmark. |
| 4 | **Reference-implementation-intent authority + `ksatadj`/SC-SUBHYD-001** | Establish **ADR-0024** that for empirical forest models with no external physical authority, the legacy reference-implementation **intent** (algorithm) is a valid `SC-*` A0 anchor — **distinct from** legacy binary *behavior* (A6 flag, ADR-0017) — then apply it: extract the `ksatadj` intent from `wepp-forest_260430_baseline/src/{infpar,input}.for`, anchor it in `SC-SUBHYD-001`, and re-adjudicate openWEPP vs the *intent* | ADR-0024 ratified; `SC-SUBHYD-001` `ksatadj` anchor + invariant; `CORRECT` (close the FARPOINT01 71% flag) or `OPENWEPP-DEFECTIVE` (defect-closure ExecPlan) | ✅ **complete 2026-06-18** — **ADR-0024 ratified**; `SC-SUBHYD-001` v33 `INV-SUBHYD-032` + `REF-SUBHYD-KSATADJ-INTENT` authored and Claude-reviewed (both sides of the `sat_frac` divergence verified against source). Verdict `OPENWEPP-DEFECTIVE`: openWEPP forms `sat_frac = Σθ/Σul` vs source-intent `avsat/(avpor·avcpm)`. Fix routes to item 5; FARPOINT01 stays open until it lands. |
| 5 | **`REFINTENT001-KSATADJ-SATFRAC` defect closure** | Rebuild the WB14 `ksatadj` operand lineage so `sat_frac` is formed per `SC-SUBHYD-001#INV-SUBHYD-032` source intent: rock-corrected `avpor*avcpm` denominator, total-water + `avsm15` residual numerator, the two `avsat` caps, top-two-tillage weighted averaging, not `sum(theta)/sum(ul)` | `INV-SUBHYD-032` satisfied; non-aliased tests where surrogate differs from intended formula; determinism preserved; re-run H2637 + close the FARPOINT01 71% flag by source-intent conformance | ✅ **complete-with-correction 2026-06-18** (`REFINTENT001-KSATADJ-SATFRAC`) — source-intent `sat_frac` fix landed (correct, gate-clean, non-aliased-tested; valuable for `ksatadj=1` soils). **But Claude review found it byte-inert on H2637** (`ksatadj = 0`; WAT SHA identical pre/post), so it does **not** close FARPOINT01 — flag re-opens. The 71% is base-conductivity-driven → item 6. |
| 6 | **H2637 base lateral/percolation conductivity adjudication** | The H2637 71% lateral magnitude is driven by the **base soil conductivity** (`Ke`/`ssc`, soil-file `ksat` + the 200 mm runtime-layer normalization), **not** `ksatadj` (which is off for H2637). Adjudicate that conductivity lineage under `SC-SUBHYD-001` / `SC-INFILE-SOIL-001`, same intent-vs-behavior discipline | Per-term verdict on the base-conductivity lineage (`CORRECT`/`OPENWEPP-DEFECTIVE`/`CONTRACT-GAP`); resolve or re-route the FARPOINT01 71% flag | ✅ **`STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE` complete 2026-06-18** — base `ksat` is byte-live (`ksat_x0.9` changed WAT/PASS checksums and magnitude outputs). Verdict `OPENWEPP-DEFECTIVE`: vertical `wb18_perc_ssc` split-layer normalization is arithmetic but source intent is inverse-conductivity/harmonic (`117.955408` vs `270.8259 mm/h` on H2637 layer 3). Hourly `wb19_lateral_ssh` remains arithmetic and must be preserved. |
| 7 | **Post-BASECOND01 H2637 magnitude disposition** | Re-dispose the remaining FARPOINT01 H2637 magnitude flag after `ksatadj` source-intent and vertical `ssc` source-intent corrections have landed and proven H2637 aggregate-inert | Verdict: `CONTRACT-GAP` / external-authority gap, or a new in-envelope defect with contract authority before any production edit | Planned follow-on; do not re-edit vertical `ssc` or make hourly `ui_ssh` harmonic without new authority. |

(MOFE01 + FARPOINT01 closed hillslope water-routing closure through 19 OFEs; the
remaining items are separate follow-on mechanisms.)

---

### 1. Per-OFE runoff magnitude adjudication ✅ (complete 2026-06-18)

FARPOINT01 closed the >10-OFE routing **conservation** but surfaced a
**magnitude** divergence: on H2637 openWEPP routes 71 % of precip to the outlet
vs legacy's 55.5 % (both bounded; legacy with_ui is the q-cap-broken 127.7 %).
Adjudicate whether the per-OFE runoff magnitude is expected Stage-2 divergence or
a defect-shaped follow-on, judged against the already-closed routed balance
(comparator a flag, ADR-0017). Package: `MOFE-MAGPARITY01`.

MAGPARITY01 verdict: no transfer, closure, area-scaling, or export defect.
The 71% `runvol` decomposes to a small local surface residual plus routed
upstream lateral/subsurface flow. STAGE2-LATQCC-H2637-MAGNITUDE verdict: the
WB19 `latqcc` equation and operand-bound checks pass, with no openWEPP defect;
the remaining bounded delta is an absolute lateral-flow magnitude authority gap.

### 2. Indexed runtime-surface — hot-path redesign ▶️

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
PERFIDX05 code was **discarded** (record kept). **PERFIDX06** *(complete 2026-06-18)*
re-measured the PERFIDX04 endpoint and legacy on the same H2637 fixture: openWEPP no-UI
`666.82s`, legacy no-UI median `9.12s`, primary ratio **73.12×**; with-UI ratio
**57.84×**. Verdict: **≤10× is not closed and is not reachable by more narrow id-table
work under the current read-mirror design**; ≤5× is not plausible without a deeper
hot-path state redesign. The next perf mechanism is a deliberate array-authoritative or
fixed-index hot-path state scoping package that avoids both the PERFIDX03 export seam and
the PERFIDX05 dual-write ceiling.
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
