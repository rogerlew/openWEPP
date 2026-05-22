# Climate Parser-to-Runtime Seam Integration Evidence

Evidence mode: `Ran`
Status: `complete`

## Integration Closure Tests

1. `climate_parser_to_hillslope_runtime_surface_closure`
- Parses climate fixture via climate parser contract.
- Adapts parser output through hillslope climate seam.
- Executes hillslope scheduler+kernel path.
- Kernel probe verifies climate symbols are present in immutable request views.

2. `climate_parser_to_watershed_runtime_surface_closure`
- Parses climate fixture and materializes per-hillslope assignment map.
- Adapts assignments through watershed climate seam.
- Executes watershed dispatch+kernel path.
- Kernel probe verifies per-hillslope climate symbols are present in immutable dispatch request views.

## Seam Guard Tests (Adapter Unit Scope)

1. Hillslope seam guard tests:
- `climate_runtime_surface_supports_explicit_datver_zero_override`
- `climate_runtime_surface_rejects_pre4_nonzero_datver_branch`
- `climate_runtime_surface_rejects_duplicate_breakpoint_times`

2. Watershed seam guard tests:
- `climate_runtime_surface_supports_explicit_datver_zero_override`
- `climate_runtime_surface_rejects_pre4_nonzero_datver_branch`
- `climate_runtime_surface_rejects_duplicate_breakpoint_times`

## Executed Commands
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

All commands completed successfully in this execution.

## Evidence
- [DIRECT] `tests/integration/parser_runtime_seam_integration.rs:197`
- [DIRECT] `tests/integration/parser_runtime_seam_integration.rs:223`
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:866`
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:881`
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:916`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:954`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:970`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:986`
