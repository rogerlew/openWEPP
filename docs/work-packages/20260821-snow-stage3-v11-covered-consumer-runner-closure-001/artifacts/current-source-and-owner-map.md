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

`Static:` `CoveredColumnAuthority::V11SnowCovered` selects an explicit LSE
lower-boundary branch. It suppresses covered ground water, sensible/vapor,
soil-storage, and WB14-facing operators while keeping the snow-free branch
unchanged. The live path now consumes destination-keyed physical boundaries,
includes Stage-3 shortwave, and closes the bounded covered fixed point using
the accepted current-trial boundary receipts. Persistent-support closure is
still blocked on precipitation mass/advection custody, the heterogeneous
snow--soil conductive-heat transaction, and an independent postcandidate
physical outcome ledger. Those remaining owners, rather than keyed
consumption, shortwave, or fixed-point availability, define the next
implementation checkpoint.
