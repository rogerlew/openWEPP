# Contract Disposition

Evidence mode: Static + Ran.

## Disposition

`SC-OFEROUTE-001` required amendment. The prior rev-42 text treated the
target-`dx` adequacy ladder as fixed at `LANED_ACTIVE_MAX_DT_S = 300` for the
mesh-policy adjudication. The package evidence shows that this fixed-300 basis
misclassified the `mn_corn_h4` day-792 lane-1 shape miss.

## Amendment

Rev 43 records:

- production default remains fixed `10 cells/OFE`
- production max substep remains `300 s`
- diagnostic max-`dt` may only reduce the cap from `300 s`
- diagnostic max-`dt` requires active trace evidence
- future target-`dx` promotion requires same-`dt` spatial and same-`dx`
  timestep-refinement controls when timestep regimes differ

## Evidence

The fixed-300 spatial pair failed the one-third shape gate:

- `dx1p25_dt300` vs `dx0p625_dt300`: shape L1 `0.020944940478490041`
- threshold: `0.016666666666666666`

The same spatial pair under a shared 75 s cap passed:

- `dx1p25_dt75` vs `dx0p625_dt75`: shape L1 `0.0029828040053040839`
- threshold: `0.016666666666666666`

Controls were clean: equal source totals, zero upstream inflow, no clamp mass,
no limiter events, no negative outlet-outflow steps, and step-to-bin
reconstruction at numerical noise.

## Non-Changes

No target-`dx` production default is promoted. No routed-shape tolerance is
widened. No source, friction coefficient, shadow-mesh, hybrid, or default/off
behavior change is authorized by this package.
