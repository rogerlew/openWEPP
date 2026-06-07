# Gate Results

Status: complete

Evidence mode: Ran.

## Commands

- `cargo fmt --check`
  - result: passed after running `cargo fmt` for the new test formatting
- `git diff --check`
  - result: passed
- `cargo clippy --workspace --all-targets -- -D warnings`
  - result: passed
- `cargo test --workspace`
  - result: passed
- `cargo deny check`
  - result: passed with existing duplicate/license warnings; final categories:
    `advisories ok, bans ok, licenses ok, sources ok`

## Additional Validation

- `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture`
  - result: `13 passed`
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - result: passed
- algebraic-radium population rerun
  - result: `42/42` runnable prefixes return code `0`
  - result: `42/42` nonzero `Q/QOFE`
  - result: annual closure max abs residual `2.808064891723916e-11 mm`

## Not Run

- `bash tools/release/check_authority_suite_antievasion.sh`
  - not applicable; this package did not edit external-authority suite posture,
    cohort fixtures, or required-case bindings.
