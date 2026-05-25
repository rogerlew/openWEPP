# simimpl18-implementation-and-test-evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Implemented production tooling fix in scope:
  - `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
  - Added explicit baseline-year policy handling:
    - `--baseline-year-policy`
    - `--expected-common-row-count`
    - provenance fields `baseline_year_policy`,
      `expected_common_row_count`, `full_span_policy_ready`, and
      baseline materialization metadata.
- Implemented baseline-year materialization path for comparison lanes
  (`require-expected-common`), producing deterministic 1095-row baseline
  comparison surfaces when baseline binary clamps to one simulation year.
- Reverted non-authoritative provisional physics edits in
  `crates/openwepp-runner/src/lib.rs`; SIMIMPL18 runner physics closure remains
  open pending baseline-authoritative migration work.

## Ran
- Candidate execution command:
  - `cargo build -q -p openwepp-runner --bin open_wepp_runner --bin openwepp-cli-hill`
  - `./target/debug/open_wepp_runner run-hillslope --hillslope-binary ./target/debug/openwepp-cli-hill --run-dir <...>/replay-run-20260525T132822Z/shared_fixture/runs --run-file case.run --output-dir <...>/replay-run-20260525T132822Z/candidate --policy compat --legacy-sidecar-discovery --manifest-path <...>/replay-run-20260525T132822Z/candidate/openwepp_hillslope_run_manifest.json`
- Replay harness commands:
  - `python3 -m venv <...>/replay-run-20260525T132822Z/venv`
  - `uv pip sync --python <...>/replay-run-20260525T132822Z/venv/bin/python tools/legacy_comparison_suite/requirements.lock.txt`
  - `<...>/venv/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py --baseline-run-dir <...>/replay-run-20260525T132822Z/shared_fixture --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --baseline-run-file case_legacy.run --candidate-wat <...>/replay-run-20260525T132822Z/candidate/H5.wat.parquet --candidate-surface-source-class native-runtime-parquet --baseline-year-policy require-expected-common --expected-common-row-count 1095 --output-root <...>/replay-run-20260525T132822Z/suite_parquet`
  - `<...>/venv/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py --baseline-run-dir <...>/replay-run-20260525T132822Z/shared_fixture --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill --baseline-run-file case_legacy.run --candidate-wat <...>/replay-run-20260525T132822Z/candidate/H5.wat.dat --candidate-surface-source-class conversion-derived-dat --baseline-year-policy require-expected-common --expected-common-row-count 1095 --output-root <...>/replay-run-20260525T132822Z/suite_dat`
- Candidate manifest summary:
  - `climate_day_count=1095`
  - `executed_day_count=1095`
  - `wb13_publication.row_count=1095`
- Full evidence bundle:
  - `artifacts/replay-run-20260525T132822Z/`
  - `artifacts/replay-run-20260525T132822Z/evidence_sha256sums.txt`
