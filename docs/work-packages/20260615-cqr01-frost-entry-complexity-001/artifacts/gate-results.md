# CQR01 Gate Results

Status: complete

Evidence mode: ran

## Ran

Preimplementation and focused gates:

| Command | Exit Code | Result |
|---|---:|---|
| `cargo test --test clim06_frost_frozen_soil_kernel_contract` | 0 | pre-refactor `46 passed` |
| `cargo llvm-cov --workspace --ignore-run-fail --no-report` | 0 | pre-refactor coverage profile generated |
| `cargo check -p openwepp-hillslope-orchestrator` | 0 | focused compile check passed |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | 0 | focused clippy passed |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract` | 0 | post-refactor `46 passed` |
| `cargo llvm-cov --workspace --ignore-run-fail --no-report` | 0 | post-refactor coverage profile generated |

Required closure gates:

| Command | Exit Code | Result |
|---|---:|---|
| `cargo fmt --check` | 0 | passed |
| `git diff --check` | 0 | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | passed |
| `cargo test --workspace` | 0 | passed |
| `cargo deny check` | 0 | `advisories ok, bans ok, licenses ok, sources ok` |

Metric/report commands:

| Command | Exit Code | Result |
|---|---:|---|
| `cargo llvm-cov --workspace --ignore-run-fail --no-run --lcov --output-path .../lcov_before.info` | 0 | saved LCOV; deprecated `--no-run` warning |
| `cargo llvm-cov --workspace --ignore-run-fail --no-run --json --summary-only --output-path .../coverage_before_summary.json` | 0 | saved summary JSON; deprecated `--no-run` warning |
| `cargo crap --workspace --lcov .../lcov_before.info --min 0 --format json --output .../crap_before.json` | 0 | saved JSON; 124 missing-LCOV-entry warning |
| `cargo llvm-cov --workspace --ignore-run-fail --no-run --lcov --output-path .../lcov_after.info` | 0 | saved LCOV; deprecated `--no-run` warning |
| `cargo llvm-cov --workspace --ignore-run-fail --no-run --json --summary-only --output-path .../coverage_after_summary.json` | 0 | saved summary JSON; deprecated `--no-run` warning |
| `cargo crap --workspace --lcov .../lcov_after.info --min 0 --format json --output .../crap_after.json` | 0 | saved JSON; 124 missing-LCOV-entry warning |
