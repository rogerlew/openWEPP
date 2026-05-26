# SIMIMPL28 Reserved Hourly Alias Coverage

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL28 emits required forcing-hourly symbol families:
  - `winter.hourly.rad_mj_m2_0000..0023`
  - `winter.hourly.air_temp_c_0000..0023`
  - `winter.hourly.cloud_fraction_0000..0023`
  - `snow.hourly.rain_m_0000..0023`
  - `snow.hourly.snowfall_m_0000..0023`
- Deferred hourly kernel-state families remain out of SIMIMPL28 scope and are
  explicitly staged to SIMIMPL29:
  - `snow.hourly.depth_*`, `snow.hourly.density_*`, `snow.hourly.melt_m`,
    `frost.hourly.*`.

## Ran
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_emits_simimpl28_hourly_forcing_symbols`
- `rg -n "winter\.hourly\.rad_mj_m2|winter\.hourly\.air_temp_c|winter\.hourly\.cloud_fraction|snow\.hourly\.rain_m|snow\.hourly\.snowfall_m" crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
