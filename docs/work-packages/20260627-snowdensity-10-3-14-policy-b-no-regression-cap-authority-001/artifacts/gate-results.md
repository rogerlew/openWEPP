# SNOWDENSITY-10.3.14 Gate Results

Evidence label: Ran.

## Diagnostic

- `Ran`: `.venv/bin/python -m py_compile tools/snowfreeze_observed/policy_b_no_regression_cap_authority.py` -> pass.
- `Ran`: `.venv/bin/python tools/snowfreeze_observed/policy_b_no_regression_cap_authority.py --workspace-regression-status not-run` -> pass; correctly closed `HOLD-POLICY-B-NO-REGRESSION-EVIDENCE-INCOMPLETE`.
- `Ran`: `.venv/bin/python tools/snowfreeze_observed/policy_b_no_regression_cap_authority.py --workspace-regression-status pass` -> pass; generated final package report.

## Focused Gates

- `Ran`: `cargo test --test snowdensity10_3_14_policy_b_no_regression_cap_authority -- --nocapture` -> pass.
- `Ran`: `cargo clippy --test snowdensity10_3_14_policy_b_no_regression_cap_authority -- -D warnings` -> pass.

Corrective note: the first selector-scoped workspace attempt reached the new
10.3.14 test and failed because the dry report still recorded
`workspace_regression_status = not-run`. The report was regenerated with the
observed selector run status and the full selector-scoped workspace gate was
rerun to completion.

## Policy-B Workspace Gate

- `Ran`:
  `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=coe_liquid_holding_capacity_v1 OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_density_compaction_v1 cargo test --workspace`
  -> pass.

## Full Closure Gates

- `Ran`: `cargo fmt --check` -> pass.
- `Ran`: `cargo test --workspace` -> pass.
- `Ran`: `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `Ran`: `cargo deny check` -> pass.
- `Ran`: `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `Ran`: `cargo test --test auth11_required_suite_obligation_guards_contract` -> pass.
- `Ran`: `git diff --check` -> pass.

## Source Boundary Scan

- `Ran`: `rg -n "qwet|frzftp" crates || true` -> no production crate hits.
- No default activation, density-cap, production physics, fixture input, output
  schema, parser/runfile/user selector, Qwet/frzftp, frost-attribution, or
  compatibility-runtime change was made.
