# CQR15 Coverage Closure

Status: complete.

Ran: before LCOV target-file coverage:

- Lines: `1220/1819`, `67.07%`
- Functions: `52/81`, `64.20%`

Ran: after LCOV target-file coverage:

- Lines: `1424/2011`, `70.81%`
- Functions: `79/108`, `73.15%`

Static: target-file line coverage improved by `3.74` percentage points.

Static: target-file function coverage improved by `8.95` percentage points.

Static: ADR-0021 module threshold is not fully satisfied for this large mixed
runner file. This is recorded as a package WARN, not a CQR15 blocker, because
the scoped metric target and new helpers are closed and target-file coverage did
not regress.
