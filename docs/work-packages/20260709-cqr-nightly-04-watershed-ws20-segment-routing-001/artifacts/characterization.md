# Characterization

Evidence label: Static/Ran.

Status: `ATTEMPTED-ROLLED-BACK`

Baseline observations:

- Existing integration coverage includes
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`, which
  asserts the direct watershed kernel retains actual routing physics markers
  including `ws20_route_case12_segment_family_core` and `direct_ws20_crfrac`.
- Baseline target LCOV is `LF:934/LH:0`, so module-local or focused
  characterization is expected before decomposition.

Provisional module-local tests added in the target file before rollback:

- `ws20_flow_partition_splits_dependency_peak_fraction`
- `ws20_flow_partition_rejects_invalid_inputs`
- `ws20_prepare_class_transport_projects_mass_fluxes_and_particles`
- `ws20_prepare_class_transport_rejects_bad_class_number`
- `ws20_segment_hydraulics_uses_profile_widths_for_zero_flow`
- `ws20_segment_hydraulics_rejects_nonpositive_segment_length`
- `ws20_transport_snapshot_records_deposition_potential`
- `ws20_case12_class_update_covers_case1_deposition`
- `ws20_case12_class_update_covers_case2_transition_values`
- `ws20_case12_class_update_covers_low_qu_flow_branch`
- `ws20_case12_class_update_covers_lateral_case2_xde_branch`
- `ws20_route_case12_segment_records_case_diagnostics_without_transition`
- `ws20_case12_transition_xdemax_accepts_only_interior_mixed_case2`
- `ws20_try_case12_transition_declines_without_mixed_case12_state`
- `ws20_route_case12_segment_family_core_handles_empty_and_no_segment_cases`

Focused command evidence from the provisional implementation:

- `cargo nextest run -p openwepp-watershed-orchestrator` - exit `0`,
  `37 tests run: 37 passed, 0 skipped`.

Behavior oracle attempted:

- Tests assert existing private helper behavior, guard classes, exact routing
  arithmetic on synthetic fixtures, case diagnostics, and no-segment/empty
  class payload identity.
- No public runner output, runtime-symbol name, diagnostic meaning, science
  formula, threshold, or fail-closed guard posture was changed.

Hold disposition:

- Review found the provisional tests did not cover key refactored case34/case4
  paths and did not satisfy ADR-0021 science-tier coverage closure.
- The provisional tests and production helper extractions were rolled back from
  the target file for local hold closure.
