# Jennings et al. (2018) Observed Precipitation-Phase Corpus

Observed rain/snow phase + co-located met (air temp, dew point, RH, pressure) for
**validating the openWEPP rain/snow partition with observed data, not a tuned
`RST` threshold** (SNOWDENSITY-10.3.5). Acceptance basis = climate-general /
no-site-calibration: a physical partition (Harder & Pomeroy 2013, R-57; Susong
1999 dew-point fallback, R-54) must reproduce these observed phases across
maritime and continental regimes without a per-site threshold.

## Source / provenance

- **Jennings, K. S., Winchell, T. S., Livneh, B., Molotch, N. P. (2018)**,
  *Data from: Spatial variation of the rain-snow temperature threshold across the
  Northern Hemisphere*. Dryad. DOI **`10.5061/dryad.c9h35`** (version 2019-01-31).
- License: **CC0-1.0** (public-domain dedication — freely redistributable).
- Paper: `references/vendorable/Jennings2018_NatComm.pdf` (R-53),
  DOI `10.1038/s41467-018-03629-7`.
- Installed via operator browser-download of the Dryad zip, extracted here.

## Files (installed)

| File | Size | Committed | Content |
|---|---:|:---:|---|
| `…file1_station_locs_elev.csv` | 0.26 MB | yes | `Station_ID, Longitude, Latitude, Elevation` |
| `…file2_ppt_phase_met_observations.csv` | **1.2 GB** | **no (gitignored)** | **17,810,805 hourly obs**: `Station_ID, Date, Hour, Air_Temp, Dewpoint, RH, gridded_data_pres, Prec_Type, Snow_Phase, Rain_Phase` |
| `…file3_temp50_observed_by_station.csv` | 0.20 MB | yes | **`Station_ID, temp50`** — observed 50% rain/snow air-temp threshold per station = **the no-calibration validation target** |
| `…file4_temp50_raster.tif` | 0.78 MB | yes | gridded NH 50% threshold product |
| `…file5_temp50_linregr_raster.tif` | 0.66 MB | yes | gridded threshold (lin-regr) |
| `…file6_…station_observations_code.R` | tiny | yes | obs processing |
| `…file7_…phasemethods_code.R` | tiny | yes | **phase-method implementations**: bivariate `glm(Rain_Phase ~ Air_Temp + RH, binomial)` and trivariate (+pressure) binary-logistic |
| `…file8/9_…merra_*.R` | tiny | yes | MERRA threshold simulation / sensitivity |
| `README_for_…file1…txt` | tiny | yes | upstream README for file1 |

`file2` (the full obs) is **gitignored** (`.gitignore` here) — kept local for the
validation run, never committed. Re-extract anytime from the Dryad zip.

## Why this matters for 10.3.5

- **`file2` is hourly** (`Date, Hour, Air_Temp, Dewpoint, RH, …, Rain_Phase`) — the
  same sub-daily resolution openWEPP partitions at (`snow.hourly.stmtim.rst_c`),
  so a candidate partition can be scored against observed phase obs-for-obs.
- **`file3`** gives the observed 50% threshold per station — the headline
  no-calibration target (a uniform `RST` cannot match its spatial variation;
  Jennings: mean 1.0 °C, range −0.4 to 2.4 °C, RH-dependent).
- **`file7`** is the reference implementation of the Jennings temp+RH method, to
  benchmark against Harder-Pomeroy (R-57) and Susong dew-point (R-54).
