# H1 Day-1 Storage Localization

Status: complete

Evidence mode: ran

Ran:

- Run root: `/tmp/hphys0253_20260602T203448Z`.
- Trace: `/tmp/hphys0253_20260602T203448Z/hillslope_output/H1.hphys0253.trace.jsonl`.
- Candidate WAT: `/tmp/hphys0253_20260602T203448Z/hillslope_output/H1.wat.parquet`.
- Baseline WAT:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet`.

## Storage Surfaces

| Surface | mm | Interpretation |
|---|---:|---|
| candidate post_seed `wb11_soil_water` | 323.346740 | trace state before day-1 scheduler |
| candidate post_seed theta sum | 293.945130 | trace layer theta state before day-1 scheduler |
| candidate post_seed `wb11 - theta` | 29.401610 | aggregate-layer alias gap at t=0 |
| candidate post_WB13 `wb11_soil_water` | 320.844074 | trace state after day-1 publication |
| candidate post_WB13 `Total-Soil` | 320.844074 | published day-1 storage |
| baseline day-1 `Total-Soil` | 343.070000 | baseline WAT end-of-day row |
| baseline inferred initial from `P` | 343.500000 | `S1 + ET + Dp + latqcc + Q - P` |
| candidate post_seed minus baseline inferred initial | -20.153260 | candidate starts drier before scheduler phases |
| candidate post_WB13 minus baseline day-1 `Total-Soil` | -22.225926 | candidate ends drier after day-1 phases |

## Day-1 Phase State

| Stage | `wb11` mm | theta sum mm | Δ `wb11` mm | `D` mm | `Pe` mm | `Ep` mm | `UPi` mm | `Ui` mm | `Ws` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| post_seed | 323.346740 | 293.945130 |  |  |  |  |  |  | 1.000000 |
| post_phase:percolation_deep_seepage | 323.095464 | 293.693853 | -0.251276 | 0.251276 | 0.251276 |  |  |  | 1.000000 |
| post_phase:evapotranspiration | 323.095464 | 293.693853 | 0.000000 | 0.251276 | 0.251276 | 0.000000 | 0.385294 | 0.000000 | 1.000000 |
| post_phase:lateral_transfer | 321.229368 | 291.827757 | -1.866096 | 0.251276 | 0.251276 | 0.000000 | 0.385294 | 0.000000 | 1.000000 |
| post_phase:runoff_reconciliation | 320.844074 | 291.442463 | -0.385294 | 0.251276 | 0.251276 | 0.385294 | 0.385294 | 0.385294 | 1.000000 |
| post_scheduler | 320.844074 | 291.442463 | 0.000000 | 0.251276 | 0.251276 | 0.385294 | 0.385294 | 0.385294 | 1.000000 |
| post_wb13 | 320.844074 | 291.442463 | 0.000000 | 0.251276 | 0.251276 | 0.385294 | 0.385294 | 0.385294 | 1.000000 |

## Baseline vs Candidate Day-1 Terms

| Term | Baseline mm | Candidate mm | Candidate - Baseline mm |
|---|---:|---:|---:|
| `P` | 0.000000 | 0.000000 | 0.000000 |
| `RM` | 0.000000 | 0.000000 | 0.000000 |
| `ET` | 0.150000 | 0.385294 | 0.235294 |
| `Ep` | 0.150000 | 0.385294 | 0.235294 |
| `Dp` | 0.240000 | 0.251276 | 0.011276 |
| `latqcc` | 0.040000 | 1.866096 | 1.826096 |
| `Q` | 0.000000 | 0.000000 | 0.000000 |
| `Snow-Water` | 0.000000 | 0.000000 | 0.000000 |
| `Total-Soil` | 343.070000 | 320.844074 | -22.225926 |

## Interpretation

Static:

- The HPHYS0252 continuation premise is confirmed: H1 needs a t=0/day-1
  localization gate before another process-surface correction.

Ran:

- The dominant H1 day-1 gap is already present before scheduler phases:
  `20.153260 mm` of the `22.225926 mm` end-of-day `Total-Soil` gap is a
  post-seed deficit against the baseline WAT-derived t=0 proxy.
- Day-1 excess loss accounts for the remaining `2.072666 mm`, mainly
  `latqcc` (`+1.826096 mm`) plus `Ep` (`+0.235294 mm`); `Dp` is near baseline
  (`+0.011276 mm`).
- The next implementation focus should be WB11 initial/runtime storage
  projection and layer/aggregate alias authority, not another WB18/WB19
  drainage correction.
