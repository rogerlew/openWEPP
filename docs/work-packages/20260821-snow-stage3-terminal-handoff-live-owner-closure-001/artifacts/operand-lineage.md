Status: partial / EXECUTED HOLD
Evidence mode: Static

| Operand | Source and custody |
|---|---|
| exposure wind/height/roughness | `SealedExposureReceipt`; sealed provider/source and 5 m/0.005 m guards |
| air and longwave forcing | `DirectSnowStage3SealedForcing`; carried in restart |
| canopy/snow transport | staged surface receipts inside sealed forcing; actual V11/Stage-3 consumer join remains missing |
| canopy cover and snow liquid/SWE | live post-span `DirectDayFrame` |
| terminal liquid | retained + post-winter rain + released melt, each depth-to-mass checked |
| WB14 continuation | persistent surface owner, configured topology, live continuation index, configured parameters |
| V11/LSE/hydrology/BGC/thermal owners | intended staged owners; current implementation hashes `Debug` projections with predecessor digest |
| publication | private rows in `DirectPublicationBatchExecution`, released only after commit |

No caller supplies the old carrier/event/ledger/executor DTOs to the ordinary
method, but the configuration still carries event/carrier operands and the
current owner payloads are not canonical typed owner state. This lineage is
therefore not sufficient for closure.
