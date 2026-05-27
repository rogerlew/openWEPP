# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Required repository gates and WSHED09 scoped validation/comparator lanes.

## Ran
1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test --workspace`
   - result: pass
4. `cargo deny check`
   - result: pass with existing duplicate-crate and unmatched-license warnings
5. Scoped watershed/comparator reruns:
   - `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract` (pass)
   - `cargo test -p openwepp-runner --test watershed_cli_behavior_contract` (pass)
   - `cargo test -p openwepp --test comparator_tier_routing_metadata` (pass)
   - `cargo test -p openwepp --test clim07_climate_comparator_and_closure_contract` (pass)
