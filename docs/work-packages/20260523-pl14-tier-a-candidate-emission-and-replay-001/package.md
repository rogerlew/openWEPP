# 20260523-pl14-tier-a-candidate-emission-and-replay-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Execute strict Tier-A comparator replay using direct openWEPP candidate output
versus the pinned legacy baseline with reproducible command trace, comparator
JSON artifacts, and provenance hashes.

## Why This Package Exists
The PL09 hold-lift queue defines `PL14` as the closeout-stage replay gate after
`INT10` coupling validation and `PL13A` alias continuity closure. `PL14` is the
first package that must run direct openWEPP-vs-legacy strict Tier-A comparator
replay under pinned-baseline provenance, producing execution evidence consumed
by `PL15` hold-lift disposition.

This package is contract-first and implementation-bound: canonical PL14 replay
contracts and contract-derived tests must be implemented (not only planned or
documented), and executed evidence is required before PL14 disposition.

## Scope
### Included
- Emit and stage direct openWEPP candidate Tier-A outputs for strict replay.
- Execute strict Tier-A comparator lane against pinned legacy baseline with
  explicit include surfaces and deterministic tolerance configuration.
- Persist comparator JSON artifacts and supporting command/provenance evidence.
- Record binary/tool hashes and output checksums for reproducible provenance.
- Implement canonical contract amendments required to represent PL14 replay
  authority and guard behavior in contract/spec files.
- Implement contract-derived PL14 tests from amended contract authority and run
  pre-implementation contract gate evidence before production replay/harness
  code edits.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- Residual Tier-A delta risk disposition and final hold-lift verdict (`PL15`).
- New standalone plant/hydrology kernel implementation beyond completed
  prerequisites (`PL13`, `WB13`, `INT10`).
- Broad Tier-B/Tier-C comparator closeout as promotion gate.

## Deliverables
1. PL14 process-contract authority implementation evidence:
   - `artifacts/pl14-contract-implementation-evidence.md`
2. PL14 replay lane configuration and guard map:
   - `artifacts/pl14-replay-lane-configuration-and-guard-map.md`
3. PL14 contract-derived test implementation evidence:
   - `artifacts/pl14-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/pl14-preimplementation-contract-gate.md`
5. Tier-A comparator run provenance manifest (commands + hashes):
   - `artifacts/pl14-comparator-run-provenance-manifest.md`
6. Tier-A comparator delta report:
   - `artifacts/pl14-tier-a-comparator-delta-report.md`
7. Comparator JSON artifact index:
   - `artifacts/pl14-comparator-json-artifact-index.md`
8. Execution and gate evidence:
   - `artifacts/pl14-implementation-and-test-evidence.md`
9. Typed-seam non-regression evidence:
   - `artifacts/pl14-typed-seam-non-regression-evidence.md`
10. Kernel profile compliance checklist:
   - `artifacts/pl14-kernel-profile-compliance-checklist.md`
11. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl14_disposition.md`
12. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`
13. Persisted comparator JSON artifacts:
   - `artifacts/h5_wat_comparator.json`
   - `artifacts/h5_plot_comparator.json`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/decisions/0003-parity-semantic-not-bit.md`
- `/workdir/openWEPP/docs/numerics/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-int10-plant-water-coupling-validation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-int10-plant-water-coupling-validation-001/artifacts/int10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13a-alias-continuity-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl13a-alias-continuity-closure-001/artifacts/pl13a_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/artifacts/wb13-tier-a-candidate-manifest-and-checksums.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-run-provenance-manifest.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/h5_wat_comparator.json`
- `/workdir/openWEPP/crates/openwepp-comparator-metadata/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-summary-accumulator/src/lib.rs`
- `/workdir/openWEPP/tests/integration/comparator_tier_routing_metadata.rs`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/**`
- `crates/openwepp-comparator-metadata/src/lib.rs`
- `crates/openwepp-summary-accumulator/src/lib.rs`
- `docs/work-packages/20260523-pl14-tier-a-candidate-emission-and-replay-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm `INT10` and `PL13A` completion state and `PL14` queue scope.
- Confirm pinned baseline provenance and comparator-tool entrypoints.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for PL14 replay authority.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests from amended contract authority.
- Execute and record pre-implementation contract-gate results before replay
  harness or production replay code edits.

### Phase 3 - Candidate Emission and Replay Execution
- Emit direct openWEPP candidate outputs and execute strict Tier-A comparator
  replay against pinned legacy baseline.

### Phase 4 - Verification
- Persist comparator JSON artifacts, command traces, hashes, and run checksums.
- Run required repository gates when code changes are in scope.

### Phase 5 - Disposition
- Publish replay provenance + comparator outcome evidence for PL15 consumption.

## Exit Criteria
- Strict Tier-A comparator replay is executed with reproducible provenance.
- Comparator JSON artifacts are persisted with clear artifact indexing.
- Command trace, binary/tool hashes, and output checksums are recorded.
- Canonical PL14-relevant contracts are implemented in contract/spec files
  (not just proposed).
- Contract-derived PL14 tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before replay/harness code edits.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: comparator replay/provenance closeout package with optional
  contract/test/harness refinements.
