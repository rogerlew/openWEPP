# 20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED03 by implementing contract-derived watershed routing/impoundment/
system vectors for currently partial runtime closures, then record
pre-implementation gate evidence before WSHED04+ production migration work.

## Why This Package Exists
WSHEDIMPL01 completed WSHED02 contract-authority normalization and explicitly
deferred test-vector and pre-implementation gate closure to WSHED03. This
package executes that required contract-first step by authoring vectors for the
newly normalized unresolved rows and recording expected-failure baseline
evidence against current runtime posture.

## Scope
### Included
- Contract-derived vector authoring for unresolved rows:
  - `SC-ROUTE-001`: `GAP-ROUTE-008`, `GAP-ROUTE-009`
  - `SC-IMPOUND-001`: `GAP-IMPOUND-005`, `GAP-IMPOUND-006`
  - `SC-SYSTEM-001`: `GAP-SYSTEM-005`, `GAP-SYSTEM-006`,
    `GAP-SYSTEM-007`, `GAP-SYSTEM-008`
  - `SC-SED-001`: `GAP-SED-006`
- WS11/WS12 test-surface expansion and watershed CLI end-to-end fixture vector
  expansion.
- Pre-migration expected-failure baseline execution and recording.
- Pre-implementation gate evidence required before WSHED04+ runtime edits.
- Governance artifacts, dual review, dual verification, and disposition.

### Explicitly Out of Scope
- Production Rust runtime/kernel logic changes.
- Canonical `SC-*` authority rewrites except strictly minimal contradiction
  corrections required to keep vectors coherent.
- Watershed output writer activation/removal of `OWSOUT-E-004` (WSHED08 scope).

## Deliverables
1. `artifacts/wshedimpl02-contract-derived-test-matrix.md`
2. `artifacts/wshedimpl02-pre-migration-failure-baseline.md`
3. `artifacts/wshedimpl02-contract-implementation-evidence.md`
4. `artifacts/wshedimpl02-contract-test-implementation-evidence.md`
5. `artifacts/wshedimpl02-preimplementation-contract-gate.md`
6. `artifacts/wshedimpl02-implementation-and-test-evidence.md`
7. `artifacts/wshedimpl02-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/wshedimpl02_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting sequencing remains mandatory:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

WSHEDIMPL02 executes steps 2 and 3 for WSHED03 scope.

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through
disposition without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.
Do not claim command execution unless it was actually run.

## Provenance and Authority Posture
- Canonical authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed in production
  migration closure claims.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl01-watershed-contract-authority-closure-and-gap-normalization-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl01-watershed-contract-authority-closure-and-gap-normalization-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/tests/integration/ws10_watershed_kernel_contract.rs`
- `/workdir/openWEPP/tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `/workdir/openWEPP/tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `/workdir/openWEPP/crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
- `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/chrqin.for`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/impmai.for`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/**`
- `docs/work-packages/README.md`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHED03 queue authority and freeze vector ownership.

### Phase B - Contract-derived vector implementation
- Add WS11/WS12 and watershed-CLI vectors for unresolved rows.

### Phase C - Pre-migration failure baseline
- Execute expected-failure vectors against current runtime and record outcomes.

### Phase D - Pre-implementation gate and governance
- Run required gates for scoped edits and record artifact evidence.
- Complete dual review and dual verification artifacts.

### Phase E - Disposition and handoff
- Publish final WSHED03 disposition and explicit WSHED04+ handoff notes.

## Exit Criteria
- Contract-derived vectors exist for:
  - KW/MC branch lineage closure expectations,
  - impoundment RK4/regime-transition closure expectations,
  - channel sediment routing entry closure expectations,
  - watershed parquet emission non-stub closure expectations.
- Pre-migration expected-failure baseline is recorded truthfully.
- Pre-implementation gate evidence is complete and explicit.
- Required artifacts are complete with truthful evidence labels.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: test-authoring and governance package only.

## Execution Outcome Summary
- WSHED03 vector authoring is complete for the required unresolved rows:
  - WS11 KW/MC wave-routing lineage expected-failure vectors,
  - channel-sediment publication expected-failure vectors,
  - WS12 parser-projected coefficient and RK4/regime-transition expected-failure vectors,
  - watershed CLI end-to-end non-stub parquet expected-failure vector.
- Pre-migration expected-failure baseline was executed and recorded for all new
  ignored WSHED03 vectors.
- Pre-implementation gate evidence is complete for WSHED03 scope and provides
  explicit downstream entry posture for WSHED04+ runtime migration packages.
- Program-level watershed closure remains `HOLD` pending WSHED04..WSHED09.
