# Public API Surface Parity Report

Static: the refactor was private-helper extraction inside
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`.

## Stable Public Surface

The following target-module public surface remains present and compatible:

| Item | Status |
|---|---|
| `HillslopePlRuntimeSurfaces` | unchanged public struct name |
| `HillslopePlRuntimeSurfaces::pl_schedule_surface` | unchanged public field |
| `HillslopePlRuntimeSurfaces::pl_growth_surface` | unchanged public field |
| `HillslopePlRuntimeSurfaces::pl_decomp_surface` | unchanged public field |
| `HillslopePlRuntimeSurfaces::merged_state_surface` | unchanged public method signature |
| `build_hillslope_pl_runtime_surfaces_from_management` | unchanged public function signature |
| `build_hillslope_runtime_surface_from_management` | unchanged public function signature |

Static: a private `PlRuntimeSurfaceBuilder` was added only to keep internal map
names shorter during projection; it converts back into the unchanged public
`pl_*` fields.

Disposition: no intentional public API delta.
