# REFACTOR004 Modularization Plan Report

Status: complete
Evidence mode: static
Date: 2026-05-25

## Static
Objective implemented as mechanical modularization with public API
preservation.

Module boundary plan executed:
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/mod.rs`
  - module root that includes section files in deterministic order.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
  - imports, error taxonomy, status/error code wiring, and top-level runtime
    request/surface types.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
  - PL management runtime projection surfaces and management merge builder.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - strict soil and slope parser-to-runtime projection builders.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`
  - climate runtime request construction plus day-seeding/build wrappers.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
  - snow/frost runtime projection and irrigation depletion/fixed-date seeding.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`
  - slope/profile validators, PL projection helpers, and symbol constructors.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  - SIMIMPL28 hourly winter forcing synthesis helpers/constants.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/07_series_helpers.rs`
  - climate-series surface helpers and mapping utilities.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - module-level runtime-input integration/unit tests (mechanically moved).

Mechanical intent constraints satisfied:
- no intentional runtime semantic changes,
- no new fallback behavior,
- typed guard/error surfaces preserved,
- `pub mod runtime_inputs;` in `lib.rs` preserved (module path unchanged).

## Ran
- not run
