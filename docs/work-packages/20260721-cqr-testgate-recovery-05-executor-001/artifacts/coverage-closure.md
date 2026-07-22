# Coverage Closure

Ran: PASS under ADR-0021 glue-tier closure. Final production coverage is
92.9551% lines and 85.0089% regions, and all 79 compiled current-profile
functions meet the 75% region floor.

Ran: the first changed-head measurement at `9fe678a7` is retained at
`/tmp/cqr-executor-post-NnnWAx`. It passed 128 executed tests but failed closure
at 80.2949% regions; 12 functions missed the region floor and
`execute_plan_stage` remained CRAP 33.9008. It was not rerun or reused.

Ran: after changed-head decomposition and focused branch characterization,
exactly one measurement at `2c0f1b12` passed 135 executed tests with two
intentional ignores in 525.36 seconds total. Source hash, clean HEAD, and Git
status matched before and after; no retry occurred.
