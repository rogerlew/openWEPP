# refactor014-kernel-gate-results

Status: complete
Evidence mode: Ran

## Commands executed
- `cargo fmt --check`
  - Exit: 0
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Exit: 0
- `cargo test -p openwepp-watershed-orchestrator --tests`
  - Exit: 0
- `cargo test --workspace`
  - Exit: 101
- `cargo deny check`
  - Exit: 0

## Additional notes
- `cargo test --workspace` failure is unrelated to kernel refactor content and
  reproduces existing ADR0017 workspace registry expectation:
  `20260605-adr0017-comparator-distrust-ratification-001` was not found in
  `docs/work-packages/README.md`.
- Workspace warning in `cargo deny check` includes pre-existing unmatched license
  allowlist entries in `deny.toml`.
