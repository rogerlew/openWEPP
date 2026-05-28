# 20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL40 to identify and close implementation gaps between openWEPP
watershed Muskingum-Cunge routing (`ipeak >= 4`) and the pinned
`wepp-forest_260430_baseline` authority, bringing the openWEPP implementation
to baseline-authoritative parity with contract-first sequencing and dual
review/verification gate closure.

## Why This Package Exists
Current watershed routing contracts declare Muskingum-Cunge authority and
branch coverage, but parity posture must be revalidated against pinned baseline
source implementation detail and any residual openWEPP drift must be closed
with explicit evidence. This package creates a focused parity burndown for
Muskingum-Cunge branch behavior and associated runtime/publication seams.

## Scope
### Included
- Baseline-to-openWEPP parity assessment for watershed Muskingum-Cunge branch
  behavior (`ipeak >= 4`) with explicit gap matrix evidence anchored to:
  - `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
  - companion call/selector lineage in
    `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
    and `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`.
- Canonical contract/index amendments required by discovered parity gaps:
  - `SC-ROUTE-001` (primary),
  - `SC-SYSTEM-001` (cross-contract integration posture),
  - `SC-HYDRAULICS-001` only if authority or seam ownership requires update,
  - `docs/specifications/science-contracts/index.md`.
- Contract-derived tests for Muskingum-Cunge parity vectors in watershed
  integration surfaces.
- Production parity migration for Muskingum-Cunge branch behavior in watershed
  runtime/orchestration paths.
- Required gate execution and dual review/dual verification evidence.

### Explicitly Out of Scope
- New process-physics feature additions outside baseline parity closure scope.
- Heuristic/proxy substitutions for unresolved process physics.
- Non-routing refactors unrelated to Muskingum-Cunge parity closure.

## Deliverables
1. `artifacts/wshedimpl40-muskingum-cunge-gap-matrix.md`
2. `artifacts/wshedimpl40-contract-implementation-evidence.md`
3. `artifacts/wshedimpl40-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl40-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl40-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl40-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl40_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL40 scope.
2. Implement contract-derived tests for discovered Muskingum-Cunge gaps.
3. Record pre-implementation contract-gate evidence.
4. Implement production Muskingum-Cunge parity migration edits.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl05-watershed-wave-routing-state-family-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/artifacts/wshedimpl39_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` (if required)
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL40 authorization from active queue/user directive and
  WSHEDIMPL39 handoff context.
- Freeze Muskingum-Cunge scope boundaries (`ipeak >= 4`) and authority inputs.

### Phase B - Gap assessment and authority mapping
- Build explicit baseline-to-openWEPP Muskingum-Cunge gap matrix from pinned
  baseline source and current runtime implementation.
- Classify each discovered gap as contract-only, test-only, runtime, or mixed.

### Phase C - Contract updates (first required gate)
- Amend canonical `SC-*` contracts and index language for all in-scope
  Muskingum-Cunge parity gaps and authority clarifications.

### Phase D - Contract-derived tests (second required gate)
- Implement/activate Muskingum-Cunge parity vectors for each discovered gap and
  expected branch behavior.

### Phase E - Pre-implementation contract gate (third required gate)
- Record explicit pre-implementation gate evidence confirming contracts and
  tests are in place before production edits.

### Phase F - Production parity migration (fourth required gate)
- Implement runtime/orchestration Muskingum-Cunge parity edits with typed
  fail-closed guards and no silent defaults/clamping.

### Phase G - Validation evidence run
- Execute required validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Phase H - Dual review, dual verification, disposition
- Complete `review_agent_a.md` and `review_agent_b.md`.
- Complete `verification_agent_a.md` and `verification_agent_b.md`.
- Publish GO/HOLD disposition with explicit closure map for discovered gaps.

## Exit Criteria
- Baseline-authoritative Muskingum-Cunge parity matrix is complete and
  dispositioned for all in-scope implementation surfaces.
- All in-scope parity gaps are either closed in runtime or explicitly retained
  with evidence-backed HOLD ownership.
- Contract-first sequencing evidence is complete and truthful.
- Dual review and dual verification artifacts are complete.
- Required validation gates pass and are recorded truthfully.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contracts/runtime/tests and package artifacts only; no
  network or credential surface changes.
