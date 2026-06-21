# R6H Operand Lineage

Status: queued.

| Output field | Direct operand | Producer | Consumer | Compatibility alias rejected | Evidence |
|---|---|---|---|---|---|
| `Es` | Queued | Queued | Direct WAT row builder | WB13 `Es`, compatibility ET runtime symbols | Queued |
| `Total-Soil` | Queued | Queued | Direct hydrology projection, then WAT row builder | WB13 storage columns, stale logical profile totals | Queued |
| `SoilWaterTotal` | Queued | Queued | Direct hydrology projection, then WAT row builder | WB13 storage columns, stale logical profile totals | Queued |
| `wepp_id` | Queued | Direct WAT id authority | Direct WAT row builder | WB13 row identity or fixture-only constant | Queued |

## Lineage Requirements

- Units and normalization basis must be recorded for every WAT operand touched.
- PMET operands must point to direct-carried state after prior-day commit.
- Any private seed-surface symbol used by the direct builder must appear in the
  allowlisted no-compatibility ledger.
