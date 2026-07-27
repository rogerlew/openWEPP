# Limitations and Caveats

Evidence class: `Static interpretation of retained and ran evidence`

- Daymet is a 1-km gridded estimate, not an in-situ plot sensor. Returned grid
  elevations differ from EML plot elevations by -79.0 to +68.9 m.
- Daymet vapor pressure is a secondary gridded variable. Derived VPD inherits
  errors in Tmax, Tmin, and VP and represents a daily aggregate, not canopy
  boundary-layer VPD.
- The protected fixture is also not plot truth. Its daily forcing differs from
  the nine-plot Daymet mean and mixes climate products; it is comparison-only.
- Weekly phenology observations are interval-censored. Midpoints used in
  descriptive diagnostics are not exact transition dates.
- Temperature, VPD, and photoperiod covary seasonally. The available
  observations cannot uniquely identify all six thresholds.
- Species, plot, elevation, calendar, and meteorology are incompletely
  separable in this observational design. Associations are not causal effects.
- The q00..q100 grid bounds observed spring forcing support, not ecological
  parameter probability or universal transferability. Boundary hits mean the
  evidence is insufficient, not that the true threshold equals the boundary.
- Daymet's 365-day calendar drops December 31 in leap years. Spring joins are
  unaffected, but annual analyses must not silently assume Gregorian row
  counts.
- Harvard remains a one-time holdout. Its role metadata in the combined
  admitted timing ledger is inspected to exclude it, but no Harvard interval
  is joined or analyzed and no raw source, modeled trace, or score is opened.
  No claim here is validated out of sample.
- The design can support honest best-effort calibration and sensitivity
  analysis. It cannot support a claim of unique physiological identification.
