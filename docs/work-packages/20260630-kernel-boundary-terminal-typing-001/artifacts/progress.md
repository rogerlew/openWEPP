# Progress

Evidence class: Static plus Ran.

## Implemented

- Deleted the compiled symbol-map hillslope runtime modules:
  `scheduler.rs`, `day_frame.rs`, `phase.rs`, `consumer_boundary.rs`,
  scheduler trace modules, scheduler publication/seed support, symbol-registry
  audit support, and scheduler-only test modules.
- Deleted scheduler-era hydrology phase entry modules and symbol request/
  writeback carrier tests that existed only to validate the removed runtime
  boundary.
- Kept production direct execution and publication on typed direct-frame state.
  The direct runner no longer constructs a static
  `HillslopeWritebackSurface` seed authority or an alternate scheduler runtime.
- Retargeted stale static contract tests to the typed/direct surfaces:
  HBP required-state ids, AUTH07 corrected parser-layer FC projection,
  HPHYS0296 direct snow trace schema, Paradigm-2 nullable meltwater-temperature
  evidence, and snow-density default selector assertions.
- Installed missing local `.venv` test dependencies (`pyarrow`, `pandas`) so the
  full profile's Python harness checks run in a venv-capable environment.

## Deleted Test Surface

Deleted tests were scheduler/symbol-boundary tests whose subject was removed:
consumer-boundary integration, `HillslopeKernelRequest`/
`KernelWritebackPayload` unit tests, scheduler/day-frame tests, static
`scheduler_trace` source tests, and old runtime-kernel binaries that asserted
files or symbols no longer present in the terminal architecture.

The retained tests now guard production behavior through direct runtime
fixtures, direct publication schema/value tests, source guards that reject
carrier reintroduction, and the full observed snow/frost diagnostic suite.

## Survivor Classification

`BoundarySymbol`/`BoundaryValue` still exist in three non-runtime classes:

- watershed CLI/channel serialization adapters;
- typed guard and diagnostic error-reporting helpers that name legacy symbols in
  error messages or diagnostic rows;
- source-guard tests that assert deleted carrier/runtime names do not reappear.

The executable carrier/runtime names
`HillslopeWritebackSurface`, `HillslopeKernelRequest`,
`KernelWritebackPayload`, `SymbolRegistry`, `HotSymbolTables`,
`HillslopePhaseScheduler`, `HillslopeDayFrame`, `scheduler_trace`,
`runtime_surface_symbol_value`, and `require_runtime_surface_scalar` occur only
inside source-guard test literals after deletion.
