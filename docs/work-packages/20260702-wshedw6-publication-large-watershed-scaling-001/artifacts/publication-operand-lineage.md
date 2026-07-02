# Publication Operand Lineage

Status: `passed`

Evidence mode: `Static`

For touched watershed output fields, record units, normalization basis,
area/volume basis, source authority, authoritative-vs-diagnostic status, and
rejected aliases/formulas.

## W6 Lineage Scope

Static: W6 changes the public output writer handoff and the typed publication
frame's handling for missing publication operands. It does not change watershed
routing kernels, hillslope pass parsing, Arrow schema definitions, or output
column names. Lineage below covers the typed publication operands copied into
public watershed parquet outputs.

| Operand | Units | Basis | Source authority | Status | Rejected aliases / formulas |
| --- | --- | --- | --- | --- | --- |
| `year`, `simulation_year`, `sim_day_index`, `julian`, `month`, `day_of_month`, `water_year` | calendar/index fields | one typed publication record | existing watershed output schema plus `WatershedPublicationFrame` field contract | authoritative publication identity | Do not infer dates from output path, wall clock, job completion order, or fixture directory names. |
| `element_id`, `channel_id` | integer ids | first routed channel id in deterministic dispatch set | `WatershedNetworkFrame::build_typed_publication_frame` | authoritative routing identity | Do not derive from old `BoundarySymbol` spelling, row order in a writeback map, or unordered job completion. |
| `runoff_volume_m3` | `m^3` | sum of routed channel `runoff_volume_m3` across dispatch channel ids | typed `RoutedChannelState` produced by `execute_watershed_dispatch_with_frame` | authoritative routed runoff volume | Reject `runoff_mm * area_m2 / 1000` as the source of truth in the public typed writer; depth-derived volume is only used for schema columns that explicitly publish depth-to-volume transforms. |
| `peak_discharge_m3_s` | `m^3/s` | first deterministic routed channel peak | typed `RoutedChannelState::peak_discharge_m3_s` | authoritative routed peak | Reject hillslope contributor peak as the watershed outlet peak. |
| `sediment_yield_kg` | `kg` | sum of routed channel sediment yield | typed `RoutedChannelState::sediment_yield_kg` | authoritative routed sediment yield | Reject total detachment or total deposition aliases. |
| `total_detachment_kg`, `total_deposition_kg` | `kg` | sum over contributing hillslope pass inventory latest-event payloads | `PassInventoryEntry::latest_event_payload` carried into `HillslopeContribution` | authoritative pass-backed latest-event totals | Reject pollutant aliases and sediment-yield aliases. These are detachment/deposition totals, not routed outlet sediment yield. |
| `soluble_pollutant_kg`, `particulate_pollutant_kg` | `kg` | nullable typed publication scalar | no current routed/pass-backed pollutant source in W6 frame | unavailable, emitted as null | Do not synthesize pollutant process physics or map detachment/deposition into pollutant columns. |
| `channel_outflow_m3`, `channel_storage_m3`, `channel_baseflow_m3`, `channel_loss_m3` | `m^3` | nullable typed publication scalar | no current W6 channel-balance volume source for these fields | unavailable in public watershed CLI frames, emitted as null | Do not derive from old writeback-surface rows, impoundment outflow, routed runoff aliases, or `cbase`; do not force closure by adjusting any term. |
| `area_m2` | `m^2` | sum of contributing hillslope source slope geometry `fwidth * slplen`, converted from English square feet when declared by the source runfile | committed `pN.source.run` plus parsed `inputs.slope` files | normalization basis when every contributing hillslope has finite positive area; otherwise null | Reject fake unit area, fixture count, number of hillslopes, output row count, or generated output path as area. |
| Depth fields: `runoff_mm` | `mm` | `runoff_volume_m3 / area_m2 * 1000` only when `area_m2` is available | routed runoff volume plus committed source slope area | authoritative depth projection of routed volume | Reject `runoff_volume_m3 * 1000` and any path that uses a fake area. |
| Depth fields: `precipitation_mm`, `rain_melt_mm`, `deep_percolation_mm`, `lateral_flow_mm`, `qofe_mm`, `transpiration_mm`, `evaporation_soil_mm`, `evaporation_residue_mm`, `upstream_q_mm`, `subsurface_runon_mm`, `total_soil_water_mm`, `soil_water_total_mm`, `profile_depth_mm`, `profile_porosity_cap_mm`, `profile_fc_store_mm`, `profile_wp_store_mm`, `interception_mm`, `interception_storage_mm`, `frozen_water_mm`, `snow_water_mm`, `tile_mm`, `irrigation_mm`, `baseflow_mm`, `tsmf_fraction`, `qrain_mm`, `qsnow_mm` | `mm` or fraction by field | nullable typed publication frame | no current W6 routed/pass-backed source for these fields | unavailable, emitted as null | Reject adjacent aliases that have distinct meanings: `Runoff` depth is not `Q` volume; `Total-Soil Water` is not `SoilWaterTotal`; `frozwt` is not `Snow-Water`; `latqcc` is not `QOFE`. |
| `sediment_class_deposition_kg`, `sediment_volume_concentration_m3_m3`, `subsurface_runoff_volume_m3` | field units | nullable typed publication frame | no current W6 routed/pass-backed source for these fields | unavailable, emitted as null | Do not fill class deposition, concentration, or subsurface volume with zero unless a producer supplies an authoritative zero. |

## Schema-Preserving Column Formulas

The direct typed writer must preserve the existing public column formulas:

- pollutant total = `soluble_pollutant_kg + particulate_pollutant_kg` when both
  operands are present; otherwise null;
- sediment yield tonnes in loss tables = `sediment_yield_kg / 1000`;
- depth-to-volume columns = `depth_mm * area_m2 / 1000` when both operands are
  present; otherwise null;
- `Q` volume uses `q_diagnostic_mm` when present for row-seed edge callers and
  `runoff_mm` for direct typed publication frames;
- channel balance = `runoff_volume_m3 - channel_outflow_m3 - channel_loss_m3`
  only when all three explicit operands are present; otherwise null.

These are output-schema projections, not new process physics.
