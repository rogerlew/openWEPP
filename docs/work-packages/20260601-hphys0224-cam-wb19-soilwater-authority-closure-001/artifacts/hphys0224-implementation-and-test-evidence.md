# HPHYS0224 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production Remediation (Static)

1. `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
   - Added `wb19_apply_soil_water_withdrawal(...)` helper:
     - enforces `realized_withdrawal <= wb11_soil_water_before` via typed
       flux-domain guard,
     - applies explicit subtraction without clamp fallback,
     - validates resulting `wb11_soil_water` state domain.

2. `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
   - Replaced lateral/drainage post-subtraction flooring (`max(0.0)`) with
     helper-based hard-fail subtraction flow.
   - Preserved existing target-cap guards (`q <= q_lateral_target`,
     `Qdd <= q_drainage_target`) and added the new soil-water-cap guard as a
     separate runtime constraint.

## Ran Validation

- `cargo fmt --check` (pass)
- `cargo clippy --workspace --all-targets -- -D warnings` (pass)
- `cargo test --workspace` (pass)
- `cargo deny check` (pass; warnings only: duplicate crates + unmatched
  license allowlist entries)
- Targeted WB19 + suite tests:
  - `cargo test --test hphys0224_wb19_withdrawal_soilwater_cap_contract` (pass)
  - `cargo test --test auth06_fixture_provenance_hash_enforcement_contract --test hphys0224_wb19_withdrawal_soilwater_cap_contract` (pass)
  - `cargo test --test wb19_lateral_drainage_physics_kernel_contract --test hphys0219_wb19_coca_threshold_contract --test hphys0221_wb19_water_yield_fcdep_coupling_contract` (pass)

## 39-Hillslope Rerun + Semantic Readjudication (Ran)

- Run root: `/tmp/hphys0224_20260601T054337Z`
- Hillslope run status:
  `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_batch_status.tsv`
  (`39/39`, all `rc=0`)
- Semantic status:
  `/tmp/hphys0224_20260601T054337Z/parity/reports/semantic_status.tsv`
  (`39/39`, all `rc=0`)
- Summary:
  `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_semantic_summary.json`

## Residual-Family Delta vs HPHYS0223 (Ran)

Reference:
`/tmp/hphys0223_20260531T201410Z/parity/reports/hillslope_semantic_summary.json`

- `Dp`: fail-count delta `0`, mean-abs-diff delta `0.0`
- `latqcc`: fail-count delta `0`, mean-abs-diff delta `0.0`
- `Total-Soil`: fail-count delta `0`, mean-abs-diff delta `0.0`
- `SoilWaterTotal`: fail-count delta `0`, mean-abs-diff delta `0.0`
- `ProfileFCStore`: fail-count delta `0`, mean-abs-diff delta `0.0`
- `ProfileWPStore`: fail-count delta `0`, mean-abs-diff delta `0.0`
