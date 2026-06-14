# MOFE Staged Increment Plan — Dispatch Artifact

Status: active (governs all MOFE01 implementation dispatches)
Author: Claude Code, 2026-06-12
Template: FDHP01 `d3-staged-increment-plan.md` (proven over 13 increments;
agent memory `staged-increment-port-template`). Companion scope authority:
`mofe-routing-port-scope.md` (produced by increment M-A).

## Universal rules (every increment)

- **Conservation hard stops** (all three, once defined by the contract in
  M-B): per-element identity, inter-OFE transfer identity, hillslope-total
  identity — at the FDHP01-era noise floor. Any regression stops the
  increment; fix or back out within it.
- **Single-OFE anchor**: the 7 single-OFE arboreal-dendrite hillslopes (and,
  at major boundaries, the 43-prefix algebraic-radium cohort) must stay
  bit-identical/at-noise vs the pre-increment boundary in any increment
  whose scope does not touch single-OFE paths — the deterministic
  bit-identical gate is the cheapest strongest check (FDHP01 increment-A
  lesson).
- **Comparator posture**: legacy is a weak, per-OFE-count-calibrated flag
  (see package.md). Divergence from legacy is adjudicated ONLY through the
  conservation identities. No tuning toward legacy numbers at any count.
- Contract-first within each increment; red tests before production edits;
  commit each completed increment under `executed-hold` until the
  acceptance increment; truthful evidence labels.
- Subagent requirement per package.md §4a block (comparator_suite_runner
  REQUIRED for heavy runs; record evidence if unavailable).
- Diagnostics are env-gated and removed before commit (Da/Df pattern).

## Increment M-A — characterization + routing scope (no production edits)

**Objective:** ground everything in measured reality before any code.

1. **openWEPP current behavior**: run the as-is binary on the full
   arboreal-dendrite cohort. Record per OFE count: does multi-OFE execute;
   what do `UpStrmQ`/`SubRIn`/`QOFE`/per-OFE WAT rows carry; does anything
   already route (the `runon_input` carryover seam in
   `hydrology_phase_runoff_reconciliation.rs:268-326` — what is it now?);
   where does per-element closure break today.
2. **Legacy per-OFE-count closure measurement** (the comparator-trust
   calibration): from the on-disk `H*.wat.dat` outputs, compute legacy's
   own per-element and hillslope-total closure per OFE count (1→5). This
   turns the operator's "legacy degrades with OFE count, typically above
   ~10 OFEs" into a measured curve over the reachable range. Expected
   shape (verify, don't assume): clean at 1-5, which would make legacy a
   usable flag for this package; cross-reference the wepppy MOFE triage
   defect families. **`pw0.slp` (15 OFEs) is the watershed-representative
   profile, not a hillslope run (operator, 2026-06-12) — it is not part of
   the cohort.** The >10-OFE defect demonstration is a named follow-on on
   a high-OFE substrate or the watershed step.
3. **Routing scope artifact** (`mofe-routing-port-scope.md`, the FDHP01
   scope-artifact shape): legacy state-machine map with lifecycle columns —
   the per-plane loop, runoff → run-on hand-off symbols (`runoff`,
   `roffon`, and whatever the source actually uses — **read the lines, do
   not infer from symbol tables**; recorded Dh lesson), run-on infiltration
   coupling on the receiving OFE, lateral/subsurface inter-element terms,
   sediment hand-off (mapped, flagged in/out of scope); openWEPP seam
   mapping (element orchestration, kernel phase order, where run-on enters
   the receiving element's day); state-shape proposal with legacy alias
   table; red-test definitions (per identity, per arm); sizing + increment
   refinement for M-C/M-D.
- Gates: no production edits; evidence artifacts + scope artifact complete
  with file:line citations against the pinned baseline; the legacy
  closure-degradation curve recorded.

## Increment restructure after M-A (2026-06-12)

M-A refuted the shadow-first premise for multi-OFE: current openWEPP
**fail-closes on all 29 multi-OFE surfaces** (28× `HKERNEL-WB14-RUNOFF-E-003`
day 2; H34 `E-001` day 1), so there is no multi-OFE behavior to preserve and
a bit-identical shadow gate has nothing to bite on. The single-OFE anchor
(7/7 passing) carries the preservation gate instead. Increments now follow
the scope artifact's port-scope sections (`mofe-routing-port-scope.md`,
authority for seams/symbols/red tests):

## Increment M-B — hydrology route closure (scope §"M-B hydrology route closure")

- Contract-first within the increment: pin the three conservation
  identities (per-element with run-on/run-off, transfer, hillslope-total)
  and the upstream-carry semantics in the routing contract(s) BEFORE
  production edits; then the scope's red tests (2-OFE surface carry, 2-OFE
  lateral handoff, H11/H6/H9/H1 cohort smoke past day 2); then populate
  `runon_input` from real upstream carry and feed lateral carry to the
  downstream `SubRIn` path per the scope's seam map.
- Gates: red tests green; 36/36 execute (fail-closed boundary retired on
  valid input); the three identities at noise on every executing surface;
  single-OFE anchor bit-identical; full Rust closure loop.

## Increment restructure after M-C (2026-06-12) — the real physics increment is runtime-state, not publication

M-C executed-hold with the correct verdict and surfaced the load-bearing
architectural finding (M-C evidence + Claude addendum): openWEPP runs **one
scheduler lifecycle per day over a 1-node topology** and **collapses the
daily WB output writeback to aggregate global scalars** (`UpStrmQ=0`,
`QOFE=Q`, single `OFE=1` row). MOFE machinery is real at the hourly-array
level (per-OFE seeds, `mofe_hourly_carry_arrays`), but **no per-OFE daily WB
output state is retained**. Synthesizing per-OFE rows from the aggregate
would be surrogate physics (correctly refused).

Re-framing: M-B = execution-unblock + **aggregate** hillslope closure (real,
holds). The **per-element and transfer identities — the actual definition of
routing closure — are still unproven** and unevaluable until per-OFE daily
state exists. The increments are therefore reordered: the per-OFE runtime
state increment comes BEFORE publication, because publication has nothing
real to publish without it.

## Increment M-C2 — per-OFE daily WB output state (runtime-state increment, the real MOFE physics)

- **Scoping first (no production edit until the seam is read):** answer the
  M-C open question — do the existing hourly carry arrays already produce
  genuine per-OFE daily balances (then this is "retain + expose through the
  writeback", narrower), or are per-OFE daily balances currently coupled so
  that distinct per-element accumulation must be built (broader)? Read the
  writeback and carry seams; cite lines.
- Contract-first: pin per-OFE daily WB output state semantics and the
  per-element + transfer identities as measurable surfaces.
- Implement: retain distinct per-OFE daily WB output state through the
  writeback (the topology/scheduler may need >1 node, or the writeback may
  need per-OFE accumulation — the scoping decides). No surrogate synthesis.
- Gates: per-element identity AND transfer identity (Σ run-off sent ≡ Σ
  run-on received) measurable and at the noise floor on the 1–5 ladder;
  aggregate identity unchanged; single-OFE anchor bit-identical; full loop.
  This is where "routing closure" is first actually proven.

**M-C2 execution result (2026-06-13): held before production edits** — the
broad-architecture branch is confirmed. Existing hourly carry arrays are
transfer/copy-forward state, not per-OFE daily WB output state; the
scheduler/writeback boundary (`HillslopeWritebackSurface`) is a single
aggregate state/flux map with no OFE-keyed collection. The "retain + expose"
narrow path is refuted; a per-OFE dynamic state model + sequential OFE
execution is required. Per the FDHP01 template (large architecture change →
design artifact before code), M-C2's hold flows into a design increment
(M-D) then an implementation sub-arc (M-E), then publication. **Tail
renumbered 2026-06-13**: the former M-D (erosion) / M-E (acceptance) are now
M-G / M-H; committed M-C/M-C2 evidence is self-describing and unaffected.

## Increment M-D — per-OFE state architecture design (design artifact, NO production code)

The architectural analog of FDHP01's `d3-fine-sublayer-port-scope.md`:
produce `artifacts/mofe-per-ofe-state-architecture.md` so the implementation
lands against a declared shape, not an evolving one. Required deliverable
sections (all `Static:`, file:line-cited against the current tree — read the
lines, do not infer from symbol tables, recorded Dh lesson):

1. **Target per-OFE state shape**: the OFE-keyed daily WB state/flux
   collection that replaces the single aggregate maps at the
   `HillslopeWritebackSurface` (`scheduler.rs:240`) and
   `KernelWritebackPayload` (`kernel-contract core_types.rs`) seams; what
   each per-OFE record holds; lifecycle (per-day rebuild vs persistent).
2. **Sequential execution model**: how OFE *i*'s completed daily state
   becomes OFE *i+1*'s run-on/run-off inflow — topology with N nodes vs
   per-OFE lane iteration over the phase graph (`execute_with_kernel`,
   `scheduler.rs:501`); where the legacy `irs`/`rochek` continuation logic
   (from `mofe-routing-port-scope.md`) maps onto it.
3. **Contract surface**: the per-element identity, the transfer identity
   (Σ run-off sent ≡ Σ run-on received), and per-OFE daily-state semantics
   as measurable contract invariants; which existing contracts amend
   (`SC-RUNOFFPART-001`/`SC-WATBAL-001`/`SC-SYSTEM-001`).
4. **Kernel-contract / scheduler / writeback / publication change map**:
   every seam the implementation touches, with the aggregate→per-OFE
   migration path and the single-OFE-anchor preservation argument.
5. **Red-test definitions** (per identity, per arm) and the implementation
   **sub-increment breakdown + sizing** for M-E (each sub-increment behind
   a conservation hard stop; per-element + transfer identities first proven).
- Gate (non-deferral compliant): no production edits; the design artifact
  complete with current-tree file:line citations and a sub-increment plan
  whose every gate is measurable in its own scope. This is a design
  increment — its only completion criterion is the artifact, so it can
  legitimately close `complete`.

## Increment M-E — per-OFE state implementation (sub-arc; routing physics proven here)

- Execute the M-D sub-increment breakdown: contract-first per sub-increment;
  red tests before code; per-OFE daily WB state retained through the
  writeback; sequential OFE handoff.
- Gates (per sub-increment): per-element identity AND transfer identity
  measurable and at the noise floor on the 1–5 ladder; aggregate identity
  unchanged; single-OFE anchor bit-identical; full Rust closure loop. **This
  is where "routing closure" is first actually proven** — no sub-increment
  closes `complete` until its own identities are evidenced (non-deferral).

## Increment M-E4-REDO — non-tautological identity validation (Claude blocking review 2026-06-13)

Status: executed 2026-06-13; non-tautological internal WB13 identity
validation now closes under `TOL-WATBAL-007`. Public per-OFE WAT publication
remains M-F scope.

M-E4 produced correct per-OFE WB13 *record production* (cardinality, ordering,
full row fields) but its identity *validation* is tautological (residuals
exactly 0.0): per-element checks `soil_water_total==total_soil` (same-row
alias), transfer checks the row's `UpStrmQ` vs the input it was built from,
aggregate cancellation compares input vs output where input was built from
output. None tests `INV-WATBAL-096`. Before M-F:

- Rebuild the per-element identity to measure the real balance **per OFE**:
  inflows (RM/Irr/UpStrmQ/SubRIn) − outflows (Interception/Q/Ep/Es/Er/Dp/
  latqcc/Tile) − **independently-measured** `ΔSoilWaterTotal` (day-over-day
  from the OFE's own state, not derived from fluxes) = residual at the noise
  floor. The record fields already exist.
- Rebuild the transfer identity as a true cross-OFE check on a 2-OFE fixture:
  OFE i's **sent** runoff vs OFE i+1's **received** run-on as
  independently-sourced quantities (not built from each other).
- Pin the numeric tolerance in `SC-WATBAL-001` (FDHP01-grade ~1e-11/1e-13 mm,
  the E0 deferral), replacing the `1.0e-6` code constant.
- Add the named frost-per-OFE fixture (FDHP01 closure re-instances per OFE
  without perturbing the single-OFE frost anchor).
- Gate: the three identities close to the pinned noise floor on real 2-OFE
  and 5-OFE runtime records (NONZERO-but-at-noise residuals are the proof of
  genuineness; exact 0.0 is the tautology smell). Single-OFE anchor unchanged.

## Increment M-F — per-OFE WAT publication

Status: executed-hold 2026-06-13; row cardinality/publication provenance
landed, but surface run-on publication remains blocked by zero current surface
carry on real multi-OFE runs.

- On real per-OFE state (M-E), publish per-OFE rows: no `UpStrmQ=0` for
  downstream OFEs, no `QOFE=Q` aliasing, one row per OFE per day or an
  explicitly contracted equivalent; handoff-to-printed-precision checks
  against the M-A calibration (legacy-clean at 1–5, so usable here). A
  genuine publication of existing state, not synthesis.
- Gates: scope red tests; the three identities still at noise; single-OFE
  anchor; full loop.

M-F execution produced real public per-OFE row shape:

- H1/H6/H9/H11 emit `day_count * contributor_ofe_count` public WAT rows.
- Manifest provenance reports
  `publication_ofe_policy=per-ofe-dynamic-water-balance-state`,
  `storage_lineage_policy=per-ofe-dynamic-wb-state`, and grouped first/last
  OFE keys.
- M-F's apparent `QOFE` non-alias finding is superseded by M-F-REDO; the
  current blocker is geometry-scaled public `QOFE`.

M-F does **not** close because the surface export producer is still hollow:
direct WAT audits show `max_upstrmq=0.0` and zero downstream nonzero `UpStrmQ`
rows for H1/H6/H9/H11. The apparent surface handoff residual of `0.0` is
zero-on-zero equality, not conservation evidence. Lateral `SubRIn` handoff has
nonzero rows and closes, so the blocker is localized to current surface carry
publication. Detailed evidence:
`m-f-per-ofe-wat-publication-evidence.md`.

## Increment M-F-REDO — per-OFE record distinctness + surface export closure

Status: executed-hold 2026-06-13; lane differentiation, active surface/lateral
handoff, and anti-clone publication are fixed, but candidate `QOFE` still
aliases candidate `Q` on real multi-OFE smoke. Geometry-scaled `QOFE`
publication remains blocked before M-G/M-H.

**Root cause deepened by Claude review (2026-06-13):** the M-F blocker is not
only the surface export producer. On the H1 M-F cohort, **every hydrology
column is identical across all 5 OFEs** (`Es`/`Ep`/`Dp`/`SoilWaterTotal`/`P`
identical; `Q` ≤2 distinct values/day; `QOFE=0`, `UpStrmQ=0`) — the published
per-OFE records are **aggregate-duplicated clones**, not distinct per-OFE
hydrology. `UpStrmQ=0` is a symptom. M-E4-REDO's genuine identities passed on
the clones because internal consistency cannot detect duplication — the legacy
comparator caught what conservation could not (see
`m-f-per-ofe-wat-publication-evidence.md` Claude review). M-F-REDO must
root-cause the duplication, and a latent M-E3 question (are the lanes actually
differentiated, or does M-F publish from the aggregate path?) is in scope.

M-F-REDO owns the M-F blocker before M-G/M-H may proceed:

- **First: trace the per-OFE record source** (read the seam, cite lines). Do
  the M-E3 lane records carry distinct per-OFE values pre-publication, or does
  M-F read aggregate? Fix the root: either M-E3 lane differentiation (static
  per-OFE soil/slope/cover slices applied; run-on delivered to the lane kernel)
  or the M-F publication source.
- **Anti-clone gate** (structural analog of the M-E4-REDO anti-tautology TOL):
  multi-OFE published `Q`/`Es`/`Ep`/`Dp`/`SoilWaterTotal` must be **distinct
  across OFEs** on a routed hillslope — pin it in `SC-WATBAL-001`/
  `SC-SYSTEM-001`; all-OFE-identical hydrology hard-fails.
- Implement a contract-backed current surface export producer feeding
  `MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT` and per-OFE `QOFE` from the
  authoritative runoff/surface-routing state, not from public WAT synthesis.
- Add red tests requiring downstream nonzero `UpStrmQ` on active multi-OFE
  surface-runoff days and proving `current UpStrmQ == previous QOFE` with
  nonzero operands.
- Preserve M-F row cardinality/provenance, `QOFE != Q`, and lateral handoff.
- Preserve M-E4-REDO non-tautological internal WB13 identities.
- Rerun H1/H6/H9/H11 local comparisons without the comparator subagent if the
  Spark quota remains exhausted, and rerun the single-OFE anchor.
- Gate: no zero-on-zero surface handoff acceptance; exact `0.0` residual is
  valid only when paired with evidence of nonzero active surface transfer rows.

M-F-REDO execution fixed the clone and active-handoff defects:

- Multi-OFE lanes now build OFE-local soil/slope/management runtime surfaces.
- H1/H6/H9/H11 final smoke under `/tmp/openwepp_mofe01_mfredo_final` exits
  zero and reports nonzero downstream `UpStrmQ`/`SubRIn` rows with adjacent
  transfer residuals at `0.0` on active operands.
- The anti-clone audit reports zero active all-OFE-identical days and per-day
  distinctness up to the OFE count for `Q` and `SoilWaterTotal`.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 remain byte-identical to M-E2
  for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`.

M-F-REDO does **not** close because the required `QOFE != Q` gate regressed at
the real publication surface: candidate `max_abs_qofe_minus_q=0.0` for
H1/H6/H9/H11. The pinned legacy-clean ladder proves `QOFE` is not a local `Q`
alias (`max abs(QOFE-Q)` is `362.13991`, `177.51694`, `185.89531`, and
`84.64425` mm respectively), and baseline `watbal.for` writes `Q` with
`efflen/totlen` scaling while `QOFE` uses `efflen/slplen` scaling.

## Increment M-F-REDO-CLONE — make per-OFE runoff genuinely distinct (prerequisite)

Status: executed 2026-06-14; stale multi-step WB12 infiltration lineage was
root-caused and fixed, H1/H6/H9/H11 local runoff anti-clone gates pass, and
the genuine per-element/transfer identities remain at `TOL-WATBAL-007`.
M-F-REDO2 remains the separate `QOFE` local-depth publication increment.

M-F-REDO's anti-clone fix reached ET but **not runoff**: 968/1057 (92%) of
runoff days still have all-5-OFE-identical local runoff (`Q − UpStrmQ`), and
the peak-runoff day is a full clone across every column (see
`m-f-per-ofe-wat-publication-evidence.md` Claude review). The `Q = 129.16 ×
ofe_id` ladder is the clone accumulating, not real routing.

- Root-cause why ET differentiated but runoff did not (the diagnostic clue):
  which per-OFE lane inputs reach the ET kernel but not the runoff/infiltration
  kernel — static soil/slope slice application, or the run-on/runoff
  generation path reading aggregate state. Read the seam, cite lines.
- The **anti-clone gate must bite on the runoff columns** (`Q`, local
  `Q − UpStrmQ`, `Es`, `Dp`, `SoilWaterTotal`), not only `Ep`. Multi-OFE
  published runoff must be distinct across OFEs on a routed hillslope.
- Gate: per-OFE local runoff distinct across OFEs on the H1/H6/H9/H11 cohort;
  genuine per-element + transfer identities still close at `TOL-WATBAL-007`;
  single-OFE anchor bit-identical.

M-F-REDO-CLONE execution result:

- Root cause: MOFE hourly lanes seeded `wb12_infiltration=0.0`, WB18 ran before
  WB14, and WB14 trusted producer-published infiltration on multi-step lanes
  without a same-pass lineage proof. That let runoff reconcile against stale
  zero infiltration and clone across lanes even though ET inputs had
  differentiated.
- Fix: WB18 reconstructs same-pass infiltration for MOFE hourly carry lanes
  and marks lineage; WB14 accepts producer-published infiltration on multi-step
  lanes only when that same-pass lineage marker is present, otherwise it
  recomputes lane-local infiltration.
- H1/H6/H9/H11 under `/tmp/openwepp_mofe01_mfredo_clone_current` have zero
  all-identical local-runoff days and zero full-vector clone days on active
  runoff days. Per-element residual maxima stay at noise
  (`<=2.56e-13 mm`); transfer and aggregate residuals close at `0.0`.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 under
  `/tmp/openwepp_mofe01_mfredo_clone_single_final` remain byte-identical for
  `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS).
- Local semantic comparisons were run directly with `tools/owcmp/semantic_wat.py`
  and no comparator subagent. Row-key coverage is complete for H1/H6/H9/H11;
  value-family semantic failures remain M-F-REDO2 publication scope.

## Increment M-F-REDO2 — per-OFE `QOFE` as local runoff depth (after runoff is distinct)

Status: executed 2026-06-14; public `QOFE` now publishes the OFE-local routed
runoff depth and public `Q` publishes the cumulative-length depth. The
downstream `QOFE == Q` alias gate is closed on H1/H6/H9/H11, active handoff
and anti-clone gates remain closed, internal identities remain at
`TOL-WATBAL-007`, and single-OFE anchors remain byte-identical.

Prerequisite: M-F-REDO-CLONE (a cloned runoff renders ÷slplen as a tidy ladder
of equal local depths, proving nothing). Then render the genuinely-distinct
routed `runoff(iplane)` as the two legacy depths:

- **`QOFE` = the runoff depth *through this OFE*** — `runoff(iplane) ·
  efflen/slplen` (`watbal.for:1099`), normalized to the OFE's **own** slope
  length. This is the sane per-OFE semantic (operator framing 2026-06-13);
  pin it in `SC-WATBAL-001` as "local runoff depth at the OFE," not as an
  opaque "geometry-scaled" term.
- **`Q`** (the "runoff" column) = `runoff(iplane) · efflen/totlen`
  (`watbal.for:1094`), normalized to **cumulative** length (the source comment:
  *"efflen may span OFEs"*) — a hillslope-cumulative depth, which is why it
  grows downslope.
- One routed runoff, two normalizations: local (÷slplen → `QOFE`) vs cumulative
  (÷totlen → `Q`). Contract test: multi-OFE rows cannot accept `QOFE == Q`
  where `slplen ≠ totlen`; `QOFE` must equal the OFE-length-normalized depth.
- Preserve active surface/lateral handoff, the runoff anti-clone gate, the
  M-E4-REDO non-tautological identities, and the single-OFE anchor.
- Gate: `QOFE` is the OFE-local depth and `Q` the cumulative depth, both from
  genuinely-distinct routed runoff; the row-key-aligned semantic comparison vs
  legacy (clean at 1–5) passes the value families or remaining diffs are
  classified by independent conservation/lineage evidence.

M-F-REDO2 execution result:

- Fix: per-OFE WB13 publication now receives the raw routed runoff transfer
  operand plus explicit OFE-local/cumulative runoff-publication geometry. It
  publishes `QOFE = routed_runoff * efflen / ofe_length` and
  `Q = routed_runoff * efflen / cumulative_length`; internal conservation
  identities continue to use the raw transfer/runoff operand, not public `Q`.
- Contract: `SC-WATBAL-001` version 159 and `SC-SYSTEM-001` version 82 pin
  the public `QOFE`/`Q` normalizations, reject downstream `QOFE == Q` aliases
  where `slplen != totlen`, and separate public runoff normalization from raw
  conservation operands.
- Runtime: H1/H6/H9/H11 under `/tmp/openwepp_mofe01_mfredo2_current` exit
  zero. Downstream active `QOFE == Q` alias rows are zero on all four; candidate
  `QOFE/Q` ratios match the legacy clean geometry ladder (`2`, `3`, `4`, `5`
  as applicable). Max `abs(QOFE-Q)` is H1 `310.9379099869`, H6
  `107.7814221517`, H9 `99.8813275553`, H11 `83.8884201819` mm.
- Conservation/anti-clone: transfer residual max remains `0.0`; per-element
  residual maxima are H1 `2.5579538487363607e-13`, H6
  `2.0250467969162855e-13`, H9 `1.9895196601282805e-13`, H11
  `2.2737367544323206e-13` mm. Active `QOFE`, public `Q`, and hydrology-vector
  all-clone active-day counts are zero for H1/H6/H9/H11.
- Comparisons: local `tools/owcmp/semantic_wat.py --candidate-year-offset
  1999` commands were run directly without the comparator subagent. Row-key
  coverage is complete for H1/H6/H9/H11. Semantic value pass remains false,
  but the remaining value-family deltas are classified as broader routed
  hydrology/storage/ET parity gaps because the M-F-REDO2 publication-ratio
  invariant matches legacy and independent conservation/lineage gates close.
- Single-OFE anchors: corrected p-run anchors under
  `/tmp/openwepp_mofe01_mfredo2_single_anchor` are byte-identical to
  `/tmp/openwepp_mofe01_mfredo_clone_single_final/output` for
  H8/H15/H19/H20/H22/H23/H28 `.hbp`, `.loss.json`, `.plot.parquet`, and
  `.wat.parquet` (28/28 PASS).

## Increment M-G — erosion `qin`/sediment coupling decision (scope §"M-D erosion qin and sediment coupling")

- Per the scope: implement only if the water seam owns it inseparably;
  otherwise contract-pin the boundary and emit the follow-on. Operator
  visibility on whichever way the evidence lands.

## Increment M-H — ladder acceptance + closure

- Full-cohort acceptance per OFE count (36-run 1–5 ladder); totalwatsed3
  end-to-end audit on routed output (the WBVAL06/6a deferral resolved or
  explicitly re-stated); package closure obligations (ROADMAP item 1,
  README narrative, handoff naming the next mechanism + the named >10-OFE
  far-point follow-on and any erosion-coupling follow-on).

## Dispatch instructions

Each Codex dispatch: *"Execute increment <M-A|M-B|M-C|M-C2|M-D|M-E|M-E4-REDO|M-F|M-F-REDO|M-F-REDO-CLONE|M-F-REDO2|M-G|M-H> of
`docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001/artifacts/mofe-staged-increment-plan.md`
end-to-end."* Required reading order: this plan; `package.md`;
`mofe-routing-port-scope.md` (once it exists); the FDHP01 staged plan
(failure modes the rules encode). An increment that cannot meet its gates
backs out with evidence recorded — localized to its seam.
