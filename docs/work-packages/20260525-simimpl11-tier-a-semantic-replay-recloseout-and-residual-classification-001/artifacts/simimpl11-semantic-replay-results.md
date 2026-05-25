# simimpl11-semantic-replay-results

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- Semantic outputs:
  - parquet lane: `artifacts/replay-run-20260525T001432Z/suite_parquet/investigation/h5_wat_semantic_comparator.json`
  - dat lane: `artifacts/replay-run-20260525T001432Z/suite_dat/investigation/h5_wat_semantic_comparator.json`
- Provenance manifests:
  - parquet lane: `.../suite_parquet/investigation/pl14s_provenance_manifest.json`
  - dat lane: `.../suite_dat/investigation/pl14s_provenance_manifest.json`

## Ran
- Parquet lane summary:
  - `semantic_pass=false`
  - `common_row_count=0`
  - `only_baseline_count=1095`
  - `only_candidate_count=1`
  - `only_candidate_examples=[[1, 1, 2000]]`
  - `baseline_only_columns=["Total-Soil"]`
  - `investigation_columns_missing=["Total-Soil"]`
- Dat lane summary:
  - `semantic_pass=false`
  - `common_row_count=0`
  - `only_baseline_count=1095`
  - `only_candidate_count=1`
  - `only_candidate_examples=[[1, 1, 2000]]`
  - `baseline_only_columns=[]`
  - `investigation_columns_missing=[]`
- SHA256 fingerprints:
  - parquet semantic JSON: `6b158444e4c443ec9d181ede90c82d57a7a2b51797541f469015b754f3e29639`
  - dat semantic JSON: `884f97e66eb20cd23ba4f2ee3d173f96b0e87af242e2a3e4a95f029b0051cc23`
