# Kinematic-Wave Equilibrium Rating Z Structure

Status: Active knowledge note.

Provenance: ADR-0037 and archive branch
`abandoned/hybrid-implicit-stepping` at
`b1d5fd4410b700012d857ef4056000163e6aa6a0`. The source contract was
`SC-OFEROUTE-002` rev 5 before its removal from main.

The overland-routing skin-friction regime dispatch in
`SC-OFEROUTE-001#INV-OFEROUTE-002` switches at
`Q_c = 1000 * nu`: Shen and Li for `Re <= 1000`, Hirsch for `Re > 1000`.
When this dispatch is used as an equilibrium rating, the full rating is
Z-shaped rather than single-valued across the overlap band. The laminar
and turbulent branches each remain monotone on their own branch, but their
valid equilibrium intervals overlap and their fixed-point basins split at
`Q_c`.

Consequences for future solvers:

- A phrase like "the equilibrium discharge" is incomplete unless the
  branch-selection rule is named.
- Warm starts and acceleration are not harmless by default. They can leak
  run history into branch selection unless they are proven to converge to
  the same deterministic branch value as the cold rule.
- Any root-finding chain over the full rating must handle branch jumps as
  branch-closure events, not as a continuous scalar residual.
- A future implicit or semi-implicit solver must preserve a pure input
  rule for branch choice and fail closed on non-convergence; it must not
  commit an interpolated or compatibility value inside the jump unless a
  new science contract explicitly authorizes that physics.

The abandoned hybrid stepper proved one useful structural fact: for the
backward-Euler cell line, which is strictly decreasing in depth, a true
both-branch jump collapse is unreachable for the rating geometry. If the
LOW branch jumps over the root, the HIGH branch hosts the genuine root.
That proof is retained here as numerics knowledge, not as authority for an
implicit stepper on main.
