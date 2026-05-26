# SIMIMPL29 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL29 contract-derived tests were implemented in integration suites:
  - `tests/integration/clim05_snow_runtime_kernel_contract.rs`
  - `tests/integration/parser_runtime_seam_integration.rs`
- Added/updated assertions cover:
  - publication of all required hourly snow kernel-state symbol families,
  - runtime carry-state seeding/presence (`snow.runtime_depth_m`,
    `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count`),
  - typed hard-fail behavior when required active-hourly snow symbol inputs are
    missing.

## Ran
- `cargo test -p openwepp --test clim05_snow_runtime_kernel_contract`
- `cargo test -p openwepp --test parser_runtime_seam_integration`
- `rg -n "simimpl29_contract_conformance_rejects_missing_hourly_snow_kernel_symbol|snow\.hourly\.depth_before_m|snow\.hourly\.melt_m|snow\.runtime_depth_m|snow\.runtime_density_kg_m3|snow\.runtime_settle_day_count" tests/integration/clim05_snow_runtime_kernel_contract.rs tests/integration/parser_runtime_seam_integration.rs`
