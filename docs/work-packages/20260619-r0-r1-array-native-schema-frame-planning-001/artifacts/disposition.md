# R0/R1 Disposition

Status: COMPLETE-PLANNING-ONLY.
Evidence mode: Static/Ran.

## Verdict

`COMPLETE-PLANNING-ONLY`.

The package scaffolded and executed the R0/R1 planning work allowed while
PERFDEEP07 remains in `HOLD`. It produced the schema envelope, type-boundary
decision, constructor/projection plan, publication ledger promotion plan,
no-compatibility proof plan, hold-lift disposition, gate results, review, and
verification artifacts.

## Findings

| Finding | Disposition | Evidence |
|---|---|---|
| R0/R1 can proceed only as planning while PERFDEEP07 is held. | accepted / enforced | Package scope and all planning artifacts block R2+ implementation. |
| Existing `HillslopeDayFrame` is not the future direct runtime frame. | accepted / documented | `direct-frame-type-boundary-decision.md`. |
| PERFDEEP06 publication ledger is seed evidence, not cutover authority. | accepted / documented | `publication-ledger-promotion-plan.md`. |
| Future direct runtime readiness needs an executable no-compatibility proof. | accepted / documented | `no-compatibility-proof-plan.md`. |
| Direct-frame implementation can proceed now. | rejected / blocked | PERFDEEP07 P0 remains unresolved; R2+ work is blocked. |

## Closure State

R0/R1 planning is closed. R2+ runtime implementation remains blocked until the
PERFDEEP07 disabled-path hold is closed or explicitly superseded by a ratified
architecture/performance decision.
