# gate-results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Required SIMIMPL16 gate set executed on final post-format state.

## Ran
- `cargo fmt --check` -> pass (after applying `cargo fmt` once).
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `cargo test --workspace` -> pass.
- `cargo deny check` -> pass (`advisories ok, bans ok, licenses ok, sources ok`; warnings only).
