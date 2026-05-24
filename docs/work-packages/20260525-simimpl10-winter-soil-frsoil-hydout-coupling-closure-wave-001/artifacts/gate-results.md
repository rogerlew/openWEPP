# gate-results

Status: complete
Evidence mode: Ran
Date: 2026-05-24

## Static
- Required SIMIMPL10 gates executed for touched code and package scope.

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only: unmatched allow-list licenses + duplicate lock entries)
- Targeted SIMIMPL04 suite -> pass
