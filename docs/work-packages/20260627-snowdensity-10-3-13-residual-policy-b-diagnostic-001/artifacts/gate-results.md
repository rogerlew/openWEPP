# Gate Results

Evidence mode: Ran.

## Commands

- `.venv/bin/python -m py_compile tools/snowfreeze_observed/residual_policy_b_diagnostic.py`
- `.venv/bin/python tools/snowfreeze_observed/residual_policy_b_diagnostic.py`
- `cargo fmt`
- `cargo test --test snowdensity10_3_13_residual_policy_b_diagnostic -- --nocapture`
- `cargo test -p openwepp --test snowdensity03_physics_bulk_offline_contract -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --test snowdensity10_3_13_residual_policy_b_diagnostic -- -D warnings`
- `git diff --check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check`

## Gate Disposition

- Passed: diagnostic tool compile.
- Passed: diagnostic report generation.
- Passed: targeted SNOWDENSITY-10.3.13 integration test.
- Passed: existing SNOWDENSITY-03 confinement test with the new v99 contract marker.
- Passed: full workspace test suite.
- Passed: full workspace clippy with `-D warnings`.
- Passed: dependency/advisory/license/source checks.
- Passed: authority-suite anti-evasion guards.
- Passed: diff whitespace check.

## Diagnostic Summary

- Disposition: `HOLD-ACTIVATION-EVIDENCE-MISSING`
- Activation blocker: `POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING`
- Frost-attribution blocker: `SNOW-CONTROL-RESIDUALS-REMAIN`
- Complete transition rows: `1414`
- Source bundle paired rows: `1415`
- Default failures: `1147`
- Bundle failures: `498`
- Under-persistence induced by density arm: `177/234`
