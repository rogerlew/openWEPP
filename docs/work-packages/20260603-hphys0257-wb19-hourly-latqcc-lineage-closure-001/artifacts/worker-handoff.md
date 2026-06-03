# Worker Handoff

Status: completed

Evidence mode: mixed

## Completed in HPHYS0257

- Static: canonical contracts now require hourly `ui_ssh` /
  `wb19_lateral_ssh_####` conductivity lineage.
- Static: runtime projection publishes `wb19_lateral_ssh_####`.
- Static: modern UI soil runtime projection publishes profile anisotropy as
  unity so layer `ui_anisrt` is not applied twice.
- Static: hourly WB19 modern lanes fail closed when required
  `wb19_lateral_ssh_####` is missing.
- Ran: targeted tests, full workspace gates, authority guards, and the full
  H1..H39 diagnostic suite passed execution.

## Continuation Recommendation

- Static: scaffold the next package around hourly WB19 cap/withdrawal and
  publication lineage rather than conductivity.
- Static: required instrumentation/contract surfaces should include
  `q_lateral_potential`, `q_lateral_target`, `lateral_capacity_tdv`, `tdvv`,
  active layer masks, per-layer withdrawal, `latqcc` accumulation, `Qd`, WB13
  `SubRIn`/`latqcc` publication, and aggregate `Total-Soil`/`SoilWaterTotal`
  reconciliation.
- Static: use H1/H7/H39 day-1 as the first red-vector target because residuals
  are now small but non-zero: H1 `+0.023532 mm`, H7 `+0.047995 mm`, H39
  `+0.180364 mm` for `latqcc`.
- Ran: use `/tmp/hphys0257_20260603T020345Z` as the immediate comparison root
  and `/tmp/hphys0256_20260603T003117Z` as the prior unchanged baseline.
