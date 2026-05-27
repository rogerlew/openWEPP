# WSHEDIMPL04 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented runtime seam projection helpers:
  - `derive_ws12_impoundment_coefficients`
  - `derive_power_law_curve_coefficients`
  in `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`.
- Added runtime-input unit-test assertions for projected coefficient symbols and
  active-structure fail-closed behavior.
- Removed test-only manual coefficient seed helpers from WS10/WS11/WS12
  integration fixtures.
- Activated WS12 parser-projection vector in
  `ws12_impoundment_physics_equivalence_contract.rs`.

## Ran
- `cargo fmt`
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_projects_ws10_symbols`
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_rejects_active_structure_projection_gap`
- `cargo test -p openwepp --test ws10_watershed_kernel_contract`
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract`
- `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract`
  - first run failed because the promoted WS12 vector still removed projected
    symbols before execution; test was corrected to assert success on
    parser-seeded surfaces directly.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_mofe05_accepts_valid_multiofe_metadata_and_reaches_output_guard`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
  - result: failed on existing unrelated lane:
    `erod13_registry_updates_reference_wave1_authority`
- `cargo deny check`
  - result: pass with existing duplicate/unmatched-license warnings
- `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract`
  - result: pass (`ws11`/`ws12` expected-failure WSHED05/06/07 vectors remain
    ignored)
