# PERFDEEP01 Publication Operand Ledger

Evidence: Static (source inventory) + Ran (Stage-0 tests).

Scope:

- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- Entry points: `build_simulation_owned_wb13_row_for_ofe`, `build_hbp_output`, and local helper calls.

## WB13/WAT/PASS Runtime-Surface Reads

Complete operand set used to assemble WB13/WAT/PASS publication surfaces:

- `prcp`, `tmax`, `tmin`
- `wb13_profile_depth_mm`, `wb13_profile_porosity_cap_mm`, `wb13_profile_wp_store_mm`, `wb13_profile_fc_tail_mm`
- `wb11_nsl` (fallback `nsl`)
- `wb19_thetfc_0001..n` (fallback `thetfc_0001..n`)
- `wb19_dg_0001..n` (fallback `dg_0001..n`)
- `wb11_soil_water`
- `frost.runtime_frwatc_frozen_water_after_m`, `frost.runtime_frdp_m`, `frost.runtime_watbtm_m`, `frost.runtime_watpdg_m`
- `snow.runtime_swe`
- `Irr`
- `snow.routed_melt_m`, `snow.post_winter_rain_m`
- `I`, `Q`, `Ep`, `Es`, `Er`, `D`, `q`, `Qdd`, `Qd`
- `SubRIn`
- `efflen` (per-OFE publication branch)
- `wb11_et_seed_branch_evappm` (branch flag)

## HBP Runtime-Surface Reads

`build_hbp_output` reads the HBP scalar edge fields (optional, default 0.0, non-negative validated):

- `peakro`
- `watdur`
- `total_detachment_kg`
- `total_deposition_kg`
- `sediment_concentration_kg_m3_0001`

## Manifest/Provenance Reads

No direct runtime-surface symbol reads in this file's manifest/provenance builders.
Manifest payload is derived from already-projected run metadata and climate span.

## Surface Preference Semantics

- State-only: profile/storage keys (`wb13_*`, `wb11_*`, frost/snow state keys, layer-family symbols, `efflen`)
- Flux-only: `snow.routed_melt_m`, `snow.post_winter_rain_m`
- Flux-preferred fallback-to-state: `I`, `Q`, `Ep`, `Es`, `Er`, `D`, `q`, `Qdd`, `Qd`

## Dynamic Family Mapping

- Layer families are keyed by `nsl` (`wb11_nsl` or `nsl` fallback):
  - `wb19_thetfc_{layer:04}` else `thetfc_{layer:04}`
  - `wb19_dg_{layer:04}` else `dg_{layer:04}`
- Aggregation used in publication path:
  - `profile_fc_store_m = Σ(thetfc[layer] * dg[layer])`
  - `profile_fc_store_mm = profile_fc_store_m * 1000 + wb13_profile_fc_tail_mm`

## Stage-0 Frame Mapping Requirement Closure

Every symbol above is mapped either:

- directly to a `HillslopeDayFrame` dense slot (`state_slots` / `flux_slots` by `SymbolRegistry` id), or
- to a Stage-0 typed capture field (`HillslopeDayFrameIoEdgeScalars` for HBP edge scalars), or
- to a Stage-0 dynamic family array (`MOFE hourly carry arrays in frame schema).

This closes PERFDEEP01 publication-operand lineage requirement: no known publication operand remains unmapped from Stage-0 frame representation.
