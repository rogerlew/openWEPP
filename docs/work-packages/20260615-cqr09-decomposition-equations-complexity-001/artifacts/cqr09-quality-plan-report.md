# CQR09 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burndown for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`.

Static: protected boundaries are public API, formulas, float expression order,
typed decomposition guards, error IDs, symbols, aliases, units,
parser-compatibility, output formulas, scheduler payload behavior, and
science-contract behavior.

Ran: baseline LCOV and CRAP were captured before production refactor in
`lcov_before.info` and `crap_before.json`.

Ran: focused characterization tests were added before production refactor
because the target `resmgt` action branches needed direct branch-freezing
coverage.

Static: production edits are limited to private helper extraction and private
data-transfer structs for annual decomposition control inputs. The target
`build_annual_decomposition_control` keeps the same inputs, output type, typed
error type, state-symbol lookup order, conversion helpers, control fields, and
active action semantics.

Ran: after LCOV and CRAP were captured after the refactor in `lcov_after.info`
and `crap_after.json`.

Status: scoped quality target closed; final gate results are recorded in
`gate-results.md`.
