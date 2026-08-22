# Current source and owner map

Status: `IN PROGRESS`.

The current covered consumer path is:

```text
sealed covered segment input
  -> DirectV11SnowCoveredRealConsumerStack
  -> shared Child-2C carrier receipt
  -> actual Stage-3 persistent support
  -> common V11 resource/owner finalization
  -> canonical V11 "snow" owner plus complete owner set
```

The existing `DirectV11RealConsumerStack` remains the snow-free adopter and
rejects snow-present operands. The runner/provider still owns only the sealed
capability bind; it does not yet construct the covered physical support from
staged owners. Terminal parcel installation, restart, and publication
consumers remain open.
