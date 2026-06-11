# Contract Test Implementation Evidence

Status: executed-hold

Evidence mode: Static

## Added Contract-Derived Tests

- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
  - `fdhp01_contract_heat_flow_depth_can_exceed_retired_proxy_cap`
  - `fdhp01_contract_heat_flow_publishes_separate_surface_and_unfrozen_fluxes`
  - `fdhp01_contract_warm_heat_flow_thaws_prior_deep_frost`
  - `fdhp01_contract_frozen_water_exchange_hard_fails_on_liquid_overdraw`
  - `fdhp01_layered_store_contract_rejects_scalar_frdp_theta_frozen_water_authority`
  - `fdhp01_layered_store_contract_freeze_updates_layer_depth_and_frzw_sum`
- `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - extended WAT parquet metadata test to require `frdp` in `mm` and dataset
    version `1.4`.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
  - `fdhp01_wb13_publication_converts_runtime_frdp_to_wat_mm`
  - `fdhp01_wb13_publication_rejects_frdp_beyond_profile_depth`
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13.rs`
  - extended `hphys0203_wb13_soil_water_total_preserves_watcon_alias` so
    `frozwt` follows `frost.runtime_frwatc_frozen_water_after_m` instead of
    `frost.runtime_ws_frz`.
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13_guard.rs`
  - `fdhp01_wb13_frozwt_guard_rejects_missing_exchange_store_symbol`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
  - extended schema metadata unit test to require default dataset version
    `1.4`.
- `tests/integration/sim_contract_boundary_unit_registry.rs`
  - extended canonical hydrology/publication registry test to require
    `hillslope_wat.frdp`.

## Expected Pre-Implementation Failures

The tests are intended to fail against the existing proxy implementation before
production edits:

- deep physical frost state above `0.20 m` is rejected;
- severe cold cannot progress `Dfrost` beyond `0.20 m`;
- separate lower-front `Quf` is not published;
- impossible frozen-water storage can be created by clamping liquid soil water;
- WAT output lacks `frdp`;
- WAT output does not version or bound `frdp`;
- WAT `frozwt` can be published from a depth-derived proxy or missing exchange
  store;
- the active frost exchange store can be a scalar `frdp * theta` proxy instead
  of a layer-state legacy `Σ soilf(i)` store;
- the unit registry lacks `hillslope_wat.frdp`.

## Post-Implementation Proof Points

Ran:

- CLIM06 FDHP01 tests pass for depth beyond the retired cap, separate
  `Qsrf`/`Quf`, warm-thaw reduction of prior deep frost, liquid-water thaw
  credit, and hard-fail frozen-water overdraw with `DomainViolation`.
- Runner unit tests pass for runtime `frdp` to WAT `mm` conversion and
  profile-bound rejection, plus fail-closed `frozwt` exchange-store symbol
  enforcement.
- Runner WB13 fixture passes for consuming
  `frost.runtime_frwatc_frozen_water_after_m` rather than `runtime_ws_frz`.
- CLIM06 layered-store fixtures pass for rejecting scalar frozen-water
  equivalence and proving per-layer frozen depth/`frzw` updates with freezing.
- WAT metadata tests pass for required `frdp` field metadata and dataset
  version `1.4`.
