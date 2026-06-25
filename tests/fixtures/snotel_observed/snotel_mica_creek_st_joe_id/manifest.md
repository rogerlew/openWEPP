# SNOTEL — Mica Creek, ID (St. Joe basin)

Target hillslope: `p1`. **Frost enabled** (`ksflag = 1`) — deliberate override of
the wepp.cloud forest default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/li/listed-scar` (wepp.cloud)
- Hillslope: TopazID 21 → `wepp_id` 1 (`p1`)
- Modeled centroid: 47.14867, −116.26324, elev 1455.0 m
- Geometry: length 367.3 m, width 276.9 m, slope 0.266, area 101,700 m²

## Soil / management
- SSURGO mukey 153349 — silt loam; `luse = forest`; `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  SAINT MARIES ID (47.32, −116.57, 652 m). Lapse: station 652 m → hillslope
  1455 m (~800 m), PRISM-supplied.
- Period 1986–2024 (39 yr; `p1.run` years = 39); ~3-yr burn-in

## SNOTEL observation authority
- Station: **Mica Creek `623:ID:SNTL`**; coords 47.1505, −116.2664, 1365 m
- Snow climate: Northern Rockies, maritime-influenced intermountain
- Basin: St. Joe River subbasin (HUC 17010304) — **mid-basin, not the extreme
  upper headwaters** (no SNOTEL exists in the upper St. Joe near Red Ives; this
  is the only SNOTEL in the St. Joe with a paired depth record)
- Elements / start: SWE `WTEQ` 1989, **snow depth `SNWD` 2002-08-08**, soil temp `STO` 2007
- Density pairing window: 2002 → 2024 (SWE+depth overlap with DAYMET)
- Access: NRCS AWDB REST API, `stationTriplets=623:ID:SNTL`, elements `WTEQ,SNWD,PREC,TOBS,TMAX,TMIN,STO`
