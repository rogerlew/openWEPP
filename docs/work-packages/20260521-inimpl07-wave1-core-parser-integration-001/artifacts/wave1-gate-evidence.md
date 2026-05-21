# INIMPL07 Wave 1 Gate Evidence

Evidence mode: `Ran`
Date: 2026-05-21

## Commands Executed

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

## Results

1. `cargo fmt --check`: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`: pass
3. `cargo test --workspace`: pass
  - climate contract tests: 9 passed
  - management contract tests: 9 passed
  - slope contract tests: 12 passed
  - soil contract tests: 7 passed
4. `cargo deny check`: pass
  - `advisories ok, bans ok, licenses ok, sources ok`
  - non-failing warnings: `license-not-encountered` for unmatched allowlist entries in `deny.toml`

## Notes

[DIRECT] No failing gate remains in Wave 1 integration state.
