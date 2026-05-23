# 20260523-pl16-growth-physics-kernelization-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Replace PL13 growth plumbing-only behavior with production growth physics
kernels (GDD, biomass, canopy, phenology, and senescence/harvest dynamics) in
the monolithic openWEPP scientific hydrology/erosion model.

## Why This Package Exists
PL15 retained PL08 hold and dispositioned `KERNEL-GAP-002` to `PL16` in the
PL09 queue addendum. This package closes the growth-physics lane by converting
transition signaling/scaffolding into equation-driven production behavior with
typed guard enforcement.

This package is contract-first and implementation-bound: canonical contract
amendments and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production kernel code edits.

## Scope
### Included
- Implement equation-driven growth updates for active growth branches,
  including GDD progression, biomass/canopy development, phenology progression,
  and senescence/harvest dynamics.
- Remove default skip/zero-reset fallback behavior for covered active growth
  branches.
- Implement typed guard behavior for invalid/non-finite/out-of-domain growth
  state transitions.
- Implement canonical contract amendments for PL16 growth-physics authority and
  guard behavior.
- Implement contract-derived PL16 tests from amended contract authority and run
  pre-implementation contract-gate evidence before production kernel code edits.
- Produce growth state-trajectory evidence and regression parity traces for
  representative annual/perennial cases.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- PL17 decomposition-physics kernelization (`KERNEL-GAP-003`).
- WB15 canopy interception coupling (`KERNEL-GAP-007`).
- ARCH22 typed-state-surface closure (`KERNEL-GAP-012`).
- Tier-A hold-lift closeout disposition updates beyond PL16 scope.

## Deliverables
1. PL16 process-contract authority implementation evidence:
   - `artifacts/pl16-contract-implementation-evidence.md`
2. PL16 growth-equation authority and guard map note:
   - `artifacts/pl16-growth-equations-authority-and-guard-map.md`
3. PL16 contract-derived test implementation evidence:
   - `artifacts/pl16-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/pl16-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/pl16-implementation-and-test-evidence.md`
6. Growth state-trajectory evidence:
   - `artifacts/pl16-growth-state-trajectory-evidence.md`
7. Regression parity trace evidence:
   - `artifacts/pl16-regression-parity-trace-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/pl16-typed-seam-non-regression-evidence.md`
9. Kernel profile compliance checklist:
   - `artifacts/pl16-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl16_disposition.md`
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
- `/workdir/openWEPP/docs/work-packages/20260523-pl13-growth-transition-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13-growth-transition-kernel-001/artifacts/pl13_disposition.md`
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
- `docs/work-packages/20260523-pl16-growth-physics-kernelization-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 queue/addendum scope and PL13 implementation baseline.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for PL16 growth-physics
  authority and invariants before runtime kernel changes.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement equation-driven growth physics behavior with typed guards and
  deterministic transition/update sequencing.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence, growth trajectories,
  parity traces, and PL16 closure posture.

## Exit Criteria
- `KERNEL-GAP-002` PL16 closure is evidence-backed.
- Growth transition updates are equation-driven in covered active growth
  branches.
- No default skip/zero-reset fallback remains for covered active growth
  branches.
- Canonical PL16-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived PL16 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Growth state-trajectory evidence and regression parity traces are produced for
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
- Rationale: scientific growth-physics kernel and contract/test implementation.
