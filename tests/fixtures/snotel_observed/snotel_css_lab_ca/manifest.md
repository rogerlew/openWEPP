# SNOTEL — CSS Lab, CA (Sierra Nevada maritime)

Target hillslope: `p2`. **Frost enabled** (`ksflag = 1`) — deliberate override of
the wepp.cloud forest default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/an/anaphylactic-vernacular` (wepp.cloud)
- Hillslope: TopazID 22 → `wepp_id` 2 (`p2`)
- Modeled centroid: 39.32574, −120.36693, elev 2165.3 m
- Geometry: length 217.4 m, width 327.0 m, slope 0.1668, area 71,100 m²

## Soil / management
- SSURGO mukey 464810 — loam; `luse = forest`; `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  DONNER MEMORIAL PK CA (39.32, −120.23, 1810 m). Lapse: station 1810 m →
  hillslope 2165 m (~355 m), PRISM-supplied.
- Period 1980–2024 (45 yr; `p2.run` years = 45); ~3-yr burn-in

## SNOTEL observation authority
- Station: **CSS Lab (Central Sierra Snow Lab) `428:CA:SNTL`**; coords 39.3257, −120.3681, 2100 m
- Snow climate: **Sierra Nevada maritime** (Donner Pass) — deep, warm snow;
  second maritime regime, distinct ecoregion/latitude from the Cascades
- Elements / start: SWE `WTEQ` 1980, **snow depth `SNWD` 2005-08-22**, soil temp `STO` 2005
- Density pairing window: 2005 → 2024
- Access: NRCS AWDB REST API, `stationTriplets=428:CA:SNTL`, elements `WTEQ,SNWD,PREC,TOBS,TMAX,TMIN,STO`
