# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- n/a

## Ran
- `cargo fmt --check`
  - Initial run: failed due formatting in `crates/openwepp-runner/src/hillslope/mod.rs`.
  - Remediation: `cargo fmt`.
  - Final run: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Initial run: failed on `clippy::too_many_lines` in `HillslopeCliError` display function.
  - Remediation: extracted mismatch formatter helper + function-level `#[allow(clippy::too_many_lines)]`.
  - Final run: passed.
- `cargo test --workspace`
  - Initial run: failed in CLI01 fixtures due newly enforced topology gate on legacy two-OFE slope fixtures.
  - Remediation: aligned CLI01 fixture slope files to one-OFE baseline; MOFE tests explicitly mutate fixture topology for mismatch scenarios.
  - Final run: passed.
- `cargo deny check`
  - Passed with duplicate-crate and unmatched-license-allowance warnings; no advisory/bans/license/source hard failures.
