# Implementation Test Evidence

Status: COMPLETE.

Static:

- Added typed frost input structures for controls, hourly forcing, thermal
  context, layer state, prior state, and full partition input in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`.
- Added and exported
  `Wb11HydrologyKernel::compute_direct_frost_liquid_partition_from_typed`.
- Refactored active-frost hourly execution so the typed path threads
  `DirectFrostHourlyForcing` and `ActiveFrostTmpadjContext` instead of
  `HillslopeKernelRequest`.
- Production direct frost day context now builds
  `DirectActiveFrostPartitionInputs` from lane/winter-column/day state and
  passes only the resulting `DirectFrostLiquidPartition` into R4A.
- Production direct day input no longer assigns
  `day_input.frost_runoff_surface = Some(frost_context.surface)`.
- Frost layer authority is read only when frost projection is present; snow-only
  direct authority no longer requires frost layer topology.
- R4A applies the typed partition to `DirectWinterColumnState.frost` through
  the existing in-place direct frost state mutation path.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r7g_typed_active_no_freeze_partition_matches_surface_adapter -- --nocapture`
  passed.
- `cargo test -p openwepp-hillslope-orchestrator r7g_typed_inactive_frost_partition_matches_surface_adapter_without_material -- --nocapture`
  passed.
- `cargo test -p openwepp-runner r7g_direct_production -- --nocapture`
  passed before the helper split; final full workspace rerun also passed the
  same runner tests.
- `cargo test -p openwepp-runner --lib` passed after the production snow-only
  authority fix.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed on the final tree.
- `cargo deny check` passed.
