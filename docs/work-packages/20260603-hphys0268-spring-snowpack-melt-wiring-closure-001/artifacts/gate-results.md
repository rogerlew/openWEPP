# Gate Results

Status: completed/HOLD
Evidence mode: Ran

Ran:

- `cargo fmt --check`: pass.
- `python3 -m py_compile .../hphys0268_diagnostics.py`: pass.
- `cargo test -p openwepp-runner hphys0268_trace_row_captures_spring_snowpack_lineage --lib`: pass.
- `cargo test --test clim05_snow_runtime_kernel_contract`: pass, 6 passed.
- HPHYS0268 targeted traces: pass for H1/H7/H39 at `/tmp/hphys0268_targeted_fix_20260603T173830Z`.
- HPHYS0268 final full suite: pass for runtime execution and semantic report generation at `/tmp/hphys0268_final_20260603T174015Z`.
- `cargo deny check`: pass with existing warnings for unmatched license allowances and duplicate crate versions.
- `cargo clippy --workspace --all-targets -- -D warnings`: fail on pre-existing `clippy::too_many_lines` in `hphys0260_trace_row_captures_wb17_wb18_storage_diagnostics`.
- `cargo test --workspace`: fail in existing SIMIMPL18 fixture tests at ET phase (`HKERNEL-WB11-ET-E-003`), before touched snow runoff code executes.
