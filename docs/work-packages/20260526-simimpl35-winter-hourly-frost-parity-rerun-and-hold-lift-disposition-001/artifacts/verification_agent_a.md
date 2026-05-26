# Verification Agent A

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Ran
- `python3 tools/legacy_comparison_suite/run_pl14s_legacy_suite.py ...`
  for:
  - `suite_wc1_parquet*`
  - `suite_wc1_filtered_parquet*`
  - `suite_wc1_filtered_conversion_dat`
- `python3 tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py ...`
  for direct duplicate-key diagnostics and filtered semantic verification.

## Result
- Unfiltered lane failures and filtered-lane admissible summaries are
  reproducible from replay bundle evidence.
