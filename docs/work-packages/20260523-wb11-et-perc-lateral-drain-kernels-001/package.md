# 20260523-wb11-et-perc-lateral-drain-kernels-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Implement ET, percolation/deep seepage, lateral transfer, and drainage
production kernels with typed invariant checks in the monolithic openWEPP
scientific hydrology/erosion model.

## Why This Package Exists
The PL09 hold-lift queue defines `WB11` as the next water-balance lane package
after WB10 phase-kernel skeleton routing. WB10 established production hydrology
phase entry scaffolding; WB11 fills algorithm execution for ET/perc/lateral/
drain lanes and provides deterministic state/flux updates for downstream WB12+
closeout work.

This package is contract-first and implementation-bound: canonical kernel
contracts and contract-derived tests must be implemented (not only planned or
documented), and executed evidence is required before WB11 disposition.

## Scope
### Included
- Implement ET, percolation/deep seepage, lateral transfer, and drainage
  production kernel execution in WB10-routed phase classes.
- Enforce typed invariants and finite/bounds checks for updated state/flux
  surfaces.
- Implement canonical contract amendments required to represent WB11 algorithm
  authority and guard behavior in science-contract files.
- Implement contract-derived tests from the amended contracts, and run a
  pre-implementation contract gate before production kernel code edits.
- Add targeted kernel/integration evidence that phase execution updates required
  symbols with explicit typed failure propagation.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- Runoff/storage reconciliation kernels (`WB12`).
- Daily comparator-ready output surface emission (`WB13`).
- Tier-A comparator closeout execution (`PL14`/`PL15`).

## Deliverables
1. WB11 process-contract authority implementation evidence:
   - `artifacts/wb11-contract-implementation-evidence.md`
2. WB11 algorithm and guard map note:
   - `artifacts/wb11-kernel-algorithm-guard-map.md`
3. WB11 contract-derived test implementation evidence:
   - `artifacts/wb11-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/wb11-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/wb11-implementation-and-test-evidence.md`
6. Typed-seam non-regression evidence:
   - `artifacts/wb11-typed-seam-non-regression-evidence.md`
7. Kernel profile compliance checklist:
   - `artifacts/wb11-kernel-profile-compliance-checklist.md`
8. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wb11_disposition.md`
9. Dual review/verification artifacts:
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb10-hydrology-phase-kernel-skeleton-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb10-hydrology-phase-kernel-skeleton-001/artifacts/wb10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13-growth-transition-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13-growth-transition-kernel-001/artifacts/pl13_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-wb11-et-perc-lateral-drain-kernels-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm WB10 and PL13 completion state and WB11 queue scope.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for WB11 authority.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation gate results before kernel code edits.

### Phase 3 - Kernel Implementation
- Implement ET/perc/lateral/drainage production kernel execution with typed
  error propagation and invariant checks.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence and closure posture.

## Exit Criteria
- WB11 production kernels for ET/perc/lateral/drainage are implemented and
  routed through WB10 skeleton path.
- Canonical WB11-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived WB11 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before kernel code edits.
- Deterministic phase execution updates required state/flux symbols with typed
  finite/bounds invariant checks.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific hydrology kernel and contract/test implementation.
