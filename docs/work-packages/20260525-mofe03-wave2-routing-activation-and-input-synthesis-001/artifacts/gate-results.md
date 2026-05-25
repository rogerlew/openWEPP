# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- n/a

## Ran
- `cargo fmt --check`
  - Initial run: failed due formatting diffs in `crates/openwepp-runner/src/hillslope/mod.rs`.
  - Remediation: `cargo fmt`.
  - Final run: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Initial MOFE03 implementation run failed in `openwepp-runner` on:
    - `clippy::too_many_lines` for Wave-2 seeding routine,
    - `clippy::cast_precision_loss` for `usize as f64` class-index conversions.
  - Remediation: mechanical modularization of Wave-2 seeding helpers and conversion via `usize_to_scalar(...)`.
  - Final run: passed.
- `cargo test --workspace`
  - Passed.
- `cargo deny check`
  - Passed with duplicate-crate and unmatched-license-allowance warnings; no advisory/bans/license/source hard failures.
