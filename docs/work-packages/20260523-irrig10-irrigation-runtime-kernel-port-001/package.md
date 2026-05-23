# 20260523-irrig10-irrigation-runtime-kernel-port-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Implement irrigation runtime kernels that consume parsed depletion and fixed-date
irrigation surfaces with typed scheduling and hydrology coupling in the
monolithic openWEPP scientific hydrology/erosion model.

## Why This Package Exists
PL15 retained PL08 hold and dispositioned `KERNEL-GAP-008` to `IRRIG10` in the
PL09 queue addendum. `IRRIG10` depends on `WB14` so infiltration/hyetograph
runtime coupling is available before irrigation-event integration alters
water-balance/forcing surfaces.

This package is contract-first and implementation-bound: canonical contract
amendments and contract-derived tests must be implemented and pre-implementation
gate evidence recorded before production kernel code edits.

## Scope
### Included
- Implement production irrigation runtime kernels for depletion-triggered and
  fixed-date irrigation schedules from parsed sidecar surfaces.
- Implement typed irrigation-event scheduling and payload validation in runtime
  execution.
- Couple irrigation events into runtime water-balance/forcing surfaces
  deterministically.
- Enforce typed guard behavior for invalid/non-finite/out-of-domain irrigation
  schedule/payload and coupled flux/state updates.
- Implement canonical contract amendments for IRRIG10 authority and guard
  behavior.
- Implement contract-derived IRRIG10 tests from amended contract authority and
  run pre-implementation contract-gate evidence before production kernel code
  edits.
- Produce replay evidence for irrigated fixtures and coupled hydrology effects.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- WB16 peak runoff kernel (`KERNEL-GAP-009`).
- CLIM05/CLIM06 snow/frost runtime kernel ports (`KERNEL-GAP-005..006`).
- PL16/PL17 physics kernel authoring beyond consumed runtime state surfaces.
- Tier-A hold-lift closeout disposition updates beyond IRRIG10 scope.

## Deliverables
1. IRRIG10 process-contract authority implementation evidence:
   - `artifacts/irrig10-contract-implementation-evidence.md`
2. IRRIG10 runtime authority and guard map note:
   - `artifacts/irrig10-runtime-authority-and-guard-map.md`
3. IRRIG10 contract-derived test implementation evidence:
   - `artifacts/irrig10-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/irrig10-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/irrig10-implementation-and-test-evidence.md`
6. Irrigation event scheduling trace evidence:
   - `artifacts/irrig10-irrigation-event-scheduling-trace-evidence.md`
7. Coupled hydrology effect evidence:
   - `artifacts/irrig10-coupled-hydrology-effect-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/irrig10-typed-seam-non-regression-evidence.md`
9. Kernel profile compliance checklist:
   - `artifacts/irrig10-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/irrig10_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IRRIG-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `tests/fixtures/infile/irrigation/**`
- `docs/work-packages/20260523-irrig10-irrigation-runtime-kernel-port-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15 queue/addendum scope and WB14 closure baseline.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for IRRIG10 runtime
  authority and invariants before runtime kernel changes.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement irrigation runtime scheduling and hydrology coupling behavior with
  typed guards and deterministic event application sequencing.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence and IRRIG10 closure
  posture.

## Exit Criteria
- `KERNEL-GAP-008` IRRIG10 closure is evidence-backed.
- Irrigation parsers are no longer orphan surfaces and drive runtime event
  scheduling behavior.
- Irrigation events alter runtime water-balance/forcing surfaces
  deterministically under typed guards.
- Canonical IRRIG10-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived IRRIG10 tests are implemented and executed (not just
  planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Replay evidence demonstrates irrigated fixture behavior and coupled effects.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific irrigation-runtime kernel and contract/test
  implementation.
