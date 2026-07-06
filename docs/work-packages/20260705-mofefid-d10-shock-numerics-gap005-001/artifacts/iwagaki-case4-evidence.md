# Iwagaki Case 4 Evidence

Status: executed
Evidence mode: Static + Ran

Reference window: `0..80 s`, Enhanced_WEPP trace from supplemental Figure 4
workbook (`sha256=2bf68787de6a715049ee635c154c640214936fd1181d08c8f7da7a34892d2fe8`).

| Run | `cells` | `sample_dt_s` | `max_dt_s` | `NS_trace` | Peak ratio | openWEPP `t_peak_s` | Reference `t_peak_s` | openWEPP rise 10-90 s | Reference rise 10-90 s | Log |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| baseline | 120 | 1.0 | 0.5 | 0.262677 | 0.836598 | 37.0 | 25.98 | 29.409 | 20.883 | `case4-ko200-c120-s1-md05.log` |
| fine | 240 | 0.25 | 0.25 | 0.193296 | 1.069855 | 39.0 | 25.98 | 31.007 | 20.883 | `case4-ko200-c240-s025-md025.log` |
| finer | 480 | 0.125 | 0.125 | 0.101244 | 0.844099 | 36.125 | 25.98 | 28.745 | 20.883 | `case4-ko200-c480-s0125-md0125.log` |

Subagent baseline/sweep artifact:
`artifacts/d10-s0-case4-resolution-sweeps.json`.

Verdict: Case 4 does not satisfy D-val acceptance. Refinement does not show a
source-authorized convergence trend; the metrics remain resolution-sensitive
and timing remains late relative to the reference trace.
