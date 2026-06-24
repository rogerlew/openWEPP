# Site 2 — Sleepers River "W9 hardwood", VT

Target hillslope: `p3`. **Frost enabled** (`ksflag = 1`) — set as a deliberate
override of the wepp.cloud non-ag default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/ba/baseless-salesmanship` (wepp.cloud)
- Hillslope: TopazID 23 → `wepp_id` 3 (`p3`)
- Modeled centroid: 44.49438, −72.16070, elev 596.4 m
- Geometry: length 54.2 m, width 282.4 m, slope 0.1864, area 15,300 m²

## Soil / management
- SSURGO mukey 282863 — sand loam; `luse = forest`; management Tah_4899 (forest);
  `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  SAINT JOHNSBURY VT (44.42, −72.02, 213 m)
- Period 1980–2024 (45 yr; `p3.run` years = 45); ~3-yr burn-in

## Observation authority
- Dataset: USGS Sleepers River — W9 hardwood-forest hillslope plot
- Obs coords: 44.4934, −72.1600, 567 m
- Method: frost tube (methylene blue) → magnitude authority; cadence 2–4×/mo
- Period: 1983–present; paired snow depth (no SWE in this release)
- Access: `DOI 10.5066/P96753GI`
- Tier: 3 (forest regime / snow insulation)
