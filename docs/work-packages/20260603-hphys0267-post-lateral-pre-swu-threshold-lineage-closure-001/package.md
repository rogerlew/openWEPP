# HPHYS0267 Post-Lateral Pre-SWU Threshold Lineage Closure

Status: completed/HOLD

## Objective

Execute the HPHYS0266 continuation by making WB19 `drfc`/`fzdrfc` thresholds,
pre/post-lateral layer storage, and WB17 stress-threshold inputs observable
across the post-lateral/pre-SWU seam, then classify whether H1/H7/H39 first
seasonal `Ep` divergences expose a baseline-authoritative production defect.

## Execution Summary

Ran:

- Diagnostic root: `/tmp/hphys0267_20260603T162040Z`.
- H1/H7/H39 targeted traces through day 130.
- Full H1..H39 hillslope runtime suite and semantic comparator suite.
- Focused Rust trace tests and diagnostic Python compile.

Static:

- Canonical contracts were amended first:
  `SC-WATBAL-001@94`, `SC-SUBHYD-001@32`, and `SC-EVAP-001@21`.
- Trace-only observability was added in `openwepp-runner`; no production
  physics/kernel correction was made.
- Pinned baseline authority in
  `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:774-824`
  confirms realized lateral flow is withdrawn top-down from any layer with
  storage above `fzdrfc`, after active-layer potential/capacity calculation.

Disposition: completed/HOLD. HPHYS0267 did not prove a production defect in
the post-lateral/pre-SWU threshold seam. Residual ownership remains upstream in
material storage magnitude and snow/runoff coupling context.

## Rationale

HPHYS0266 proved that WB17/SWU identities, WB11/WB18 aggregate recompute, and
WB19 realized lateral publication identities close at first-divergence days.
The remaining actionable seam is layer-threshold lineage: H7 withdraws from a
layer that is also SWU-stressed, while H1/H39 show cleaner bottom-zone lateral
activity below stressed root layers. The next package must expose the threshold
inputs that decide lateral eligibility and root stress before changing physics.

## Included Scope

- Canonical `SC-WATBAL-001`, `SC-SUBHYD-001`, and `SC-EVAP-001` amendments for
  post-lateral/pre-SWU threshold lineage evidence.
- Trace-only runtime observability for WB19 `fc`, `coca`, `drfc`, `frzw`,
  `fzdrfc`, pre/post-lateral `theta`, lateral withdrawal, WB17 `ul`,
  `pltol*ul`, and storage-to-threshold ratios.
- Contract-derived tests for the new trace fields.
- H1/H7/H39 targeted trace classification through at least day 130.
- Full H1..H39 semantic metrics after diagnostic execution and after any
  production patch.
- Production correction only if trace evidence proves an in-scope
  baseline-authoritative defect.

## Excluded Scope

- Heuristic/proxy hydrology or ET tuning.
- Changing WB17/SWU, WB19 lateral, snow/runoff, or storage physics without a
  pinned-baseline defect proof.
- Watershed routing changes.
- Full snow/frost or EVAPPM post-ET redistribution migration unless the
  threshold evidence directly proves that exact defect.

## Deliverables

- Updated canonical `SC-*` authority before production edits.
- Trace schema extensions and focused tests for post-lateral/pre-SWU threshold
  lineage.
- H1/H7/H39 threshold-lineage diagnostic report.
- Full H1..H39 semantic summary for continuation metrics.
- Final `GO`/`HOLD` disposition and continuation handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/swu.for`
- Pinned baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- `docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260603-hphys0266-layer-storage-lateral-snow-coupling-closure-001/artifacts/targeted-h1-h7-h39-storage-lateral-classification.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0267-post-lateral-pre-swu-threshold-lineage-closure-001/**`
- Production kernel files only if evidence proves an in-scope defect.

## Phase Plan

1. Amend canonical contracts with post-lateral/pre-SWU threshold-lineage
   evidence requirements.
2. Add trace-only threshold lineage observability and focused tests.
3. Record the pre-implementation contract gate before production-code decision.
4. Run H1/H7/H39 targeted traces and classify first-divergence threshold
   lineage.
5. Patch production code only if evidence identifies an in-scope
   baseline-authoritative defect.
6. Run focused gates and full H1..H39 semantic metrics.
7. Complete review, verification, disposition, and continuation handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived diagnostic/test evidence.
3. Pre-implementation contract gate.
4. Production code edits, if justified.

## Exit Criteria

- Truthfulness-labeled artifacts distinguish `Static:` from `Ran:`.
- H1/H7/H39 first-divergence rows include pre/post-lateral `theta`, WB19
  `drfc`/`fzdrfc`, lateral withdrawal, WB17 `ul`, WB17 stress threshold,
  storage-to-threshold ratios, and same-day snow/runoff/storage context.
- Any production patch traces directly to pinned baseline authority and has
  focused tests.
- Full H1..H39 semantic metrics are recorded.
- Known remaining gaps stay in `HOLD`; no heuristic closure is claimed.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must work
through the phase plan, update artifacts through disposition, and only ask for
user direction when hard-blocked by missing local authority or unavailable
validation substrate.

## Final Disposition

HOLD. The work package closed the WB19 threshold-lineage question for
H1/H7/H39 without authorizing a production physics patch. Continue with a
contract-first HPHYS0268 package focused on pre-WB17 material storage magnitude,
snow/runoff partition timing, and layer redistribution lineage.
