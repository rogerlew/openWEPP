# R4E-H Operand Lineage

Status: complete.

Evidence class: Static.

| Field | Units | Sign | Source authority | Authoritative in R4E-H? | Rejected aliases |
|---|---:|---|---|---|---|
| `subsurface_loss_handoff_m` | m | finite nonnegative | `SC-SUBHYD-001` `Qd` daily closure coupling | yes, as handoff input only | `q`, `Qdd`, `D`, `latqcc`, `Dp`, ET, runoff, residual |
| `subsurface_loss_m` | m | finite nonnegative | R4E-H direct state/downstream projection | yes, consumed by R4B | same as above |
| `evapotranspiration_handoff_m` | m | finite nonnegative | `SC-EVAP-001` aggregate ET withdrawal + `SC-WATBAL-001` closure term | yes, as handoff input only | `Ep`, `Es`, `Er`, residue interception, drainage, precipitation, runoff, residual |
| `evapotranspiration_m` | m | finite nonnegative | R4E-H direct state/downstream projection | yes, consumed by R4B | same as above |
| `snow_coupling_handoff_m` | m | finite signed | `SC-WATBAL-001#INV-WATBAL-013` signed `S` coupling | yes, as handoff input only | raw precipitation, routed melt, post-winter rain, `RM`, `Snow-Water`, `frozwt`, runoff, ET, `D`, `Qd`, residual |
| `snow_coupling_m` | m | finite signed | R4E-H direct state/downstream projection | yes, consumed by R4B | same as above |

Closure tolerance remains a policy input owned by R4B. R4E-H does not change
the tolerance formula or publication surfaces.
