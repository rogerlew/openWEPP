# simimpl13-continuous-simulation-run-gap-assessment

Status: addendum-complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- This addendum enumerates the remaining blockers for an actual continuous
  hillslope simulation run (multi-day forcing progression with state carry and
  trajectory publication), beyond one-day projection-style execution.
- Continuous-run target for this package:
  - iterate all available climate daily forcing rows in order,
  - execute scheduler/kernel lifecycle per timestep with carried forward state,
  - publish trajectory outputs with monotonic day indexing and replay-comparable
    key semantics.

## Ran
- Runner execution-path evidence inspected:
  - `crates/openwepp-runner/src/lib.rs:1679`
  - `crates/openwepp-runner/src/lib.rs:1716`
  - `crates/openwepp-runner/src/lib.rs:2772`
  - `crates/openwepp-runner/src/lib.rs:2801`
  - `crates/openwepp-runner/src/lib.rs:2813`
  - `crates/openwepp-runner/src/lib.rs:2845`
  - `crates/openwepp-runner/src/lib.rs:3044`
  - `crates/openwepp-runner/src/lib.rs:3206`
  - `crates/openwepp-runner/src/lib.rs:3227`
- Orchestrator runtime-input seam evidence inspected:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1992`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2145`
- Fixture evidence inspected:
  - `tests/fixtures/cli01/hillslope_run_dir/case.cli` includes 2 daily records.

## Continuous-run gap register
| gap_id | blocker statement | impact | closure requirement | status |
|---|---|---|---|---|
| `SIMIMPL13-CONT-001` | Runner always seeds climate runtime surface with `day_index=0` (`build_hillslope_runtime_surface_from_climate(&climate, 0)`). | Forcing never advances past the first climate day. | Add a day-indexed execution driver that iterates daily forcing records and reseeds climate symbols per timestep. | open |
| `SIMIMPL13-CONT-002` | Runner executes scheduler/kernel lifecycle exactly once per run. | No multi-day progression occurs even when climate contains multiple daily records. | Execute lifecycle across the full daily forcing span and carry runtime writeback state between iterations. | open |
| `SIMIMPL13-CONT-003` | `RunnerDailyPhaseKernel` returns `KernelWritebackPayload::empty()` for every phase. | Kernel path does not advance process state; trajectory remains projection-shaped instead of physics-driven continuity. | Replace no-op kernel with production phase-kernel wiring that returns finite, contract-valid writeback updates. | open |
| `SIMIMPL13-CONT-004` | WB13/H.wat publication path consumes one row only (`append_row` once, `vec![single_row]`), with `sim_day_index` hard-coded to `1`. | Output cannot represent a continuous trajectory; downstream replay span is collapsed. | Publish one row per executed timestep with monotonic `sim_day_index` and full-span dat/parquet parity. | open |
| `SIMIMPL13-CONT-005` | Current row-key year uses calendar year from runtime symbols (`from_surface(..., year, ...)`) while replay comparator baseline uses simulation-year key domain. | Prevents key-domain overlap needed for promotable parity and continuous replay comparability. | Implement explicit candidate key policy mapping (calendar/simulation year contract) and enforce with preflight gates/tests. | open |
| `SIMIMPL13-CONT-006` | Loss and optional text outputs are first-day projections (`first_day_projection`) rather than run-span summaries. | Auxiliary outputs are not continuous-run truthful and cannot validate end-to-end multi-day behavior. | Emit run-span-aware auxiliary outputs or explicitly scope/defer unsupported surfaces with contract-backed gating. | open |
| `SIMIMPL13-CONT-007` | Run manifest records provenance but omits trajectory continuity assertions (executed day count, first/last day, monotonic day index checks). | Regression gates cannot automatically detect continuity collapse after refactors. | Extend manifest/test gates with explicit continuity metrics and fail conditions. | open |

## Priority order
1. `SIMIMPL13-CONT-001`
2. `SIMIMPL13-CONT-002`
3. `SIMIMPL13-CONT-003`
4. `SIMIMPL13-CONT-004`
5. `SIMIMPL13-CONT-005`
6. `SIMIMPL13-CONT-006`
7. `SIMIMPL13-CONT-007`

## Addendum conclusion
- Current runner/orchestrator integration is not yet a continuous simulation
  implementation; it is a single-day lifecycle execution and publication path.
- Closure requires both execution-lifecycle continuity (forcing progression +
  carried state) and publication continuity (trajectory rows + continuity
  assertions) before parity reruns can be interpreted as continuous-run
  evidence.
