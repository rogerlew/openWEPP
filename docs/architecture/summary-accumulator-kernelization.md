# Summary Accumulator Kernelization

Status: Draft (ARCH10)
Evidence: Ran + Static
Ran evidence:
- `cargo fmt --manifest-path crates/openwepp-summary-accumulator/Cargo.toml --check`
- `cargo clippy --manifest-path crates/openwepp-summary-accumulator/Cargo.toml --all-targets -- -D warnings`
- `cargo test --manifest-path crates/openwepp-summary-accumulator/Cargo.toml`

## Purpose

Define the dedicated summary kernel phase for deterministic rollup accumulation
across daily, monthly, yearly, and end-of-simulation windows.

Implementation path:
- `/home/workdir/openWEPP/crates/openwepp-summary-accumulator/src/lib.rs`

## Kernel Phase Placement

`summary_accumulator` is a standalone kernelized phase downstream of hillslope
and watershed kernel execution. It is explicitly separated from orchestrator
routing logic and sidecar behavior.

## Typed Surface

Core contract types:
- `CalendarDay` (validated calendar key)
- `SummaryScalarSurface` (deterministic symbol->scalar surface)
- `SummaryAccumulator` (stateful daily/monthly/yearly/EOS reducer)
- `SummaryRollup` (window, key, totals, status)

Entry points:
- `accumulate_day(day, delta)`
- `finalize()`

Outputs are typed `SummaryRollup` records with `SimulationStatus` phase fixed to
`summary_accumulator`.

## Deterministic Window Semantics

Transition behavior is deterministic and order-stable:
1. Incoming day equal to active day: accumulate only.
2. Incoming day advances within month/year: emit previous daily window.
3. Incoming day advances to a new month: emit previous daily, then previous monthly.
4. Incoming day advances to a new year: emit previous daily, previous monthly,
   then previous yearly.
5. `finalize()`: emit active daily, active monthly, active yearly, then EOS.

Rollup totals are maintained in `BTreeMap` surfaces to guarantee deterministic
symbol ordering.

## Status Outcomes

| rollup window | message id | status phase |
| --- | --- | --- |
| daily | `SUMACC-DAILY-001` | `summary_accumulator` |
| monthly | `SUMACC-MONTHLY-001` | `summary_accumulator` |
| yearly | `SUMACC-YEARLY-001` | `summary_accumulator` |
| EOS | `SUMACC-EOS-001` | `summary_accumulator` |

## Invalid Input Policy (No Fallback)

Accumulator rejection is explicit and typed for:
- invalid dates
- empty scalar surfaces
- empty or duplicate symbols
- non-finite scalar values (`NaN` / `Inf`)
- non-monotonic day sequences
- finalize with no accumulated samples

No default substitution or silent drops are permitted.
