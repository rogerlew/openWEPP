# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Ran
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Result
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass (warnings only for duplicate lock entries and
  unmatched allow-list licenses; no advisory/license/source failures)
