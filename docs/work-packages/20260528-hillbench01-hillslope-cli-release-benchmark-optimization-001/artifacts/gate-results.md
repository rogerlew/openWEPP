# Gate Results

Status: complete  
Evidence mode: ran  
Date: 2026-05-28

## Ran
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass
  - warnings-only:
    - duplicate crate-version entries in `Cargo.lock`
    - unmatched license-allow entries in `deny.toml`
  - final summary: `advisories ok, bans ok, licenses ok, sources ok`
