# 20260530-hphys0209-profilewp-near-closed-adjudication-001

## Status
- state: completed
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0209 to adjudicate the near-closed `ProfileWPStore` residual lane
(`1/39` fail hillslopes) and determine whether the remaining delta is:
1) an unresolved migration defect requiring implementation closure, or
2) an expected process-correct delta with explicit authority-backed acceptance.

## Why This Package Exists
HPHYS0204 separated `ProfileWPStore` from the larger FC/saturated residual
families and marked it as near-closed. This package isolates that one-column
lane to avoid conflating its closure posture with broader threshold-lineage
work.

## Scope
### Included
- Canonical contract clarification for `ProfileWPStore` publication/lineage
  obligations where needed.
- Contract-derived tests that target the one-column residual lane and guard
  hard-fail behavior.
- Minimal production updates only if evidence proves unresolved defect lineage.
- Cohort rerun evidence focused on `ProfileWPStore` with non-regression checks
  for `ProfileDepth`/`ProfilePorosityCap`.

### Explicitly Out of Scope
- Coupled FC + saturated residual-family closure lane (HPHYS0208 scope).
- Integrated final HOLD/GO adjudication (HPHYS0210 scope).
- Watershed/channel/impoundment changes.

## Closure Measures (Required)
1. `MEASURE-HP209-001`: `ProfileWPStore` fail-hillslope count moves from `1`
   to `0`, or is explicitly dispositioned as expected process-correct delta
   with canonical authority and reproducible evidence.
2. `MEASURE-HP209-002`: `ProfileDepth` and `ProfilePorosityCap` remain
   non-regressing (`0/39` fail hillslopes).
3. `MEASURE-HP209-003`: contract-derived lane-specific tests pass.
4. `MEASURE-HP209-004`: required workspace gates pass:
   `fmt`, `clippy`, `test`, `deny`.

## Deliverables
1. `artifacts/hphys0209-residual-gap-matrix.md`
2. `artifacts/hphys0209-contract-implementation-evidence.md`
3. `artifacts/hphys0209-contract-test-implementation-evidence.md`
4. `artifacts/hphys0209-preimplementation-contract-gate.md`
5. `artifacts/hphys0209-implementation-and-test-evidence.md`
6. `artifacts/hphys0209-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0209_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments.
2. Implement contract-derived tests for the near-closed WP lane.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply minimal production edits required by test/evidence outcomes.

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
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0208-fc-threshold-coupled-residual-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/artifacts/hphys0204_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/artifacts/claude-code-review-findings.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0209-profilewp-near-closed-adjudication-001/**`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0209_profilewp_adjudication_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0209 authorization from HPHYS0204 handoff and freeze to
  `ProfileWPStore` near-closed lane.

### Phase B - Contract/spec authority updates
- Amend canonical contract rows for WP lane adjudication obligations.
- Update science-contract index references for HPHYS0209.

### Phase C - Contract-derived tests
- Add WP-lane tests and required non-regression checks.

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production edits.

### Phase E - Production implementation (conditional)
- Implement minimal production fixes if defect lineage is confirmed.

### Phase F - Validation and rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Run cohort diagnostics and summarize `ProfileWPStore` outcome.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification artifacts and publish disposition.

## Exit Criteria
- Closure measures `MEASURE-HP209-001..004` are satisfied and evidenced.
- Handoff includes explicit input package for HPHYS0210 integrated adjudication.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
