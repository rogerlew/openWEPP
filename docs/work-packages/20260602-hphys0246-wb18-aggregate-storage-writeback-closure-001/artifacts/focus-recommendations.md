# HPHYS0246 Focus Recommendations

Status: completed
Evidence mode: Static + Ran

## Primary Focus
- Scaffold the next HPHYS package around WB19 lateral transfer day-1 behavior.
- Prioritize H39 because it retains the largest post-WB18 day-1 lateral loss:
  `-79.515092 mm`.

## Specific Questions for Follow-Up
- Does WB19 lateral withdrawal use the same aggregate pool and layer-pool bounds
  as pinned legacy WEPP for the day-1 H1/H7/H39 profiles?
- Are `coca`, `fcdep`, `unsdep`, saturated thickness, and `watyld` surfaces
  seeded with baseline-equivalent lineage at the WB19 boundary?
- Is WB19 subtracting from aggregate `wb11_soil_water` before or after the same
  layer/storage lineage updates used by legacy WATBAL?
- Is the H39 lateral transfer a real baseline-authoritative flux or an
  over-withdrawal caused by openWEPP pool/threshold mismatch?

## Do Not Prioritize First
- Do not tune WB18 `D`/`Pe`; HPHYS0246 shows WB18 aggregate continuity now
  behaves as `-D` with preserved residual storage.
- Do not compensate in WB13; WB13 is reflecting scheduler aggregate storage and
  adds no observed discontinuity.
- Do not clamp lateral flux as a parity shortcut; baseline-authoritative WB19
  process lineage must be audited.
