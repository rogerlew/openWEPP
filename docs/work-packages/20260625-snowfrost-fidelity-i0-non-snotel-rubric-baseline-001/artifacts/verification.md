# Verification

Evidence mode: Ran.

Commands:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate --observations-dir tests/fixtures/snowfreeze_observed/observations`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/non_snotel_rubric_baseline.py tools/snowfreeze_observed/observed_harness.py tools/snowfreeze_observed/snotel_density_three_way.py`
- `.venv/bin/python tools/snowfreeze_observed/non_snotel_rubric_baseline.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfrost_fidelity_i0_non_snotel_rubric_baseline --binary target/release/openwepp-cli-hill`
- `jq -e '.schema == "snowfreeze-non-snotel-rubric-baseline-v1" and .site_count == 5 and .summary.openwepp_defective_cells == 0 and .summary.production_physics_changed == false' docs/work-packages/20260625-snowfrost-fidelity-i0-non-snotel-rubric-baseline-001/artifacts/non_snotel_rubric_baseline.json`
- `git diff --check`
- `wctl doc-lint --path docs/work-packages/README.md`

Generated:

- `target/snowfrost_fidelity_i0_non_snotel_rubric_baseline/non_snotel_rubric_baseline.{json,md}`
- `target/snowfrost_fidelity_i0_non_snotel_rubric_baseline/site_reports/*/comparison_report.{json,md}`
- Package copies:
  - `artifacts/non_snotel_rubric_baseline.json`
  - `artifacts/non_snotel_rubric_baseline.md`

Result:

- Site count: `5`
- Snow-control status counts:
  - `SNOW_CONTROL_FAILED`: `3`
  - `MODELED_SNOW_DEPTH_DIAGNOSTIC_PRESENT_NO_PAIRED_OBSERVED_SNOW`: `2`
- Rubric counts: `{'fail': 19, 'marginal': 8, 'pass': 5, 'strong': 20, 'unavailable': 63}`
- Forcing-robust rubric counts: `{'fail': 9, 'marginal': 7, 'pass': 4, 'strong': 20, 'unavailable': 45}`
- `openwepp_defective_cells`: `0`
- `production_physics_changed`: `false`

All listed gates passed. `wctl doc-lint` validated `docs/work-packages/README.md`;
other touched Markdown paths are covered by `git diff --check` rather than a
repo-wide Markdown lint pass.
