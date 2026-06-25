# Worker Handoff

Status: SNOWDENSITY-02 complete.

Next recommended package: `SNOWDENSITY-03 Offline Physics Core`.

First actionable item:

- Scaffold `docs/work-packages/20260625-snowdensity-03-offline-physics-core-001/`.

Required objective:

- Implement `physics_bulk` in Rust snowbench only.
- Add unit conservation tests for SWE, physical snow depth, density, retained
  liquid water, and thermal/cold-content state.
- Bind exact candidate equations/constants to `SC-SNOWFREEZE-001#INV-SNOWFREEZE-051`
  and ADR-0027.
- Produce SNOTEL rubric output without per-site constants.

Hard boundaries:

- Do not couple `physics_bulk` into production runtime yet.
- Do not change `legacy_wepp` default status.
- No per-site constants.
- Do not tune `ssd` by residual minimization.
- Do not use PySnobal/SNOBAL or legacy WEPP as acceptance targets.

Suggested gates:

- Offline state mass/energy closure.
- Fresh-snow-density bounds and monotonicity.
- Dry/wet densification sanity tests.
- Liquid retention/release/refreeze conservation.
- v74/v75 rubric profile generation for the SNOTEL corpus.
