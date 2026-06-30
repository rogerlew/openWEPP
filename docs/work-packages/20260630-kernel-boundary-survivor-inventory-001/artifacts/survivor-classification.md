# Survivor Classification

Evidence class: Static source classification.

Each row classifies the file-level owner of core carrier/runtime references.
Reference-level classification inherits the file row unless a follow-on package
finds mixed ownership inside that file.

Legend:

- `EXEC` - compiled executable scheduler/day-frame runtime or lifecycle support.
- `KB` - kernel request/writeback boundary requiring typed API replacement.
- `TRACE` - diagnostic, trace, shadow, or audit support requiring typed events.
- `PUB` - WB13, publication, manifest, or output support requiring typed streams.
- `IO` - genuine intake/output or serialization adapter candidate.
- `TEST` - tests to migrate to typed-boundary coverage or delete if scheduler-only.
- `META` - direct-runtime provenance/counter support.

## File-Level Classification

| Matches | Class | File | Route |
| ---: | --- | --- | --- |
| 162 | EXEC | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | Delete after typed kernel boundary and typed event sinks exist. |
| 122 | EXEC | `crates/openwepp-hillslope-orchestrator/src/day_frame.rs` | Delete after typed event sinks replace roundtrip/frame shadow support. |
| 111 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs` | Replace with typed phase-result tests or delete if only symbol writeback behavior. |
| 72 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` | Replace symbol lookup helpers with typed phase context accessors. |
| 62 | IO | `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs` | Keep temporarily as serialization/legacy edge; remove from production kernel boundary later. |
| 31 | PUB | `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | Remove scheduler imports and runtime-surface provenance branches after typed publication/event streams. |
| 31 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/phase.rs` | Convert to typed phase-boundary tests. |
| 30 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/boundaries.rs` | Convert to typed boundary tests or keep only I/O adapter tests. |
| 27 | PUB | `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | Move WB13/scheduler-output helpers behind typed publication streams. |
| 27 | KB | `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs` | Replace request/writeback API with typed kernel-boundary API. |
| 26 | EXEC | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs` | Delete after typed trace/publication replacements land. |
| 26 | TRACE | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs` | Replace request-carried frost trace access with typed frost trace payloads. |
| 23 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs` | Convert hydrology tests to typed request/result fixtures. |
| 22 | TEST | `crates/openwepp-kernel-contract/src/lib.rs` | Split symbol-boundary contract tests from new typed-boundary tests. |
| 20 | PUB | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/01_wb12_wb16_wb19_seed.rs` | Preserve only typed seed/publication projections; remove surface writer support. |
| 18 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/growth.rs` | Convert growth transition tests to typed result/mutation API. |
| 18 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs` | Type EROD19 inputs/results. |
| 18 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs` | Type runoff reconciliation inputs/results. |
| 16 | PUB | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/02_mofe03_wave2_seed.rs` | Preserve typed Wave-2 seed projection; delete symbol writer support. |
| 16 | TRACE | `crates/openwepp-runner/src/hillslope/indexed_shadow_surface.rs` | Delete after typed diagnostics no longer need shadow symbol projection. |
| 16 | IO | `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs` | Keep only if needed for serialization/adapters; remove from production kernel boundary. |
| 15 | TRACE | `crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs` | Replace with typed source-scan/audit guard or delete after carrier removal. |
| 15 | PUB | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/00_wb11_runtime_seed.rs` | Keep typed seed cores only; remove runtime-surface writer support. |
| 15 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs` | Keep direct-runtime assertions; remove symbol allowlist assertions as carriers disappear. |
| 15 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs` | Type plant/percolation inputs/results. |
| 13 | PUB | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | Remove residual surface authority helpers once typed metadata is complete. |
| 13 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/00_lateral_transfer.rs` | Type lateral transfer inputs/results. |
| 12 | IO | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs` | Keep temporarily as input projection adapter; remove symbol projection after typed consumers exist. |
| 11 | TEST | `crates/openwepp-runner/src/hillslope/tests03/trace.rs` | Repoint trace tests to typed event payloads. |
| 11 | TRACE | `crates/openwepp-runner/src/hillslope/scheduler_trace/hphys_trace.rs` | Replace surface reads with typed HPHYS event payload. |
| 11 | PUB | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/02_publication_and_manifest_helpers.rs` | Remove scheduler/publication fallback fields after typed streams. |
| 11 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs` | Replace `HillslopeKernel` request/response dispatch with typed dispatch. |
| 10 | PUB | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs` | Move any residual surface metadata to typed frost/layer inputs. |
| 10 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/day_frame.rs` | Delete with `day_frame.rs` unless an I/O adapter test remains. |
| 10 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs` | Type Ksat adjustment inputs/results. |
| 9 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/fixtures.rs` | Replace symbol fixtures with typed phase fixtures. |
| 8 | TEST | `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs` | Keep direct state assertions; remove symbol surface setup helpers. |
| 8 | META | `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | Remove scheduler/runtime-surface branches after typed provenance is complete. |
| 8 | TEST | `crates/openwepp-runner/src/hillslope/03_tests.rs` | Keep direct selector tests; remove obsolete counter expectations when counters are retired. |
| 8 | IO | `crates/openwepp-hillslope-orchestrator/src/lib.rs` | Split exports into typed API exports and edge-only symbol adapter exports. |
| 7 | PUB | `crates/openwepp-runner/src/hillslope/intake_lane_setup/wb11_seed_helpers.rs` | Delete symbol seed helpers after typed seed authority is sole test path. |
| 7 | IO | `crates/openwepp-runner/src/hillslope/intake_lane_setup/runtime_surface_helpers.rs` | Remove runtime-surface helper after no I/O adapter requires it. |
| 7 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs` | Remove scheduler/day-frame test modules after replacements. |
| 7 | IO | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs` | Keep as intake adapter until typed climate projection no longer emits symbols. |
| 7 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs` | Type frost coupling context and outputs. |
| 7 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs` | Type coupling helpers. |
| 7 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod14.rs` | Type EROD14 inputs/results. |
| 7 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/01_tile_drainage.rs` | Type tile drainage inputs/results. |
| 7 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_infiltration_evap.rs` | Type infiltration/evap inputs/results. |
| 5 | TEST | `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb19_wb12_wb16.rs` | Convert publication seed tests to typed seed/event assertions. |
| 5 | TEST | `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs` | Update guard allowlist as symbol carriers are removed. |
| 5 | IO | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs` | Keep temporarily as intake adapter; remove symbol projection later. |
| 5 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs` | Type runoff reconciliation helpers. |
| 4 | TEST | `crates/openwepp-runner/src/hillslope/tests03/publication/publication_scheduler_pl_activation.rs` | Replace scheduler PL activation coverage with typed direct coverage. |
| 4 | PUB | `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs` | Replace WB13 scheduler rows with typed publication/audit events. |
| 4 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/irrigation.rs` | Type irrigation helper context. |
| 4 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_peak_runoff.rs` | Type peak runoff inputs/results. |
| 4 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod13.rs` | Type EROD13 inputs/results. |
| 4 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/05_pl_phase_dispatch.rs` | Type plant/decomposition dispatch. |
| 4 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/00_pl_slot_resolution.rs` | Type PL slot resolution inputs. |
| 3 | TEST | `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb11_seed.rs` | Keep only typed seed authority tests. |
| 3 | PUB | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs` | Delete after typed publication streams replace scheduler publication. |
| 3 | IO | `crates/openwepp-runner/src/hillslope/intake_lane_setup/lane_setup_helpers.rs` | Remove residual symbol helpers after typed lane setup cleanup. |
| 3 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_storage_reconciliation.rs` | Type storage reconciliation inputs/results. |
| 2 | TRACE | `crates/openwepp-runner/src/hillslope/scheduler_trace/perfdeep02_frame_roundtrip.rs` | Delete with frame-roundtrip diagnostics unless converted to typed frame check. |
| 2 | TEST | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4mo.rs` | Keep only direct typed assertions. |
| 2 | EXEC | `crates/openwepp-hillslope-orchestrator/src/scheduler/water_balance.rs` | Delete with scheduler or convert to typed support if still needed. |
| 2 | IO | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/common.rs` | Keep only intake adapter test fixtures. |
| 2 | IO | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs` | Keep as intake adapter until typed management projection no longer emits symbols. |
| 1 | PUB | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs` | Remove residual symbol metadata after typed snow/frost publication. |
| 1 | TEST | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/irrigation_fixeddate.rs` | Keep only intake adapter coverage. |
| 1 | IO | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs` | Keep only if runtime-input serialization needs it. |
| 1 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs` | Type infiltration reconciliation helper. |
| 1 | KB | `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs` | Type shared kernel helper context. |

## Classification Summary

| Class | Files | Immediate handling |
| --- | ---: | --- |
| `EXEC` | 4 | Delete only after typed boundary and event sinks remove live support needs. |
| `KB` | 23 | First implementation target: typed phase context and typed phase result APIs. |
| `TRACE` | 5 | First practical migration slice: typed diagnostic/event payloads. |
| `PUB` | 12 | Stream typed audit/publication rows; stop reconstructing from WB13/surfaces. |
| `IO` | 10 | Temporary allowlist for true intake/output adapters; narrow later. |
| `TEST` | 18 | Migrate to typed tests or delete scheduler-only tests. |
| `META` | 1 | Remove obsolete scheduler provenance/counters after API cleanup. |

## Boundary Symbol/Value Treatment

The `BoundarySymbol` and `BoundaryValue` surface is larger than the core carrier
surface. It includes:

- kernel-contract serialization types;
- runtime input projection helpers;
- guard and error message payloads;
- legacy-style fixtures and tests;
- trace/publication test scaffolding.

This package does not classify every `BoundarySymbol` and `BoundaryValue`
occurrence as a deletion target. The next implementation packages should first
remove the core request/writeback/scheduler carriers; then rescan symbol/value
survivors and keep only documented intake/output serialization adapters.
