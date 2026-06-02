# Targeted H1/H13/H39 Diagnostics

Status: complete

Evidence mode: ran

Ran:

- Full candidate root: `/tmp/hphys0251_20260602T184933Z`.
- Targeted trace root: `/tmp/hphys0251_trace_20260602T190044Z`.
- Final trace summary:
  `/tmp/hphys0251_trace_20260602T190044Z/targeted_final_trace_summary.md`.
- Targeted delta summary:
  `/tmp/hphys0251_trace_20260602T190044Z/targeted_delta_from_hphys0250.md`.

# HPHYS0251 Targeted H1/H13/H39 Final Trace Summary

| Hillslope | Final Rows | rtd Nonzero Rows | Etp Sum m | UPi Sum m | Ui/Ep Trace Sum m | Candidate Ep Sum mm | Baseline Ep Sum mm | Candidate Ep Max mm | Baseline Ep Max mm | Ws Min | Ws Mean | Candidate Total-Soil Mean mm | Baseline Total-Soil Mean mm |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| H1 | 1461 | 1461 | 4.95949 | 4.9595 | 0.0988265 | 98.8265 | 3036.91 | 1.4251 | 7.78 | 0 | 0.0479252 | 38.9501 | 251.288 |
| H13 | 1461 | 1461 | 4.95949 | 4.9595 | 0.0927143 | 92.7143 | 2876.25 | 1.42296 | 7.78 | 0 | 0.0467247 | 37.2956 | 229.909 |
| H39 | 1461 | 1461 | 4.95949 | 4.9595 | 0.119722 | 119.722 | 2028.08 | 2.05621 | 7.02 | 0 | 0.0566457 | 46.7077 | 174.968 |

# HPHYS0251 Targeted Delta from HPHYS0250

| Hillslope | Baseline Ep Sum mm | HPHYS0250 Ep Sum mm | HPHYS0251 Ep Sum mm | Ep Sum Δ mm | Baseline Total-Soil Mean mm | HPHYS0250 Total-Soil Mean mm | HPHYS0251 Total-Soil Mean mm | Total-Soil Mean Δ mm |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| H1 | 3036.91 | 92.6715 | 98.8265 | 6.15494 | 251.288 | 41.2353 | 38.9501 | -2.2852 |
| H13 | 2876.25 | 88.0011 | 92.7143 | 4.71313 | 229.909 | 39.4989 | 37.2956 | -2.2033 |
| H39 | 2028.08 | 118.819 | 119.722 | 0.902597 | 174.968 | 49.103 | 46.7077 | -2.39526 |

Diagnosis:

- `rtd`, `Etp`, and `UPi` are present and nonzero on all final rows.
- `UPi` is effectively equal to `Etp`, so the cumulative root-depth potential
  uptake lineage is not the dominant remaining H1/H13/H39 limiter.
- `Ui`/`Ep` remains far below `Etp`: mean final-row `Ws` is only `0.0479`,
  `0.0467`, and `0.0566` for H1/H13/H39.
- Candidate aggregate soil storage is already far below baseline before this
  package’s additional uptake (`38.95` vs `251.29` mm for H1), so further
  uptake correctly worsens aggregate-storage residuals.
