# 20260602-hphys0252-wb19-lateral-storage-availability-closure-001

Status: HOLD

This package is a living ExecPlan. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing it. It
follows `/workdir/openWEPP/docs/codex_exec_plans.md`.

## Purpose / Big Picture

HPHYS0251 showed that WB17 root uptake is now limited by upstream storage
availability, not by the `swu.for` uptake curve itself. HPHYS0246 already
removed the WB18 aggregate-storage drop and left WB19 day-1 lateral transfer as
the next dominant storage-loss surface. This package diagnoses, corrects, and
validates the WB19 lateral storage-availability lineage feeding post-WB19 WB17
root uptake. The observable outcome is a contract-backed decision on whether
WB19 lateral transfer now preserves baseline-authoritative active-layer,
capacity, and aggregate `watcon` semantics for H1/H13/H39 and the full
`H1..H39` semantic suite.

## Objective

Execute the HPHYS0251 continuation recommendation by targeting upstream
storage availability before WB17 root uptake: `wb18_perc_theta_####`,
`wb11_soil_water`, `watcon`, `thetdr_####`, `dg_####`, WB18/WB19 mutation
timing, and WB13 aggregate publication. The primary implementation focus is
WB19 lateral transfer, because HPHYS0246 identified it as the remaining
dominant post-WB18 day-1 storage withdrawal.

## Rationale

HPHYS0251 final H1/H13/H39 traces show `UPi≈Etp` but `Ws≈0.05`, while
candidate aggregate `Total-Soil` means are far below baseline. HPHYS0246 shows
WB18 now preserves aggregate `watcon` lineage and the remaining day-1
discontinuity is WB19 lateral transfer. Therefore additional SWU tuning would
only draw down already-insufficient storage. The next defensible slice is
contract-first WB19 lateral storage availability.

## Included Scope

- Amend canonical `SC-SUBHYD-001` and `SC-WATBAL-001` authority for any
  diagnosed WB19 lateral storage-availability correction.
- Add contract-derived tests before production edits.
- Diagnose current WB19 lateral active-layer, available-capacity, frozen-water,
  and aggregate-storage mutation lineage against the pinned baseline
  `/workdir/wepp-forest_260430_baseline`.
- Correct only baseline-authoritative WB19 lateral storage semantics that can
  be proven from the canonical contracts and pinned baseline source.
- Run targeted H1/H13/H39 telemetry plus full `H1..H39` semantic metrics.
- Record dual review and dual verification artifacts. If independent agents
  are unavailable, label the artifacts truthfully as same-agent static review.

## Excluded Scope

- Do not tune `Ep`, `Es`, `Dp`, `Pe`, `Q`, or `Snow-Water` residuals directly.
- Do not inflate `wb11_soil_water`, `Total-Soil`, or `SoilWaterTotal` with a
  heuristic compensation term.
- Do not patch WB13 publication to hide upstream storage errors.
- Do not change snow, runoff, or plant-growth physics except where a WB19
  storage contract requires their existing symbols as diagnostics.
- Do not close the package as semantic parity if full-suite residuals remain
  unresolved.

## Deliverables

1. Package-local evidence artifacts under this directory.
2. Contract amendments in canonical `SC-*` files when production behavior is
   changed.
3. Contract-derived tests that fail before the production correction and pass
   after it.
4. Production WB19 lateral storage-availability correction, if contract and
   diagnostics identify a proven defect.
5. Targeted H1/H13/H39 telemetry focused on WB18/WB19/WB17 storage handoff.
6. Full `H1..H39` semantic metrics and continuation disposition.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0246-wb18-aggregate-storage-writeback-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0251-swu-root-uptake-stress-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/drain.for`

## Intended Write Set

- `docs/work-packages/20260602-hphys0252-wb19-lateral-storage-availability-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs`
- `tests/integration/wb17_et_physics_kernel_contract.rs` only if the final
  storage handoff requires a WB17-facing assertion.
- `crates/openwepp-runner/src/hillslope/mod.rs` only if diagnostics need
  trace-surface additions.

## Contract-First Sequence

1. Implement required contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production code.

No kernel code edits are allowed before steps 1-3 are complete.

## Phase Plan

### Phase A — Contract and Baseline Diagnosis

Read the required documents and baseline source. Compare current Rust WB19
lateral transfer against baseline `watbal_hourly.for` lateral lines for active
layer selection, frozen-water thresholding, conductivity weighting, capacity
cap, withdrawal order, and aggregate `watcon` mutation. Record the diagnosis in
`artifacts/wb19-lateral-storage-diagnosis.md`.

### Phase B — Contract and Test Gate

Amend canonical contracts for the proven defect. Add contract-derived tests that
demonstrate the defect and expected behavior. Run targeted pre-implementation
tests and record failing evidence in `artifacts/pre-implementation-contract-gate.md`.

### Phase C — Production Correction

Apply the minimal baseline-authoritative WB19 correction. Preserve typed
fail-closed guards and avoid silent defaults. Run targeted tests and update
`artifacts/implementation-test-evidence.md`.

### Phase D — Metrics, Review, and Disposition

Run targeted H1/H13/H39 telemetry and the full `H1..H39` semantic suite. Record
metrics, dual review artifacts, verification artifacts, gate results, and final
`HOLD`/`GO` disposition. If `Ep` and aggregate storage residuals do not improve
together, keep disposition `HOLD` and name the next focus.

## Exit Criteria

- Canonical contracts and tests cover the implemented behavior.
- Production changes trace to pinned baseline source, not heuristic math.
- Targeted H1/H13/H39 telemetry records WB18/WB19/WB17 storage handoff.
- Full `H1..H39` semantic metrics exist.
- All claims in artifacts are labeled `Static:` or `Ran:`.
- Full Rust gates are run or explicitly marked not-run with rationale.
- Disposition is truthful; unresolved semantic parity remains `HOLD`.

## Security-Impact Gate

No external systems or network actions are required. Work is local repository
engineering over flat files and local test/diagnostic commands. No secrets,
credentials, user data, or production service state are accessed or modified.

## Progress

- [x] (2026-06-02) Scaffolded HPHYS0252 from HPHYS0251 continuation and
  HPHYS0246 WB19 handoff evidence.
- [x] (2026-06-02) Completed contract/baseline diagnosis.
- [x] (2026-06-02) Added contract amendments and red tests.
- [x] (2026-06-02) Implemented baseline-authoritative WB19 `fzdrfc`
  lateral capacity/withdrawal correction.
- [x] (2026-06-02) Ran targeted diagnostics and full `H1..H39` suite.
- [x] (2026-06-02) Recorded same-agent reviews, verification, and
  disposition.

## Surprises & Discoveries

- Observation: baseline hourly lateral transfer uses frozen-adjusted
  `fzdrfc = max(drfc-frzw,0)` for lateral capacity and withdrawal floors, but
  still uses unfrozen `drfc` for hourly conductivity `fffx`.
  Evidence: `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:629`
  through `watbal_hourly.for:814`.
- Observation: the contract-derived vector failed before production edits with
  `q=0.10000000000000009` instead of the baseline-authoritative `q=0.5`.
  Evidence:
  `artifacts/gate-logs/pre_implementation_hphys0252_wb19.log`.
- Observation: full H1/H13/H39 selected outputs are unchanged from HPHYS0251;
  the current 39-suite does not exercise a non-zero WB19 `frzw` correction on
  those selected water-balance surfaces.
  Evidence:
  `/tmp/hphys0252_20260602T195147Z/reports/targeted_h1_h13_h39_diagnostics.md`
  and
  `/tmp/hphys0252_20260602T195147Z/reports/hphys0252_apples_to_apples_delta_from_hphys0251.md`.

## Decision Log

- Decision: Scope HPHYS0252 to WB19 lateral storage availability before root
  uptake.
  Rationale: HPHYS0251 points upstream from SWU, and HPHYS0246 names WB19
  lateral transfer as the remaining post-WB18 dominant day-1 storage loss.
  Date/Author: 2026-06-02 / Codex.
- Decision: Keep disposition `HOLD` after implementing the `fzdrfc` correction.
  Rationale: the correction is baseline-authoritative and contract-tested, but
  apples-to-apples HPHYS0251 vs HPHYS0252 full-suite metrics show no movement
  on `Ep`, `Total-Soil`, `SoilWaterTotal`, `Dp`, `latqcc`, `Q`, `RM`, or
  `Snow-Water`; remaining residuals require upstream storage/forcing lineage
  work rather than lateral frozen-threshold tuning.
  Date/Author: 2026-06-02 / Codex.

## Outcomes & Retrospective

- Outcome: canonical contracts now bind WB19 lateral frozen-adjusted storage
  availability to `SC-SUBHYD-001#INV-SUBHYD-025` and
  `SC-WATBAL-001#INV-WATBAL-040`.
- Outcome: WB19 lateral production now separates capacity-active layers
  (`fzdrfc`) from conductivity-active layers (`drfc`), matching the pinned
  hourly baseline.
- Outcome: full `H1..H39` runtime completed `39/39`; semantic pass remains
  `0/39`.
- Outcome: disposition is `HOLD`; next work should target WB11 seed/runtime
  storage scale, `watcon`/`st(i)` lineage, and the snow/runoff timing surfaces
  that dominate storage availability into WB17.

## Idempotence and Recovery

All edits are additive or contract/test/code changes under git. If diagnostics
fail due to missing temporary data, regenerate traces rather than reusing stale
`/tmp` paths. If a contract-derived test reveals the current code is already
correct, record the diagnostic result and keep production code unchanged.
