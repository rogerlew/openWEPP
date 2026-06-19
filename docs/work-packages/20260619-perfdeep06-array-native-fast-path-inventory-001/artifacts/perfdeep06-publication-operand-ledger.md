# PERFDEEP06 Publication Operand Ledger

Status: complete 2026-06-19.
Evidence class: Static.

## Current Publication Shape

Output row schemas are already typed (`HillslopeWatRow`, `HillslopePassRow`).
The remaining hot-path risk is construction: WB13 and HBP still read
`HillslopeWritebackSurface` through string symbols in
`02_output_and_climate_helpers.rs`.

## Operand Ledger

| Output operand | Units / basis | Current source | Proposed typed source | Wrong aliases to reject | Required fixture/evidence |
|---|---|---|---|---|---|
| HBP `peakro` / PASS `peakro` | `m^3/s`, diagnostic peak runoff | `optional_non_negative_runtime_scalar(runtime_surface, "peakro")` | `publication.peak_runoff_m3_s` from erosion/storage phase output | daily `Q`, `QOFE`, or runoff volume | HBP byte identity and PASS Arrow equality with nonzero peak runoff. |
| HBP `watdur` | seconds, event duration | runtime symbol `watdur` | `publication.runoff_duration_s` | day length, hyetograph duration, irrigation duration | HBP byte identity with nonzero duration. |
| HBP/PASS `tdet`, `tdep` | kg, erosion diagnostics | `total_detachment_kg`, `total_deposition_kg` | `publication.total_detachment_kg`, `publication.total_deposition_kg` | sediment concentration, class totals, zero defaults on erosion-active run | HBP byte identity and PASS Arrow equality on erosion-active fixture. |
| HBP/PASS `sedcon_1..5` | `kg/m^3`, particle classes | HBP reads `sediment_concentration_kg_m3_0001`; PASS currently emits `[0.0; 5]` | `[f64; 5]` sediment concentration projection, with current HBP class-1 behavior preserved until sediment package changes it | particle flow fraction, detachment/deposition mass, all-zero classes when class 1 is nonzero | HBP byte identity; PASS fixture that distinguishes class 1 from zeros before changing PASS behavior. |
| WAT `P` | mm over row area | `prcp` meters times 1000 | `publication.precipitation_mm` from borrowed climate/day forcing | rainfall-only, RM, snowmelt | WAT Arrow equality and precipitation/rainmelt non-alias test. |
| WAT `RM` | mm liquid input | `snow.post_winter_rain_m + snow.routed_melt_m + Irr` | `publication.liquid_input.rm_mm` | `P`, `Irr`, snow melt alone | fixture where rain, snowmelt, and irrigation differ. |
| WAT `Q` | mm, daily runoff over effective length | `Q` or routed runoff geometry conversion | `publication.runoff.q_mm` | `QOFE`, `runvol`, physical `Q` in per-OFE routed mode | per-OFE fixture preserving current geometry formula. |
| WAT/PASS `QOFE` / `runvol` | WAT mm; PASS `m^3` | `QOFE`; PASS outlet uses `qofe * outlet row area / 1000` | `publication.runoff.qofe_mm`; `pass.runvol_m3` from outlet row area | `Q * area`, upstream area, publication-area sum | Existing anti-alias fixture in `per_ofe_state.rs` remains required. |
| WAT `Ep`, `Es`, `Er` | mm | preferred flux/state `Ep`, `Es`, `Er` | `publication.evaporation.{ep,es,er}_mm` | ET total, raw negative-tolerance `Es`, seed branch flag | WAT equality; fixture with separate Ep/Es/Er. |
| WAT `Dp` | mm | `D + frost.runtime_watbtm_m`, canonicalized | `publication.subsurface.dp_mm` | base `D` alone, `watpdg`, `latqcc` | closure fixture with nonzero frost bottom water. |
| WAT `UpStrmQ`, `SubRIn` | mm | `TransferInput`/runtime symbols | `publication.transfer.upstream_surface_mm`, `publication.transfer.upstream_lateral_mm` | current lane output arrays, unscaled area ratio | MOFE identity fixture with non-1 area ratio. |
| WAT/PASS `latqcc` / `sbrunv` | WAT mm; PASS `m^3` | preferred flux/state `q`, PASS `latqcc * outlet area / 1000` | `publication.subsurface.latqcc_mm` | `Qd`, `Qdd`, lateral output arrays | existing `q` vs state/flux preference tests plus PASS volume check. |
| WAT `Tile` | mm | preferred flux/state `Qdd` | `publication.subsurface.tile_mm` | `Qd`, `q` | fixture enforcing `Qd = latqcc + Tile`. |
| WAT `Total-Soil` and `SoilWaterTotal` | mm | `wb11_soil_water * 1000`; SoilWaterTotal alias to same value | `publication.storage.total_soil_mm` and alias | `watcon` stale logical, frozen water, profile depth | existing hydout-equivalent closure test remains required. |
| WAT `frozwt`, `frdp`, `Snow-Water` | mm | `frost.runtime_frwatc_frozen_water_after_m`, `frost.runtime_frdp_m`, `snow.runtime_swe` | `publication.storage.{frozwt,frdp,snow_water}_mm` | snow depth, frozen delta, profile depth | snow/frost active fixture with distinct values. |
| WAT profile optional fields | mm | `wb13_profile_depth_mm`, porosity cap, layer-derived FC, WP | `publication.profile` from `SoilLayerColumns` | `solthk(nsl)` alone for FC, porosity for FC, layer 1 only | anti-tautology fixture where profile depth, porosity, FC, WP differ. |
| WAT `Interception` | mm | preferred flux/state `I` | `publication.interception_mm` | interception storage, rainfall input | fixture with nonzero I and storage. |
| WAT `InterceptionStorage` | mm | currently `None` | future optional `publication.interception_storage_mm` | daily I flux | preserve `None` until producer-authoritative storage is ported. |
| Manifest WB13 provenance | metadata, not numeric output | `HillslopeWb13PublicationProvenance` from rows/runtime surface | typed provenance projection | runtime logical row counts as authority | manifest JSON comparison after typed projection. |

## Identity and Metadata Projection

Typed publication cannot only preserve numeric operands. PERFDEEP07/08 must
also keep row identity, calendar projection, schema metadata, and producer
metadata out of the hot symbol path while preserving exact output meaning.

| Metadata / identity field | Current source | Proposed typed source | Required fixture/evidence |
|---|---|---|---|
| WAT `wepp_id`, `ofe_id` | runner publication helpers and lane context | `HillslopeDayContext` / publication projection identity fields | WAT Arrow equality across multi-OFE H2637; guard `wepp_id > 0`, `ofe_id > 0`. |
| PASS `wepp_id` | runner pass-row builder argument | outlet publication context | PASS Arrow equality; multi-hillslope fixture before watershed fan-in changes. |
| `sim_day_index` | scheduler day counter, converted to `i32` for output | `HillslopeDayContext.sim_day_index` | WAT/PASS Arrow equality and fail-closed range guard. |
| `julian`, `month`, `day_of_month`, `water_year` | `CalendarDayProjection` and WB13 calendar helper | `HillslopeDayContext.calendar` | Calendar anti-alias fixture across water-year boundary; WAT/PASS equality. |
| schema version and dataset metadata | output schema crates | unchanged schema crate authority, not frame state | Parquet/Arrow schema metadata tests remain required after typed projection. |
| field units/descriptions | output schema crates and unit registry | unchanged schema crate authority | Unit metadata validation remains required; direct frame must not define duplicate output schema authority. |
| producer/provenance metadata | PASS/WAT writers and WB13 manifest provenance | writer metadata plus typed publication provenance | manifest JSON comparison and PASS/WAT metadata inspection. |

## Gate

PASS. PERFDEEP07 must not delete logical publication sources until HBP byte
identity, WAT byte/Arrow identity, PASS Arrow identity, metadata/provenance
equivalence, and the anti-alias fixtures above are green from the typed
projection.
