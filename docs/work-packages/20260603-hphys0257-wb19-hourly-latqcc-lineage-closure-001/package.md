# 20260603-hphys0257-wb19-hourly-latqcc-lineage-closure-001

Status: completed/HOLD

This package is a living ExecPlan. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing it. It
follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

HPHYS0256 corrected daily WB19 lateral lane authority but the H1/H7/H39 and
full `H1..H39` diagnostics selected the hourly lane and retained the same
`latqcc` residuals. This package executes that continuation by diagnosing,
correcting, and validating hourly WB19 lateral lineage for saturated-depth
selection, `tdvv` cap, layer withdrawal, and publication into `latqcc`,
`Qd`, and storage.

## Objective

Diagnose, correct, and validate hourly WB19 lateral-transfer `latqcc` lineage
against pinned baseline `watbal_hourly.for` authority. Closure requires
contract-backed hourly semantics, contract-derived tests, production correction
only where proven, H1/H7/H39 diagnostics, and full `H1..H39` metrics.

## Rationale

HPHYS0256 proved that daily and hourly lateral lanes must remain distinct.
The continuation evidence shows the authoritative H1/H7/H39 suite is hourly
(`selected_lane: hourly`, `substep_count: 24`) and still has `latqcc` diffs of
H1 `+0.595319 mm`, H7 `+1.469954 mm`, and H39 `+8.733643 mm`. The next
correctness surface is therefore hourly WB19 lateral state mutation and
publication, not daily lane authority.

## Included Scope

- Amend canonical `SC-SUBHYD-001` and `SC-WATBAL-001` for any missing proven
  hourly WB19 lineage details.
- Add contract-derived tests before production edits.
- Diagnose pinned baseline hourly lateral blocks in
  `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`.
- Correct only baseline-authoritative hourly WB19 lateral behavior needed for
  `latqcc`/`Qd`/storage closure.
- Run targeted H1/H7/H39 diagnostics and a fresh full `H1..H39` semantic metric
  snapshot.
- Record review, verification, gates, and final disposition.

## Excluded Scope

- No heuristic `latqcc` damping or storage compensation.
- No daily WB19 branch changes unless required to preserve already-corrected
  HPHYS0256 daily semantics.
- No WB17 Ep/root uptake, snow/runoff timing, WB18 Dp/Pe, or WB11 seed
  projection changes unless a proven hourly WB19 contract seam requires a
  narrow fixture expectation update.
- No watershed rerun unless separately authorized.
- No commit/push unless separately requested.

## Deliverables

1. Package-local evidence artifacts under this directory.
2. Canonical contract amendments for hourly WB19 lateral lineage authority.
3. Contract-derived tests that fail before any production correction and pass
   after it.
4. Minimal production correction for hourly WB19 lateral state/publication
   behavior, if diagnosis proves a defect.
5. Targeted H1/H7/H39 `latqcc` diagnostics.
6. Full `H1..H39` semantic metrics and continuation disposition.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/artifacts/targeted-h1-h7-h39-diagnostics.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepppy/.venv/bin/python`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`

## Intended Write Set

- `docs/work-packages/20260603-hphys0257-wb19-hourly-latqcc-lineage-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `tests/integration/hphys0256_wb19_latqcc_lane_branch_contract.rs`

## Contract-First Sequence

1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

No production code edits are allowed before steps 1-3 are complete.

## Phase Plan

### Phase A — Hourly Baseline Diagnosis

Read required contracts, HPHYS0256 evidence, and pinned hourly
`watbal_hourly.for` lateral blocks. Record saturated-depth selection, `tdvv`
cap, layer withdrawal, and publication semantics in
`artifacts/wb19-hourly-latqcc-lineage-diagnosis.md`.

### Phase B — Contract and Test Gate

Amend canonical contracts for proven missing hourly semantics. Add
contract-derived tests that demonstrate the defect. Run the targeted test
before production edits and record expected failing evidence in
`artifacts/pre-implementation-contract-gate.md`.

### Phase C — Production Correction

Apply the minimal hourly WB19 correction. Preserve typed fail-closed guards;
do not introduce silent defaults or heuristic flux scaling. Run targeted tests
and update `artifacts/implementation-test-evidence.md`.

### Phase D — Metrics, Review, and Disposition

Run targeted H1/H7/H39 diagnostics and the full `H1..H39` semantic suite.
Record metrics, review artifacts, verification artifacts, gate results, and
final `HOLD`/`GO` disposition.

## Exit Criteria

- Contracts and tests cover implemented behavior.
- Production changes trace to pinned baseline source, not heuristic math.
- H1/H7/H39 `latqcc` evidence is recorded.
- Full `H1..H39` semantic metrics exist or not-run rationale is explicit.
- All claims in artifacts are labeled `Static:` or `Ran:`.
- Full Rust gates are run or explicitly marked not-run with rationale.
- Disposition is truthful; unresolved semantic parity remains `HOLD`.

## Security-Impact Gate

No external systems or network actions are required. Work is local repository
engineering over flat files and local diagnostic commands. No secrets,
credentials, user data, or production service state are accessed or modified.

## Progress

- [x] (2026-06-03) Scaffolded HPHYS0257 from HPHYS0256 continuation.
- [x] (2026-06-03) Completed hourly WB19 `ui_ssh` lineage diagnosis.
- [x] (2026-06-03) Amended contracts and added red contract-derived tests.
- [x] (2026-06-03) Implemented production correction authorized by diagnosis.
- [x] (2026-06-03) Ran targeted and full-suite metric snapshots.
- [x] (2026-06-03) Recorded review, verification, disposition, and handoff.

## Surprises & Discoveries

- Static: pinned hourly WB19 uses `ui_ssh(i)` in `totK += ui_ssh*fffx*dg`
  while preserving `anisrt(iplane)` in the final `subq` formula.
- Static: openWEPP already had vertical `wb18_perc_ssc_####` but no distinct
  `wb19_lateral_ssh_####` runtime surface.
- Static: runtime projection was also carrying layer `ui_anisrt` into the
  profile `anisrt` multiplier, which double-applied modern layer anisotropy
  once `ui_ssh` was introduced.
- Ran: the corrected `ui_ssh` plus modern profile-anisotropy unity projection
  improves `latqcc`, `Dp`, and aggregate storage residuals but does not close
  semantic parity.

## Decision Log

- Decision: Scope HPHYS0257 to hourly WB19 lateral lineage.
  Rationale: HPHYS0256 diagnostics selected hourly mode and retained `latqcc`
  residuals after daily lane correction.
  Date/Author: 2026-06-03 / Codex.
- Decision: Add a distinct hourly `wb19_lateral_ssh_####` surface rather than
  repurposing `wb18_perc_ssc_####`.
  Rationale: pinned `watbal_hourly.for` consumes `ui_ssh(i)` for hourly lateral
  conductivity; vertical `ssc(i)` remains percolation/daily-lane authority.
  Date/Author: 2026-06-03 / Codex.
- Decision: Keep package disposition `HOLD`.
  Rationale: contract vectors and Rust gates pass, but the full 39-hillslope
  semantic pass remains `0/39` after the corrected HPHYS0257 run.
  Date/Author: 2026-06-03 / Codex.

## Outcomes & Retrospective

- HPHYS0257 closes a real hourly WB19 authority gap: modern hourly lanes now
  require and consume `wb19_lateral_ssh_####`, projected from layer `ui_anisrt`
  conductivity lineage.
- H1/H7/H39 targeted day-1 `latqcc` diffs improved from HPHYS0256 to H1
  `+0.023532 mm`, H7 `+0.047995 mm`, and H39 `+0.180364 mm`.
- Full-suite `latqcc` mean absolute diff improved from `0.805148` to
  `0.675393`, and max absolute diff improved from `28.005815` to `14.760000`.
- Continuation should focus on the remaining hourly `tdvv` cap,
  capacity/withdrawal threshold, and `latqcc`/storage publication lineage so
  the next package can close residuals without altering validated
  conductivity lineage.
