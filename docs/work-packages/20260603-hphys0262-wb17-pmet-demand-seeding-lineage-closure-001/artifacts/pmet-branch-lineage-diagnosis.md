# PMET Branch Lineage Diagnosis

Status: completed

Evidence mode: static + ran

Static:

- Pinned legacy authority selects `evap` only when `iflget.eq.1`; otherwise it
  calls `evappm` from `watbal_hourly.for:557-559`.
- Pinned legacy `evappm.for:181-297` computes Penman-Monteith reference ET,
  crop coefficient adjustment, soil evaporation coefficients, water-stress
  factors, and final `Es`/`Ep`.
- `SC-INFILE-PMETPARA-001` defines sidecar-present `iflget=2`, crop-key lookup,
  `kcb`, `rawp`, and fallback observability.
- openWEPP now observes and projects this sidecar/crop lineage, but the actual
  WB11 ET demand equation path remains the Priestley-Taylor seed branch.

Ran:

- H1, H7, and H39 all classify as
  `PMET_SIDECAR_SELECTS_EVAPPM_BUT_PT_DEMAND_SEEDED`.
- Each targeted hillslope uses PMET line `1`, no fallback, `kcb=0.95`,
  `rawp=0.80`, and `wb11_et_demand=0.385294 mm`.

Conclusion:

- The day-1 WB17 `Ep` residual is not explained by missing sidecar discovery or
  crop-coefficient lookup. The next required closure is baseline-authoritative
  `evappm.for` migration into the openWEPP ET demand path.
