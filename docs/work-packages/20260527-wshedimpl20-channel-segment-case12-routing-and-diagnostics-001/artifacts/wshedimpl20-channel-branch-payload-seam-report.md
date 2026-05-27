# WSHEDIMPL20 Channel Segment Routing Seam Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Added WS20 opt-in seam symbol:
  - `ws10_channel_{id}_ws20_case12_enable`
- Added WS20 diagnostic publication seam symbols:
  - `ws10_channel_{id}_ws20_case1_segment_count`
  - `ws10_channel_{id}_ws20_case2_segment_count`
  - `ws10_channel_{id}_ws20_detachment_unmigrated_segment_count`
- Added baseline-derived helper lineage in runtime path:
  - fall-velocity (`falvel`) translation from class diameter + specific gravity.
  - segment-loop routing scaffolding for inflow partition and case-family
    progression.
- Residual closure remains open by design:
  - full detachment family parity (`case34/detach/dcap/enddet`) is not closed.

## Ran
- WS11 integration suite passed with new WS20 vectors and no regressions in
  existing WS11/WS15/WS18/WS19 vectors.
