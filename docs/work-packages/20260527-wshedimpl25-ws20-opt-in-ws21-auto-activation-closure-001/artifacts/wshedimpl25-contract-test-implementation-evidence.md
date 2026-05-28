# WSHEDIMPL25 Contract Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Contract-derived WS11 integration vectors updated in:
  - `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- WS25 vectors implemented:
  - `wshedimpl25_contract_ws20_only_opt_in_requires_crfrac_projection`
    - Enables WS20-only opt-in (`ws20_case12_enable=1`) without `crfrac`.
    - Asserts fail-closed typed kernel error `WKERNEL-WS10-CHANNEL-E-001`.
  - `wshedimpl25_contract_ws20_only_opt_in_auto_activates_ws21_with_crfrac_projection`
    - Enables WS20-only opt-in and seeds `ws10_channel_1_crfrac_{class:04}`.
    - Asserts success, WS21 activity (`case3 + case4 > 0`), and
      `ws10_channel_1_ws20_detachment_unmigrated_segment_count == 0`.
- Existing WS24/default-off vectors preserved.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract`
  - Result: pass.
