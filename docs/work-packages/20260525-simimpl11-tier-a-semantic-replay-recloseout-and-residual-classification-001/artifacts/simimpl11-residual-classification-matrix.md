# simimpl11-residual-classification-matrix

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Residual matrix
| residual_id | residual statement | evidence | tier classification | owner | status |
|---|---|---|---|---|---|
| `SIMIMPL11-R-KEYDOMAIN-001` | Baseline/candidate row-key domains do not overlap (`common_row_count=0`, `1095` baseline-only, `1` candidate-only with year `2000`). | semantic JSONs (parquet + dat lanes) | Daily Tier-A lane remains investigation-only; no acceptance signal can be claimed. | Runner output parity closure in downstream SIMIMPL queue (SIMIMPL12 disposition intake). | open |
| `SIMIMPL11-R-CANDIDATE-SPAN-001` | Candidate WAT trajectory spans one row while baseline spans `1123` lines / `1095` keyed rows. | strict JSON + semantic JSONs | Investigation blocker for parity closure. | Runner execution/candidate emission parity closure wave. | open |
| `SIMIMPL11-R-SEMANTIC-MAP-001` | Parquet semantic lane reports `baseline_only_columns=["Total-Soil"]` and `investigation_columns_missing=["Total-Soil"]` (mapping drift in semantic comparator parquet alias set). | parquet semantic JSON + provenance | Tooling residual (non-physics), blocks complete column-level semantic comparability in parquet lane. | Legacy comparison suite maintenance queue. | open |
