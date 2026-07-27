# Intent Plan

Status: `PROSPECTIVE FREEZE / COMPLETE`

Evidence class: `Static: frozen before calibration-result inspection`

## Authorized intent

Use Daymet V4 R1 only to establish meteorological forcing support at the nine
admitted Hubbard plots and to diagnose covariance and scale mismatch. Join
only the 932 Hubbard calibration intervals. Use source EML elevations for plot
geometry and Daymet-returned elevations only as grid-cell metadata.

## Frozen derivation and analysis

- Years: 1989–2024; variables: `tmax`, `tmin`, `vp`, `dayl`.
- VPD: exact native saturation-vapor-pressure equation with Daymet actual VP;
  reject negative/non-finite results and never clamp.
- Photoperiod: exact native FAO-56 geometry; retain Daymet day length
  separately.
- Forcing support: all nine plots, all years, yday 60–180.
- Support levels: empirical q00, q05, q25, q50, q75, q95, q100.
- Phenology association: fixed yday 60–120, within-plot anomalies; interval
  midpoint is diagnostic only, never an observed date.
- Elevation association: regress interval lower and upper bounds separately
  after centering within year/species; midpoint remains diagnostic.
- Protected fixture: daily climate comparison only; no parameter selection.

## Frozen prospective calibration design

Each threshold family uses every strictly ordered pair of its seven empirical
support levels: 21 temperature, 21 VPD, and 21 photoperiod profiles. Their
Cartesian product contains 9,261 complete six-threshold vectors, enumerated
lexicographically by pair ID. Levels are deterministic support points, not
probability priors or claims about ecological plausibility.

Later calibration must evaluate the complete vector and retain every finite
profile within 1.0 day RMSE of the minimum finite objective. It must report
boundary and exact saturation classes. No refinement is authorized; stopping
occurs after the complete 9,261-vector coarse grid. Boundary hits do not
authorize domain expansion.

## Contamination and closure

The combined admitted timing ledger is inspected for role filtering, but no
Harvard row is joined or analyzed and no raw Harvard source, modeled trace, or
score is opened. Current native/Bill Elliot values, downstream
responses, and model calibration scores did not choose the domains. Every
operand is classified in the evidence ledger, and later calibration must
carry the sensitivity and equifinality obligations in the proposed design.
