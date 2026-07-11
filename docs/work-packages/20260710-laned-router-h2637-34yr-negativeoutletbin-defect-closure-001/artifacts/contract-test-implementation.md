# Contract-Test Implementation

Status: `EXECUTED-PRE-FIX-FAIL`

Evidence mode: `Static + Ran`

The existing terminal-deficit test was converted into the rev-51
contract-derived regression
`source_quiet_dry_front_outlet_flux_stays_nonnegative_and_conservative`.
It constructs a finite, locally consistent wet penultimate cell followed by a
near-dry outlet cell with positive committed discharge, zero source, and zero
upstream forcing. The raw predictor donor extrapolation is negative, but
accepted behavior is completion with:

- an exact-zero first predictor face distinct from positive outlet-cell `q`;
- finite nonnegative predictor/corrector faces on every retained step;
- finite nonnegative scheme outflow, outlet bins, and hydrograph samples;
- exact outlet-bin sum versus booked outflow;
- no material positivity-clamp injection; and
- independent closure using committed cell depths for storage reconstruction.

`bin_recorder_retains_material_terminal_deficit_signal` separately pins the
defensive recorder behavior: a directly injected material negative terminal
mass remains surfaced as a deficit rather than clamped or silently dropped.

Ran before any production correction:

    cargo nextest run -p openwepp-hillslope-orchestrator \
      source_quiet_dry_front_outlet_flux_stays_nonnegative_and_conservative \
      --no-capture

Result: expected `FAIL`, nextest run
`30a17d5a-de3e-41d0-9bab-513da8203b6a`; the original exact-dry form of the
vector panicked at its completion
expectation with `NegativeOutletBin`. Summary: `0 passed, 1 failed, 339
skipped`. Review disposition then strengthened the vector to a small positive
outlet discharge and directly observed each stage face. With only the rev-51
lower-bound line temporarily removed, that final vector failed with
`NegativeOutletBin` in run `22a7683c-1528-444b-9bb6-c7f630bc96f4`; after the
exact line was restored, the vector and defensive recorder passed `2/2` in run
`287ebe1a-0f18-4a1f-bdc2-86c352289576`. This proves the final regression flips
on the production correction rather than passing vacuously on pre-fix code.
