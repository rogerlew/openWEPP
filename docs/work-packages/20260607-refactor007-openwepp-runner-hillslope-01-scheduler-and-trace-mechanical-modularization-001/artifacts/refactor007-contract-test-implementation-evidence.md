# REFACTOR007 refactor007 contract test implementation evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-06-08

## Scope
Updated layout-coupled source-residency checks in the following contracts to
scan the full hillslope module tree (`crates/openwepp-runner/src/hillslope/**`) via
recursive helpers:

- `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs`
- `tests/integration/hphys0291_snow_publication_lifecycle_contract.rs`
- `tests/integration/hphys0293_winter_melt_timing_contract.rs`
- `tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`
- `tests/integration/hphys0295_cumulative_storage_budget_contract.rs`
- `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`
- `tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`
- `tests/integration/hphys0305_paired_melt_term_state_contract.rs`
- `tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`

## Static
- The runner source scan now collects all `.rs` files recursively, so assertions are
  no longer brittle to module residency shifts.

## Ran
- `cargo test -p openwepp-runner --tests`: pass.
- `cargo test --workspace`: pass.
