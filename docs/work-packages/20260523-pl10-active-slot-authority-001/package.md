# 20260523-pl10-active-slot-authority-001

## Status
- state: queued
- date: 2026-05-23
- timezone: UTC

## Objective
Replace first-slot placeholder dispatch coupling (`slot_0001/crop_0001`) with
runtime day-aware active slot/crop authority per OFE for PL growth and
decomposition transition routing.

## Why This Package Exists
PL09 identified `PL09-GAP-003` as a high-severity blocker: transition routing
is currently hard-coded to `pl_*_slot_0001_crop_0001_*` symbols. PL09A cleared
pre-execution preconditions and explicitly gated queue start on this package.
PL10 is the first implementation package in the hold-lift chain.

## Scope
### Included
- Replace hard-coded growth/decomposition dispatch symbol constants with
  day-aware active slot/crop authority resolution logic.
- Introduce typed selection/resolution errors for ambiguous, missing, or
  out-of-range active slot/crop contexts.
- Preserve deterministic phase ordering constraints and existing ordering-flag
  checks (`decomp -> growth -> watbal`).
- Add/extend tests demonstrating multi-slot and rotation-boundary routing
  behavior.
- Record typed-seam non-regression posture per ARCH15/ARCH21 closure evidence.

### Explicitly Out of Scope
- Event-payload projection expansion (`PL11`).
- Process-level growth/decomposition kinetics implementation (`PL12+`).
- Tier-A comparator closeout execution (`PL14/PL15`).

## Deliverables
1. Active slot authority contract note:
   - `artifacts/pl10-active-slot-authority-contract.md`
2. Slot-resolution algorithm and typed error model:
   - `artifacts/pl10-slot-resolution-algorithm.md`
3. Symbol-family generalization map:
   - `artifacts/pl10-symbol-family-generalization-map.md`
4. Implementation and test evidence:
   - `artifacts/pl10-implementation-and-test-evidence.md`
5. Typed-seam non-regression evidence:
   - `artifacts/pl10-typed-seam-non-regression-evidence.md`
6. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl10_disposition.md`
7. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-vs-baseline-pl-parity-gap-register.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/artifacts/precondition-2-symbol-wiring-disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/artifacts/precondition-3-typed-surface-strategy-decision.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/**`
- `tests/integration/**`
- `docs/work-packages/20260523-pl10-active-slot-authority-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL09/PL09A gating constraints and active-slot authority acceptance
  targets.

### Phase 1 - Design
- Define active slot/crop selection model and typed failure classes.

### Phase 2 - Implementation
- Replace hard-coded slot/crop symbols in growth/decomposition dispatch
  precondition selection path.

### Phase 3 - Verification
- Add/extend multi-slot/rotation-boundary tests and run required gates.

### Phase 4 - Disposition
- Publish implementation evidence and finalize review/verification artifacts.

## Exit Criteria
- Growth/decomposition dispatch no longer depends on
  `slot_0001/crop_0001` placeholder symbols.
- Active slot/crop selection is deterministic and typed-failure guarded.
- Existing ordering-flag invariants remain enforced.
- Multi-slot + rotation-boundary tests demonstrate correct active-branch
  routing behavior.
- Typed-seam non-regression evidence references ARCH15/ARCH21 closure posture.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scheduler/dispatch authority implementation and test changes.
