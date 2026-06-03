# Targeted H1/H7/H39 PMET Branch Classification

Status: completed

Evidence mode: ran

Ran: HPHYS0254 diagnostic harness plus HPHYS0262 PMET branch classification.

| Hillslope | Trace schema | Classification | Baseline Ep mm | Candidate Ep mm | Ep diff mm | iflget | kcb | rawp | PMET line | fallback | seed branch | demand mm | Trace Ep mm | ΣUi mm | LAI |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | openwepp-hphys0245-wb11-wb18-wb19-wb17-pmet-branch-trace-v6 | PMET_SIDECAR_SELECTS_EVAPPM_BUT_PT_DEMAND_SEEDED | 0.150000 | 0.385294 | 0.235294 | 2.000000 | 0.950000 | 0.800000 | 1.000000 | 0.000000 | evap_priestley_taylor | 0.385294 | 0.385294 | 0.385294 | 11.874844 |
| H7 | openwepp-hphys0245-wb11-wb18-wb19-wb17-pmet-branch-trace-v6 | PMET_SIDECAR_SELECTS_EVAPPM_BUT_PT_DEMAND_SEEDED | 0.150000 | 0.385294 | 0.235294 | 2.000000 | 0.950000 | 0.800000 | 1.000000 | 0.000000 | evap_priestley_taylor | 0.385294 | 0.385294 | 0.385294 | 11.874844 |
| H39 | openwepp-hphys0245-wb11-wb18-wb19-wb17-pmet-branch-trace-v6 | PMET_SIDECAR_SELECTS_EVAPPM_BUT_PT_DEMAND_SEEDED | 0.150000 | 0.385294 | 0.235294 | 2.000000 | 0.950000 | 0.800000 | 1.000000 | 0.000000 | evap_priestley_taylor | 0.385294 | 0.385294 | 0.385294 | 11.874844 |

Static legacy authority:

- `watbal_hourly.for:557-559` calls `evap` when `iflget.eq.1`; otherwise
  `evappm`.
- `evappm.for:181-297` computes Penman-Monteith reference ET, crop
  coefficients, `Es`, and `Ep` from PMET inputs.
- `SC-INFILE-PMETPARA-001` defines sidecar-present `iflget=2`, crop-key lookup,
  `kcb`, `rawp`, and fallback observability.

Interpretation:

- `PMET_SIDECAR_SELECTS_EVAPPM_BUT_PT_DEMAND_SEEDED` means the run discovers
  and projects PMET sidecar/crop coefficients, but current WB11 ET demand is
  still seeded by the Priestley-Taylor `evap` branch; closure requires
  baseline-authoritative `evappm.for` migration, not coefficient tuning or
  proxy demand.
