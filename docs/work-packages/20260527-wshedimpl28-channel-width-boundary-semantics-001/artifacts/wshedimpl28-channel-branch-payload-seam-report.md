# WSHEDIMPL28 Channel Branch Payload Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Seam target:
  - Baseline `chnrt.for` uses boundary-specific width inputs in segment routing:
    - upper boundary hydraulic call: `widb(i-1)`
    - lower boundary hydraulic call: `wida(i)`
  - Prior runtime consumed `widb` for both boundaries in WS20 segment routing.
- WS28 seam closure:
  - runtime now reads and validates both symbol families and applies them with
    boundary-correct indexing in WS20 segment routing lanes.
  - no silent defaulting/clamping behavior was added; existing typed guard
    posture remains unchanged.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshedimpl28_contract_ws20_routing_responds_to_wida_lower_boundary_widths -- --exact` -> pass
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass (includes WS28 vector)
