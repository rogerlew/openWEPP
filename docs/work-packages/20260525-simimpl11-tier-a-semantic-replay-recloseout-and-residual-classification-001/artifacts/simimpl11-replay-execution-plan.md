# simimpl11-replay-execution-plan

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- Baseline authority:
  - run dir: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0`
  - binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
  - run file: `p5.run`
- Candidate emission authority:
  - source fixture: `tests/fixtures/cli01/hillslope_run_dir`
  - runner binary: `target/debug/open_wepp_runner`
  - hillslope binary: `target/debug/openwepp-cli-hill`
- Planned lanes:
  1. semantic lane with parquet candidate (`H5.wat.parquet`),
  2. strict lane with `.dat` candidate (`H5.wat.dat`) to force strict comparator execution branch.

## Ran
- Candidate run root: `/tmp/simimpl11_candidate_20260525T001432Z`
- Semantic output root: `/tmp/simimpl11_suite_parquet_20260525T001432Z`
- Strict output root: `/tmp/simimpl11_suite_dat_20260525T001432Z`
- Evidence snapshot copied to:
  - `artifacts/replay-run-20260525T001432Z/`
