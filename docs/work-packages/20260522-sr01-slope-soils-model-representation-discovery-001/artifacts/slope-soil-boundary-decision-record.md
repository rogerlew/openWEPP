# Slope/Soil Boundary Decision Record

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline slope/soil representations are both cross-cutting runtime dependencies, not isolated parser-only surfaces.
- openWEPP architecture requires explicit typed ownership and symbol continuity before promotion.

Ran:
- Completed SR01 baseline and openWEPP source audit.

## Decision
- outcome: `BOUNDARY_EXTEND_SERIES_REQUIRED`
- allowed values:
  - `BOUNDARY_SLOPE_SOIL_ONLY`
  - `BOUNDARY_EXTEND_SERIES_REQUIRED`

## Rationale

1. Baseline slope semantics are consumed by runoff partition (`xinflo`), routing/erosion (`route`, `param`), and watershed setup (`wshinp`), so slope cannot be finalized as an isolated parser boundary.
2. Baseline soil semantics include deep layer/state transformations (0.2 m re-binning, conductivity/porosity/water-state coupling) consumed by `soil`, `infpar`, `watbal`, `perc`, and `tilage`; current openWEPP seam exports only a minimal subset.
3. openWEPP currently lacks slope runtime-surface projection and full symbol alias continuity for slope fields; therefore slope+soil cannot be treated as closed implementation boundary yet.
4. Correctness-over-completion policy requires sequencing additional boundary packages before declaring closure.

## Consequences

- SR01 closes as discovery/decision complete, with explicit follow-on queue required.
- Next packages must implement:
  - slope runtime seam,
  - expanded soil runtime seam,
  - alias completion,
  - integration closure across hillslope runtime consumers.
- Climate-runtime integration remains paused until these ownership and seam closures are complete enough to prevent mis-coupling.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/xinflo.for:147`
- `/workdir/wepp-forest_260430_baseline/src/route.for:183`
- `/workdir/wepp-forest_260430_baseline/src/wshinp.for:374`
- `/workdir/wepp-forest_260430_baseline/src/input.for:688`
- `/workdir/wepp-forest_260430_baseline/src/input.for:911`
- `/workdir/wepp-forest_260430_baseline/src/soil.for:179`
- `/workdir/wepp-forest_260430_baseline/src/infpar.for:152`
- `/workdir/wepp-forest_260430_baseline/src/perc.for:101`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:388`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:209`
