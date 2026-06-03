# WB19 Hourly Latqcc Lineage Diagnosis

Status: completed

Evidence mode: mixed

## Baseline Findings

- Static: `/workdir/wepp-forest_260430_baseline/src/input.for:753-761` and
  `:836-844` assemble `ui_ksari` from thickness-weighted
  `ssc2(j)*ui_anisrt(j)` for `solwpv >= 2006`.
- Static: `/workdir/wepp-forest_260430_baseline/src/input.for:927-928`
  publishes `ui_ssh1(i,iplane) = ui_ksari(i)/slayth`.
- Static: `/workdir/wepp-forest_260430_baseline/src/tilage.for:571-656`
  projects `ui_ssh1` into runtime `ui_ssh`, separately from vertical `ssc`.
- Static: `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:705-715`
  uses `ui_ssh(mn,iplane)` in the hourly `totK` loop:
  `totK = totK + ui_ssh*fffx*dg`.
- Static: `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:741-745`
  preserves the final profile `anisrt(iplane)` multiplier and caps `latqcc`
  to `tdvv`.

## openWEPP Defect

- Static: pre-HPHYS0257 openWEPP published `wb18_perc_ssc_####` vertical
  conductivity and used it in hourly WB19 lateral conductivity.
- Static: no distinct `wb19_lateral_ssh_####` state surface existed, so modern
  hourly lanes could not prove `ui_ssh` lineage or fail closed when omitted.
- Static: runtime projection also reused layer `ui_anisrt` as the profile
  `wb19_lateral_anisotropy_ratio`; after adding `ui_ssh`, that would
  double-apply modern layer anisotropy. Modern UI soil profile anisotropy is
  therefore projected as unity unless a separate profile `anisrt` authority is
  introduced.

## Metric Interpretation

- Ran: the corrected HPHYS0257 full H1..H39 run completed at
  `/tmp/hphys0257_20260603T020345Z`.
- Ran: full-suite `latqcc` mean absolute diff improved by `-0.129755`, and
  max absolute diff improved by `-13.245815` versus HPHYS0256.
- Static: remaining residuals are no longer explained by missing hourly
  horizontal-conductivity lineage. The next controlling surface is likely the
  hourly cap/availability and withdrawal/publication lineage.
