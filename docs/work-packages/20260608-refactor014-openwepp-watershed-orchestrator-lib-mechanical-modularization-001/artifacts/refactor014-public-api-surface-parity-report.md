# REFACTOR014 public API surface parity report

Status: complete
Evidence mode: Static + Ran

## Public API parity statement
- Static: Facade exports preserved from `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - `DispatchDiagnostic`, `DispatchDiagnosticCode`, `DispatchStep`
  - `WatershedDispatchReport`, `WatershedWritebackSurface`, `WatershedKernelStepReport`, `WatershedKernelExecutionReport`
  - `WatershedDispatchError`
  - `schedule_watershed_dispatch`, `schedule_watershed_dispatch_with_gate`
  - `execute_watershed_dispatch_with_kernel`, `execute_watershed_dispatch_with_gate_and_kernel`
  - `Ws10ChannelImpoundmentKernel`
- Static: `lib_mod` module wiring remains the structural ownership of dispatch/kernels.
- Ran: `cargo test -p openwepp-watershed-orchestrator --tests` validates unchanged public behavioral surface for tests using this API.
