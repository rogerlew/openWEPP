# simimpl11-implementation-and-test-evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- Package execution produced replay evidence and classification artifacts only.
- No production Rust/kernel source files were modified in SIMIMPL11 scope.
- Evidence bundle persisted at:
  - `artifacts/replay-run-20260525T001432Z/`

## Ran
- Build/runtime commands:
  - `cargo build -p openwepp-runner --bin open_wepp_runner --bin openwepp-cli-hill`
  - `target/debug/open_wepp_runner run-hillslope --hillslope-binary target/debug/openwepp-cli-hill --run-dir /tmp/simimpl11_candidate_20260525T001432Z --run-file case.run --output-dir /tmp/simimpl11_candidate_20260525T001432Z/output --policy strict --manifest-path /tmp/simimpl11_candidate_20260525T001432Z/output/openwepp_hillslope_run_manifest.json`
- Replay harness commands:
  - `uv venv /tmp/simimpl11_venv_20260525T001432Z --python 3.12`
  - `uv pip sync --python /tmp/simimpl11_venv_20260525T001432Z/bin/python tools/legacy_comparison_suite/requirements.lock.txt`
  - `/tmp/simimpl11_venv_20260525T001432Z/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py --baseline-run-dir /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0 --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --baseline-run-file p5.run --candidate-wat /tmp/simimpl11_candidate_20260525T001432Z/output/H5.wat.parquet --output-root /tmp/simimpl11_suite_parquet_20260525T001432Z`
  - `/tmp/simimpl11_venv_20260525T001432Z/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py --baseline-run-dir /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0 --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --baseline-run-file p5.run --candidate-wat /tmp/simimpl11_candidate_20260525T001432Z/output/H5.wat.dat --output-root /tmp/simimpl11_suite_dat_20260525T001432Z`
