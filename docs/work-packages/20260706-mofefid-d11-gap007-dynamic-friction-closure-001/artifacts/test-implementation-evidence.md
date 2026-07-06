# Test Implementation Evidence

Status: **PASS**.

## Implemented Tests

Static:

- Added `laned_shadow_consumes_live_dynamic_friction_operands` in
  `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`.
  It guards the frame-aware stream callback, live `I`/`LAI`/`h_c` operand
  sourcing, vegetation-active `canhgt` fail-closed logic, and removal of the
  all-lane `I = 0` placeholder.
- Added `dynamic_canopy_operands_reach_cell_parameters` in
  `crates/openwepp-runner/src/hillslope/laned_shadow.rs`. It proves dynamic
  `LAI` and `h_c` are written into `CellParameters` with the static routing
  coefficients.
- Added `laned_shadow_dynamic_operands_reject_missing_canhgt_when_lai_positive`,
  `laned_shadow_dynamic_operands_reject_zero_canhgt_when_lai_positive`, and
  `laned_shadow_dynamic_operands_preserve_hourly_rainfall_when_valid` in
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`.
  These executable tests cover the vegetation-active fail-closed rules and
  preservation of live WB14 hourly rainfall.
- Added `dynamic_rainfall_intensity_changes_routed_cascade_result` in
  `crates/openwepp-runner/src/hillslope/laned_shadow.rs`. It routes the actual
  `LanedShadowCollector::route_buffered_day` path twice and proves nonzero
  dynamic rainfall intensity changes the cascade outlet result.

## Ran

- `cargo test -q -p openwepp-runner laned_shadow` -> PASS (`6` tests passed).
- `cargo test -q -p openwepp-runner laned_shadow_dynamic_operands` -> PASS
  (`3` tests passed).
- `cargo test -q -p openwepp-runner dynamic_rainfall_intensity_changes_routed_cascade_result`
  -> PASS (`1` test passed).
- `cargo test -q --test laned_shadow_h2637 h2637_legacy_shadow_fails_closed_without_routing_coefficients`
  -> PASS (`1` test passed, `35.86 s` local; `26.47 s` heavy-gate runner).
