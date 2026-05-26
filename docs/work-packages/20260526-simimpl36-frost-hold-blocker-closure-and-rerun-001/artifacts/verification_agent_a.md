# Verification Agent A

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Static
- none

## Ran
- Replay/suite evidence inspection from:
  - `artifacts/replay-run-20260526T164400Z/`
- Key executed lanes captured by exit-code artifacts:
  - `candidate/openwepp_runner.exit_code=0`
  - `candidate_wc1/openwepp_runner.exit_code=0`
  - `suite_wc1_parquet.exit_code=1`
  - `suite_wc1_partitioned_parquet.exit_code=1`
  - `suite_wc1_year_offset.exit_code=0`
- Comparator/provenance evidence inspection:
  - `.../suite_wc1_year_offset/investigation/h5_wat_semantic_comparator.json`
  - `.../suite_wc1_year_offset/investigation/pl14s_provenance_manifest.json`

## Result
- Blocker closure evidence is reproducible from the replay bundle, with final
  admissible row-key overlap in the year-offset lane.
