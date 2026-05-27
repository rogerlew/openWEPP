# WSHEDIMPL16 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented/updated contract-derived vectors:
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
    - Promoted channel sediment vector from publication-only to equation checks
      for current production branch (`qsed`, `tc`).
    - Added fail-closed vector:
      `wshedimpl16_contract_channel_sediment_requires_particle_diameter_payload`.
  - Updated watershed contributor payload seed helpers in:
    - `tests/integration/ws10_watershed_kernel_contract.rs`
    - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
    - `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
  - Added alias registry coverage for `particle_diameter_m` in:
    - `tests/integration/erod15_wave3_contract_authority_closure_contract.rs`

## Ran
- `cargo test --workspace --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract --test erod15_wave3_contract_authority_closure_contract --test watershed_cli_behavior_contract` passed.
