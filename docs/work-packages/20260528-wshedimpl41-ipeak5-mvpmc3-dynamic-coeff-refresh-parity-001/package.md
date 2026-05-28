# 20260528-wshedimpl41-ipeak5-mvpmc3-dynamic-coeff-refresh-parity-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: GO

## Objective
Execute WSHEDIMPL41 as the immediate follow-on closure package for WSHEDIMPL40
by migrating baseline-authoritative `ipeak = 5` variable-parameter
Muskingum-Cunge (MVPMC3) dynamic-coefficient refresh behavior into openWEPP
WS11 watershed routing lanes, with contract-first sequencing and dual
review/verification evidence through disposition.

## Why This Package Exists
WSHEDIMPL40 closed prior-wave-state and coefficient-publication seams for
`ipeak >= 4` routing but retained explicit HOLD posture for unresolved
`ipeak = 5` dynamic-coefficient parity (`GAP-ROUTE-011`,
`GAP-SYSTEM-010`). This package closes that remaining baseline-authoritative
gap.

## Scope
### Included
- Baseline-to-openWEPP parity closure for `ipeak = 5` MVPMC3 dynamic
  coefficient refresh lineage from pinned baseline:
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for` (`MVPMC3` block),
  - selector/call lineage in
    `/workdir/wepp-forest_260430_baseline/src/wshpek.for` and
    `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`.
- Canonical contract/index amendments required for closure:
  - `SC-ROUTE-001`,
  - `SC-SYSTEM-001`,
  - `docs/specifications/science-contracts/index.md`.
- Contract-derived WS11 vectors proving `ipeak = 5` dynamic refresh behavior.
- Production WS10 routing runtime migration for `ipeak = 5` dynamic refresh.
- Required validation gates and dual review/dual verification artifacts.

### Explicitly Out of Scope
- New routing physics/features outside MVPMC3 parity closure.
- Non-routing refactors unrelated to WSHEDIMPL41 objective.
- Heuristic/proxy substitutions for unresolved baseline-authoritative
  process-physics behavior.

## Deliverables
1. `artifacts/wshedimpl41-mvpmc3-gap-matrix.md`
2. `artifacts/wshedimpl41-contract-implementation-evidence.md`
3. `artifacts/wshedimpl41-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl41-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl41-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl41-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl41_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL41 scope.
2. Implement contract-derived tests for `ipeak = 5` dynamic refresh parity.
3. Record pre-implementation contract-gate evidence.
4. Implement production MVPMC3 runtime parity migration edits.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy parity source authority defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed in production
  parity closure claims.
- Variable naming continuity must be preserved with explicit alias mapping when
  runtime names differ from baseline/canonical symbols.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/artifacts/wshedimpl40_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl41-ipeak5-mvpmc3-dynamic-coeff-refresh-parity-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL41 authorization from user directive and WSHEDIMPL40
  worker-handoff/disposition context.
- Freeze scope to `ipeak = 5` MVPMC3 dynamic-coefficient parity.

### Phase B - Gap assessment and authority mapping
- Build explicit baseline-to-openWEPP MVPMC3 gap matrix for closure.
- Classify residual seams as contract/test/runtime write obligations.

### Phase C - Contract updates (first required gate)
- Amend canonical `SC-*` contracts and index language for MVPMC3 parity scope.

### Phase D - Contract-derived tests (second required gate)
- Implement `ipeak = 5` parity vectors that exercise dynamic-coefficient
  refresh behavior and publication continuity.

### Phase E - Pre-implementation contract gate (third required gate)
- Record explicit pre-implementation gate evidence confirming contract/test
  updates before runtime edits.

### Phase F - Production parity migration (fourth required gate)
- Implement WS10 runtime parity edits for MVPMC3 dynamic-coefficient refresh
  behavior with typed fail-closed guards and no silent defaults.

### Phase G - Validation evidence run
- Execute required validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Phase H - Dual review, dual verification, disposition
- Complete `review_agent_a.md` and `review_agent_b.md`.
- Complete `verification_agent_a.md` and `verification_agent_b.md`.
- Publish GO/HOLD disposition with explicit closure map for WSHEDIMPL41 scope.

## Exit Criteria
- `ipeak = 5` MVPMC3 dynamic-coefficient parity matrix is completed and
  dispositioned for all in-scope implementation surfaces.
- `GAP-ROUTE-011` and `GAP-SYSTEM-010` are closed or explicitly retained with
  evidence-backed ownership.
- Contract-first sequencing evidence is complete and truthful.
- Dual review and dual verification artifacts are complete.
- Required validation gates pass and are recorded truthfully.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contracts/runtime/tests and package artifacts only; no
  network or credential surface changes.
