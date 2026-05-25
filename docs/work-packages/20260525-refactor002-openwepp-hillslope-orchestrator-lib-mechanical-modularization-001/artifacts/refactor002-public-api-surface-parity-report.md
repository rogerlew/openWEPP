# REFACTOR002 Public API Surface Parity Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Legacy export inventory (`HEAD` pre-refactor `crates/openwepp-hillslope-orchestrator/src/lib.rs`) was compared to post-refactor facade exports.

Legacy public surface families identified:
- module:
  - `runtime_inputs`
- public enums:
  - `HillslopePhase`
  - `HillslopeConsumerBoundaryError`
  - `HillslopePlActiveSlotResolutionError`
  - `HillslopeGrowthBoundaryError`
  - `HillslopeDecompositionBoundaryError`
  - `HillslopeHydrologyRoutingError`
  - `Wb11HydrologyKernelGuardError`
  - `SchedulerOutcomeClass`
  - `HillslopeSchedulerError`
- public structs:
  - `Wb11HydrologyKernel`
  - `PhaseDependency`
  - `HillslopePhaseGraph`
  - `HillslopePhaseOutcome`
  - `HillslopeSchedulerReport`
  - `HillslopeWritebackSurface`
  - `HillslopeKernelPhaseReport`
  - `HillslopeKernelExecutionReport`
  - `HillslopePhaseScheduler`
- public functions:
  - `hillslope_consumer_adapter_for_phase`
  - `required_hillslope_consumer_state_symbols`
  - `validate_hillslope_consumer_boundary`

Post-refactor `lib.rs` re-exports preserve these families via:
- `pub mod runtime_inputs;`
- `pub use phase::HillslopePhase;`
- `pub use consumer_boundary::{...};`
- `pub use hydrology::{...};`
- `pub use scheduler::{...};`

Conclusion:
- Public API surface parity preserved for previously exported orchestrator API.

## Ran
Compatibility evidence:
1. `cargo test -p openwepp-hillslope-orchestrator`
   - result: pass
2. `cargo test --workspace`
   - result: pass
