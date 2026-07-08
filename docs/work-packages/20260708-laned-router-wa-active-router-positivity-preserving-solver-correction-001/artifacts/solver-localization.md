# Solver Localization

Status: EXECUTED
Evidence mode: Static + Ran.

## Localized Mechanism

The material clamp class is inside the explicit TVD-MacCormack stage update,
not in hydrology/source producers:

- Prior WA day-1122 investigation showed `H1.wat.parquet` hydrology rows were
  unchanged across inspected rungs and days.
- The current package reproduced the failure through the active production
  consumer with identical source inputs and rev-40 failure phase.
- `ofe_routing::kinematic_wave::step` was the only production math path that
  could turn a finite non-negative source/inflow series into booked
  positivity-clamp mass before active day closure.

Before rev 41, predictor/corrector/final candidate depths that went negative
were clamped to dry and booked as positive clamp mass. On WA, that turned a
local numerical over-drainage event into day-level clamp masses larger than the
day's external routed source.

## Correction Shape

The accepted correction is conservative and local to the finite-volume solver:

- Predictor and corrector stage updates are expressed as stage face fluxes.
- Each outgoing face is capped by the upwind cell's available water over that
  substep: current storage plus already-limited incoming face flux plus valid
  source.
- The final TVD face correction is scaled uniformly toward zero only if the
  full correction would make any cell negative.
- No source, route coefficient, mesh policy, closure tolerance, or downstream
  consumer path is changed.

This keeps mass inside the numerical flux update instead of adding external
clamp mass. Residual `positivity_clamp_m2` remains only as a roundoff cleanup
surface.

## Focused Regression

`stage_flux_limiter_prevents_positive_clamp_injection` constructs a shallow
three-cell over-drain stage directly in the solver and verifies:

- the step succeeds,
- depths remain non-negative,
- `positivity_clamp_m2 <= 1.0e-18`, and
- closure is exact after run-level storage change is applied.

`final_tvd_scaling_preserves_positivity_and_total` directly exercises the
rev-41 final TVD scaling branch with a telescoping correction whose full
negative lobe would drive one cell negative. It verifies `tvd_scale = 0.5`,
non-negative committed depths, and exact total preservation.
