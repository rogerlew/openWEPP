# 20260523-arch22-typed-state-surface-closure-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Close CRF-001 carry-forward by migrating stringly
`BoundarySymbol(String)` kernel surfaces to typed state surfaces in production
runtime interfaces.

## Why This Package Exists
PL15 retained PL08 hold and dispositioned `KERNEL-GAP-012` to
`ARCH22-typed-state-surface-closure` in the PL09 queue addendum. Queue
authority defines dependencies `PL16`, `PL17`, and `WB14`; this package closes
architecture-level typed-surface migration so production kernel interfaces no
longer depend on stringly symbol keys.

This package is contract-first and implementation-bound: typed-surface contract
updates and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production migration code edits.

## Scope
### Included
- Migrate production kernel interface surfaces away from stringly
  `BoundarySymbol(String)` usage to typed state surfaces.
- Implement canonical contract-authority updates for typed boundary/state
  surfaces and migration constraints.
- Implement migration proof tests that demonstrate production kernel interfaces
  no longer rely on stringly symbol keys for covered state surfaces.
- Enforce typed guard behavior for missing/invalid/non-finite typed boundary
  states in migrated interfaces.
- Preserve runtime behavior semantics while improving type safety and boundary
  determinism.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- New process-physics kernel equations outside typed-surface migration scope.
- Watershed/erosion lane feature expansion beyond interface migration needed for
  typed-state closure.
- Tier-A hold-lift closeout disposition updates beyond ARCH22 scope.

## Deliverables
1. Typed-surface contract updates artifact:
   - `artifacts/arch22-typed-surface-contract-updates.md`
2. Migration proof tests evidence artifact:
   - `artifacts/arch22-migration-proof-tests-evidence.md`
3. ARCH22 closure artifact:
   - `artifacts/arch22-closure-artifact.md`
4. Migration map and write-scope matrix:
   - `artifacts/arch22-boundary-symbol-migration-map.md`
5. Pre-implementation contract gate evidence:
   - `artifacts/arch22-preimplementation-contract-gate.md`
6. Implementation and test evidence:
   - `artifacts/arch22-implementation-and-test-evidence.md`
7. Typed-seam non-regression evidence:
   - `artifacts/arch22-typed-seam-non-regression-evidence.md`
8. Kernel profile compliance checklist:
   - `artifacts/arch22-kernel-profile-compliance-checklist.md`
9. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/arch22_disposition.md`
10. Dual review/verification artifacts:
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
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/symbol-alias-registry.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl16-growth-physics-kernelization-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl16-growth-physics-kernelization-001/artifacts/pl16_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl17-decomposition-physics-kernelization-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl17-decomposition-physics-kernelization-001/artifacts/pl17_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`
- `/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs`
- `/workdir/openWEPP/crates/openwepp-sim-contract/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/symbol-alias-registry.md`
- `crates/openwepp-sim-contract/src/symbols.rs`
- `crates/openwepp-sim-contract/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-arch22-typed-state-surface-closure-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 queue/addendum scope and PL16/PL17/WB14 completion baselines.

### Phase 1 - Contract Implementation
- Implement required canonical typed-surface contract updates and migration
  constraints before production migration code edits.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived migration proof tests from amended authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Migration Implementation
- Implement typed-state surface migration across covered production kernel
  interfaces with typed guard behavior.

### Phase 4 - Verification
- Run targeted migration proof tests/integration tests and required repository
  gates.

### Phase 5 - Disposition
- Publish typed-surface contract updates, migration proof evidence, and ARCH22
  closure artifact.

## Exit Criteria
- `KERNEL-GAP-012` ARCH22 closure is evidence-backed.
- Covered production kernel interfaces no longer rely on stringly symbol keys.
- Typed-surface contract updates are implemented in canonical authority files.
- Migration proof tests are implemented and executed.
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production migration code edits.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: typed-interface migration and contract/test implementation.
