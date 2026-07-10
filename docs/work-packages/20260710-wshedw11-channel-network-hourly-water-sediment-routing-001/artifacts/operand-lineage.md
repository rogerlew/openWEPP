# Operand Lineage

Status: `EXECUTED-STATIC-HOLD`

Evidence mode: `Static`.

| Operand | Units/basis | Authority | Status/rejected aliases |
|---|---|---|---|
| HBP `V_h` | m3 per half-open hour `[h*3600,(h+1)*3600)` | ADR-0036 D2, SC-INFILE-HBP-001 | Authoritative inlet volume; reject peak/duration reconstruction. |
| HBP `S_h` | kg per same hour | ADR-0036 D2, SC-INFILE-HBP-001 | Authoritative total sediment ingress timing; reject `tdet-tdep` redistribution. |
| Local class mass `M[h,k]` | kg per hour/class | ADR-0036 D2 uniform event `frcflw` rule | Authoritative only as labeled uniform day-blend reconstruction; reject enriched per-hour claim. |
| Channel-grid inlet | m3 or kg per `dtchr` interval | Exact overlap of HBP half-open bins | Water projection is authority-backed; sediment total/class projection is algebraically closing but downstream process sequencing is blocked. |
| Routed `q1[it,node]` | m3/s at channel-grid boundary | baseline `wshchr` | Authoritative for `ipeak` 3-5; reject scalar `qpo/roff/durrof` reconstruction. |
| Routed class egress | kg per interval/class | No current canonical algorithm | BLOCKED; reject independent repeated event solves and uniform output redistribution. |
| Daily outlet water/mass | m3, kg | Sum/integral of terminal routed series | Future W11 authority; reject sum across internal channels (network double count). |
