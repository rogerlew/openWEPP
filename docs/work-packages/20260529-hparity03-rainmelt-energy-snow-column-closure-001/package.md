# 20260529-hparity03-rainmelt-energy-snow-column-closure-001

## Status
- state: queued
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPARITY03 to close `RM`, `Ep`, `Es`, and `Snow-Water` semantic parity
deviations in hillslope `H.wat` outputs using baseline-authoritative process
lineage.

## Why This Package Exists
After HPARITY02 profile-capacity closure, the remaining dominant atmospheric
and cryosphere residuals are `RM`, `Ep`, `Es`, and `Snow-Water` (all `39/39`
failures in current evidence). These columns govern daily water/energy balance
and must be resolved before percolation/subsurface closure.

## Scope
### Included
- Contract amendments for rain+melt partition, ET energy partition, and snow
  storage lineage across canonical SC contracts.
- Contract-derived tests for `RM`, `Ep`, `Es`, and `Snow-Water` invariants and
  publication continuity.
- Production runtime updates for the above column family, including required
  sidecar/runtime projection wiring.
- 39-hillslope semantic rerun evidence for this column family.

### Explicitly Out of Scope
- Profile-capacity residual closure (HPARITY02 scope).
- Dp/latqcc/SoilWaterTotal/Total-Soil residual closure (HPARITY04 scope).
- Final cohort hold-lift closeout (HPARITY05 scope).

## Closure Measures (Required)
1. `MEASURE-HP03-001`: `RM` fail count across 39 hillslopes is reduced from
   `39` to `0`.
2. `MEASURE-HP03-002`: `Ep` and `Es` fail counts across 39 hillslopes are each
   reduced from `39` to `0`.
3. `MEASURE-HP03-003`: `Snow-Water` fail count across 39 hillslopes is reduced
   from `39` to `0`.
4. `MEASURE-HP03-004`: control columns closed by HPARITY02 remain passing.

## Deliverables
1. `artifacts/hparity03-energy-snow-gap-matrix.md`
2. `artifacts/hparity03-contract-implementation-evidence.md`
3. `artifacts/hparity03-contract-test-implementation-evidence.md`
4. `artifacts/hparity03-preimplementation-contract-gate.md`
5. `artifacts/hparity03-implementation-and-test-evidence.md`
6. `artifacts/hparity03-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hparity03_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index amendments for RM/ET/snow
   lineage and invariants.
2. Implement contract-derived tests for this column family.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production runtime updates for this family only.

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
- No surrogate/proxy climate/ET/snow equations are permitted.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hparity03-rainmelt-energy-snow-column-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-input-contract/src/parsers/pmetpara.rs`
- `tests/integration/hparity03_energy_snow_parity_contract.rs`
- `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPARITY03 authorization from HPARITY02 handoff and freeze to
  RM/ET/snow columns only.

### Phase B - Contract/spec authority updates
- Amend SC rows for RM, Ep, Es, and Snow-Water process lineage and guards.
- Update science-contract index references for HPARITY03.

### Phase C - Contract-derived tests
- Add invariant/parity tests for RM/ET/snow publication surfaces.

### Phase D - Pre-implementation contract gate
- Record contract/test readiness before production edits.

### Phase E - Production implementation
- Implement runtime/process closure for RM/ET/snow family.

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
- Closure measures `MEASURE-HP03-001..004` are satisfied and evidenced.
- RM/ET/snow family no longer appears in always-fail column reports.
- Handoff cleanly scopes remaining Dp/latqcc/soil-water work for HPARITY04.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contract/runtime/test updates only; no external auth/network
  changes.
