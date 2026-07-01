# Old Surface Inventory

Status: `EXECUTED-PRE-EDIT`

Evidence class: `Static:`

Pre-edit inventory of current `WatershedWritebackSurface`, `BoundarySymbol`,
and `BoundaryValue` use before W4 production edits.

Commands:

```text
rg -n "WatershedWritebackSurface|BoundarySymbol|BoundaryValue|WatershedKernelExecutionReport|execute_watershed_dispatch_with_kernel|write_watershed_interchange_outputs|PassInventory|latest_event|chan_out" crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs crates/openwepp-runner/src/watershed_supervisor.rs crates/openwepp-watershed-orchestrator/src crates/openwepp-watershed-output/src tests/integration crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs
rg -n "request\.(state_surface|flux_surface)|state_surface\.get|flux_surface\.get|KernelWritebackPayload|WritebackField|BoundarySymbol::from|WatershedProduction(State|Flux)Symbol" crates/openwepp-watershed-orchestrator/src/lib_mod/kernel crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs crates/openwepp-watershed-orchestrator/src/lib_mod/types.rs
```

## Production Routing Read

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` builds
  `runtime_surface` from parsed `chan.inp` or deterministic fallback globals,
  then seeds channel, slope-profile, impoundment, and hillslope pass payload
  values into `runtime_surface.state_surface` / `runtime_surface.flux_surface`.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs` creates
  `WatershedKernelRequest` values from the mutable writeback maps for each
  topology dispatch step.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` reads
  required state/flux scalars from `request.state_surface` and
  `request.flux_surface`; these are production channel/impoundment routing
  reads.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
  reads opt-in toggles and channel state diagnostics from
  `request.state_surface`; these are production routing reads when the routed
  branch is active.

## Production Routing Write

- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs`
  returns routed channel and impoundment state/flux as
  `KernelWritebackPayload` / `WritebackField` values keyed by
  `BoundarySymbol`.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs` applies the
  kernel writeback payload to `WatershedWritebackSurface` after every accepted
  dispatch step.

## Production Publication Read

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
  `build_watershed_output_row_seed` reads `report.writeback_surface` to
  compute protected watershed output row fields.
- Publication currently reads channel runoff volume, impoundment outflow,
  first-channel peak, channel sediment yield, hillslope detachment, and
  baseflow from symbol-map state/flux surfaces before calling the watershed
  output writer.

## Compatibility Projection

- Existing parser-to-runtime helpers in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs`
  project parsed `chan.inp`, watershed channel, slope-profile, and impoundment
  inputs into symbol maps. In W4 these may only remain as explicit compatibility
  projection edges unless the package closes in `HOLD`.
- Existing `WatershedKernelExecutionReport` carries the final
  `WatershedWritebackSurface`. W4 may read that report only as a named
  compatibility result-harvest edge until typed kernel writeback is complete.

## Replay / Comparator / Diagnostic Edge

- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
  emits symbol-keyed diagnostics used by existing watershed routing physics
  tests. These remain legitimate diagnostic/comparator surfaces if they are not
  part of the production W4 routing/publication claim.
- Integration tests under `tests/integration/ws11_*`, `ws12_*`, `ws18_*`,
  `ws20_*`, and related routed-physics contracts assert current
  symbol-surface behavior. They are protected science coverage until migrated
  or backfilled by typed-frame tests.

## Test-Only Protected Behavior

- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/tests.rs`
  asserts parser-to-runtime projection values and fail-closed guards. These
  tests protect input parsing and runtime projection semantics; they should be
  migrated to typed frame assertions before any old-surface deletion claim.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` protects
  public CLI behavior, W2/W3 supervisor fail-closed semantics, and protected
  output identity. These tests remain protected user-facing coverage.

## Obsolete-Internal Test Candidates

- Tests that assert only map-key spelling or `WatershedWritebackSurface`
  existence, without protecting parser semantics, route physics, guard posture,
  output schema, or public CLI behavior, are W5 deletion candidates. None were
  deleted before W4 implementation.

## Pre-Edit Disposition

The current production routing and production publication path depends on the
old symbol-map surface. W4 cannot close complete until those reads/writes are
removed from the real public runner path, or it must close `EXECUTED-HOLD` with
the remaining old-surface dependency named.
