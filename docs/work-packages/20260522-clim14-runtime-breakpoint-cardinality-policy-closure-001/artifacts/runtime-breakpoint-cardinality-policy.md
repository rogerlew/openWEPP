# Runtime Breakpoint Cardinality Policy (CLIM14)

Evidence mode: `Static`
Status: `implemented`

## Policy Closure
1. Runtime seam breakpoint cardinality policy is explicit: `<= 1500` points per breakpoint day.
2. Runtime enforcement is applied in shared adaptation (`openwepp-climate-runtime-adapter`) before projection to hillslope/watershed surfaces.
3. Effective runtime cardinality is evaluated as `max(declared nbrkpt, materialized breakpoint row count)` to avoid implicit acceptance through metadata/row divergence.

## Strict vs Parser-Override Contract
1. Strict parser mode:
- parser enforces `nbrkpt <= 1500`.
- runtime also enforces `<= 1500`.

2. Parser compatibility mode with `allow_breakpoint_cardinality_override=true`:
- parser may accept `nbrkpt > 1500` for controlled investigation.
- runtime remains strict and rejects `>1500` with typed policy failure.

## Error Semantics
1. Shared runtime seam uses `SharedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded { value, max }`.
2. Runtime code remains `CLIM-RUNTIME-E-011` for cardinality-policy failures.
3. Watershed contextual mapping preserves hillslope identity in
`WatershedClimateRuntimeInputError::BreakpointCardinalityPolicyExceeded { hillslope_id, value, max }`.

## Implementation Surfaces
1. `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- Adds runtime `MAX_BREAKPOINTS_PER_DAY`.
- Adds explicit policy error variant and enforcement in breakpoint adaptation path.

2. `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- Adds coverage proving 1500 boundary acceptance and parser-override runtime rejection.

3. `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- Adds shared-error mapping and coverage for watershed assignment/runtime projection path.
