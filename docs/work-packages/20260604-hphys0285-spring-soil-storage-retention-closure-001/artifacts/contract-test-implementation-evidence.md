# Contract-Test Implementation Evidence

Status: complete
Evidence mode: Static

## Contract-Derived Tests

Static:
- Added `tests/integration/hphys0285_spring_soil_storage_retention_contract.rs`.
- Registered the test in `Cargo.toml` as `hphys0285_spring_soil_storage_retention_contract`.
- Extended `tests/integration/hphys0284_negative_melt_snowpack_state_contract.rs` for the HPHYS0285 Claude review disposition.

## Assertion Scope

Static:
- `hphys0285_contract_direct_rain_infiltration_enters_profile_storage`
  - Direct-rain vector with no active snow, no snowpack state, and `wb18_perc_lane_substeps = 24.0` to exercise substep ingress.
  - Positive `wb12_rainfall_input = 0.001 m` fully infiltrates under high GAML conductivity.
  - Requires `wb12_infiltration ≈ 0.001 m`, `Q ≈ 0`, aggregate `wb11_soil_water` increase, and WB18 layer theta-sum increase.
- `hphys0285_contract_inactive_stale_snow_state_does_not_gate_direct_rain_ingress`
  - Direct-rain vector with inactive stale negative `snow.runtime_swe`.
  - Requires direct-rain infiltration/storage ingress to proceed instead of being active-snow-gated.
- `hphys0285_contract_dry_cold_stale_snow_state_does_not_gate_percolation`
  - Dry no-event vector with cold projected snow controls and inactive stale negative SWE.
  - Requires no-event percolation path to avoid invoking active-snow coupling.
- `hphys0284_large_negative_melt_state_overdraw_fails_closed`
  - Synthetic inconsistent-SWE snowpack vector where available depth permits melt processing but runtime SWE is too small for the corrected carried state-loss.
  - Requires a typed kernel domain-failure report instead of silently canonicalizing material negative SWE to zero.

## Expected Pre-Fix Behavior

Static:
- The pre-fix WB18 same-pass ingress branch was active-snow-gated.
- With direct rain but no active snow, WB12 could publish infiltration while WB18 layer storage remained unchanged.
- The direct-rain test failed before production code changes and passed after the HPHYS0285 fix. MOFE carry/runon storage-ingress coverage is deferred with the narrowed contract scope.
