# Implementation

Evidence label: Static/Ran.

Status: `IMPLEMENTED`

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`

Static:

- Refactored `run_direct_channel_node` into private setup, hydrology, runon,
  peak, runoff, and output helpers.
- Split WS11 peak branch execution into branch-specific helpers while
  preserving the existing ipeak dispatch, formulas, and wave-state assignment.
- Refactored `assemble_direct_incoming_peak_partition` into whole-inlet hourly
  authority, hillslope runon, dependency runon, checked-volume, and incoming
  total validation helpers.
- Refactored `run_direct_impoundment_node` into private context, integration
  horizon, outflow, and output helpers.
- Refactored WS19 sediment assembly into private accumulator, active-class,
  WS20 routing, sediment-rate, publication, terminal-hydraulic, and transport
  capacity helpers.
- Added private characterization tests for the helper surfaces touched by the
  channel, impoundment, runon, and sediment extraction.
- Split package-local tests into
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct_tests.rs`
  and kept `direct.rs` as the production include plus a `#[cfg(test)]`
  `include!("direct_tests.rs")` hook.

Behavior-preserving constraints maintained:

- Existing guard labels and typed fail-closed classes were preserved.
- Existing floating-point formulas and accumulation order were preserved,
  including hillslope sediment before dependency-channel sediment and WS20
  routing conditionality.
- No public output schema, runtime-symbol, threshold, tolerance, or science
  contract authority was changed.

Ran:

- `cargo fmt --check` - pass.
- `cargo nextest run -p openwepp-watershed-orchestrator` - pass,
  `68 tests run: 68 passed`.
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` - pass,
  `18 tests run: 18 passed`.
- `cargo clippy -p openwepp-watershed-orchestrator --all-targets -- -D warnings` - pass.
