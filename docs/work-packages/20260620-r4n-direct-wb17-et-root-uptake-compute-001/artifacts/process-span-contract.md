# Process Span Contract

Status: pre-implementation.

Static: R4N adds two direct spans with the same direct-runtime contract used by
R3/R4 predecessors: typed inputs, direct compute, direct state mutation,
downstream operands, and shadow projection.

| Span | Phase Path | Inputs | Compute | Mutation | Downstream | Shadow |
|---|---|---|---|---|---|---|
| R4N surface ET | `Evapotranspiration` | typed WB17 layer state from R4M, ET demand, canopy, LAI, residue interception, optional stage/PMET state | request-free residue evaporation, soil evaporation, transpiration-demand seed, stage update | layer storage after soil evaporation, aggregate soil water, component ET seed | `Es`, `Er`, `Etp`, layer state after surface ET | direct WB17 surface projection |
| R4N root uptake | `PlantRootUptake -> StorageReconciliation` | typed layer state from R4O, R4N surface ET seed, root depth, `pltol` | request-free SWU uptake | layer storage after root uptake, aggregate `ET`, `Ep`, `Ws`, R4B `evapotranspiration_m` | final aggregate ET and uptake vectors | direct WB17 final projection |

Static: R4O must consume R4N surface-ET-mutated layer state when present. R4B
must fail closed unless final R4N shadow projection exists. R4F handoff remains
available only as older scaffold and must not be authoritative in aggregate
R4 execution.
