# Default-Disabled Regression Gate

Status: not run.
Evidence mode: Static.

## Gate

R6 must preserve the existing default-disabled timing guard:

- run at least three clean H2637 no-UI reps with direct-runtime and diagnostic
  opt-ins disabled;
- record min, median, max, and RSS;
- PASS requires median `<= 676.67 s`;
- protected output identity/equivalence must pass.

## Current Disposition

NOT RUN. The default path was not benchmarked in this turn because R6 does not
have a valid direct-publication endpoint: the opt-in cutover candidate fails
closed before writing outputs. The code change keeps default
`HillslopeRuntimeSelection::Compatibility` behavior unchanged and focused
tests continue to assert no direct publication capture in default mode through
the existing R2A/R6A counter coverage.

## Gate

NOT RUN blocks R6 completion. It is acceptable only for the current
executed-hold disposition.
