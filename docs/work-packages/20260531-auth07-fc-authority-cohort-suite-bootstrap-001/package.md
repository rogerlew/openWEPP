# 20260531-auth07-fc-authority-cohort-suite-bootstrap-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: GO

## Objective
Bootstrap an independent field-capacity authority cohort suite that compares
model `ProfileFCStore` publication against direct `theta_fc(-33kPa)` authority,
with explicit thresholded classification and rock-fragment stratified reporting.

## Why This Package Exists
The AUTH05 worked example showed that legacy parity can remain near-closed while
both model and legacy disagree strongly with direct constitutive FC authority.
AUTH07 promotes that evidence into a reproducible, tracked, contract-derived
suite scaffold with fixture provenance and stable cohort checks.

## Scope
### Included
- Create an AUTH07 work-package and tracked artifact closure.
- Introduce a new external-authority suite:
  - `cas_l5_soil_fc_direct_theta_minus33_cohort_001`
- Add reproducible cohort fixture root with lock/provenance sidecars.
- Add contract-derived integration test for:
  - direct-authority vs model FC store comparison,
  - explicit relative-threshold classification,
  - rock-fragment bucket stratification.
- Add canonical contract/index references for AUTH07 suite posture.

### Explicitly Out of Scope
- Immediate promotion of this suite to required hard-fail lane.
- Production process-physics rewrites to close detected residuals.

## Deliverables
1. `artifacts/contract-implementation-evidence.md`
2. `artifacts/contract-test-implementation-evidence.md`
3. `artifacts/preimplementation-contract-gate.md`
4. `artifacts/implementation-and-test-evidence.md`
5. `artifacts/kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/governance references for AUTH07 cohort suite.
2. Add contract-derived AUTH07 tests and fixture metadata.
3. Record pre-implementation contract gate evidence.
4. Implement registry/suite/fixture scaffolding and publish disposition.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without additional user
direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical process authority remains in `SC-*` contracts.
- AUTH07 adds an independent external authority comparator and does not use
  legacy parity as acceptance oracle.
- Fixture provenance is required and locked for deterministic reruns.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
- `/workdir/openWEPP/docs/specifications/external-authority/README.md`
- `/workdir/openWEPP/docs/specifications/external-authority/suite-schema.md`
- `/workdir/openWEPP/docs/specifications/external-authority/registry.yaml`
- `/workdir/openWEPP/tests/fixtures/constitutive/cas_l5_soil_fc_direct_theta_minus33_cohort_001/h1_worked_example_source.md`

## Intended Write Set
- `.gitattributes`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth07-fc-authority-cohort-suite-bootstrap-001/**`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/suites/cas_l5_soil_fc_direct_theta_minus33_cohort_001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/fixtures/constitutive/cas_l5_soil_fc_direct_theta_minus33_cohort_001/**`
- `tests/integration/auth07_fc_authority_cohort_contract.rs`

## Phase Plan
### Phase A - Scope freeze and worked-example promotion plan
- Freeze AUTH07 scope around independent FC authority cohort suite bootstrap.

### Phase B - Contract/spec registration
- Register AUTH07 suite in registry and SC-SOIL addendum/index references.

### Phase C - Contract-derived test and fixtures
- Add reproducible cohort fixtures + sidecars + AUTH07 integration test.

### Phase D - Validation and artifact publication
- Run scoped validation gates and publish disposition.

## Exit Criteria
- AUTH07 suite is registered with periodic/investigation posture.
- Cohort fixture lock/provenance sidecars are present and valid.
- AUTH07 integration test enforces threshold + rock-bucket classification.
- Package artifacts capture evidence and disposition.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: docs/fixtures/tests/governance updates only.
