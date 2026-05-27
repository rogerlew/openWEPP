# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Gate commands and outcomes for WSHEDIMPL04.

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
5. Scoped seam validation:
   - `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_projects_ws10_symbols` (pass)
   - `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_rejects_active_structure_projection_gap` (pass)
   - `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract` (pass; expected WSHED05/06/07 vectors remain ignored)
