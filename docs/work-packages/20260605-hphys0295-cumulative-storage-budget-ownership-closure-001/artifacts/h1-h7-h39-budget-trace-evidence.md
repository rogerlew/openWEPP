# H1/H7/H39 Budget Trace Evidence

Status: executed-hold
Evidence mode: Ran

Ran:
- Targeted traces for H1/H7/H39 under
  `/tmp/hphys0295_full_20260605T052422Z/trace_targeted/`.
- Cumulative window extraction output:
  `/tmp/hphys0295_full_20260605T052422Z/reports/hphys0295_cumulative_budget.md`.

| Hillslope | Window | Days | Storage error change (mm) | Known flux (mm) | Budget gap (mm) | ΣΔET (mm) | ΣΔDp (mm) | ΣΔlatqcc (mm) | ΣΔRM (mm) | Dominant residual |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| H1 | first_2013 | 112-127 | `-12.085824` | `-11.579983` | `-0.505841` | `-2.898661` | `0.076776` | `-0.270708` | `-14.672576` | `RM` |
| H1 | spring2014 | 120-146 | `-63.592277` | `-66.463807` | `2.871529` | `-2.655244` | `0.129559` | `5.426909` | `-63.562583` | `RM` |
| H1 | spring2016 | 104-111 | `-16.681497` | `-17.346783` | `0.665286` | `-0.005411` | `0.038388` | `2.037399` | `-15.276407` | `RM` |
| H7 | first_2013 | 112-127 | `-8.032805` | `-7.442501` | `-0.590304` | `-4.150698` | `0.076792` | `0.089138` | `-11.427268` | `RM` |
| H7 | spring2014 | 120-146 | `-55.398461` | `-59.656030` | `4.257568` | `-3.993089` | `0.129587` | `1.720507` | `-61.799024` | `RM` |
| H7 | spring2016 | 104-111 | `-17.081913` | `-17.630353` | `0.548440` | `-0.005411` | `0.038396` | `0.711941` | `-16.885426` | `RM` |
| H39 | first_2013 | 97-112 | `-7.210435` | `-7.530933` | `0.320498` | `-0.522392` | `0.076798` | `-2.712771` | `-10.689298` | `RM` |
| H39 | spring2014 | 120-146 | `-35.807838` | `-38.693636` | `2.885797` | `-1.420166` | `0.129596` | `-25.771016` | `-65.755222` | `RM` |
| H39 | spring2016 | 104-111 | `-14.025120` | `-14.366240` | `0.341120` | `-0.011476` | `0.038399` | `-1.600845` | `-15.940163` | `RM` |

Interpretation:
- Each diagnosed storage window is dominated by `RM`, not by `Dp`, `Ep`/`Es`,
  or same-day WB19 lateral accounting.
- H39 lateral residuals can be large, but the storage-collapse direction still
  tracks `RM` after cumulative accounting.
- A downstream compensation patch would hide the dominant producer residual
  and violate `SC-WATBAL-001#INV-WATBAL-070`.
