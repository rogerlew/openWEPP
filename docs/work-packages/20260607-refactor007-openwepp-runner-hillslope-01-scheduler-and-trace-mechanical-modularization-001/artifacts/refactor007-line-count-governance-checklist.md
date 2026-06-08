# REFACTOR007 refactor007 line count governance checklist

Status: complete  
Evidence mode: static  
Date: 2026-06-08

## Static
- Pre-state snapshot (selected touched `.rs` files from `HEAD`):
  - `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs`: `3156` lines
  - `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`: `60` lines
  - `tests/integration/hphys0290_post_winter_rain_publication_contract.rs`: `84` lines
  - `tests/integration/hphys0291_snow_publication_lifecycle_contract.rs`: `337` lines
  - `tests/integration/hphys0293_winter_melt_timing_contract.rs`: `142` lines
  - `tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`: `121` lines
  - `tests/integration/hphys0295_cumulative_storage_budget_contract.rs`: `116` lines
  - `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`: `128` lines
  - `tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`: `182` lines
  - `tests/integration/hphys0305_paired_melt_term_state_contract.rs`: `153` lines
  - `tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`: `297` lines
- Post-state snapshot (post-refactor):
  - `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs`: `13` lines
  - `crates/openwepp-runner/src/hillslope/scheduler_trace/mod.rs`: `6` lines
  - `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`: `1792` lines
  - `crates/openwepp-runner/src/hillslope/scheduler_trace/hphys_trace.rs`: `1056` lines
  - `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs`: `308` lines
  - `tests/integration/hphys0289_wb13_rm_snowwater_publication_contract.rs`: `66` lines
  - `tests/integration/hphys0290_post_winter_rain_publication_contract.rs`: `90` lines
  - `tests/integration/hphys0291_snow_publication_lifecycle_contract.rs`: `342` lines
  - `tests/integration/hphys0293_winter_melt_timing_contract.rs`: `147` lines
  - `tests/integration/hphys0294_post_ingress_storage_retention_contract.rs`: `127` lines
  - `tests/integration/hphys0295_cumulative_storage_budget_contract.rs`: `122` lines
  - `tests/integration/hphys0296_snow_rm_acceptance_authority_contract.rs`: `134` lines
  - `tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`: `187` lines
  - `tests/integration/hphys0305_paired_melt_term_state_contract.rs`: `159` lines
  - `tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`: `302` lines

- Files >= 2000 lines pre-refactor:
  - `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs` (3156)
- Files >= 3000 lines pre-refactor:
  - `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs` (3156)
- Files >= 2000 lines post-refactor:
  - None
- Files >= 3000 lines post-refactor:
  - None

## Line-count governance disposition
- Requirement satisfied: target module count reduced below 3000 lines and split into
  focused files.
- Decomposition rationale: preserve execution/trace continuity while removing the
  single 3156-line monolith and consolidating scheduler/trace internals into three
  bounded seams.
- Exception owner/sunset: no exception required.
