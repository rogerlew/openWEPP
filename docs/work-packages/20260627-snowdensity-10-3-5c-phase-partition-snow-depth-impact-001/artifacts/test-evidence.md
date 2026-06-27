# Test Evidence

Evidence mode: Ran.

## Focused Execution

- `.venv/bin/python -m py_compile tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py` -> PASS.
- `cargo fmt --check && cargo test -p openwepp-meteorology` -> PASS.
  - Adds and passes `bisection_fallback_solves_warm_unsaturated_hydrometeor_temperature`.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill` -> PASS.
- `.venv/bin/python tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py` -> PASS after the solver robustness correction.
  - First run failed before correction on HJ Andrews opt-in:
    `snow.phase.harder_pomeroy_hourly=NaN`.
  - Corrected run completed all fourteen coupled WAT executions.
- `.venv/bin/python tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py --skip-model-runs` -> PASS; regenerated report artifacts from completed WAT outputs.
- `cargo fmt --check && cargo test --test snowdensity10_3_5c_phase_partition_snowdepth_impact` -> PASS (`4 passed`).
- `cargo clippy --workspace --all-targets -- -D warnings` -> PASS after fixing the
  initial `manual-let-else` style finding.
- `cargo test --workspace` -> PASS.
- `cargo deny check` -> PASS.
- `wctl doc-lint --path docs/work-packages` -> PASS (`971 files validated, 0 errors, 0 warnings`).

## Result Summary

The coupled WAT report disposition is
`PHASE-PARTITION-NEUTRAL-OR-WORSE`. Default snow-control failures were `1147`;
opt-in failures were `1273`; all four paired surfaces were worse.
