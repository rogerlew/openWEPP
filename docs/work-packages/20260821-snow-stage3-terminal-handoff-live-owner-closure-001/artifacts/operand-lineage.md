Status: complete
Evidence mode: Static

| Operand | Source and custody |
|---|---|
| exposure wind/height/roughness | `SealedExposureReceipt`; sealed provider/source and 5 m/0.005 m guards |
| air and longwave forcing | `DirectSnowStage3SealedForcing`; carried in restart |
| canopy/snow transport | staged surface receipts inside sealed forcing |
| canopy cover and snow liquid/SWE | live post-span `DirectDayFrame` |
| terminal liquid | retained + post-winter rain + released melt, each depth-to-mass checked |
| WB14 continuation | persistent surface owner, configured topology, live continuation index, configured parameters |
| V11/LSE/hydrology/BGC/thermal owners | sequential live day-frame owner projections with predecessor digest |
| publication | private rows in `DirectPublicationBatchExecution`, released only after commit |

No caller supplies a carrier, terminal event, ledger, raw owner bytes, or
owner executor to the ordinary method.
