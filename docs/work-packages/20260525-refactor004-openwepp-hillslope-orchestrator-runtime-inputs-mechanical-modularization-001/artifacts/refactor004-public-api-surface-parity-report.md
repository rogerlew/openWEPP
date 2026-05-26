# REFACTOR004 Public API Surface Parity Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Runtime-inputs module wiring in
`crates/openwepp-hillslope-orchestrator/src/lib.rs` remains:
- `pub mod runtime_inputs;`

No `pub` symbol removals or renames were introduced in the runtime-inputs
surface. Public exports continue to come from module `runtime_inputs` and
preserve prior API shape for crate consumers and integration tests.

## Ran
Compatibility evidence:
1. `cargo test -p openwepp-hillslope-orchestrator`
   - result: pass
2. `cargo test --workspace`
   - result: pass
