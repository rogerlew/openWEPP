# simimpl17-tier-a-rerun-execution-evidence-index

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Canonical evidence bundle root:
- `artifacts/replay-run-20260525T062534Z/`
- Integrity manifest:
- `artifacts/replay-run-20260525T062534Z/evidence_sha256sums.txt`

## Ran
- Candidate run manifest and outputs:
- `artifacts/replay-run-20260525T062534Z/candidate/openwepp_hillslope_run_manifest.json`
- `artifacts/replay-run-20260525T062534Z/candidate/H5.wat.parquet`
- `artifacts/replay-run-20260525T062534Z/candidate/H5.wat.dat`
- Parquet lane comparator outputs:
- `artifacts/replay-run-20260525T062534Z/suite_parquet/investigation/pl14s_provenance_manifest.json`
- `artifacts/replay-run-20260525T062534Z/suite_parquet/investigation/h5_wat_semantic_comparator.json`
- Dat lane comparator outputs:
- `artifacts/replay-run-20260525T062534Z/suite_dat/investigation/h5_wat_strict_comparator.json`
- `artifacts/replay-run-20260525T062534Z/suite_dat/investigation/h5_wat_semantic_comparator.json`
- Dat lane provenance manifest:
- not generated (lane terminated by conversion-derived row-consistency guard).

### Key metrics
- Parquet semantic summary:
- `semantic_pass=false`
- `common_row_count=2`
- `only_baseline_count=1093`
- `only_candidate_count=0`
- `investigation_columns_missing=[]`
- Dat strict summary:
- `strict_pass=false`
- `status_counts={"structure_diff":1}`
- `line_count_baseline=1123`, `line_count_candidate=2`
- Dat guard termination reason:
- `conversion-derived dat row-count mismatch: baseline has unmatched replay rows`
