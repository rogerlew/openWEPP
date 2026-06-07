# Implementation Test Evidence

Status: corrected

Evidence mode: executed

Purpose: record production edits, test commands, validation commands, and
skipped gates with rationale.

Required commands unless a legitimate boundary prevents production edits:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- Targeted WBVAL06 release validation commands.

Static:

- Production correction:
  - WAT schema version minor bumped to `1.3`.
  - `HillslopeWatRow` now includes optional `interception`.
  - WB13 publication requires finite, nonnegative runtime `I` and converts it
    from meters to WAT `mm`.
  - WAT row construction publishes `Interception = I_mm`.
  - Shared WB13 unit-test fixture seeds `I=0.0` so tests exercise intended
    guards.
- Documentation and registry corrections:
  - `SC-WATBAL-001` v146.
  - Boundary/unit registry and output registry include `Interception`.
  - Runner/output docs list `Interception` and `InterceptionStorage`
    separately.

Ran:

- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed; emitted only existing warnings for duplicate
  crates and unmatched allow-license entries.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: passed.
- WBVAL06 corrected release validation emitted `22` WAT parquet files and
  closed all annual years `2..6` within tolerance.
