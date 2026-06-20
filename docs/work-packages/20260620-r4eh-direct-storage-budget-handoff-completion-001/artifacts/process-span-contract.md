# R4E-H Process Span Contract

Status: complete.

Evidence class: Static.

## Spans

| Span | Direct phases | Input | State mutation | Downstream operand | Shadow projection |
|---|---|---|---|---|---|
| Subsurface loss | `Drainage -> LateralTransfer -> StorageReconciliation` | typed `subsurface_loss_handoff_m` | direct subsurface-loss state and R4B `subsurface_loss_m` input | `subsurface_loss_m` | lane/day `subsurface_loss_m` projection |
| Evapotranspiration | `Evapotranspiration -> StorageReconciliation` | typed `evapotranspiration_handoff_m` | direct aggregate ET state and R4B `evapotranspiration_m` input | `evapotranspiration_m` | lane/day `evapotranspiration_m` projection |
| Snow/frost coupling | `Normalization -> StorageReconciliation` | typed `snow_coupling_handoff_m` | direct signed snow-coupling state and R4B `snow_coupling_m` input | `snow_coupling_m` | lane/day `snow_coupling_m` projection |

## Ordering

The direct executor must run R4E-H after R4D and before R4B. R4A may remain
between R4E-H and R4B because R4B consumes both R4A runoff and the R4E-H
storage-budget operands.

Expected per-lane order after this package:

```text
R3A -> R4C -> R4D -> R4E -> R4F -> R4G -> R4A -> R4B -> R3B
```

## Completeness Gate

R4B storage reconciliation must fail closed unless all of the following shadows
are present in the same direct day frame:

- R4C storage input;
- R4D deep seepage;
- R4E-H subsurface loss;
- R4E-H aggregate ET;
- R4E-H signed snow coupling;
- R4A runoff partition.
