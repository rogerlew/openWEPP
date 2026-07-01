# Line-Count Governance

Status: `EXECUTED`

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

## Current Counts

Evidence class: `Ran:`

```text
wc -l crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs \
  crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs \
  crates/openwepp-watershed-orchestrator/src/lib.rs \
  crates/openwepp-watershed-orchestrator/src/lib_mod/mod.rs
```

Result:

```text
  2131 crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs
  2343 crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs
   907 crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs
   678 crates/openwepp-watershed-orchestrator/src/lib.rs
    22 crates/openwepp-watershed-orchestrator/src/lib_mod/mod.rs
  6081 total
```

Disposition:

- `openwepp-cli-watershed.rs` remains in `WARN` but W4 reduced its line count
  by moving old publication helpers into the typed frame path.
- `watershed_cli_behavior_contract.rs` remains in `WARN`; W4 added one source
  guard there to keep public CLI handoff evidence close to the existing
  watershed behavior contract suite.
- No touched Rust file exceeds the `3000`-line mandatory split threshold.
