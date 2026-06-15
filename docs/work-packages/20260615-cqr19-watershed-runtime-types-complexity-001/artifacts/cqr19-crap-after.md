# CQR19 CRAP After

Status: complete.

Ran: after CRAP command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001/artifacts/crap_after.json
```

Ran: ranked after rows for
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`:

```text
WatershedClimateRuntimeInputError::code                line 332  CC 19.0  coverage 100.0              CRAP 19.0
WatershedRuntimeInputError::code                       line 59   CC 13.0  coverage 100.0              CRAP 13.0
WatershedRuntimeInputError::fmt_basic                  line 76   CC 10.0  coverage 96.66666666666667  CRAP 10.003703703703703
WatershedClimateRuntimeInputError::fmt_daily_record    line 359  CC 9.0   coverage 97.36842105263158  CRAP 9.001476162705934
WatershedClimateRuntimeInputError::fmt_disaggregation  line 467  CC 6.0   coverage 96.15384615384616  CRAP 6.002048247610378
WatershedClimateRuntimeInputError::fmt_breakpoint      line 421  CC 6.0   coverage 96.42857142857143  CRAP 6.0016399416909625
WatershedClimateRuntimeInputError::fmt                 line 557  CC 6.0   coverage 100.0              CRAP 6.0
WatershedClimateRuntimeInputError::fmt_runtime_context line 511  CC 5.0   coverage 96.15384615384616  CRAP 5.001422394173874
WatershedRuntimeInputError::fmt_channel                line 120  CC 4.0   coverage 93.33333333333333  CRAP 4.004740740740741
WatershedRuntimeInputError::fmt_impoundment            line 145  CC 4.0   coverage 93.33333333333333  CRAP 4.004740740740741
WatershedRuntimeInputError::fmt                        line 172  CC 4.0   coverage 100.0              CRAP 4.0
```

Static: final target `WatershedClimateRuntimeInputError::fmt` is CRAP `6.0`.
All newly extracted helpers and remaining target-file rows are CRAP `<= 30`.
