# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- n/a

## Ran
- `cargo fmt --check`
  - Final run: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Initial MOFE04 closeout run was blocked by `clippy::float_cmp` in `tests/integration/cli03_runner_contract_derived_tests.rs`.
  - Remediation: replaced direct float equality assertion with epsilon comparison.
  - Final run: passed.
- `cargo test --workspace`
  - Passed.
- `cargo deny check`
  - Passed with duplicate-crate and unmatched-license-allowance warnings; no advisory/bans/license/source hard failures.
