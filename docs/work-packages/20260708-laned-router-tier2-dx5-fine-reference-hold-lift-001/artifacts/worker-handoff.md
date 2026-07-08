# Worker Handoff

Status: `READY`
Evidence mode: Static.

## Next Package

Suggested package:
`20260708-laned-router-mn-corn-h4-routed-shape-attribution-001`.

Objective: attribute the persistent `mn_corn_h4` routed hourly shape
fine-reference adequacy miss before any renewed target-`dx` production
promotion attempt.

## Starting Evidence

- `dx1p25` vs `dx0p625` max routed-shape L1:
  `0.02094494047849004`.
- Adequacy threshold: `0.0166667`.
- Only one positive-source lane-day exceeds the adequacy threshold:
  `sim_day_index=792`, `lane_index=1`.
- `uniform_shape_rows`, `erosion_source_shape_degenerate_rows`, and
  `positive_shape_rows` do not change between `dx1p25` and `dx0p625`.
- The max row has no tail fold and only a `9.04e-5 m3` end-window storage
  difference.
- `dx5` remains the best provisional candidate and passes the
  `mn_corn_h4` candidate table against `dx1p25`, but promotion is blocked
  because the reference basis is not adequate.

## First Actions

1. Build a single-member/single-day attribution fixture around
   `mn_corn_h4`, `sim_day_index=792`, `lane_index=1`.
2. Compare routed hydrograph binning and normalized D13 hourly weights across
   `dx2p5`, `dx1p25`, and `dx0p625`.
3. Attribute whether the delta is in TVD mesh convergence, hourly-bin
   sampling, end-window storage partitioning, source-shape sampling, or D13
   normalization.
4. Only after attribution, decide between a numerics fix, consumer fix,
   predeclared tolerance amendment, or explicit no-promotion verdict.

## Constraints

- Do not amend the one-third adequacy rule at the margin without a new
  contract-reviewed package explicitly scoped to tolerance authority.
- Do not promote `dx5` until the reference adequacy blocker is closed or
  formally re-adjudicated.
- Keep runtime cost priced but secondary under the operator's fidelity-first
  posture.
- Raw run trees should stay ignored; commit compact summaries, hashes, command
  provenance, and attribution artifacts.
