# Old Runtime Deletion Manifest

Status: `executed`

Evidence mode: `static + ran`

Deleted production files:

- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/climate.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/tests.rs`

Deleted integration tests:

- `tests/integration/ws10_watershed_kernel_contract.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

Deleted symbols and entrypoints:

- `WatershedWritebackSurface`
- `WatershedKernelExecutionReport`
- `WatershedKernelStepReport`
- `WatershedKernelRequest`
- `WatershedKernel`
- `execute_watershed_dispatch_with_kernel`
- `execute_watershed_dispatch_with_gate_and_kernel`
- `compatibility_writeback_surface`
- compatibility harvest helpers and old symbol-map runtime seeders
- stale chan.inp/channel/climate runtime-input error variants that only served
  deleted builders

Replacement path:

- public CLI builds `WatershedNetworkFrame`;
- routing executes via `execute_watershed_dispatch_with_frame`;
- direct WS10/WS11/WS12/WS18/WS20 helpers write typed routed state into the
  frame;
- publication uses `publish_typed_routing_report`.

Retained out-of-scope symbols:

- `KernelWritebackPayload`, `KernelRunResponse`, `BoundarySymbol`, and
  `BoundaryValue` remain for generic/hillslope contract infrastructure.
- `SimulationPhase::WatershedKernel` remains the status phase for typed
  watershed execution.
- public runner `--policy compat` remains a fixture/input compatibility policy,
  not an old watershed request/writeback runtime selector.

Source guard:

- `wshedw5_public_cli_uses_typed_network_and_publication_frames` asserts the
  CLI handoff markers, forbids old watershed runtime markers in the public CLI,
  and scans frame, dispatch, types, kernel core, included kernel helper files,
  direct kernel, and runtime-input exports for deleted old-runtime strings.
