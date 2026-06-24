# Typed Solver Proof

Status: COMPLETE.

Static:

- Typed input boundary exists in
  `DirectFrostControlInputs`, `DirectFrostHourlyForcing`,
  `DirectFrostThermalInputs`, `DirectFrostLayerInput`,
  `DirectFrostPriorStateInput`, and `DirectActiveFrostPartitionInputs`.
- `Wb11HydrologyKernel::compute_direct_frost_liquid_partition_from_typed`
  validates soil conductivity and dispatches active frost without constructing
  `HillslopeKernelRequest`.
- Request-backed active-frost support remains only in the compatibility adapter
  path. The typed path calls `compute_active_frost_coupling_from_typed` and
  passes `None` for request-only finalization checks.
- Production direct frost day context builds typed inputs from:
  direct lane layer/water state, `DirectWinterColumnState.frost`,
  prior `DirectWinterColumnState.snow`, day forcing, direct winter hourly
  forcing, and typed frost controls.
- Production direct build assigns `frost_liquid_partition` and
  `frost_layer_carry_projection`, not `frost_runoff_surface`.

Ran:

- Source scan over runner direct-publication helpers found the typed production
  call at `00_builders_and_authority.rs` and no production
  `frost_surface_template`, production `overlay_frost_day_forcing`, or
  production `day_input.frost_runoff_surface = Some(frost_context.surface)`.
- `cargo test -p openwepp-runner r7g_direct_production -- --nocapture`
  passed during focused iteration.
- `cargo test --workspace` passed on the final tree.
