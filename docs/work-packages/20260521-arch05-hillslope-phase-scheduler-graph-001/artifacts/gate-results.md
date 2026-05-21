# Gate Results — ARCH05

Evidence: Ran
Date: 2026-05-21 UTC

## Required Worker-Local Gates

| gate | command | result | notes |
| --- | --- | --- | --- |
| 1 | `cargo fmt --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --check` | pass | no formatting drift |
| 2 | `cargo clippy --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --all-targets -- -D warnings` | pass | zero warnings under `-D warnings` |
| 3 | `cargo test --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml` | pass | 5 tests passed |

## Command Evidence Excerpts

- `cargo clippy --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --all-targets -- -D warnings`
  - `Checking openwepp-hillslope-orchestrator ...`
  - `Finished 'dev' profile ...`
- `cargo test --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml`
  - `running 5 tests`
  - `test result: ok. 5 passed; 0 failed`

## Gate Verdict
- ARCH05 gate status: `PASS`
