# 20260523-ws10-channel-impoundment-production-kernels-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Replace watershed test/probe kernel posture with production channel and
impoundment kernels under typed boundary integration in the monolithic
openWEPP scientific hydrology/erosion model.

## Why This Package Exists
PL15 retained PL08 hold and dispositioned `KERNEL-GAP-011` to
`WS10-channel-impoundment-production-kernels` in the PL09 queue addendum.
`WS10` depends on `WB16` so peak-runoff outputs and upstream hydrology coupling
surfaces are available before watershed production kernel execution.

This package is contract-first and implementation-bound: canonical contract
amendments and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production kernel code edits.

## Scope
### Included
- Implement at least one production `impl WatershedKernel` execution path for
  channel and impoundment processing under typed guards.
- Replace test/probe watershed kernel posture for covered channel/impoundment
  runtime paths.
- Enforce typed guard behavior for invalid/non-finite/out-of-domain watershed
  boundary state/flux and routing/impoundment branch inputs.
- Implement canonical contract amendments for WS10 routing/impoundment
  authority and guard behavior.
- Implement contract-derived WS10 tests from amended contract authority and run
  pre-implementation contract-gate evidence before production kernel code edits.
- Produce routing/impoundment contract evidence and watershed kernel test
  evidence for covered execution branches.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- EROD10 sediment-kernel intake execution (`KERNEL-GAP-010`) beyond consumed
  boundary interfaces.
- ARCH22 typed-state migration execution (`KERNEL-GAP-012`).
- Tier-A hold-lift closeout disposition updates beyond WS10 scope.

## Deliverables
1. WS10 process-contract authority implementation evidence:
   - `artifacts/ws10-contract-implementation-evidence.md`
2. WS10 watershed kernel authority and guard map note:
   - `artifacts/ws10-watershed-kernel-authority-and-guard-map.md`
3. WS10 contract-derived test implementation evidence:
   - `artifacts/ws10-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/ws10-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/ws10-implementation-and-test-evidence.md`
6. Routing/impoundment contract evidence:
   - `artifacts/ws10-routing-impoundment-contract-evidence.md`
7. Production watershed-kernel path evidence:
   - `artifacts/ws10-production-watershed-kernel-path-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/ws10-typed-seam-non-regression-evidence.md`
9. Kernel profile compliance checklist:
   - `artifacts/ws10-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/ws10_disposition.md`
11. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/artifacts/wb16_disposition.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/watershed_channel.rs`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-ws10-channel-impoundment-production-kernels-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 queue/addendum scope and WB16 closure baseline.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for WS10 routing/
  impoundment authority and invariants before runtime kernel changes.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement production watershed channel/impoundment kernel behavior with typed
  guards and deterministic branch sequencing.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence and WS10 closure
  posture.

## Exit Criteria
- `KERNEL-GAP-011` WS10 closure is evidence-backed.
- At least one production `impl WatershedKernel` path exists for
  channel/impoundment execution under typed guards.
- Routing/impoundment contract evidence and watershed kernel tests are
  produced.
- Canonical WS10-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived WS10 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: watershed channel/impoundment kernel and contract/test
  implementation.
