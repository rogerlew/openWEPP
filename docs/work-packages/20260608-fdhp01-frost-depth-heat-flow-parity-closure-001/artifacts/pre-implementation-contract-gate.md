# Pre-Implementation Contract Gate

Status: complete

Evidence mode: Ran

Date: 2026-06-11

## Commands

- `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_contract -- --nocapture`
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests cli04_fixture_run_emits_valid_wat_parquet_with_required_metadata_keys -- --nocapture`
- `cargo test --test sim_contract_boundary_unit_registry canonical_registry_contains_hydrology_et_percolation_publication_units -- --nocapture`

## Result

Expected failing gate captured before production edits.

- `fdhp01_contract_heat_flow_depth_can_exceed_retired_proxy_cap` failed because
  `Dfrost` did not exceed the retired `0.20 m` proxy cap.
- `fdhp01_contract_warm_heat_flow_thaws_prior_deep_frost` failed because prior
  physical frost depth above `0.20 m` was rejected by the existing guard.
- CLI04 WAT metadata test failed because `frdp` field metadata did not exist.
- Unit registry test failed because `hillslope_wat.frdp` alias did not resolve.
