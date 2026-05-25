# simimpl11-replay-blocker-register

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Active blockers
1. `SIMIMPL11-R-KEYDOMAIN-001`
- Impact: no common key rows; semantic tolerance checks cannot evaluate runtime trajectory parity.
- Evidence: both semantic lanes report `common_row_count=0`.
- Owner: SIMIMPL downstream parity closure (SIMIMPL12 intake).

2. `SIMIMPL11-R-CANDIDATE-SPAN-001`
- Impact: strict lane reports structural mismatch (`1123` vs `1` lines).
- Evidence: `h5_wat_strict_comparator.json` with `status=structure_diff`.
- Owner: runner/candidate emission parity closure wave.

3. `SIMIMPL11-R-SEMANTIC-MAP-001`
- Impact: parquet semantic lane omits `Total-Soil` from shared columns.
- Evidence: parquet semantic JSON `baseline_only_columns=["Total-Soil"]`.
- Owner: `tools/legacy_comparison_suite` parquet canonical-field mapping maintenance.
