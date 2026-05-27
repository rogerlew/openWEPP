# WSHEDIMPL08 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Updated `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`:
  - promoted `wshed03_watershed_cli_end_to_end_vector_requires_non_stub_parquet_emission`
    to active execution (no ignore),
  - converted legacy/output-guard expectations to output-emission assertions
    (`assert_all_watershed_outputs_exist`),
  - retained typed-failure checks for unrelated MOFE intake/domain guard paths.
- Updated `crates/openwepp-watershed-output/src/writers.rs` unit tests to
  validate non-empty parquet emission and schema metadata presence.

## Ran
- `cargo test -p openwepp-watershed-output`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
