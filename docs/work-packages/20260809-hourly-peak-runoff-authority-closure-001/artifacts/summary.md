# Probe summary

- **Package**: 20260809-hourly-peak-runoff-authority-closure-001
- **Run**: comparator probe with resume
- **Overall status**: PASS

1. Build
- Command: `TMPDIR=/home/workdir/openwepp-task-tmp cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- Status: PASS (`exit_code:0`, `duration_seconds:68`)
- Log: `/home/workdir/openWEPP/docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/comparator-probe-build.log`

2. Census probe
- Command: `/home/workdir/openWEPP/.venv/bin/python docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/topanga_openwepp_census.py --plan /workdir/wepppy/docs/work-packages/20260808_peakflow_topanga_census_prep/artifacts/topanga-trial-plan.json --source-root /home/workdir/peakflow-topanga-census-evidence/b575fde4a28cf85f1d28e0dfff305472b5419fd9b3639d39dc437600617080de --evidence-root /home/workdir/openwepp-hourly-peak-topanga-probe --binary target/release/openwepp-cli-hill --jobs 1 --limit 1 --resume`
- Status: PASS (`exit_code:0`, `duration_seconds:4`)
- Log: `/home/workdir/openWEPP/docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-run.log`

Validation metrics (`/home/workdir/openwepp-hourly-peak-topanga-probe/summary.json`):
- `event_pair_rows`: 638
- `finite_positive_peak_pairs`: 638
- `invalid_max_hour_fraction_count`: 0
- `max_abs_ratio_decomposition_residual`: 3.3306690738754696e-16
- `max_hour_fraction_ratio_max`: 1.0350812903034823
- `max_hour_fraction_ratio_p99`: 1.0032257023174906
- `peak_ratio_max`: 6.830159847318132
- `peak_ratio_p99`: 1.2569430238083918
- `volume_within_5pct_peak_at_least_2x_count`: 0
- `zero_runoff_peak_topology_mismatch_count`: 0
