# WSHEDIMPL05 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Added `Ws11WaveRoutingState` and branch-specific helper functions in
  `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - `compute_kinematic_wave_state`
  - `compute_muskingum_cunge_state`
  - `channel_wave_state_symbol`
  - `require_non_negative_computed`
- WS10 channel writeback now conditionally publishes
  `ws10_channel_{id}_{q1,qin,qlat,c0,c1,c2,c3,c4}` for `ipeak` branches 3/4.
- Promoted WS11 WSHED03 expected-failure vector to active conformance.
- Updated `SC-ROUTE-001` + `science-contracts/index.md` to reflect WSHED05
  closure slice and narrowed residual gap scope.

## Ran
- `cargo fmt`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract`
- `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract`
- `cargo test --workspace`
  - result: failed on existing unrelated lane:
    `erod13_registry_updates_reference_wave1_authority`
- `cargo deny check`
  - result: pass with existing duplicate/unmatched-license warnings
