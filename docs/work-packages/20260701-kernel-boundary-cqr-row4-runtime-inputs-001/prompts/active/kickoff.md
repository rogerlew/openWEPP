# Kickoff

Execute row #4 of `docs/work-packages/kernel-boundary-cqr-burndown-execplan.md`
on `main`.

Target owned files:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/07_series_helpers.rs`

Required closure:

- CRAP-before and CRAP-after evidence using workspace LCOV plus `cargo crap`.
- Focused typed-surface tests for row-relevant deleted contract assertions.
- H2637 identity for behavior preservation.
- Full Rust and authority gates.
- Dual review, dual verification, line-count governance, and final row commit.
