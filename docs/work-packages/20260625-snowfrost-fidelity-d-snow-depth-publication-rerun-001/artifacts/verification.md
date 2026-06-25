# Verification

Evidence mode: Ran.

## Final Gates

All final gates passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`

Focused gates also passed:

- `cargo test -p openwepp-hillslope-output hillslope_wat`
- `cargo test --test snowfreeze_observed_frost_depth_contract`
- `cargo test -p openwepp-runner --lib`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/observed_harness.py tools/snowfreeze_observed/classify_residuals.py`

## Observed Rerun

The all-site rerun completed and produced package-local reports under
`artifacts/site_reports/`.

Classification result:

- Site count: `5`
- `OPENWEPP-DEFECTIVE`: `0`
- Defect-attribution eligible: `0`
- `SNOW-CONTROL-FAILED`: `3`
- `INCONCLUSIVE`: `2`

SNOWFROST-FIDELITY-D closes the missing modeled snow-depth publication blocker.
It does not close snow/frost fidelity because paired snow-depth control now
fails or is unavailable.
