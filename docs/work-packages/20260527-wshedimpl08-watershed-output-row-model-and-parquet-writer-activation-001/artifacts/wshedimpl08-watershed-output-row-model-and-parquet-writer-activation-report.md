# WSHEDIMPL08 Watershed Output Row Model and Parquet Writer Activation Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Activated watershed interchange writer in
  `crates/openwepp-watershed-output/src/writers.rs`:
  - replaced placeholder guard return with real parquet emission,
  - writes all 14 required watershed outputs,
  - emits non-empty row batches with schema metadata.
- Added typed row-seed carrier (`WatershedInterchangeRowSeed`) and runtime
  row-seed mapping in
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` from dispatch and
  writeback surfaces.
- Updated watershed CLI behavior contract tests:
  - removed ignored WSHED03 non-stub parquet vector,
  - converted output-guard expected-failure assertions to active output
    emission assertions.
- Synchronized `SC-SYSTEM-001` and science-contract index for
  `GAP-SYSTEM-006` closure posture.

## Ran
- `cargo fmt`
- `cargo test -p openwepp-watershed-output`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
