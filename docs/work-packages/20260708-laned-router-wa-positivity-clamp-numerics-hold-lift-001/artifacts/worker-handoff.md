# Worker Handoff

Status: FOLLOW-ON REQUIRED
Evidence mode: Static.

## Next Package Candidate

`20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001`

## Objective

Replace the current WA active fail-closed outcome with a contract-authorized
positivity-preserving solver behavior, or prove that the active explicit
TVD-MacCormack path cannot support WA without a different solver policy.

## Starting Evidence

- This package: rev-40 clamp-source guard and executor pre-publication ordering.
- `artifacts/wa-rerun-evidence.md`: fixed10 fails day 1418; dx5 fails day
  1167 at `laned_active_clamp_exceeds_source`.
- Predecessor package: day-1122 high-resolution closure investigation and
  day-1418 magnitude attribution.

## First Actions

1. Build a focused WA day/rung reproducer that preserves source series,
   upstream handoff, geometry, friction operands, and mesh count.
2. Instrument per-step predictor/corrector/final depth minima, discharge
   extrema, clamp increments, CFL, and upstream handoff mass.
3. Evaluate solver changes contract-first. Do not alter source producers,
   route coefficients, or target-`dx` policy as a substitute for solver
   correction.
4. Re-run D10B oracle/conservation gates and WA active fixed10/dx5 evidence
   before any target-`dx` or default-promotion claim.

## Non-Goals

- No target-`dx` production promotion.
- No H2637 synthetic-stress promotion.
- No hybrid subsystem revival.
- No relaxation of rev-27 closure tolerances or rev-40 clamp-source guard.
