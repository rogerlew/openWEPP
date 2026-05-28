# CLIM17 Implementation and Test Evidence

Status: complete  
Evidence mode: Static + Ran  
Date: 2026-05-28

## Static

Production runtime change implemented:

- File: `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- Function: `adapt_breakpoint`
- Behavior change:
  - Accepts `ibrkpt=1` records with `nbrkpt=0` and empty breakpoint vectors as
    valid dry-day forcing.
  - Emits deterministic zero forcing payload:
    `stmstr=0`, `prcp=0`, `stmdur=0`, `mxint=0`, empty `timem`/`intsty`.
  - Retains `CLIM-RUNTIME-E-008` for malformed positive-cardinality empty
    breakpoint series.

## Ran

Targeted CLIM17 vectors executed and passed:

1. `cargo test -p openwepp-climate-runtime-adapter runtime_request_accepts_breakpoint_zero_cardinality_dry_day`
2. `cargo test -p openwepp-climate-runtime-adapter runtime_request_rejects_malformed_positive_cardinality_with_empty_series`
3. `cargo test --test infile_climate_parser_contract strict_mode_accepts_curated_wc1_breakpoint_fixture_with_zero_points`
4. `cargo test -p openwepp-hillslope-orchestrator breakpoint_runtime_surface_accepts_curated_wc1_zero_breakpoint_dry_day`
5. `cargo test -p openwepp-watershed-orchestrator breakpoint_runtime_surface_accepts_curated_wc1_zero_breakpoint_dry_day`
6. `cargo test --test clim07_climate_comparator_and_closure_contract clim07_breakpoint_zero_cardinality_vector_projects_dry_day_surface`

All targeted vectors passed with no failures.
