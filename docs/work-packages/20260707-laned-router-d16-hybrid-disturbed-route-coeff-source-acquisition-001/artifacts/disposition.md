# Disposition

Status: EXECUTED. Evidence mode: Static.

## Review Findings

Accepted and fixed:

- Real runtime producer path: added opt-in
  `disturbed.openwepp_native_managements_enabled` handling in
  `wepp_prep_service.py`. The management write path now converts to native
  `ow-lanuse-1` before writing `pN.man` only when this flag is true.
- Base lookup coverage: added defaults for `deciduous forest`, `mixed forest`,
  `high use skid`, `low or treated skid`, and `thinning`; added test coverage
  over the base lookup CSV.
- Stale evidence: updated package status and artifacts.
- Failure atomicity: `Management.apply_openwepp_native_cropland()` validates
  all loops and plant data before mutating datver/landuse/routing fields.

Accepted and held:

- Full D16 active plain-vs-hybrid cohort run was not executed in this source
  acquisition package. It remains the named follow-on.

## Verification Findings

Accepted and fixed:

- CSV trailing-whitespace/CRLF issue: static extended CSV was rewritten with LF
  line endings.
- Fixture row/key mismatch: regenerated the fixture from `forest moderate sev
  fire` key `118`, matching the route coefficient row.

No rejected findings.
