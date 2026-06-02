# HPHYS0244 H1/H7/H39 Storage and Dp Summary

Ran: joined baseline and candidate WAT rows for `H1`, `H7`, and `H39` using
`ofe_id`, `julian`, and `year + 2012` candidate offset.

## Inputs
- Baseline:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H*.parquet`
- Candidate:
  `/tmp/hphys0243_20260602T042747Z/parity/hillslope_output/H*.wat.parquet`
- Output root:
  `/tmp/hphys0244_20260602T045926Z`

All three hillslopes joined `1461/1461` baseline rows to `1461/1461`
candidate rows.

## Summary
| Hillslope | Column | Mean Signed Δ candidate-baseline (mm) | Mean Abs Δ (mm) | Max Abs Δ (mm) | Day-1 Δ (mm) | Day-1..7 Mean Δ (mm) | Day-1..30 Mean Δ (mm) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `H1` | `Dp` | `-0.077336` | `0.341108` | `44.004399` | `44.004399` | `24.512059` | `6.309401` |
| `H1` | `Total-Soil` | `-176.689463` | `183.198107` | `573.768037` | `-113.495625` | `-191.819552` | `-242.163352` |
| `H1` | `SoilWaterTotal` | `-176.689463` | `183.198107` | `573.768037` | `-113.495625` | `-191.819552` | `-242.163352` |
| `H7` | `Dp` | `-0.096054` | `0.279306` | `33.612610` | `33.612610` | `17.883903` | `4.343834` |
| `H7` | `Total-Soil` | `-133.072479` | `139.195998` | `550.795901` | `-117.530510` | `-171.992073` | `-197.627454` |
| `H7` | `SoilWaterTotal` | `-133.072479` | `139.195998` | `550.795901` | `-117.530510` | `-171.992073` | `-197.627454` |
| `H39` | `Dp` | `-0.064031` | `0.234643` | `22.740342` | `22.740342` | `16.061610` | `4.039694` |
| `H39` | `Total-Soil` | `-62.976312` | `89.677059` | `471.175736` | `-166.340456` | `-210.062828` | `-194.702661` |
| `H39` | `SoilWaterTotal` | `-62.976312` | `89.677059` | `471.175736` | `-166.340456` | `-210.062828` | `-194.702661` |

## First-Week Signature
| Hillslope | Day | Baseline `Dp` | Candidate `Dp` | `Dp` Δ | Baseline Total Soil | Candidate Total Soil | Total Soil Δ |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `H1` | 1 | `0.240000` | `44.244399` | `44.004399` | `343.070000` | `229.574375` | `-113.495625` |
| `H1` | 2 | `0.240000` | `40.545476` | `40.305476` | `342.470000` | `188.767905` | `-153.702095` |
| `H1` | 3 | `0.240000` | `30.670602` | `30.430602` | `341.720000` | `157.983750` | `-183.736250` |
| `H1` | 4 | `0.240000` | `21.991933` | `21.751933` | `340.920000` | `136.468530` | `-204.451470` |
| `H1` | 5 | `0.240000` | `15.767278` | `15.527278` | `340.090000` | `120.135512` | `-219.954488` |
| `H1` | 6 | `0.240000` | `11.473463` | `11.233463` | `339.210000` | `109.270045` | `-229.939955` |
| `H1` | 7 | `0.240000` | `8.571261` | `8.331261` | `338.220000` | `100.763023` | `-237.456977` |
| `H7` | 1 | `0.240000` | `33.852610` | `33.612610` | `287.680000` | `170.149490` | `-117.530510` |
| `H7` | 2 | `0.240000` | `26.807781` | `26.567781` | `286.970000` | `143.080715` | `-143.889285` |
| `H7` | 3 | `0.240000` | `22.126783` | `21.886783` | `286.070000` | `120.840379` | `-165.229621` |
| `H7` | 4 | `0.240000` | `17.142684` | `16.902684` | `285.090000` | `104.174408` | `-180.915592` |
| `H7` | 5 | `0.240000` | `12.451213` | `12.211213` | `284.000000` | `91.157455` | `-192.842545` |
| `H7` | 6 | `0.240000` | `8.645849` | `8.405849` | `282.720000` | `83.119601` | `-199.600399` |
| `H7` | 7 | `0.240000` | `5.840399` | `5.600399` | `281.280000` | `77.343442` | `-203.936558` |
| `H39` | 1 | `0.240000` | `22.980342` | `22.740342` | `386.590000` | `220.249544` | `-166.340456` |
| `H39` | 2 | `0.240000` | `22.311615` | `22.071615` | `384.710000` | `197.676935` | `-187.033065` |
| `H39` | 3 | `0.240000` | `21.067140` | `20.827140` | `382.090000` | `176.496242` | `-205.593758` |
| `H39` | 4 | `0.240000` | `17.594840` | `17.354840` | `378.340000` | `159.378116` | `-218.961884` |
| `H39` | 5 | `0.240000` | `13.451551` | `13.211551` | `373.590000` | `145.360824` | `-228.229176` |
| `H39` | 6 | `0.240000` | `9.767559` | `9.527559` | `368.040000` | `136.201260` | `-231.838740` |
| `H39` | 7 | `0.240000` | `6.938220` | `6.698220` | `361.770000` | `129.327280` | `-232.442720` |

## Extrema
| Hillslope | Metric | Max/First Case | Baseline Year | Julian | Sim Day | Value |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| `H1` | `Dp_delta` | max abs | `2013` | `1` | `1` | `44.004399` |
| `H1` | `TotalSoil_delta` | max abs | `2014` | `145` | `510` | `-573.768037` |
| `H1` | `Dp_candidate<=baseline+1mm` | first occurrence | `2013` | `13` | `13` | `1.097315` |
| `H7` | `Dp_delta` | max abs | `2013` | `1` | `1` | `33.612610` |
| `H7` | `TotalSoil_delta` | max abs | `2014` | `146` | `511` | `-550.795901` |
| `H7` | `Dp_candidate<=baseline+1mm` | first occurrence | `2013` | `11` | `11` | `1.035594` |
| `H39` | `Dp_delta` | max abs | `2013` | `1` | `1` | `22.740342` |
| `H39` | `TotalSoil_delta` | max abs | `2014` | `145` | `510` | `-471.175736` |
| `H39` | `Dp_candidate<=baseline+1mm` | first occurrence | `2013` | `12` | `12` | `0.930559` |

Full tables:
- `/tmp/hphys0244_20260602T045926Z/storage_dp_summary.tsv`
- `/tmp/hphys0244_20260602T045926Z/first_30_storage_dp_timeseries.tsv`
- `/tmp/hphys0244_20260602T045926Z/early_transient_extrema.tsv`

## Finding
The three hillslopes show the same coupled signature:
- `Dp` is a large early-transient overdrain signal and then becomes small enough
  that the full-period signed mean is negative.
- `Total-Soil` and `SoilWaterTotal` are identical in every reported summary,
  confirming shared publication lineage in this cohort.
- Candidate storage is persistently below baseline for most rows:
  `88.8%` of `H1`, `82.1%` of `H7`, and `69.3%` of `H39` rows for
  `Total-Soil`.

This points at WB11/WB18 mutable storage and percolation update ordering, not a
standalone WAT publication-column naming issue.
