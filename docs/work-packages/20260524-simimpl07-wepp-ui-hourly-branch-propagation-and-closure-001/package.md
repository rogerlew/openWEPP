# 20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001

## Status
- state: complete
- date: 2026-05-24
- timezone: UTC

## Objective
Execute SIMIMPL07 end-to-end by propagating parsed `wepp_ui` requested/effective
mode into runtime lane selection and enforcing strict typed closure for branch
mismatch, closing `GAP-SIMMODE-001`.

## Why This Package Exists
SIMIMPL03 established contract authority for mode-propagation invariants,
SIMIMPL04 introduced contract-derived mode-closure tests, and SIMIMPL05 closed
daily execution-path ownership. SIMIMPL07 must now implement deterministic mode
propagation so runtime lane selection is contract-authoritative and
misconfiguration is surfaced via typed failures.

## Scope
### Included
- Carry parsed `wepp_ui` mode surfaces (`requested`, `effective`) through
  production runtime flow.
- Bind runtime lane selection to mode surfaces with strict typed mismatch
  behavior.
- Remove/replace parse-only discard behavior for `wepp_ui` in runner flow.
- Activate and satisfy mode-closure contract-derived tests from SIMIMPL04.
- Record mode-propagation integration evidence, gate results, and disposition.

### Explicitly Out of Scope
- Full hourly kernel physics foundation and timestep policy surface
  (`SIMIMPL09` scope).
- Consolidated-kernel intake triage (`SIMIMPL08` scope).
- Replay rerun and residual classification (`SIMIMPL11` scope).

## Deliverables
1. Contract/authority evidence:
   - `artifacts/simimpl07-contract-implementation-evidence.md`
2. Contract-test prerequisite evidence:
   - `artifacts/simimpl07-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl07-preimplementation-contract-gate.md`
4. Implementation/test evidence log:
   - `artifacts/simimpl07-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl07-kernel-profile-compliance-checklist.md`
6. Mode propagation integration map:
   - `artifacts/simimpl07-mode-propagation-integration-map.md`
7. Mode closure test matrix:
   - `artifacts/simimpl07-mode-closure-test-matrix.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl07_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL07 must execute in this order:
1. confirm canonical contract authority is complete and authoritative
   (SIMIMPL03),
2. confirm contract-derived tests/pre-gate prerequisites are present and valid
   (SIMIMPL04),
3. confirm SIMIMPL05 daily execution-path integration closure,
4. record SIMIMPL07 pre-implementation contract gate evidence,
5. then implement production mode-propagation edits.

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
- Mode-propagation authority is defined by:
  `SC-INFILE-WEPPUI-001`, `SC-SYSTEM-001`, and SIMIMPL03 amendments.
- No silent defaulting/clamping of branch-selection mismatch is permitted.

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
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05-runner-orchestrator-daily-integration-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl06-simulation-owned-wb13-output-publication-001/artifacts/simimpl06_disposition.md`

## Intended Write Set
- `docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`
- `crates/openwepp-runner/tests/**`

## Phase Plan
### Phase A - Contract/Test/Integration Prerequisite Intake
- Confirm SIMIMPL03 authority, SIMIMPL04 mode-closure tests, and SIMIMPL05
  daily execution integration readiness.

### Phase B - SIMIMPL07 Pre-Implementation Contract Gate
- Record SIMIMPL07 pre-implementation gate evidence and release conditions for
  mode-propagation edits.

### Phase C - Mode Propagation and Typed Closure Integration
- Implement requested/effective mode propagation into runtime lane selection.
- Enforce typed mismatch closure and remove discard-path behavior.

### Phase D - Validation and Evidence
- Run targeted tests and workspace gates for touched code.
- Record mode-closure pass/fail evidence and residuals.

### Phase E - Review, Verification, Disposition
- Complete dual review/disposition + dual verification.
- Finalize gate results, owned-file manifest, and worker handoff.

## Exit Criteria
- Runtime lane selection is driven by propagated `wepp_ui` mode surfaces.
- Mode mismatch is surfaced as typed failure; no silent fallback remains.
- SIMIMPL04 mode-closure contract-derived tests are active and passing for the
  implemented scope.
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
- Rationale: internal runtime mode-selection integration; no network or
  privilege-surface expansion.
