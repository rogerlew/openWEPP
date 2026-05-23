# 20260523-int10-plant-water-coupling-validation-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Validate coupled daily execution ordering and state coupling
(`decomp -> growth -> watbal`) across plant and hydrology phases in the
monolithic openWEPP scientific hydrology/erosion model.

## Why This Package Exists
The PL09 hold-lift queue defines `INT10` as the cross-lane integration gate
after `PL13` and `WB13`. `PL13` closes production growth transition execution
and `WB13` closes comparator-ready daily water-balance output. `INT10` proves
that coupled daily ordering and state-transfer semantics hold under replay.

This package is contract-first and implementation-bound: canonical coupling
contracts and contract-derived tests must be implemented (not only planned or
documented), and executed evidence is required before INT10 disposition.

## Scope
### Included
- Validate coupled daily execution ordering for
  `decomp -> growth -> watbal` phase progression.
- Validate coupled state-transfer semantics from plant/decomposition phases into
  hydrology/water-balance phases under fixture replay.
- Enforce typed failure behavior for ordering violations, missing coupling
  symbols, and invalid coupled state domains.
- Implement canonical contract amendments required to represent INT10 coupling
  ordering/state-transfer authority and guard behavior in science-contract files.
- Implement contract-derived INT10 tests from amended contract authority and run
  pre-implementation contract gate evidence before production integration code
  edits.
- Produce coupled replay evidence with ordering flags, coupled state traces,
  and explicit disposition of any residual coupling deltas.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- Tier-A comparator closeout and risk disposition (`PL14`/`PL15`).
- New standalone plant-kernel implementation work (`PL10..PL13`).
- New standalone hydrology-kernel implementation work (`WB10..WB13`).

## Deliverables
1. INT10 process-contract authority implementation evidence:
   - `artifacts/int10-contract-implementation-evidence.md`
2. INT10 coupled ordering/state-transfer authority map:
   - `artifacts/int10-coupling-ordering-and-state-transfer-map.md`
3. INT10 contract-derived test implementation evidence:
   - `artifacts/int10-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/int10-preimplementation-contract-gate.md`
5. Coupled replay implementation and test evidence:
   - `artifacts/int10-implementation-and-test-evidence.md`
6. Coupled replay ordering/state trace evidence:
   - `artifacts/int10-coupled-replay-ordering-and-state-trace-evidence.md`
7. Typed-seam non-regression evidence:
   - `artifacts/int10-typed-seam-non-regression-evidence.md`
8. Kernel profile compliance checklist:
   - `artifacts/int10-kernel-profile-compliance-checklist.md`
9. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/int10_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13-growth-transition-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13-growth-transition-kernel-001/artifacts/pl13_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/artifacts/wb13_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-summary-accumulator/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/tests/integration/hillslope_consumer_boundary_integration.rs`
- `/workdir/openWEPP/tests/integration/wb13_daily_water_balance_output_surface_contract.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-summary-accumulator/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-int10-plant-water-coupling-validation-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm `PL13` and `WB13` completion state and `INT10` queue scope.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for INT10 coupling
  ordering/state-transfer authority.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate results before
  production integration code edits.

### Phase 3 - Coupled Integration Implementation
- Implement and/or refine coupled execution ordering/state-transfer integration
  surfaces with typed failure propagation.

### Phase 4 - Verification
- Run targeted coupled replay/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence and coupled
  ordering/state-transfer validation posture.

## Exit Criteria
- Coupled daily execution ordering (`decomp -> growth -> watbal`) is validated
  with explicit evidence.
- Coupled state-transfer semantics across plant and hydrology lanes are
  validated with explicit evidence.
- Canonical INT10-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived INT10 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production integration code edits.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific plant-water coupling integration and contract/test
  implementation.
