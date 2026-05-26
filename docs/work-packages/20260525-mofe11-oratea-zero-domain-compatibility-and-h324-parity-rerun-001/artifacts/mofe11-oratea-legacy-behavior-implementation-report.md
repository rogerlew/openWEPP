# MOFE11 Legacy `oratea/orater` Behavior Implementation Report

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Management runtime projection now accepts non-negative decomposition
  constants (`>=0`) in:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - `decomposition_equation_parameter_values(...)` now uses
    `validate_projection_non_negative(...)` for `oratea` and `orater`.
- Decomposition transition input guard now accepts non-negative decomposition
  constants (`>=0`) in:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology.rs`
  - `require_decomposition_equation_inputs(...)` changed from
    `minimum = f64::EPSILON` + `"must be positive"` to
    `minimum = 0.0` + `"must be non-negative"`.
- Typed error posture is preserved:
  - Negative values remain `HS-RUNTIME-E-050` / `HS-DECOMP-E-010` class
    domain violations.
  - Non-finite values continue to fail typed finite checks.

Legacy behavior provenance:
- `/workdir/wepp-forest_260430_baseline/src/infile.for`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for`
- Zero decomposition constants map to no-decay contribution in
  `exp(-ENVIND * ORate*)` terms.

Ran:
- Contract-derived zero and negative guard tests pass post-implementation.
