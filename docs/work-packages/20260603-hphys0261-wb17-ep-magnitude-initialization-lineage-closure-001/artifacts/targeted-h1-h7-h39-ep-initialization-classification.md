# Targeted H1/H7/H39 Ep Initialization Classification

Status: completed

Evidence mode: ran

## Classification

Ran: HPHYS0261 diagnostic wrapper using run root
`/tmp/hphys0261_20260603T042648Z`.

| Hillslope | Trace schema | Classification | Baseline Ep mm | Candidate Ep mm | Ep diff mm | Etp mm | Trace Ep mm | ΣUi mm | LAI | rtd m | effective pltol | Threshold layers | min theta/threshold | stress-limited layers |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | openwepp-hphys0245-wb11-wb18-wb19-wb17-ep-init-trace-v5 | ETP_FULL_DEMAND_NO_SWU_STRESS_MAGNITUDE_FOCUS | 0.150000 | 0.385294 | 0.235294 | 0.385294 | 0.385294 | 0.385294 | 11.874844 | 1.800000 | 0.100000 | 9 | 2.100182 | 0 |
| H7 | openwepp-hphys0245-wb11-wb18-wb19-wb17-ep-init-trace-v5 | ETP_FULL_DEMAND_NO_SWU_STRESS_MAGNITUDE_FOCUS | 0.150000 | 0.385294 | 0.235294 | 0.385294 | 0.385294 | 0.385294 | 11.874844 | 1.800000 | 0.100000 | 9 | 1.723200 | 0 |
| H39 | openwepp-hphys0245-wb11-wb18-wb19-wb17-ep-init-trace-v5 | ETP_FULL_DEMAND_NO_SWU_STRESS_MAGNITUDE_FOCUS | 0.150000 | 0.385294 | 0.235294 | 0.385294 | 0.385294 | 0.385294 | 11.874844 | 1.800000 | 0.100000 | 9 | 2.663155 | 0 |

## Interpretation

Ran: Candidate final `Ep` equals traced `Etp` and `ΣUi_####` for H1/H7/H39.

Ran: Every traced storage-to-threshold ratio is above one; no layer is
stress-limited under the effective `pltol=0.1` branch.

Conclusion: the stable day-1 `Ep +0.235294 mm` residual is not an SWU
stress-clipping defect. Continuation should target the baseline-authoritative
`evap` demand seed and plant-state initialization/call-order lineage.
