# 20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Re-run strict Tier-A comparator replay using direct openWEPP candidate output
versus the pinned legacy baseline after post-PL15 kernel-closure execution,
with reproducible command trace, comparator JSON artifacts, and provenance
hashes for PL15R decision consumption.

## Why This Package Exists
PL15 retained the PL08 hold based on unresolved strict Tier-A comparator
blockers captured in PL14/PL15 artifacts. Since the post-PL15 closure wave
(`WB14`, `PL16`, `PL17`, `CLIM05`, `CLIM06`, `WB15`, `IRRIG10`, `WB16`,
`WS10`, `ARCH22`, `CLIM07`) is now evidence-complete, PL14R is the formal
recheck replay lane to refresh strict Tier-A evidence before PL15R hold-lift
supersession review.

This package is contract-first and implementation-bound: canonical PL14R replay
contracts and contract-derived tests must be implemented and
pre-implementation contract-gate evidence recorded before production
replay/harness code edits.

## Scope
### Included
- Re-emit and stage direct openWEPP candidate Tier-A outputs for strict replay.
- Re-execute strict Tier-A comparator lane against pinned legacy baseline with
  explicit include surfaces and deterministic tolerance configuration.
- Persist refreshed comparator JSON artifacts and supporting
  command/provenance evidence.
- Record binary/tool hashes and output checksums for reproducible provenance.
- Implement canonical contract/spec amendments required to represent PL14R
  replay authority and guard behavior in contract/spec files.
- Implement contract-derived PL14R tests from amended contract authority and run
  pre-implementation contract gate evidence before production replay/harness
  code edits.
- Preserve ARCH15/ARCH21/ARCH22 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- Final PL08 hold-lift supersession verdict (`PL15R`).
- New standalone plant/hydrology kernel implementation beyond completed
  prerequisite packages.
- Broad Tier-B/Tier-C comparator closeout as release gate replacement.

## Deliverables
1. PL14R process-contract authority implementation evidence:
   - `artifacts/pl14r-contract-implementation-evidence.md`
2. PL14R replay lane configuration and guard map:
   - `artifacts/pl14r-replay-lane-configuration-and-guard-map.md`
3. PL14R contract-derived test implementation evidence:
   - `artifacts/pl14r-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/pl14r-preimplementation-contract-gate.md`
5. Tier-A comparator rerun provenance manifest (commands + hashes):
   - `artifacts/pl14r-comparator-run-provenance-manifest.md`
6. Tier-A comparator rerun delta report:
   - `artifacts/pl14r-tier-a-comparator-delta-report.md`
7. Comparator JSON artifact index:
   - `artifacts/pl14r-comparator-json-artifact-index.md`
8. Execution and gate evidence:
   - `artifacts/pl14r-implementation-and-test-evidence.md`
9. Typed-seam non-regression evidence:
   - `artifacts/pl14r-typed-seam-non-regression-evidence.md`
10. Kernel profile compliance checklist:
   - `artifacts/pl14r-kernel-profile-compliance-checklist.md`
11. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/pl14r_disposition.md`
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
- `/workdir/openWEPP/docs/work-packages/20260523-pl14-tier-a-candidate-emission-and-replay-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl14-tier-a-candidate-emission-and-replay-001/artifacts/pl14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl16-growth-physics-kernelization-001/artifacts/pl16_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl17-decomposition-physics-kernelization-001/artifacts/pl17_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim05-snow-runtime-kernel-port-001/artifacts/clim05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim06-frost-frozen-soil-kernel-port-001/artifacts/clim06_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb15-canopy-interception-kernel-coupling-001/artifacts/wb15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-irrig10-irrigation-runtime-kernel-port-001/artifacts/irrig10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/artifacts/wb16_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-ws10-channel-impoundment-production-kernels-001/artifacts/ws10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-arch22-typed-state-surface-closure-001/artifacts/arch22_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim07-climate-comparator-and-closure-evidence-001/artifacts/clim07_disposition.md`
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
- `docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase 0 - Intake
- Confirm completion state of post-PL15 closure-wave dependencies.
- Confirm pinned baseline provenance and comparator-tool entrypoints.

### Phase 1 - Contract Implementation
- Implement required canonical contract amendments for PL14R replay authority.

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
- Publish replay provenance + comparator outcome evidence for PL15R consumption.

## Exit Criteria
- Strict Tier-A comparator replay is re-executed with reproducible provenance.
- Candidate lane includes required comparator surfaces (`H5.wat.dat`,
  `H5.plot.dat`) in replay artifact set.
- Comparator JSON artifacts are persisted with clear artifact indexing.
- Command trace, binary/tool hashes, and output checksums are recorded.
- Canonical PL14R-relevant contracts are implemented in contract/spec files
  (not just proposed).
- Contract-derived PL14R tests are implemented and executed (not just planned).
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before replay/harness code edits.
- Existing typed-seam closure posture from ARCH15/ARCH21/ARCH22 remains
  non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: comparator replay/provenance recheck package with optional
  contract/test/harness refinements.
