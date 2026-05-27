# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Gate commands and outcomes for WSHEDIMPL06.

## Ran
1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test --workspace`
   - result: fails on existing unrelated lane:
     `erod13_registry_updates_reference_wave1_authority`
4. `cargo deny check`
   - result: pass (warnings only: duplicate crate entries and unmatched license
     allowances already present in repository policy file)
5. Scoped WSHED06 validation:
   - `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract` (pass)
   - `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract` (pass; WSHED06 vector active, WSHED07 vector still ignored)
