# Contract Test Implementation Evidence

Status: completed
Evidence mode: static + ran

Static:

- Added `hphys0271_contract_conformance_records_melt_terms_and_hourly_forcing` in `tests/integration/clim05_snow_runtime_kernel_contract.rs`.
- Added `hphys0271_trace_row_captures_melt_term_hourly_forcing_maps` in `crates/openwepp-runner/src/hillslope/mod.rs`.
- Bumped HPHYS trace schema to `openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v11`.

Ran:

- `cargo test -p openwepp-runner hphys0271_trace_row_captures_melt_term_hourly_forcing_maps --lib -- --nocapture` -> pass, `1 passed`.
- `cargo test --test clim05_snow_runtime_kernel_contract hphys0271_contract_conformance_records_melt_terms_and_hourly_forcing -- --nocapture` -> pass, `1 passed`.
