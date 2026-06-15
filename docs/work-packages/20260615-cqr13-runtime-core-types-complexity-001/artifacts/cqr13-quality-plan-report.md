# CQR13 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`.

Static: protected boundaries are public API, error-code/display behavior, typed
errors, field names, allowed strings, parser compatibility, units, numeric
expression order, and kernel-facing projection behavior.

Static: live baseline target identity is
`HillslopeRuntimeInputError::soil_core_code`, line `393`, CC `14.0`,
coverage `93.75`, CRAP `14.0478515625`.

Static: the original rank-7 snapshot row is already closed; every current
target-file function is below CRAP `30`.

Static: no production refactor is required. Closure evidence consists of
before/after LCOV and CRAP plus required gates.

Status: complete.
