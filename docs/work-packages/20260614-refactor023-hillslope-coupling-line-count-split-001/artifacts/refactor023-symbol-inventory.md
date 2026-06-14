# REFACTOR023 Symbol Inventory

Status: complete

## Static

Pre-refactor inventory from
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`:

- Structs: `FrostLayerWaterState`, `FrostFineLayerState`,
  `FrostLayerExchangeState`, `FrostFineShadowState`, `FrostDepthSummary`,
  `FrostSeasonalTemperatureCurve`.
- Public crate methods: `interval_overlap_duration`,
  `bounded_interval_overlap_duration`, `resolve_active_snow_coupling`,
  `validate_runtime_snow_state_domains`, `resolve_active_frost_coupling`,
  `compute_active_frost_coupling`.
- Frost helper cluster moved to `coupling/frost.rs`: helper methods
  from `frost_layer_soilf_sum` through `legacy_tmpadj_surface_temperature_c`.
- Frost entry cluster moved to `coupling/frost_entry.rs`:
  `resolve_active_frost_coupling` and `compute_active_frost_coupling`.

Post-refactor inventory:

- `coupling.rs`: module wiring, six shared frost structs, interval helpers,
  active snow coupling, runtime snow domain validation.
- `coupling/frost.rs`: frost fine-layer state, heat-flow, seasonal
  temperature, freeze/thaw, and `tmpadj` helpers.
- `coupling/frost_entry.rs`: active frost gate and active frost coupling
  orchestration.

Visibility note: only the 21 frost helpers directly called by the sibling
`frost_entry` module are `pub(super)`. The remaining moved helpers are private
`fn`. This is module-internal visibility under `support_helpers_mod::coupling`
and does not change the public crate surface.

## Ran

- `rg -n "^\\s*(pub\\(crate\\)\\s+)?fn\\s+|^\\s*struct\\s+|^\\s*impl\\s+Wb11HydrologyKernel" crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  - exit_code: 0
  - result: captured pre-refactor symbol inventory.
- `rg -n "^\\s*pub\\(crate\\)\\s+fn\\s+(interval_overlap_duration|bounded_interval_overlap_duration|resolve_active_snow_coupling|validate_runtime_snow_state_domains|resolve_active_frost_coupling|compute_active_frost_coupling)\\b" ...`
  - exit_code: 0
  - result: all six expected public crate methods found after refactor.
