# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- none

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only: duplicate crates and
  license-not-encountered policy entries)
