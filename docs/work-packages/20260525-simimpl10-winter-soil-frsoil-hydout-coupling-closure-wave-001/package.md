# 20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute SIMIMPL10 end-to-end by closing legacy winter/soil/frsoil/hydout
coupling gaps in the production execution path with typed invariants, explicit
boundary provenance, and no silent fallback behavior.

## Why This Package Exists
SIMIMPL09 closed typed hourly lane/timestep policy and adapter-boundary
foundation. SIMIMPL10 closes `GAP-SIMCOUP-001` by integrating frozen-soil and
winter coupling vectors (`winter`, `soil`, `frsoil`, hydout-equivalent
boundaries) into production flow so coupling behavior is contract-governed,
validated, and explicitly dispositioned.

## Scope
### Included
- Close runtime coupling vectors for winter/frozen-soil state and hydrology
  boundary behavior across runner/orchestrator/kernel handoff surfaces.
- Implement typed coupling surfaces and guards for `winter`, `soil`, `frsoil`,
  and hydout-equivalent boundary signals in production execution flow.
- Preserve SIMIMPL09 typed lane/timestep-policy semantics while adding coupling
  behavior.
- Add/extend contract-derived tests required for coupling vector closure in this
  wave.
- Record coupling-vector pass/fail matrix and explicit unresolved residual
  disposition.
- Complete governance/review/verification/disposition artifacts.

### Explicitly Out of Scope
- Route/impoundment coupling expansion deferred by SIMIMPL08 (`defer` class).
- Tier-A semantic replay recloseout and residual promotion (`SIMIMPL11`).
- New physics not already authorized by canonical `SC-*` contracts.
- Sub-hourly physics activation (remains out of scope).

## Deliverables
1. Contract/authority evidence:
   - `artifacts/simimpl10-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl10-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl10-preimplementation-contract-gate.md`
4. Implementation/test evidence log:
   - `artifacts/simimpl10-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl10-kernel-profile-compliance-checklist.md`
6. Coupling-vector integration map:
   - `artifacts/simimpl10-coupling-vector-integration-map.md`
7. Coupling validation matrix:
   - `artifacts/simimpl10-coupling-validation-matrix.md`
8. Unresolved coupling residual register:
   - `artifacts/simimpl10-unresolved-coupling-residual-register.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl10_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL10 must execute in this order:
1. ratify or amend canonical contract authority for coupling surfaces if any
   required authority is missing,
2. implement/extend coupling-focused contract-derived tests,
3. record SIMIMPL10 pre-implementation contract gate evidence,
4. then implement production coupling edits.

No production code edits before step 3 is complete.

## Autonomous Execution Intent (Required)
This package is execution-ready and self-contained. Assigned agents must execute
all phases through disposition without requesting additional user direction
unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections.

## Physics and Authority Posture
- Baseline comparator/provenance authority remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No physics invention is permitted.
- Coupling vectors must map to canonical contract invariants and typed guards;
  no silent fallback wrappers or implicit clamping.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-contract-invariant-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-owner-surface-gap-closure-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-contract-derived-test-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/artifacts/simimpl09_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/artifacts/simimpl09-adapter-boundary-closure-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/artifacts/simimpl09-timestep-policy-surface-map.md`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest`

## Intended Write Set
- `docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/**`
- `crates/openwepp-sim-contract/src/**`
- `crates/openwepp-kernel-contract/src/**`
- `crates/openwepp-climate-runtime-adapter/src/**`
- `crates/openwepp-hillslope-output/src/**`

## Phase Plan
### Phase A - Authority and Coupling-Prerequisite Intake
- Confirm queue authorization, SIMIMPL09 closure posture, and coupling gaps from
  SIMIMPL02 crosswalk artifacts.

### Phase B - Contract Ratification and Test Design
- Amend canonical contracts if needed for missing coupling authority.
- Implement/extend coupling-focused contract-derived tests.

### Phase C - SIMIMPL10 Pre-Implementation Contract Gate
- Record SIMIMPL10 gate evidence and explicit release conditions for production
  coupling edits.

### Phase D - Production Coupling Integration
- Implement winter/soil/frsoil/hydout-equivalent coupling behavior through
  typed runtime surfaces.
- Enforce explicit typed failures for domain/availability violations.

### Phase E - Validation and Evidence
- Run targeted and workspace gates for touched code.
- Record coupling-vector validation matrix and unresolved residual register.

### Phase F - Review, Verification, Disposition
- Complete dual review/disposition + dual verification.
- Finalize gate results, owned-file manifest, and worker handoff.

## Exit Criteria
- Coupling vectors for winter/soil/frsoil/hydout-equivalent behavior execute
  through typed runtime surfaces with explicit provenance.
- No silent fallback/default/clamping behavior remains in promoted coupling
  pathways.
- Coupling validation matrix is complete with pass/fail interpretation.
- Unresolved coupling gaps, if any, are explicitly recorded and dispositioned.
- Required tests/gates for touched code are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Governance/review/verification artifacts are complete with no queued
  placeholders.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: internal process-coupling integration; no network or
  privilege-surface expansion.
