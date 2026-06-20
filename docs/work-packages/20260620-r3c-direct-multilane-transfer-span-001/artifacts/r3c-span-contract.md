# R3C Span Contract

Status: complete.
Evidence mode: Static.

Selected direct span:

```text
DIRECT_R3C_LANE_TRANSFER_SPAN =
  [LateralTransfer, RunoffReconciliation, ClosureDiagnostics]
```

The span consumes:

- `DirectLaneFrame::lane_id`, `upstream_lane_id`, `downstream_lane_id`;
- `DirectLaneFrame::upstream_area_ratio`;
- `DirectLaneFrame::area_m2`;
- `DirectTransferBuffers::surface_carry_m`;
- `DirectTransferBuffers::lateral_carry_m`.

The span computes, per lane:

- `outgoing_surface_m = sum(surface_carry_m)`;
- `outgoing_lateral_m = sum(lateral_carry_m)`;
- `received_surface_m = upstream outgoing_surface_m * upstream_area_ratio`;
- `received_lateral_m = upstream outgoing_lateral_m * upstream_area_ratio`;
- `net_transfer_m = received_surface_m + received_lateral_m
  - outgoing_surface_m - outgoing_lateral_m`.

The span computes, per run:

- lane count;
- outlet lane id;
- total outgoing surface and lateral transfer;
- total received surface and lateral transfer;
- total signed net transfer.

The ledger is diagnostic only. R3C does not assert hydrology-process closure,
correct process magnitudes, or publication meaning. Negative net transfer is
valid if finite.

The span mutates direct run-level transfer ledger state, produces downstream
transfer operands, and shadow-projects the run-level transfer result. It must
not call compatibility storage/request/writeback APIs.
