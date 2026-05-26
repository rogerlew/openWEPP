# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Static
- none

## Ran
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Result
- `cargo fmt --check`: pass (`cargo_fmt_check.exit_code=0`)
- `cargo clippy --workspace --all-targets -- -D warnings`: pass (`cargo_clippy.exit_code=0`)
- `cargo test --workspace`: pass (`cargo_test.exit_code=0`)
- `cargo deny check`: pass (`cargo_deny.exit_code=0`; warnings only for duplicate lock entries and unmatched allowlist licenses)

## Evidence bundle
- `artifacts/gates-20260526T170356Z/`
