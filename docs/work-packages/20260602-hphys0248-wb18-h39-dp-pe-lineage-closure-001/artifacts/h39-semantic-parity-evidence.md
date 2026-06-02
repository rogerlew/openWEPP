# H39 Semantic Parity Evidence

Status: completed

Evidence mode: Ran

Ran:
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: pass.
- H39 run: pass (`rc=0`) under
  `/tmp/hphys0248_20260602T114714Z_final`.
- H39 semantic comparator:
  `/tmp/hphys0248_20260602T114714Z_final/reports/semantic/H39.semantic.json`.
- H39 semantic pass: `false`.
- H39 common rows: `1461`.
- H39 selected final residuals:

| Column | Fail Count | Mean Abs | Max Abs | Max Key |
|---|---:|---:|---:|---|
| `Dp` | 889 | 0.145745 | 0.240000 | `[1, 1, 2014]` |
| `latqcc` | 845 | 1.261932 | 28.703997 | `[1, 4, 2013]` |
| `Total-Soil` | 1453 | 76.333018 | 430.764834 | `[1, 145, 2014]` |
| `SoilWaterTotal` | 1453 | 76.333018 | 430.764834 | `[1, 145, 2014]` |
| `Ep` | 1460 | 1.388145 | 7.020000 | `[1, 183, 2014]` |
| `Es` | 1461 | 3.499716 | 10.008919 | `[1, 179, 2015]` |
| `Q` | 68 | 0.882300 | 77.220396 | `[1, 93, 2014]` |
| `RM` | 278 | 2.342016 | 78.410396 | `[1, 93, 2014]` |
| `Snow-Water` | 635 | 63.849723 | 532.700000 | `[1, 108, 2014]` |

Interpretation:
- H39 early-season `Dp` burst is corrected to baseline scale.
- H39 full semantic parity remains blocked by storage, ET partition, snow/runoff
  timing, and residual lateral/storage coupling families.
