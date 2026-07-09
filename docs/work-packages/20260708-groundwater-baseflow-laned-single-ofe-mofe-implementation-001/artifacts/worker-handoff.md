# Worker Handoff

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

This package implemented the Lane D groundwater/baseflow recurrence and closed
the generated baseflow (`gwbfv`) WAT consumer path. Do not rerun this package as
if it were still scaffolded.

## Implemented Surfaces

- `DirectGroundwaterAuthority`, `DirectGroundwaterRunState`, and
  `DirectGroundwaterDayOutput`.
- Runner `gwcoeff.txt` sidecar resolution and direct authority conversion.
- Active Lane D daily recharge aggregation from deep percolation.
- Direct WAT nullable `Base` and unit registry metadata.
- Active summary groundwater totals.

## Remaining Follow-On

Create a narrow package for generated groundwater deep-seepage and channel
threshold closure:

1. Choose and contract the real consumer surface for `gwdsv`.
2. Implement the consumer path; do not close on producer-only output.
3. Implement or explicitly defer `bftharea` threshold behavior with watershed
   area lineage.
4. Preserve namespace separation from current soil `Dp`, `latqcc`, generated
   `gwbfv`, and channel `cbase`.

## Verified Gates

See `artifacts/gate-results.md`. Full workspace nextest passed:
`cargo nextest run --workspace --profile full` (1462 passed, 3 skipped).
