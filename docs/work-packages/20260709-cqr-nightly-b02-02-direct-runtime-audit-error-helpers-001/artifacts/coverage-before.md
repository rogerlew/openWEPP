# Coverage Baseline

Ran: delegated fresh batch LCOV measurement, as recorded in
`target-selection.md`.

The target's LCOV summary is `198/319` executable lines (62.069%). The high-CRAP
`DirectRuntimeError::fmt` branch family has only 7.692% coverage. Existing
direct-runtime tests mostly assert typed variants at call sites; they do not
exercise each public display diagnostic.

Characterization will add one exact-output case per `DirectRuntimeError` variant
before decomposing the display implementation.
