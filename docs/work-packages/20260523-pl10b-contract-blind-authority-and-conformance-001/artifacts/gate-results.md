# PL10b Gate Results

Status: `complete`
Evidence mode: `Ran`

## Required Workspace Gates

| gate | command | result | notes |
|---|---|---|---|
| format | `cargo fmt --check` | `pass` | formatting clean after test additions |
| lint | `cargo clippy --workspace --all-targets -- -D warnings` | `pass` | no warnings |
| tests | `cargo test --workspace` | `pass` | workspace tests pass; PL10b conformance tests remain ignored |
| dependency-policy | `cargo deny check` | `pass` | advisories/bans/licenses/sources `ok`; existing unmatched allowlist warnings in `deny.toml` |

## PL10b Conformance Gate Run (Explicit)

| gate | command | result | notes |
|---|---|---|---|
| transition-control contract conformance | `cargo test --test parser_runtime_seam_integration -- --ignored` | `fail` | `0 passed; 5 failed`; failures reconciled into PL11 implementation scope |
