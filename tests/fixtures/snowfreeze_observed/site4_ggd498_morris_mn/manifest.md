# Site 4 — GGD498 "Morris", MN (WEPP-lineage agricultural bridge)

Target hillslope: `p3`. **Frost enabled** (`ksflag = 1`) — set as a deliberate
override of the wepp.cloud non-ag default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/op/open-plan-conservatism` (wepp.cloud)
- Hillslope: TopazID 23 → `wepp_id` 3 (`p3`)
- Modeled centroid: 45.57942, −95.88054, elev 330.8 m
- Geometry: length 72.4 m, width 273.4 m, slope 0.017, area 19,800 m²

## Soil / management
- SSURGO mukey 428368 — loam; `luse = short grass`; management bromegrass;
  `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  MORRIS WC SCHOOL MN (45.58, −95.92, 344 m)
- Period 1980–2024 (45 yr; `p3.run` years = 45); ~3-yr burn-in

## Observation authority
- Dataset: NSIDC GGD498 "Seasonal Frost Depths, Midwestern USA" — station #10 "Morris"
- Obs coords: 45.58, −95.88
- Method: frost tube → magnitude authority
- Period: **1971–1981** (no 1976/77). ⚠️ The observation period largely
  **precedes DAYMET (1980+)**, so only ~1980–1981 overlaps the simulated
  climate. Left-censored at onset (observers began at 1–2″ frost) → do not
  validate onset timing against GGD498.
- Access: `ftp://sidads.colorado.edu/pub/DATASETS/fgdc/ggd498_seasfrost_usa/` (`DOI 10.7265/1mcs-q536`, ASCII)
- Note: ≈11 km from the canonical Dun-2010 Morris WEPP frost-validation farm
  (Swan Lake, 45.683, −95.800), whose measured series is request-only. This is
  the public agricultural bridge to that lineage.
- Tier: 1 (magnitude), agricultural, WEPP-lineage bridge
