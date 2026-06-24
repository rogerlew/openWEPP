# Dataset Inventory

Evidence class: Static/Ran.

Status: acquired for USGS Sleepers, NRCS SCAN Mandan, NSIDC GGD498 Morris, and
USDA-ARS Reynolds Creek station 127. Dun-2010 Pullman/Morris remains
request-only and excluded from normal gates.

## Source Summary

| Site | Source / DOI | Access route | License/terms | Redistributable | Status | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `site1_sleepers_south_field_vt` | USGS Sleepers River, DOI `10.5066/P96753GI` | ScienceBase item `5e6bce83e4b01d5092632650` (CSV/XML, HTTPS) | USGS public data release | yes | acquired; 392 rows | frost tube + paired snow depth |
| `site2_sleepers_w9_hardwood_vt` | USGS Sleepers River, DOI `10.5066/P96753GI` | same item (CSV/XML) | USGS public data release | yes | acquired; 200 rows | frost tube + paired snow depth (forest) |
| `site3_scan_mandan_nd` | NRCS SCAN `2020:ND:SCAN` | AWDB REST API `STO:*:*` (JSON) | USDA NRCS public station data | yes | acquired; 10,643 rows | soil-temperature profile -> `0 degC` isotherm |
| `site4_ggd498_morris_mn` | NSIDC GGD498 v1, DOI `10.7265/1mcs-q536` | `ftp://sidads.colorado.edu/.../10.txt` (ASCII) | NSIDC public data with required citation | yes | acquired; 232 rows | frost tube; limited DAYMET overlap |
| `site5_reynolds_creek_us_rls_id` | USDA-ARS Reynolds Creek soil temperature | Data.gov / Figshare `soiltemperature.zip`, station 127 | included `license.txt` says public domain, customary citation requested | yes | acquired; 4,356 rows | soil-temperature profile -> `0 degC` isotherm |
| `dun2010_pullman_morris` | Dun et al. 2010, `doi:10.13031/2013.34896` | request-only (USDA-ARS Pullman/Morris) | request-only | no | source-blocked | WEPP-lineage; excluded from required local gate |

Coordinates are decimal degrees (longitude negative = West); depths normalize to
meters.

## Per-Site Details

### site1_sleepers_south_field_vt - magnitude (frost tube)
- Observation site: USGS Sleepers River Research Watershed "South field" (corn/hay plot).
- Obs coords / elev: `44.4783, -72.1467, 474 m`.
- Method / cadence: frost tube (methylene blue dye); 2-4x/month, Nov-May. Paired snow **depth** co-located per tube.
- Period of record: 1983-2017.
- Files: `Frost site description.csv`, `Sleepers frost1983-2020.csv`, `frost metadata.xml`.
- Access: ScienceBase item `5e6bce83e4b01d5092632650`, DOI `10.5066/P96753GI`.
- Censoring: left-censored onset (not used as a direct onset timing error in this package).
- Fixture mapping: `p1`; modeled centroid `44.4648, -72.1492, 483 m`.
- Caveat: SWE lives in a separate, non-co-located release; do not pair it as the insulation control.

### site2_sleepers_w9_hardwood_vt - forest regime (frost tube)
- Observation site: Sleepers River "W9 hardwood" forest hillslope plot.
- Obs coords / elev: `44.4934, -72.1600, 567 m`.
- Method / cadence / access: same Sleepers dataset as site 1.
- Fixture mapping: `p3`; modeled centroid `44.4944, -72.1607, 596 m`.
- Role: forest / snow-insulation regime; magnitude where the snow-control gate passes.

### site3_scan_mandan_nd - timing/duration + magnitude upper-bound (soil-temp isotherm)
- Observation site: NRCS SCAN "Mandan #1", station triplet `2020:ND:SCAN`.
- Obs coords / elev: `46.7667, -100.9167, 588 m`.
- Method: soil temperature at 2/4/8/20/40 in -> explicit `0 degC` isotherm interpolation.
- Access: AWDB REST `data?stationTriplets=2020:ND:SCAN&elements=STO:*&duration=DAILY`.
- Censoring: **right-censored** at the deepest available sensor; excluded from magnitude/upper-bound error.
- Fixture mapping: `p3`; modeled centroid `46.7665, -100.9161, 590 m`.
- Caveat: soil-temperature isotherm is a magnitude upper bound on `frdp`, not a frost-depth target.

### site4_ggd498_morris_mn - magnitude / WEPP-lineage bridge (frost tube)
- Observation site: NSIDC GGD498 "Seasonal Frost Depths, Midwestern USA" station #10 "Morris".
- Obs coords: `45.58, -95.88`.
- Method: frost tube. Period 1971-1984 in the acquired flat file.
- Access: NSIDC GGD498 v1, DOI `10.7265/1mcs-q536`; station file `10.txt`.
- Censoring: left-censored onset.
- Fixture mapping: `p3`; modeled centroid `45.5794, -95.8805, 331 m`.
- Caveat: most observation dates precede DAYMET; useful overlap is sparse.

### site5_reynolds_creek_us_rls_id - timing/duration + magnitude upper-bound (soil temp)
- Observation site: USDA-ARS Reynolds Creek Experimental Watershed, Lower Sheep Creek / Low Sagebrush.
- Obs coords / elev from fixture authority: `43.1439, -116.7356, 1608 m`.
- Normalized archive station: `127`; archive header location `521742E 4776189N` UTM zone 11 and elevation `1652 m`, matching the fixture centroid/elevation class.
- Method: soil temperature profile, 5-240 cm -> explicit `0 degC` isotherm interpolation.
- Access: Data.gov / Figshare `soiltemperature.zip`; HydroShare metadata is optional and not the download authority.
- Censoring: sensor-depth cap; soil temperature remains timing/upper-bound authority only.
- Fixture mapping: `p1`; modeled centroid `43.1437, -116.7348, 1653 m`.
- Caveat: CLIGEN-station-to-hillslope lapse (707 m -> 1653 m) remains a follow-up review item.

### dun2010_pullman_morris - WEPP-lineage (request-only, SOURCE-BLOCKED)
- Sites: USDA-ARS Palouse Conservation Field Station near Pullman WA and USDA-ARS Swan Lake Research Farm near Morris MN.
- Method: frost tubes (methylene blue).
- Access: request-only; no public deposit found during this package.
- Status: `SOURCE-BLOCKED`; excluded from the required local gate until acquired.

## Provenance Records

Checked-in provenance records:

- `tests/fixtures/snowfreeze_observed/observations/provenance/usgs_sleepers_p96753gi.json`
- `tests/fixtures/snowfreeze_observed/observations/provenance/nrcs_scan_mandan_2020.json`
- `tests/fixtures/snowfreeze_observed/observations/provenance/nsidc_ggd498_morris_10.json`
- `tests/fixtures/snowfreeze_observed/observations/provenance/usda_ars_reynolds_creek_soil_temperature.json`

Raw files are fetched into `target/snowfreeze_observed/` and are not committed.
