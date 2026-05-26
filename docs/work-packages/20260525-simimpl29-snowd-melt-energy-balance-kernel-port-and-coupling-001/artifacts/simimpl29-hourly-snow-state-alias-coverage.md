# SIMIMPL29 Hourly Snow State Alias Coverage

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL29 publishes required hourly snow kernel-state symbol families:
  - `snow.hourly.depth_before_m_0001..0024`
  - `snow.hourly.depth_available_m_0001..0024`
  - `snow.hourly.density_before_kg_m3_0001..0024`
  - `snow.hourly.depth_after_m_0001..0024`
  - `snow.hourly.density_after_kg_m3_0001..0024`
  - `snow.hourly.melt_m_0001..0024`
- Runtime carry-state writeback is now explicit:
  - `snow.runtime_swe`
  - `snow.runtime_depth_m`
  - `snow.runtime_density_kg_m3`
  - `snow.runtime_settle_day_count`
- Remaining deferred hourly family is explicit and unchanged:
  - `frost.hourly.*`

## Ran
- `cargo test -p openwepp --test clim05_snow_runtime_kernel_contract clim05_contract_conformance_couples_snow_controls_into_hydrology_reconciliation`
- `rg -n "SNOW_HOURLY_DEPTH_BEFORE_ROOT|SNOW_HOURLY_DEPTH_AVAILABLE_ROOT|SNOW_HOURLY_DENSITY_BEFORE_ROOT|SNOW_HOURLY_DEPTH_AFTER_ROOT|SNOW_HOURLY_DENSITY_AFTER_ROOT|SNOW_HOURLY_MELT_ROOT|SNOW_RUNTIME_DEPTH_M_SYMBOL|SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL|SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL" crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
