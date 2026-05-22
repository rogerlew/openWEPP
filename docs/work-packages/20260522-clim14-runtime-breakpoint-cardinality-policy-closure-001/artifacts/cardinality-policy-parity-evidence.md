# Cardinality Policy Parity Evidence (CLIM14)

Evidence mode: `Static + Ran`
Status: `collected`

## Static
1. Shared runtime adaptation now contains explicit cardinality policy enforcement (`<=1500`) independent of parser mode.
2. Parser compatibility override behavior is explicitly codified as parse-time only; runtime policy remains strict.
3. Watershed contextual seam preserves shared policy failure semantics while attaching `hillslope_id`.

## Ran
Commands executed:
1. `cargo test -p openwepp-climate-runtime-adapter --lib`
2. `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::`
3. `cargo test -p openwepp-watershed-orchestrator runtime_inputs::tests::`
4. `cargo fmt --check`
5. `cargo clippy --workspace --all-targets -- -D warnings`
6. `cargo test --workspace`
7. `cargo deny check`

All commands above completed successfully.

## Targeted Test Evidence
1. Shared runtime seam:
- `runtime_request_accepts_breakpoint_cardinality_at_1500_boundary`
- `runtime_request_rejects_breakpoint_cardinality_over_1500_even_with_parser_override`
- `runtime_request_rejects_declared_cardinality_over_1500_when_rows_are_truncated`

2. Hillslope runtime seam:
- `climate_runtime_surface_accepts_breakpoint_cardinality_at_1500_boundary`
- `climate_runtime_surface_rejects_breakpoint_cardinality_over_1500_even_with_parser_override`

3. Watershed runtime seam:
- `climate_runtime_surface_accepts_breakpoint_cardinality_at_1500_boundary`
- `climate_runtime_surface_rejects_breakpoint_cardinality_over_1500_even_with_parser_override`

## Closure Claim
The strict/override runtime cardinality policy contract is closed with explicit implementation and passing boundary-path coverage at shared, hillslope, and watershed seams.
