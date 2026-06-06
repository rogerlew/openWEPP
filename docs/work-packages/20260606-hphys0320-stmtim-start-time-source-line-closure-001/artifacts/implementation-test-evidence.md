# Implementation and Test Evidence

Status: complete

Evidence mode: Ran

Ran:

Production implementation:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  - Applies `simimpl28_stmtim_start_time(wnttim)?` before storm-duration
    filtering and active-interval evaluation.
  - Fails closed on non-finite `wnttim` with
    `ClimateRuntimeInputError::NonFiniteField`.
  - Normalizes only the legacy-authorized `wnttim < 1.0` start-time lower
    bound.

Focused runtime tests:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - `hphys0320_stmtim_normalizes_zero_start_before_active_interval`
  - `hphys0320_stmtim_nonfinite_start_time_fails_closed`

Command:

```sh
cargo test -p openwepp-hillslope-orchestrator hphys0320
```

Result:

- Passed.
- `2` tests passed.
