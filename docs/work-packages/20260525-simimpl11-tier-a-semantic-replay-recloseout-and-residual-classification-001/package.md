# 20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001

## Status
- state: completed-with-hold
- date: 2026-05-25
- timezone: UTC

## Objective
Execute SIMIMPL11 end-to-end by re-running strict and semantic Tier-A replay
after SIMOUT/SIMMODE/SIMCOUP closure packages, classifying residuals by
confidence-tier governance, and producing an evidence-backed promote/hold
posture for downstream disposition.

## Why This Package Exists
SIMIMPL06 replaced projection-first publication with simulation-owned output.
SIMIMPL07/09 closed mode and lane-policy foundations, and SIMIMPL10 closed
winter/soil/frsoil/hydout coupling vectors. SIMIMPL11 now re-closeouts replay
status under the updated production path and converts residuals into explicit
classification and ownership artifacts.

## Scope
### Included
- Re-run strict replay and semantic replay flows against current production
  behavior after SIMIMPL06 and SIMIMPL10 closure.
- Capture replay execution evidence, result deltas, and blocker sets.
- Classify residuals using confidence-tier governance (acceptance signal vs
  investigation signal) and assign promote/hold disposition posture.
- Record replay residual ownership and next-action recommendations for
  SIMIMPL12 disposition package.
- Complete governance/review/verification/disposition artifacts.

### Explicitly Out of Scope
- New production physics or coupling implementation changes outside scoped
  replay validation and evidence updates.
- Route/impoundment deferred coupling intake not yet authorized.
- Final hold-lift decision and next-wave queue finalization (`SIMIMPL12`).

## Deliverables
1. Contract/authority evidence:
   - `artifacts/simimpl11-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl11-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl11-preimplementation-contract-gate.md`
4. Implementation/test evidence log:
   - `artifacts/simimpl11-implementation-and-test-evidence.md`
5. Kernel-profile compliance checklist:
   - `artifacts/simimpl11-kernel-profile-compliance-checklist.md`
6. Replay execution plan:
   - `artifacts/simimpl11-replay-execution-plan.md`
7. Strict replay results summary:
   - `artifacts/simimpl11-strict-replay-results.md`
8. Semantic replay results summary:
   - `artifacts/simimpl11-semantic-replay-results.md`
9. Residual classification matrix:
   - `artifacts/simimpl11-residual-classification-matrix.md`
10. Replay blocker register:
   - `artifacts/simimpl11-replay-blocker-register.md`
11. Promote/hold recommendation:
   - `artifacts/simimpl11-promote-hold-recommendation.md`
12. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl11_disposition.md`
   - `artifacts/worker-handoff.md`
13. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
14. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
SIMIMPL11 is replay/validation focused and must preserve contract-first posture:
1. confirm canonical contract authority and required closure prerequisites are
   complete,
2. confirm replay harness/tests are aligned with contract surfaces,
3. record SIMIMPL11 pre-implementation gate evidence,
4. then execute replay runs and produce residual classifications.

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
- Replay interpretation must follow confidence-tier governance from ADR-0011 and
  numerics policy.
- No residual masking: unresolved mismatches must be explicitly classified and
  owned.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/numerics/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-pipeline-gap-audit.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl06-simulation-owned-wb13-output-publication-001/artifacts/simimpl06_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/artifacts/simimpl10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/artifacts/simimpl09_disposition.md`
- `/workdir/wepp-forest_260430_baseline`

## Intended Write Set
- `docs/work-packages/20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001/**`
- `docs/work-packages/README.md`
- `docs/reports/**` (replay evidence outputs if generated)
- comparator harness/config paths only if required for replay execution fidelity
  and explicitly recorded in owned-file manifest

## Phase Plan
### Phase A - Prerequisite Intake and Replay Plan
- Confirm queue authorization and upstream package closure prerequisites
  (`SIMIMPL06`, `SIMIMPL10`).
- Finalize strict/semantic replay execution plan and comparator inputs.

### Phase B - SIMIMPL11 Pre-Implementation Contract Gate
- Record SIMIMPL11 pre-implementation gate evidence and replay release
  conditions.

### Phase C - Replay Execution
- Run strict replay and capture full evidence.
- Run semantic replay and capture full evidence.

### Phase D - Residual Classification
- Classify residuals using confidence-tier rules.
- Build blocker register and promote/hold recommendation.

### Phase E - Validation and Evidence Consolidation
- Run required repository gates if production code changes were introduced.
- Consolidate replay evidence, ownership, and residual disposition tables.

### Phase F - Review, Verification, Disposition
- Complete dual review/disposition + dual verification.
- Finalize gate results, owned-file manifest, and worker handoff.

## Exit Criteria
- Strict and semantic replay evidence are both updated and traceable.
- Residuals are explicitly classified with ownership and confidence-tier tags.
- Promote/hold recommendation is explicit and evidence-backed.
- Required tests/gates are run and recorded when code changes are introduced:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Governance/review/verification artifacts are complete with no queued
  placeholders.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: replay validation and evidence-classification wave; no network or
  privilege-surface expansion expected.
