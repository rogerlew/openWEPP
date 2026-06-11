# Contract Test Implementation Evidence

Status: complete

Evidence mode: Static

## Added Contract-Derived Tests

- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`
  - `fdhp01_contract_heat_flow_depth_can_exceed_retired_proxy_cap`
  - `fdhp01_contract_heat_flow_publishes_separate_surface_and_unfrozen_fluxes`
  - `fdhp01_contract_warm_heat_flow_thaws_prior_deep_frost`
  - `fdhp01_contract_frozen_water_exchange_hard_fails_on_liquid_overdraw`
- `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - extended WAT parquet metadata test to require `frdp` in `mm` and dataset
    version `1.4`.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
  - `fdhp01_wb13_publication_converts_runtime_frdp_to_wat_mm`
  - `fdhp01_wb13_publication_rejects_frdp_beyond_profile_depth`
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
- the unit registry lacks `hillslope_wat.frdp`.

## Post-Implementation Proof Points

Ran:

- CLIM06 FDHP01 tests pass for depth beyond the retired cap, separate
  `Qsrf`/`Quf`, warm-thaw reduction of prior deep frost, liquid-water thaw
  credit, and hard-fail frozen-water overdraw with `DomainViolation`.
- Runner unit tests pass for runtime `frdp` to WAT `mm` conversion and
  profile-bound rejection.
- WAT metadata tests pass for required `frdp` field metadata and dataset
  version `1.4`.
