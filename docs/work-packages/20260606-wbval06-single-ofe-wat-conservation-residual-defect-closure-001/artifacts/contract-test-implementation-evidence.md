# Contract-Test Implementation Evidence

Status: corrected

Evidence mode: executed

Purpose: record contract-derived tests for the WAT residual defect and their
before/after results.

Static:

- Contract-derived checks added or updated:
  - `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
    requires WAT schema metadata for `Interception`.
  - `tests/integration/sim_contract_boundary_unit_registry.rs` requires
    `hillslope_wat.Interception` unit coverage.
  - `crates/openwepp-hillslope-output/src/hillslope_wat.rs` tests require the
    schema field, `mm` units, and description metadata.
  - `crates/openwepp-runner/src/hillslope/mod.rs` unit test verifies WAT rows
    publish the daily interception flux from runtime `I`.
  - Static version-pin tests for `SC-WATBAL-001` were updated to v146.

Ran:

- `cargo test -p openwepp-hillslope-output hillslope_wat --lib`: passed.
- `cargo test -p openwepp-runner wbval06_hillslope_wat_row_publishes_daily_interception_flux --lib`: passed.
- `cargo test --test sim_contract_boundary_unit_registry`: passed.
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests`: passed.
- `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract hphys0320_contract_authority_is_registered`: passed after v146 pin update.
- `cargo test --workspace`: passed after updating the shared WB13 unit-test
  probe to seed required `I=0.0`.
