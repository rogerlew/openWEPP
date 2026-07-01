# Line-Count Governance

Status: `QUEUED`

Record line counts for touched Rust files before closure.

Known warning state from W4:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` is in `WARN`.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` is in
  `WARN`.

Governance:

- `2000+` lines is `WARN`.
- `3000+` non-exempt lines requires refactor before closure.
- Prefer routing/orchestrator modules over growing the public CLI binary or
  monolithic behavior test.
