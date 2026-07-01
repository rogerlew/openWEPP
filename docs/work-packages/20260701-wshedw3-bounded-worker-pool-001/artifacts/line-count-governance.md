# Line-Count Governance

Status: `EXECUTED`

W3 line-count check.

Evidence class: `Ran:`

```text
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/src/watershed_supervisor.rs \
  crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs
```

Result:

```text
  2318 crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs
   917 crates/openwepp-runner/src/watershed_supervisor.rs
  2288 crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs
  5523 total
```

Governance:

- `2000+` lines is `WARN`.
- `3000+` non-exempt lines requires refactor before closure.
- W2 left `openwepp-cli-watershed.rs` in the `WARN` band; W3 should avoid
  growing that binary body when a supervisor-module edit is viable.

Disposition:

- `openwepp-cli-watershed.rs` remains in `WARN`; W3 added the public
  handoff/parser/help edits needed for `--jobs N` plus a success-only
  supervisor timing sidecar. The worker-pool implementation lives in
  `watershed_supervisor.rs`.
- `watershed_cli_behavior_contract.rs` is also in `WARN`; W3 added focused
  public CLI coverage there to preserve the existing test locality. No touched
  Rust file exceeds the `3000`-line mandatory split threshold.
