# CLIM17 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: Static  
Date: 2026-05-28

## Contract-derived test vectors implemented

1. Fixture added:
   - `tests/fixtures/infile/climate/wc1_unpalatable_rind_breakpoint_nbrkpt_0.cli`
   - Provenance: sourced from `/wc1/runs/un/unpalatable-rind/wepp/runs/p1.cli`
     (`ibrkpt=1`, `nbrkpt=0` day shape).

2. Parser contract vector:
   - `tests/integration/infile_climate_parser_contract.rs`
   - Added test:
     `strict_mode_accepts_curated_wc1_breakpoint_fixture_with_zero_points`.

3. Runtime adapter vectors:
   - `crates/openwepp-climate-runtime-adapter/src/lib.rs` test module.
   - Added tests:
     - `runtime_request_accepts_breakpoint_zero_cardinality_dry_day`
     - `runtime_request_rejects_malformed_positive_cardinality_with_empty_series`

4. Hillslope seam vector:
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
   - Added test:
     `breakpoint_runtime_surface_accepts_curated_wc1_zero_breakpoint_dry_day`.

5. Watershed seam vector:
   - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` tests.
   - Added test:
     `breakpoint_runtime_surface_accepts_curated_wc1_zero_breakpoint_dry_day`.

6. CLIM07 closure vector:
   - `tests/integration/clim07_climate_comparator_and_closure_contract.rs`
   - Added test:
     `clim07_breakpoint_zero_cardinality_vector_projects_dry_day_surface`.

## Static
- Contract-derived vectors for parser + runtime + hillslope seam + watershed
  seam + comparator contract are implemented.

## Ran
- not-run
