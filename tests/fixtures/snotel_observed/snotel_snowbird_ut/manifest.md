# SNOTEL — Snowbird, UT (Wasatch intermountain)

Target hillslope: `p8`. **Frost enabled** (`ksflag = 1`) — deliberate override of
the wepp.cloud forest default (`ksflag = 0`); see fixture README.

## Provenance
- Source run: `/wc1/runs/ba/barred-pro` (wepp.cloud)
- Hillslope: TopazID 53 → `wepp_id` 8 (`p8`)
- Modeled centroid: 40.57124, −111.66052, elev 2968.9 m
- Geometry: length 510.6 m, width 532.3 m, slope 0.4637 (steep), area 271,800 m²

## Soil / management
- SSURGO mukey 508208 — loam; `luse = forest`; `ksflag = 1` (override; as-built 0), `ksatadj = 0`

## Climate
- Observed DAYMET (GRIDMET wind) + PRISM revision; CLIGEN station
  ALTA UT (40.60, −111.63, 2651 m) — near the SNOTEL, moderate lapse to the
  2969 m hillslope.
- Period 1986–2024 (39 yr; `p8.run` years = 39); ~3-yr burn-in

## SNOTEL observation authority
- Station: **Snowbird `766:UT:SNTL`**; coords 40.5691, −111.6585, 2795 m
- Snow climate: **Intermountain** (Wasatch, Little Cottonwood Canyon) — the
  middle regime between maritime and continental. (Classification note: Sturm
  intermountain; Trujillo & Molotch group the Wasatch with continental.)
- Elements / start: SWE `WTEQ` 1989, **snow depth `SNWD` 2002-07-31**, soil temp `STO` 2006
- Density pairing window: 2002 → 2024
- Access: NRCS AWDB REST API, `stationTriplets=766:UT:SNTL`, elements `WTEQ,SNWD,PREC,TOBS,TMAX,TMIN,STO`

## Snow-adjusted precipitation diagnostic

Snowbird also publishes `PRCP`, `PRCPSA`, and `SNRR`. `PRCPSA` is derived
snow-adjusted precipitation, and its daily values for the 1989-10-01 through
2024-09-30 diagnostic window are exactly reconstructed on all `12,749`
non-water-year-reset days as:

```text
PRCPSA = round_to_0.1_in(max(PRCP, max(daily_delta_WTEQ, 0)))
```

Across the `35` complete primary accumulation windows (October 1 through the
observed positive SWE peak), the medians are:

| Ratio | Median |
|---|---:|
| daily `PRCP` sum / pillow peak SWE | `0.9400` |
| `PRCPSA` sum / pillow peak SWE | `1.2507` |
| fixture all-phase precipitation / `PRCPSA` sum | `0.6563` |
| fixture all-phase precipitation / pillow peak SWE | `0.8227` |

Thus the unadjusted gauge-derived record does not establish a physical
precipitation ceiling, while the current `p8.cli` remains mass-limited relative
to the pillow target under a no-lateral-import assumption. This does not
authorize a precipitation multiplier: `PRCPSA` incorporates the same `WTEQ`
target, and the modeled hillslope is 173.9 m above the station with different
footprint, canopy, and redistribution exposure. See the machine-readable
[PRCPSA diagnostic sidecar](../observations/provenance/snotel_snowbird_ut_prcpsa_diagnostic.json)
for exact source identity, ranges, checksums, and claim limits.

## Development-only precipitation lane

Future snow-process packages may use
[`development/precip_x1p2155576/p8.cli`](development/precip_x1p2155576/p8.cli)
as a separately labeled Snowbird development lane. It is derived from canonical
`p8.cli` by scaling daily precipitation only with exact decimal factor
`1.2155576` and rounding to `0.1 mm` with `ROUND_HALF_UP`. Canonical `p8.cli`
remains unchanged at SHA-256
`10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7`.

The derivative is not precipitation truth, an observation, a calibration or
default, independent snow-physics validation, or transferable evidence. Copy
the complete canonical fixture to a fresh run directory, then replace only the
staged `p8.cli`; never overwrite this fixture's canonical file. The derivative
[`manifest.json`](development/precip_x1p2155576/manifest.json) freezes the
transform, hashes, totals, consumer protocol, and claim limits.
