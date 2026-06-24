# Snow/Freeze Observed-Site Validation Fixtures

Permanent WEPP hillslope input fixtures for **frost-depth fidelity validation**
against historic frost-depth observations, under
[`SC-SNOWFREEZE-001`](../../../docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md)
`GAP-SNOWFREEZE-002` (reopened 2026-06-24) and the ADR-0017 external-authority
discipline (legacy is a flag, observations are the authority).

Each fixture is the complete single-hillslope WEPP input set for one site, plus
its legacy sidecar files. These are faithful copies of models built
by the operator through wepp.cloud (target hillslope selected by TopazID and
resolved to its `wepp_id` (`pN`) via the run's watershed translator), with **one
documented modification**: `ksflag` was set to `1` at Sites 2–5 to activate the
frost model (see Frost activation status below).

## Climate configuration (all sites)

Observed **DAYMET** daily precipitation/temperature (GRIDMET wind), closest
**CLIGEN** station for sub-daily storm patterns, **PRISM**-revision
spatialization to the hillslope. DAYMET availability is 1980–2024; a 3-year
burn-in is included in each simulation period where possible.

## Site catalog

| Fixture | Source run (TopazID→pN) | Modeled lat / lon / elev | Cover / soil | `ksflag` | Climate (CLIGEN stn) | Sim years | Observation authority |
|---|---|---|---|---|---|---|---|
| `site1_sleepers_south_field_vt` | hard-bitten-doze (21→p1) | 44.4648, −72.1492, 483 m | ag crops / Cabot silt loam (mukey 282872) | **1 (active)** | St. Johnsbury VT | 1980–2017 (38) | Sleepers River "South field", frost tube |
| `site2_sleepers_w9_hardwood_vt` | baseless-salesmanship (23→p3) | 44.4944, −72.1607, 596 m | forest / sand loam (282863) | 1 (override†) | St. Johnsbury VT | 1980–2024 (45) | Sleepers River "W9 hardwood", frost tube |
| `site3_scan_mandan_nd` | forced-bop (23→p3) | 46.7665, −100.9161, 590 m | tall grass / silt loam (2699220) | 1 (override†) | Mandan Exp Stn ND | 1991–2024 (34) | SCAN Mandan #1, soil-temp 0 °C isotherm |
| `site4_ggd498_morris_mn` | open-plan-conservatism (23→p3) | 45.5794, −95.8805, 331 m | short grass / loam (428368) | 1 (override†) | Morris WC School MN | 1980–2024 (45) | GGD498 #10 "Morris", frost tube |
| `site5_reynolds_creek_us_rls_id` | feline-wrangler (21→p1) | 43.1437, −116.7348, 1653 m | shrub / loam (486088) | 1 (override†) | Swan Falls ID | 1980–2024 (45) | Reynolds Creek US-Rls, soil temp (SHAW) |

`ksatadj = 0` for all sites (standard WEPP, not the forest sat-fraction
conductivity model).

## Frost activation status (`ksflag`)

The standard WEPP frost-depth model is gated by the soil-file **`ksflag`** flag
(data line 1 after `Any comments:`, field 2: `<nofe> <ksflag>`). **All five
fixtures have frost enabled (`ksflag = 1`).**

wepp.cloud set `ksflag = 1` only for the agricultural site (Site 1); the
forest/grass/shrub sites (2–5) inherited `ksflag = 0`, the legacy "frost disabled
for non-ag" default. Because every site here has real frost observations,
`ksflag` was deliberately set to `1` at Sites 2–5 (`†` in the catalog) so the
frost-depth model runs — consistent with how legacy WEPP runs all land use as
cropland with `ksflag` in the soil.

This override is the **only** modification to the wepp.cloud as-built inputs.
Each changed soil now reads data line `1 1` and comment `# ksflag -> 1`. To
recover the as-built `ksflag = 0` state, revert both (`1 1` → `1 0` on the line
after `Any comments:`, and `# ksflag -> 1` → `# ksflag -> 0`) or re-extract from
the source run.

## The measurement ↔ `frdp` correspondence (validity gate)

Three non-equivalent "frost depths" appear across these observation sources and
must not be conflated (doing so manufactures a fake model error):

- **Frost tube** (Sites 1, 2, 4) — frozen-water boundary; closest analog to the
  model `frdp` ice front → use for **magnitude**.
- **Soil-temperature 0 °C isotherm** (Sites 3, 5) — systematically **deeper**
  than the ice front (freezing-point depression) → use for **onset/thaw timing**
  with an explicit offset, not magnitude.
- Snow **depth + density** (not SWE) drives insulation and is the dominant
  confound; sites with paired snow observations are the ones that can isolate
  the frost model.

The binding correspondence and per-tier tolerances must be written into
`SC-SNOWFREEZE-001` before any divergence is called a defect.

## Observation data (not stored here)

These fixtures are the **model inputs**. The observed frost-depth series are
fetched separately; see each `manifest.md` for the site's DOI/URL, method,
cadence, period, and censoring caveats. Summary access:
- Sleepers River VT — `DOI 10.5066/P96753GI` (CSV, per-plot coords + snow depth)
- SCAN Mandan ND — NRCS AWDB API, `stationTriplets=2020:ND:SCAN`, element `STO`
- GGD498 Morris MN — `ftp://sidads.colorado.edu/pub/DATASETS/fgdc/ggd498_seasfrost_usa/` (`DOI 10.7265/1mcs-q536`)
- Reynolds Creek ID — USDA-ARS Box / Ag Data Commons (soil temperature, CC-BY)

## Fixture contents

Each directory contains the single-hillslope WEPP input set and the hillslope
legacy sidecars:
- `pN.run`, `pN.man`, `pN.slp`, `pN.sol`, `pN.cli` (target hillslope; `N` = `wepp_id`)
- `snow.txt` — rain/snow threshold (0.0 °C), new-snow density (100), settling density (250)
- `pmetpara.txt` — PMET coefficients (kcb 0.95, rawp 0.8)
- `gwcoeff.txt` — groundwater/baseflow coefficients

Watershed-scoped files present in the source runs (`chan.inp`, `chntyp.txt`,
`tc.txt`, `wepp_ui.txt`) are **not** hillslope inputs and are intentionally
excluded.

## Running

```
openwepp-cli-hill <fixture_dir> pN.run   # produces HBP shard + parquet
```

## Status

Pilot (5 sites) for the `GAP-SNOWFREEZE-002` frost-depth heat-flow fidelity
work. Covers agricultural + forest + rangeland and frost-tube + soil-temp
authorities. The canonical WEPP-lineage Dun-2010 Pullman/Morris measured series
remain request-only; Site 4 (GGD498 #10) is the public agricultural bridge
~11 km from the Dun-2010 Morris farm.
