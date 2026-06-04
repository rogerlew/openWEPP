# Gate Results

Status: complete
Evidence mode: Ran

## Ran: Command Gates

- Final rerun command: `cargo fmt && cargo test --test hphys0284_negative_melt_snowpack_state_contract -- --nocapture && cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture && cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`.
- `cargo test --test hphys0284_negative_melt_snowpack_state_contract -- --nocapture`: passed, `2 passed`.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture`: passed, `9 passed`.
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed with existing duplicate-crate and license-not-encountered warnings; final status `advisories ok, bans ok, licenses ok, sources ok`.

## Ran: H1..H39 Semantic Gate

- Full release run root: `/tmp/hphys0284_full_release_20260604T182144Z`.
- Runtime status: all 39 hillslopes status `0`.
- Semantic status: all 39 comparisons status `0`.
- Semantic pass remains `0/39`; metrics recorded in `full-39-suite-metrics.md`.

## Gate Disposition

- Implementation gates pass.
- Semantic parity closure remains open; continuation required for storage/runoff residuals.
