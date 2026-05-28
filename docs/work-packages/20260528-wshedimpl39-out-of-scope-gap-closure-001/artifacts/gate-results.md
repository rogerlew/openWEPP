# WSHEDIMPL39 Gate Results

Status: complete  
Evidence mode: ran  
Date: 2026-05-28

## Static
- Required gate stack executed:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

## Ran
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (warnings-only output:
  duplicate crate versions and unmatched license allowances in `deny.toml`;
  no advisory/bans/licenses/sources failures)
