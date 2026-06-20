# Contract Implementation Evidence

Status: complete.
Evidence mode: Static + Ran.

Trace from `r3c-span-contract.md` to implementation:

| Contract Item | Implementation Evidence | Test Evidence |
|---|---|---|
| Selected span is `[LateralTransfer, RunoffReconciliation, ClosureDiagnostics]` | `DIRECT_R3C_LANE_TRANSFER_SPAN` in `direct_runtime.rs`. | `r3c_lane_transfer_span_projects_multilane_topology` asserts exact span identity. |
| Inputs are lane topology, upstream-area ratio, area, and transfer buffers | `compute_r3c_lane_transfer_ledger` reads only `DirectLaneFrame` direct fields. | R3C fixture populates three direct lanes and transfer buffers. |
| Direct compute produces per-lane outgoing, received, and net transfer | `DirectLaneTransferLedger` records outgoing/received/net fields. | Expected ledger fixture asserts exact binary-fraction values. |
| State mutation records run-level transfer ledger | `run_r3c_lane_transfer_span` stores `lane_transfer_ledger`. | Test asserts frame state equals expected ledger. |
| Downstream operands are produced | `DirectRunTransferDownstreamOperands` is built from shadow projection. | Test asserts frame downstream operands equal report totals. |
| Shadow projection is produced | `DirectRunTransferShadowProjection::from_ledger` computes run totals. | Test asserts report/frame shadow projection equals expected totals. |
| Invalid domains fail closed | `validate_r3c_lane_transfer_domain` rejects bad counts, ids, topology, outlet count, negative/nonfinite inputs, and nonfinite totals. | Rejection test covers negative ratio, invalid downstream id, nonreciprocal topology, multiple outlets, and overflow cases. |
| Compatibility calls are excluded | `direct_runtime.rs` forbidden-token scan has no matches. | Runtime direct compatibility-edge counters remain zero. |
| Default-disabled path remains inactive | Runner default fixture asserts zero direct counters. | `cargo test -p openwepp-runner r2a_ -- --nocapture` passes. |

R3C is contract-complete for the package scope. It remains diagnostic-only and
does not publish outputs or migrate hydrology-process equations.
