# 20260523-wb16-peak-runoff-kernel-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Implement production peak runoff calculation required for downstream
sediment/routing coupling in the monolithic openWEPP scientific hydrology/
erosion model.

## Why This Package Exists
PL15 retained PL08 hold and dispositioned `KERNEL-GAP-009` to `WB16` in the
PL09 queue addendum. `WB16` depends on `WB14` and `WB15` so infiltration,
hyetograph integration, and canopy-interception coupling are available before
peak-flow computation.

This package is contract-first and implementation-bound: canonical contract
amendments and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production kernel code edits.

## Scope
### Included
- Implement production peak runoff kernel behavior with documented method
  branches per canonical authority.
- Couple peak runoff outputs into downstream routing/sediment boundary surfaces
  where required by contracts.
- Enforce typed guard behavior for invalid/non-finite/out-of-domain peak-flow
  inputs, intermediates, and outputs.
- Implement canonical contract amendments for WB16 peak-flow authority and
  guard behavior.
- Implement contract-derived WB16 tests from amended contract authority and run
  pre-implementation contract-gate evidence before production kernel code edits.
- Produce peak-flow trace outputs and downstream coupling readiness evidence.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- EROD10 sediment-kernel intake execution (`KERNEL-GAP-010`).
- WS10 watershed production-kernel execution (`KERNEL-GAP-011`).
- CLIM05/CLIM06 snow/frost runtime kernel authoring (`KERNEL-GAP-005..006`).
- Tier-A hold-lift closeout disposition updates beyond WB16 scope.

## Deliverables
1. WB16 process-contract authority implementation evidence:
   - `artifacts/wb16-contract-implementation-evidence.md`
2. WB16 peak-flow authority and guard map note:
   - `artifacts/wb16-peak-flow-kernel-authority-and-guard-map.md`
3. WB16 contract-derived test implementation evidence:
   - `artifacts/wb16-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/wb16-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/wb16-implementation-and-test-evidence.md`
6. Peak-flow trace evidence:
   - `artifacts/wb16-peak-flow-trace-evidence.md`
7. Downstream coupling readiness evidence:
   - `artifacts/wb16-downstream-coupling-readiness-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/wb16-typed-seam-non-regression-evidence.md`
9. Kernel profile compliance checklist:
   - `artifacts/wb16-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wb16_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb15-canopy-interception-kernel-coupling-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb15-canopy-interception-kernel-coupling-001/artifacts/wb15_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-wb16-peak-runoff-kernel-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 queue/addendum scope and WB14 + WB15 closure baselines.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for WB16 peak-flow
  authority and invariants before runtime kernel changes.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement peak runoff kernel behavior with typed guards and deterministic
  branch sequencing.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence and WB16 closure
  posture.

## Exit Criteria
- `KERNEL-GAP-009` WB16 closure is evidence-backed.
- Peak runoff outputs are produced in canonical runtime path with documented
  method branches and typed guards.
- Peak runoff outputs are surfaced for downstream sediment/routing coupling
  readiness.
- Canonical WB16-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived WB16 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Trace outputs and coupling-readiness evidence are produced.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific peak-runoff kernel and contract/test implementation.
