# RST And Pre-Peak Flux Disposition

Status: `COMPLETE EXECUTION / PHASE INSUFFICIENT / PRE-PEAK LIQUID RELEASE DOMINATES / NO CORRECTION`

Ran: 72 real-consumer cells across Mica Creek, Niwot, Paradise, and Snowbird:
nine `rst` values from `0` through `4 deg C` under both the active
`harder_pomeroy_hourly` phase model and the existing diagnostic `legacy_rst`
selector. Annual ledgers span water-year start through each observed SNOTEL
peak. SNOTEL remains calibration evidence. Values above `1 deg C` are
`ASSUMED_FOR_EXECUTION` best-case stress values, not recommended calibration.

## Phase Response

The active Harder-Pomeroy arm is exactly invariant to `rst`. This is expected
from the real consumer: that model uses hourly air temperature plus relative
humidity to estimate hydrometeor temperature and phase fractions, while only
the legacy selector evaluates `hrtemp > rst`.

The legacy-threshold stress arm responds monotonically, but even `4 deg C`
does not recover observed peak SWE:

| Site | Peak ratio at 0 C | Peak ratio at 4 C | Change | Peak-date offset at 4 C |
|---|---:|---:|---:|---:|
| Mica Creek | 0.497 | 0.843 | +0.346 | -15 d |
| Niwot | 0.434 | 0.476 | +0.043 | -30 d |
| Paradise | 0.349 | 0.641 | +0.292 | -28 d |
| Snowbird | 0.308 | 0.385 | +0.076 | -46 d |

Snowbird is decisive. Raising legacy `rst` from `0` to `4 deg C` increases
median admitted snow accumulation from `0.561` to `0.689 m`, but actual
pre-peak pack loss also rises from `0.458` to `0.506 m`. The resulting median
peak ratio improves only `0.308 -> 0.385`. The active Harder-Pomeroy result is
already similar to the legacy best-case input: accumulation `0.670 m`, pack
loss `0.482 m`, peak ratio `0.377`, and peak timing `-47 d`. Therefore an `rst`
calibration cannot explain or repair Snowbird under the active model, and even
an extreme legacy threshold cannot overcome the retention/timing failure.

The stress response is heterogeneous. Mica and Paradise gain substantial
snowfall mass, while Niwot barely responds. No common `rst` value repairs the
four-site cohort, and every site remains early at `4 deg C`.

## Flux Ownership

The additive SWE identity is:

`storage change = accumulation + rain retained - snowpack SWE loss - sublimation`.

Raw CoE melt, routed melt, liquid release, rain release, and refreezing overlap
that identity and are reported only as process diagnostics. Across all sites:

- sublimation is zero because the frozen model configuration disables it;
- retained rain is small relative to snow accumulation;
- refreezing is tiny (`~0.00003-0.0036 m` in baseline medians);
- actual snowpack SWE loss is almost exactly expressed as liquid-water release;
  and
- CoE raw melt is the dominant named generator of that liquid, although routed
  melt also contains direct rain and cannot be treated as an additive pack sink.

At active-model Snowbird, median pre-peak accumulation is `0.670 m`, actual
pack loss is `0.482 m`, liquid release is `0.483 m`, and raw CoE melt demand is
`0.392 m`. Retained rain is only `0.0025 m`, refreezing about `0.0010 m`, and
sublimation zero. This makes accumulation-season melt/liquid evacuation—not
phase partition alone—the principal modeled mechanism suppressing retained
SWE.

## Evidence Limits

Median flux differences are descriptive and are not additive across different
annual medians. Harder-Pomeroy versus legacy-RST is a model-form diagnostic,
not a promotion comparison. The experiment identifies where modeled mass goes;
it does not establish which melt coefficient or energy term is physically
wrong. No production selector, threshold domain, coefficient, forcing,
contract, or snow physics changed.

Maximum daily/window mass-closure residuals are `8.95e-16 m` and `3.56e-15 m`
across both arms.
