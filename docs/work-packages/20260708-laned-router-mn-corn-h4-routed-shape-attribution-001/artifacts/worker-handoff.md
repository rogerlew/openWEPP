# Worker Handoff

Evidence mode: Static.

## Next Package

Suggested package:
`20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001`.

## Objective

Close the `mn_corn_h4` day-792 lane-1 raw outlet-hydrograph nonconvergence
blocker before any renewed Tier-2 target-`dx` promotion.

## Starting Evidence

- Day 792 lane 1 fails the fine-reference shape adequacy path on
  `dx1p25` vs `dx0p625`.
- The normalized hourly-shape miss maps to `0.011445388178193001 m3` of
  hourly redistribution, not noise-scale total mass.
- Hourly CDF Linf also worsens on the fine pair, so the miss is not only
  projection aliasing.
- Raw outlet-bin and sampled hydrograph comparisons also worsen on the fine
  pair, so the miss is solver/day class under the binding handoff tests.
- No `SC-OFEROUTE-001` metric amendment, target-`dx` flip, or tolerance
  widening landed.

## First Actions

1. Extend the diagnostic trace, package-locally or behind an opt-in selector,
   to capture per-step mass, CFL, limiter, face-flux, source-application, and
   storage state for day 792 lane 1.
2. Re-run `dx2p5`, `dx1p25`, and `dx0p625` with the same release-binary
   provenance discipline.
3. Identify the first divergent interval and spatial region in the raw
   hydrograph.
4. Compare against other high routed-shape rows from the ladder, especially the
   `dx2p5` vs `dx1p25` package-level max row, before designing a correction.
5. Either land a contract-authorized active-router numerics correction or hold
   with a mechanism-level authority blocker.

## Constraints

- Preserve `SC-OFEROUTE-001` rev-41 positivity and closure guards.
- Do not widen the shape threshold in place.
- Do not promote `dx5` until the raw-hydrograph blocker is closed or formally
  re-adjudicated.
- Keep H2637 synthetic-stress-only for performance/fidelity claims.
- Keep raw run trees ignored and commit compact summaries, hashes, and command
  logs.
