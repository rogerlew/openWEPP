# Gate Results

Status: completed/HOLD
Evidence mode: ran

Ran:

- `cargo fmt --check` -> pass.
- `python3 -m py_compile docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/hphys0269_diagnostics.py` -> pass.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture` -> pass, `8 passed; 0 failed`.
- `cargo test -p openwepp-runner hphys0268_trace_row_captures_spring_snowpack_lineage --lib -- --nocapture` -> pass, `1 passed; 0 failed`.
- `git diff --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract` -> pass, `2 passed; 0 failed`.
- `cargo deny check` -> pass with existing duplicate/unmatched-license warnings only.
- `.venv/bin/python .../hphys0269_diagnostics.py --run-root /tmp/hphys0269_full_final_20260603T185740Z --trace-max-days 180` -> pass; targeted H1/H7/H39 traces returned `0`.
- `.venv/bin/python .../hphys0269_diagnostics.py --run-root /tmp/hphys0269_full_final_20260603T185740Z --trace-max-days 180` -> pass; full runtime `39/39`, semantic pass `0/39`.

Ran after corrected negative-melt authority follow-up:

- `git diff --check` -> pass.
- `python3 -m py_compile docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/hphys0269_diagnostics.py` -> pass.
- `cargo fmt --check` -> pass.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract` -> pass, `2 passed; 0 failed`.

Ran with failure:

- `python3 .../hphys0269_diagnostics.py --run-root <targeted> --skip-full-suite --trace-max-days 180` -> failed during report writing because the system Python environment lacked a parquet engine (`pyarrow`/`fastparquet`). The package reran successfully with `.venv/bin/python`.
- `cargo test --workspace` -> fail in pre-existing SIMIMPL18 fixture tests: `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage` and `simimpl18_contract_requires_multi_day_storage_state_mutation`, both at ET guard `HKERNEL-WB11-ET-E-003`. The same failure is recorded in HPHYS0268 artifacts and is not introduced by HPHYS0269 snowpack writeback paths.
