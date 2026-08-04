# Architecture Boundary

Status: `complete / production design frozen`

Evidence mode: `Static: exact source and consumer inventory`

## Frozen Boundary

The terminal design has one authoritative calculation, one production state
mutation, two exact linked compact ledgers, and one optional verbose payload:

```text
hourly CoE solve / finalization
  -> DirectSnowSolidToLiquidLedger
       raw signed melt
       redistributed positive melt
       authoritative snowpack SWE loss
       released rain
       exact routed-liquid handoff
  -> Stage-3 solve consumes exact routed-liquid handoff
       -> DirectSnowStage3Outcome (enabled, sublimation, meltwater temperature)
       -> DirectSnowLiquidDispositionLedger
            incoming, routed, retained delta, refrozen, residual
  -> DirectSnowLiquidPartition authoritative state/result
       -> direct runtime consumes outcome/state/compact ledgers
       -> optional DirectSnowVerboseDiagnostics only for real JSONL writer
```

The owned ledger structs replace duplicate top-level transition scalars; they
do not copy an independently mutable second accounting state. The linked
Stage-3 incoming operand is the exact upstream `liquid_handoff_m` argument.

## Capture Path

`00c_day_input_builder_impl.rs` resolves the existing nonempty trace path and
day/lane filters before calling snow partition. It passes
`DirectSnowDiagnosticCapture::{Disabled,Verbose}` into a new explicit compute
entry point. The existing public compute method remains as a compatibility
entry point that requests verbose diagnostics, preserving existing diagnostic
tests and non-runner callers.

The production runner uses the explicit method. Disabled capture retains local
working state required by the solve and all production closure guards, but it
does not assemble `SnowHourlyTrace`, the 24-hour melt diagnostic array, the
24-hour Stage-3 surface-energy array, or a post-solve verbose carrier. Enabled
capture moves those exact values into one boxed optional payload.

## Consumer Map

1. `finalize_active_snow_coupling` produces the upstream exact operands.
2. `compute_direct_snow_liquid_partition_*` owns the partition, linked ledger,
   Stage-3 outcome, and state mutation.
3. `resolve_stage3_liquid_routing` produces the downstream exact operands and
   guards liquid/energy closure.
4. `DirectSnowCoupling*` direct-runtime types carry only the production
   `DirectSnowStage3Outcome`, not verbose diagnostics.
5. `maybe_write_r7h_direct_production_snow_trace` requires the optional payload
   only on a selected row and fails typed if the request/producer disagree.
6. `r7h_direct_production_snow_trace_line` formats compact ledgers, production
   outcome, and verbose diagnostics without recomputation.
7. The package parser consumes the real schema-v4 file.

Capture cannot alter snow equations, selectors, thresholds, input values,
state order, or output values. Production-required `enabled`, Stage-3
`sublimation_m`, and typed meltwater temperature move to the outcome and remain
available on both capture paths.
