# Gate Results

Status: completed/HOLD
Evidence mode: ran

Static: HPHYS0277 gate ledger. Commands were run from `/home/workdir/openWEPP`.

Ran:

| Gate | Command | Result |
| --- | --- | --- |
| Red guard test | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_hourly_radiation --lib -- --nocapture` before production edit | Failed as expected; no typed high-flux guard existed |
| Green guard test | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_hourly_radiation --lib -- --nocapture` | Passed |
| Unit conversion regression | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion --lib -- --nocapture` | Passed |
| Near-isothermal regression | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_near_isothermal_radiation_is_radmj_over_24 --lib -- --nocapture` | Passed |
| Formatting | `cargo fmt --check` | Passed |
| Raw conversion guard | `tools/release/check_raw_unit_conversions.sh` | Passed |
| Climate context tests | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context --lib` | Passed |
| Runtime input tests | `cargo test -p openwepp-hillslope-orchestrator runtime_inputs` | Passed |
| Snow runtime contract | `cargo test --test clim05_snow_runtime_kernel_contract` | Passed |
| Boundary unit typing contract | `cargo test --test hphys0275_boundary_value_dimensional_typing_contract` | Passed |
| Package clippy | `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | Passed |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| Cargo deny | `cargo deny check` | Passed with existing duplicate/unmatched-license warnings |
| Docs lint | `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --path docs/specifications/science-contracts/index.md --path docs/specifications/units/boundary-symbol-unit-registry.md --path docs/work-packages/20260603-hphys0277-climate-radiation-physical-flux-guard-001 --path docs/work-packages/README.md` | Passed |
| Diff whitespace | `git diff --check` | Passed |
| Targeted/full diagnostics | `.venv/bin/python docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/artifacts/hphys0272_diagnostics.py --run-root /tmp/openwepp-hphys0277-radiation-guard` | Passed with `rc=0` |
| Workspace tests | `cargo test --workspace` | Failed/HOLD in known SIMIMPL18/WB11 ET domain tests outside HPHYS0277 |
| Post-verification guard rerun | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_hourly_radiation --lib -- --nocapture` | Passed |
| Post-verification climate context rerun | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context --lib` | Passed |

## Diagnostic Retry Note

Ran: the first diagnostic attempt with system `python3` failed after producing
targeted traces because pandas could not find a Parquet engine
(`pyarrow`/`fastparquet`). The diagnostics were rerun successfully with
`.venv/bin/python`, where `pandas 3.0.3` and `pyarrow 22.0.0` are installed.
