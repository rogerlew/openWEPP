# PL13A Exception Disposition Register

Status: `complete`
Evidence mode: `Static`

## Exception Rows

| exception_id | row class | decision | rationale | owner |
|---|---|---|---|---|
| `PL13A-EX-001` | Scheduler structural metadata symbols (for example `pl_schedule_slot_count`, `pl_schedule_rotation_years`, ordering flags) | `exceptioned` | These symbols encode runtime topology/orchestration metadata, not canonical WEPP science variables; they are intentionally boundary-structural and excluded from canonical alias authority. | openWEPP maintainers |

## Deferred Rows

- No deferred-non-blocking rows remain for PL13A scope.

## Blocking Posture

- `PL09-GAP-007` required closure/explicit exception is satisfied.
- No unresolved alias continuity row remains silently deferred.
