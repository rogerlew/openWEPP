# Implementation

Evidence mode: Static + Ran.

## Rust Surface

Added a diagnostic-only active max-substep selector:

- `OPENWEPP_LANED_ACTIVE_MAX_DT_S`
- finite positive seconds
- bounded to `<= 300`
- rejected unless `OPENWEPP_LANED_ACTIVE=1` and
  `OPENWEPP_LANED_ACTIVE_TRACE=1`

The selector is parsed in
`crates/openwepp-runner/src/hillslope/laned_active.rs`, carried through
`DirectLanedActiveConfig`, validated again in the orchestrator, and consumed
only by the active routed lane call. The production default remains
`LANED_ACTIVE_MAX_DT_S = 300`; subsystem-off/default behavior is not routed
through the new selector.

## Evidence Tooling

Added package-local tooling:

- `artifacts/run_timestep_policy_ladder.py`
- `artifacts/analyze_timestep_policy.py`

The run harness materializes `mn_corn_h4`, builds a release binary, records
binary provenance, runs the six `dx`/`max_dt` combinations, and records hashes
for produced outputs/traces. Bulk run trees are ignored package-locally under
`artifacts/timestep-policy-runs/`.

The analyzer compares:

- same-`dx` timestep refinement at `dx1p25` and `dx0p625`
- same-`dt` spatial refinement at `300`, `150`, and `75` seconds

## Contract Surface

`SC-OFEROUTE-001` is amended to rev 43. The amendment does not change the
production mesh default, routed-shape tolerance, source terms, coefficients,
shadow mesh, or default/off behavior. It records that future target-`dx`
promotion must use coupled space-time evidence when active timestep regimes
differ across rungs.

## Outcome

The package classifies the prior `mn_corn_h4` day-792 fixed-300 shape miss as
`TIMESTEP-POLICY-ARTIFACT-CLOSED`.
