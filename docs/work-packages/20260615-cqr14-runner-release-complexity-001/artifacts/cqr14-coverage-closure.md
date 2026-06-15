# CQR14 Coverage Closure

Ran: before LCOV: `lcov_before.info`.

Ran: after LCOV: `lcov_after.info`.

Static: target-file line coverage improved from `254/426` (`59.62%`) to
`488/571` (`85.46%`).

Static: target-file function coverage improved from `22/38` (`57.89%`) to
`46/59` (`77.97%`).

Static: `lint_release_directory` coverage improved from `0.0%` to `100.0%`.

Static: no target-file coverage regression was observed.

WARN: ADR-0021 release-tier threshold closure is not claimed for every function
in the file because `validate_release_sidecar_unlocked` remains an out-of-scope
CRAP row above `30`.
