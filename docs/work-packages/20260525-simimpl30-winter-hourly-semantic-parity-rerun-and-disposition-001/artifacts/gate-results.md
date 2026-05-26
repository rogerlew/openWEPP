# gate results

Status: complete
Evidence mode: ran
Date: 2026-05-26
Gate bundle: `artifacts/gates-20260526T125552Z/`

## Ran
- `cargo fmt --check` -> `0`
- `cargo clippy --workspace --all-targets -- -D warnings` -> `0`
- `cargo test --workspace` -> `0`
- `cargo deny check` -> `0`

## Notes
- `cargo test --workspace` includes long-running integration vectors; final exit status remained success (`0`).
