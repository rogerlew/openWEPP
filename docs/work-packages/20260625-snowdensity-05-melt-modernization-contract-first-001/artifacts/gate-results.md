# Gate Results

Status: queued.
Evidence mode: not-run.

Required final gates:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `wctl doc-lint --path docs/work-packages/README.md`
- package-specific focused tests
