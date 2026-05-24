# 20260523-wb17-et-physics-equivalence-port-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Replace WB11 evapotranspiration surrogate behavior with legacy-equivalent ET
physics authority, including explicit plant/soil/residue partition semantics,
under contract-first sequencing in the monolithic openWEPP scientific
hydrology/erosion model.

## Why This Package Exists
PL15R reversal retained PL08 hold and the PL09 queue addendum created
`WB17-et-physics-equivalence-port` as the first physics-authority closure step
in the new parity-recovery lane. Static evidence showed current ET behavior is
surrogate demand-consumption logic and not full ET physics authority.

This package is contract-first and physics-authority-bound: canonical science
contracts must explicitly encode ET physics equations, symbol tables,
units/guards, and partition semantics before production kernel code edits.
Contract-derived tests and pre-implementation gate evidence are mandatory before
kernel implementation changes.

## Scope
### Included
- Implement canonical ET physics-authority amendments in
  `SC-EVAP-001` and `SC-WATBAL-001`, including explicit equation forms,
  variable definitions, units, domain guards, and plant/soil/residue partition
  semantics.
- Derive ET physics migration authority from the pinned legacy baseline
  `/workdir/wepp-forest_260430_baseline` (commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) and record equation-level
  provenance in WB17 artifacts.
- Amend companion contracts/index references where ET physics authority or
  alias continuity requires cross-contract updates.
- Implement WB17 contract-derived tests from amended canonical ET authority.
- Record pre-implementation contract-gate evidence proving contract +
  contract-test completion before production kernel edits.
- Replace WB11 ET surrogate production behavior with equation-driven ET physics
  behavior aligned to canonical contract authority.
- Produce equation-vector and partition-trajectory evidence for ET parity
  direction and runtime closure.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- WB18 percolation physics-equivalence port.
- WB19 lateral/drainage physics-equivalence port.
- WB20 forward water-balance solver lane execution.
- CLI10 executable driver implementation.
- Tier-A hold-lift disposition updates beyond WB17 scope.

## Deliverables
1. WB17 process-contract authority implementation evidence:
   - `artifacts/wb17-contract-implementation-evidence.md`
2. WB17 ET physics authority and guard map note:
   - `artifacts/wb17-et-physics-authority-and-guard-map.md`
3. WB17 contract-derived test implementation evidence:
   - `artifacts/wb17-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/wb17-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/wb17-implementation-and-test-evidence.md`
6. ET equation-vector parity evidence:
   - `artifacts/wb17-et-equation-vector-parity-evidence.md`
7. ET partition trajectory evidence:
   - `artifacts/wb17-et-partition-trajectory-evidence.md`
8. Typed-seam non-regression evidence:
   - `artifacts/wb17-typed-seam-non-regression-evidence.md`
9. Legacy ET physics provenance map:
   - `artifacts/wb17-legacy-et-physics-provenance-map.md`
10. Kernel profile compliance checklist:
   - `artifacts/wb17-kernel-profile-compliance-checklist.md`
11. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wb17_disposition.md`
12. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb11-et-perc-lateral-drain-kernels-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb11-et-perc-lateral-drain-kernels-001/artifacts/wb11_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb15-canopy-interception-kernel-coupling-001/artifacts/wb15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb16-peak-runoff-kernel-001/artifacts/wb16_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/wepp-forest_260430_baseline` (legacy ET migration authority)
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-wb17-et-physics-equivalence-port-001/**`
- `docs/work-packages/README.md`
- `references to /workdir/wepp-forest_260430_baseline ET authority in WB17 artifacts`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15R reversal posture, WB17 queue objective, and current WB11 ET
  surrogate behavior baseline.

### Phase 1 - Canonical Physics-Contract Implementation
- Implement required ET physics authority in canonical `SC-*` contracts,
  including equation forms, symbol/alias continuity, partition semantics,
  units, and guard invariants.
- Publish WB17 ET physics provenance mapping from baseline legacy authority to
  canonical contract equations before kernel code edits.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived ET tests from amended canonical authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement ET production behavior to match canonical ET physics-contract
  authority and remove surrogate-only logic from production claim basis.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish WB17 contract/contract-test/implementation evidence and closure
  posture.

## Exit Criteria
- `WB17` queue objective is evidence-backed.
- Canonical science contracts explicitly contain ET physics authority
  (equations, symbols, units, guards, partition semantics) in `SC-EVAP-001`
  and `SC-WATBAL-001`; work-package notes alone do not satisfy closure.
- Legacy ET migration provenance is explicit and reproducible against pinned
  `/workdir/wepp-forest_260430_baseline` authority in the WB17 provenance-map
  artifact.
- Production ET behavior is equation-driven and no longer governed by the WB11
  surrogate demand-consumption reduction.
- Contract-derived WB17 tests are implemented and executed.
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- ET equation-vector and partition trajectory evidence are produced.
- Dual review and dual verification artifacts are completed and dispositioned.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: ET kernel physics-authority and contract/test implementation.
