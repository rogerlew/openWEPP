# 20260529-hparity04-percolation-lateralflow-soilwater-closure-001

## Status
- state: queued
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPARITY04 to close semantic parity deviations for `Dp`, `latqcc`,
`SoilWaterTotal`, and `Total-Soil` in hillslope `H.wat` outputs.

## Why This Package Exists
After HPARITY03, the residual hydrology/storage continuity family remains:
`Dp`, `latqcc`, `SoilWaterTotal`, and `Total-Soil` (all `39/39` failures in
current evidence). This package closes the percolation, lateral-subsurface
flow, and soil-water publication lineage required for full hillslope parity.

## Scope
### Included
- Contract amendments for percolation/subsurface flow and soil-water total
  publication lineage in canonical SC contracts.
- Contract-derived tests for Dp/latqcc/SoilWaterTotal/Total-Soil invariants.
- Production runtime/publication closure for this column family.
- 39-hillslope semantic rerun evidence and residual analysis for this family.

### Explicitly Out of Scope
- Profile-capacity closure (HPARITY02 scope).
- RM/ET/snow closure (HPARITY03 scope).
- Final hold-lift closeout and watershed closure package (HPARITY05 scope).

## Closure Measures (Required)
1. `MEASURE-HP04-001`: `Dp` and `latqcc` fail counts across 39 hillslopes are
   each reduced from `39` to `0`.
2. `MEASURE-HP04-002`: `SoilWaterTotal` and `Total-Soil` fail counts across 39
   hillslopes are each reduced from `39` to `0`.
3. `MEASURE-HP04-003`: `|SoilWaterTotal - Total-Soil| <= 1e-9` for all common
   rows in the rerun cohort.
4. `MEASURE-HP04-004`: control columns closed in HPARITY02/03 remain passing.

## Deliverables
1. `artifacts/hparity04-flow-soilwater-gap-matrix.md`
2. `artifacts/hparity04-contract-implementation-evidence.md`
3. `artifacts/hparity04-contract-test-implementation-evidence.md`
4. `artifacts/hparity04-preimplementation-contract-gate.md`
5. `artifacts/hparity04-implementation-and-test-evidence.md`
6. `artifacts/hparity04-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hparity04_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for Dp/latqcc/
   soil-water lineage and invariants.
2. Implement contract-derived tests for this family.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production runtime/publication edits for this family only.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Physics and symbol provenance must trace to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No surrogate/proxy hydrology equations are permitted.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity03-rainmelt-energy-snow-column-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hparity04-percolation-lateralflow-soilwater-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
- `tests/integration/hparity04_flow_soilwater_parity_contract.rs`
- `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPARITY04 authorization from HPARITY03 handoff and freeze to
  Dp/latqcc/soil-water family only.

### Phase B - Contract/spec authority updates
- Amend canonical contract rows for percolation, lateral subsurface flow, and
  soil-water total publication invariants.
- Update science-contract index references for HPARITY04.

### Phase C - Contract-derived tests
- Add tests enforcing row-level continuity and soil-water alias invariants.

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production edits.

### Phase E - Production implementation
- Implement runtime/publication closure for the four-column family.

### Phase F - Validation and parity rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Rerun 39-hillslope semantic comparison and summarize deltas.

### Phase G - Dual review, dual verification, disposition
- Complete dual review/verification artifacts and publish disposition with
  family-specific closure evidence.

## Exit Criteria
- Closure measures `MEASURE-HP04-001..004` are satisfied and evidenced.
- Dp/latqcc/soil-water family no longer appears in always-fail column reports.
- Handoff contains HPARITY05 closeout-ready execution instructions.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
