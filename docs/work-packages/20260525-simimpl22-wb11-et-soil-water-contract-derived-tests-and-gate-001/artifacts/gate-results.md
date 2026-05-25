# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- SIMIMPL22 modified non-doc test code and therefore executed required gate
  commands per package exit criteria.

## Ran
- `cargo fmt --check`
  - result: pass
- `cargo clippy --workspace --all-targets -- -D warnings`
  - result: pass
- `cargo test --workspace`
  - result: pass
- `cargo deny check`
  - result: pass (warnings only: duplicate crate versions and unmatched allowed licenses in `deny.toml`)
