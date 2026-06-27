# Canopy-Stratified Forest Snow Melt / Cancov Fixtures

Permanent WEPP single-hillslope input fixtures spanning a **canopy gradient**
(coniferous → mixed → deciduous → pasture) for snow **melt** and **canopy-cover
(`cancov`)** fidelity work under `SC-SNOWFREEZE-001` / `GAP-SNOWFREEZE-002` and
the [snow-frost-fidelity strategy](../../../docs/planning/snow-frost-fidelity-strategy.md).
Companion to [`../snotel_observed/`](../snotel_observed/) (high-`cancov` conifer +
SNOTEL clearings) and [`../snowfreeze_observed/`](../snowfreeze_observed/) (frost).

## Why this set

The SNOWDENSITY melt-modernization (CoE energy-balance + shortwave + Brock
albedo) was found **neutral at high evergreen `cancov ≈ 0.9`** (SNOWDENSITY-05G:
`amelt ∝ (1-cancov) ≈ 0.1`, so the shortwave/albedo term is ~90% attenuated).
**It has never been tested in the lower-`cancov` regime where it could matter** —
deciduous leaf-off, mixed forest, and pasture, where `(1-cancov)` is larger and
the radiation/albedo melt physics is more active. These fixtures provide exactly
that regime. They also:

- exercise the new wepppy **deciduous / mixed forest managements** (Harvard,
  Hubbard Brook — distinct winter-canopy phenology vs the evergreen default);
- include a **pasture** site (Sleepers River) in the same maritime climate as the
  non-SNOTEL Vermont frost blocker (`NON-SNOTEL-OPT-IN-SNOW-CONTROL-FAILED`,
  SNOWDENSITY-08/09), where modeled snow over-accumulates — a winter-melt /
  partition hypothesis these lower-`cancov` fixtures are positioned to probe.

## Site catalog

| Fixture | Source run (TopazID→pN) | Canopy | Climate | `ksflag` | Observation source |
|---|---|---|---|---|---|
| `hjandrews_conifer_or` | joyous-armchair (22→p2) | coniferous | Pacific maritime, 1980–2024 | 0→1 | EDI `MS007` + `719:OR:SNTL` |
| `tenderfoot_conifer_mt` | askance-regularity (22→p2) | coniferous | N. Rockies continental, 1980–2024 | 0→1 | `1008`/`1009:MT:SNTL` (on-forest) |
| `berthoud_conifer_co` | old-fluorosis (32→p4) † | coniferous | CO subalpine, 1986–2024 | 0→1 | `05K14S:CO:SNTL` |
| `morescreek_conifer_id` | praetorian-talcum (42→p6) † | coniferous | ID intermountain, 1986–2024 | 0→1 | `15F01S:ID:SNTL` |
| `harvard_mixed_ma` | undescended-conserve (43→p8) | **mixed** | NE transitional, 1980–2024 | 0→1 | `HF155` SWE + `HF237` strat. |
| `marcell_mixed_mn` | juvenile-separatist (61→p10) | **mixed** | Laurentian cold continental, 1980–2024 | 0→1 | USFS `10.2737/RDS-2021-0016` (strat.) |
| `hubbardbrook_deciduous_nh` | scabby-demographic (62→p10) | **deciduous** | N. Appalachian, 1980–2024 | 0→1 | `knb-lter-hbr.27` + `2069:NH:SCAN` |
| `sleepers_pasture_vt` | interconnected-fit (23→p3) | pasture/ag | NE VT maritime, 1980–2024 | 1 (already) | USGS `10.5066/P9NMQX70` |

† **RAP_TS-adjusted `cancov`** in the source build (Berthoud, Mores Creek).

The deciduous/mixed/pasture rows are the **lower-`cancov`** half of the gradient
(the new regime); the four coniferous rows are the high-`cancov` control that
extends `snotel_observed`.

## Canopy-stratified hillslopes (Marcell / Harvard / Hubbard Brook)

Per SNOWDENSITY-10.3.2, a single mixed hillslope cannot bind to the **stratified**
observations (Marcell `conifer/deciduous/open`; Harvard `hemlock/hardwood/open`).
These additional **within-watershed** hillslopes provide model counterparts for
each stratum — same site, same climate, so they are forcing-robust for isolating
the canopy-attenuation term from snow-state/forcing error.

| Fixture | Source (topaz→p) | Stratum (`luse`) | Pairs with observed stratum |
|---|---|---|---|
| `marcell_conifer_mn` | 52→p8 | `forest` (evergreen conifer) | Marcell **conifer** |
| `marcell_deciduous_mn` | 73→p15 | `deciduous forest` | Marcell **deciduous** |
| `marcell_mixed_mn` (catalog) | 61→p10 | `mixed forest` | — |
| `marcell_open_mn` | 42→p6 | `short grass` (open) | Marcell **open** |
| `harvard_deciduous_ma` | 41→p6 | `deciduous forest` (hardwood) | Harvard **hardwood** |
| `harvard_mixed_ma` (catalog) | 43→p8 | `mixed forest` | — |
| `harvard_open_ma` | 31→p3 | `short grass` (open) | Harvard **open** |
| `hubbardbrook_mixed_nh` | 33→p4 | `mixed forest` | complements `hubbardbrook_deciduous_nh` |

**Coverage and limits (from the NLCD-delineation enumeration):**

- **Marcell is the clean stratified site** — its delineation contains all four
  strata (`forest`/conifer, `deciduous`, `mixed`, `short grass`/open), so the full
  conifer/deciduous/open observation trio has model counterparts.
- **Harvard has no pure conifer/hemlock hillslope** — its delineation produced only
  `mixed`, `deciduous`, and `short grass`. The Harvard **hemlock** observation
  stratum therefore has no pure model counterpart; the `mixed` hillslope is the
  closest proxy.
- **Hubbard Brook** is `deciduous` + `mixed` only (no pure conifer or open in the
  delineation); `hubbardbrook_mixed_nh` adds the within-site deciduous-vs-mixed
  contrast.
- **Sleepers River is mono-cover** — all three of its hillslopes are
  `agriculture crops`; it provides no forest strata, so `sleepers_pasture_vt`
  alone represents it.

## Stratified observation tables

Normalized Harvard HF237 and Marcell RDS-2021-0016 observations are installed in
`observations/`:

- `observations/sites/harvard_hf237_strata.csv` provides daily Harvard open,
  hardwood, and hemlock snow depth/SWE/density rows.
- `observations/profiles/harvard_hf237_density_profiles.csv` provides Harvard
  vertical density profile rows.
- `observations/sites/marcell_rds_2021_0016_points.csv` preserves Marcell
  point-level snow depth/SWE observations.
- `observations/sites/marcell_rds_2021_0016_snowcourse_means.csv` and
  `observations/sites/marcell_rds_2021_0016_stratum_means.csv` provide
  snowcourse and conifer/deciduous/open means.

Harvard hemlock remains installed as an observation stratum but unbound to a
pure model hillslope. Marcell conifer/deciduous/open and Harvard open/hardwood
are model-bound for 10.3.3-style gradient adjudication.

## Frost activation (`ksflag`)

The forest and short-grass builds inherited `ksflag = 0` (legacy "frost disabled
for non-ag" default); each was set to **`1`** so the frost model runs alongside snow — soil
line `1 0` → `1 1`, comment `# ksflag -> 0` → `# ksflag -> 1`. This is the
**only** modification to the as-built inputs. `sleepers_pasture_vt` already had
`ksflag = 1` (ag/pasture default) and was left unchanged. Revert the two edits
to recover `ksflag = 0`.

## Climate configuration

DAYMET daily precip/temperature + GRIDMET wind + closest CLIGEN station +
PRISM spatialization (CONUS). Per-site periods in the catalog; Berthoud and
Mores Creek start 1986 (SNOTEL-aligned), the rest 1980.

## Fixture contents

Each directory: `pN.{run,man,slp,sol,cli}` + hillslope sidecars `snow.txt`,
`pmetpara.txt`, `gwcoeff.txt`, and a `manifest.md`. Watershed-scoped files
(`chan.inp`, `chntyp.txt`, `tc.txt`, `wepp_ui.txt`) are excluded.

## Running

```
openwepp-cli-hill <fixture_dir> pN.run   # produces HBP shard + parquet
```

## Notes

- **Marcell EF, MN** (`marcell_mixed_mn`, TopazID 61) is the canopy-stratified
  mixed standout (biweekly SWE/depth/frost by conifer/deciduous/open cover type
  since 1962). Its earlier delineation failure (WhiteboxTools *"No channels
  remain after initial qualification thresholding"*, flat peatland terrain) was
  resolved with a lowered channel-initiation threshold, and the fixture is now
  included — completing the conifer→pasture gradient (now extended to 14 fixtures
  with the per-stratum hillslopes above).
