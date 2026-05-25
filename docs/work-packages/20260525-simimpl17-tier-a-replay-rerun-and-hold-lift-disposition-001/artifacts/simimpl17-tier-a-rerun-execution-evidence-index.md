# simimpl17-tier-a-rerun-execution-evidence-index

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Canonical evidence bundle root:
- `artifacts/replay-run-20260525T072842Z/`
- Integrity manifest:
- `artifacts/replay-run-20260525T072842Z/evidence_sha256sums.txt`

## Ran
- Candidate run manifest and outputs:
- `artifacts/replay-run-20260525T072842Z/candidate/openwepp_hillslope_run_manifest.json`
- `artifacts/replay-run-20260525T072842Z/candidate/H5.wat.parquet`
- `artifacts/replay-run-20260525T072842Z/candidate/H5.wat.dat`
- Parquet lane comparator outputs:
- `artifacts/replay-run-20260525T072842Z/suite_parquet/investigation/pl14s_provenance_manifest.json`
- `artifacts/replay-run-20260525T072842Z/suite_parquet/investigation/h5_wat_semantic_comparator.json`
- Dat lane comparator outputs:
- `artifacts/replay-run-20260525T072842Z/suite_dat/investigation/h5_wat_strict_comparator.json`
- Dat lane provenance manifest:
- not generated (lane exits non-zero after semantic comparator failure).

### Key metrics
- Parquet semantic summary:
- `semantic_pass=false`
- `common_row_count=365`
- `only_baseline_count=0`
- `only_candidate_count=730`
- `investigation_columns_missing=[]`
- Dat strict summary:
- `strict_pass=false`
- `status_counts={"structure_diff":1}`
- `line_count_baseline=393`, `line_count_candidate=1095`
- Legacy baseline lane warning evidence:
- `suite_parquet/investigation/baseline_stdout.txt` and
  `suite_dat/investigation/baseline_stdout.txt` record
  `Number of years to simulate can't be larger than 1` and
  `SIMULATION YEAR = 1`.
