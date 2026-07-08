# Disposition

Evidence mode: Static + Ran.

## Outcome

Package status: `EXECUTED-HOLD-CFL-TIMESTEP-TRANSITION`.

No production router correction, tolerance amendment, or target-`dx` promotion
landed. The package adds opt-in row-scoped diagnostic step tracing and records
the mechanism hold evidence.

## Review Disposition

All review findings are accepted and dispositioned:

- A-M1: fixed with clipped step-to-bin reconstruction.
- A-M2: fixed with mesh cell count, `dx`, and max-Courant cell/x trace fields.
- B-H1: fixed by completing closure artifacts and status.
- B-H2: fixed by adding a formal hold legitimacy audit.
- B-H3: fixed by documenting rerun-before-analyzer replay and analyzer missing
  raw-trace failure guidance.
- B-M1: fixed by adding `cascade.rs` to the write set.
- B-M2: fixed by adding line-count governance.

## Mechanism Decision

The step trace rules out the in-package correction candidates:

- Source sampling: source totals match to roundoff.
- Upstream handoff: upstream inflow is zero.
- Outlet-bin attribution: clipped step-to-bin reconstruction matches published
  outlet bins to roundoff.
- Boundary-sign bug: no negative outlet outflow steps.
- Positivity/limiter defect: clamp is zero, stage limiter reductions are zero,
  and TVD limiter scaling does not fire.

The remaining blocker is timestep-policy authority. The fine pair compares a
300-second capped rung (`dx1p25`) with a CFL-limited rung (`dx0p625`). That is
not a pure spatial fine-reference check, so the next package must adjudicate
coupled timestep/mesh adequacy before renewed `dx5` ratification.
