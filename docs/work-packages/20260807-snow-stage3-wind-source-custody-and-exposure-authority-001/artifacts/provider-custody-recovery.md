# WEPPpy Provider Custody Recovery

Status: `complete / partial custody recovered / exposure authority missing`.

Evidence mode: `Static + Ran`.

## Exact retained lineage

The provider runs remain available under `/wc1/runs`. Their published WEPP
climates are byte-identical to the frozen openWEPP fixtures:

| Site | Provider run and CLI | SHA-256 | Retained watershed centroid |
| --- | --- | --- | --- |
| Mica Creek | `li/listed-scar/wepp/runs/p1.cli` | `e8470ae78711f85cc84045052467fa5d75fc8ec4ca1f92ce49b1af9ecf95fb63` | `(-116.26483416760449, 47.14987731602243)` |
| Niwot | `de/deathless-wangle/wepp/runs/p2.cli` | `841d6390b511c3b6ad613e166788fd0b3c48b1d83317779ecd7ba2cfd7916ead` | `(-105.5424440268039, 40.036382248555775)` |
| Paradise | `op/open-source-thirtieth/wepp/runs/p2.cli` | `6e0c874e38825a7f4def18b87d81e61be9c59496a25e5f5affa9d25755db173c` | `(-121.74839302639597, 46.7843679183757)` |
| Snowbird canonical | `ba/barred-pro/wepp/runs/p8.cli` | `10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7` | `(-111.65847092309646, 40.56532186724135)` |

Ran: all four provider/fixture SHA-256 pairs match. The retained Snowbird
development CLI changes precipitation only, so its wind lineage remains the
canonical provider CLI.

Each run's `climate.nodb` records observed Daymet, multiple spatial climates,
GRIDMET wind enabled, and the stated year range. `watershed.nodb` records the
coordinate above. These are watershed centroids, not the selected hillslope
centroids. The retained `daymet_*.parquet` files have complete naive daily
indexes (`14,245` rows for 1986--2024; `16,437` for 1980--2024), no missing
wind, and daily `vs(m/s)` equal to serialized CLI `w-vl` after the declared
one-decimal formatting.

## Provider implementation

The nearest wepppy repository commit before the 2026-06-25 builds is
`a8c9f164e96f3a8fcfb792ff1ee874bbff72260b`; the run records themselves store
`commit_hash = unknown`, so this commit is a static reconstruction anchor, not
an exact deployed-image identity.

Static: at that revision, the implementation would:

1. obtains `ws_lng, ws_lat` from the watershed centroid;
2. calls the NKN `get-netcdf-data` service once for GRIDMET wind;
3. requests `agg_met_vs_1979_CurrentYear_CONUS.nc` variable
   `daily_mean_wind_speed` (`vs`) and the corresponding direction product for
   the inclusive calendar-year range;
4. casts returned `vs(m/s)` and date strings directly into a daily dataframe;
5. passes the same run-level wind series to every hillslope climate; and
6. replaces CLI `w-vl` by matching calendar date, formatting wind to one
   decimal place before writing the CLI.

That nearby source performs no local wind interpolation, height conversion,
attenuation, timezone conversion, or aggregation; its requested variable is
already a daily mean. Because the deployed revision and request receipt are
not retained, these are reconstructed implementation semantics, not proof of
the historical request or exact code path.

## Remaining custody and applicability gaps

The provider recovery does not retain the exact deployed code/container SHA,
request URL or response JSON, upstream asset version/status, pixel row/column,
server-side point-to-pixel selection or interpolation rule, timezone/day
boundary, upstream missing/fill policy, or endpoint-side precision behavior.
The service currently returns HTTP 500 for an authority-only one-day replay,
so no current response metadata was substituted for the historical receipt.

Run metadata establishes that the selected hillslopes model evergreen forest
and use nominal `cancov=0.9`. It does not classify the GRIDMET wind as open,
above-canopy, within-canopy, or sub-canopy, classify the physical target as a
specific aerodynamic exposure, or establish representativeness between them.
All sites therefore remain `AUTHORITY_MISSING`, but the hold is narrowed to
the named provider-side receipt fields and two-sided exposure linkage rather
than an unknown GRIDMET-to-CLI transformation.
