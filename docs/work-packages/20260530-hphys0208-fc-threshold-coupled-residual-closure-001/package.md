# 20260530-hphys0208-fc-threshold-coupled-residual-closure-001

## Status
- state: queued
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0208 to close the coupled FC-threshold residual families in
hillslope `H.wat` outputs:
`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, and `SoilWaterTotal`.

## Why This Package Exists
HPHYS0204 disposition established that the saturated residual families are not
independent: `Dp`/`latqcc`/`Total-Soil`/`SoilWaterTotal` share kernel-side FC/WP
threshold input lineage (`thetfc_####`/`thetdr_####`) with `ProfileFCStore`.
HPHYS0208 is the coupled closure lane for that shared threshold lineage.

## Scope
### Included
- Canonical contract amendments for coupled threshold-lineage authority across:
  - FC/WP threshold symbols,
  - percolation and lateral-flow outputs,
  - soil-water aggregate publication.
- Contract-derived tests that explicitly exercise threshold propagation from
  authoritative FC/WP symbols into `ProfileFCStore`, `Dp`, `latqcc`,
  `Total-Soil`, and `SoilWaterTotal`.
- Minimal production runtime/publication updates required by those tests.
- 39-hillslope semantic rerun and residual-direction evidence.

### Explicitly Out of Scope
- `ProfileWPStore` near-closed adjudication lane (HPHYS0209 scope).
- Integrated HOLD/GO adjudication wave (HPHYS0210 scope).
- Watershed/channel/impoundment behavior changes.

## Closure Measures (Required)
1. `MEASURE-HP208-001`: fail-hillslope counts for `Dp`, `latqcc`,
   `Total-Soil`, and `SoilWaterTotal` are each reduced from `39` to `0` on the
   39-hillslope cohort.
2. `MEASURE-HP208-002`: fail-hillslope count for `ProfileFCStore` is reduced
   from `27` to `0` on the same cohort.
3. `MEASURE-HP208-003`: contract-derived coupled-lineage tests pass and assert
   fail-closed behavior for missing/non-finite/out-of-domain threshold inputs.
4. `MEASURE-HP208-004`: required workspace gates pass:
   `fmt`, `clippy`, `test`, `deny`.

## Deliverables
1. `artifacts/hphys0208-residual-gap-matrix.md`
2. `artifacts/hphys0208-contract-implementation-evidence.md`
3. `artifacts/hphys0208-contract-test-implementation-evidence.md`
4. `artifacts/hphys0208-preimplementation-contract-gate.md`
5. `artifacts/hphys0208-implementation-and-test-evidence.md`
6. `artifacts/hphys0208-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0208_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for coupled
   threshold-lineage authority.
2. Implement contract-derived tests for the coupled residual families.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply minimal production edits required by those tests.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Physics and symbol provenance must trace to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No surrogate/proxy process-physics substitutions are permitted.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/artifacts/hphys0204_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/artifacts/claude-code-review-findings.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0207-fcwp-depth-authority-tail-closure-001/artifacts/hphys0207_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0208-fc-threshold-coupled-residual-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0208_fc_threshold_coupled_residual_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0208 authorization from HPHYS0204 handoff and freeze scope to
  coupled FC-threshold residual families only.

### Phase B - Contract/spec authority updates
- Amend canonical contract rows for coupled threshold lineage and dependent
  publication families.
- Update science-contract index references for HPHYS0208.

### Phase C - Contract-derived tests
- Add tests that bind threshold-symbol authority to dependent publication
  outputs and enforce typed fail-closed guards.

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production edits.

### Phase E - Production implementation
- Implement minimal runtime/publication closure for the coupled families.

### Phase F - Validation and parity rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Rerun 39-hillslope semantic comparison and summarize deltas.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification artifacts and publish disposition.

## Exit Criteria
- Closure measures `MEASURE-HP208-001..004` are satisfied and evidenced.
- Coupled residual families are either closed to `0` fail hillslopes or
  explicitly held with bounded, contract-authoritative defect ownership.
- Handoff includes scoped next actions for HPHYS0209.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
