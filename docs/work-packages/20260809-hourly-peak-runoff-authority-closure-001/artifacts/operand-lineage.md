# Peak Operand Lineage

Status: `implemented / terminal verification pending`

| Operand | Units | Time basis | Spatial basis | Authority | Disposition |
|---|---|---|---|---|---|
| WB14 hourly infiltration excess | m/bin | modeled hour | depth over OFE | authoritative source | include |
| hourly saturation carry | m/bin | WB19 production hour | depth over OFE | authoritative return source | include without retiming |
| hourly routed melt/liquid | m/bin | modeled hour | depth over OFE | authoritative source | include |
| normalized hourly runoff depth | m/bin | 24 one-hour bins | runoff-volume area basis | authoritative after closure | peak input |
| maximum hourly mean depth rate | m/s | one-hour mean | depth basis | derived | internal peak |
| hillslope maximum hourly mean flow | m3/s | one-hour mean | event-runoff area | derived once | public peak |
| rectangular-equivalent duration | s | derived | depth basis | diagnostic | `Q / peak_depth_rate` |
| rainfall elapsed duration / max rainfall | s; m/s | rainfall envelope | forcing | rejected for production peak | diagnostic only |
| daily surface return redistributed over rain excess | m/bin | invented | depth | rejected | forbidden |

Independent acceptance reconstructs hourly depth sum and public flow from
produced source bins plus published area. Equal-volume concentrated/spread
fixtures reject the old rainfall-envelope and uniform synthetic-time formulas.
