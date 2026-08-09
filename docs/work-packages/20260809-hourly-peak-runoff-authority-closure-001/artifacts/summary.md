# Corrected Exact-Anchor Probe Summary

- **Package**: 20260809-hourly-peak-runoff-authority-closure-001
- **Run**: fresh corrected one-trial probe
- **Overall status**: PASS

1. Build
- Anchor: `949349e7055c5d19277eeb708401c4614a52cd77`
- Binary SHA-256: `569f586516283c633cf4a2c99eb4c89725f8c57c476047b7b03a0b59e327ca88`
- Command: `TMPDIR=/home/workdir/openwepp-task-tmp cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- Status: PASS
- Log: `artifacts/comparator-probe-build-v2.log`

2. Census probe
- Command: `.venv/bin/python .../topanga_openwepp_census.py --evidence-root /home/workdir/openwepp-hourly-peak-topanga-probe-v2 --binary target/release/openwepp-cli-hill --jobs 1 --limit 1`
- Status: PASS
- Log: `artifacts/topanga-openwepp-census-probe-v2.log`

Validation metrics (`/home/workdir/openwepp-hourly-peak-topanga-probe-v2/summary.json`):
- `event_pair_rows`: 1,832
- `finite_positive_peak_pairs`: 1,832
- `invalid_max_hour_fraction_count`: 0
- `max_abs_ratio_decomposition_residual`: 2.220446049250313e-16
- `max_hour_fraction_ratio_max`: 1.0587777478250922
- `max_hour_fraction_ratio_p99`: 1.001064489324337
- `peak_ratio_max`: 19433266351.225124 (near-zero denominator)
- `peak_ratio_p99`: 1.0447344614102623
- `volume_within_5pct_peak_at_least_2x_count`: 0
- `zero_runoff_peak_topology_mismatch_count`: 0
