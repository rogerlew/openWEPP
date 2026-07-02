# Consumer-Path Evidence

Status: `PASS`

Evidence class: `Static` and `Ran`.

Required proof fields:

- Producer source: public watershed CLI HBP pass inventory.
- Typed network frame object:
  `WatershedNetworkFrame::from_parsed_inputs`.
- Hillslope contribution ingress:
  `network_frame.add_hillslope_contribution`.
- Typed routing dispatch object/function:
  `execute_watershed_dispatch_with_frame(&mut network_frame, ...)`.
- Typed routed-state output:
  `record_routed_channel_state` and `record_routed_impoundment_state`.
- Typed publication frame object:
  `publish_typed_routing_report`.
- Runner handoff:
  public CLI passes the typed publication frame to `publication_frame_to_row_seed`.
- Downstream routing consumer:
  direct typed WS10/WS11/WS12/WS18/WS20 execution in
  `kernel/direct.rs` and shared WS20 segment-routing core.
- Downstream publication consumer:
  typed routed-state maps on `WatershedNetworkFrame`.
- Output/API surface:
  unchanged watershed interchange writers.

Positive evidence:

- Source guard test
  `wshedw4_public_cli_handoff_uses_typed_network_and_publication_frames`
  requires the public CLI direct handoff, direct typed publication, absence of
  old surface markers in the direct production path, and direct physics call
  markers.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract --
  --nocapture`: all 24 tests passed.
- Public generated-mode and reuse-mode tests wrote and decoded watershed
  Parquet outputs through the public CLI.
- Worker-pool jobs=1/jobs=N test decoded all watershed output Parquet rows and
  proved row-content/order identity.

Negative proof:

- `rg` over `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` and
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` found
  no `compatibility_writeback_surface`, `harvest_compatibility_routing_report`,
  `execute_watershed_dispatch_with_kernel`, `WatershedWritebackSurface`,
  `BoundarySymbol`, `BoundaryValue`, `KernelWritebackPayload`, or
  `WatershedKernelRequest`.

Path-scoped old-surface remainder:

- Legacy compatibility dispatch and old integration tests remain in the crate
  for replay/contract coverage. They are no longer the production public CLI
  route.
