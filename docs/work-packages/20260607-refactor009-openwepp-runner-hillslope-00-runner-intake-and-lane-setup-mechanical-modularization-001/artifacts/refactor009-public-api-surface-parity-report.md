# REFACTOR009 refactor009 public api surface parity report

Status: complete  
Evidence mode: Static

## Scope
- Intake helpers were extracted into `intake_lane_setup/*` and re-exposed in
  `crates/openwepp-runner/src/hillslope/intake_lane_setup/mod.rs` with
  crate-local visibility.

## Preserved exports
- `build_execution_lane_context`
- `build_mode_selection_provenance`
- `build_adapter_boundary_provenance`
- `build_timestep_policy_provenance`
- `validate_hillslope_ofe_topology_parity`
- `absent_pmetpara_file`
- `build_hillslope_runtime_surface_from_pmetpara`
- `merge_runtime_surfaces`
- `discover_sidecars`
- `hillslope_sidecar_contracts`
- `ensure_output_parent_directory`
- `resolve_run_file`
- `parse_runfile_execution_config`
- `optional_sidecar_binding_path`
- `compute_wb11_et_demand_seed`
- `publish_wb11_et_demand_seed`
- `optional_output_paths`/`required_output_paths` call sites are unchanged by module location.

## Parity disposition
- No public API removals observed for the stated write-set.
- No new exported surface was introduced outside the package seam.
