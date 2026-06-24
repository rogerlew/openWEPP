# Site 5 — Reynolds Creek US-Rls (Low Sagebrush), ID

Target hillslope: `p1`. **Frost enabled** (`ksflag = 1`) — set as a deliberate
override of the wepp.cloud non-ag default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/fe/feline-wrangler` (wepp.cloud)
- Hillslope: TopazID 21 → `wepp_id` 1 (`p1`)
- Modeled centroid: 43.14370, −116.73477, elev 1653.0 m
- Geometry: length 404.6 m, width 271.4 m, slope 0.1292, area 109,800 m²

## Soil / management
- SSURGO mukey 486088 — loam; `luse = shrub`; management Tah_9591 (shrub);
  `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  SWAN FALLS PW HOUSE ID (43.25, −116.38, 707 m). ⚠️ The CLIGEN station elev
  (707 m) is far below the modeled hillslope (1653 m); PRISM-revision
  spatialization supplies the lapse correction — verify the winter
  temperature/precip lapse is reasonable for this high-relief site.
- Period 1980–2024 (45 yr; `p1.run` years = 45); ~3-yr burn-in

## Observation authority
- Dataset: USDA-ARS Reynolds Creek Experimental Watershed — Lower Sheep Creek /
  Low Sagebrush (AmeriFlux US-Rls)
- Obs coords: 43.1439, −116.7356, 1608 m
- Method: **soil temperature** (YSI thermistor profile; SHAW-validated lineage,
  Flerchinger & Hanson 1989) → timing authority only, 0 °C-isotherm proxy
- Period: 1977–present (DB 1981–1996); public CC-BY
- Access: USDA-ARS Box portal / Ag Data Commons (`soiltemperature.zip`)
- Tier: 2 / regime breadth (semi-arid rangeland, snow-insulated transect)
