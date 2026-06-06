# Implementation Test Evidence

Status: complete

Evidence mode: Ran

Ran:

- `cargo test -p openwepp-hillslope-orchestrator hphys0318_stmtim_control_surfaces_publish_branch_inputs_and_outputs -- --nocapture`
  - Result: passed; exit status was `0`.
- `cargo test -p openwepp-runner hphys0318_trace_row_captures_stmtim_control_surfaces -- --nocapture`
  - Result: passed; exit status was `0`.
- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`
  - Result: passed; exit status was `0`.
- `cargo test --test hphys0318_stmtim_control_surface_instrumentation_contract hphys0318_runtime_and_trace_symbols_are_registered -- --nocapture`
  - Result: passed; exit status was `0`.
- `cargo test -p openwepp-runner hphys0245_trace_writer_serializes_jsonl_rows -- --nocapture`
  - Result: passed; exit status was `0`.
- `cargo test -p openwepp-runner hphys0271_trace_row_captures_melt_term_hourly_forcing_maps -- --nocapture`
  - Result: passed; exit status was `0`.
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context -- --nocapture`
  - Result: passed; exit status was `0`.
- `cargo test --test hphys0291_snow_publication_lifecycle_contract hphys0291_trace_preserves_snow_publication_lifecycle_surfaces -- --nocapture`
  - Result: passed after updating the stale expected HPHYS0245 trace schema
    from `trace-v16` to `trace-v17`; exit status was `0`.
- `cargo test --workspace`
  - Result: initial rerun failed on the stale HPHYS0291 trace schema
    expectation; rerun after the schema-expectation update passed with exit
    status `0`.
