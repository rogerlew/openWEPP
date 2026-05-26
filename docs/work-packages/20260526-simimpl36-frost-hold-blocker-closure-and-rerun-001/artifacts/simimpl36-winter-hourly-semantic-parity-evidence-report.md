# SIMIMPL36 Winter Hourly Semantic Parity Evidence Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- SIMIMPL36 scope is SIMIMPL35 blocker closure and fresh rerun admissibility.
- Comparator governance remains semantic-tier investigation for parquet lanes.

## Ran
Replay bundle: `artifacts/replay-run-20260526T164400Z/`

1. Shared-fixture post-fix candidate rerun
- Command class: `openwepp-cli-hill` via `open_wepp_runner`.
- Result: success (`candidate/openwepp_runner.exit_code=0`).
- Evidence: no runtime stderr blocker; manifest records
  `scheduler_outcome_class=completed` and executed day count `1095`.

2. Direct `/wc1` post-fix candidate rerun
- Command class: `openwepp-cli-hill` via custom `case_wc1.run`.
- Result: success (`candidate_wc1/openwepp_runner.exit_code=0`).
- Evidence: no `SOL-E-006`; only sidecar warning `WUI-W-001` (daily fallback).

3. Suite attempt without candidate year mapping
- `suite_wc1_parquet.exit_code=1`.
- Failure evidence: baseline-year-policy requirements mismatch with
  `common_row_count=0` from calendar-year vs simulation-year key mismatch.

4. Suite attempt with parquet partition value only
- `suite_wc1_partitioned_parquet.exit_code=1`.
- Failure evidence: selected partition had no rows in fresh candidate surface,
  so semantic lane remained non-admissible in that configuration.

5. Final `/wc1` semantic lane with candidate year-key offset
- `suite_wc1_year_offset.exit_code=0`.
- Comparator summary (`h5_wat_semantic_comparator.json`):
  - `common_row_count=1095`
  - `only_baseline_count=0`
  - `only_candidate_count=0`
  - `semantic_pass=false` (value deltas remain)
- Provenance summary (`pl14s_provenance_manifest.json`):
  - `strict_equivalent_ready=true`
  - `strict_source_promotable_for_final_tier_a_closeout=true`

## Residual classification
- Blocker closure classification: PASS for SIMIMPL35 blockers.
- Full semantic value parity: NOT CLOSED in SIMIMPL36 scope (`semantic_pass=false`).
