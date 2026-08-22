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
  -> common V11 resource/owner finalization
  -> canonical V11 "snow" owner plus complete owner set
```

The existing `DirectV11RealConsumerStack` remains the snow-free adopter and
rejects snow-present operands. The runner/provider still owns only the sealed
capability bind; it does not yet construct the covered physical support from
staged owners. Terminal parcel installation, restart, and publication
consumers remain open.

`Static:` The selected covered authority is currently an identity and routing
guard on the generic covered-column solver. The solver still consumes the
legacy ground/litter/mineral lower-surface terms, so the source path does not
yet prove that Stage 3 is the sole V11 lower-surface radiation, sensible, and
vapor owner. This is the next implementation blocker.
