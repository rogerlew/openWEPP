# Stratum Correspondence Evidence

Status: complete.

Evidence class: Static.

## Current Modeled Surfaces

Harvard and Marcell are both current single-hillslope mixed-forest fixtures.

| Fixture | WEPP hillslope | Management class | `.man` plant reference | `.man` initial `cancov` | Current runtime `cancov` | Runtime source |
|---|---|---|---:|---:|---:|---|
| `harvard_mixed_ma` | `p8` | mixed forest | `0.55` | `0.55` | `0.55` | 10.3.1 `generated_openwepp_runtime_surface.cancov` |
| `marcell_mixed_mn` | `p10` | mixed forest | `0.55` | `0.55` | `0.55` | 10.3.1 `generated_openwepp_runtime_surface.cancov` |

The fixture `.man` files identify both as `Seasonal mixed forest` with
`Partial evergreen winter canopy`; the initial-condition line seeds
`cancov = 0.55`. The 10.3.1 runtime evidence confirms the current diagnostic
surface remains static at `0.55`.

## Observed Strata Advertised By Fixture Metadata

| Site | Observation source in current metadata | Advertised strata |
|---|---|---|
| Harvard Forest | `HF155` snow-pillow SWE + `HF237` depth/density | `hemlock`, `hardwood`, `open` for `HF237`; `HF155` is not a stratum-resolved depth/density surface in current metadata. |
| Marcell Experimental Forest | USFS RDA `10.2737/RDS-2021-0016` | `conifer`, `deciduous`, `open` SWE/depth/frost by cover type. |

No Harvard `HF237` or Marcell RDA observation tables are currently installed
under `tests/fixtures/cancov_forest/` or `tests/fixtures/snowfreeze_observed/`.
This package therefore binds metadata-level strata to modeled surfaces; it does
not ingest or compare observations.

## Binding Table

| Site | Observed stratum | Current model binding | Disposition |
|---|---|---|---|
| Harvard | `open` | none | Requires open / clearing variant or documented aggregate. Current mixed hillslope is not open. |
| Harvard | `hardwood` | none | Requires deciduous/hardwood leaf-off variant or documented aggregate. Current mixed hillslope collapses hardwood and evergreen. |
| Harvard | `hemlock` | none | Requires conifer/hemlock under-canopy variant or documented aggregate. Current mixed hillslope is not high-evergreen. |
| Harvard | site-level `HF155` SWE | provisional context only | May support site-level SWE context after source semantics are documented, but it is not the canopy-stratified depth/density verdict surface requested by 10.3.2. |
| Marcell | `open` | none | Requires open / clearing variant or documented aggregate. Current mixed hillslope is not open. |
| Marcell | `deciduous` | none | Requires deciduous leaf-off variant or documented aggregate. Current mixed hillslope collapses deciduous and conifer components. |
| Marcell | `conifer` | none | Requires conifer under-canopy variant or documented aggregate. Current mixed hillslope is not high-evergreen. |
| Marcell | mixed-site aggregate | provisional context only | Requires authored stratum weights and aggregation rule before it can carry a modeled-vs-observed verdict. |

## One-Hillslope Defensibility

One representative mixed hillslope is defensible for:

- runtime plumbing checks;
- static mixed-canopy diagnostic context;
- climate/site context in later package planning.

It is not defensible for:

- open-vs-under-canopy ordering;
- hemlock/hardwood/open or conifer/deciduous/open stratum verdicts;
- canopy-attenuation melt value claims for Harvard or Marcell;
- density/frost attribution that depends on those canopy strata.

Reason: the observed datasets intentionally separate canopy regimes, while the
current model surface collapses them into one static mixed canopy.

## Revision 2026-06-26 — Updated Binding (Paired Strata Built)

Per-stratum within-watershed hillslopes were added to
`tests/fixtures/cancov_forest/`, replacing the single-mixed-hillslope binding for
the spatial (canopy-type) axis:

| Site | Observed stratum | Model binding | `luse` | Disposition |
|---|---|---|---|---|
| Marcell | `conifer` | `marcell_conifer_mn` (52→p8) | `forest` (evergreen) | bound |
| Marcell | `deciduous` | `marcell_deciduous_mn` (73→p15) | `deciduous forest` | bound |
| Marcell | `open` | `marcell_open_mn` (42→p6) | `short grass` | bound |
| Marcell | (mixed) | `marcell_mixed_mn` (61→p10) | `mixed forest` | context |
| Harvard | `hardwood` | `harvard_deciduous_ma` (41→p6) | `deciduous forest` | bound |
| Harvard | `open` | `harvard_open_ma` (31→p3) | `short grass` | bound |
| Harvard | `hemlock` | `harvard_mixed_ma` (proxy only) | `mixed forest` | **unbound** — no pure conifer hillslope in Harvard delineation |

Enumeration basis: the `.sol` disturbed `luse` per hillslope across each
watershed. Marcell's delineation contains all four cover classes; Harvard's
contains only `mixed`, `deciduous`, and `short grass` (no pure `forest`).

The static-`cancov` limitation from SNOWDENSITY-10.3.1 is **unchanged** by this
revision — the new hillslopes still seed static initial `cancov` into the
diagnostic (deciduous `0.20` vs winter `0.067`, etc.). This revision resolves the
*spatial* (canopy-type) binding only; the *temporal* (per-day winter canopy)
precondition and observation ingest remain open before any stratum verdict.
