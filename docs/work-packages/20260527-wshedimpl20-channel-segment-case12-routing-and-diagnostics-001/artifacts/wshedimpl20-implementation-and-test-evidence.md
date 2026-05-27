# WSHEDIMPL20 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented in `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - WS20 opt-in gate symbol:
    - `ws10_channel_{id}_ws20_case12_enable`
  - Incoming peak-partition struct and routing-input partition assembly.
  - Baseline-derived fall-velocity helper (`falvel` lineage constants and
    equation form).
  - WS20 segment-loop function:
    - inflow partition,
    - segment hydraulic/capacity coupling,
    - `case12` deposition-family formula path,
    - explicit unresolved-detachment diagnostics fallback path.
  - Published WS20 diagnostic output symbols:
    - `ws20_case1_segment_count`
    - `ws20_case2_segment_count`
    - `ws20_detachment_unmigrated_segment_count`
- Added WS11 integration vectors for default-off and opt-in diagnostics.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` passed.
