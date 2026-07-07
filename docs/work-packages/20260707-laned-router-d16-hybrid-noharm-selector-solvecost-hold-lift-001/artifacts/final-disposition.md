# Final Disposition

Status: EXECUTED-COMPLETE-NOHARM-SELECTOR. Evidence mode: Static + Ran.

The package closed the selected-cohort timing no-harm blocker for opt-in
hybrid request.

Key evidence:

- H2637 selected hybrid for `11590/11590` requested lane-days and improved
  from `40.05 s` active plain to `33.62 s` hybrid-request user time.
- The three non-bare selected-cohort members fell back to active plain for
  `7299/7299` requested lane-days.
- Selected-cohort aggregate user time improved from `57.01 s` active plain to
  `50.58 s` hybrid request.
- Non-bare fallback members are output-identical to active plain on summarized
  outlet and pass-sediment surfaces.
- Full workspace nextest passed: `1442/1442`.

What this unblocks:

- `GAP-OFEHYB-003` selected-cohort no-harm is resolved.
- The opt-in hybrid request is now safe from the observed selected-cohort
  non-bare timing regression at current mesh.

What remains blocked:

- Default promotion.
- Promotion-facing fidelity/tolerance ratification.
- Non-bare hybrid solve-cost viability.
- H2637 first-divergent-day/OFE attribution for outlet and pass-sediment
  deltas.

Recommended next package:

- Choose the next D16 path explicitly:
  - non-bare solve-cost optimization if broad forest/fleet hybrid value is
    still desired;
  - H2637 divergent-day/OFE attribution plus predeclared tolerance ratification
    if a narrowed bare/low-cover opt-in surface is acceptable;
  - or abandon broad promotion and document bare/low-cover scope.
