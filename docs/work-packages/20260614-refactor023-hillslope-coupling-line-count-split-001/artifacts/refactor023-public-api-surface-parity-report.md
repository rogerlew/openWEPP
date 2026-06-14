# REFACTOR023 Public API Surface Parity Report

Status: complete

## Static

Expected stable inherent methods remain present:

- `Wb11HydrologyKernel::interval_overlap_duration`
- `Wb11HydrologyKernel::bounded_interval_overlap_duration`
- `Wb11HydrologyKernel::resolve_active_snow_coupling`
- `Wb11HydrologyKernel::validate_runtime_snow_state_domains`
- `Wb11HydrologyKernel::resolve_active_frost_coupling`
- `Wb11HydrologyKernel::compute_active_frost_coupling`

No intentional public API deltas.

Post-refactor locations:

- `coupling.rs`: interval and snow methods.
- `coupling/frost_entry.rs`: frost activation and active frost computation.

## Ran

- Public method search after refactor:
  - exit_code: 0
  - result:
    - `coupling.rs:74 interval_overlap_duration`
    - `coupling.rs:87 bounded_interval_overlap_duration`
    - `coupling.rs:101 resolve_active_snow_coupling`
    - `coupling.rs:160 validate_runtime_snow_state_domains`
    - `coupling/frost_entry.rs:8 resolve_active_frost_coupling`
    - `coupling/frost_entry.rs:73 compute_active_frost_coupling`
- `cargo check -p openwepp-hillslope-orchestrator`
  - exit_code: 0
  - result: crate compiled after module split.
