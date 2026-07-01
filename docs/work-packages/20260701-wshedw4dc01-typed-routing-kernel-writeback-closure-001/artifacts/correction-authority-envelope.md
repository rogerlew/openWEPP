# Correction Authority Envelope

Status: `QUEUED`

Defect ID: `WSHED-W4-HOLD-001`.

Observed failure:

- Public watershed CLI still routes through
  `WatershedNetworkFrame::compatibility_writeback_surface()`.
- Production routing still calls `execute_watershed_dispatch_with_kernel`.
- Orchestrator dispatch/kernel code still reads/writes
  `WatershedWritebackSurface`, `BoundarySymbol`, `BoundaryValue`, and
  `KernelWritebackPayload`.
- Typed publication still harvests from compatibility report state and can
  inherit compatibility zero-default behavior.

In-scope correction:

- Replace the production compatibility projection with frame-native dispatch.
- Replace symbol-map request/writeback routing internals with typed
  frame/request/response values.
- Replace compatibility-harvested publication operands with typed routed-state
  operands and fail-closed missing-operand handling.

Protected boundaries:

- No physics changes for performance.
- No silent clamps or guard loosening.
- No output schema redesign.
- No W5 full deletion claim.

Acceptance:

- Public CLI routes without `compatibility_writeback_surface`.
- Production routing loops no longer read/write old symbol-map surfaces.
- Protected outputs are identity-equivalent or contract-governed.
- Conservation reconstruction and magnitude audit are recorded.
