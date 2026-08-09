# Peak Operand Lineage

Status: `implemented / terminal verification pending`

| Operand | Units | Time basis | Spatial basis | Authority | Disposition |
|---|---|---|---|---|---|
| WB14 hourly post-partition runoff | m/bin | modeled hour | depth over OFE | authoritative runoff | include after infiltration and depression storage; complete daily frost retention clears it, while partial positive retention requires an hourly producer or hard-fails |
| hourly saturation carry | m/bin | WB19 production hour | depth over OFE | authoritative return source | include without retiming |
| hourly routed melt/liquid | m/bin | producer-modeled hour | depth over OFE | authoritative liquid supply | admit to WB14 once; never append as runoff |
| hourly surface/lateral runon | m/bin | produced upstream carry hour | depth over OFE | authoritative liquid supply | admit to WB14 once; missing positive shape hard-fails |
| later daily same-pass infiltration reconstruction | m/day | no lawful hourly clock | depth over OFE | rejected alias | cannot override WB14 infiltration or debit any hourly runoff bin, including local-only supply |
| closing hourly runoff depth | m/bin | 24 one-hour bins | runoff-volume area basis | authoritative after `TOL-WATBAL-009` reconciliation | peak input; normalized weights are derivative only |
| maximum hourly mean depth rate | m/s | one-hour mean | depth basis | derived | internal peak |
| hillslope maximum hourly mean flow | m3/s | one-hour mean | event-runoff area | derived once | public peak |
| rectangular-equivalent duration | s | derived | depth basis | diagnostic | `Q / peak_depth_rate` |
| rainfall elapsed duration / max rainfall | s; m/s | rainfall envelope | forcing | rejected for production peak | diagnostic only |
| daily surface return redistributed over rain excess | m/bin | invented | depth | rejected | forbidden |

Independent acceptance reconstructs hourly depth sum and public flow from
produced post-partition bins plus published area. Equal-volume concentrated/
spread fixtures reject the old rainfall-envelope and uniform synthetic-time
formulas. Melt/runon tests separately prove infiltration opportunity and
residual timing, preventing raw-source double admission.
