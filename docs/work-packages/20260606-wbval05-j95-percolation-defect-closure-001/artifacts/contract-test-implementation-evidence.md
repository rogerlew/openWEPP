# Contract-Test Implementation Evidence

Status: complete

Evidence mode: static+ran

Purpose: record contract-derived tests for the J-95 percolation defect and
their before/after results.

Static:

- Added contract-derived regression
  `wbval05_wb18_percolation_consumes_published_zero_infiltration_without_snow_recompute`
  in `crates/openwepp-hillslope-orchestrator/src/tests.rs`.
- The test constructs the WBVAL05 class: `management.initial.params.tillay2_m`
  present, published `wb12_infiltration=0`, and stale negative
  `snow.runtime_swe`. It asserts WB18 percolation succeeds by consuming the
  published zero infiltration lineage rather than revalidating snow state.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator
  wbval05_wb18_percolation_consumes_published_zero_infiltration_without_snow_recompute
  -- --nocapture` passed.
