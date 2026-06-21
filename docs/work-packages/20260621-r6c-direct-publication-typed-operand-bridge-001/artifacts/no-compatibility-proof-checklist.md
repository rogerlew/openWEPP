# No-Compatibility Proof Checklist

Evidence mode: Static.

Accepted direct publication consumers must not read:

- `SimulationOwnedWb13Row`;
- compatibility WB13 row slices;
- `HillslopeWritebackSurface`;
- `BoundarySymbol` / `BoundaryValue`;
- `KernelWritebackPayload`;
- `SymbolRegistry` hot tables;
- stale logical state;
- wrappers around any of the above.

Execution must record source scans and focused tests before closure.

## Execution Evidence

Ran:

- `rg -n "DirectPublicationFrameCutover|DirectRunFrame::skeleton|direct_publication_typed_bridge_blocked|run_publication_capture" crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`

Relevant result:

- `DirectPublicationFrameCutover` returns
  `direct_publication_typed_bridge_blocked()` before the
  `DirectRunFrame::skeleton` call in `build_direct_publication_artifacts`.

Ran:

- `rg -n "SimulationOwnedWb13Row|HillslopeWritebackSurface|KernelWritebackPayload|BoundarySymbol|SymbolRegistry" crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs | head -120`

Result:

- Compatibility publication surfaces remain present in compatibility writer
  helpers and scheduler lifecycle code.
- R6C does not accept them as direct cutover authority; cutover fails before any
  direct publication artifact is built from them.
