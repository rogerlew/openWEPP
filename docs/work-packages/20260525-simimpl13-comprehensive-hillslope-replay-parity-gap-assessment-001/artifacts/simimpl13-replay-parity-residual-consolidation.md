# Simimpl13 replay parity residual consolidation

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- This consolidation supersedes ad hoc residual phrasing by normalizing
  SIMIMPL11 residuals into SIMIMPL13 closure-track IDs.
- Source authority:
  - `simimpl11_disposition.md`
  - `simimpl11-residual-classification-matrix.md`
  - SIMIMPL11 replay evidence bundle:
    `artifacts/replay-run-20260525T001432Z/`

## Ran
- Extracted residual metrics from comparator outputs and provenance manifests:
  - `common_row_count=0`
  - `only_baseline_count=1095`
  - `only_candidate_count=1`
  - candidate-only key example: `(OFE=1, J=1, Y=2000)`
  - strict comparator: `line_count_baseline=1123`, `line_count_candidate=1`
- Confirmed candidate manifest provenance flags:
  - `scheduler_kernel_executed=true`
  - `publication_source="scheduler-kernel"`
  - replay surfaces include `interchange/H.wat.parquet` and
    `interchange/H.pass.parquet`.

## Consolidated residual register
| simimpl13_residual_id | upstream_residual_id | residual statement | evidence anchor | owner surface | status |
|---|---|---|---|---|---|
| `SIMIMPL13-R-ROWKEY-001` | `SIMIMPL11-R-KEYDOMAIN-001` | Baseline and candidate key domains do not overlap (`common_row_count=0`), with candidate keyed at `Y=2000` while baseline keys use simulation-year indexing (`Y=1..3`). | `suite_dat/investigation/h5_wat_semantic_comparator.json`; baseline first/last keyed rows from `/tmp/simimpl11_suite_dat_20260525T001432Z/baseline/lane/output/H5.wat.dat` | runner publication semantics + comparator key-policy alignment | open |
| `SIMIMPL13-R-SPAN-001` | `SIMIMPL11-R-CANDIDATE-SPAN-001` | Candidate WB13 trajectory span is one row while baseline span is 1095 keyed rows (1123 total strict lines including non-numeric/header rows). | `suite_dat/investigation/h5_wat_strict_comparator.json`; candidate `H5.wat.dat`; candidate parquet row count (`duckdb`) | runner simulation timeseries publication path | open |
| `SIMIMPL13-R-TOOLMAP-001` | `SIMIMPL11-R-SEMANTIC-MAP-001` | Parquet semantic lane reports `Total-Soil` as baseline-only / investigation-missing due comparator parquet-to-canonical alias drift. | `suite_parquet/investigation/h5_wat_semantic_comparator.json`; `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py` | legacy comparison suite tooling | open |

## Consolidation conclusion
- SIMIMPL11 Tier-A residuals are now canonicalized for SIMIMPL13.
- No residual is downgraded or reclassified as closed.
- All three residual families remain promotability blockers pending closure-wave
  execution.
