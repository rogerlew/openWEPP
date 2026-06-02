# HPHYS0243 Hillslope Semantic Summary

Status: complete
Evidence mode: Ran

## Ran

- Semantic comparator:
  - `/workdir/wepppy/.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
  - baseline: `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H{i}.parquet`
  - candidate: `/tmp/hphys0243_20260602T042747Z/parity/hillslope_output/H{i}.wat.parquet`
  - `--candidate-year-offset 2012`
  - tolerance config: `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- Result: `39/39` comparator runs completed with `rc=0`.
- Row overlap: `min_common_row_count=1461`, `max_common_row_count=1461`.
- Semantic pass count: `0/39`.

## Current Top Residuals

| Column | Hillslope fail count | Mean abs diff mean | Max abs diff |
| --- | ---: | ---: | ---: |
| `SoilWaterTotal` | 39/39 | 140.707505 | 578.117583 |
| `Total-Soil` | 39/39 | 140.707505 | 578.117583 |
| `Snow-Water` | 39/39 | 91.221051 | 562.470000 |
| `Es` | 39/39 | 3.340827 | 10.028919 |
| `RM` | 39/39 | 2.486510 | 45.740000 |
| `Ep` | 39/39 | 1.739422 | 7.780000 |
| `latqcc` | 39/39 | 0.785638 | 118.431335 |
| `Dp` | 39/39 | 0.288527 | 45.704712 |
| `ProfileFCStore` | 27/39 | 2.052691 | 9.334426 |
| `ProfileWPStore` | 1/39 | 0.057297 | 1.669863 |
| `ProfilePorosityCap` | 0/39 | 0.020913 | 0.143103 |
| `Area` | 0/39 | 0.000513 | 0.010000 |

## Prior Trend

| Column | HPHYS0229 | HPHYS0234 | HPHYS0236 | HPHYS0243 |
| --- | ---: | ---: | ---: | ---: |
| `Dp` | 39/39; 0.324920 | 39/39; 0.223504 | 39/39; 0.288527 | 39/39; 0.288527 |
| `latqcc` | 39/39; 0.751768 | 39/39; 0.790397 | 39/39; 0.785638 | 39/39; 0.785638 |
| `Total-Soil` | 39/39; 140.708961 | 39/39; 134.129092 | 39/39; 140.707505 | 39/39; 140.707505 |
| `SoilWaterTotal` | 39/39; 140.708961 | 39/39; 134.129092 | 39/39; 140.707505 | 39/39; 140.707505 |
| `ProfileFCStore` | 27/39; 2.052691 | 27/39; 2.052691 | 27/39; 2.052691 | 27/39; 2.052691 |
| `Q` | 39/39; 1.014132 | — | — | 0/39; 0.000000 |
| `Ep` | 39/39; 1.739422 | — | — | 39/39; 1.739422 |
| `Es` | 39/39; 3.340827 | — | — | 39/39; 3.340827 |
| `RM` | 39/39; 2.298138 | — | — | 39/39; 2.486510 |
| `Snow-Water` | 39/39; 58.179851 | — | — | 39/39; 91.221051 |

## Directional Probe

Representative signed deltas show candidate-minus-baseline storage is negative
while first-week `Dp` is much larger than baseline:

- `H1` day 1..7 `Dp`: baseline `0.24 mm/day`; candidate
  `44.244, 40.545, 30.671, 21.992, 15.767, 11.473, 8.571 mm/day`.
- `H1` day 1..7 `Total-Soil`: baseline declines `343.1 -> 338.2 mm`;
  candidate declines `229.6 -> 100.8 mm`.
- `H1` mean signed `Total-Soil` delta: `-176.689463 mm`.
- `H7` mean signed `Total-Soil` delta: `-133.072479 mm`.
- `H39` mean signed `Total-Soil` delta: `-62.976312 mm`.

## Artifacts

- `/tmp/hphys0243_20260602T042747Z/parity/reports/semantic_status.tsv`
- `/tmp/hphys0243_20260602T042747Z/parity/reports/semantic/H*.semantic.json`
- `/tmp/hphys0243_20260602T042747Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0243_20260602T042747Z/parity/reports/hillslope_semantic_summary.tsv`
