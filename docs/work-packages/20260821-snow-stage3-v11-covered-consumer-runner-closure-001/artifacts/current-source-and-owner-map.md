# Current source and owner map

Status: `IN PROGRESS`.

The current covered consumer path is:

```text
sealed covered segment input
  -> DirectV11SnowCoveredRealConsumerStack
  -> live carrier operands derived from committed V11/Stage-3 owners
  -> per-lane shared Child-2C carrier receipts
  -> exact Stage-3 persistent boundary receipt
  -> actual Stage-3 persistent support
  -> keyed `(OFE, tile)` covered lower-boundary receipt set
  -> common V11 resource/owner finalization
  -> canonical V11 "snow" owner plus complete owner set
```

The existing `DirectV11RealConsumerStack` remains the snow-free adopter and
rejects snow-present operands. The runner/provider still owns only the sealed
capability bind; it does not yet construct the covered physical support from
staged owners. Terminal parcel installation, restart, and publication
consumers remain open.

`Static:` `CoveredColumnAuthority::V11SnowCovered` now selects an explicit LSE
lower-boundary branch. It suppresses covered ground water, sensible/vapor,
soil-storage, and WB14-facing operators, while keeping the snow-free branch
unchanged. Generic ground/shortwave accounting structures remain in the
transaction, and released Stage-3 shortwave, precipitation-advection,
soil-coupling, fixed-point, and independent ledger joins are not yet present.
The current parent-level carrier aggregate is therefore not proof of a fully
keyed heterogeneous covered solve. This remains the next implementation
blocker.
