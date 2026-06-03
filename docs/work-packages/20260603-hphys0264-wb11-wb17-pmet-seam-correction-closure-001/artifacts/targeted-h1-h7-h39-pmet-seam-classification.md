# H1/H7/H39 PMET Seam Classification

Status: completed

Evidence mode: Ran

Ran:

- Targeted trace root:
  `/tmp/hphys0264_20260603T083941Z`.
- Trace status:
  `/tmp/hphys0264_20260603T083941Z/reports/targeted_trace_status.tsv`.
- Storage summary:
  `/tmp/hphys0264_20260603T083941Z/reports/targeted_h1_h7_h39_storage_summary.md`.

| Hillslope | Branch | iflget | pmet ep mm | ET Etp mm | pmet es mm | WB13 Ep diff mm | Classification |
| --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | `evappm_pmet` | 2.0 | 0.151823 | 0.151823 | 0.000729 | +0.001823 | `PMET_COMPONENT_SEAM_CORRECTED_SWU_FINAL_EP` |
| H7 | `evappm_pmet` | 2.0 | 0.151823 | 0.151823 | 0.000729 | +0.001823 | `PMET_COMPONENT_SEAM_CORRECTED_SWU_FINAL_EP` |
| H39 | `evappm_pmet` | 2.0 | 0.151823 | 0.151823 | 0.000729 | +0.001823 | `PMET_COMPONENT_SEAM_CORRECTED_SWU_FINAL_EP` |

Interpretation:

- The prior seam defect would have published ET `Etp = 0.0016 m` in the
  contract vector; corrected PMET mode publishes `Etp = pmet_ep_m`.
- The remaining day-1 `Ep` residual is `+0.001823 mm` for H1/H7/H39 and is below
  the old double-partition magnitude; longer-season `Ep` residuals remain a
  follow-on issue.
