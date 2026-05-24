# simimpl05 runner orchestrator daily integration map

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Integration map
- Entry point:
  - `execute_hillslope_run(...)`
- New lifecycle gate:
  - `execute_daily_scheduler_kernel_lifecycle(runtime_surface)`
  - Invoked before any publication writes (`outputs.pass`, `outputs.loss`,
    optional outputs).
- Topology precondition path:
  - Build minimal single-hillslope graph with
    `TopologyGraph::new(1, 0, 0, Vec::new())`.
  - Validate with `validate_pre_execution_topology`.
- Scheduler/kernel path:
  - `HillslopePhaseScheduler::canonical().execute_with_kernel(...)`
  - Runner-local phase kernel adapter (`RunnerDailyPhaseKernel`) executes
    nominal per-phase status through orchestrator-owned lifecycle boundaries.
- Publication provenance surface:
  - Manifest now carries `execution_provenance`:
    - `scheduler_kernel_executed`
    - `publication_source`
    - `simpipe_guard_id`
    - `selected_lane`
    - `scheduler_outcome_class`
    - `scheduler_status_message_id`

## Guard/failure map
- Guard ID:
  - `HS-SIMPIPE-E-001`
- Failure surface:
  - `HillslopeCliError::RuntimeSurfaceFailure { surface: "execution_provenance", ... }`
- Trigger classes:
  - topology precondition build/validation failure,
  - scheduler/kernel execution error,
  - non-success scheduler outcome.

## Scope boundary notes
- SIMIMPL05 closes daily execution ownership provenance only.
- SIMMODE (`/mode_selection/*`) and SIMOUT (`/wb13_publication/*`) manifest
  closure remains deferred to SIMIMPL07/SIMIMPL06.
