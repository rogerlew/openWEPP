# Public API Surface Parity Report

Static: public symbols retained:

- `pub enum HillslopeRuntimeInputError` remains at
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`.
- `pub const fn code(&self) -> &'static str` remains public.
- `impl fmt::Display for HillslopeRuntimeInputError` remains implemented.

Static: no enum variants, variant fields, public functions, public type aliases,
or public module exports were added, removed, or renamed.

Static: new production helpers are private inherent methods on
`HillslopeRuntimeInputError`.

Static: `08_tests/core_types.rs` adds test-only helpers and one test inside the
existing runtime-input test module.

Disposition: public API parity preserved.
