# PERFDEEP07 Disabled-Path Baseline

Status: queued.
Evidence mode: not-run.

## Required Evidence

Record H2637 no-UI default-disabled runs with all PERFDEEP opt-ins disabled.

Minimum gate:

- protected output identity evidence;
- at least three clean endpoint runs;
- min/median/max seconds;
- RSS for each run;
- exact command and environment;
- same-machine control where feasible;
- PASS only if median `<= 676.67 s` or a hard external-environment attribution
  is accepted by review.
