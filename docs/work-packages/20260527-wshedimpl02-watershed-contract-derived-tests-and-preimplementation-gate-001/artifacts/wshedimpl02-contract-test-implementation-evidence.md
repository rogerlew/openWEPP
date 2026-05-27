# WSHEDIMPL02 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
Implemented WSHED03 contract-derived vectors in:
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
  - added ignored expected-failure vectors for:
    - KW/MC wave-routing state-family publication closure,
    - channel-sediment publication-family closure.
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
  - added ignored expected-failure vectors for:
    - parser-projected coefficient closure without manual seed,
    - RK4/regime-transition timestep-stability closure.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
  - added ignored expected-failure vector for watershed CLI end-to-end
    non-placeholder parquet emission closure.

## Ran
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract`
  - result: pass (`6 passed; 2 ignored`)
- `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract`
  - result: pass (`4 passed; 2 ignored`)
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
  - result: pass (`7 passed; 1 ignored`)
