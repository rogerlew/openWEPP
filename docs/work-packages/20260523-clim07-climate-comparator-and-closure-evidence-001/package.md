# 20260523-clim07-climate-comparator-and-closure-evidence-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Add targeted integration tests and comparator vectors for continuous-daily and
breakpoint climate modes, including parser-to-kernel seam checks and legacy
confidence-tier reporting evidence.

## Why This Package Exists
The CLIM01 climate implementation queue defines `CLIM07` as climate comparator
and closure evidence after CLIM03..CLIM06. This package closes evidence gaps by
building deterministic comparator vectors and integration checks that prove
runtime climate seam behavior for accepted policy branches.

This package is contract-first and implementation-bound: canonical contract
updates and contract-derived comparator/test vectors must be implemented and
pre-implementation contract-gate evidence recorded before production
comparator/integration code edits.

## Scope
### Included
- Implement targeted integration tests for continuous-daily (`ibrkpt=0`) and
  breakpoint (`ibrkpt=1`) climate modes.
- Implement comparator vectors and expected-result evidence for accepted
  climate runtime policy branches.
- Implement parser-to-kernel seam checks for climate runtime payload surfaces.
- Implement confidence-tier reporting evidence for legacy comparator posture.
- Implement canonical contract updates required for CLIM07 comparator/seam
  authority and test-vector intent.
- Implement contract-derived CLIM07 tests/vectors from amended authority and
  run pre-implementation contract-gate evidence before comparator/integration
  code edits.
- Preserve ARCH15/ARCH17/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- New climate runtime algorithm branches beyond CLIM03/CLIM04 accepted policy.
- New snow/frost or irrigation kernel authoring beyond CLIM05/CLIM06 evidence
  consumption.
- Tier-A hold-lift disposition changes beyond CLIM07 evidence scope.

## Deliverables
1. CLIM07 process-contract authority implementation evidence:
   - `artifacts/clim07-contract-implementation-evidence.md`
2. CLIM07 comparator-vector manifest:
   - `artifacts/clim07-comparator-vector-manifest.md`
3. CLIM07 contract-derived test/vector implementation evidence:
   - `artifacts/clim07-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/clim07-preimplementation-contract-gate.md`
5. Continuous-daily comparator evidence:
   - `artifacts/clim07-continuous-daily-comparator-evidence.md`
6. Breakpoint comparator evidence:
   - `artifacts/clim07-breakpoint-comparator-evidence.md`
7. Parser-to-kernel seam-check evidence:
   - `artifacts/clim07-parser-to-kernel-seam-check-evidence.md`
8. Confidence-tier reporting evidence:
   - `artifacts/clim07-confidence-tier-reporting-evidence.md`
9. Implementation and test evidence:
   - `artifacts/clim07-implementation-and-test-evidence.md`
10. Typed-seam non-regression evidence:
   - `artifacts/clim07-typed-seam-non-regression-evidence.md`
11. Kernel profile compliance checklist:
   - `artifacts/clim07-kernel-profile-compliance-checklist.md`
12. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim07_disposition.md`
13. Dual review/verification artifacts:
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IRRIG-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/climate-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim03-continuous-daily-climate-runtime-kernel-port-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim03-continuous-daily-climate-runtime-kernel-port-001/artifacts/clim03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/artifacts/clim04_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim05-snow-runtime-kernel-port-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim05-snow-runtime-kernel-port-001/artifacts/clim05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim06-frost-frozen-soil-kernel-port-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim06-frost-frozen-soil-kernel-port-001/artifacts/clim06_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/package.md`
- `/workdir/openWEPP/crates/openwepp-comparator-metadata/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/tests/integration/comparator_tier_routing_metadata.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-IRRIG-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-comparator-metadata/src/lib.rs`
- `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/**`
- `tests/fixtures/infile/climate/**`
- `docs/work-packages/20260523-clim07-climate-comparator-and-closure-evidence-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm CLIM01 queue scope and CLIM03..CLIM06 completion baselines.

### Phase 1 - Contract Implementation
- Implement required canonical contract updates for CLIM07 comparator/seam
  authority before comparator/integration code edits.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived comparator vectors and seam tests from amended
  authority.
- Execute and record pre-implementation contract-gate evidence before
  production comparator/integration code edits.

### Phase 3 - Comparator and Integration Implementation
- Implement comparator vectors, parser-to-kernel seam checks, and confidence-
  tier reporting evidence outputs.

### Phase 4 - Verification
- Run targeted climate/comparator integration tests and required repository
  gates.

### Phase 5 - Disposition
- Publish comparator closure evidence and CLIM07 disposition posture.

## Exit Criteria
- CLIM07 queue objective (comparator + closure evidence) is evidence-backed.
- Targeted continuous-daily and breakpoint comparator vectors are implemented
  and executed.
- Parser-to-kernel seam checks are implemented and executed for climate runtime
  payload surfaces.
- Confidence-tier reporting evidence is produced from executed comparator paths.
- Canonical CLIM07-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived CLIM07 tests/vectors are implemented and executed.
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production comparator/integration code edits.
- Existing typed-seam closure posture from ARCH15/ARCH17/ARCH21 remains
  non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: climate comparator/integration evidence and contract/test
  implementation.
