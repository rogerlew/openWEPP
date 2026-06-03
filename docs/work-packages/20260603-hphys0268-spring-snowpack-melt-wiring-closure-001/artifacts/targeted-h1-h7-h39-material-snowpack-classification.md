# Targeted H1/H7/H39 Material Snowpack Classification

Status: completed/HOLD
Evidence mode: Ran

Ran:

- Final run root: `/tmp/hphys0268_final_20260603T174015Z`.
- Report: `/tmp/hphys0268_final_20260603T174015Z/reports/hphys0268_snowpack_lineage_classification.md`.

| Hill | Classification | Julian | Candidate Day | Cand Ep | Base Ep | Ep Diff | Cand Snow-Water | Base Snow-Water | RM Diff |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 99 | 99 | 0.862388 | 1.890000 | -1.027612 | 2.768750 | 144.340000 | -0.003750 |
| H7 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 99 | 99 | 0.879520 | 1.890000 | -1.010480 | 2.768750 | 159.590000 | -0.003750 |
| H39 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 115 | 115 | 1.790149 | 2.930000 | -1.139851 | 0.000000 | 141.230000 | -16.050000 |

Interpretation:

- Trace closure is no longer incomplete after the inactive-snow stale-field fix.
- H39 no longer reports stale hourly melt on inactive snow days.
- Residual ownership remains snowpack timing/magnitude, not WB17 publication.
