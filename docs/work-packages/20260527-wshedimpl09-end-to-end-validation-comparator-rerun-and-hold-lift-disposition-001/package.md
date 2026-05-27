# 20260527-wshedimpl09-end-to-end-validation-comparator-rerun-and-hold-lift-disposition-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED09 by running required watershed validation lanes and available
comparator evidence paths after WSHED08, then publish explicit GO/HOLD
disposition for watershed routing/orchestration/parquet closure with residual
ownership.

## Why This Package Exists
WSHED08 activated watershed parquet publication and closed
`GAP-SYSTEM-006`. Program-level watershed closure still requires explicit
WSHED09 validation/comparator disposition across residual system gaps.

## Scope
### Included
- Execute required repository gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Execute watershed-focused validation lanes:
  - WS10/WS11/WS12 contract-derived runtime vectors.
  - watershed CLI end-to-end publication vector lane.
- Execute available comparator evidence rerun lanes and classify by confidence
  tier.
- Update canonical system closure posture if evidence supports amendment.
- Publish explicit GO/HOLD decision and residual follow-on ownership.
- Complete required package artifacts (evidence, reviews, verifications,
  disposition, handoff).

### Explicitly Out of Scope
- New production kernel/process-physics implementation.
- Closing active-structure projection and full channel sediment process gaps
  without dedicated implementation packages.

## Deliverables
1. `artifacts/wshedimpl09-watershed-validation-and-comparator-rerun-report.md`
2. `artifacts/wshedimpl09-hold-lift-decision-report.md`
3. `artifacts/wshedimpl09-contract-implementation-evidence.md`
4. `artifacts/wshedimpl09-contract-test-implementation-evidence.md`
5. `artifacts/wshedimpl09-preimplementation-contract-gate.md`
6. `artifacts/wshedimpl09-implementation-and-test-evidence.md`
7. `artifacts/wshedimpl09-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/wshedimpl09_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
WSHED09 is primarily rerun/disposition scope. If contract authority changes are
required by findings, apply sequence:
1. amend canonical contracts,
2. amend contract-derived tests as needed,
3. record gate evidence, then
4. apply production changes (not expected in this package).

## Autonomous Execution Intent (Required)
This package must execute end-to-end through reruns, evidence capture, and
final disposition without requesting additional user direction unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` labels.

## Provenance and Authority Posture
- Canonical authority remains in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts provide evidence and disposition context; they do not
  replace canonical contract authority.
- Comparator evidence must be confidence-tier classified per ADR-0011 and
  numerics policy.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl08-watershed-output-row-model-and-parquet-writer-activation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl08-watershed-output-row-model-and-parquet-writer-activation-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/numerics/README.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`
- `/workdir/openWEPP/tests/integration/ws10_watershed_kernel_contract.rs`
- `/workdir/openWEPP/tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `/workdir/openWEPP/tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `/workdir/openWEPP/crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl09-end-to-end-validation-comparator-rerun-and-hold-lift-disposition-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (if needed)
- `docs/specifications/science-contracts/index.md` (if needed)

## Phase Plan
### Phase A - Intake and prerequisite confirmation
- Confirm WSHED09 queue authorization and WSHED08 completion status.

### Phase B - Watershed rerun evidence collection
- Execute watershed-focused test lanes and capture results.
- Execute available comparator evidence rerun path and classify confidence tier.

### Phase C - Required repository gates
- Run required validation commands and capture outcomes.

### Phase D - Governance and hold-lift decision
- Update artifacts with evidence and issue explicit GO/HOLD disposition.

### Phase E - Disposition and handoff
- Publish final disposition, residual ownership, and follow-on package guidance.

## Exit Criteria
- Required repository gates executed and recorded.
- Watershed validation lanes rerun and recorded.
- Comparator evidence path outcome recorded and confidence-tier classified.
- Explicit GO/HOLD decision with residual ownership is published.
- Required artifacts are complete with truthful evidence labeling.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: validation/disposition package only; no external connectivity or
  secret handling changes.

## Execution Outcome Summary
- WSHED09 validation reruns completed:
  - watershed WS10/WS11/WS12 contract-derived lanes pass,
  - watershed CLI end-to-end non-stub parquet emission lane passes,
  - full repository gates pass (`fmt`, `clippy`, `test --workspace`,
    `deny check` with existing warnings only).
- Comparator-tier routing evidence reruns completed and confirm watershed
  surfaces classify as `investigation` confidence tier.
- Final disposition remains `HOLD`:
  - `GAP-SYSTEM-005` remains non-promotable (baseline-authoritative watershed
    end-to-end comparator lane still absent),
  - `GAP-SYSTEM-007` and `GAP-SYSTEM-008` remain open non-promotable physics
    blockers outside WSHED09 validation-only scope.
