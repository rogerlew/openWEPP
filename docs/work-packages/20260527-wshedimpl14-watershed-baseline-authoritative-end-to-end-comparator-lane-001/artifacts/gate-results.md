# WSHEDIMPL14 Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Required repository gates executed after WSHEDIMPL14 implementation.

## Ran
1. `cargo fmt --check` -> pass
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
3. `cargo test --workspace` -> pass
4. `cargo deny check` -> pass with existing non-fatal warnings:
   - duplicate lockfile crate versions (`getrandom`, `hashbrown`, `twox-hash`)
   - unmatched license allowances (`ISC`, `Unicode-DFS-2016`)
   - advisories/bans/licenses/sources overall status: `ok`
