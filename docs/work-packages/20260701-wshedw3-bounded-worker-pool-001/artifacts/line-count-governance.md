# Line-Count Governance

Status: `QUEUED`

W3 execution must record line counts for touched Rust files before closure.

Required command shape:

```text
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/src/watershed_supervisor.rs \
  crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs
```

Governance:

- `2000+` lines is `WARN`.
- `3000+` non-exempt lines requires refactor before closure.
- W2 left `openwepp-cli-watershed.rs` in the `WARN` band; W3 should avoid
  growing that binary body when a supervisor-module edit is viable.
