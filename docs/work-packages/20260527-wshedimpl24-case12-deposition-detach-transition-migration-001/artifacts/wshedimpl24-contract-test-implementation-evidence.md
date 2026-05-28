# WSHEDIMPL24 Contract Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Updated WS11 contract vectors in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`:
  - added `seed_ws24_case12_transition_forcing(...)` to drive a stable
    `case12 -> detach` transition vector.
  - added
    `wshedimpl24_contract_case12_transition_requires_crfrac_projection`.
  - added
    `wshedimpl24_contract_case12_transition_routes_with_crfrac_projection`.
- WS24 contract-derived assertions:
  - transition vector fails closed without `ws10_channel_{id}_crfrac_{class:04}`
    (`WKERNEL-WS10-CHANNEL-E-001` path).
  - transition vector succeeds with `crfrac` projection and emits
    `ws10_channel_1_ws24_case2_detach_segment_count > 0`.
  - WS20 legacy default-off publication continuity keeps
    `ws10_channel_1_ws24_case2_detach_segment_count == 0.0`.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed
  (`24 passed; 0 failed`).
