# Melt Operand Lineage

Status: complete for 05A.
Evidence mode: Static.

| Operand | Units | Source authority | Runtime source | 05A disposition | Rejected aliases |
|---|---|---|---|---|---|
| `hrmelt_raw` | `m` | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-052`, WEPP Ch. 3 | existing legacy CoE path | authoritative current identity | SWE delta, degree-day proxy |
| `amelt` | legacy inch-equivalent term | WEPP Ch. 3 + v76 amendment | existing trace family | authoritative current term | tuned radiation scalar |
| `melt_bmelt_in` | legacy inch-equivalent signed term | v76 sign convention | existing trace family | already-signed contribution | silent sign flip, double subtraction |
| `hrrad` / hourly shortwave | `MJ m^-2 h^-1` | `SC-CLIMATE-001#INV-CLIMATE-013` | existing winter hourly radiation | source binding deferred to 05B | raw Langleys/day, fitted scalar |
| `snow_albedo` | fraction | v76 placeholder + future 05C | none in production | deferred to 05C | constant hidden default |
| `cancov` | fraction | WEPP Ch. 3 / plant state | existing plant state | preserve `(1 - cancov)` | density surrogate |
| `cmelt` | legacy inch-equivalent term | WEPP Ch. 3 + v76 amendment | existing trace family | authoritative current term | degree-day melt factor |
| `wmelt` / routed melt | `m` | `INV-SNOWFREEZE-015/019/021/022/052` | existing routed melt lineage | conservation reconstruction deferred to 05D | raw precipitation, SWE delta |
