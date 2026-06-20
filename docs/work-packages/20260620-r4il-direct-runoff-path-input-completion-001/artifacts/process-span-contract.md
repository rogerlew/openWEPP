# R4I-L Process Span Contract

Status: complete.

Evidence class: Static.

## Spans

| Span | Direct phases | Input | State mutation | Downstream operand | Shadow projection |
|---|---|---|---|---|---|
| Liquid input | `Normalization -> RunoffReconciliation` | typed `liquid_input_handoff_m` | direct liquid state and R4A `liquid_input_m` input | `liquid_input_m` | lane/day `liquid_input_m` projection |
| Runon/carry | `LateralTransfer -> RunoffReconciliation` | typed surface runon and subsurface carry handoffs | direct runon/carry state and R4A `runon_input_m` input | `runon_input_m`, `subsurface_carry_m` | lane/day runon/carry projection |
| Infiltration/depression | `RunoffReconciliation -> StorageReconciliation` | typed cumulative infiltration and depression delta handoffs | direct infiltration/depression state and R4A inputs | `cumulative_infiltration_m`, `depression_storage_delta_m` | lane/day infiltration/depression projection |
| Saturation addback | `RunoffReconciliation -> StorageReconciliation` | typed saturation-addback handoff | direct saturation-addback state and R4A `surface_saturation_runoff_m` input | `surface_saturation_runoff_m` | lane/day saturation-addback projection |

## Scope Decision

R4I-L is handoff-only for infiltration/depression/saturation. It does not
migrate full WB14 compute; later R4/R6 package evidence may promote those
handoffs when enough branch authority and fixtures are present.

## Completeness Gate

R4A runoff partition must fail closed unless all of the following shadows are
present in the same direct day frame:

- R4I liquid input;
- R4J runon/carry;
- R4K infiltration/depression;
- R4L saturation addback.
