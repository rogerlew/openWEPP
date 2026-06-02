# 20260602-hphys0246-wb18-aggregate-storage-writeback-closure-001

## Status
- state: completed
- date: 2026-06-02
- timezone: America/Los_Angeles
- decision: HOLD_PENDING_WB19_DAY1_LATERAL_CLOSURE_AND_DUAL_REVIEW_VERIFICATION

## Objective
Implement contract-first WB18 aggregate storage writeback closure so
`wb11_soil_water` preserves baseline-authoritative total soil-water semantics
across percolation, then rerun H1/H7/H39 telemetry to reassess residuals.

## Why This Package Exists
HPHYS0245 showed the first H1/H7/H39 hillslope water-balance discontinuity at
WB18 `percolation_deep_seepage`: openWEPP currently writes `wb11_soil_water`
from `Σ wb18_perc_theta_####`, dropping the legacy `soilw = st + thetdr*dg`
component. HPHYS0246 corrects that handoff using canonical contract authority
and contract-derived tests before production code edits.

## Scope
### Included
- Amend canonical WB18/WB11 aggregate-storage authority in `SC-PERC-001` and
  companion WB13/WATBAL lineage text in `SC-WATBAL-001` if required.
- Add contract-derived tests for WB18 aggregate storage writeback.
- Implement baseline-authoritative WB18 aggregate writeback in the hydrology
  kernel.
- Run targeted H1/H7/H39 telemetry using the HPHYS0245 diagnostics sidecar.
- Publish implementation evidence, gate results, residual analysis,
  disposition, and worker handoff.

### Explicitly Out of Scope
- WB19 lateral/drainage remediation beyond post-WB18 residual assessment.
- `D`/`Pe` tuning, clamps, or heuristic compensation formulas.
- Watershed rerun.
- Commit/push unless separately requested.

## Closure Measures
1. `MEASURE-HP246-001`: canonical contract authority no longer defines WB18
   aggregate writeback as `wb11_soil_water = Σtheta` when residual/dead-water
   layer surfaces are available.
2. `MEASURE-HP246-002`: contract-derived tests fail on the pre-HPHYS0246 WB18
   aggregate writeback and pass after implementation.
3. `MEASURE-HP246-003`: production WB18 writeback computes aggregate
   `wb11_soil_water` from baseline-authoritative `soilw` semantics:
   `Σ(st_i + thetdr_i * (dg_i - frozen_i))` with typed guard behavior.
4. `MEASURE-HP246-004`: H1/H7/H39 telemetry documents post-fix WB18 storage
   continuity and identifies any remaining WB19 residual focus.

## Deliverables
1. `artifacts/contract-implementation-evidence.md`
2. `artifacts/contract-test-implementation-evidence.md`
3. `artifacts/pre-implementation-contract-gate.md`
4. `artifacts/implementation-test-evidence.md`
5. `artifacts/kernel-profile-compliance-checklist.md`
6. `artifacts/hphys0246-telemetry-run-evidence.md`
7. `artifacts/hphys0246-residual-analysis.md`
8. `artifacts/focus-recommendations.md`
9. `artifacts/gate-results.md`
10. `artifacts/owned-file-manifest.md`
11. `artifacts/hphys0246_disposition.md`
12. `artifacts/worker-handoff.md`
13. `artifacts/science-contracts/SC-PERC-001/*`
14. `artifacts/science-contracts/SC-WATBAL-001/*`

## Mandatory Sequence
1. Implement required canonical contract amendments.
2. Implement contract-derived tests.
3. Record pre-implementation contract-gate evidence.
4. Modify production code.
5. Run targeted validation and telemetry.
6. Publish review/verification posture and disposition.

## Autonomous Execution Intent
Execute this package end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`. Do not mark
independent dual-agent review or verification complete unless independent agent
outputs exist.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001/`
- `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0246-wb18-aggregate-storage-writeback-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `tests/integration/auth03_level4_constitutive_gate_contract.rs`
- `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`
- `tests/integration/wb18_percolation_physics_kernel_contract.rs`

## Phase Plan
### Phase A - Contract Authority
Amend canonical contracts to require WB18 aggregate writeback to preserve
baseline total soil-water semantics and record provenance.

### Phase B - Contract-Derived Tests
Add tests that prove WB18 preserves the `thetdr * dg` residual component and
fails typed when required aggregate-publication symbols are missing or invalid.

### Phase C - Implementation
Update WB18 production writeback to compute `wb11_soil_water` from layer theta
plus residual/dead-water depth using explicit symbols and typed guard behavior.

### Phase D - Telemetry and Residual Assessment
Run H1/H7/H39 day `1..30` telemetry and compare WB18/WB19/WB13 residuals with
HPHYS0245.

### Phase E - Disposition
Publish evidence, gate results, dual-review posture, recommendation, and worker
handoff.

## Exit Criteria
- Contract authority and tests are in place before production edits.
- Targeted tests and telemetry run successfully.
- Residual analysis clearly states whether WB19 is now the dominant remaining
  focus or whether WB18 remains unresolved.
- If independent dual-agent review/verification is unavailable, disposition
  remains `HOLD` with that governance gap explicit.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local hydrology kernel, tests, docs, and local comparator runs;
  no credentials or network writes.
