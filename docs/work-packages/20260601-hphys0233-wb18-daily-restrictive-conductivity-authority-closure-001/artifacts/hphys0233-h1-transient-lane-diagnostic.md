# HPHYS0233 H1 Transient Lane Diagnostic

Status: completed  
Evidence mode: Ran

## Inputs

- Baseline H1 parquet:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet`
- HPHYS0232 candidate H1 parquet:
  `/tmp/hphys0232_20260601T201921Z/parity/hillslope_output/H1.wat.parquet`
- HPHYS0233 candidate H1 parquet:
  `/tmp/hphys0233_20260601T211306Z/parity/hillslope_output/H1.wat.parquet`

## H1 day-1..7 readjudication (baseline vs HPHYS0233)

| day | year | month | dom | `Dp` baseline | `Dp` HPHYS0233 | `Dp` delta | `Total-Soil` baseline | `Total-Soil` HPHYS0233 | `Total-Soil` delta |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2013 | 1 | 1 | 0.240 | 1.647 | +1.407 | 343.070 | 272.172 | -70.898 |
| 2 | 2013 | 1 | 2 | 0.240 | 1.685 | +1.445 | 342.470 | 250.498 | -91.972 |
| 3 | 2013 | 1 | 3 | 0.240 | 1.719 | +1.479 | 341.720 | 248.665 | -93.055 |
| 4 | 2013 | 1 | 4 | 0.240 | 1.750 | +1.510 | 340.920 | 247.392 | -93.528 |
| 5 | 2013 | 1 | 5 | 0.240 | 1.777 | +1.537 | 340.090 | 245.050 | -95.040 |
| 6 | 2013 | 1 | 6 | 0.240 | 1.800 | +1.560 | 339.210 | 243.858 | -95.352 |
| 7 | 2013 | 1 | 7 | 0.240 | 1.819 | +1.579 | 338.220 | 242.103 | -96.117 |

## Delta vs HPHYS0232 candidate (transient improvement check)

| day | `Dp` baseline | `Dp` HPHYS0232 | `Dp` HPHYS0233 | `Dp` (0233-0232) | `Total-Soil` baseline | `Total-Soil` HPHYS0232 | `Total-Soil` HPHYS0233 | `Total-Soil` (0233-0232) |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 0.240 | 39.525 | 1.647 | -37.879 | 343.070 | 254.021 | 272.172 | +18.151 |
| 2 | 0.240 | 67.128 | 1.685 | -65.443 | 342.470 | 186.632 | 250.498 | +63.866 |
| 3 | 0.240 | 41.265 | 1.719 | -39.545 | 341.720 | 145.254 | 248.665 | +103.411 |
| 4 | 0.240 | 13.805 | 1.750 | -12.055 | 340.920 | 131.926 | 247.392 | +115.466 |
| 5 | 0.240 | 20.195 | 1.777 | -18.418 | 340.090 | 111.165 | 245.050 | +133.885 |
| 6 | 0.240 | 11.799 | 1.800 | -9.999 | 339.210 | 99.974 | 243.858 | +143.883 |
| 7 | 0.240 | 7.766 | 1.819 | -5.946 | 338.220 | 92.273 | 242.103 | +149.830 |

## Interpretation

1. Daily-lane restrictive conductivity migration materially reduced H1
   early-transient overdrainage and raised `Total-Soil` relative to HPHYS0232.
2. H1 remains outside baseline parity in both `Dp` and storage columns.
3. Stream disposition remains `HOLD`; additional WB18/WB19 closure is required.
