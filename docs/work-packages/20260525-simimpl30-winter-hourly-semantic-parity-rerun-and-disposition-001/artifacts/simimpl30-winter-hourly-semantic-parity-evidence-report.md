# simimpl30 winter hourly semantic parity evidence report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- SIMIMPL30 replay authority and queue sequencing were sourced from:
  - SNOWPLAN01 queue (`SIMIMPL30` as closure gate),
  - SIMIMPL29 disposition/handoff (carry-forward HOLD until frost-hourly closure and downstream parity rerun).
- Comparator governance for hourly surfaces remains investigation-tier (`COMPMETA-I-HOURLY-001`).

## Ran
Replay bundle: `artifacts/replay-run-20260526T125111Z/`
- Note: `replay-run-*` directories are git-ignored by repository policy; this
  report captures the durable evidence required for disposition.

1. Native parquet lane via `run_pl14s_legacy_suite.py` (`source-class=native-runtime-parquet`, baseline-year policy `require-expected-common`):
- Result: failed (`suite_p5_parquet.exit_code=1`).
- Immediate failure: `semantic comparator failed with return code 1`.

2. Native parquet lane with fallback baseline policy (`passthrough`):
- Result: failed (`suite_p5_parquet_passthrough.exit_code=1`).
- Immediate failure: `semantic comparator failed with return code 1`.

3. Direct semantic comparator diagnosis on native parquet:
- Result: failed (`semantic_direct_p5.exit_code=1`).
- Root cause: duplicate row key in parquet input:
  - `RuntimeError: duplicate row key (1, 1, 1997) ... H.wat.parquet`.

4. `open_wepp_runner run-hillslope` attempt using `/wc1/.../p5.run`:
- Result: failed.
- Root cause: run file is not TOML parseable by this path:
  - `CLIHILL-E-010 parse failure ... invalid TOML ... expected '.' or '='`.

5. Conversion-derived dat candidate lane (`wepp_id=5`) with semantic + strict comparators:
- Result: suite failed (`suite_p5_conversion_dat.exit_code=1`) on baseline-year policy requirement:
  - `expected 1095, observed 0` common rows.
- Semantic report (`investigation/h5_wat_semantic_comparator.json`):
  - `common_row_count: 0`
  - `only_baseline_count: 1095`
  - `only_candidate_count: 1095`
  - Baseline key examples use years `1997..1999`; candidate key examples use `1..3`.
- Strict report (`investigation/h5_wat_strict_comparator.json`):
  - `strict_pass: false`
  - `status_counts.structure_diff: 1`
  - `line_count_baseline: 1123`, `line_count_candidate: 1095`

## Residual classification
- Residual class: comparability/investigation residual (not promotable closure signal).
- Confidence tier posture: hourly lane remains investigation-tier by design.
- Admissibility conclusion: no successful winter-hourly parity lane was produced for hold-lift promotion in this package.
