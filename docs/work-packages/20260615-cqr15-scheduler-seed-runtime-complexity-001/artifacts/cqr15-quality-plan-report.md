# CQR15 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`.

Static: protected boundaries are public API, runtime symbols, lane policy,
state seed formulas, stable error variants and details, unit meanings, parser
compatibility, float expression order, and science-contract behavior.

Ran: live baseline identified `seed_wb11_runtime_surface_inputs` as the CQR15
target with CRAP `580.6018405181356`, CC `94.0`, and coverage
`61.95426195426196`.

Static: quality action was limited to behavior-preserving decomposition of the
target into private helpers and four focused characterization tests.

Ran: final target CRAP is `15.0`; highest new helper CRAP is
`23.01930315500686`.

Status: complete.
