# Default-Disabled Regression Gate

Static: planned. Ran evidence must be recorded before closure.

## Required Evidence

R5C must prove the default compatibility path remains isolated from the opt-in
direct runtime:

- runner default fixture records zero direct-runtime counters;
- `git diff` shows no default scheduler/API activation;
- release H2637 default-disabled median is `<= 676.67 s` across three reps with
  direct-runtime and diagnostic env vars unset.

## H2637 Command Family

Static: use the command family from
`docs/work-packages/r5-burndown-execplan.md`, replacing `r5x` with `r5c` and
recording elapsed seconds/RSS for all three reps.

## Ran Evidence

Ran: runner default fixture
`r2a_default_fixture_run_constructs_no_direct_runtime_skeleton` passed and
recorded zero direct-runtime counters.

Ran: scheduler/API diff review was empty for:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/api.rs`

Ran: H2637 default-disabled release reps with direct-runtime and diagnostic env
vars unset:

| Rep | Seconds | RSS KB | Warning |
|---:|---:|---:|---|
| 1 | 639.05 | 228348 | `MOFE01-MG-W-001` |
| 2 | 646.33 | 228100 | `MOFE01-MG-W-001` |
| 3 | 643.96 | 228840 | `MOFE01-MG-W-001` |

Median: `643.96 s`.

Gate: PASS, median `643.96 s <= 676.67 s`.
