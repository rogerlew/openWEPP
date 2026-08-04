# Operand Lineage

Status: `complete / production edits authorized`

Evidence mode: `Static: contract and source trace`

## Stage-3 Daily Liquid Identity

| JSONL v4 field | Units / cadence | Exact typed source | Meaning | Rejected adjacent alias |
|---|---|---|---|---|
| `stage3_incoming_liquid_m` | `m` / day | `DirectSnowStage3Diagnostics.incoming_liquid_m` | Liquid admitted to the Stage-3 layer-routing solve. | Top-level daily raw or routed CoE melt. |
| `stage3_routed_liquid_m` | `m` / day | `DirectSnowStage3Diagnostics.routed_liquid_m` | Liquid leaving the Stage-3 layers after retention/refreeze. | `DirectSnowLiquidPartition.routed_melt_m`. |
| `stage3_retained_liquid_delta_m` | `m` / day, signed | `DirectSnowStage3Diagnostics.retained_liquid_m` | Net Stage-3 layer-store change returned by `route_stage3_liquid_through_layers`. The JSON name makes the existing delta semantics explicit. | CoE `liquid_water_retained_after_m`, a store rather than this delta. |
| `stage3_refrozen_liquid_m` | `m` / day | `DirectSnowStage3Diagnostics.refrozen_liquid_m` | Routed liquid converted to layer ice by Stage 3. | Layer cumulative refreeze sum when used a second time. |
| `stage3_liquid_closure_residual_m` | `m` / day | `DirectSnowStage3Diagnostics.liquid_closure_residual_m` | Producer residual guarded at `1e-9 m`; supporting evidence only. | Treating the producer residual as proof without reconstructing it. |

The independent consumer identity is:

    incoming - routed - retained_delta - refrozen = residual

Every operand above is diagnostic publication of an existing typed value. None
is a new snow state, forcing, flux, equation, or authoritative public output.

## Signed-Hour Context

| JSONL v4 field | Units / cadence | Exact source | Time-basis semantics |
|---|---|---|---|
| `wind_m_s` | `m s^-1` / day | `DirectActiveSnowPartitionInputs.wind_m_s` | Daily forcing used by all hourly CoE evaluations. |
| `dewpoint_c` | `degC` / day | `DirectActiveSnowPartitionInputs.dewpoint_c` | Daily forcing used by all hourly CoE evaluations. |
| `canopy_cover_fraction` | `1` / day | `DirectActiveSnowPartitionInputs.canopy_cover_fraction` | Daily forcing used by the snow partition. |
| `air_temperature_c` | `degC` / hour | `inputs.hourly[hour].air_temperature_c` | Existing SIMIMPL28 hourly forcing. |
| `radiation_mj_m2` | `MJ m^-2` / hour | `inputs.hourly[hour].radiation_mj_m2` | Existing hourly climate radiation. |
| `cloud_fraction` | `1` / hour | `inputs.hourly[hour].cloud_fraction` | Existing hourly cloud forcing. |
| `coe_melt_uncapped_m` | `m` / hour | `SnowHourlyState.melt_diagnostics.coe_melt_uncapped_m` | Sum of the four CoE contributions before pack cap. |
| `coe_melt_applied_m` | `m` / hour | `SnowHourlyState.melt_diagnostics.coe_melt_applied_m` | Cap-adjusted signed raw melt before redistribution. |
| `routed_melt_m` | `m` / hour | `SnowCouplingOutcome.hourly_routed_melt` | Authoritative nonnegative daily-closed CoE/rain routed shape. |
| `liquid_holding_capacity_m` | `m` / hour | `SnowHourlyState.liquid_holding_capacity_m` | Capacity evaluated during that hour. |
| `liquid_water_retained_before_m` | `m` / hour | `SnowHourlyState.liquid_water_retained_before_m` | CoE retained store before the hourly state transition. |
| `liquid_water_retained_after_m` | `m` / hour | `SnowHourlyState.liquid_water_retained_after_m` | CoE retained store after the hourly state transition. |
| `liquid_water_released_m` | `m` / hour | `SnowHourlyState.liquid_water_released_m` | CoE store drainage during the hour. |
| `rain_released_m` | `m` / hour | `SnowHourlyState.rain_released_m` | Rain not retained by the pack during the hour. |
| `sublimation_m` | `m` / hour | `SnowHourlyState.sublimation_m` | Existing selected CoE Stage A/B hourly sublimation. |
| `pack_depth_before_m` | `m` / hour | `ActiveSnowPackState.depth_m` sampled after settle-clock advance and before hourly phase/melt mutation | Exact state snapshot, not reconstructed from SWE/density. |
| `pack_depth_after_m` | `m` / hour | `ActiveSnowPackState.depth_m` after hourly state mutation | Exact state snapshot. |
| `pack_density_before_kg_m3` | `kg m^-3` / hour | `ActiveSnowPackState.density_kg_m3` at the same before boundary | Exact state snapshot. |
| `pack_density_after_kg_m3` | `kg m^-3` / hour | `ActiveSnowPackState.density_kg_m3` after hourly state mutation | Exact state snapshot. |

## Stage-3 Hourly Thermal State

The v4 arrays `stage3_hourly_{active,lower}_{mass_kg_m2,depth_m,
temperature_c,cold_content_j_m2}` are direct projections of the corresponding
fields in each existing `DirectSnowSurfaceEnergyHourDiagnostics` row. They are
duration-weighted within-hour diagnostics accumulated across Stage-3 substeps,
not end-of-hour snapshots. Lower-volume values retain full-hour weighting with
zero contribution while no lower volume exists; the separately published
`stage3_hourly_lower_present_fraction` is therefore required for conditional
interpretation. The writer does not recompute any of these values.

## Real Consumer Chain

`DirectActiveSnowPartitionInputs` and `SnowHourlyState` ->
`SnowCouplingOutcome` -> `DirectSnowAccumulationMeltDiagnostics` ->
`DirectSnowLiquidPartition` -> `r7h_direct_production_snow_trace_line` ->
`OPENWEPP_R7H_SNOW_TRACE_PATH` JSONL -> package-independent parser.

`DirectSnowStage3Diagnostics` already flows through the same partition and
writer. The implementation may extend only typed diagnostic carriers and the
real formatter; wrappers, test adapters, shadow paths, and runtime-memory
inspection cannot satisfy the claim.
