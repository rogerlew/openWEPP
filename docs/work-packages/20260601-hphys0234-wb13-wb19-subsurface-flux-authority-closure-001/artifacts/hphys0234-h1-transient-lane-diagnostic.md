# HPHYS0234 H1 Transient Lane Diagnostic

Status: completed  
Evidence mode: Ran

## Inputs

- Baseline H1 parquet:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet`
- HPHYS0233 candidate H1 parquet:
  `/tmp/hphys0233_20260601T211306Z/parity/hillslope_output/H1.wat.parquet`
- HPHYS0234 candidate H1 parquet:
  `/tmp/hphys0234_20260601T215019Z/parity/hillslope_output/H1.wat.parquet`

## H1 day-1..7 readjudication (baseline vs HPHYS0233 vs HPHYS0234)

| sim day | month | day | `Dp` baseline | `Dp` HPHYS0233 | `Dp` HPHYS0234 | `Dp` delta (0234-0233) | `latqcc` baseline | `latqcc` HPHYS0233 | `latqcc` HPHYS0234 | `latqcc` delta (0234-0233) | `Total-Soil` baseline | `Total-Soil` HPHYS0233 | `Total-Soil` HPHYS0234 | `Total-Soil` delta (0234-0233) |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1 | 1 | 0.240000 | 1.646895 | 1.646895 | +0.000000 | 0.040000 | 19.728001 | 19.728001 | +0.000000 | 343.070000 | 272.171879 | 272.171879 | +0.000000 |
| 2 | 1 | 2 | 0.240000 | 1.684804 | 1.684804 | +0.000000 | 0.090000 | 19.728001 | 19.728001 | +0.000000 | 342.470000 | 250.498080 | 250.498080 | +0.000000 |
| 3 | 1 | 3 | 0.240000 | 1.719109 | 1.719109 | +0.000000 | 0.150000 | 0.000000 | 0.000000 | +0.000000 | 341.720000 | 248.665419 | 248.665419 | +0.000000 |
| 4 | 1 | 4 | 0.240000 | 1.749937 | 1.749937 | +0.000000 | 0.240000 | 0.000000 | 0.000000 | +0.000000 | 340.920000 | 247.392195 | 247.392195 | +0.000000 |
| 5 | 1 | 5 | 0.240000 | 1.776757 | 1.776757 | +0.000000 | 0.350000 | 0.000000 | 0.000000 | +0.000000 | 340.090000 | 245.049697 | 245.049697 | +0.000000 |
| 6 | 1 | 6 | 0.240000 | 1.799815 | 1.799815 | +0.000000 | 0.410000 | 0.000000 | 0.000000 | +0.000000 | 339.210000 | 243.857878 | 243.857878 | +0.000000 |
| 7 | 1 | 7 | 0.240000 | 1.819348 | 1.819348 | +0.000000 | 0.460000 | 0.000000 | 0.000000 | +0.000000 | 338.220000 | 242.102769 | 242.102769 | +0.000000 |

## H1 full-span delta (HPHYS0234 vs HPHYS0233)

- `Dp`: mean_abs_diff=`0.000000000000000`, max_abs_diff=`0.000000000000000`
- `latqcc`: mean_abs_diff=`0.000000000000000`, max_abs_diff=`0.000000000000000`
- `Total-Soil`: mean_abs_diff=`0.000000000000000`, max_abs_diff=`0.000000000000000`
- `SoilWaterTotal`: mean_abs_diff=`0.000000000000000`, max_abs_diff=`0.000000000000000`
- `ProfileFCStore`: mean_abs_diff=`0.000000000000000`, max_abs_diff=`0.000000000000000`

## Interpretation

1. HPHYS0234 WB13 anti-shadow hardening made no H1 transient change relative to
   HPHYS0233 for monitored columns.
2. Baseline-to-candidate deviations remain large for `Dp`, `latqcc`, and
   storage columns.
3. Stream disposition remains `HOLD`.
