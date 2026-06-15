# CQR12 Public API Surface Parity Report

Status: complete.

Static: planned production edits are private helper extraction in a runtime
input projection module. No public API change is authorized.

Static: after production refactor, public runtime input functions in
`04_snow_frost_irrigation.rs` remain:

- `build_hillslope_runtime_surface_from_snow`
- `build_hillslope_runtime_surface_from_frost`
- `build_hillslope_runtime_surface_from_irrigation_depletion`
- `seed_hillslope_runtime_surface_from_irrigation_depletion`
- `build_hillslope_runtime_surface_from_irrigation_fixeddate`
- `seed_hillslope_runtime_surface_from_irrigation_fixeddate`
- `seed_hillslope_runtime_surface_from_snow`
- `seed_hillslope_runtime_surface_from_frost`

Static: the only `00_core_types.rs` production delta is import expansion for
existing parser types used by private helpers. `HillslopeRuntimeInputError`
public variants were not changed.

Result: public API surface parity preserved.
