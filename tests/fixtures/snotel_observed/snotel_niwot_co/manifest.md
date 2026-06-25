# SNOTEL — Niwot, CO (Front Range continental)

Target hillslope: `p2`. **Frost enabled** (`ksflag = 1`) — deliberate override of
the wepp.cloud forest default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/de/deathless-wangle` (wepp.cloud)
- Hillslope: TopazID 22 → `wepp_id` 2 (`p2`)
- Modeled centroid: 40.03686, −105.54218, elev 3086.6 m
- Geometry: length 149.0 m, width 664.3 m, slope 0.1881, area 99,000 m²

## Soil / management
- SSURGO mukey 762986 — silt loam; `luse = forest`; `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  BOULDER CO (40.02, −105.27, 1645 m). ⚠️ **Large lapse:** station 1645 m →
  hillslope 3087 m (~1442 m); PRISM-revision spatialization carries the entire
  correction. Verify the winter temperature/precip lapse before trusting
  magnitudes at this site.
- Period 1980–2024 (45 yr; `p2.run` years = 45); ~3-yr burn-in

## SNOTEL observation authority
- Station: **Niwot `663:CO:SNTL`**; coords 40.0358, −105.5452, 3030 m
- Snow climate: **Continental** (CO Front Range, near Niwot Ridge LTER) — cold,
  dry, lower-density snow; the continental end-member for the density spread
- Elements / start: SWE `WTEQ` 1979, **snow depth `SNWD` 2005-07-28**; **no soil
  temperature (`STO` absent)** — the only site of the five without soil temp
- Density pairing window: 2005 → 2024
- Access: NRCS AWDB REST API, `stationTriplets=663:CO:SNTL`, elements `WTEQ,SNWD,PREC,TOBS,TMAX,TMIN`
