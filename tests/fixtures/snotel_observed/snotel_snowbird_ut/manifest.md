# SNOTEL — Snowbird, UT (Wasatch intermountain)

Target hillslope: `p8`. **Frost enabled** (`ksflag = 1`) — deliberate override of
the wepp.cloud forest default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/ba/barred-pro` (wepp.cloud)
- Hillslope: TopazID 53 → `wepp_id` 8 (`p8`)
- Modeled centroid: 40.57124, −111.66052, elev 2968.9 m
- Geometry: length 510.6 m, width 532.3 m, slope 0.4637 (steep), area 271,800 m²

## Soil / management
- SSURGO mukey 508208 — loam; `luse = forest`; `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  ALTA UT (40.60, −111.63, 2651 m) — near the SNOTEL, moderate lapse to the
  2969 m hillslope.
- Period 1986–2024 (39 yr; `p8.run` years = 39); ~3-yr burn-in

## SNOTEL observation authority
- Station: **Snowbird `766:UT:SNTL`**; coords 40.5691, −111.6585, 2795 m
- Snow climate: **Intermountain** (Wasatch, Little Cottonwood Canyon) — the
  middle regime between maritime and continental. (Classification note: Sturm
  intermountain; Trujillo & Molotch group the Wasatch with continental.)
- Elements / start: SWE `WTEQ` 1989, **snow depth `SNWD` 2002-07-31**, soil temp `STO` 2006
- Density pairing window: 2002 → 2024
- Access: NRCS AWDB REST API, `stationTriplets=766:UT:SNTL`, elements `WTEQ,SNWD,PREC,TOBS,TMAX,TMIN,STO`
