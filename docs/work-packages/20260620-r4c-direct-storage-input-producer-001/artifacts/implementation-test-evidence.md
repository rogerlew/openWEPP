# R4C Implementation And Test Evidence

Status: complete.
Evidence mode: Static + Ran.

## Implementation Summary

R4C added a direct WB12 storage-input producer under the existing direct-runtime
namespace. The span consumes:

- R3A direct `DirectDownstreamOperands::precipitation_m`;
- current direct `DirectWaterState::soil_water_m`.

It computes and mutates `DirectStorageInputState`, writes
`DirectStorageReconciliationInputs::storage_initial_m` and
`DirectStorageReconciliationInputs::precip_input_m`, produces
`DirectStorageInputDownstreamOperands`, and records a
`DirectStorageInputShadowProjection`.

R4B now fails closed unless R4C storage input and R4A runoff partition have both
run. Remaining storage terms stay explicit R4B inputs:
`snow_coupling_m`, `evapotranspiration_m`, `deep_seepage_m`,
`subsurface_loss_m`, and `closure_tolerance_m`.

The package also split storage-related direct-runtime code into:

```text
crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs
```

No scheduler, compatibility runtime, output schema, publication writer, or
default-activation path was changed.

## Tests

Focused tests added or updated:

- `r4c_storage_input_producer_consumes_r3a_precipitation_and_direct_storage`
- `r4c_storage_input_producer_rejects_invalid_inputs`
- `r4b_storage_reconciliation_consumes_r4a_q_and_shadow_projects`
- `r4b_storage_reconciliation_rejects_invalid_inputs`
- `r2a_direct_runtime_source_excludes_compatibility_storage_tokens`

The no-compat source test now scans both `direct_runtime.rs` and
`direct_runtime/storage.rs`.

## Ran

- `cargo test -p openwepp-hillslope-orchestrator r4c_ -- --nocapture`:
  PASS, 2 tests.
- `cargo test -p openwepp-hillslope-orchestrator r4b_ -- --nocapture`:
  PASS, 2 tests.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`:
  PASS, 3 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`: PASS, 2 tests.
- `cargo test --workspace`: PASS.
