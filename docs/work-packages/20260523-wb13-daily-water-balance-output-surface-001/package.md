# 20260523-wb13-daily-water-balance-output-surface-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Emit comparator-ready daily water-balance output surface (`H5.wat.dat`
equivalent contract surface) from the openWEPP run path in the monolithic
openWEPP scientific hydrology/erosion model.

## Why This Package Exists
The PL09 hold-lift queue defines `WB13` as the follow-on to `WB12`, with
explicit dependency on `PL13` growth-transition closure. WB12 completes
runoff/storage reconciliation kernels; WB13 closes the missing Tier-A daily
output surface required for comparator-ready replay evidence.

This package is contract-first and implementation-bound: canonical WB13 output
contracts and contract-derived tests must be implemented (not only planned or
documented), and executed evidence is required before WB13 disposition.

## Scope
### Included
- Implement deterministic daily water-balance output emission for comparator
  Tier-A single-OFE workflows (`H5.wat.dat` equivalent surface).
- Implement and document canonical schema, units, field ordering, row keys,
  and formatting/serialization rules required for replay comparability.
- Implement typed guard behavior for missing/non-finite/out-of-domain required
  output-surface symbols before row emission.
- Implement canonical contract amendments for WB13 output-surface authority and
  guard behavior in science-contract files.
- Implement contract-derived WB13 tests from amended contract authority and run
  pre-implementation contract gate evidence before production output-surface
  code edits.
- Produce reproducible Tier-A candidate output artifacts with run manifest,
  output checksums, and schema/field mapping evidence.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- Full Tier-A comparator closeout and risk disposition (`PL14`/`PL15`).
- New hydrology algorithm implementation beyond WB12 reconciliation closure.
- Cross-lane coupling validation package (`INT10`).

## Deliverables
1. WB13 process-contract authority implementation evidence:
   - `artifacts/wb13-contract-implementation-evidence.md`
2. WB13 output schema/units/field mapping note:
   - `artifacts/wb13-output-surface-schema-and-field-mapping.md`
3. WB13 contract-derived test implementation evidence:
   - `artifacts/wb13-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/wb13-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/wb13-implementation-and-test-evidence.md`
6. Tier-A candidate output manifest and checksums:
   - `artifacts/wb13-tier-a-candidate-manifest-and-checksums.md`
7. Comparator-readiness evidence:
   - `artifacts/wb13-comparator-readiness-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/wb13-typed-seam-non-regression-evidence.md`
9. Kernel profile compliance checklist:
   - `artifacts/wb13-kernel-profile-compliance-checklist.md`
10. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wb13_disposition.md`
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb12-runoff-storage-reconciliation-kernels-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb12-runoff-storage-reconciliation-kernels-001/artifacts/wb12_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13-growth-transition-kernel-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13-growth-transition-kernel-001/artifacts/pl13_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl09a-pre-execution-preconditions-clearance-001/artifacts/precondition-1-h5-wat-structure-diff-diagnosis.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-summary-accumulator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/comparator_tier_routing_metadata.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-summary-accumulator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm WB12 and PL13 completion state and WB13 queue scope.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for WB13 output authority.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate results before
  production output-surface code edits.

### Phase 3 - Output-Surface Implementation
- Implement deterministic WB13 daily output emission with typed failure
  propagation for invalid output domains.

### Phase 4 - Verification
- Run targeted output-surface/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish contract + contract-test implementation evidence and comparator
  readiness posture.

## Exit Criteria
- WB13 comparator-ready daily water-balance output surface is implemented.
- Canonical WB13-relevant contracts are implemented in SC files (not just
  proposed).
- Contract-derived WB13 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production output code edits.
- Reproducible Tier-A candidate output is generated with documented schema,
  field mapping, and checksums.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: scientific hydrology output-surface and contract/test implementation.
