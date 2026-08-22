# Carrier operand source map

Status: `PASS / source-level map established; call-site proof pending`.

| Carrier operand | Producer/owner | Runtime field/source | Consumer boundary |
| --- | --- | --- | --- |
| reference temperature/humidity/pressure | sealed half-hour provider | `LandSurfaceForcing` and V11 atmospheric forcing | covered stack input join |
| exposure wind, transfer height, roughness | sealed Stage-3 exposure receipt | `SealedExposureReceipt` (`5 m`, `0.005 m`) | carrier validation |
| canopy temperatures/humidity | current V11 state | occupancy leaf/stem/wet-surface and tile canopy-air fields | carrier construction from staged V11 beginning |
| canopy conductances | current V11 owner/configuration | V11 geometry and owner-specific boundary conductance | carrier construction; never runner input |
| snow temperature/humidity | current Stage-3 state + sealed atmosphere | current Stage-3 layer thermal state and phase humidity | carrier construction |
| snow mass/liquid/cold-content | current Stage-3 owner | `DirectSnowStage3PersistentState` and current support forcing | Stage-3 evaluator and carrier ledger |
| canopy longwave components | current V11 canopy state | ordered V11 leaf/stem component temperatures/weights | reciprocal longwave evaluation |
| support receipts | coupled-time/provider | accepted `TimeSupport`, participant receipts, forcing digest | carrier/parent join |
| mass/vapor/energy ledgers | Stage-3 + carrier transaction | Stage-3 result and current carrier receipt; carrier input still assembles a pre-execution ledger | complete independent outcome-ledger validation remains open |

The runner may seal atmospheric and identity inputs but may not supply ending
owners, event time, terminal parcel, or live conductance. The covered stack
derives those from committed beginning owners and the accepted support.
