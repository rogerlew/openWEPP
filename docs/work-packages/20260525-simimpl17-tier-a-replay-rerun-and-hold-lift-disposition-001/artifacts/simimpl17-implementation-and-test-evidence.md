# simimpl17-implementation-and-test-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL17 executed as a rerun/disposition package with targeted replay
  blocker fixes applied before rerun evidence capture:
- runner sidecar policy is compat-only (strict removed).
- legacy sidecar discovery tolerates unknown files with `LSB-W-002` warnings.
- soil parser compatibility accepts the policy-first `9002` fixture variant
  used by the shared replay lane.
- Tier-A evidence bundle captured at:
- `artifacts/replay-run-20260525T072842Z/`
- Candidate execution surfaces captured:
- `candidate/openwepp_hillslope_run_manifest.json`
- `candidate/H5.wat.parquet`
- `candidate/H5.wat.dat`
- `candidate/H5.plot.parquet`
- `candidate/H5.loss.json`
- `candidate/H5.hbp`

## Ran
- Candidate execution commands:
- `cargo build -q -p openwepp-runner --bin open_wepp_runner --bin openwepp-cli-hill`
- `./target/debug/open_wepp_runner run-hillslope --hillslope-binary ./target/debug/openwepp-cli-hill --run-dir docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/shared_fixture/runs --run-file case.run --output-dir docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/candidate --policy compat --legacy-sidecar-discovery --manifest-path docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/candidate/openwepp_hillslope_run_manifest.json`
- Replay harness commands:
- `uv pip sync --python docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/venv/bin/python tools/legacy_comparison_suite/requirements.lock.txt`
- `docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/venv/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py --baseline-run-dir docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/shared_fixture --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --baseline-run-file case_legacy.run --candidate-wat docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/candidate/H5.wat.parquet --candidate-surface-source-class native-runtime-parquet --output-root docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/suite_parquet`
- `docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/venv/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py --baseline-run-dir docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/shared_fixture --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --baseline-run-file case_legacy.run --candidate-wat docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/candidate/H5.wat.dat --candidate-surface-source-class conversion-derived-dat --output-root docs/work-packages/20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/artifacts/replay-run-20260525T072842Z/suite_dat`
- Candidate manifest summary:
- `climate_day_count=1095`
- `executed_day_count=1095`
- `wb13_publication.row_count=1095`
- Dat lane outcome:
- non-zero exit with comparator failure (`suite_dat_rc=1`):
  `semantic comparator failed with return code 1`.
