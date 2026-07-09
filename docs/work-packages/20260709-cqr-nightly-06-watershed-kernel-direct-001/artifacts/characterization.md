# Characterization

Evidence label: Static/Ran.

Status: `COMPLETE`

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`

Static:

- Existing watershed runtime integration coverage already exercised direct
  channel and impoundment execution through
  `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`.
- Characterization gaps for private sediment helpers were package-local:
  channel dependency sediment payload projection, WS20 profile/crfrac guard
  paths, sediment accumulator partitioning, disabled/enabled WS20 helper
  routing, terminal transport-capacity guards, and hourly sediment-rate timing.

Ran:

- `cargo nextest run -p openwepp-watershed-orchestrator` - pass,
  `68 tests run: 68 passed` after final characterization split.
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` -
  pass, `18 tests run: 18 passed`.

Added characterization tests:

- `direct_ws20_profile_projects_segment_points_and_guards_domains`
- `direct_ws20_crfrac_normalizes_selected_classes_and_guards_payloads`
- `read_direct_channel_sediment_payload_covers_zero_valid_and_guard_paths`
- `read_direct_hillslope_sediment_payload_prefers_hourly_mass_authority`
- `direct_sediment_accumulator_projects_ingress_and_active_classes`
- `direct_sediment_routing_helpers_cover_disabled_and_enabled_paths`
- `direct_sediment_capacity_helpers_cover_terminal_and_guard_paths`
- `direct_range_and_ipeak_branch_helpers_cover_guard_cases`
- `direct_ws11_runon_cases_are_characterized`
- `direct_ws11_runon_guards_fail_closed`
- `direct_channel_peak_helpers_cover_branch_cases`
- `direct_channel_peak_helpers_cover_zero_and_positive_wave_paths`
- `direct_channel_runoff_helpers_cover_branch_cases`
- `direct_scalar_validation_helpers_cover_remaining_guard_paths`
- `direct_hourly_and_sediment_input_validation_helpers_fail_closed`
- `direct_impoundment_horizon_and_channel_scaffold_guards_are_characterized`
- `direct_impoundment_outflow_helper_routes_valid_context`
- `direct_impoundment_outflow_helper_guards_terminal_values`
- `direct_ws20_profile_guard_cases_are_characterized`
- `direct_active_sediment_and_publication_guards_are_characterized`
- `direct_dependency_sediment_payload_guards_cover_remaining_paths`
- `direct_dependency_peak_payload_covers_channel_paths`
- `direct_dependency_peak_payload_covers_impoundment_and_kind_paths`
- `direct_dependency_baseflow_and_contributor_area_helpers_cover_frame_paths`
- `direct_dependency_sediment_accumulation_covers_channel_paths`
- `direct_contributor_groundwater_and_area_guards_fail_closed`
- `direct_hillslope_sediment_payload_guards_cover_remaining_paths`
- `direct_sediment_capacity_additional_guard_paths_are_characterized`
- `direct_terminal_sediment_hydraulic_guards_are_characterized`
