# Hydrograph-Shape Lineage

Status: **PENDING**.

Record the active-routed-water erosion shape lineage before runtime edits.

| Surface | Units | Basis | Source authority | Consumer | Notes |
|---|---|---|---|---|---|
| DC01 source weights | fraction | lane-local source volume | pending | default/off erosion substrate | Must not carry active-routed-water closure. |
| Lane D routed hydrograph | pending | per-lane/per-OFE routed water | pending | active-mode erosion substrate | D13 target. |
| `V_h` | m3 | hillslope exit / active equivalent | ADR-0036 | HBP EVENT | Closure required. |
| `S_h` | kg | hillslope exit / active equivalent | SC-SED-001 | HBP EVENT | Closure required. |

