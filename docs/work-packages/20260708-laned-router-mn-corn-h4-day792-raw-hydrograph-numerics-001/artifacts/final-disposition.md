# Final Disposition

Status: `EXECUTED-HOLD-CFL-TIMESTEP-TRANSITION`.

## Summary

This package scaffolded and executed the `mn_corn_h4` day-792 raw-hydrograph
numerics investigation. It added opt-in row-scoped active-router step tracing,
reran the three-rung ladder, and classified the blocker as a timestep-policy
transition rather than a localized production bug.

No production mesh default, routed-shape tolerance, or active-router physics
change landed.

## Binding Evidence

- `dx1p25` vs `dx0p625` routed-shape L1 remains
  `0.020944940478490041`, above `0.0166667`.
- Source totals match to roundoff.
- Upstream inflow is zero.
- Clamp injection is zero.
- Stage/TVD limiter events are zero.
- Negative outlet outflow steps are zero.
- Published outlet bins reconstruct from clipped step masses to roundoff.
- `dx1p25`: 65 cells, `dx=1.2492307692307694 m`, 228 steps, max Courant
  `0.85874995859419834`.
- `dx0p625`: 130 cells, `dx=0.62461538461538468 m`, 330 steps, max Courant
  `0.9`.

## Follow-On

Scaffold `20260708-laned-router-active-router-timestep-policy-adjudication-001`
before any renewed `dx5` production mesh-policy promotion.

The first follow-on action is to run controlled `max_dt` discriminants around
`mn_corn_h4` day 792 and decide contract-first whether active target-`dx`
adequacy is a coupled space-time convergence gate.
