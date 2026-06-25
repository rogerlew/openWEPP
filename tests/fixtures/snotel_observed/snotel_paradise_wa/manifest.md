# SNOTEL — Paradise, WA (Cascades maritime)

Target hillslope: `p2`. **Frost enabled** (`ksflag = 1`) — deliberate override of
the wepp.cloud forest default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/op/open-source-thirtieth` (wepp.cloud)
- Hillslope: TopazID 22 → `wepp_id` 2 (`p2`)
- Modeled centroid: 46.78357, −121.74879, elev 1575.4 m
- Geometry: length 195.0 m, width 120.0 m, slope 0.1456, area 23,400 m²

## Soil / management
- SSURGO mukey 2756438 — sand loam; `luse = forest`; `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  RAINIER PARADISE RS WA (46.78, −121.73, 1691 m) — essentially co-located with
  the SNOTEL and close in elevation to the hillslope (good lapse case).
- Period 1980–2024 (45 yr; `p2.run` years = 45); ~3-yr burn-in

## SNOTEL observation authority
- Station: **Paradise `679:WA:SNTL`**; coords 46.7827, −121.7477, 1570 m
- Snow climate: Pacific **maritime** (Cascades, SW flank of Mt. Rainier) — deep,
  warm, high-density snowpack; the maritime end-member for the density spread
- Elements / start: SWE `WTEQ` 1979, **snow depth `SNWD` 2006-08-17**, soil temp `STO` 2011
- Density pairing window: 2006 → 2024
- Access: NRCS AWDB REST API, `stationTriplets=679:WA:SNTL`, elements `WTEQ,SNWD,PREC,TOBS,TMAX,TMIN,STO`
