# Consumer Map

Status: COMPLETE.

Static:
- Deleted bridge authorities:
  - `DirectFrostRunoffSurface` production handoff.
  - `DirectFrostLiquidPartition` production handoff.
  - `Wb11HydrologyKernel::compute_direct_frost_liquid_partition`.
  - `day_input_and_helpers/03_frost_comparator_seam.rs`.
- Replacement authority:
  - `DirectWinterFrostComputeInputs` carries controls, thermal inputs, scalar
    soil authority, per-layer bulk density, and hourly forcing.
  - `DirectWinterFrostPartitionOutcome` is the typed winter-column frost
    outcome.
  - `Wb11HydrologyKernel::compute_direct_winter_frost_partition` is the typed
    compute entry point used by production.
- Producer path:
  - Runner direct-publication day builders derive typed frost compute inputs
    from runtime winter state and sidecar/parser surfaces.
  - Builder-local typed frost compute still derives the same-day frozen
    infiltration capacity for WB14 inputs; the outcome is no longer handed
    through a day-frame bridge.
- Consumer path:
  - `DirectPublicationDayInput` and `DirectDayConstructorInputs` can carry
    `winter_frost_compute_inputs`.
  - `DirectFrameExecutor` borrows that payload for the current day and passes it
    into `DirectDayFrame::run_r4a_runoff_partition_span_with_winter_frost`.
  - `DirectDayFrame` does not store the compute payload; this preserves the R7B
    frame-size budget.
  - R4A reads latest post-ET/subsurface/percolation layers, reads prior
    `winter_column.frost`, computes the typed outcome, mutates winter-column
    frost state, mirrors temporary runtime carry, and projects storage and
    hydrology frost operands.
