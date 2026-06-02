# Targeted H1/H7/H39 Diagnostics

Status: complete

Evidence mode: ran

Ran:

- Run root: `/tmp/hphys0254_20260602T220046Z`
- Trace status: `/tmp/hphys0254_20260602T220046Z/reports/targeted_trace_status.tsv`
- Summary: `/tmp/hphys0254_20260602T220046Z/reports/targeted_h1_h7_h39_storage_summary.md`

Trace status:

| Hillslope | rc | seconds |
| --- | --- | --- |
| H1 | 0 | 23.733 |
| H7 | 0 | 16.036 |
| H39 | 0 | 15.619 |

Storage and process summary:

| Hillslope | post_seed wb11 mm | baseline inferred t0 mm | post_seed - baseline t0 mm | Total-Soil diff mm | Dp diff mm | latqcc diff mm | Ep diff mm |
| --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | 343.515748 | 343.500000 | 0.015748 | -0.819663 | 0.004798 | 0.595319 | 0.235294 |
| H7 | 288.258917 | 288.180000 | 0.078917 | -1.631130 | 0.004800 | 1.469954 | 0.235294 |
| H39 | 387.834258 | 387.750000 | 0.084258 | -8.889479 | 0.004800 | 8.733643 | 0.235294 |

Interpretation:

- The HPHYS0253 post-seed storage defect is effectively closed for H1/H7/H39.
- Remaining day-1 Total-Soil deficits are post-seed process residuals; H39 is dominated by `latqcc`.
