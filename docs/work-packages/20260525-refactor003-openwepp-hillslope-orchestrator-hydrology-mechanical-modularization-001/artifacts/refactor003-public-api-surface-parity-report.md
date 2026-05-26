# REFACTOR003 Public API Surface Parity Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Hydrology module wiring in
`crates/openwepp-hillslope-orchestrator/src/lib.rs` remains:
- `mod hydrology;`
- `pub use hydrology::{ HillslopeHydrologyRoutingError, Wb11HydrologyKernel, Wb11HydrologyKernelGuardError };`

No `pub` symbol removals or renames were introduced in the hydrology surface.
Public exports continue to come from module `hydrology` and preserve prior API
shape for crate consumers and integration tests.

## Ran
Compatibility evidence:
1. `cargo test -p openwepp-hillslope-orchestrator`
   - result: pass
2. `cargo test --workspace`
   - result: pass
