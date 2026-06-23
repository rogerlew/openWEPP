# Performance Evidence

Status: blocked-by-R7F.

## H2637 Same-Binary Runs

- Compatibility default: not run in this package.
- Direct default: not run in this package.
- Explicit direct: not run in this package.
- Rollback compatibility: not run in this package.

## Metrics To Record

- Wall seconds.
- RSS.
- OFE-day count.
- us/OFE-day.
- Legacy multiplier.
- Output checksums/parity status.

## Profile And Remediation Ledger

- Static blocker before profiling: production direct still invokes the
  interleaved `DirectPublicationDayInputBuilder`, which constructs and merges
  `HillslopeWritebackSurface` seed/context surfaces inside the day/OFE loop.
- Ran: focused R7 tests prove the direct manifest compatibility-edge counter
  now reports that edge instead of falsely reporting zero.

## Disposition

R7G performance closure is invalid until
`HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE` is lifted.
The next benchmark package must run same-binary H2637 only after production
direct day-input synthesis is typed and no-compatibility clean, or run it
explicitly as blocker evidence for the replacement work.
