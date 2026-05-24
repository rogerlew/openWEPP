# 20260524-simimpl05-runner-orchestrator-daily-execution-integration-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute SIMIMPL05 end-to-end by wiring the production runner path to execute
hillslope scheduler/kernel lifecycle for the daily lane, with typed error
propagation, deterministic writeback governance, and evidence-backed closure of
`GAP-SIMPIPE-001`.

## Why This Package Exists
SIMIMPL03 closed contract authority gaps and SIMIMPL04 implemented
contract-derived pre-implementation tests/gate evidence. SIMIMPL05 is the first
authorized production integration package and must convert the validated
contract/test requirements into executable daily runner/orchestrator behavior.

## Scope
### Included
- Integrate runner daily execution path with orchestrator scheduler/kernel
  execution flow.
- Ensure output-emission preconditions depend on executed simulation state for
  daily lane (simulation-owned output publication refinements remain SIMIMPL06
  scope).
- Propagate and preserve typed failure surfaces (no silent fallback).
- Add/adjust tests needed to validate daily execution integration behavior and
  gate regressions.
- Complete package governance artifacts, reviews, verification, and
  disposition.

### Explicitly Out of Scope
- Hourly lane branch closure (`wepp_ui` mode propagation) beyond daily lane
  requirements (SIMIMPL07 scope).
- Full simulation-owned WB13/H.wat publication replacement (SIMIMPL06 scope).
- Consolidated-kernel intake triage/adoption decisions (SIMIMPL08 scope).

## Deliverables
1. Contract/authority evidence:
   - `artifacts/simimpl05-contract-implementation-evidence.md`
2. Contract-test prerequisite evidence:
   - `artifacts/simimpl05-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl05-preimplementation-contract-gate.md`
4. Implementation/test evidence log:
   - `artifacts/simimpl05-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl05-kernel-profile-compliance-checklist.md`
6. Integration design + code touch map:
   - `artifacts/simimpl05-runner-orchestrator-daily-integration-map.md`
7. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl05_disposition.md`
   - `artifacts/worker-handoff.md`
8. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
9. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL05 must execute in this order:
1. confirm canonical contract amendments required for this scope are complete
   and authoritative (SIMIMPL03),
2. confirm contract-derived tests/pre-gate prerequisites are present and valid
   (SIMIMPL04),
3. record SIMIMPL05 pre-implementation contract gate evidence,
4. then implement production runner/orchestrator integration edits.

## Autonomous Execution Intent (Required)
This package is execution-ready and self-contained. Assigned agents must execute
all phases through disposition without requesting additional user direction
unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections.

## Physics and Authority Posture
- Baseline comparator authority remains
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Contract authority for this package is defined by amended canonical surfaces:
  `SC-WATBAL-001`, `SC-SYSTEM-001`, `SC-INFILE-WEPPUI-001`.
- No physics invention; maintain legacy symbol continuity and explicit aliasing
  where names diverge.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-contract-derived-test-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-expected-fail-pass-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04_disposition.md`

## Intended Write Set
- `docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/tests/**`
- (if required for integration-only surface wiring)
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`

## Phase Plan
### Phase A - Contract/Test Prerequisite Intake
- Confirm SIMIMPL03 contract amendments are authoritative for this scope.
- Confirm SIMIMPL04 contract-derived tests and pre-gate evidence are complete.

### Phase B - SIMIMPL05 Pre-Implementation Contract Gate
- Record SIMIMPL05 pre-implementation contract gate evidence and release
  criteria for production edits.

### Phase C - Daily Runner/Orchestrator Integration
- Implement production runner integration for daily scheduler/kernel execution.
- Preserve typed error propagation and writeback governance invariants.

### Phase D - Validation and Closure Evidence
- Run targeted tests/gates and capture evidence with explicit pass/fail
  interpretation.
- Update integration map and owned-file manifest.

### Phase E - Review, Verification, Disposition
- Complete dual review/disposition + dual verification.
- Finalize gate results and worker handoff.

## Exit Criteria
- Runner daily path executes scheduler/kernel lifecycle in production flow.
- No silent fallback to projection-only daily behavior for the integrated path.
- Typed failure surfaces are preserved and tested.
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
- Rationale: internal execution-path integration in runner/orchestrator;
  no network or privilege-surface expansion.
