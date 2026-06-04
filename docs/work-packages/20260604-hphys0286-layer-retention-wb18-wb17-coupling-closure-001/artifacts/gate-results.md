# Gate Results

Status: complete
Evidence mode: Ran

## Evidence

Ran:
- `cargo test --test hphys0286_layer_retention_wb18_wb17_contract -- --nocapture`: passed.
- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`: passed.
- `cargo test --test hphys0285_spring_soil_storage_retention_contract -- --nocapture`: passed.
- `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`: passed.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture`: passed.
- `cargo test -p openwepp-hillslope-orchestrator --lib`: passed.
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed with existing duplicate-crate and unmatched-license-allowance warnings.
- `cargo build --release --package openwepp-runner --bin openwepp-cli-hill`: passed.
- Full H1..H39 release runtime/semantic suite at `/tmp/hphys0286_full_release_20260604T211814Z`: runtime `39/39`, semantic reports `39/39`, semantic pass `0/39`.
