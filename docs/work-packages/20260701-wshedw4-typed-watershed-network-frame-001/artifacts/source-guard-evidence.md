# Source Guard Evidence

Status: `EXECUTED-HOLD`

Evidence class: `Static:` plus `Ran:`

W4 source guards were added for the real public watershed CLI handoff and the
new typed frame boundary.

## Guard Implemented

`crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` now contains
`wshedw4_public_cli_handoff_uses_typed_network_and_publication_frames`.

The guard requires the public CLI source to contain:

- `WatershedNetworkFrame::from_parsed_inputs`
- `network_frame.add_hillslope_contribution`
- `compatibility_writeback_surface`
- `harvest_compatibility_routing_report`
- `publication_frame_to_row_seed`

The guard rejects direct old-surface markers in
`crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`:

- `BoundarySymbol`
- `BoundaryValue`
- `WatershedWritebackSurface`
- `build_watershed_output_row_seed`
- `build_default_chaninp_surface`
- `.writeback_surface`
- `state_surface.insert`
- `flux_surface.insert`

The guard also requires
`crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` to define
`WatershedNetworkFrame`, `WatershedPublicationFrame`,
`HillslopeContribution`, and the explicitly named
`compatibility_writeback_surface` migration edge.

## Ran

```text
cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw4 -- --nocapture
```

Result: `PASS`, `1` test passed.

```text
cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture
```

Result: `PASS`, `24` tests passed.

## Static Scan

`rg` confirms the public CLI now contains the typed frame handoff and no longer
imports or directly writes `BoundarySymbol`, `BoundaryValue`, or
`WatershedWritebackSurface`.

Remaining old-surface reads/writes are still present in:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`:
  explicit `compatibility_writeback_surface` projection and compatibility
  result harvest from `WatershedKernelExecutionReport`.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs`:
  production scheduler still invokes kernels through `WatershedWritebackSurface`
  and applies `KernelWritebackPayload` into symbol maps.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/**`:
  WS10/WS11/WS12/WS18/WS20 routing helpers still use `BoundarySymbol` and
  symbol-keyed request/writeback surfaces.

## Disposition

The source guard proves the public CLI no longer directly constructs or
publishes from the old map surface. It does **not** prove W4 complete because
the routed kernel loop still uses a named compatibility projection into
`WatershedWritebackSurface`. This is the W4 hold blocker.
