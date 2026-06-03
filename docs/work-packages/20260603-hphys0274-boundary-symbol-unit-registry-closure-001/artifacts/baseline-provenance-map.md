# Baseline Provenance Map

Status: completed
Evidence mode: static

Static: HPHYS0274 did not port or alter process-physics equations. Unit
provenance is drawn from active openWEPP governance, contract references, source
seam names, and existing output metadata.

Ran: not-run; provenance mapping is static.

## Authority Inputs

- `docs/specifications/unit-governance.md`: registry schema, authority order,
  canonical internal unit classes, scalar-exception policy, and gate posture.
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`: climate
  seam authority for precipitation, radiation, temperature, and wind symbols.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`: snow
  and winter hourly state authority.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`: WAT
  publication and aggregate storage authority.
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`: soil layer
  depth, theta, porosity, conductivity, and layer-count authority.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`: `Dp`/`Pe`
  percolation/deep-seepage authority.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`: lateral
  and subsurface-flow publication authority.
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`: `Ep`/`Es`/`Er`
  ET publication authority.

## Source-Seam Evidence

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`
  inserts climate runtime aliases including `prcp`, `rad`, `tmax`, `tmin`,
  `tdpt`, `wind`, `vwind`, and intensity templates.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  projects soil layer depths from millimeters to `dg`/`solthk` meters and
  inserts theta, porosity, and `ssc` runtime aliases.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
  seeds `snow.runtime_swe`, `snow.runtime_depth_m`,
  `snow.runtime_density_kg_m3`, and snow option scalars.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  publishes snow hourly depth/density/melt diagnostics and winter hourly
  radiation/temperature/wind aliases.
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs` publishes existing
  WAT schema unit metadata for high-risk output columns.

## Legacy Baseline Note

The pinned `/workdir/wepp-forest_260430_baseline` remains the legacy process
physics authority, but HPHYS0274 is registry/governance implementation only.
No baseline equation, constant, or routine was migrated in this package.
