# CRAP After

Ran: PASS at exact clean `2c0f1b12`. No compiled current-profile production
function exceeds CRAP 30.

- `execute_plan_stage`: CC 19, CRAP 21.216;
- `admit_stage`: CC 13, CRAP 13.0246;
- `finalize_stage_execution`: CC 9, CRAP 9.00049;
- `ExecutionRecord::from_stage_receipt`: CC 5, CRAP 5;
- four extracted stage-field helpers: CRAP 1--2.

The same-LCOV CRAP JSON SHA-256 is
`069128f181ec40a9bb459683539a82ff74d3758cdfb8ba8ba9735b630202e4c9`.
The raw inventory contains 80 functions: 79 compiled in the measured profile
and one explicit `cfg(non-unix)` omission.
