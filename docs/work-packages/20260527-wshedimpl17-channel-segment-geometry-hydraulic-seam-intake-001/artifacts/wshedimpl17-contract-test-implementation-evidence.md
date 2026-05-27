# WSHEDIMPL17 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented/updated contract-derived vectors:
  - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` unit tests:
    - `watershed_channel_slope_runtime_seed_projects_ws17_segment_symbols`
    - `watershed_channel_slope_runtime_seed_rejects_profile_count_mismatch`
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`:
    - Added fail-closed vector
      `wshedimpl17_contract_channel_segment_scaffold_requires_ws17_symbols`.
  - Updated watershed seeded test surfaces to include WS17 scaffold families in:
    - `tests/integration/ws10_watershed_kernel_contract.rs`
    - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
    - `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

## Ran
- `cargo test --workspace` passed, including WS10/WS11/WS12 watershed contract
  suites and new WS17 vectors.
