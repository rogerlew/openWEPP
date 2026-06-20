# R3A Direct Phase API Plan

Status: complete.
Evidence mode: Static + Ran.

## Implemented API

Static:

- Selected phase-span entrypoint:
  `DirectDayFrame::run_r3a_input_accounting_span`.
- Phase constant:
  `DIRECT_R3A_INPUT_ACCOUNTING_SPAN =
  [DirectPhaseKind::Normalization, DirectPhaseKind::LateralTransfer]`.
- Typed input view:
  `DirectDayFrame` carries `DirectDayForcing`, `DirectWaterState`,
  `DirectTransferBuffers`, and `DirectPublicationFrame`.
- Compute function:
  finite/nonnegative validation helpers plus direct transfer-input accounting
  in `run_r3a_input_accounting_span`.
- Mutation target:
  `DirectInputAccountingState` on `DirectDayFrame`.
- Downstream operand structure:
  `DirectDownstreamOperands`.
- Shadow projection structure:
  `DirectShadowProjection`.
- Status/error type:
  `DirectPhaseSpanReport` and `DirectRuntimeError::{NonFiniteDirectValue,
  NegativeDirectValue}`.
- Audit/counter fields:
  `DirectRuntimeAuditSnapshot` now records phase-span runs, direct phase
  entries, direct compute operations, direct state mutations, downstream
  operand productions, shadow projections, and compatibility edge invocations.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r3a_ -- --nocapture`: PASS.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`:
  PASS.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`: PASS.

No compatibility storage, request, writeback, registry, hot-table,
indexed-surface, dense-refresh, or dirty-flush type was introduced into direct
phase execution.
