# simimpl17-implementation-and-test-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL17 executed as a rerun/disposition package with no production code
  edits.
- Tier-A evidence bundle captured at:
- `artifacts/replay-run-20260525T062534Z/`
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
- `./target/debug/open_wepp_runner run-hillslope --hillslope-binary ./target/debug/openwepp-cli-hill --run-dir /tmp/simimpl17_replay_20260525T062534Z/candidate/run_dir --run-file case.run --output-dir /tmp/simimpl17_replay_20260525T062534Z/candidate/output --policy strict --manifest-path /tmp/simimpl17_replay_20260525T062534Z/candidate/output/openwepp_hillslope_run_manifest.json`
- Replay harness commands:
- `uv venv /tmp/simimpl17_replay_20260525T062534Z/venv --python 3.12`
- `uv pip sync --python /tmp/simimpl17_replay_20260525T062534Z/venv/bin/python tools/legacy_comparison_suite/requirements.lock.txt`
- `/tmp/simimpl17_replay_20260525T062534Z/venv/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py --baseline-run-dir /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0 --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --baseline-run-file p5.run --candidate-wat /tmp/simimpl17_replay_20260525T062534Z/candidate/run_dir/output/H5.wat.parquet --candidate-surface-source-class native-runtime-parquet --output-root /tmp/simimpl17_replay_20260525T062534Z/suite_parquet`
- `/tmp/simimpl17_replay_20260525T062534Z/venv/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py --baseline-run-dir /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0 --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --baseline-run-file p5.run --candidate-wat /tmp/simimpl17_replay_20260525T062534Z/candidate/run_dir/output/H5.wat.dat --candidate-surface-source-class conversion-derived-dat --output-root /tmp/simimpl17_replay_20260525T062534Z/suite_dat`
- Dat lane outcome:
- non-zero exit with expected guard message:
  `conversion-derived dat row-consistency requirements not satisfied: conversion-derived dat row-count mismatch: baseline has unmatched replay rows`.
