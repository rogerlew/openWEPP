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
