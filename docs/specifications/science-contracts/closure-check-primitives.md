# Closure Check Primitives

Status: Draft (ARCH03)
Evidence: Static
Ran evidence: none

## Purpose

Define typed closure/invariant primitives for simulation contracts so closure and
invariant failures are explicit and machine-classified.

Implementation path:
`crates/openwepp-sim-contract/src/closure.rs`.

## Violation Surface

`ClosureViolation` fields:

| field | type | semantics |
| --- | --- | --- |
| `check_id` | `String` | Stable invariant/check ID (for example `INV-WATBAL-001`). |
| `message_id` | `String` | Stable error message ID for status routing. |
| `subject` | `String` | Field/state surface under validation. |
| `kind` | `ClosureViolationKind` | Machine-readable violation type. |
| `severity` | `ClosureSeverity` | Gate severity (`HardFail` or `GovernanceHold`). |
| `details` | `ClosureViolationDetails` | Structured numeric diagnostics. |

## Primitive Set

| primitive | behavior on failure | kind |
| --- | --- | --- |
| `check_finite` | rejects non-finite scalar | `NonFinite` |
| `check_min` | rejects scalar below lower bound | `DomainLowerBound` |
| `check_max` | rejects scalar above upper bound | `DomainUpperBound` |
| `check_range` | rejects scalar outside `[min, max]` | `DomainRange` |
| `check_unit_interval` | range check for `[0, 1]` | `DomainRange` |
| `check_balance_residual` | rejects conservation residual above tolerance | `ResidualExceeded` |
| `check_equal_count` | rejects expected/observed count mismatch | `CardinalityMismatch` |

## Closure Residual Convention

`check_balance_residual` computes:

`residual = inputs_total - outputs_total - storage_delta`

Pass condition:

`abs(residual) <= tolerance`

Violation payload includes all components (`inputs_total`, `outputs_total`,
`storage_delta`, `residual`, `tolerance`) for deterministic diagnostics.

## Policy

- No implicit clamping or default-value fallback is performed by primitives.
- Primitive failures are returned as typed `ClosureViolation` values.
- Orchestrators decide stop/propagation behavior from typed violation output.

## ARCH03 Test Linkage

Covered by:
- `tests/integration/sim_contract_closure_checks.rs`
