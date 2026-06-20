# R4I-L Operand Lineage

Status: complete.

Evidence class: Static.

| Field | Units | Sign | Source authority | Authoritative in R4I-L? | Rejected aliases |
|---|---:|---|---|---|---|
| `liquid_input_handoff_m` | m | finite nonnegative | `SC-RUNOFFPART-001` runoff liquid-input operand | yes, as handoff input only | raw precipitation, routed melt, irrigation publication, interception storage, runoff, residual |
| `liquid_input_m` | m | finite nonnegative | R4I direct state/downstream projection | yes, consumed by R4A | same as above |
| `surface_runon_handoff_m` | m | finite nonnegative | R3C/R4A surface carry into runoff partition | yes, as handoff input only | subsurface carry, lateral transfer, local precipitation, public `UpStrmQ`, residual |
| `runon_input_m` | m | finite nonnegative | R4J direct state/downstream projection | yes, consumed by R4A | same as above |
| `subsurface_carry_handoff_m` | m | finite nonnegative | R3C diagnostic separation from surface runon | diagnostic only in R4I-L | surface runon, `Qd`, lateral flow publication |
| `cumulative_infiltration_handoff_m` | m | finite nonnegative | `SC-RUNOFFPART-001` retained infiltration operand | yes, as handoff input only | depression storage, saturation addback, partition runoff, liquid input, publication runoff |
| `depression_storage_delta_handoff_m` | m | finite nonnegative | `SC-RUNOFFPART-001` retained depression-storage operand | yes, as handoff input only | infiltration, saturation addback, storage residual |
| `surface_saturation_runoff_handoff_m` | m | finite nonnegative | `SC-RUNOFFPART-001` saturation addback operand | yes, as handoff input only | infiltration, depression storage, partition runoff, public runoff |
