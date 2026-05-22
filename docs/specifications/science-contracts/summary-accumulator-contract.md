# Summary Accumulator Contract

Status: Draft (ARCH10)
Evidence: Ran + Static
Ran evidence:
- `cargo test --manifest-path crates/openwepp-summary-accumulator/Cargo.toml`

## Purpose

Specify the typed contract for summary accumulation over daily, monthly,
yearly, and end-of-simulation windows.

Implementation path:
- `/home/workdir/openWEPP/crates/openwepp-summary-accumulator/src/lib.rs`

## Contract Inputs

### Temporal key
- `CalendarDay { year, month, day }`
- Validation:
  - `month` must be `1..=12`
  - `day` must match Gregorian month bounds (including leap-year handling)

### Scalar surface
- `SummaryScalarSurface`
- Deterministic symbol-keyed map of finite scalar values.
- Validation:
  - surface must be non-empty
  - symbols must be non-empty
  - symbols must be unique
  - values must be finite

## Contract Outputs

### Per-step outcome
- `SummaryAccumulatorStepOutcome { emitted_rollups }`
- `emitted_rollups` is ordered by deterministic transition rules.

### Rollup record
- `SummaryRollup`
  - `window: SummaryWindow`
  - `key: SummaryWindowKey`
  - `totals: SummaryScalarSurface`
  - `status: SimulationStatus`

Status phase is always `summary_accumulator`.

## Window Transition Rules

Given monotonic incoming days:
1. Same day input -> no window emission.
2. Day boundary crossed -> emit prior `Daily` rollup.
3. Month boundary crossed -> emit prior `Daily`, then prior `Monthly`.
4. Year boundary crossed -> emit prior `Daily`, prior `Monthly`, prior `Yearly`.
5. `finalize()` -> emit active `Daily`, active `Monthly`, active `Yearly`, then `EOS`.

## Deterministic Message-ID Map

| window | message id | status constructor |
| --- | --- | --- |
| `Daily` | `SUMACC-DAILY-001` | `SimulationStatus::ok` |
| `Monthly` | `SUMACC-MONTHLY-001` | `SimulationStatus::ok` |
| `Yearly` | `SUMACC-YEARLY-001` | `SimulationStatus::ok` |
| `EndOfSimulation` | `SUMACC-EOS-001` | `SimulationStatus::ok` |

## Typed Error Surface

`SummaryAccumulatorError` covers:
- `InvalidDate`
- `EmptyScalarSurface`
- `EmptySymbol`
- `DuplicateSymbol`
- `NonFiniteInput`
- `NonMonotonicDate`
- `WindowStateMissing`
- `WindowTotalsMissing`
- `FinalizeWithoutSamples`
- `Status(StatusError)`

## Invariants

- `INV-SUMACC-001`: input scalar surfaces are finite and non-empty.
- `INV-SUMACC-002`: input day sequence is monotonic non-decreasing.
- `INV-SUMACC-003`: rollup emission ordering is deterministic by boundary class.
- `INV-SUMACC-004`: rollup statuses always use phase `summary_accumulator`.
- `INV-SUMACC-005`: EOS totals equal the sum of all accepted daily deltas.
- `INV-SUMACC-006`: invalid inputs are explicit typed errors; no silent fallback.

## Symbol Continuity

The accumulator does not rename symbols. Upstream kernels/orchestrators provide
canonical symbol keys; accumulation preserves those keys verbatim for downstream
contract and comparator routing.
