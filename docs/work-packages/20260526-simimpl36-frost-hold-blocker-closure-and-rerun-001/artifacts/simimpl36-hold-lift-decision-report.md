# SIMIMPL36 Hold-Lift Decision Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26
Decision: GO

## Static
- SIMIMPL36 is the authorized SIMIMPL35 blocker-closure follow-on package.
- Decision criterion is blocker closure + admissible rerun lane evidence, not
  full semantic value parity closure.

## Ran
- Replay bundle: `artifacts/replay-run-20260526T164400Z/`
- Required gates bundle: `artifacts/gates-20260526T170356Z/`

## Decision rationale
- GO is supported for SIMIMPL35 blocker closure:
  1. Shared-fixture candidate rerun succeeds (`candidate/openwepp_runner.exit_code=0`) and no longer fails with `KWRITEBACK-E-DOMAIN-VIOLATION`.
  2. Direct `/wc1` candidate rerun succeeds (`candidate_wc1/openwepp_runner.exit_code=0`) and no longer fails with `SOL-E-006`.
  3. `/wc1` semantic lane row-key admissibility is restored with explicit
     candidate year-key offset support (`--candidate-year-offset 1996`):
     `common_row_count=1095`, `only_baseline_count=0`, `only_candidate_count=0`.

## Residuals (non-blocking for SIMIMPL36 objective)
- `semantic_pass=false` persists in the year-offset lane due value deltas across
  multiple columns; this is a follow-on parity domain and not a SIMIMPL35
  blocker recurrence.
