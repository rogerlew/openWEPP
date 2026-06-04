# Localization Evidence

Status: complete
Evidence mode: Static + Ran

## Ran: Pre-Fix Signal

- Post-0281 run root: `/tmp/hphys0281_rebaseline_20260604T143411Z`.
- Runtime completed `39/39`; semantic pass `0/39`.
- Pre-fix selected metrics:
  - `Total-Soil`: fail count `55908`, mean abs diff `149.442866`, max `611.813445`.
  - `Q`: fail count `5547`, mean abs diff `1.245240`, max `43.926329`.
  - `Ep`: fail count `56132`, mean abs diff `1.669264`, max `7.778863`.

## Static: Root-Cause Seam

- `run_runoff_reconciliation` computed routed snowmelt as `signed_s + accumulation + rain_retained`, then added that term to runoff closure.
- The pre-fix Green-Ampt helper received liquid rainfall and irrigation but not routed snowmelt, so meltwater could become `Q` without first being offered to infiltration.
- WB18 percolation also did not mutate layer storage from same-pass infiltrated melt before aggregate `watcon` recomputation, so reduced runoff alone would not repair `Total-Soil`.

## Static: Baseline Authority

- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:342` through `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:345`: `fin` includes `wmelt`.
- `/workdir/wepp-forest_260430_baseline/src/grna.for:267` through `/workdir/wepp-forest_260430_baseline/src/grna.for:269`: snowmelt contributes to infiltration forcing as `smrate`.

## Ran: Final Targeted Trace

- Trace root: `/tmp/hphys0283_springtrace3_20260604T164525Z`.
- H1 trace: `/tmp/hphys0283_springtrace3_20260604T164525Z/hillslope_output/H1.hphys0283.trace.jsonl`.
- H7 trace: `/tmp/hphys0283_springtrace3_20260604T164525Z/hillslope_output/H7.hphys0283.trace.jsonl`.
- H39 trace: `/tmp/hphys0283_springtrace3_20260604T164525Z/hillslope_output/H39.hphys0283.trace.jsonl`.

| Case | Stage | wb11 mm | theta sum mm | Snow-Water m | S m | Q m | Ep m |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| H1 day 510 | post-seed | 312.376 | 280.974 | 0.097103 | 0.034860 | 0.000250 | 0.001832 |
| H1 day 510 | post-WB18 | 348.153 | 316.752 | 0.097103 | 0.034860 | 0.000250 | 0.001832 |
| H1 day 510 | post-WB12 | 343.986 | 312.585 | 0.061263 | 0.035839 | 0.000254 | 0.003894 |
| H7 day 511 | post-seed | 262.109 | 230.319 | 0.085314 | 0.035627 | 0.000269 | 0.003816 |
| H7 day 511 | post-WB18 | 300.490 | 268.699 | 0.085314 | 0.035627 | 0.000269 | 0.003816 |
| H7 day 511 | post-WB12 | 296.668 | 264.878 | 0.048472 | 0.036842 | 0.000296 | 0.003509 |
| H39 day 510 | post-seed | 272.078 | 229.667 | 0.095207 | 0.034889 | 0.000509 | 0.001915 |
| H39 day 510 | post-WB18 | 307.919 | 265.508 | 0.095207 | 0.034889 | 0.000509 | 0.001915 |
| H39 day 510 | post-WB12 | 303.333 | 260.922 | 0.059304 | 0.035903 | 0.000596 | 0.003970 |

## Disposition

- Confirmed original bypass: melt was in runoff closure but absent from infiltration forcing/layer ingress.
- Corrected path: active routed melt now increases WB18 layer storage before percolation and aggregate output.
- Remaining residual owner: snowpack timing/retention and earlier-season storage divergence, not the HPHYS0283 partition seam.
