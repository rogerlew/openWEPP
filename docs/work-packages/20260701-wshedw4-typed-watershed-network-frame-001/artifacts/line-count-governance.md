# Line-Count Governance

Status: `QUEUED`

W4 execution must record line counts for touched Rust files before closure.

Known W3 warning state:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`: `2318` lines.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`: `2288`
  lines.

Governance:

- `2000+` lines is `WARN`.
- `3000+` non-exempt lines requires refactor before closure.
- W4 should move typed routing/publication logic into orchestrator/output
  modules rather than growing the watershed CLI binary or monolithic tests.
