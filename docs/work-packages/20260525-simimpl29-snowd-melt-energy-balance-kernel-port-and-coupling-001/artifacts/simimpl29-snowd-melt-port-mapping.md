# SIMIMPL29 Snowd/Melt Port Mapping

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Baseline lineage mapped into `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`:
  - `snowd.for` hourly snow depth/density state update flow ->
    active-coupling hourly state progression and depth/density before/after
    symbol publication.
  - `melt.for` hourly melt energy-balance path ->
    `compute_simimpl29_melt_hour(...)` and per-hour melt writeback.
  - `winter.for` coupling context usage -> daily + hourly forcing symbol
    consumption (`snow.hourly.rain_m_*`, `snow.hourly.snowfall_m_*`,
    `winter.hourly.rad_mj_m2_*`, `winter.hourly.air_temp_c_*`,
    `winter.hourly.cloud_fraction_*`) with typed required-symbol guards.
- Runtime carry-state closure implemented and persisted across executions using
  boundary symbols (`snow.runtime_*`).

## Ran
- `rg -n "compute_simimpl29_melt_hour|SnowHourlyState|SNOW_HOURLY_|SNOW_RUNTIME_" crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `rg -n "seed_hillslope_runtime_surface_from_snow|snow.runtime_depth_m|snow.runtime_density_kg_m3|snow.runtime_settle_day_count" crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
