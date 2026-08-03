# SNOW-HOURLY-ERA5-DIAGNOSTIC Historical Hourly Forcing Attribution

Status: `complete / radiation-first DIVERGES / dual review and verification pass`

Date: `2026-08-03`

Plan class: `External-data diagnostic / calibration-site forcing attribution`

## Purpose

Compare ERA5 and ERA5-Land hourly radiation, cloud, dewpoint, wind, pressure,
and elevation-reconciled temperature against the retained Daymet/gridMET-based
forcing at the four EB-04W2 calibration sites. Isolate radiation before
temperature and preserve calibrated precipitation and observation roles.

## Implementation Intent

Intent: `historical forcing diagnostic + optional-provider admission design`.
This package does not change production forcing, precipitation, snow physics,
calibration multipliers, selectors, or defaults.

## Data Authority And External Boundary

- Retained sites: Mica Creek, Paradise, Snowbird, and Niwot, with the exact
  fixture centroids and 1980/1986–2024 periods recorded in the acquisition
  manifest.
- ERA5 and ERA5-Land are optional historical providers, not required runtime
  dependencies. Official Copernicus/ECMWF dataset documentation governs units,
  grids, UTC validity, and accumulation semantics.
- Result-bearing comparison requires authenticated CDS custody plus retained
  downloaded bytes and a readable NetCDF/GRIB/CSV stack. Missing credentials or
  data is an external evidence boundary, not permission to synthesize ERA5.
- Existing precipitation factors and SNOTEL observation roles are protected.

## Included Scope And Write Set

- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

All production, contracts, tests, fixtures, observations, predecessor packages,
and calibrated outputs are read-only.

## Frozen Variables And Transformations

| Family | ERA5/ERA5-Land variables | Canonical diagnostic unit |
|---|---|---|
| Shortwave | surface solar radiation downwards | hourly increment, `MJ m^-2 h^-1` |
| Longwave | ERA5 and ERA5-Land surface thermal radiation downwards | hourly value, `MJ m^-2 h^-1` |
| Cloud | total cloud cover (ERA5); ERA5-Land has no independent land-generated cloud state | fraction `[0,1]` |
| Temperature | 2 m temperature | `degC`; raw and elevation-reconciled retained separately |
| Humidity | 2 m dewpoint temperature | `degC` |
| Wind | 10 m u/v components | vector plus speed `m s^-1` |
| Elevation ancillary | Compatible official grid-orography/geopotential source, acquired separately because it is absent from the point time-series request | `m`; geopotential uses `z / 9.80665` |
| Pressure | surface pressure | `Pa` |

The selected ERA5 and ERA5-Land point time-series products expose radiation as
de-accumulated hourly energy. Convert `J m^-2` once to `MJ m^-2 h^-1`; do not
apply the gridded ERA5-Land 00 UTC accumulation differencing convention.
Authenticated `ssrd` in `[-4, 0) J m^-2` is normalized to exact zero under the
bounded, cohort-specific source disposition; values below `-4 J m^-2` fail
closed. Any missing hour, duplicate UTC timestamp, nonfinite value, unit
ambiguity, or grid identity mismatch also fails closed.

## Frozen Diagnostic Order And Operators

1. Authenticate source bytes, complete hourly UTC inventory, and separately
   validated compatible grid-elevation ancillary.
2. Compare shortwave and longwave chronology before any temperature adjustment:
   hourly correlation, signed/absolute error against retained hourly forcing,
   fixed-local-standard peak-hour offset, daily-energy closure, and winter-event
   windows. This operator name was corrected after review found that differing
   source-grid longitudes prevent exact cancellation in a local-solar claim.
3. Compare cloud/dewpoint/wind joint structure without changing precipitation.
4. Compare raw 2 m temperature, then an explicitly separate elevation
   reconciliation using fixed `-6.5 K km^-1`; this is
   `ASSUMED_FOR_EXECUTION`, not a calibrated mountain lapse rate.
5. Run snow chronology only as a diagnostic with calibrated precipitation held
   byte-identical. Radiation-only precedes temperature-only; a combined run is
   admissible only after both isolated lanes close.

## Advancement Criteria

ERA5/ERA5-Land may advance only to a separately authorized optional-provider
package when complete authenticated data show improved retained chronology
without magnitude compensation, precipitation changes, mass/energy closure
loss, or open-control regression. Required admission also includes explicit UTC
and accumulated-flux handling, mountain-elevation treatment, provider fallback
semantics, and a separate stochastic-climate path calibrated to observations
that reproduces the required hourly joint structure when reanalysis is absent.

No result in this package can make ERA5 a required runtime dependency.

## Phase Plan

1. Scaffold and freeze sites, variables, units, time conventions, operators,
   roles, and advancement rules.
2. Audit local ERA5 custody, CDS authorization, reader capability, retained
   Daymet/gridMET hourly evidence, and output capacity.
3. If admitted, retrieve/hash data and execute radiation-first comparisons;
   otherwise close in external-data HOLD with an exact acquisition manifest and
   first runnable command.
4. Run review, finding disposition, verification, exact-diff, security, prompt,
   roadmap, and final disposition gates appropriate to the achieved claim.

## Acceptance Criteria

1. Four retained site identities, periods, roles, and protected precipitation
   settings are frozen before external access.
2. Variables, units, UTC conventions, de-accumulated time-series radiation
   handling, and separate elevation treatment are explicit and source-backed.
3. No comparison or provider claim is made without authenticated complete data.
4. If data are unavailable, the HOLD audit names exact missing evidence,
   considered routes, and a runnable acquisition handoff without credential
   invention or silent dependency installation.
5. Dual review, finding disposition, dual verification, exact-diff, security,
   line-count, prompt, and roadmap/catalog evidence pass for the final claim.

## Validation And Delegation

Run package Python syntax/self-check, manifest/schema/hash checks, protected-path
empty diff, scoped Markdown lint, `git diff --check`, and exact inventory. No
Rust regression is selected unless production/test code changes.

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only science/data-governance reviewers and two read-only
terminal verifiers. Expected outputs are their named package-local artifacts;
write access is limited to that artifact.

## Security Impact

CDS credentials are user-owned secrets. Never print, copy, commit, or synthesize
them. No package command may modify external datasets or accounts. No dependency
installation is authorized merely to disguise unavailable data.

## Progress

- [x] (2026-08-03) User authorized scaffold and autonomous execution.
- [x] (2026-08-03) Authenticated CDS retrieval acquired the complete eight-file
  hourly cohort; `cdsapi`, `xarray`, and `netCDF4` are available locally.
- [x] (2026-08-03) Earlier no-data dual review/verification passed and is now
  explicitly historical after authenticated acquisition.
- [x] (2026-08-03) Fresh dual review passed the acquired-data shortwave-negative
  disposition; both fresh terminal verifications pass after lifecycle fixes.
- [x] (2026-08-03) Dispositioned checksum-bound `ssrd` values in
  `[-4,0) J m^-2` to exact zero at diagnostic ingress, with lower values and new
  identities failing closed.
- [x] (2026-08-03) Directly validated all eight hourly files and eight official
  grid-geopotential records; bound target elevations to exact local WEPPcloud
  project DEM/parquet identities and calculated product/site-specific offsets.
- [x] (2026-08-03) Completed dual-reviewed and dual-verified content validation
  and compatible grid-elevation custody.
- [x] (2026-08-03) Executed the preregistered radiation-first comparison and
  corrected interval, plane, and peak-operator findings through dual review;
  both exact-current terminal verifications pass.
- [x] (2026-08-03) Generated four accessible SVG figures with same-stem
  Markdown sidecars and checksum-bound data/figure manifests; dual figure
  review and both exact-current figure verifications pass.
- [x] (2026-08-03) Closed the package at the verified radiation-first boundary;
  later cloud-proxy analysis moved to its own package.

## Outcomes

Authenticated complete ERA5/ERA5-Land point-series bytes and compatible
gridded geopotential are in untracked local custody. Direct validation passes;
target elevations are bound to the originating WEPPcloud projects. Dual review
and dual verification of the content/elevation increment pass. Radiation-first
execution reports site-specific divergence and passes dual review and dual
verification. Four result-bound figure/sidecar pairs pass dual review and dual
verification. No production forcing, precipitation, calibration, or
observation artifact changed.

Final disposition: `COMPLETE / DIVERGES`. This diagnostic does not admit an
ERA provider or claim forcing accuracy, snow improvement, or causal ownership.
