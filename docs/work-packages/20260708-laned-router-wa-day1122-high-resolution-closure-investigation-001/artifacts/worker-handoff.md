# Worker Handoff

Status: EXECUTED-HOLD-ACTIVE-ROUTER-CLAMP-NUMERICS
Evidence mode: Static.

## Next Package Candidate

`20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001`

## Objective

Resolve or explicitly guard the WA active-router positivity-clamp numerics hold
identified by this package.

## Starting Evidence

- `artifacts/day1122-reproduction.md`
- `artifacts/magnitude-attribution.md`
- `artifacts/numerics-adjudication.md`
- `artifacts/wa-day1122-analysis.json`
- `artifacts/mesh-ladder-summary.json`
- Package-local WA rung logs and traces under
  `artifacts/mesh-ladder-runs/wa_cascades_forest_h1/`

## Required First Actions

1. Extract or build a minimal WA day-1418/day-1122 reproducer that preserves:
   source series, geometry, friction operands, upstream handoff, mesh counts,
   and active day closure books.
2. Instrument per-step or per-OFE positivity clamp, CFL, depth/discharge
   extrema, and upstream handoff mass.
3. Decide contract-first whether large positivity-clamp ratios require:
   a solver fix, a hard clamp-magnitude fidelity guard, or a ratified bounded
   residual class.
4. Re-run D10B conservation/oracle fixtures and the WA reproducer before
   reopening any target-`dx` mesh-policy promotion.

## Boundaries

- Do not change production mesh policy as a first move.
- Do not relax `SC-OFEROUTE-001` closure tolerances without contract-first
  authority.
- Do not tune route coefficients or source producers to hide the clamp
  signature.
- Keep H2637 synthetic-stress-only for performance and promotion claims.
