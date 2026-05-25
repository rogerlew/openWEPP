# simimpl17-closure-criteria-evaluation-matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Evaluation authority:
- `docs/work-packages/20260525-simimpl13-comprehensive-hillslope-replay-parity-gap-assessment-001/artifacts/simimpl13-replay-parity-full-closure-criteria.md`

## Ran
| criterion_id | status | evidence | rationale |
|---|---|---|---|
| `SIMIMPL13-CRIT-001` | fail | `replay-run-20260525T062534Z/suite_parquet/investigation/h5_wat_semantic_comparator.json` | Candidate replay span remains collapsed vs baseline (`common_row_count=2`, baseline unmatched rows `1093`). |
| `SIMIMPL13-CRIT-002` | fail | `replay-run-20260525T062534Z/suite_parquet/investigation/h5_wat_semantic_comparator.json` | Required key-domain closure not met (`only_baseline_count=1093`, `only_candidate_count=0`). |
| `SIMIMPL13-CRIT-003` | fail | `replay-run-20260525T062534Z/suite_dat/investigation/h5_wat_strict_comparator.json`; `replay-run-20260525T062534Z/candidate/suite_dat_stderr.log` | Dat strict lane does not pass (`strict_pass=false`, structure diff; conversion-derived dat lane terminates under row-consistency guard). |
| `SIMIMPL13-CRIT-004` | fail | `replay-run-20260525T062534Z/suite_parquet/investigation/h5_wat_semantic_comparator.json`; `replay-run-20260525T062534Z/suite_dat/investigation/h5_wat_semantic_comparator.json` | Semantic comparator remains failing in both lanes (`semantic_pass=false`). |
| `SIMIMPL13-CRIT-005` | pass | same semantic reports | Investigation-column completeness met (`investigation_columns_missing=[]`, `baseline_only_columns=[]`, `candidate_only_columns=[]`). |
| `SIMIMPL13-CRIT-006` | pass | `replay-run-20260525T062534Z/candidate/openwepp_hillslope_run_manifest.json` | Provenance remains simulation-owned and runner-executed (`scheduler_kernel_executed=true`, `publication_source=scheduler-kernel`, `wb13_publication.source=simulation-owned`). |
| `SIMIMPL13-CRIT-007` | pass | `replay-run-20260525T062534Z/gates/contract_gate_openwepp.stdout.log`; `replay-run-20260525T062534Z/gates/contract_gate_runner.stdout.log` | Contract-derived closure tests for span/key/alias/strict-compensation/provenance coverage executed and passing. |
| `SIMIMPL13-CRIT-008` | partial | `replay-run-20260525T062534Z/`; `replay-run-20260525T062534Z/candidate/suite_dat_stderr.log` | Bundle is reproducible with command/log/hash traces, but dat lane lacks `pl14s_provenance_manifest.json` because execution halts on required conversion-derived row-consistency guard. |
