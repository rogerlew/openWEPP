# Final Disposition

Status: EXECUTED-HOLD-ACTIVE-ROUTER-CLAMP-NUMERICS
Evidence mode: Ran.

## Outcome

The package reproduced and classified the WA selected-cohort active-router
blocker. It does not promote target-`dx`, does not relax closure tolerances,
and does not change production code.

The hold is active-router clamp numerics:
- `dx2p5` and `dx1p25` fail the first active day-cascade closure guard at day
  1122.
- The absolute residuals are small, but they arise after cancellation against
  very large clamp/storage operands.
- Completed-rung traces localize the dominant magnitude to day 1418 lane 5.
- Hydrology source rows are invariant across completed rungs, so the issue is
  router-internal.
- Fixed `10 cells/OFE` active default passes closure but already shows a
  material clamp-magnitude risk, so the package cannot close this as
  high-resolution-only.

## Final Status

`EXECUTED-HOLD-ACTIVE-ROUTER-CLAMP-NUMERICS`

## Next Package

`20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001`

First action: build a minimal WA day-1418/day-1122 reproducer and instrument
per-step/per-OFE positivity clamp, CFL, depth/discharge extrema, and upstream
handoff mass before any solver or contract change.
