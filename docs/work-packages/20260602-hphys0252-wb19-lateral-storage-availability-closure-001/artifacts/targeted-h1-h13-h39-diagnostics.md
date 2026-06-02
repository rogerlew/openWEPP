# Targeted H1/H13/H39 Diagnostics

Status: complete

Evidence mode: ran

Ran:

- Current root:
  `/tmp/hphys0252_20260602T195147Z`.
- Targeted report:
  `/tmp/hphys0252_20260602T195147Z/reports/targeted_h1_h13_h39_diagnostics.md`.

Summary:

| Hillslope | Symbol | Baseline Sum/Final | HPHYS0251 Sum/Final | HPHYS0252 Sum/Final | Δ Current-Prev |
|---|---|---:|---:|---:|---:|
| H1 | `Ep` | 3036.91 | 98.8265 | 98.8265 | 0 |
| H1 | `Total-Soil` | 260.23 | 29.4016 | 29.4016 | 0 |
| H1 | `latqcc` | 535.48 | 159.887 | 159.887 | 0 |
| H13 | `Ep` | 2876.25 | 92.7143 | 92.7143 | 0 |
| H13 | `Total-Soil` | 247.66 | 29.4016 | 29.4016 | 0 |
| H13 | `latqcc` | 723.04 | 170.742 | 170.742 | 0 |
| H39 | `Ep` | 2028.08 | 119.722 | 119.722 | 0 |
| H39 | `Total-Soil` | 177.86 | 40.4109 | 40.4109 | 0 |
| H39 | `latqcc` | 1757.62 | 194.569 | 194.569 | 0 |

Interpretation:

- The HPHYS0252 code path is correct but does not move the targeted H1/H13/H39
  selected outputs relative to HPHYS0251.
- Remaining targeted storage deficit is upstream or orthogonal to WB19
  frozen-adjusted lateral thresholding in these runs.
