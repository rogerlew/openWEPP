# Operand Lineage

Status: `EXECUTED-PRE-EDIT`

Evidence class: `Static:`

W4 typed publication operand lineage recorded before production
routing/publication edits. W4 is intended to preserve current output schemas and
current routed-stage semantics unless a later contract-governed delta is
explicitly accepted.

Source authority:

- `docs/architecture/watershed-runtime-architecture-specification.md`
  Sections 3.5, 3.6, and 4.5 require typed network and publication frames.
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` governs
  channel routing authority.
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` governs
  impoundment routing authority.
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` governs
  sediment publication and diagnostic posture.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` governs
  water-balance output lineage and placeholder/diagnostic publication posture.
- `crates/openwepp-watershed-output/src/writers.rs` defines current Parquet
  schemas and unit metadata for protected watershed interchange outputs.

## Typed Publication Operands

| Output operand | Units | Normalization / denominator | Area or volume basis | Typed source field | Authority | Status | Rejected aliases or wrong candidates | Metadata/schema alignment |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `runoff_volume_m3` | `m^3` | Sum across routed channel nodes | Routed channel volume | `WatershedPublicationFrame.runoff_volume_m3` from `RoutedChannelState.runoff_volume_m3` | `SC-ROUTE-001`, output schema | authoritative routed water volume | reject hillslope peak-only volume, impoundment outflow volume, and `runoff_mm` back-conversion without area basis | feeds `Runoff Volume (m^3)` families through `WatershedInterchangeRowSeed` |
| `peak_discharge_m3_s` | `m^3/s` | First routed channel node selected by stable channel id order, preserving current schema behavior | Routed channel peak | `WatershedPublicationFrame.peak_discharge_m3_s` from `RoutedChannelState.peak_discharge_m3_s` | `SC-ROUTE-001`, output schema | authoritative peak-discharge publication for current single-row watershed output | reject sum of all channel peaks, max of impoundment outflows, and hillslope contributor peak as direct substitute | feeds `Peak_Discharge (m^3/s)` / peak fields |
| `sediment_yield_kg` | `kg` | Sum across routed channel sediment publication states | Routed sediment mass | `WatershedPublicationFrame.sediment_yield_kg` from `RoutedChannelState.sediment_yield_kg` | `SC-SED-001`, output schema | authoritative routed sediment publication under current WS19/WS20/WS22 path | reject hillslope detachment sum, deposition sum, concentration-only aliases, and class-fraction sums | feeds sediment-yield fields in protected interchange outputs |
| `particulate_pollutant_kg` | `kg` | Sum across contributing hillslope detachment payloads | Hillslope contribution mass | `WatershedPublicationFrame.particulate_pollutant_kg` from `HillslopeContribution.total_detachment_kg` | pass inventory / current watershed writer schema | diagnostic placeholder matching current output semantics | reject routed `qsed` and total deposition as aliases | feeds particulate pollutant placeholder fields |
| `channel_outflow_m3` | `m^3` | Sum across routed impoundment outflow volumes | Routed impoundment outflow volume | `WatershedPublicationFrame.channel_outflow_m3` from `RoutedImpoundmentState.outflow_volume_m3` | `SC-IMPOUND-001`, output schema | authoritative impoundment routed outflow where impoundments exist; zero otherwise | reject channel runoff volume and `qo` rate without duration | feeds channel outflow fields |
| `channel_baseflow_m3` | `m^3` | Current compatibility scalar carried from `cbase` because the routed stage publishes this field as current writer seed state | Routing global | `WatershedPublicationFrame.channel_baseflow_m3` from `WatershedNetworkFrame.routing_globals.cbase` | `SC-ROUTE-001`, current output schema | diagnostic under current writer seed semantics | reject channel runoff volume, channel loss, and impoundment outflow aliases | feeds baseflow fields |
| `runoff_mm` | `mm` | `runoff_volume_m3 * 1000.0` with current single-row area basis `1.0 m^2` | Current publication area placeholder | `WatershedPublicationFrame.runoff_mm` | `SC-WATBAL-001`, current writer schema | diagnostic placeholder; not independent conservation evidence | reject using hillslope area, channel length, or OFE area unless package amends area authority | feeds `Q`/runoff-mm compatible fields |
| fixed hydrology placeholders (`precipitation_mm`, `rain_melt_mm`, `deep_percolation_mm`, `lateral_flow_mm`, `qofe_mm`, `transpiration_mm`, `evaporation_soil_mm`, `evaporation_residue_mm`, `upstream_q_mm`, `subsurface_runon_mm`, `total_soil_water_mm`, `soil_water_total_mm`, `profile_depth_mm`, `profile_porosity_cap_mm`, `profile_fc_store_mm`, `profile_wp_store_mm`, `interception_mm`, `interception_storage_mm`, `frozen_water_mm`, `snow_water_mm`, `tile_mm`, `irrigation_mm`, `baseflow_mm`) | mixed schema units | Fixed zero values under current watershed routed-stage writer seed | No independent watershed-frame source yet | `WatershedPublicationFrame` zero placeholders | `SC-WATBAL-001`, output schema | diagnostic placeholders only | reject treating zeros as closure proof or substituting hillslope WB13/MOFE values without typed area/time lineage | preserve schema metadata and record no acceptance claim from these placeholders |

## Anti-Alias Acceptance Requirements

- Protected identity checks must compare produced Parquet row content, not
  merely row counts or schema existence.
- Independent reconstruction must recompute the listed operands from typed
  publication-frame fields or produced output rows and reject at least these
  wrong candidates: sediment yield from hillslope detachment, peak discharge
  from summed peaks, channel outflow from `qo` without duration, and runoff
  depth from an unstated area basis.
- Exact self-consistency, non-negativity, and one-sided bounds are supporting
  sanity evidence only; they do not prove conservation closure.
