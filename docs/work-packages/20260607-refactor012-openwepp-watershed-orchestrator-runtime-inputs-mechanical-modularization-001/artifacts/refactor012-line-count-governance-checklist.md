# REFACTOR012 refactor012 line count governance checklist

Status: complete  
Evidence mode: Static: completed; Ran: completed

## Current line counts
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`: 18
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`: 1997
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/climate.rs`: 658
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/mod.rs`: 6
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/tests.rs`: 1192
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`: 513

## Governance checks
- files >=2000 lines: none
- files >=3000 lines: none
- decomposition rationale: pre-refactor monolith exceeded 3000 lines (~4330, per package objective).
- post-refactor split keeps all runtime-input `.rs` files below the 2000-line threshold.
- decomposition owner and sunset: complete in this package under `REFACTOR012`.
