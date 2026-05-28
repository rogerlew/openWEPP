# WSHEDIMPL30 Pre-Implementation Contract Gate

Status: complete  
Evidence mode: ran  
Date: 2026-05-28

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl30_contract_ws20_ishape3_erodible_lane_vector_executes -- --exact`
  - result: fail (expected pre-implementation)
  - observed status: `WKERNEL-WS10-CHANNEL-E-003` domain violation (channel step rejected).
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl30_contract_ws20_ishape3_depa_depb_fallback_mapping_affects_outputs -- --exact`
  - result: fail (expected pre-implementation)
  - observed status: `WKERNEL-WS10-CHANNEL-E-003` domain violation (channel step rejected).
