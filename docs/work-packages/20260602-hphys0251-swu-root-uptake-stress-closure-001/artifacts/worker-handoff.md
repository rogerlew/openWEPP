# Worker Handoff

Status: complete

Evidence mode: static + ran

Completed in HPHYS0251:

- Contract-first amendments for `SC-EVAP-001#INV-EVAP-017` and
  `SC-WATBAL-001#INV-WATBAL-039`.
- Contract-derived tests for crop `pltol`, legacy normalization, layer
  `UPi_####`/`Ui_####`, aggregate `UPi`/`Ui`, final `Ep`, and `Ws`.
- Production migration for crop `pltol` runtime projection and SWU layer uptake
  trace publication.
- Full code gates and full `H1..H39` semantic suite.

Key evidence roots:

- Full suite: `/tmp/hphys0251_20260602T184933Z`.
- Targeted traces: `/tmp/hphys0251_trace_20260602T190044Z`.

Continuation recommendation:

- Scaffold the next package for upstream storage availability into WB17 root
  uptake, not additional SWU tuning.
- Diagnostic anchors: H1/H13/H39 final rows show `UPi≈Etp` but `Ws≈0.05`, while
  candidate `Total-Soil` means are `38.95`, `37.30`, and `46.71` mm versus
  baseline `251.29`, `229.91`, and `174.97` mm.
- Required scope: contract-first diagnosis of `wb18_perc_theta_####`,
  `wb11_soil_water`, `watcon`, `thetdr_####`, `dg_####`, WB18/WB19 mutation
  timing, and WB13 aggregate publication before post-WB19 root uptake.
- Keep disposition `HOLD` until `Ep` and aggregate storage residuals improve
  together without heuristic storage inflation.
