# 20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute SIMIMPL04 end-to-end by implementing contract-derived integration tests
that enforce production runner-to-scheduler execution ownership, `wepp_ui`
mode-closure invariants, and simulation-owned WB13/H.wat publication
requirements, then recording the pre-implementation contract gate that must
precede SIMIMPL05 production edits.

## Why This Package Exists
SIMIMPL03 closed contract authority gaps for SIMPIPE/SIMMODE/SIMOUT/SIMCONS.
The next mandatory contract-first step is to convert that authority into
failing-then-passing contract-derived tests and an explicit pre-implementation
gate before any production path changes.

## Scope
### Included
- Derive executable test requirements from amended canonical contracts:
  - `SC-WATBAL-001`
  - `SC-SYSTEM-001`
  - `SC-INFILE-WEPPUI-001`
- Implement integration tests and fixtures that validate:
  - runner path executes scheduler/kernel lifecycle (not projection-only path),
  - `requested` vs `effective` mode closure behavior is typed and deterministic,
  - WB13/H.wat publication provenance is simulation-owned.
- Record expected-fail/expected-pass semantics and explicit blocker rationale
  for SIMIMPL05 handoff.
- Produce pre-implementation contract gate evidence for downstream code package
  start authorization.
- Complete review/verification/disposition artifacts.

### Explicitly Out of Scope
- Production runner/orchestrator kernel behavior edits (SIMIMPL05 scope).
- Canonical contract amendments beyond contradiction logging (SIMIMPL03
  authority already established).
- Comparator replay reruns.

## Deliverables
1. Contract-derived test plan:
   - `artifacts/simimpl04-contract-derived-test-plan.md`
2. Expected fail/pass matrix:
   - `artifacts/simimpl04-expected-fail-pass-matrix.md`
3. Contract/authority evidence:
   - `artifacts/simimpl04-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/simimpl04-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/simimpl04-preimplementation-contract-gate.md`
6. Implementation/test evidence log:
   - `artifacts/simimpl04-implementation-and-test-evidence.md`
7. Kernel-profile compliance checklist:
   - `artifacts/simimpl04-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl04_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
11. Contract-derived test files (create/update as needed):
   - `crates/openwepp-runner/tests/simimpl04_runner_kernel_execution_contract.rs`
   - `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`
   - `crates/openwepp-runner/tests/simimpl04_wb13_publication_contract.rs`

## Mandatory Contract-First Sequence (Required)
SIMIMPL04 must complete before any production code edits in SIMIMPL05+.

Downstream sequence remains:
1. canonical contract amendments (SIMIMPL03),
2. contract-derived tests + pre-implementation gate (SIMIMPL04),
3. production edits (SIMIMPL05+).

## Autonomous Execution Intent (Required)
This package is execution-ready and self-contained. Assigned agents must execute
all phases through disposition without requesting additional user direction
unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label evidence mode using `Static:` and/
or `Ran:` sections.

## Physics and Authority Posture
- Baseline comparator authority remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Consolidated architecture intake remains selective and contract-gated.
- No physics invention is permitted.

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
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-owner-surface-gap-closure-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-contract-invariant-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-pipeline-gap-audit.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`

## Intended Write Set
- `docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/tests/simimpl04_runner_kernel_execution_contract.rs`
- `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`
- `crates/openwepp-runner/tests/simimpl04_wb13_publication_contract.rs`

## Phase Plan
### Phase A - Intake and Contract Requirement Extraction
- Extract executable test obligations from SIMIMPL03-amended contracts.
- Record invariant-to-test mapping and expected failure surfaces.

### Phase B - Contract-Derived Test Authoring
- Implement integration tests and any required fixtures.
- Ensure tests encode typed error/guard expectations and provenance assertions.

### Phase C - Pre-Implementation Contract Gate Recording
- Record expected-fail/expected-pass state and gate readiness for SIMIMPL05.
- Confirm no production-code edits were introduced in SIMIMPL04.

### Phase D - Review, Verification, and Disposition
- Complete dual review/disposition and dual verification.
- Finalize gate results, owned-file manifest, and worker handoff.

## Exit Criteria
- Contract-derived integration tests exist for SIMPIPE/SIMMODE/SIMOUT
  authority obligations.
- Expected-fail/expected-pass matrix is explicit and evidence-linked.
- Pre-implementation contract gate is completed and states SIMIMPL05 readiness
  posture.
- Review/verification/gate artifacts are complete with no queued placeholders.
- If code is changed beyond docs/tests, required repository gates are run and
  recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: test/gate package; no direct production execution-path edits
  authorized in this package.
