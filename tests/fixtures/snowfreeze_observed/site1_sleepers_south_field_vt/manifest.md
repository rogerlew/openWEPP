# Site 1 — Sleepers River "South field", VT

Target hillslope: `p1`. **Frost-active** (`ksflag = 1`) — the only as-built
frost-active fixture in this set.

## Provenance
- Source run: `/wc1/runs/ha/hard-bitten-doze` (wepp.cloud)
- Hillslope: TopazID 21 → `wepp_id` 1 (`p1`)
- Modeled centroid: 44.46479, −72.14916, elev 482.7 m
- Geometry: length 434.6 m, width 329.3 m, slope 0.0432, area 143,100 m²

## Soil / management
- SSURGO mukey 282872 — "Cabot silt loam, 3 to 8 percent slopes" (silt loam)
- `luse = agriculture crops`; management bromegrass; `ksflag = 1`, `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  SAINT JOHNSBURY VT (44.42, −72.02, 213 m)
- Period 1980–2017 (38 yr; `p1.run` years = 38); ~3-yr burn-in

## Observation authority
- Dataset: USGS Sleepers River Research Watershed — "South field" (corn/hay plot)
- Obs coords: 44.4783, −72.1467, 474 m
- Method: frost tube (methylene blue) → magnitude authority; cadence 2–4×/mo Nov–May
- Period: 1983–2017; paired snow **depth** (no SWE in this release)
- Access: `DOI 10.5066/P96753GI` — `Frost site description.csv` + `Sleepers frost1983-2020.csv`
- Tier: 1 (magnitude), agricultural, frost-active
