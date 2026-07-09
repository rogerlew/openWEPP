# Characterization

Status: `COMPLETE`

Target:
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`

Focused live test inventory:

- Existing integration coverage in
  `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` binds WS12
  active impoundment runtime publication to the 15-function min-controller path
  and preserves `WKERNEL-WS10-IMPOUNDMENT-E-002`/`E-003` guard classes.
- Existing parser coverage in
  `tests/integration/infile_watershed_impoundment_parser_contract.rs` covers
  active impoundment payload parsing for drop spillway, culvert, emergency,
  filter, riser, and typed domain errors.
- This package added module-local `#[cfg(test)]` characterization because the
  high-CRAP functions are private numerical helpers and active-projection
  branch guards. The module-local placement is deliberate: it reaches private
  helper surfaces without exposing new production API.

Added characterization cases:

- `inactive_impoundment_projection_preserves_defaults`
- `active_impoundment_projection_covers_all_function_families`
- `drop_spillway_ids2_and_ids3_project_function_families`
- `emergency_open_channel_and_filter_modes_are_projected`
- `riser_sampling_regression_and_qs_branches_are_characterized`
- `quartic_fit_and_solver_cover_success_and_failure_modes`
- `discharge_helpers_cover_thresholds_interpolation_and_errors`
- `projection_guards_preserve_error_classes`
- `projection_guards_cover_contract_boundary_failures`
- `family_projection_guards_cover_invalid_active_payloads`
- `discharge_guards_cover_invalid_payloads`
- `piecewise_helpers_cover_remaining_intervals`
- `power_law_projection_covers_valid_and_invalid_domains`

Behavior oracle:

- Valid active branch payloads produce finite WS12 runtime coefficient families
  and preserve expected coefficient values for the active drop-spillway and
  projection paths.
- Threshold, interpolation, riser regression, quartic-fit, and discharge helper
  branches preserve existing finite/non-negative behavior and zero-discharge
  below-threshold behavior.
- Invalid active payloads preserve typed
  `WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain` or
  `ImpoundmentSymbolNonFinite` classes, including expected symbol names and
  rule text for package-owned guard paths.

Ran:

- `cargo nextest run -p openwepp-watershed-orchestrator chaninp`
  - Exit code: `0`
  - Current result after final clippy-fix patch: `13 tests run: 13 passed, 9
    skipped`

`SC-IMPOUND-001` mapping:

- Active-projection vector: covered by
  `active_impoundment_projection_covers_all_function_families`,
  `drop_spillway_ids2_and_ids3_project_function_families`,
  `emergency_open_channel_and_filter_modes_are_projected`, and existing
  integration test
  `typed_frame_active_impoundment_matches_drop_spillway_min_controller_composition`.
- Missing/non-finite/domain vectors: covered by the new package-local guard
  tests plus existing integration tests
  `typed_frame_impoundment_projection_preserves_non_finite_guard_class` and
  `typed_frame_impoundment_projection_preserves_domain_guard_class`.
- Surrogate-deauthorization and WS12 continuity vectors were not changed by
  this CQR package; existing WS12 runtime integration coverage remains the
  behavior authority.
