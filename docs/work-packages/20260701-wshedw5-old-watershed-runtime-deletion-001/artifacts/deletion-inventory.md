# Deletion Inventory

Status: `executed`

Evidence mode: `static + ran`

Pre-edit scan found the old watershed runtime concentrated in
`openwepp-watershed-orchestrator`, `openwepp-kernel-contract`, and three root
integration suites:

- `delete`: `WatershedWritebackSurface`,
  `WatershedKernelExecutionReport`, `WatershedKernelStepReport`.
- `delete`: watershed `WatershedKernelRequest` / `WatershedKernel` protocol.
- `delete`: `execute_watershed_dispatch_with_kernel` and
  `execute_watershed_dispatch_with_gate_and_kernel`.
- `delete`: `compatibility_writeback_surface`,
  `harvest_compatibility_routing_report`, compatibility channel/impoundment
  harvest helpers, and runtime-surface builders/seeders.
- `delete`: obsolete old-surface tests
  `ws10_watershed_kernel_contract`,
  `ws11_channel_routing_physics_equivalence_contract`, and
  `ws12_impoundment_physics_equivalence_contract`.
- `retained-out-of-scope`: generic/hillslope `KernelWritebackPayload`,
  `KernelRunResponse`, `BoundarySymbol`, and `BoundaryValue` remain for
  non-watershed writeback and generic contract infrastructure.
- `retained-out-of-scope`: `SimulationPhase::WatershedKernel` remains the
  status taxonomy for the typed direct watershed kernel.

Post-edit scan:

```text
rg -n "WatershedKernelRequest|WatershedKernel\b|WatershedWritebackSurface|WatershedKernelExecutionReport|WatershedKernelStepReport|execute_watershed_dispatch_with_kernel|execute_watershed_dispatch_with_gate_and_kernel|compatibility_writeback_surface|harvest_compatibility|build_watershed_runtime_surface|seed_watershed_runtime_surface|WatershedClimateRuntime|ParseOutcomeNotRuntimeReady|ChannelSymbolNonFinite|ChannelSymbolOutOfDomain|NonFiniteDtchrInput" crates/openwepp-watershed-orchestrator/src crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs tests/integration Cargo.toml --glob '!target'
```

Result: no deleted old-runtime matches. Remaining scan output is limited to
`SimulationPhase::WatershedKernel`, which is the live typed-kernel status phase.
