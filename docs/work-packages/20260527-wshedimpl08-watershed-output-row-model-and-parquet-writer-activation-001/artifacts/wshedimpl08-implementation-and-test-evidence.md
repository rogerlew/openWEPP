# WSHEDIMPL08 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- `crates/openwepp-watershed-output/src/writers.rs`
  - introduced `WatershedInterchangeRowSeed`,
  - implemented parquet writer path for all required watershed outputs,
  - added Arrow record-batch construction for schema-compatible one-row
    emission with typed writer failure codes (`OWSOUT-E-003/005/006`),
  - replaced placeholder `OWSOUT-E-004` fail-closed block in valid lanes.
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
  - added `build_watershed_output_row_seed` builder from dispatch/writeback
    report surfaces,
  - wired writer invocation to pass row-seed data.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
  - promoted WSHED03 parquet vector,
  - updated legacy and MOFE-valid lanes to assert successful output emission.

## Ran
- `cargo fmt`
- `cargo test -p openwepp-watershed-output`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
