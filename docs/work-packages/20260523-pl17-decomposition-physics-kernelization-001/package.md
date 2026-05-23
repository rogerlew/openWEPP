# 20260523-pl17-decomposition-physics-kernelization-001

## Status
- state: complete
- date: 2026-05-23
- timezone: UTC

## Objective
Replace PL12 decomposition plumbing-only behavior with production
residue/decomposition kinetics and transition-pool transfer physics in the
monolithic openWEPP scientific hydrology/erosion model.

## Why This Package Exists
PL15 retained PL08 hold and dispositioned `KERNEL-GAP-003` to `PL17` in the
PL09 queue addendum. This package closes the decomposition-physics lane by
converting transition signaling/scaffolding into equation-driven residue/
decomposition kinetics with typed guard enforcement.

This package is contract-first and implementation-bound: canonical contract
amendments and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production kernel code edits.

## Scope
### Included
- Implement equation-driven residue/decomposition kinetics for covered
  decomposition branches and transition events.
- Implement transition-pool transfer updates under typed domain/invariant
  guards.
- Remove default skip/placeholder transition behavior for covered
  decomposition physics paths.
- Implement canonical contract amendments for PL17 decomposition-physics
  authority and guard behavior.
- Implement contract-derived PL17 tests from amended contract authority and run
  pre-implementation contract-gate evidence before production kernel code edits.
- Produce residue trajectory and kinetic-validation evidence for representative
  annual/perennial cases.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- PL16 growth-physics kernelization (`KERNEL-GAP-002`).
- WB15 canopy interception coupling (`KERNEL-GAP-007`).
- ARCH22 typed-state-surface closure (`KERNEL-GAP-012`).
- Tier-A hold-lift closeout disposition updates beyond PL17 scope.

## Deliverables
1. PL17 process-contract authority implementation evidence:
   - `artifacts/pl17-contract-implementation-evidence.md`
2. PL17 decomposition-kinetics authority and guard map note:
   - `artifacts/pl17-decomposition-kinetics-authority-and-guard-map.md`
3. PL17 contract-derived test implementation evidence:
   - `artifacts/pl17-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/pl17-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/pl17-implementation-and-test-evidence.md`
6. Residue trajectory evidence:
   - `artifacts/pl17-residue-trajectory-evidence.md`
7. Kinetic-validation evidence:
   - `artifacts/pl17-kinetic-validation-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/pl17-typed-seam-non-regression-evidence.md`
9. Kernel profile compliance checklist:
   - `artifacts/pl17-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl17_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl12-decomp-resup-transition-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl12-decomp-resup-transition-kernel-001/artifacts/pl12_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-pl17-decomposition-physics-kernelization-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 queue/addendum scope and PL12 implementation baseline.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for PL17 decomposition-
  physics authority and invariants before runtime kernel changes.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement equation-driven decomposition physics behavior with typed guards and
  deterministic transition/update sequencing.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence, residue
  trajectories, kinetic validation, and PL17 closure posture.

## Exit Criteria
- `KERNEL-GAP-003` PL17 closure is evidence-backed.
- Decomposition outputs are equation-driven in covered decomposition branches.
- Transition payloads drive real residue/pool updates under typed guards.
- No default skip/placeholder transition behavior remains for covered
  decomposition physics paths.
- Canonical PL17-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived PL17 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Residue trajectory and kinetic-validation evidence are produced for
  representative annual/perennial scenarios.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific decomposition-physics kernel and contract/test
  implementation.
