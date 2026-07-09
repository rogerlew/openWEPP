# Obligation To Test Map

Evidence label: Static/Ran.

Status: `COMPLETE`

Scope:

- This map binds the direct-kernel obligations made applicable by
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`.
- It does not create new science authority; authority remains in `SC-ROUTE-001`,
  `SC-SED-001`, and `SC-IMPOUND-001`.

Applicable obligation vectors:

| Contract row | Direct-kernel vector in this package | Test evidence |
|---|---|---|
| `SC-ROUTE-001 INV-ROUTE-001` | Runon decomposition, positive-area conversion, and contributor-area admission. | `direct_ws11_runon_cases_are_characterized`; `direct_ws11_runon_guards_fail_closed`; `direct_dependency_baseflow_and_contributor_area_helpers_cover_frame_paths`; `direct_contributor_groundwater_and_area_guards_fail_closed`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-ROUTE-001 INV-ROUTE-002` | Channel duration selection from runon, channel, and irrigation candidates. | `direct_ws11_runon_cases_are_characterized`; `direct_channel_runoff_helpers_cover_branch_cases`; `direct_scalar_validation_helpers_cover_remaining_guard_paths`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-ROUTE-001 INV-ROUTE-003` | Explicit channel runoff Case I-IV branch behavior and zero-flow branch. | `direct_channel_runoff_helpers_cover_branch_cases`; `direct_ws11_runon_cases_are_characterized`; `direct_ws11_runon_guards_fail_closed`. |
| `SC-ROUTE-001 INV-ROUTE-004` | Transmission-loss closure and entering-water/runoff bounds. | `direct_channel_runoff_helpers_cover_branch_cases`; `direct_scalar_validation_helpers_cover_remaining_guard_paths`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-ROUTE-001 INV-ROUTE-005` | Hour-resolved inlet superposition, partial/mixed hourly fail-closed behavior, and sediment mass timing authority. | `direct_hourly_and_sediment_input_validation_helpers_fail_closed`; `read_direct_hillslope_sediment_payload_prefers_hourly_mass_authority`; `direct_hillslope_sediment_payload_guards_cover_remaining_paths`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-ROUTE-001 INV-ROUTE-006` | `ipeak` branch selection for rational, CREAMS, kinematic, Muskingum, and variable Muskingum paths. | `direct_range_and_ipeak_branch_helpers_cover_guard_cases`; `direct_channel_peak_helpers_cover_branch_cases`; `direct_channel_peak_helpers_cover_zero_and_positive_wave_paths`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-ROUTE-001 INV-ROUTE-007` | Peak-duration closure and zero/positive routed peak paths. | `direct_channel_peak_helpers_cover_branch_cases`; `direct_channel_peak_helpers_cover_zero_and_positive_wave_paths`; `direct_channel_runoff_helpers_cover_branch_cases`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-ROUTE-001 INV-ROUTE-008` | Channel profile, hydraulic terminal, and shear-driving domain guards at the direct WS20 boundary. | `direct_ws20_profile_projects_segment_points_and_guards_domains`; `direct_ws20_profile_guard_cases_are_characterized`; `direct_terminal_sediment_hydraulic_guards_are_characterized`; `direct_sediment_capacity_helpers_cover_terminal_and_guard_paths`. |
| `SC-ROUTE-001 INV-ROUTE-009` | Segment sediment continuity, inlet/lateral source accounting, and particle class partitioning. | `direct_sediment_accumulator_projects_ingress_and_active_classes`; `direct_dependency_sediment_accumulation_covers_channel_paths`; `direct_dependency_sediment_payload_guards_cover_remaining_paths`; `direct_hillslope_sediment_payload_guards_cover_remaining_paths`. |
| `SC-ROUTE-001 INV-ROUTE-010` | Detachment/deposition and transport-capacity branch surfaces used by direct routing. | `direct_ws20_crfrac_normalizes_selected_classes_and_guards_payloads`; `direct_sediment_capacity_additional_guard_paths_are_characterized`; `direct_terminal_sediment_hydraulic_guards_are_characterized`; `direct_sediment_capacity_helpers_cover_terminal_and_guard_paths`. |
| `SC-ROUTE-001 INV-ROUTE-011` | Required hillslope, channel dependency, and impoundment coupling payload admission. | `read_direct_channel_sediment_payload_covers_zero_valid_and_guard_paths`; `direct_dependency_peak_payload_covers_channel_paths`; `direct_dependency_peak_payload_covers_impoundment_and_kind_paths`; `direct_dependency_baseflow_and_contributor_area_helpers_cover_frame_paths`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-SED-001 INV-SED-001` | Direct sediment continuity assembler and publication mass partitioning. | `direct_sediment_accumulator_projects_ingress_and_active_classes`; `direct_active_sediment_and_publication_guards_are_characterized`; `direct_dependency_sediment_accumulation_covers_channel_paths`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-SED-001 INV-SED-006` | Direct transport-capacity calculator guard and terminal-hydraulic inputs. | `direct_sediment_capacity_additional_guard_paths_are_characterized`; `direct_terminal_sediment_hydraulic_guards_are_characterized`; `direct_sediment_capacity_helpers_cover_terminal_and_guard_paths`. |
| `SC-SED-001 INV-SED-010` | Hillslope-to-routing sediment payload fields, hourly sediment authority, class count, concentration, diameter, and flow-fraction guards. | `read_direct_hillslope_sediment_payload_prefers_hourly_mass_authority`; `direct_hillslope_sediment_payload_guards_cover_remaining_paths`; `direct_hourly_and_sediment_input_validation_helpers_fail_closed`; `read_direct_channel_sediment_payload_covers_zero_valid_and_guard_paths`. |
| `SC-IMPOUND-001 INV-IMPOUND-001` | Direct impoundment continuity integration result and residual guard surfaces. | `direct_impoundment_outflow_helper_routes_valid_context`; `direct_impoundment_outflow_helper_guards_terminal_values`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-IMPOUND-001 INV-IMPOUND-002` | Stage, area, and horizon domain guards consumed by direct impoundment routing. | `direct_impoundment_horizon_and_channel_scaffold_guards_are_characterized`; `direct_impoundment_outflow_helper_guards_terminal_values`; `direct_scalar_validation_helpers_cover_remaining_guard_paths`. |
| `SC-IMPOUND-001 INV-IMPOUND-003` | Outlet-structure outflow summation and no-flow/flow guard behavior at the direct helper boundary. | `direct_impoundment_outflow_helper_routes_valid_context`; `direct_impoundment_outflow_helper_guards_terminal_values`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-IMPOUND-001 INV-IMPOUND-004` | Adaptive route helper admission and terminal value guard behavior. | `direct_impoundment_horizon_and_channel_scaffold_guards_are_characterized`; `direct_impoundment_outflow_helper_routes_valid_context`; `direct_impoundment_outflow_helper_guards_terminal_values`. |
| `SC-IMPOUND-001 INV-IMPOUND-009` | Impoundment dependency payloads feeding downstream direct routing. | `direct_dependency_peak_payload_covers_impoundment_and_kind_paths`; `direct_dependency_baseflow_and_contributor_area_helpers_cover_frame_paths`; `wshedw5_typed_watershed_runtime_contract`. |
| `SC-IMPOUND-001 INV-IMPOUND-010` | Unit-governance preservation for direct impoundment handoff fields. | `numeric-equivalence.md`; `direct_impoundment_outflow_helper_routes_valid_context`; `wshedw5_typed_watershed_runtime_contract`. |

Current run evidence:

- `cargo nextest run -p openwepp-watershed-orchestrator` - pass,
  `68 tests run: 68 passed`.
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` - pass,
  `18 tests run: 18 passed`.
