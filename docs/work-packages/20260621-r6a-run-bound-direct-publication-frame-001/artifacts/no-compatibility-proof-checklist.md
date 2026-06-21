# No-Compatibility Proof Checklist

Status: partial.
Evidence mode: Static + Ran.

Forbidden direct publication reads:

- `SimulationOwnedWb13Row`
- `HillslopeWritebackSurface`
- `BoundarySymbol`
- `BoundaryValue`
- `KernelWritebackPayload`
- `SymbolRegistry`
- `HotSymbolTables`
- `IndexedWritebackSurface`
- `state_value_for_symbol`
- `flux_value_for_symbol`
- stale logical output frames
- diagnostic compatibility ledgers as authority

Required proof:

- source scan over direct publication frame constructors and direct output
  projection consumers;
- focused tests proving compatibility rows/surfaces cannot be passed to direct
  projection APIs;
- runtime/default-disabled counter proof that compatibility mode constructs no
  direct publication frame;
- consumer-path scan answering "what still reads the old path?" before closure.

## R6A Evidence

Source scans:

```bash
nl -ba crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs \
  | sed -n '720,930p' \
  | rg -n "SimulationOwnedWb13Row|HillslopeWritebackSurface|BoundarySymbol|BoundaryValue|KernelWritebackPayload|SymbolRegistry|runtime_surface|wb13_rows|\\bpass_rows\\b|execution\\.pass_rows|execution\\.wb13_rows"
```

Result: no matches.

```bash
nl -ba crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  | sed -n '2000,2065p' \
  | rg -n "SimulationOwnedWb13Row|HillslopeWritebackSurface|BoundarySymbol|BoundaryValue|KernelWritebackPayload|SymbolRegistry|runtime_surface|wb13_rows|\\bpass_rows\\b|execution\\.pass_rows|execution\\.wb13_rows"
```

Result: no matches.

Runtime/default-disabled proof:

- `r2a_default_fixture_run_constructs_no_direct_runtime_skeleton` now asserts
  `publication_capture_runs = 0`.
- `r6a_direct_publication_frame_shadow_runs_without_skeleton_counter` asserts
  `publication_capture_runs = 1`, `skeleton_runs = 0`, and
  `compatibility_edge_invocations = 0` for the opt-in direct publication frame
  path.

Residual old-path reads:

- Public production writers still call compatibility HBP/WAT/PASS/loss/manifest
  paths. This is intentional R6A scope control; R6A supplies the missing direct
  frame and direct consumers required before R6 writer cutover.
