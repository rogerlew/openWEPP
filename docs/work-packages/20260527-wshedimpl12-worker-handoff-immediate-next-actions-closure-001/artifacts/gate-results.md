# WSHEDIMPL12 Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Required gate set executed after WSHEDIMPL12 package-spec and governance
  updates.

## Ran
1. `cargo fmt --check` -> pass
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
3. `cargo test --workspace` -> pass
4. `cargo deny check` -> pass with existing non-fatal warnings:
   - duplicate lockfile crate versions (`getrandom`, `hashbrown`, `twox-hash`)
   - unmatched license allowances (`ISC`, `Unicode-DFS-2016`)
   - advisories/bans/licenses/sources overall status: `ok`
