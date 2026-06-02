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
- Claude Code post-commit review confirms the implementation but identifies a
  strategic gap: the non-frozen "dominant lateral withdrawal" premise remains
  untested, and targeted `latqcc` is below baseline rather than an obvious
  over-drainage signature.

Disposition:

- `HOLD`.

Continuation:

- Do not continue tuning WB19 `fzdrfc` for the current 39-suite residuals.
- Treat HPHYS0246's "dominant WB19 withdrawal" premise as open, not closed.
- Next package should be diagnostic-only localization: compare H1 baseline vs
  openWEPP t=0/day-1 `Total-Soil`/layer `st(i)`/`watcon` and compute
  `inputs - (ET + Dp + latqcc + Q + delta-storage)` before any further
  loss-surface correction.
- Keep WB11 seed/runtime storage scale and snow/runoff timing in view, but do
  not re-chase the withdrawn producer-intermediate `ProfileFCStore` lead
  without direct t=0 state authority.
