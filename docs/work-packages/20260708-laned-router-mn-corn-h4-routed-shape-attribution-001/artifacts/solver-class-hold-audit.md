# Solver-Class Hold Audit

Evidence mode: Ran.

## Hold

Status: `EXECUTED-HOLD-SOLVER-CLASS-DAY792`.

The package cannot complete the metric-repair branch. The day-792 lane-1 miss
is attributable to raw outlet-hydrograph nonconvergence on the tested rungs,
not to a scale-free metric magnifying near-zero mass and not to hourly edge
aliasing with converged cumulative arrival.

## Blocker

`mn_corn_h4`, `sim_day_index=792`, `lane_index=1` fails the strict
fine-reference shape adequacy path on `dx1p25` vs `dx0p625`:

- hourly shape L1: `0.02094494047849004`
- one-third adequacy threshold: `0.0166667`
- absolute hourly mass L1: `0.011445388178193001 m3`
- pair outlet delta: `9.041475876114813e-5 m3`
- pair storage delta: `9.041475875914973e-5 m3`
- hourly CDF Linf: `0.009920733019868733`
- raw outlet-bin mass L1: `0.01864667157360986 m3`
- sampled raw hydrograph L1: `2.071852397067762e-5 m3/s`

The same raw-hydrograph surfaces are smaller on `dx2p5` vs `dx1p25`, so the
fine-rung pair does not show monotone convergence for this row.

## Considered In-Envelope Closure Routes

### Contract metric amendment

Rejected for this package. The user handoff made the amendment path binding
only after the three discriminating tests classify the miss as metric-class.
They do not.

### Widen the existing threshold

Rejected by scope and authority. The package explicitly forbids widening the
existing threshold in place under any classification.

### Production target-dx flip

Rejected by scope. The package stops after classification and does not perform
the `dx5` ratification or production default flip.

### Numerics correction

Out of envelope for this package. The artifact identifies a solver/day
nonconvergence class but does not yet isolate the numerical mechanism. The
next package needs trace-level solver diagnostics for the same row and nearby
package-level shape outliers before a contract-authorized solver correction can
be designed.

## First Actionable Follow-On

Scaffold and execute:
`20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001`.

Initial objective:
close the day-792 raw outlet-hydrograph nonconvergence blocker by identifying
the active TVD-MacCormack numerical mechanism, then either implement a
contract-authorized correction or hold with a narrower mechanism-level
authority blocker.

Minimum starting evidence:

- rerun day 792 lane 1 with per-step mass, limiter, CFL, face flux, and source
  application traces at `dx2p5`, `dx1p25`, and `dx0p625`
- compare the first divergent time interval and cell region
- include the other high shape rows from the current ladder so a row-specific
  fix is not mistaken for a general mesh-policy correction
- preserve `SC-OFEROUTE-001` rev-41 positivity and closure guards

## No Partial Flip

No production mesh-policy default change, target-`dx` promotion, shape-gate
contract amendment, or tolerance widening landed in this package.
