# WSHEDIMPL38 Channel-Sediment Symbol Burndown Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Scope
- Retire unresolved-detachment diagnostics symbol families from WS10 channel
  sediment publication surfaces.
- Replace residual unresolved fallback continuation in WS20/WS21 channel
  sediment process lanes with typed fail-closed domain guards.

## Static
- Symbols retired from runtime publication:
  - `ws10_channel_{id}_ws20_detachment_unmigrated_segment_count`
  - `ws10_channel_{id}_ws21_detach_unmigrated_segment_count`
- Case-family diagnostics retained and still published:
  - `ws20_case1_segment_count`
  - `ws20_case2_segment_count`
  - `ws24_case2_detach_segment_count`
  - `ws21_case3_segment_count`
  - `ws21_case4_segment_count`
  - `ws21_enddet_segment_count`
- Residual invalid-segment fallback branches now hard-fail with typed domain
  errors rather than fallback continuation.

## Ran
- `cargo test --test ws11_channel_routing_physics_equivalence_contract` -> pass
