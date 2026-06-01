# HPHYS0232 H1 Transient Lane Diagnostic

Status: completed  
Evidence mode: Ran

## Inputs

- Baseline H1 parquet:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet`
- Candidate H1 parquet:
  `/tmp/hphys0232_20260601T201921Z/parity/hillslope_output/H1.wat.parquet`

## H1 day-1..7 acceptance trace

| day | year | month | dom | `Dp` baseline | `Dp` candidate | `Dp` delta | `Total-Soil` baseline | `Total-Soil` candidate | `Total-Soil` delta |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2013 | 1 | 1 | 0.240 | 39.525 | +39.285 | 343.070 | 254.021 | -89.049 |
| 2 | 2013 | 1 | 2 | 0.240 | 67.128 | +66.888 | 342.470 | 186.632 | -155.838 |
| 3 | 2013 | 1 | 3 | 0.240 | 41.265 | +41.025 | 341.720 | 145.254 | -196.466 |
| 4 | 2013 | 1 | 4 | 0.240 | 13.805 | +13.565 | 340.920 | 131.926 | -208.994 |
| 5 | 2013 | 1 | 5 | 0.240 | 20.195 | +19.955 | 340.090 | 111.165 | -228.925 |
| 6 | 2013 | 1 | 6 | 0.240 | 11.799 | +11.559 | 339.210 | 99.974 | -239.236 |
| 7 | 2013 | 1 | 7 | 0.240 | 7.766 | +7.526 | 338.220 | 92.273 | -245.947 |

## Interpretation

1. H1 early-transient overdrainage remains materially open.
2. Lane-attenuation migration closes hourly lineage in WB18 contract and kernel,
   but no measurable movement is observed in this daily-lane cohort replay.
3. Stream disposition remains `HOLD`.
