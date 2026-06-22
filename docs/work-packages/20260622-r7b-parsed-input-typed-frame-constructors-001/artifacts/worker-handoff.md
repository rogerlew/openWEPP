# Worker Handoff

Status: complete.

## Handoff

- R7B is complete and ready for R7C.
- R7C first implementation step: add an explicit production direct runtime
  selection distinct from skeleton, shadow, and publication-only cutover modes,
  then route that mode from parsed typed constructor inputs into
  `DirectFrameExecutor`.
- R7C must prove the production direct executor loop consumes the R7B
  constructor-owned day inputs and does not construct
  `HillslopeKernelRequest`, `KernelWritebackPayload`,
  `HillslopeWritebackSurface`, symbol registry, indexed surfaces, dense
  refresh, dirty flush, or WB13-row publication authority inside normal direct
  execution.
- R7C must preserve the default compatibility path and should keep the R7B
  runner static scan green until an explicit production direct mode invokes the
  constructor boundary outside default compatibility execution.
