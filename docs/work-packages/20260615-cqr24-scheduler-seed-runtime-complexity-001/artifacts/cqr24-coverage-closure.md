# CQR24 Coverage Closure

Status: complete-with-warnings.

Static: target-file coverage before CQR24:

- Lines: `1424/2011`, `70.81%`
- Functions: `79/108`, `73.15%`

Static: target-file coverage after CQR24:

- Lines: `1587/2178`, `72.87%`
- Functions: `100/129`, `77.52%`

Static: CQR24 did not regress target-file coverage. Line coverage improved by
`2.06` percentage points and function coverage improved by `4.37` percentage
points.

WARN: target-file line coverage remains below the ADR-0021 `90%` line coverage
threshold for large touched modules. The scoped CRAP target and extracted
helpers are closed; broader module coverage remains out of CQR24 scope.
