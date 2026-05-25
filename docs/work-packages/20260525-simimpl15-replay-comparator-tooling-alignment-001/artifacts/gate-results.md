# gate-results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Required SIMIMPL15 gate set executed for package disposition.

## Ran
- `cargo fmt --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `cargo test --workspace` -> pass.
- `cargo deny check` -> pass (warnings only; final summary `advisories ok, bans ok, licenses ok, sources ok`).
