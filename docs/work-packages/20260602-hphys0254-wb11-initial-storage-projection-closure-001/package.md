# 20260602-hphys0254-wb11-initial-storage-projection-closure-001

Status: complete/HOLD

This package is a living ExecPlan. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing it. It
follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

HPHYS0253 localized the H1 day-1 water-balance gap upstream of day-1 process
losses: openWEPP starts `20.153260 mm` drier than the baseline WAT-derived
t=0 proxy before scheduler phases run. This package closes the next
continuation recommendation by making WB11 initial/runtime storage projection
contract-authoritative, tested, and validated against H1/H7/H39 plus the full
`H1..H39` semantic suite.

## Objective

Diagnose, correct, and validate baseline-authoritative WB11 initial storage
projection for layer `st(i)`/`theta`/`watcon` and aggregate `soil_water`
seeding. The observable outcome is a contract-backed implementation decision
that either reduces the H1 t=0 storage deficit or records a sharper blocker
with full-suite metrics for continuation.

## Rationale

HPHYS0253 proved that candidate day-1 accounting is internally balanced and
that most H1 day-1 storage deficit exists before percolation, ET, lateral
transfer, or WB13 publication execute. Therefore another WB18/WB19/WB17
loss-surface correction would be poorly targeted. The next defensible package
is WB11 seed/runtime projection: the code that maps parsed soil and runtime
inputs into layer storage and aggregate water surfaces consumed by WB11.

## Included Scope

- Amend canonical `SC-WATBAL-001` and `SC-SOIL-001` for any proven WB11
  initial/runtime storage projection semantics.
- Add contract-derived tests before production edits.
- Diagnose current openWEPP H1/H7/H39 post-seed `wb11_soil_water`,
  `wb18_perc_theta_####`, depth (`dg_####`), `thetdr_####`, and aggregate
  alias mapping against pinned baseline source and available baseline WAT
  surfaces.
- Correct only baseline-authoritative seed/projection behavior proven from
  canonical contract text and pinned baseline source.
- Run targeted H1/H7/H39 day-1 traces and a fresh full `H1..H39` semantic
  metric snapshot.
- Record review, verification, gates, and final disposition.

## Excluded Scope

- No heuristic storage inflation to match baseline totals.
- No direct WB18 percolation, WB19 lateral, WB17 ET, snow, runoff, or WB13
  publication compensation unless a proven WB11 projection contract requires
  existing symbols as diagnostics.
- No raw-theta fallback when authoritative corrected storage lineage exists.
- No watershed rerun unless explicitly authorized by a later package.
- No commit/push unless separately requested.

## Deliverables

1. Package-local evidence artifacts under this directory.
2. Canonical contract amendments when production behavior changes.
3. Contract-derived tests that fail before the production correction and pass
   after it.
4. Minimal production correction for WB11 initial/runtime storage projection,
   if diagnosis proves a defect.
5. Targeted H1/H7/H39 post-seed/day-1 trace metrics.
6. Full `H1..H39` semantic metrics and continuation disposition.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0253-h1-day1-storage-localization-diagnostics-001/artifacts/hphys0253_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0253-h1-day1-storage-localization-diagnostics-001/artifacts/worker-handoff.md`
- `/tmp/unpalatable_parity_20260529T192707Z/runs/`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`
- `/workdir/wepppy/.venv/bin/python`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/scon.for`

## Intended Write Set

- `docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`
- `tests/integration/wb11_storage_projection_kernel_contract.rs`

## Contract-First Sequence

1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

No production code edits are allowed before steps 1-3 are complete.

## Phase Plan

### Phase A — Baseline and Projection Diagnosis

Read the required contracts, HPHYS0253 evidence, and pinned baseline sources.
Map openWEPP seed surfaces to legacy `st(i)`, `soilw(i)`, and `watcon`.
Record the diagnosis in `artifacts/wb11-initial-storage-projection-diagnosis.md`.

### Phase B — Contract and Test Gate

Amend canonical contracts for the proven projection semantics. Add a
contract-derived test that demonstrates the defect and expected behavior. Run
the targeted test before production edits and record the expected failing
evidence in `artifacts/pre-implementation-contract-gate.md`.

### Phase C — Production Correction

Apply the minimal WB11 seed/projection correction. Preserve typed fail-closed
guards; do not introduce silent defaults or heuristic compensation. Run the
targeted tests and update `artifacts/implementation-test-evidence.md`.

### Phase D — Metrics, Review, and Disposition

Run targeted H1/H7/H39 traces and the full `H1..H39` semantic suite. Record
metrics, review artifacts, verification artifacts, gate results, and final
`HOLD`/`GO` disposition.

## Exit Criteria

- Contracts and tests cover implemented behavior.
- Production changes trace to pinned baseline source, not heuristic math.
- H1/H7/H39 post-seed and day-1 storage evidence is recorded.
- Full `H1..H39` semantic metrics exist.
- All claims in artifacts are labeled `Static:` or `Ran:`.
- Full Rust gates are run or explicitly marked not-run with rationale.
- Disposition is truthful; unresolved semantic parity remains `HOLD`.

## Security-Impact Gate

No external systems or network actions are required. Work is local repository
engineering over flat files and local diagnostic commands. No secrets,
credentials, user data, or production service state are accessed or modified.

## Progress

- [x] (2026-06-02) Scaffolded HPHYS0254 from HPHYS0253 continuation.
- [x] (2026-06-02) Complete WB11 seed/projection diagnosis.
- [x] (2026-06-02) Amend contracts and add red contract-derived test.
- [x] (2026-06-02) Implement production correction authorized by diagnosis.
- [x] (2026-06-02) Run targeted traces and full `H1..H39` metric snapshot.
- [x] (2026-06-02) Record review, verification, disposition, and handoff.

## Surprises & Discoveries

- Alias separation was required to satisfy both HPHYS0254 and AUTH05/AUTH07:
  generic parser/external-authority symbols remain generic, while hydrology
  seed/runtime consumers use `wb11_nsl` and `wb19_*` aliases.
- WB18 lower-layer over-UL handling exposed a baseline contract gap: finite
  `stu >= 0.95` lower-layer ratios must be capped to `0.95`, not hard-failed.
- H1/H7/H39 post-seed storage aligns after the fix, but day-1 residuals remain
  dominated by post-seed process terms, especially WB19 `latqcc`.

## Decision Log

- Decision: Scope HPHYS0254 to WB11 initial/runtime storage projection.
  Rationale: HPHYS0253 localized the dominant H1 day-1 gap to post-seed state,
  before WB18/WB19/WB17 process losses.
  Date/Author: 2026-06-02 / Codex.
- Decision: Preserve generic parser and constitutive authority symbols and add
  hydrology-owned WB11/WB19 aliases for normalized seed geometry.
  Rationale: This closes WB11 seed projection without breaking AUTH05/AUTH07
  authority boundaries.
  Date/Author: 2026-06-02 / Codex.
- Decision: Set final disposition to `HOLD`.
  Rationale: Package objective is executed, but full semantic parity remains
  `0/39`.
  Date/Author: 2026-06-02 / Codex.

## Outcomes & Retrospective

- HPHYS0254 corrected the WB11 initial hydrology seed projection defect and
  added contract/tests for the normalized seed alias contract.
- Targeted post-seed storage now aligns with baseline inferred t=0:
  H1 `+0.015748 mm`, H7 `+0.078917 mm`, H39 `+0.084258 mm`.
- Full Rust gates passed.
- Full `H1..H39` runtime succeeded for `39/39`, but semantic pass remains
  `0/39`.
- Continuation should focus next on WB19 `latqcc` residuals using the corrected
  seed state as the fixed starting condition.
