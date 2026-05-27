# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Required gate commands and scoped WS12 validation outcomes for WSHEDIMPL07.

## Ran
1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test --workspace`
   - result: pass
4. `cargo deny check`
   - result: pass with existing duplicate-crate and unmatched-license warnings
5. Scoped watershed validation:
   - `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract` (pass; WSHED07 vector active)
   - `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract` (pass)
