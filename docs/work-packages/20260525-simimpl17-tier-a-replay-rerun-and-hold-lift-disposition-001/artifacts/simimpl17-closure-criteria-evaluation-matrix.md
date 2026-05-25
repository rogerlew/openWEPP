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
| `SIMIMPL13-CRIT-001` | fail | `replay-run-20260525T075424Z/suite_parquet/investigation/h5_wat_semantic_comparator.json`; `replay-run-20260525T075424Z/suite_parquet/investigation/baseline_stdout.txt` | Replay span parity remains open: candidate emits `1095` daily rows while legacy baseline run in this lane clamps to one simulation year (`393` rows), leaving `common_row_count=365`. |
| `SIMIMPL13-CRIT-002` | fail | `replay-run-20260525T075424Z/suite_parquet/investigation/h5_wat_semantic_comparator.json` | Required key-domain closure is not met (`only_baseline_count=0`, `only_candidate_count=730`). |
| `SIMIMPL13-CRIT-003` | fail | `replay-run-20260525T075424Z/suite_dat/investigation/h5_wat_strict_comparator.json`; `replay-run-20260525T075424Z/candidate/suite_dat_stderr.log` | Dat strict lane does not pass (`strict_pass=false`, `structure_diff`, line counts `393` vs `1095`); lane exits non-zero (`suite_dat_rc=1`). |
| `SIMIMPL13-CRIT-004` | fail | `replay-run-20260525T075424Z/suite_parquet/investigation/h5_wat_semantic_comparator.json`; `replay-run-20260525T075424Z/suite_dat/investigation/h5_wat_semantic_comparator.json` | Semantic closure remains failing in both lanes (`semantic_pass=false`, `common_row_count=365`, `only_candidate_count=730`). |
| `SIMIMPL13-CRIT-005` | pass | same semantic reports | Investigation-column completeness met (`investigation_columns_missing=[]`, `baseline_only_columns=[]`, `candidate_only_columns=[]`). |
| `SIMIMPL13-CRIT-006` | pass | `replay-run-20260525T075424Z/candidate/openwepp_hillslope_run_manifest.json` | Provenance remains simulation-owned and runner-executed (`scheduler_kernel_executed=true`, `publication_source=scheduler-kernel`, `wb13_publication.source=simulation-owned`). |
| `SIMIMPL13-CRIT-007` | pass | `replay-run-20260525T075424Z/gates/contract_gate_openwepp.stdout.log`; `replay-run-20260525T075424Z/gates/contract_gate_runner.stdout.log` | Contract-derived closure tests for span/key/alias/strict-compensation/provenance coverage executed and passing. |
| `SIMIMPL13-CRIT-008` | partial | `replay-run-20260525T075424Z/`; `replay-run-20260525T075424Z/candidate/suite_dat_stderr.log` | Bundle is reproducible with command/log/hash traces, but dat lane lacks `pl14s_provenance_manifest.json` because closeout stops at conversion-derived row-consistency guard failure. |
