# 20260523-clim05-snow-runtime-kernel-port-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Implement runtime snow accumulation/melt kernel coupling from parsed snow
controls into hydrology boundary surfaces in the monolithic openWEPP
scientific hydrology/erosion model.

## Why This Package Exists
PL15 retained PL08 hold and dispositioned `KERNEL-GAP-005` to `CLIM05` in the
PL09 queue addendum. WB14 closed infiltration and within-day hyetograph
coupling preconditions; CLIM05 is the next required closure so snow forcing is
no longer an orphan parser surface and affects water-balance terms under typed
invariants.

This package is contract-first and implementation-bound: canonical contract
amendments and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production kernel code edits.

## Scope
### Included
- Implement runtime snow accumulation/melt kernel behavior driven by parsed
  snow control surfaces.
- Couple snow-state outputs into hydrology water-balance and runoff/infiltration
  boundary terms where contract authority requires.
- Enforce typed guard behavior for invalid/non-finite/out-of-domain snow state,
  melt rates, and coupled flux/state updates.
- Implement canonical contract amendments for CLIM05 snow-runtime authority and
  guard behavior.
- Implement contract-derived CLIM05 tests from amended contract authority and
  run pre-implementation contract-gate evidence before production kernel code
  edits.
- Produce fixture-backed replay evidence for snow scenarios and coupled
  water-balance effects.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- CLIM06 frost/frozen-soil runtime kernel port (`KERNEL-GAP-006`).
- PL16/PL17 plant-physics kernelization (`KERNEL-GAP-002..003`).
- WB15 canopy interception and WB16 peak-runoff follow-ons.
- Tier-A hold-lift closeout disposition updates beyond CLIM05 scope.

## Deliverables
1. CLIM05 process-contract authority implementation evidence:
   - `artifacts/clim05-contract-implementation-evidence.md`
2. CLIM05 snow-runtime authority and guard map note:
   - `artifacts/clim05-snow-runtime-authority-and-guard-map.md`
3. CLIM05 contract-derived test implementation evidence:
   - `artifacts/clim05-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/clim05-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/clim05-implementation-and-test-evidence.md`
6. Snow-scenario fixture replay evidence:
   - `artifacts/clim05-snow-scenario-fixture-replay-evidence.md`
7. Coupled water-balance effect evidence:
   - `artifacts/clim05-coupled-water-balance-effect-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/clim05-typed-seam-non-regression-evidence.md`
9. Kernel profile compliance checklist:
   - `artifacts/clim05-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim05_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/artifacts/clim04_disposition.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/snow.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `tests/fixtures/infile/snow/**`
- `docs/work-packages/20260523-clim05-snow-runtime-kernel-port-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 queue/addendum scope, WB14 closure baseline, and snow-control
  parser/runtime seam readiness.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for CLIM05 snow-runtime
  authority and invariants before runtime kernel changes.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement snow accumulation/melt runtime coupling behavior with typed guards
  and deterministic coupled update sequencing.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence, snow scenario
  replay evidence, and CLIM05 closure posture.

## Exit Criteria
- `KERNEL-GAP-005` CLIM05 closure is evidence-backed.
- Snow forcing is no longer an orphan parser output and drives runtime
  water-balance coupled behavior.
- Snow accumulation/melt effects on coupled hydrology terms are implemented
  under typed invariants.
- Canonical CLIM05-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived CLIM05 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Snow-scenario fixture replay evidence demonstrates coupled effects.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific snow-runtime kernel and contract/test implementation.
