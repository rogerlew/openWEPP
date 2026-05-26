# SIMIMPL35 Winter Hourly Semantic Parity Evidence Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- SIMIMPL35 authority chain confirmed from FROSTPLAN01 queue and
  SIMIMPL31/32/33/34 dispositions.
- Comparator governance remains investigation-tier for hourly surfaces;
  promotability depends on lane admissibility plus provenance.

## Ran
Replay bundle: `artifacts/replay-run-20260526T160058Z/`

1. Fresh post-SIMIMPL34 candidate generation from shared fixture
(`shared_fixture/runs/case.run`):
- Result: failed (`candidate/openwepp_runner.stderr.log`).
- Error: `CLIHILL-E-011 ... message_id=KWRITEBACK-E-DOMAIN-VIOLATION`.

2. Direct `/wc1` candidate generation via open_wepp_runner with custom runfile:
- Result: failed (`candidate_wc1/openwepp_runner.stderr.log`).
- Error: `SOL-E-006 ... variant arity mismatch: expected 9 token(s), found 13`.

3. Unfiltered `/wc1` candidate parquet lane (`H.wat.parquet`) using
`run_pl14s_legacy_suite.py`:
- `suite_wc1_parquet.exit_code=1`
- `suite_wc1_parquet_passthrough.exit_code=1`
- direct semantic comparator failed with duplicate key:
  `RuntimeError: duplicate row key (1, 1, 1997)`.

4. Filtered lane (`wepp_id=5`) derived from `/wc1` candidate parquet:
- Filtering evidence:
  - `candidate_wc1_filtered/filter.stdout.log`: `source_rows=35040`,
    `filtered_rows=1095`.
- Native parquet lane:
  - `suite_wc1_filtered_parquet.exit_code=0`
  - `suite_wc1_filtered_parquet_passthrough.exit_code=0`
  - semantic summary:
    - `semantic_pass=true`
    - `common_row_count=1095`
    - `only_baseline_count=0`
    - `only_candidate_count=0`
- Conversion-derived dat lane (from filtered parquet):
  - `suite_wc1_filtered_conversion_dat.exit_code=0`
  - semantic summary:
    - `semantic_pass=true`
    - `common_row_count=1095`
    - `only_baseline_count=0`
    - `only_candidate_count=0`
  - strict summary:
    - strict comparator executed but `strict_pass=false`
    - source class is `conversion-derived-dat` (non-promotable for final
      Tier-A closeout by suite policy metadata).

5. Provenance timing check for post-SIMIMPL34 freshness:
- SIMIMPL34 commit timestamp:
  - `9b8c25a 2026-05-26T08:53:20-07:00`
- Source `/wc1` parquet timestamp used for filtered lane:
  - `2026-05-25 22:49:42 -0700`
- Conclusion: admissible filtered lanes are derived from pre-SIMIMPL34
  candidate output and therefore do not prove post-SIMIMPL34 rerun closure.

## Residual classification
- Admissibility:
  - unfiltered `/wc1` lane: non-admissible (duplicate key)
  - filtered `/wc1` lane: admissible semantic lane (non-zero overlap)
- Hold-lift promotability for SIMIMPL35 objective: not met due inability to
  produce a fresh post-SIMIMPL34 candidate run reaching comparator stage.
