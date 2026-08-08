# Operand Lineage

Status: complete

Evidence mode: Static + Ran

| Operand | Units | Authority/custody | Rejected alias |
|---|---:|---|---|
| carried layers | m, kg/m3, C, J/m2 | private evaluator snapshot | production CoE state |
| hourly snowfall | m precipitation, converted by 0.1 SWE ratio to kg/m2 | typed hourly forcing; credited before vapor/melt | rainfall or production accumulation |
| external liquid | m and kg/m2 | typed hourly rain; unresolved liquid ledger | snowfall, ice, or routed production water |
| deposition/sublimation | kg/m2 | bounded Stage 3 vapor transfer | raw vapor opportunity |
| melt | kg/m2 | bounded sequential Stage 3 debit | CoE production melt |
| terminal energy | J/m2 | explicit censored diagnostic | soil or receiving-surface flux |
| support | s | evaluated/requested hourly support | nominal whole-day coverage |

The v7 consumer reconstructs end ice as start + snowfall + deposition -
sublimation - melt and reads complete end-state layers and cumulative ledgers.
