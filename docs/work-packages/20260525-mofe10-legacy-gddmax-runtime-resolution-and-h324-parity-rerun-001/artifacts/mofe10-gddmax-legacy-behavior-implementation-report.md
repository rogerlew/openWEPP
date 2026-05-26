# MOFE10 Legacy `gddmax` Behavior Implementation Report

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Legacy-compatible `gddmax` resolution implemented in
  `crates/openwepp-hillslope-orchestrator/src/hydrology.rs`:
  - Added runtime resolver for `gddmax==0` sentinel.
  - Branch behavior mirrors legacy `yldopt/gdmax` semantics for annual summer,
    annual winter (cross-year), and perennial windows.
  - Resolver uses runtime monthly climate vectors (`obmaxt_*`, `obmint_*`)
    and management day controls.
  - Resolver preserves typed fail-closed posture for missing vectors,
    non-finite intermediate values, and non-positive resolved totals.
- Management runtime seam domain guard updated in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs` to allow
  non-negative sentinel intake (`gddmax>=0`) while preserving strict typed
  rejection for negative values.
- Monthly climate vectors are now projected into hillslope runtime surfaces in
  canonical symbol form:
  - `obmaxt_0001..0012`, `obmint_0001..0012`, `radave_0001..0012`,
    `obrain_0001..0012`.
- Watershed climate adapter parity fix implemented in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` to project
  prefixed monthly vectors (`hs{n}_obmaxt_*`, `hs{n}_obmint_*`, etc.) and keep
  seam parity tests green after monthly symbol introduction.

Source references:
- `/workdir/wepp-forest_260430_baseline/src/yldopt.for:121-200,271-277`
- `/workdir/wepp-forest_260430_baseline/src/gdmax.for:1-130`

Ran:
- Targeted sentinel tests and parser/runtime seam tests pass post-implementation.
