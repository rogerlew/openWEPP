# REFINTENT001 Non-Aliased Tests

Evidence class: Static + Ran

## Unit fixtures

Added in `02_ksat_adjustment.rs:635-676`:

1. `wb14_ksatadj_sat_frac_uses_source_intent_avsat_not_ul_surrogate`
   - source-intent expected value: `0.41 / 0.55`
   - old surrogate value: `0.06 / 0.40`
   - asserts source-intent equality at `1.0e-12`
   - asserts the result is not accidentally equal to `theta_sum/ul_sum`
   - checks direct `avthetafc = 0.425` and `avthetadr = 0.11`
2. `wb14_ksatadj_missing_source_intent_operand_is_typed_failure`
   - removes `cpm_0002`
   - expects `MissingRequiredStateSymbol { symbol: cpm_0002 }`

## Integration oracle

`wb14_infiltration_hyetograph_kernel_contract.rs:231-262` now recomputes the
expected value with the source-intent operands:

- top-two `dg`
- `por`
- `cpm`
- `thetdr`
- both `avsat` caps
- direct `thetfc` / `thetdr` theta metrics

## Ran evidence

- `cargo test -p openwepp-hillslope-orchestrator wb14_ksatadj -- --nocapture`
  passed: 2 tests.
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture`
  passed: 16 tests.

The fixture is non-degenerate: the old surrogate and intended formula differ by
about `0.595454545`, so a regression back to `theta_sum/ul_sum` would fail.
