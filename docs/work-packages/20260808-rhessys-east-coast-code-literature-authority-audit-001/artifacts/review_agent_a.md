# Independent Science/Source Review A

Status: `PASS / initial HOLD findings closed`

Evidence mode: `Ran + Static`

Reviewer A inspected the pinned sources read-only and did not read Review B.
Initial verdict: `HOLD`.

Primary coordinates inspected included
`rad/compute_surface_heat_flux.c:23-32,98-106`,
`cycle/canopy_stratum_daily_F.c:1141-1173,1253-1259`,
`rad/compute_Lstar_canopy.c:136-139,196-200`,
GIS `g2w_cf_RHESSysEC.R:530-559`, GIS
`g2w_cf_RHESSysEC_soil_fullextraction.R:1054-1083`,
`init/construct_stratum_defaults.c:128,169,292,346-347`, and
`include/rhessys.h:2618,2622,2700-2701` at the pinned commits.

- Critical: `compute_surface_heat_flux`, its dimensionally divergent depth
  branches, PM energy callers, the homogeneous canopy-longwave slab, and warm
  negative-longwave clamps were omitted.
- Critical: both accepted GIS world initializers synthesize C/N state with
  fixed row indices, nonfinite-to-zero behavior, hardcoded `333.33` and `0.05`,
  and an SLA initialization/runtime mismatch.
- Critical: allocation/turnover deferral lacked severability despite feedback
  into future LAI/root state; deferred profiles/surfaces lacked named custody.
- High: source citation discovery omitted radiation, longwave, respiration,
  turnover, and litter leads.
- High: `daily_fire_turnover`, `kfrag_base`, and heat-capacity units were wrong.
- High: contract version 2 lacked an audit-sidecar Binding Exposure Index row.

Rights handling and the successor's closed production posture passed. All
findings were accepted; disposition and the reviewer's closure verdict are
recorded in `review-disposition.md`.

Closure recheck verdict: `GO`. Reviewer A confirmed every prior finding family
was closed and found no residual blocker; this is science/source review closure,
not terminal package verification.
