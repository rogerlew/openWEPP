# Iterative Diagnosis Log

Status: completed/HOLD
Evidence mode: static + ran

Static: HPHYS0268 left H1/H7/H39 trace closure intact but identified a
semantic snowpack magnitude/timing gap: baseline keeps large spring snowpack
where openWEPP has nearly depleted snow.

Static: source seams inspected and dispositioned.

| Seam | Baseline authority | openWEPP target | Disposition |
|---|---|---|---|
| Hourly winter loop | `/workdir/wepp-forest_260430_baseline/src/winter.for` lines 260-367 | active snow coupling | Existing dispatch path; no forcing-path production edit in this slice. |
| Daily melt aggregation | `winter.for` lines 373-464 | `redistribute_daily_signed_snowmelt` | Implemented as mass-closed daily net-melt redistribution with signed raw melt retained for trace. |
| Rain-on-snow holding capacity | `snowd.for` lines 240-279 | `SnowCouplingOutcome::rain_retained` and hourly trace | Implemented retained-rain accounting below the `350 kg m^-3` snow-density gate. |
| Signed melt equation | `melt.for` lines 126-301 | `compute_simimpl29_melt_hour` | Raw signed melt no longer clamps negative values at the equation boundary; positive melt remains bounded to available snow. |
| WB12 liquid forcing | snow/rain partition lineage | hydrology phase liquid input | Retained rain is subtracted from direct liquid forcing. |
| WB13 snow closure trace | `S`, `RM`, `Snow-Water` publication seam | HPHYS trace schema v8 | Added retained-rain/raw-melt sums and closure identity `S = melt - snow_we - rain_retained`. |

Static: negative-melt authority is resolved for target behavior. The pinned
baseline contains a bug-compatible daily negative-melt branch, while
`/workdir/wepp-forest` HEAD includes commit `03fee4558456535138592630b5dedc4d81ce8d06`
(`winter: apply ngtvML/pstvML math fix and close P4 parity lane`). openWEPP
keeps the corrected `wepp-forest` fix as authority and does not reproduce the
pinned baseline sign/branch bug. An inverted-authority run may be useful only
as a counterfactual diagnostic to prove whether the old bug explains comparator
residuals.

Ran:

- Targeted H1/H7/H39 diagnostics passed runtime execution and trace generation
  at `/tmp/hphys0269_full_final_20260603T185740Z`.
- Full H1..H39 diagnostics passed runtime execution for all hillslopes at
  `/tmp/hphys0269_full_final_20260603T185740Z`.
- The resulting trace classifications are internally closed but semantically
  divergent from baseline snow-water/`RM` magnitude.
