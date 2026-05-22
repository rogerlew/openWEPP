# Gate Results — ARCH07

Evidence: Ran
Date: 2026-05-21 UTC

## Required Gates

| gate | command | result | notes |
| --- | --- | --- | --- |
| 1 | `cargo fmt --check` | pass | no formatting drift |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass | strict clippy + pedantic under `-D warnings` |
| 3 | `cargo test --workspace` | pass | all workspace unit/integration/doc tests passed, including new ARCH07 tests |
| 4 | `cargo deny check` | pass | advisories/bans/licenses/sources all ok; non-failing `license-not-encountered` warnings in allowlist |

## Command Evidence Excerpts

- `cargo clippy --workspace --all-targets -- -D warnings`
  - `Checking openwepp v0.1.0 ...`
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) ...`
- `cargo test --workspace`
  - `Running tests/integration/kernel_writeback_contract.rs`
  - `test result: ok. 4 passed; 0 failed; ...`
- `cargo deny check`
  - Final line: `advisories ok, bans ok, licenses ok, sources ok`

## Gate Verdict
- ARCH07 gate status: `PASS`
