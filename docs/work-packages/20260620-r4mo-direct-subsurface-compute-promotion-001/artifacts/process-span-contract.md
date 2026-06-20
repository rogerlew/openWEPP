# Process Span Contract

Status: pre-implementation.

Static: R4M/O adds direct spans with the same direct-runtime contract used by
R3/R4 predecessors: typed inputs, direct compute, direct state mutation,
downstream operands, and shadow projection.

| Span | Phase Path | Inputs | Compute | Mutation | Downstream | Shadow |
|---|---|---|---|---|---|---|
| R4M WB18 percolation | `PercolationDeepSeepage -> StorageReconciliation` | typed WB18 layer state, soil-water ledger, same-pass infiltration, lane config | request-free percolation routing | layer storage, `D`, `Pe`, R4B `deep_seepage_m` | `D`, `Pe`, per-layer flux | direct WB18 projection |
| R4O WB19 subsurface | `Drainage -> LateralTransfer -> StorageReconciliation` | typed WB19 layer state, geometry, branch config, upstream direct `Pe` | request-free drainage/lateral withdrawal | layer storage, `q`, `Qdd`, `Qd`, R4B `subsurface_loss_m` | `q`, `Qdd`, `Qd`, carry diagnostics | direct WB19 projection |

Static: R4B must fail closed unless R4M and R4O shadows exist. R4D/R4E-H
handoff inputs may remain as legacy scaffold fields but must no longer be the
authoritative R4B source in the direct executor path.
