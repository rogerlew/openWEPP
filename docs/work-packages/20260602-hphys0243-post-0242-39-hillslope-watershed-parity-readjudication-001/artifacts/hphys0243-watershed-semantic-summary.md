# HPHYS0243 Watershed Semantic Summary

Status: complete
Evidence mode: Ran

## Ran

- Compared candidate watershed interchange output:
  `/tmp/hphys0243_20260602T042747Z/parity/watershed_output/interchange`
- Against baseline interchange output:
  `/wc1/runs/un/unpalatable-rind/wepp/output/interchange`
- Report artifacts:
  - `/tmp/hphys0243_20260602T042747Z/parity/reports/watershed/watershed_semantic_summary.json`
  - `/tmp/hphys0243_20260602T042747Z/parity/reports/watershed/watershed_semantic_summary.tsv`

## Row-Shape and Overlap Summary

| File | Baseline rows | Candidate rows | Common rows | Numeric cols compared |
| --- | ---: | ---: | ---: | ---: |
| `chan.out.parquet` | 1461 | 1 | 0 | 0 |
| `chanwb.parquet` | 1461 | 1 | 0 | 0 |
| `chnwb.parquet` | 0 | 1 | 0 | 0 |
| `ebe_pw0.parquet` | 1461 | 1 | 0 | 0 |
| `loss_pw0.all_years.chn.parquet` | 68 | 1 | 0 | 0 |
| `loss_pw0.all_years.class_data.parquet` | 20 | 1 | 0 | 0 |
| `loss_pw0.all_years.hill.parquet` | 156 | 1 | 0 | 0 |
| `loss_pw0.all_years.out.parquet` | 36 | 1 | 0 | 0 |
| `loss_pw0.chn.parquet` | 17 | 1 | 0 | 0 |
| `loss_pw0.class_data.parquet` | 5 | 1 | 1 | 7 |
| `loss_pw0.hill.parquet` | 39 | 1 | 0 | 0 |
| `loss_pw0.out.parquet` | 11 | 1 | 0 | 0 |
| `soil_pw0.parquet` | 24837 | 1 | 0 | 0 |
| `totalwatsed3.parquet` | 1461 | 1 | 0 | 0 |

## Interpretation

- Watershed execution itself passed (`pw0 rc=0`).
- Watershed semantic parity remains investigation-only because current
  openWEPP watershed output emits one-row summary surfaces where the baseline
  interchange contains daily or multi-entity spans.
- The immediate watershed blocker is output temporal/entity span parity, not a
  numerical tolerance failure on aligned rows.
