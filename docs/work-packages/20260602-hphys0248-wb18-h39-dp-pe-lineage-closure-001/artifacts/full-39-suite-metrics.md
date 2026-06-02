# Full 39 Hillslope Suite Metrics

Status: completed

Evidence mode: Ran

Ran:
- Final suite root:
  `/tmp/hphys0248_20260602T114714Z_final`.
- `H1..H39` runtime success: `39/39`.
- `H1..H39` semantic comparator success: `39/39`.
- Semantic pass: `0/39`.
- Common rows: `1461..1461`.
- Aggregate summary:
  `/tmp/hphys0248_20260602T114714Z_final/reports/hillslope_semantic_summary.md`.

| Column | Pass Hillslopes | Fail Count Sum | Mean Abs Mean | Mean Abs Max | Max Abs Max | Worst H(max) |
|---|---:|---:|---:|---:|---:|---:|
| `Dp` | 0/39 | 40112 | 0.169471 | 0.220781 | 0.240000 | H1 |
| `latqcc` | 0/39 | 39816 | 0.803720 | 1.402700 | 36.779055 | H33 |
| `Total-Soil` | 0/39 | 56757 | 119.105962 | 194.458580 | 548.715973 | H24 |
| `SoilWaterTotal` | 0/39 | 56757 | 119.105962 | 194.458580 | 548.715973 | H24 |
| `Ep` | 0/39 | 56834 | 1.739422 | 2.126851 | 7.780000 | H1 |
| `Es` | 0/39 | 56973 | 3.340827 | 3.500756 | 10.028919 | H6 |
| `Er` | 39/39 | 0 | 0.000000 | 0.000000 | 0.000000 | H1 |
| `Q` | 0/39 | 2986 | 0.925027 | 1.117244 | 194.715728 | H6 |
| `RM` | 0/39 | 10678 | 2.301802 | 2.401303 | 204.850510 | H6 |
| `Snow-Water` | 0/39 | 24137 | 58.195696 | 65.985290 | 562.470000 | H9 |
| `frozwt` | 39/39 | 0 | 0.000000 | 0.000000 | 0.000000 | H1 |
| `ProfileFCStore` | 12/39 | 39447 | 2.052691 | 9.334426 | 9.334426 | H7 |
| `ProfileWPStore` | 38/39 | 1461 | 0.057297 | 1.669863 | 1.669863 | H7 |

Continuation focus:
1. WB17 `Ep`/`Es` partition remains all-hillslope failing and high leverage.
2. Snowmelt/runoff timing (`Snow-Water`, `RM`, `Q`) remains materially off.
3. Aggregate storage (`Total-Soil`, `SoilWaterTotal`) remains coupled to ET,
   snow, and residual lateral/storage timing.
4. WB19 `latqcc` remains non-closed after WB18 scale correction, especially H33.
