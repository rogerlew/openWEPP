# Executor Routing Evidence

Status: complete.

## Routing

Static:

- `HillslopeRuntimeSelection::DirectProductionExecutor` is a distinct runner
  selection with manifest string `direct-production-executor`.
- `openwepp-cli-hill --direct-production-executor` selects that mode
  explicitly. Default API/CLI execution remains `Compatibility`.
- `execute_hillslope_run_with_runtime_selection` branches once after static
  setup. `DirectProductionExecutor` calls
  `execute_hillslope_direct_production_days`; other modes call
  `execute_hillslope_climate_days`.
- Production direct selection skips symbol-registry audit and indexed-shadow
  diagnostic adapter construction.
- `select_direct_runtime_skeleton_once` returns immediately for
  `DirectProductionExecutor`; skeleton construction is not part of R7C
  execution.
- Direct production output writes use direct publication artifacts retained by
  `DirectFrameExecutor`. R7C does not claim R7D publication parity or default
  activation.

## Counters

Ran:

- Focused fixture:
  `cargo test -p openwepp-runner r7c -- --nocapture`.
- H2637 final direct-production manifest:
  `/tmp/r7c-h2637-final/direct-production/rep1/h2637_same/openwepp_hillslope_run_manifest.json`.

Focused fixture direct-production manifest:

- `scheduler_kernel_executed=false`.
- `publication_source=direct-publication-frame`.
- `climate_day_count=2`.
- `executed_day_count=2`.
- `run_frame_constructions=1`.
- `day_frame_constructions=2`.
- `day_frame_commits=2`.
- `executor_constructions=1`.
- `skeleton_runs=0`.
- `publication_capture_runs=1`.
- `phase_view_constructions=28`.
- `phase_span_runs=41`.
- `direct_phase_entries=75`.
- `direct_compute_operations=41`.
- `direct_state_mutations=41`.
- `downstream_operand_productions=41`.
- `shadow_projections=41`.
- `compatibility_edge_invocations=0`.

H2637 final direct-production manifest:

- `scheduler_kernel_executed=false`.
- `publication_source=direct-publication-frame`.
- `climate_day_count=12419`.
- `executed_day_count=12419`.
- `run_frame_constructions=1`.
- `day_frame_constructions=235961`.
- `day_frame_commits=235961`.
- `executor_constructions=1`.
- `skeleton_runs=0`.
- `publication_capture_runs=1`.
- `phase_view_constructions=3303454`.
- `phase_span_runs=4719221`.
- `direct_phase_entries=8494599`.
- `direct_compute_operations=4719221`.
- `direct_state_mutations=4719221`.
- `downstream_operand_productions=4719221`.
- `shadow_projections=4719221`.
- `compatibility_edge_invocations=0`.

Default H2637 final compatibility manifest:

- `scheduler_kernel_executed=true`.
- `publication_source=scheduler-kernel`.
- `climate_day_count=12419`.
- `executed_day_count=12419`.
- `direct_runtime_counters=null`.

## Static Scans

Ran:

- `cargo test -p openwepp-runner r7c -- --nocapture`.

Static test evidence:

- `r7c_direct_production_source_excludes_compatibility_entrypoints` inspects
  `execute_hillslope_direct_production_days` and rejects
  `execute_hillslope_climate_days(`, `execute_with_kernel`, and
  `HillslopeKernelRequest`.
- `r7c_direct_production_executor_runs_without_compatibility_edges` verifies
  manifest provenance, nonzero direct execution counters, `skeleton_runs=0`,
  and `compatibility_edge_invocations=0`.

Manual static review:

- `execute_hillslope_direct_production_days` constructs a direct run frame,
  builds interleaved day inputs, runs `DirectFrameExecutor` in
  `ProductionDirect` mode, and returns retained direct publication artifacts.
- The direct production function body does not construct
  `HillslopeKernelRequest`, call `execute_with_kernel*`, or enter
  `execute_hillslope_climate_days`.
