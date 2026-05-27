# WSHEDIMPL21 Contract Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Added WS11 vectors in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`:
  - `wshedimpl21_contract_case34_routing_is_opt_in_and_defaults_to_zero_diagnostics`
  - `wshedimpl21_contract_case34_opt_in_tracks_case34_and_unmigrated_diagnostics`
- Vectors assert:
  - default-off (`ws21_case34_enable` absent) keeps WS21 diagnostics at zero;
  - WS20+WS21 opt-in publishes WS21 case-family diagnostics and explicit
    unresolved detach/dcap diagnostics while preserving scheduler/kernel
    success.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed
  (`19 passed; 0 failed`).
