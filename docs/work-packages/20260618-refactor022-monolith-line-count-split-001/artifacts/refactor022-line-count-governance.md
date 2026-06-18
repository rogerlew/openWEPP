# REFACTOR022 Line-Count Governance

Evidence class: Static + Ran.

## Target Tier

All four target-tier files were split under the 2000-line WARN threshold.

| Original target | Before | Final parent | Largest split piece |
|---|---:|---:|---:|
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs` | 2807 | 201 | 1195 |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2672 | 4 | 865 |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` | 2671 | 27 | 1094 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs` | 2549 | 161 | 1253 |

## Final Split Counts

```text
   201 crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs
   520 crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/00_ws15_ws18_scaffold_and_hydraulics.rs
   898 crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs
  1195 crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs
     4 crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs
   621 crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/00_wb11_runtime_seed.rs
   865 crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/01_wb12_wb16_wb19_seed.rs
   412 crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/02_mofe03_wave2_seed.rs
   772 crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs
    27 crates/openwepp-kernel-contract/src/lib_mod/core_types.rs
  1094 crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs
   685 crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs
   866 crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs
   161 crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs
  1253 crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/00_lateral_transfer.rs
   627 crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/01_tile_drainage.rs
   514 crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs
```

## Remaining WARN Inventory

The deferred 2000-2500 line tier remains advisory WARN-band work. No file is over the
3000-line required-refactor threshold.

```text
2452 crates/openwepp-hillslope-orchestrator/src/scheduler.rs
2410 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
2219 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs
2095 crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs
2062 crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs
2002 crates/openwepp-watershed-output/src/writers.rs
```

Disposition: leave the deferred tier as documented WARN-band hygiene. Do not force those
splits into REFACTOR022.
