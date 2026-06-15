# CQR14 Kickoff Prompt

Execute CQR14 as a behavior-preserving CRAP burn-down package for
`crates/openwepp-runner/src/release.rs`.

Required sequencing:

1. capture before LCOV and CRAP;
2. identify the live target function;
3. add characterization before production edits when needed;
4. decompose only the scoped target;
5. prove the target and new helpers are CRAP `<= 30`;
6. run and record all package gates;
7. commit and push before checking the ExecPlan row.
