# Implementation

Status: `EXECUTED`

Evidence mode: `Static + Ran`

## Production Correction

`SC-OFEROUTE-001` rev 51 now makes the one-way downstream boundary domain
explicit: every scheme-actual stage face is finite and nonnegative, and the
predictor outlet face is
`max(0, 2 q[n-1] - q[n-2])` before the existing rev-41 available-water upper
cap.

`KinematicWaveSolver::step` implements that contract at the point where the
predictor face is constructed. It:

1. computes the raw donor extrapolation;
2. rejects a non-finite result with the existing typed
   `RoutingError::NonFiniteState`;
3. applies the exact physical lower bound of zero; and
4. passes that same bounded face through the existing conservative stage
   limiter, state update, and outflow ledger.

The correction does not alter the corrector stencil, depth state, forcing,
seam booking, bin recorder, guard threshold, or daily/off path. It adds no
tolerance, empirical damping, post-update clamp, mass injection/removal, or
publication-only masking. `NegativeOutletBin` remains an unchanged defensive
failure for an independently supplied material negative bin.

## Contract-Derived Regression

`source_quiet_dry_front_outlet_flux_stays_nonnegative_and_conservative`
constructs a two-cell wet-penultimate/near-dry-positive-outlet state whose raw
predictor face is provably negative. It requires an exact-zero first predictor
face distinct from the positive committed outlet discharge, finite
nonnegative predictor/corrector faces on every retained step, completion,
nonnegative routed quantities, exact bin-to-ledger equality, numerical-scale
clamp mass, and an independent storage reconstruction from committed depths.

`bin_recorder_retains_material_terminal_deficit_signal` separately proves
that the downstream guard was not weakened.

The original exact-dry regression failed on the pre-correction source with
`NegativeOutletBin` (nextest run
`30a17d5a-de3e-41d0-9bab-513da8203b6a`). After independent review strengthened
the vector to a positive near-dry outlet with direct stage-face observations,
temporarily removing only the rev-51 lower-bound line reproduced the same
expected failure (run `22a7683c-1528-444b-9bb6-c7f630bc96f4`). Restoring the
exact correction made the strengthened vector and defensive recorder pass
`2/2` (run `287ebe1a-0f18-4a1f-bdc2-86c352289576`).

The complete `openwepp-hillslope-orchestrator` crate passed all `340` tests in
`148.988 s`; the final post-review workspace profile then passed all `1694`
executed tests.
