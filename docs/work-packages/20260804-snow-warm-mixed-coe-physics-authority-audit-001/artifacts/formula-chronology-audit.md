# Formula Chronology Audit

Status: complete

Evidence mode: Static + Ran

## 1995 Handbook

Chapter 3 Section 3.6 describes an hourly empirical melt-depth equation derived
from a Corps basin equation and modified by Hendrick et al. and WEPP. It writes
`hrmelt = 0.0254(A-B+C+D)` and states:

- `A=0.0607*hrrad*(1-cancov)`, with a cold-hour attenuation from `-4 C` to
  `0 C`;
- `B=0.84*(1-clouds)`, subtracted as a clear-sky/longwave-loss proxy;
- `C` as a wind, air-temperature, dewpoint, canopy-height,
  displacement-height, and roughness expression; and
- `D` as an hourly air-temperature term plus rain heat.

The same section assumes snow albedo `0.5`, surface soil temperature `0 C`
during melt, no snowmelt when daily `Tmax < -3 C`, non-negative final melt, a
pack cap, and no exported liquid melt below the density gate. Those handbook
assumptions are distinct from independent full energy-balance use of snow
surface temperature and cold content. See PDF pages 3.6-3.7, Equations
3.6.1-3.6.6.

## Pinned 2007-2008 Legacy Amendments

The normative pinned blob records its changes in source:

1. The below-freezing `A` attenuation is commented out
   (`melt.for:156-169`).
2. `B` becomes a signed combination of hourly air temperature and clear-sky
   loss (`melt.for:175-182`).
3. `C` drops the dynamic roughness/displacement expression, uses CLIGEN's
   10 m wind height and a one-sixth adjustment, and adds an air-temperature
   canopy branch plus a calm branch (`melt.for:188-229`).
4. `D` drops the handbook background air-temperature term and retains only
   rain heat, selecting dewpoint only when it is above freezing
   (`melt.for:232-262`).
5. Signed hourly totals are retained and later redistributed at daily cadence
   (`melt.for:272-301`; `winter.for:414-464`).
6. `snowd.for:112-193` gates all hourly formula calls with the daily
   `(Tmax+Tmin)/2 >= 0 C` condition.

These are material equation changes, not comments or unit-only refactoring.
They postdate and supersede the handbook as migration behavior, but the source
does not supply independent scientific validation of their transferability.

## Current Rust

Static comparison finds the current producer uses the same constants, signs,
branches, legacy unit factors, 10 m wind-height adjustment, and caller gates.
Typed unit conversion and finite/domain errors replace Fortran's implicit
surfaces without changing the audited arithmetic. Canonical 21L lanes use a
shortwave absorbed fraction of exactly `1.0`, preserving the pinned `A`
coefficient lineage.

Ran: independent reconstruction of all four produced terms, their uncapped
sum, applied-plus-cap closure, and daily aggregates passed on `394705` hours
and `17431` site-days. The worst residual was
`9.941202185450096e-18 m`; daily aggregates reproduced exactly. The first
attempt was rejected before publication because it evaluated the formula on
mixed days that the daily midpoint-temperature caller gate bypassed. The
corrected freeze adds the four accepted climate identities and changes no
threshold, coefficient, population, or outcome rule.

## Dimensional Classification

`A`, `B`, `C`, and `D` are empirical inches-of-water contributions before the
final `0.0254 m/in` conversion. Their coefficients combine physical analogy,
time scaling, and legacy calibration. They are not separately measured
shortwave, longwave, sensible, latent, or precipitation energy fluxes.

The package converts a depth `d` to `d*rho_w*L_f/3600` only to show a
latent-heat-equivalent hourly magnitude. This is explicitly
`ASSUMED_FOR_DIMENSIONAL_AUDIT`, not energy closure. The largest per-hour
applied equivalents were `217.56`, `228.63`, `306.37`, and `255.80 W m^-2`
for Mica Creek, Niwot, Paradise, and Snowbird, respectively.

## Chronology Finding

The current term generator and audited caller gates are faithful to the pinned
baseline, while the pinned formula materially differs from its handbook
description. Full energy-balance formulations use state-resolved preconditions
that CoE omits, but Ohmura 2001 also supports bounded temperature-index methods
as practical proxies. The decisive gap is therefore the absence of cited
independent validation or bounded transferability authority for the material
2007/2008 changes, not the mere existence of an empirical formulation. This
supports `BASELINE_FIDELITY_WITH_AUTHORITY_GAP`, not
`RUST_TRANSCRIPTION_DEFECT`.
