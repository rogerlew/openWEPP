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

NOT RUN. No benchmark was run because resumed R6 stopped before production
Rust/output edits. The blocker is structural: no run-bound direct publication
frame exists to benchmark as an R6 direct-publication candidate.
