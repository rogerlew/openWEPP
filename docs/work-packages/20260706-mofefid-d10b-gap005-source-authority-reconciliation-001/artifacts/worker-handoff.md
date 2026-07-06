# Worker Handoff (D10B)

Status: COMPLETE (defect-shaped handoff per ADR-0018 §7)

## First actionable item

**Close defect: D14 endpoint-timing refresh** (a REQUIRED precondition of
the D15 rerun per strategy §6.1's ordering rule, triggered — not optional
— because D10B changed the profiled path):

- observable change: `prepare_step_alpha` now performs TWO friction
  fixed-point evaluations per wet cell per step (true-celerity `dq/dh`),
  partially unwinding D14 OPT-1's single-evaluation budget; the handoff
  and hydrograph export moved to conservative bin semantics; CFL dt on
  laminar-limb regimes is ~2x smaller (the pre-rev-24 dt was running at
  true Courant ~1.8).
- write set: D14-style profiling evidence only (no scheme change);
  refresh the release-grade H2637 shadow timing and the slot attribution;
  then the D15 rerun proceeds against the fresh budget.
- acceptance: a re-pinned shadow-on endpoint timing with the same
  protected-output identity evidence class D14 used.

## Then

- **D15 rerun** (opt-in production activation): `GAP-OFEROUTE-005` is
  RESOLVED (rev 25) and `INV-OFEROUTE-011`'s Case-4 surface is CLOSED;
  the remaining flip preconditions are the ones D15's preflight already
  enumerates (INV-OFEROUTE-012 activation wiring, DC01-disable, runtime
  closure hard-fail, rev-21 operand-path consumer proof, byte-flat
  default). A real-H2637 shadow re-execution needs a native
  `routing_coefficients` fixture (the D11 rev-20 fail-closed gate retired
  the legacy-fixture executed-vector test).

## Named residual refinement items (non-blocking, bounded by tests)

1. Strict-TVD gap: max single-step homogeneous TV(q) transient
   ~5e-4 m^2/s (~6% of peak) at shock formation; bounded at 1e-3 by
   `case4_manning_tvd_dissipation_is_mass_neutral_and_tv_diminishing`.
2. Boundary-flux ripple: slow, zero-mean, mass-exact instantaneous-flux
   oscillation at the outlet cell under the one-sided closure pair;
   bounded by the bin-mean export; characterized via
   `examples/steady_probe.rs`.
3. Shock-peak grid wobble +-1.3% about the oracle limit (inside the 5%
   tolerance).

These are refinement candidates for a future numerics-polish package;
none blocks activation because the acceptance surfaces (oracle
convergence, exact conservation) are independently enforced.

## Forbidden relay check

This handoff names defects/preconditions with acceptance criteria, not
diagnostic steps. `GAP-OFEROUTE-005` itself requires NO further work.
