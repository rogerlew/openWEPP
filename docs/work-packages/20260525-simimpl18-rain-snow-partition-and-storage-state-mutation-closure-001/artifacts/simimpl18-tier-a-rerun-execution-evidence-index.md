# simimpl18-tier-a-rerun-execution-evidence-index

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Canonical evidence bundle root:
  - `artifacts/replay-run-20260525T132822Z/`
- Integrity manifest:
  - `artifacts/replay-run-20260525T132822Z/evidence_sha256sums.txt`
- Shared-input hash manifest:
  - `artifacts/replay-run-20260525T132822Z/shared_fixture/input_file_sha256.txt`

## Ran
- Candidate outputs:
  - `candidate/openwepp_hillslope_run_manifest.json`
  - `candidate/H5.hbp`
  - `candidate/H5.wat.parquet`
  - `candidate/H5.wat.dat`
- Parquet lane outputs:
  - `suite_parquet/investigation/pl14s_provenance_manifest.json`
  - `suite_parquet/investigation/h5_wat_semantic_comparator.json`
- Dat lane outputs:
  - `suite_dat/investigation/pl14s_provenance_manifest.json`
  - `suite_dat/investigation/h5_wat_strict_comparator.json`
  - `suite_dat/investigation/h5_wat_semantic_comparator.json`
- Gate logs:
  - `gates/gate_exit_codes.log`
  - `gates/fmt.stdout.log`
  - `gates/clippy.stdout.log`
  - `gates/test.stdout.log`
  - `gates/deny.stdout.log`

### Key metrics
- Parquet semantic summary:
  - `semantic_pass=false`
  - `common_row_count=1095`
  - `only_baseline_count=0`
  - `only_candidate_count=0`
- Dat strict summary:
  - `strict_pass=false`
  - `status_counts={"numeric_diff_exceeds_tol":1}`
- Baseline-year policy materialization:
  - `policy_applied=true`, `row_count_before=365`, `row_count_after=1095`.
