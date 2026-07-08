# Consumer Path Proof

Status: `PASS`
Evidence mode: Static + Ran.

## Active Default Reaches The Consumer

Static path:

- Runner active config construction calls `mesh_policy_from_env()` and stores
  the result in `DirectLanedActiveConfig.mesh_policy`
  (`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:55`).
- With no `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M`, the runner returns
  `DirectLanedActiveMeshPolicy::production_default()`
  (`crates/openwepp-runner/src/hillslope/laned_active.rs:205`).
- The production default is `TargetDx { target_dx_m: 5.0, min_cells: 10,
  max_cells: 4096 }`
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:171`).
- The active executor passes `config.mesh_policy` into every routed lane
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs:570`).
- The mesh policy computes `ceil(slplen_m / target_dx_m)`, checks the cap, and
  applies the `10` cell floor
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:223`).

Runtime proof:

- `artifacts/default-dx5-evidence.json` shows all selected real-cohort active
  no-env runs serialize `mesh_policy.mode = target_dx`, `target_dx_m = 5.0`,
  `min_cells = 10`, `max_cells = 4096`, and `max_dt_s = 300.0`.
- Active no-env outputs are byte-identical to explicit `dx5` outputs for all
  selected real-cohort members.

## Protected Off Path

Static path:

- Active routing is opt-in only through `OPENWEPP_LANED_ACTIVE=1`
  (`crates/openwepp-runner/src/hillslope/laned_active.rs:22`).
- The runner only attaches `frame.laned_active` when that selector is enabled
  (`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:95`).
- Active and shadow are mutually exclusive and fail closed if both are set
  (`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:96`).

Runtime proof:

- `off_default` and `off_mesh_env_control` are byte-identical for HBP, loss
  JSON, pass parquet, and WAT parquet for `mn_corn_h4`,
  `n_idaho_forest_h1`, and `wa_cascades_forest_h1`.
- The off-mode manifest has no `execution_provenance.laned_active` block.

## DC01 Disable And Routed Erosion Consumer

Static path:

- Phase 1 hydrology runs with surface transfer suppressed and lateral transfer
  left unchanged
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs:497`).
- Every active lane routes before row consumption or frame commit
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs:549`).
- The active double-feed guard fails closed if DC01 surface runon remains live
  on an active lane
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:647`).
- Zero-source active lanes still set the erosion shape authority to routed
  hydrograph with all-zero weights
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs:587`).
- Positive routed days map the routed outlet bin series to the D13 hourly
  erosion weights, folding drain-tail mass into hour 24 and counting
  full-mesh-hold degeneracy rather than silently falling back
  (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:673`).

Runtime proof:

- The active default selected-cohort runs completed with live closure
  hard-fails, `days_routed > 0`, routed trace rows, terminal outlet totals,
  end-window storage, tail-fold totals, and counted degenerate-shape rows in
  the manifest evidence.
- No active run published through the old DC01 surface path; any such
  double-feed would trip `laned_active_dc01_double_feed_guard` before
  publication.

## Shadow Mesh

Static proof:

- The diagnostic shadow still owns `LANED_SHADOW_CELLS = 10`,
  `LANED_SHADOW_SAMPLE_DT_S = 900`, and `LANED_SHADOW_MAX_DT_S = 300`
  (`crates/openwepp-runner/src/hillslope/laned_shadow.rs:37`).
- This package changed only the active production mesh default; no shadow code
  or selector path changed.
