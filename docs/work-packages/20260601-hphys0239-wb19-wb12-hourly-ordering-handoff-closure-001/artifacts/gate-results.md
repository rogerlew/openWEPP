# HPHYS0239 Gate Results

Status: completed  
Evidence mode: Ran

## Required Gates

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Results

- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass with pre-existing warnings for duplicate
  `getrandom`/`hashbrown`/`twox-hash` lock entries and unmatched `ISC` /
  `Unicode-DFS-2016` license allowances.
