# SNOTEL Snow Validation Fixtures

Permanent WEPP hillslope input fixtures for snow-depth and **snow-density**
fidelity validation against NRCS **SNOTEL** observations, under
[`SC-SNOWFREEZE-001`](../../../docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md)
`GAP-SNOWFREEZE-002` / `INV-SNOWFREEZE-048` and ADR-0017 external-authority
discipline. Companion to [`../snowfreeze_observed/`](../snowfreeze_observed/)
(the frost-tube / soil-temp pilot).

**Why SNOTEL:** these stations report paired **SWE (`WTEQ`)** *and* **physical
snow depth (`SNWD` sonic sensor)**, and most report soil temperature (`STO`).
`SWE / depth` yields **observed bulk density** — the quantity the
SNOWFROST-FIDELITY A–G1 arc could not measure. That is what distinguishes a
modeled snow-depth over-prediction caused by **over-accumulation** (SWE too
high) from one caused by **low density** (SWE about right, depth inflated) —
the exact fork G0/G1 reached for PySnobal to settle. These five also span five
distinct mountain snow climates.

Each fixture is the single-hillslope WEPP input set + legacy sidecars, a faithful
copy of an operator wepp.cloud build, with **one documented modification**:
`ksflag` set to `1` (see Frost activation below).

## Climate configuration (all sites)

Observed **DAYMET** daily precip/temperature (GRIDMET wind), closest **CLIGEN**
station for sub-daily storm patterns, **PRISM**-revision spatialization. DAYMET
availability is 1980–2024; ~3-yr burn-in where possible. ⚠️ These are
**high-relief mountain** sites, so the CLIGEN station (a valley town with a long
record) sits well below the alpine hillslope and PRISM supplies the lapse. The
gap is largest at **Niwot** (CLIGEN Boulder 1645 m → hillslope 3087 m) — verify
the winter temperature/precip lapse there before trusting magnitudes.

## Site catalog

| Fixture | Source run (TopazID→pN) | Modeled lat / lon / elev | SNOTEL station | Snow climate | `ksflag` | Sim years |
|---|---|---|---|---|---|---|
| `snotel_mica_creek_st_joe_id` | listed-scar (21→p1) | 47.1487, −116.2632, 1455 m | Mica Creek `623:ID:SNTL` (St. Joe) | N. Rockies maritime-intermountain | 1 (override†) | 1986–2024 (39) |
| `snotel_paradise_wa` | open-source-thirtieth (22→p2) | 46.7836, −121.7488, 1575 m | Paradise `679:WA:SNTL` | Cascades **maritime** | 1 (override†) | 1980–2024 (45) |
| `snotel_css_lab_ca` | anaphylactic-vernacular (22→p2) | 39.3257, −120.3669, 2165 m | CSS Lab `428:CA:SNTL` | Sierra **maritime** | 1 (override†) | 1980–2024 (45) |
| `snotel_snowbird_ut` | barred-pro (53→p8) | 40.5712, −111.6605, 2969 m | Snowbird `766:UT:SNTL` | Wasatch **intermountain** | 1 (override†) | 1986–2024 (39) |
| `snotel_niwot_co` | deathless-wangle (22→p2) | 40.0369, −105.5422, 3087 m | Niwot `663:CO:SNTL` | CO Front Range **continental** | 1 (override†) | 1980–2024 (45) |

All `luse = forest`, `ksatadj = 0` (standard WEPP, not the forest sat-fraction
conductivity model).

## SNOTEL observation source

- **NRCS AWDB REST API**, network code `SNTL` — the same API the harness already
  uses for the SCAN site in `../snowfreeze_observed/`; only the network code
  changes, so it is a near drop-in.
- Daily data: `https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/data?stationTriplets=<triplet>&elements=WTEQ,SNWD,PREC,TOBS,TMAX,TMIN&duration=DAILY&beginDate=<YYYY-MM-DD>&endDate=<YYYY-MM-DD>`
- Metadata/elements: `https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/stations?stationTriplets=<triplet>&returnStationElements=true`
- **Units:** AWDB returns English — `SNWD`/`WTEQ` in **inches**, temps in **°F**.
  Convert to SI (depth → m, SWE → mm, °F → °C). `STO` returns depth-tagged values.

The normalized H corpus is checked in under `observations/`:

- `observations/manifest.json`
- `observations/sites/*.csv`
- `observations/provenance/*.json`
- `observations/ssd_characterization.{json,md}`

SNOWFROST-FIDELITY-H scores these rows under the v74
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-050` rubric. PySnobal is diagnostic flag
evidence only; CSS Lab WY2017 is dispositioned as a known upstream
PySnobal/SNOBAL thin-snow numerical instability, so affected PySnobal profile
cells are unavailable rather than openWEPP failures.

## Pairing window (read before validating)

The **`SNWD` depth sensor postdates the SWE pillow at every site**, so the paired
SWE+depth (density) record starts at the per-site `SNWD` begin date (2002–2006),
runs to present, and overlaps DAYMET to 2024. **Use the `SNWD` era for
depth/density work** (per-site dates in the manifests). SWE-only checks can reach
back to ~1979–1989.

## Frost activation (`ksflag`)

All five wepp.cloud builds are `forest` land use and inherited `ksflag = 0` (the
legacy "frost disabled for non-ag" default). Because the program needs the frost
model active alongside snow, **`ksflag` was deliberately set to `1` at all five**
(`†` in the catalog): soil data line `1 0` → `1 1`, comment `# ksflag -> 0` →
`# ksflag -> 1`. This is the **only** modification to the as-built inputs; revert
both edits (or re-extract from the source run) to recover `ksflag = 0`.

## Fixture contents

Each directory contains the single-hillslope WEPP input set + hillslope sidecars:
`pN.{run,man,slp,sol,cli}` (`N` = `wepp_id`), `snow.txt`, `pmetpara.txt`,
`gwcoeff.txt`. Watershed-scoped files (`chan.inp`, `chntyp.txt`, `tc.txt`,
`wepp_ui.txt`) are excluded.

## Running

```
openwepp-cli-hill <fixture_dir> pN.run   # produces HBP shard + parquet
```

## Status

SNOTEL pilot (5 sites) for `GAP-SNOWFREEZE-002` snow-depth/density fidelity,
spanning Cascades/Sierra maritime, Wasatch intermountain, CO-Front-Range
continental, and N.-Rockies (St. Joe). Adds observed **density** (SWE/depth) the
existing `snowfreeze_observed/` pilot lacks.
