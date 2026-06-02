# HPHYS0252 Disposition

Status: HOLD

Evidence mode: static + ran

Static:

- HPHYS0252 closed a real WB19 lateral implementation defect: baseline hourly
  lateral capacity and top-down withdrawal floors use `fzdrfc`, not raw `drfc`.
- The correction is contract-backed by `SC-SUBHYD-001#INV-SUBHYD-025` and
  `SC-WATBAL-001#INV-WATBAL-040`.

Ran:

- Contract vector red/green passed after production correction.
- Full `H1..H39` runtime suite completed `39/39`.
- Full semantic pass remains `0/39`.
- Apples-to-apples HPHYS0251 semantic rerun vs HPHYS0252 shows no selected
  residual movement for `Ep`, `Total-Soil`, `SoilWaterTotal`, `Dp`, `Es`, `Q`,
  `RM`, `Snow-Water`, or `latqcc`.

Disposition:

- `HOLD`.

Continuation:

- Do not continue tuning WB19 `fzdrfc` for the current 39-suite residuals.
- Next focus should move upstream to WB11 seed/runtime storage scale and
  `st(i)`/`watcon` lineage, while keeping snow/runoff timing in view because
  targeted H1/H13/H39 diagnostics still show large `Q`, `RM`, and
  `Snow-Water` residuals alongside severely low aggregate storage.
