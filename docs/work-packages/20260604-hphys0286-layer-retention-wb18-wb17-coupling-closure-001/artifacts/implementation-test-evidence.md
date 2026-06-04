# Implementation Test Evidence

Status: complete
Evidence mode: Ran

## Evidence

Ran:
- `cargo test --test hphys0286_layer_retention_wb18_wb17_contract -- --nocapture`
  - Result: passed, `2 passed`.
- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`
  - Result: passed, `12 passed`.
- `cargo test --test hphys0285_spring_soil_storage_retention_contract -- --nocapture`
  - Result: passed, `3 passed`.
- `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`
  - Result: passed, `16 passed`.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture`
  - Result: passed, `6 passed`.
- `cargo test -p openwepp-hillslope-orchestrator --lib`
  - Result: passed.
- `cargo fmt --check`
  - Result: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed.
- `cargo test --workspace`
  - Result: passed.
- `cargo deny check`
  - Result: passed with existing duplicate-crate and unmatched-license-allowance warnings.
- `cargo build --release --package openwepp-runner --bin openwepp-cli-hill`
  - Result: passed.
- Full H1..H39 release runtime and semantic suite:
  - Root: `/tmp/hphys0286_full_release_20260604T211814Z`
  - Runtime: `39/39`.
  - Semantic reports: `39/39`.
  - Semantic pass: `0/39`.
