# Line-Count Governance

Status: `EXECUTED-COMPLETE`

Ran:

```text
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/src/watershed_supervisor.rs \
  crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs
```

Result:

| File | Lines | Governance |
| --- | ---: | --- |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2248 | `WARN` (`2000+`) |
| `crates/openwepp-runner/src/watershed_supervisor.rs` | 730 | OK |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | 1977 | OK |

Disposition:

- The watershed CLI binary was already near the warning band before W2 and now
  remains above `2000` lines. W2 avoided pushing the supervisor implementation
  into that monolith by adding `crates/openwepp-runner/src/watershed_supervisor.rs`.
- No touched Rust file is above the `3000`-line refactor-required threshold.
- Follow-on W3/W4 work should continue moving supervisor and typed frame logic
  out of `openwepp-cli-watershed.rs` rather than growing the binary body.
