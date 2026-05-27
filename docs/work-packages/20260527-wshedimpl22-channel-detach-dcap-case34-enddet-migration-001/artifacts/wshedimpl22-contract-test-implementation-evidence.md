# WSHEDIMPL22 Contract Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Added WS22 vectors in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`:
  - `wshedimpl22_contract_ws21_detach_dcap_requires_crfrac_projection`
  - `wshedimpl22_contract_ws21_opt_in_routes_with_crfrac_projection`
- Updated WS21 opt-in vector setup to seed `crfrac` projection family:
  - `wshedimpl21_contract_case34_opt_in_tracks_case34_and_unmigrated_diagnostics`
- WS22 vectors assert:
  - WS20+WS21 opt-in fails typed when required
    `ws10_channel_{id}_crfrac_{class:04}` symbols are absent.
  - WS20+WS21 opt-in succeeds when `crfrac` symbols are projected and publishes
    WS21 case-family diagnostics continuity.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed
  (`21 passed; 0 failed`).
