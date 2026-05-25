# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- n/a

## Ran
- `cargo fmt --check`
  - Passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Initial run identified MOFE05 lint issues:
    - `clippy::format_push_string` in watershed CLI behavior test helper,
    - `clippy::uninlined_format_args` in watershed CLI intake guards,
    - `clippy::too_many_lines` in manifest metadata validator.
  - Remediation: replaced formatted push pattern, inlined format args, and
    added targeted line-count allowance on the metadata validator helper.
  - Final run: passed.
- `cargo test --workspace`
  - Passed.
- `cargo deny check`
  - Passed with duplicate-crate and unmatched-license-allowance warnings; no advisory/bans/license/source hard failures.
