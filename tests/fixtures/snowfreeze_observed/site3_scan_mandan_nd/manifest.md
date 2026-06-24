# Site 3 — SCAN Mandan #1, ND

Target hillslope: `p3`. **Frost enabled** (`ksflag = 1`) — set as a deliberate
override of the wepp.cloud non-ag default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/fo/forced-bop` (wepp.cloud)
- Hillslope: TopazID 23 → `wepp_id` 3 (`p3`)
- Modeled centroid: 46.76652, −100.91610, elev 589.6 m
- Geometry: length 161.3 m, width 284.6 m, slope 0.0307, area 45,900 m²

## Soil / management
- SSURGO mukey 2699220 — silt loam; `luse = tall grass`; management Tah_7670;
  `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  MANDAN EXP STATION ND (46.80, −100.90, 533 m)
- Period 1991–2024 (34 yr; `p3.run` years = 34); ~3-yr burn-in (validation from
  1994 aligns with SCAN STO start)

## Observation authority
- Dataset: NRCS SCAN Mandan #1, station triplet `2020:ND:SCAN`
- Obs coords: 46.7667, −100.9167, 588 m
- Method: **soil-temperature 0 °C isotherm** (derived, NOT frost tube) → timing
  authority only; depths 5/10/20/50/100 cm, capped at ~1.0 m; co-located met
- Period: STO from ~1994; daily/hourly
- Access: NRCS AWDB REST API, `stationTriplets=2020:ND:SCAN`, element `STO`
- Tier: 2 (onset/thaw timing; carries 0 °C-isotherm-vs-ice-front offset)
