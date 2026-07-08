# Step Trace Design

Status: `ACTIVE`
Evidence mode: Static.

The diagnostic step trace is opt-in and row-scoped. It must not change
default/off behavior, production mesh policy, or solver math.

Required row selector:

- `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1`
- active routing and active trace must also be enabled

Required per-step evidence:

- step index, start/end time, and dt
- rung mesh cell count and dx
- source mass and upstream inflow mass
- storage before/after
- predictor and corrector downstream face fluxes
- booked outflow mass
- max Courant number plus controlling cell index/x-position
- predictor/corrector limiter reductions and face positions
- final TVD scale and dominant TVD correction position
- outlet cell depth/discharge after commit

Committed analysis summarizes this raw trace; raw per-step rows stay in the
ignored run tree. The analyzer must independently reconstruct the published
outlet bins by clipping step outflow masses to the 900-second bin spans before
it rules out a boundary/bin attribution defect.
