# REFACTOR014 owned file manifest

Status: complete
Evidence mode: Static + Ran

## Ownership
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/mod.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/types.rs`

## Scope coverage
- Static: this package moved kernel internals under `lib_mod` and introduced a `kernel` module folder.
- Ran: 43 package tests pass in `openwepp-watershed-orchestrator`.
