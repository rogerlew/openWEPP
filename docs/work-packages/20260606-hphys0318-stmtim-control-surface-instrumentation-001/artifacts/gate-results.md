# Gate Results

Status: complete

Evidence mode: Ran

Ran:

| Gate | Command | Result |
|---|---|---|
| Pre-implementation contract gate | `cargo test --test hphys0318_stmtim_control_surface_instrumentation_contract hphys0318_contract_authority_is_registered -- --nocapture` | Passed; exit status was `0`. |
| Focused orchestrator instrumentation | `cargo test -p openwepp-hillslope-orchestrator hphys0318_stmtim_control_surfaces_publish_branch_inputs_and_outputs -- --nocapture` | Passed; exit status was `0`. |
| Focused runner trace instrumentation | `cargo test -p openwepp-runner hphys0318_trace_row_captures_stmtim_control_surfaces -- --nocapture` | Passed; exit status was `0`. |
| Boundary unit registry | `cargo test --test sim_contract_boundary_unit_registry -- --nocapture` | Passed; exit status was `0`. |
| HPHYS0318 static runtime/trace symbol test | `cargo test --test hphys0318_stmtim_control_surface_instrumentation_contract hphys0318_runtime_and_trace_symbols_are_registered -- --nocapture` | Passed; exit status was `0`. |
| Existing HPHYS0245 trace writer | `cargo test -p openwepp-runner hphys0245_trace_writer_serializes_jsonl_rows -- --nocapture` | Passed; exit status was `0`. |
| Existing HPHYS0271 trace maps | `cargo test -p openwepp-runner hphys0271_trace_row_captures_melt_term_hourly_forcing_maps -- --nocapture` | Passed; exit status was `0`. |
| Existing SIMIMPL28 context tests | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context -- --nocapture` | Passed; exit status was `0`. |
| HPHYS0291 trace schema regression repair | `cargo test --test hphys0291_snow_publication_lifecycle_contract hphys0291_trace_preserves_snow_publication_lifecycle_surfaces -- --nocapture` | Passed after updating the stale expected HPHYS0245 trace schema from `trace-v16` to `trace-v17`; exit status was `0`. |
| Formatting | `cargo fmt --check` | Passed; exit status was `0`. |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Passed after test assertion cleanup; final exit status was `0`. |
| Cargo deny | `cargo deny check` | Passed; exit status was `0`; existing duplicate/unmatched-license warnings only. |
| Authority anti-evasion | `bash tools/release/check_authority_suite_antievasion.sh` | Passed; exit status was `0`. |
| AUTH11 required-suite guards | `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Passed; exit status was `0`. |
| Markdown/package lint | `markdown-doc lint --path docs/work-packages/20260606-hphys0318-stmtim-control-surface-instrumentation-001 --path docs/work-packages/README.md --path docs/specifications/science-contracts/index.md --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | Passed; `29` files validated; exit status was `0`. |
| Workspace tests | `cargo test --workspace` | Initial rerun failed on stale HPHYS0291 `trace-v16` schema expectation after the HPHYS0318 trace schema bump. After updating the expectation to `trace-v17`, the rerun passed; final exit status was `0`. |
| Final diff hygiene | `git diff --check` | Passed after final artifact update; exit status was `0`. |
