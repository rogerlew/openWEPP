# Gate Results — ARCH06

Evidence: Ran
Date: 2026-05-21 UTC

## Required Gates

| gate | command | result | notes |
| --- | --- | --- | --- |
| 1 | `cargo fmt --check` | pass | no formatting drift reported |
| 2 | `cargo clippy --manifest-path crates/openwepp-watershed-orchestrator/Cargo.toml --all-targets -- -D warnings` | pass | no warnings under `-D warnings` |
| 3 | `cargo test --manifest-path crates/openwepp-watershed-orchestrator/Cargo.toml` | pass | crate-local unit tests and doc-tests passed |

## Command Evidence Excerpts

- `cargo clippy --manifest-path crates/openwepp-watershed-orchestrator/Cargo.toml --all-targets -- -D warnings`
  - `Checking openwepp-watershed-orchestrator v0.1.0 ...`
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) ...`
- `cargo test --manifest-path crates/openwepp-watershed-orchestrator/Cargo.toml`
  - `running 4 tests`
  - `test result: ok. 4 passed; 0 failed; ...`

## Gate Verdict
- ARCH06 gate status: `PASS`
