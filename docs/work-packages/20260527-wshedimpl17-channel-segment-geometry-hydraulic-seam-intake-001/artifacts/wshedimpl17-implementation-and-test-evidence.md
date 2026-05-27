# WSHEDIMPL17 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Production/runtime seam edits:
  - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
    - Added `seed_watershed_runtime_surface_from_slope_channel_profile` for
      WS17 channel segment/hydraulic scaffold projection.
    - Added fail-closed cardinality/domain checks for slope-profile coverage,
      point count, monotonic segment `x`, finite/value-domain validation.
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`
    - Added WS10 guard path
      `require_ws17_channel_segment_scaffold(...)` for mandatory WS17 families.
    - Added helper `require_channel_state_symbol_scalar(...)` for typed symbol
      access with missing/non-finite enforcement.
  - `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
    - Added slope parse path for watershed runfile (`CLIWAT-E-038`).
    - Wired WS17 seam seeding into runtime-surface build
      (`CLIWAT-E-039` failure path).
  - `tests/integration/ws10_watershed_kernel_contract.rs`
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
    - Added WS17 scaffold seeding helpers to seeded watershed surfaces.
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
    - Added WS17 fail-closed missing-symbol vector.

## Ran
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with non-failing duplicate/license-not-encountered
  warnings already present in policy output.
