# Dataset Inventory

Evidence class: Static (coordinates, periods, methods, and access endpoints
verified against canonical source catalogs/APIs by research subagents; not yet
downloaded — acquisition is Phase 1).

Status: catalogued. Per-source acquisition/checksum/normalized-row-count stay
`queued` until Phase 1 runs.

## Source summary

| Site | Source / DOI | Access route | License/terms | Redistributable | Status | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `site1_sleepers_south_field_vt` | USGS Sleepers River, DOI `10.5066/P96753GI` | ScienceBase item `5e6bce83e4b01d5092632650` (CSV, HTTPS) | CC0 / US public domain | yes | catalogued | frost tube + paired snow depth |
| `site2_sleepers_w9_hardwood_vt` | USGS Sleepers River, DOI `10.5066/P96753GI` | same item (CSV) | CC0 / US public domain | yes | catalogued | frost tube + paired snow depth (forest) |
| `site3_scan_mandan_nd` | NRCS SCAN `2020:ND:SCAN` | AWDB REST API (JSON/CSV) | US public domain | yes | catalogued | soil-temperature profile → derive `0 degC` isotherm |
| `site4_ggd498_morris_mn` | NSIDC GGD498 v1, DOI `10.7265/1mcs-q536` | `ftp://sidads.colorado.edu/...` (ASCII) | NSIDC, public w/ attribution | yes | catalogued | frost tube; limited DAYMET overlap |
| `site5_reynolds_creek_us_rls_id` | USDA-ARS Reynolds Creek soil temperature | USDA-ARS Box / Ag Data Commons (`soiltemperature.zip`) | CC-BY | yes (attribution) | catalogued | soil-temperature profile → derive `0 degC` isotherm |
| `dun2010_pullman_morris` | Dun et al. 2010, `doi:10.13031/2013.34896` | request-only (USDA-ARS Pullman/Morris) | request-only | no | source-blocked | WEPP-lineage; excluded from required local gate |

Coordinates are decimal degrees (longitude negative = West); depths normalize to
meters.

## Per-site details

### site1_sleepers_south_field_vt — magnitude (frost tube)
- Observation site: USGS Sleepers River Research Watershed "South field" (corn/hay plot).
- Obs coords / elev: `44.4783, -72.1467, 474 m`.
- Method / cadence: frost tube (methylene blue dye); 2-4x/month, Nov-May. Paired snow **depth** co-located per tube (no SWE in this release).
- Period of record: 1983-2017.
- Files: `Frost site description.csv` (per-plot coords), `Sleepers frost1983-2020.csv` (per-tube columns `[Tube#]S` = snow depth cm, `[Tube#]Ftop`/`Fbottom` = frost depths cm; `Ftop` begins 1993).
- Access: ScienceBase `https://www.usgs.gov/data/soil-frost-sleepers-river-research-watershed-danville-vermont` (item `5e6bce83e4b01d5092632650`, DOI `10.5066/P96753GI`).
- Censoring: left-censored onset (excluded from onset-timing error).
- Fixture mapping: `p1`; modeled centroid `44.4648, -72.1492, 483 m`.
- Caveat: SWE lives in a separate, non-co-located release (`DOI 10.5066/P9NMQX70`) — do not pair it as the insulation control; use co-located snow depth.

### site2_sleepers_w9_hardwood_vt — forest regime (frost tube)
- Observation site: Sleepers River "W9 hardwood" forest hillslope plot.
- Obs coords / elev: `44.4934, -72.1600, 567 m`.
- Method / cadence / period / files / access / censoring: same dataset as site 1 (`DOI 10.5066/P96753GI`); 1983-present; frost tube + paired snow depth.
- Fixture mapping: `p3`; modeled centroid `44.4944, -72.1607, 596 m`.
- Role: forest / snow-insulation regime; magnitude where the snow-control gate passes.

### site3_scan_mandan_nd — timing/duration + magnitude upper-bound (soil-temp isotherm)
- Observation site: NRCS SCAN "Mandan #1", station triplet `2020:ND:SCAN`.
- Obs coords / elev: `46.7667, -100.9167, 588 m`.
- Method: soil temperature at 2/4/8/20/40 in (5/10/20/50/100 cm) → derive `0 degC` isotherm by explicit interpolation. Co-located air temp + precip on the same mast. STO series begins ~1994; daily/hourly.
- Access (metadata): `https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/stations?stationTriplets=2020:ND:SCAN&returnStationElements=true`.
- Access (time series): `https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/data?stationTriplets=2020:ND:SCAN&elements=STO:*&duration=DAILY&beginDate=<YYYY-MM-DD>&endDate=<YYYY-MM-DD>` (STO depths returned as negative inches). Browser export: `https://wcc.sc.egov.usda.gov/nwcc/site?sitenum=2020`.
- Censoring: **right-censored** at ~1.0 m (deepest sensor) — exclude from magnitude error when frost exceeds it. The isotherm is a magnitude **upper bound** on `frdp` and the timing authority for onset/thaw/duration.
- Fixture mapping: `p3`; modeled centroid `46.7665, -100.9161, 590 m`.
- Caveat: shallow (2 in) sensor icing/dropouts common in winter; onsite SCAN precip undercatches snow — cross-check winter precip.

### site4_ggd498_morris_mn — magnitude / WEPP-lineage bridge (frost tube)
- Observation site: NSIDC GGD498 "Seasonal Frost Depths, Midwestern USA" station #10 "Morris".
- Obs coords: `45.58, -95.88`.
- Method: frost tube. Period 1971-1981 (no 1976/77).
- Access: `ftp://sidads.colorado.edu/pub/DATASETS/fgdc/ggd498_seasfrost_usa/` (ASCII); DOI `10.7265/1mcs-q536`; landing `https://nsidc.org/data/ggd498/versions/1`; user guide `https://nsidc.org/sites/default/files/ggd498-userguide-v1.pdf`.
- Censoring: left-censored onset (observers began once frost reached 1-2 in).
- Fixture mapping: `p3`; modeled centroid `45.5794, -95.8805, 331 m`.
- Caveat: the **observation period (1971-81) largely precedes DAYMET (1980+)** — only ~1980-81 overlaps the simulated climate, so usable comparison rows are sparse. Value is the public agricultural bridge ~11 km from the request-only Dun-2010 Morris farm (Swan Lake, `45.683, -95.800`).

### site5_reynolds_creek_us_rls_id — timing/duration + magnitude upper-bound (soil temp)
- Observation site: USDA-ARS Reynolds Creek Experimental Watershed, Lower Sheep Creek / Low Sagebrush (AmeriFlux US-Rls).
- Obs coords / elev: `43.1439, -116.7356, 1608 m`.
- Method: soil temperature (YSI thermistor profile, 5-240 cm; SHAW-validated lineage, Flerchinger & Hanson 1989) → derive `0 degC` isotherm. Period 1977-present (clean DB 1981-1996).
- Access: USDA-ARS Box `https://ars-usda.app.box.com/s/4jwgmxyxb8vacosvp1t5sdlibdc5qxoe` (linked from `https://www.ars.usda.gov/pacific-west-area/boise-id/northwest-watershed-research-center/docs/reynolds-creek-experimental-watershed-data/`); Ag Data Commons `soiltemperature.zip` (CC-BY). The HydroShare copy `b22305cd06eb4e37bdac1ec090daf7ef` is **discoverable but access-gated — do not depend on it.**
- Censoring: isotherm magnitude upper-bound only; sensor-depth cap.
- Fixture mapping: `p1`; modeled centroid `43.1437, -116.7348, 1653 m`.
- Caveat: soil **temperature**, not frost depth → timing/upper-bound authority only. CLIGEN-station-to-hillslope lapse (707 m → 1653 m) noted in the fixture manifest.

### dun2010_pullman_morris — WEPP-lineage (request-only, SOURCE-BLOCKED)
- Sites: USDA-ARS Palouse Conservation Field Station near Pullman WA (`~46.75, -117.20`, approx plot centroid; continuous-tilled bare fallow, Palouse silt loam); USDA-ARS Swan Lake Research Farm near Morris MN (`45.683, -95.800, 369 m`; Barnes loam).
- Method: frost tubes (methylene blue). Pullman record fall 1978 - spring 1991 (13 winters) + Singh 2003/04-2006/07 subset.
- Access: **request-only** — no public deposit found; measured series are inside paywalled Singh et al. 2009 (VZJ) / Sharratt et al. 1998 (S&TR). Path to raw data: direct request to USDA-ARS Pullman (NSAR) and Morris (NCSCRL).
- Status: `SOURCE-BLOCKED`; excluded from the required local gate until acquired. Site 4 (GGD498 #10) is the public proxy for the Morris lineage.

## Required per-source provenance fields (record in `acquisition-log.md` at Phase 1)

source URL or DOI; access date; raw file name; raw checksum; license/terms;
parser version or git commit; normalized output checksum; unit-conversion notes;
censoring notes; site-mapping evidence.
