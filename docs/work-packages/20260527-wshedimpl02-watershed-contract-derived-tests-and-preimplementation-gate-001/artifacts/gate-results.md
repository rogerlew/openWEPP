# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- WSHED03 scope edits include test-surface changes; validation commands were run
  to establish gate posture.

## Ran
- `cargo fmt --check`
  - result: pass
- `cargo clippy --workspace --all-targets -- -D warnings`
  - result: pass
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract`
  - result: pass (`6 passed; 2 ignored`)
- `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract`
  - result: pass (`4 passed; 2 ignored`)
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
  - result: pass (`7 passed; 1 ignored`)
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract -- --ignored --nocapture`
  - result: expected fail (`0 passed; 2 failed`)
- `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract -- --ignored --nocapture`
  - result: expected fail (`0 passed; 2 failed`)
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --ignored --nocapture`
  - result: expected fail (`0 passed; 1 failed`)
- `cargo test --workspace`
  - result: fail on unrelated existing lane
    `erod13_registry_updates_reference_wave1_authority`
- `cargo test -p openwepp --test erod13_contract_authority_closure_contract`
  - result: reproduces same unrelated failure
- `cargo deny check`
  - result: pass (existing duplicate-crate and unmatched-license warnings)
