# PERFIDX03 Verification A

Status: HOLD 2026-06-17
Evidence mode: **Ran**

Verification focus: runtime evidence.

## Verified

- Pre-flip registry audit:
  - `cli01_cropland`: `unknown_symbol_count = 0`.
  - `perennial_cut`: `unknown_symbol_count = 0`.
  - `perennial_grazing`: `unknown_symbol_count = 0`.
  - `irrigation_combo`: `unknown_symbol_count = 0`.
  - `rotation_two_year`: `unknown_symbol_count = 0`.
- Active flip timing regression:
  - Baseline OFE5 mean: `27.01s`.
  - Active flip OFE5 mean: `38.34s`.
- No-flip current timing:
  - OFE5 sample: `26.80s`.
- Current closure gates:
  - `cargo fmt --check`: PASS.
  - `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
  - `cargo test --workspace`: PASS.
  - `cargo deny check`: PASS.
  - `git diff --check`: PASS.

## Verification Result

Runtime evidence verifies HOLD: the registry precondition and generic gates pass,
but active authority flip performance fails.
