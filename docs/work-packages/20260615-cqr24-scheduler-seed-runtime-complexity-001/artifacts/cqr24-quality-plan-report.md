# CQR24 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`.

Static: protected boundaries are public API, runtime symbols, units, formulas,
float expression order, parser compatibility, publication shape, typed guards,
and science-contract behavior.

Status: complete.

Static: quality target closed. Final target CRAP is
`6.010666666666666`, and all extracted WB16 helpers are below CRAP `30`.

Static: quality warnings retained:

- Target file line coverage remains below ADR-0021 threshold despite improving.
- Same-file non-target CRAP rows above `30` remain for later CQR scope.
