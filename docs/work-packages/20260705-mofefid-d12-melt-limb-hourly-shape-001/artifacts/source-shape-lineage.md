# Source-Shape Lineage

Status: **PENDING**.

Record the operand lineage before runtime edits:

| Surface | Units | Lane/OFE basis | Timing basis | Source authority | Consumer | Notes |
|---|---|---|---|---|---|---|
| `runvol/area` | m | lane-local | daily total | pending | Lane D shadow | Existing source-depth basis. |
| `wb14_hourly_excess_m[h]` | m/hour slot | lane-local | 24 hourly bins | pending | DC01 shape | Existing D1 limb. |
| `ui_SCrunf` lineage | m/hour slot | lane-local | 24 hourly bins | pending | DC01 shape | Existing D1 limb. |
| melt/routed-liquid candidate | pending | pending | pending | pending | pending | D12 target. |

Acceptance must include an independent reconstruction, not only a producer
self-check.

