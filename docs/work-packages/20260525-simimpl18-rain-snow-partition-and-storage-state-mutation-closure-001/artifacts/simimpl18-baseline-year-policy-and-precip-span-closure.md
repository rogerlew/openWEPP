# simimpl18-baseline-year-policy-and-precip-span-closure

Status: complete-with-open-items
Evidence mode: ran
Date: 2026-05-25

## Static
- SIMIMPL18 requires explicit baseline-year policy handling so candidate and
  baseline comparisons run on a common 1095-key horizon.

## Ran
- Replay lanes executed with explicit policy:
  - `--baseline-year-policy require-expected-common`
  - `--expected-common-row-count 1095`
- Provenance evidence:
  - `artifacts/replay-run-20260525T132822Z/suite_parquet/investigation/pl14s_provenance_manifest.json`
  - `artifacts/replay-run-20260525T132822Z/suite_dat/investigation/pl14s_provenance_manifest.json`
- Policy materialization result (both lanes):
  - `baseline_year_policy_materialization.policy_applied=true`
  - `row_count_before=365`
  - `row_count_after=1095`
  - `replicated_years=3`
  - `full_span_policy_ready=true`
- Common-row closure achieved:
  - `common_row_count=1095`
  - `only_baseline_count=0`
  - `only_candidate_count=0`
- Baseline binary warnings still present in stdout logs (policy-handled):
  - `Number of years to simulate can't be larger than 1`
  - `1 years used`

## Precipitation parity status
- Full-span precipitation parity remains failing in semantic reports:
  - parquet lane `P.fail_count=447`.
  - dat lane `P.fail_count=446`.
- Relevant evidence:
  - `suite_parquet/investigation/h5_wat_semantic_comparator.json`
  - `suite_dat/investigation/h5_wat_semantic_comparator.json`

## Interpretation
- Baseline-year policy and key-span closure are implemented and evidenced.
- Full-span precipitation parity is still open.
