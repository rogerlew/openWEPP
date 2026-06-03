# HPHYS0263 EVAPPM Demand-Migration Classification

Status: completed

Evidence mode: ran

Ran: HPHYS0254 diagnostic harness plus HPHYS0263 EVAPPM migration classification.

| Hillslope | Trace schema | Classification | Baseline Ep mm | Candidate Ep mm | Ep diff mm | iflget | kcb | rawp | seed branch | demand mm | etorc mm | pmet_ep mm | Trace Ep mm | ΣUi mm | LAI | Root depth |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v7 | EVAPPM_MIGRATED_BRANCH_OBSERVED | 0.150000 | 0.151823 | 0.001823 | 2.000000 | 0.950000 | 0.800000 | evappm_pmet | 0.151823 | 0.210143 | 0.151823 | 0.151823 | 0.151823 | 11.874844 | 1.800000 |
| H7 | openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v7 | EVAPPM_MIGRATED_BRANCH_OBSERVED | 0.150000 | 0.151823 | 0.001823 | 2.000000 | 0.950000 | 0.800000 | evappm_pmet | 0.151823 | 0.210143 | 0.151823 | 0.151823 | 0.151823 | 11.874844 | 1.800000 |
| H39 | openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v7 | EVAPPM_MIGRATED_BRANCH_OBSERVED | 0.150000 | 0.151823 | 0.001823 | 2.000000 | 0.950000 | 0.800000 | evappm_pmet | 0.151823 | 0.210143 | 0.151823 | 0.151823 | 0.151823 | 11.874844 | 1.800000 |

Interpretation:

- `EVAPPM_MIGRATED_BRANCH_OBSERVED` means WB11 selected the PMET branch, published migrated `evappm.for` intermediates, and seeded demand from `pmet_ep_m`.
- Remaining `Ep` residuals after this classification should be assigned to post-demand SWU/growth timing, WB18/WB19 storage availability, or WB13 publication only with additional trace evidence.
