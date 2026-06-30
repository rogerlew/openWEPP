# Sub-Computation Map

Evidence mode: Static/Ran.

## Implemented In This Package

| Sub-computation | Typed projection | Surface writer status | Shadow evidence |
|---|---|---|---|
| WB18/WB19 lane substep controls | `project_typed_wb11_lane_substeps` | Existing `seed_wb11_lane_substep_controls` writes the typed projection to the surface, then calls the MOFE hourly carry surface adapter. | `typed_day_zero_wb11_lane_substeps_match_surface_seed` compares typed substeps and carry activation against surface seed outputs. |
| Rainfall/hyetograph normalization | `project_typed_wb11_hyetograph` with typed intervals and optional synthesized event | Existing `seed_wb11_hyetograph_inputs` reads current-day point authority, calls the typed projection, and writes normalized `ninten`/`nbrkpt` plus synthesized points when needed. | `typed_day_zero_wb11_hyetograph_breakpoint_seed_matches_surface_seed`, `typed_day_zero_wb11_hyetograph_zero_event_matches_surface_seed`, and `typed_day_zero_wb11_hyetograph_total_exceeds_prcp_matches_surface_seed`. |
| WB11 initial layer water stores | `project_typed_wb11_initial_storage` with typed layer inputs and totals | Existing `seed_initial_wb11_storage_if_needed` reads layer inputs from the surface, calls the typed projection, and writes projected layer stores/totals back. | `typed_day_zero_wb11_initial_storage_projection_matches_surface_seed` compares typed saturation, layer theta/fc/ul, and WB11 totals against surface seed outputs by `to_bits()`. |
| Fine-frost frozen-depth refresh | `project_typed_wb11_frozen_depth_refresh` with typed layer depth and optional fine-layer frozen-depth vectors | Existing `refresh_wb18_frozen_depth_from_fine_frost_state` reads the surface into typed layer inputs, calls the typed projection, and writes `wb18_perc_frozen_depth_*`. | `typed_day_zero_wb11_scalar_frost_depth_refresh_matches_surface_seed` and `typed_day_zero_wb11_fine_frost_depth_refresh_matches_surface_seed`. |
| Residue interception and `Ws` defaults | `project_typed_wb11_optional_defaults` | Existing `seed_wb11_optional_default_symbols` writes typed defaults only when the symbols are absent. | `typed_day_zero_wb11_optional_defaults_match_surface_seed`. |
| WB12 reconciliation defaults | `project_typed_wb12_reconciliation_seed` | Existing `seed_wb12_reconciliation_runtime_inputs` writes typed projection values to state/flux surfaces. | `typed_day_zero_wb12_reconciliation_seed_matches_surface_seed` compares typed rainfall/storage/precip/carry/forward-solver values against surface seed outputs. |
| ET-demand seed | `project_typed_wb11_priestley_taylor_et_demand` and `project_typed_wb11_evappm_et_demand` | Existing `compute_wb11_et_demand_seed` selects the typed branch mode, reads the surface into typed ET inputs, calls the typed projection, and keeps `publish_wb11_et_demand_seed` as the writer adapter. | `typed_day_zero_wb11_priestley_taylor_et_seed_matches_surface_seed` and `typed_day_zero_wb11_evappm_et_seed_matches_surface_seed`. |
| `efflen` and default `m` | `project_typed_wb11_efflen_and_m` | Existing `seed_wb11_efflen_and_m_if_missing` writes typed defaults only when each symbol is absent. | `typed_day_zero_wb11_efflen_and_m_match_surface_seed`. |
| WB16 `ealpha` compatibility-default decision | `project_typed_wb16_ealpha_compatibility` | Existing `seed_wb16_ealpha_compatibility` still calls the runtime producer, then uses the typed projection for the default/flag decision. | `typed_day_zero_wb16_ealpha_default_compatibility_matches_surface_seed`. |

## Still Surface-Authoritative

These sub-computations remain in the symbol-map seed path and block full Phase 1
seed identity:

- typed static input projection from `ParsedHillslopeRunInputs` and sidecars;
- lateral/drain validation inputs;
- full WB16 `ealpha` producer inputs;
- MOFE03/Wave-2 core, route topology ingress, case state, particle classes, and
  flag authority;
- full consumer-read-set carrier assembly for lane constructor,
  `DirectProductionDayInputBuilder`, coupling metadata, Wave-2 flag, and winter
  hourly geometry.

## Cutover Boundary

No consumer cutover happened. The typed projections currently feed the existing
surface writer adapter only. That is an intentional Phase 1 transition state:
the typed computation is real for the implemented sub-computations, but the
production seed authority as a whole is not yet typed or single-authority.
