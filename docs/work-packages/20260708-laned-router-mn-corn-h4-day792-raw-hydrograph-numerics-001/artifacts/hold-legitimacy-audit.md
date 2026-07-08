# Hold Legitimacy Audit

Evidence mode: Ran.

## Hold

Status: `EXECUTED-HOLD-CFL-TIMESTEP-TRANSITION`.

The `mn_corn_h4` day-792 lane-1 raw outlet-hydrograph nonconvergence is not
closed. The package identified the mechanism as a CFL timestep-transition in
the fine-reference comparison, not a localized source, limiter, clamp, or
boundary-sign implementation defect.

## Evidence

Release run:

```bash
OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1 \
  .venv/bin/python \
  docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/run_raw_hydrograph_numerics_ladder.py \
  --members mn_corn_h4 --rungs dx2p5 dx1p25 dx0p625
```

Analysis:

```bash
.venv/bin/python \
  docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/analyze_raw_hydrograph_numerics.py
```

Key values:

- Release binary SHA256:
  `df6fa6cd7fcfb2312cfc9d1fb75f9e1a79372d0c2cd7b1d61618ba7c07c698fd`.
- `dx1p25` vs `dx0p625` hourly-shape L1:
  `0.020944940478490041`.
- Threshold from the standing one-third adequacy rule: `0.0166667`.
- `dx1p25` vs `dx0p625` bin-mass CDF Linf:
  `0.0053651985495015708 m3`.
- `dx1p25` step source total:
  `1.3852874865748002 m3`.
- `dx0p625` step source total:
  `1.3852874865748011 m3`.
- Upstream inflow total: `0 m3` on both rungs.
- Clamp total: `0 m3` on both rungs.
- TVD-limited steps: `0` on both rungs.
- Stage-limiter reductions: `0` on both rungs.
- Negative outlet-outflow steps: `0` on both rungs.
- Clipped step-to-bin reconstruction Linf:
  `3.4694469519536142e-18 m3` at `dx1p25` and
  `1.5265566588595902e-16 m3` at `dx0p625`.
- `dx1p25`: 228 steps, max Courant `0.85874995859419834`.
- `dx0p625`: 330 steps, max Courant `0.9`.
- `dx1p25`: 65 cells, `dx=1.2492307692307694 m`,
  max-Courant cell `64` at `80.575384615384621 m`.
- `dx0p625`: 130 cells, `dx=0.62461538461538468 m`,
  max-Courant cell `8` at `5.3092307692307701 m`.

The top outlet-bin difference occurs in bin `70` (`63000` to `63900` s), with
absolute delta `0.0055405842968888272 m3`. The maximum cumulative outlet
difference occurs in bin `71` (`63900` to `64800` s), with absolute CDF delta
`0.0053651985495015708 m3`.

## Why This Is Outside This Package

The package authority allowed localized, contract-authorized corrections:
stage-face limiter bugs, outlet attribution bugs, source sampling bugs, or pure
diagnostic trace plumbing.

The trace evidence did not identify any of those defects. The failing rung pair
mixes two timestep regimes:

- `dx1p25` is still max-`dt` capped at 300 seconds.
- `dx0p625` is CFL-limited through the divergent interval.

Changing the timestep cap, enforcing a fixed-`dt` reference ladder, or redefining
the mesh-policy adequacy gate as a coupled space-time gate requires contract and
mesh-adjudication authority. Doing that inside this defect-closure package would
be tolerance fitting or policy fitting at the exact failing margin.

## First Actionable Follow-On

Scaffold `20260708-laned-router-active-router-timestep-policy-adjudication-001`.

First actions:

- Add a controlled diagnostic timestep surface or harness that can rerun
  `mn_corn_h4` day 792 at fixed `dx` with fixed or systematically halved
  `max_dt`.
- Run paired discriminants:
  `dx1p25@300s`, `dx1p25@150s`, `dx1p25@75s`, and `dx0p625` under comparable
  timestep controls, using the same day/lane step trace.
- Decide contract-first whether target-`dx` adequacy is a coupled space-time
  convergence gate.
- Only after that, resume the fidelity-first `dx5` production mesh-policy
  ratification.

No target-`dx` production flip, tolerance widening, or timestep default change
lands in this package.
