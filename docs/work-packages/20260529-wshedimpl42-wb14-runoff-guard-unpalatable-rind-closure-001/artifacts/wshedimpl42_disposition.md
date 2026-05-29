# WSHEDIMPL42 Disposition

Status: completed  
Evidence mode: Static + Ran

## Decision
- `HOLD`

## What Closed
- WB14 runoff guard blocker (`HKERNEL-WB14-RUNOFF-E-003`) is closed.
- Evidence:
  - pre-fix unpalatable-rind probe showed WB14 failure on all hillslopes.
  - post-fix rerun at `/tmp/wshedimpl42_unpalatable_20260529T143937Z`
    completed `39/39` hillslope executions.

## Why HOLD Remains
1. Watershed intake blocks on impoundment parser domain guard:
   - `CLIWAT-E-010` / `IMP-E-004` (`jpond=0` in `pw0.imp`).
2. After impoundment workaround, watershed intake blocks on pass format:
   - `CLIWAT-E-017` / `HBP-E-002` (`H*.hbp` are ASCII pass text, not binary
     HBP shards).

## Closure Condition Status
- Required closure condition from package objective:
  - successful watershed rerun with parquet emission.
- Status:
  - not achieved.

## Follow-On Requirement
- Open a follow-on package to close watershed intake parity gaps:
  - compatibility path for no-impoundment `pw0.imp` legacy shape (`jpond=0`)
    or equivalent contract-authoritative intake adaptation;
  - production path for binary HBP shard publication from hillslope run outputs
    (or authoritative watershed intake adapter for current pass surface).
