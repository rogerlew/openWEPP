# R6F No-Compatibility Proof

Status: scaffolded.

This artifact proves `DirectPublicationFrameCutover` does not use compatibility
authority for public output construction.

## Forbidden Authority Sources

The cutover path must not read these as direct publication authority:

- compatibility WB13 rows;
- compatibility runtime surfaces;
- writeback payloads;
- stale logical state;
- skeleton direct frame capture;
- output writer self-consistency values;
- wrappers around compatibility structures with direct names.

## Static Scans

| Date | Command | Scope | Result | Notes |
|---|---|---|---|---|
| Pending | Pending | Pending | Pending |  |

## Runtime Counters

| Date | Command | Counter | Expected | Observed | Result |
|---|---|---|---|---|---|
| Pending | Pending | Skeleton-run counter | 0 | Pending | Pending |
| Pending | Pending | Compatibility-edge counter | 0 | Pending | Pending |
| Pending | Pending | Direct frame counter | >0 | Pending | Pending |
| Pending | Pending | Direct executor counter | >0 | Pending | Pending |
| Pending | Pending | Shadow projection counter | >0 | Pending | Pending |

## Anti-Alias Probe

Record a fixture or targeted test proving direct publication changes when the
typed direct operand changes and does not silently mirror compatibility output.

| Probe | Mutated direct operand | Expected effect | Observed effect | Result |
|---|---|---|---|---|
| Pending | Pending | Pending | Pending | Pending |

## Conclusion

Pending execution.
