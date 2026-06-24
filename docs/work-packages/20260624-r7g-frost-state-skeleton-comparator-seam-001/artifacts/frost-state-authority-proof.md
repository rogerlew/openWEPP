# Frost State Authority Proof

Status: complete.

Evidence mode: Static + Ran.

Static:

- `DirectFrostLaneState::has_runtime_state` now treats active coupling, scalar
  front/thaw/liquid/frozen ledger fields, `watpdg`, `watbtm`, layer shadows,
  and fine layers as persistent state.
- `DirectFrostRuntimeCarry` converts to/from `DirectFrostLaneState`, including
  `DirectFrostLayerShadowCarry` and `DirectFrostFineLayerCarry`.
- `DirectLaneFrame::from_constructor_inputs`, `DirectRunFrame::seed_day_frame`,
  `DirectDayFrame::apply_constructor_inputs`, and `DirectLaneFrame::commit_day`
  now preserve frost through `DirectWinterColumnState.frost` and regenerate the
  carry mirror from winter state.
- R4A frost reconciliation writes `direct_frost_runtime_carry(partition)` into
  `self.winter_column.frost` before storing the legacy mirror.
- Direct publication reads prior frost from `lane.winter_column.frost` in both
  day-input build paths and provenance extraction.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r7g_ -- --nocapture`
  passed: 14 passed, 0 failed.
- `cargo test -p openwepp-runner r7g_direct_production -- --nocapture`
  passed: 4 passed, 0 failed.
