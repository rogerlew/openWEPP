# 20260531-hphys0216-profilefc-layer-authority-realignment-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0216 to close `ProfileFCStore` structural split by realigning WB13
`ProfileFCStore` publication authority to baseline-authoritative layer
aggregation (`Σ(thetfc_i * dg_i) * 1000`) while preserving corrected-layer
projection and typed fail-closed guards.

## Why This Package Exists
HPHYS0214 integrated adjudication retained `HOLD` with `ProfileFCStore`
`27/39` fail hillslopes, and HPHYS0215 queued HPHYS0216 as the first execution
stream. Baseline authority in
`/workdir/wepp-forest_260430_baseline/src/watbal.for` and `watbalprint.for`
computes `profile_fcstore` from layer-level `thetfc(i)*dg(i)`, not the
normalized-profile storage seed symbol.

## Scope
### Included
- Contract-first authority correction for `ProfileFCStore` in canonical
  `SC-*` contracts.
- Contract-derived test updates/additions for FC publication authority and
  guard posture.
- Production implementation updates in hillslope runner for FC publication
  authority.
- Required workspace validation gates and targeted parity rerun evidence on the
  39-hillslope `unpalatable-rind` cohort.

### Explicitly Out of Scope
- `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal` remediation streams.
- Watershed/channel process work.
- Release/deployment packaging.

## Closure Measures (Required)
1. `MEASURE-HP216-001`: required workspace gates pass and are recorded.
2. `MEASURE-HP216-002`: canonical contract authority for `ProfileFCStore`
   explicitly maps to baseline layer aggregation lineage with provenance.
3. `MEASURE-HP216-003`: WB13 publication implementation consumes authoritative
   layer symbols for `ProfileFCStore` and preserves typed fail-closed guards.
4. `MEASURE-HP216-004`: 39-hillslope semantic rerun shows reduced
   `ProfileFCStore` fail hillslopes versus HPHYS0214 baseline (`27/39`).

## Deliverables
1. `artifacts/hphys0216-residual-gap-matrix.md`
2. `artifacts/hphys0216-contract-implementation-evidence.md`
3. `artifacts/hphys0216-contract-test-implementation-evidence.md`
4. `artifacts/hphys0216-preimplementation-contract-gate.md`
5. `artifacts/hphys0216-implementation-and-test-evidence.md`
6. `artifacts/hphys0216-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0216_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contracts for FC publication authority realignment.
2. Amend/add contract-derived tests covering authority and guard behavior.
3. Record pre-implementation contract gate evidence.
4. Modify production code and execute gates/reruns.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy baseline comparator provenance remains
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Migration closure target is baseline-authoritative process-physics mapping,
  not surrogate approximation.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0215-coupled-family-remediation-planning-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214-residual-gap-matrix.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for`
- `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0216-profilefc-layer-authority-realignment-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0216 authorization from HPHYS0215 handoff and freeze scope to
  `ProfileFCStore` only.

### Phase B - Contract authority amendments
- Amend canonical contracts to encode baseline-authoritative layer-aggregation
  authority for `ProfileFCStore`.

### Phase C - Contract-derived tests
- Update/add tests validating FC publication authority and fail-closed behavior.

### Phase D - Production implementation
- Update WB13 publication path in runner to consume authoritative layer symbols
  for `ProfileFCStore`.

### Phase E - Validation and diagnostics
- Execute required workspace gates.
- Execute 39-hillslope rerun + semantic summary for `ProfileFCStore` delta.

### Phase F - Disposition and handoff
- Publish residual matrix, decision, and immediate next package queue.

## Exit Criteria
- Closure measures `MEASURE-HP216-001..004` are satisfied and evidenced.
- HPHYS0217 handoff scope is explicit for next unresolved family.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: localized hydrology publication authority and contract/test updates;
  no auth/network boundary changes.
