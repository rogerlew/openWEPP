# Content Validation And Grid Elevation

Status: `VALIDATED_COMPLETE / DUAL REVIEW PASS / DUAL VERIFICATION PASS`

Ran: `tools/validate_acquired_data.py` directly opened the eight long-range
hourly NetCDFs and eight gridded geopotential records. It validated exact
manifest identity, SHA-256, expected variables and units, finite/domain values,
complete unique one-hour UTC axes, shortwave disposition bounds, and point-cell
coordinates. It also rejects dewpoint more than `0.003 K` above temperature;
the 3,797 positive differences in this cohort are retained unchanged because
their maximum is only `0.002838134765625 K`, within the manifest's explicit
empirical checksum-bound tolerance. Their exact cause is unresolved. The
machine-readable receipt is
`validated-source-inventory.json`.

Each target elevation is the exact selected hillslope `elevation` from the
originating local WEPPcloud project's `watershed/hillslopes.parquet`, keyed by
both Topaz and WEPP ID. The receipt binds that parquet and the project's
fixture manifest, centroid, and `dem/dem.tif` by SHA-256. The DEM hash is
project-identity evidence; it is not presented as an independent reconstruction
of the parquet's hillslope-aggregate elevation. The gridded source elevation is
official geopotential divided by `9.80665 m s^-2` at the exact point-series grid
coordinate. Both hourly and ancillary coordinates must equal the frozen site's
nearest product-native grid cell.

| Product | Site | Project elevation (m) | Grid elevation (m) | Site - grid (m) | Fixed-lapse offset (°C) |
|---|---|---:|---:|---:|---:|
| ERA5 | Mica Creek | 1455.032 | 1171.484 | 283.547 | -1.8431 |
| ERA5 | Paradise | 1575.415 | 1209.487 | 365.928 | -2.3785 |
| ERA5 | Snowbird | 2968.881 | 1967.412 | 1001.469 | -6.5095 |
| ERA5 | Niwot | 3086.569 | 2667.040 | 419.529 | -2.7269 |
| ERA5-Land | Mica Creek | 1455.032 | 1110.802 | 344.230 | -2.2375 |
| ERA5-Land | Paradise | 1575.415 | 1601.547 | -26.132 | +0.1699 |
| ERA5-Land | Snowbird | 2968.881 | 2499.044 | 469.837 | -3.0539 |
| ERA5-Land | Niwot | 3086.569 | 2841.060 | 245.509 | -1.5958 |

The offset uses the preregistered diagnostic operator
`T_site = T_grid - 6.5 K km^-1 * (z_site - z_grid)`. It remains
`ASSUMED_FOR_EXECUTION`; raw temperature must be retained separately and no
lapse parameter is calibrated here.

## Implication

Elevation treatment must be product/site-specific. In particular, Snowbird's
ERA5 correction is about `3.46 °C` colder than its ERA5-Land correction because
the products represent different grid elevations. Paradise ERA5-Land is the
only case where the grid is slightly higher than the project hillslope, giving
a small warming correction. Reusing fixture elevation alone, applying a common
offset to both products, or sampling the project DEM at the reanalysis grid
coordinate is inadmissible.
