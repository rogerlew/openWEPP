# Implementation Evidence

Status: technical implementation complete

Evidence mode: **Static + Ran**

The production change is confined to Wave-1 diagnostic integration:

- `wave1_integrate_rate_block` implements uniform-grid Simpson `1/3`, Simpson
  `3/8`, and the unavoidable one-interval trapezoid fallback;
- `wave1_flux_run_residual` forms nonoverlapping blocks and sums absolute block
  disagreements;
- each analytic-deposition or RK4 sub-march allocates a diagnostic-zone ID, so
  non-grid-aligned coefficient and critical-shear boundaries cannot be crossed;
- `wave1_flux_closure` admits only contiguous, unclamped, same-region,
  same-zone intervals and computes numerator and scale over the identical
  eligible population; and
- `wave1_totals` invokes this diagnostic before the unchanged typed
  `erosion.wave1.flux_closure` refusal.

The exact publication closure is evaluated separately before this diagnostic
and remains unchanged. The production hourly fold is dependency-injected only
at its internal test seam; its production call still uses the same quantum
solver. A behavioral test proves only `erosion.wave1.flux_closure` becomes an
explicit zero-sediment refusal and `erosion.wave1.publication_closure` remains
hard-fail. The Wave-1 solver, constitutive rate, grid spacing,
transport and detachment coefficients, denormalization, snow correction, and
consumer refusal handling did not change.

The corrected EROD16 fixture retains `4/231` diagnostic refusals and 227
depositing solutions. Final review-correction evidence is in logs 31–33.
