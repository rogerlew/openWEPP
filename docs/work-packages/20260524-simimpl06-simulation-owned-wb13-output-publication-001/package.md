# 20260524-simimpl06-simulation-owned-wb13-output-publication-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute SIMIMPL06 end-to-end by replacing projection-first WB13/H.wat emission
with simulation-owned output assembly and provenance-complete publication
surfaces, closing `GAP-SIMOUT-001` and unblocking replay recloseout work.

## Why This Package Exists
SIMIMPL05 integrates daily runner -> scheduler/kernel execution ownership.
SIMIMPL06 must now ensure WB13/H.wat publication is sourced from executed
simulation state rather than first-day projection synthesis, with explicit
provenance fields and typed guard behavior.

## Scope
### Included
- Replace projection-first publication path for WB13/H.wat with
  simulation-owned publication built from executed scheduler/kernel outputs.
- Enforce provenance-complete publication metadata/reporting for daily lane.
- Preserve typed error propagation and prohibit silent publication fallback.
- Add/adjust integration tests validating publication-source authority and
  provenance fields.
- Complete package governance, review, verification, and disposition artifacts.

### Explicitly Out of Scope
- Hourly branch mode-propagation closure (`SIMIMPL07` scope).
- Consolidated-kernel intake triage/adoption decisions (`SIMIMPL08` scope).
- Replay rerun and residual classification (`SIMIMPL11` scope).

## Deliverables
1. Contract/authority evidence:
   - `artifacts/simimpl06-contract-implementation-evidence.md`
2. Contract-test prerequisite evidence:
   - `artifacts/simimpl06-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl06-preimplementation-contract-gate.md`
4. Implementation/test evidence log:
   - `artifacts/simimpl06-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl06-kernel-profile-compliance-checklist.md`
6. Publication provenance map:
   - `artifacts/simimpl06-wb13-publication-provenance-map.md`
7. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl06_disposition.md`
   - `artifacts/worker-handoff.md`
8. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
9. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL06 must execute in this order:
1. confirm canonical contract authority is complete and authoritative
   (SIMIMPL03),
2. confirm contract-derived tests/pre-gate prerequisites are present and valid
   (SIMIMPL04),
3. confirm SIMIMPL05 execution-path integration closure for daily lane,
4. record SIMIMPL06 pre-implementation contract gate evidence,
5. then implement production publication path edits.

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
- Contract authority for publication ownership/provenance is defined by:
  `SC-WATBAL-001`, `SC-SYSTEM-001`, and SIMIMPL03 amendments.
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
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-contract-derived-test-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-expected-fail-pass-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05-runner-orchestrator-daily-integration-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05_disposition.md`

## Intended Write Set
- `docs/work-packages/20260524-simimpl06-simulation-owned-wb13-output-publication-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-output/src/**`

## Phase Plan
### Phase A - Contract/Test/Integration Prerequisite Intake
- Confirm SIMIMPL03 authority, SIMIMPL04 test gate outputs, and SIMIMPL05 daily
  integration readiness.

### Phase B - SIMIMPL06 Pre-Implementation Contract Gate
- Record SIMIMPL06 pre-implementation gate evidence and release conditions for
  publication-path edits.

### Phase C - Simulation-Owned Publication Integration
- Implement simulation-owned WB13/H.wat publication path using executed
  scheduler/kernel state.
- Remove or gate projection-first publication behavior from production flow.

### Phase D - Validation and Provenance Evidence
- Run targeted tests and workspace gates for touched code.
- Record provenance map and pass/fail interpretation.

### Phase E - Review, Verification, Disposition
- Complete dual review/disposition + dual verification.
- Finalize gate results, owned-file manifest, and worker handoff.

## Exit Criteria
- WB13/H.wat publication is simulation-owned in production daily lane path.
- Publication surfaces expose provenance-complete fields/evidence.
- No silent fallback to projection-first publication remains in integrated
  production path.
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
- Rationale: internal output-provenance integration; no network or privilege
  expansion.
