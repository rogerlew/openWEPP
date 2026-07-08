# Implementation Notes

Evidence mode: Static + Ran.

## Diagnostic Surface

Implemented a row-scoped active-router step trace, gated by all three runtime
selectors:

- `OPENWEPP_LANED_ACTIVE=1`
- `OPENWEPP_LANED_ACTIVE_TRACE=1`
- `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=<sim_day:lane>`
- `OPENWEPP_LANED_ACTIVE_STEP_TRACE=1`

The default active path and subsystem-off path do not request step trace
storage. `RoutingResult.step_trace` remains `None` unless the selected
day/lane route call explicitly enables it.

Captured per step:

- step index, start/end time, `dt`, and max Courant
- max-Courant controlling cell index and x-position
- upstream flux/integral, source integral, scheme-actual outlet outflow
- storage before/after, positivity clamp injection
- predictor/corrector outlet face fluxes
- stage-face limiter reductions and largest reduction
- TVD scale and largest raw TVD delta location

Unit conversion stays at the active-lane boundary: the solver records per-unit
width terms (`m2`, `m2/s`), and `laned_active.rs` multiplies by lane width for
published active trace detail (`m3`, `m3/s`).

## Package Tooling

Added package-local tooling:

- `run_raw_hydrograph_numerics_ladder.py`
- `analyze_raw_hydrograph_numerics.py`

The run harness reruns only `mn_corn_h4` at `dx2p5`, `dx1p25`, and `dx0p625`
with exact release-binary provenance. Raw run trees are ignored under
`artifacts/raw-hydrograph-numerics-runs/`; committed artifacts carry summaries,
hashes, and compact mechanism evidence.

## Production Correction

No production numerics correction landed.

The step trace ruled out the in-scope localized bug classes:

- Source totals are identical across the failing fine pair to roundoff.
- Upstream inflow is zero for the target lane.
- Positivity clamp injection is zero.
- Stage-face limiter reductions are zero.
- TVD limiter scaling does not fire.
- Negative outlet outflow does not occur.
- Published outlet bins reconstruct from clipped per-step outlet masses to
  roundoff (`dx0p625` Linf `1.5265566588595902e-16 m3`).

The remaining mechanism is a timestep-policy transition: `dx1p25` remains on
the 300-second cap for 228 steps with max Courant `0.85874995859419834`, while
`dx0p625` becomes CFL-limited for 330 steps with max Courant `0.9`. The
selected-row trace records mesh/cell evidence: `dx1p25` has 65 cells at
`1.2492307692307694 m`; `dx0p625` has 130 cells at `0.62461538461538468 m`.
Changing that behavior would require timestep-policy or coupled space-time
adequacy authority, not a safe local bug fix in this package.
