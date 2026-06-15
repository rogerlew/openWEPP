# CQR21 CRAP After

Status: complete.

Ran: after CRAP command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr21-climate-runtime-adapter-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr21-climate-runtime-adapter-complexity-001/artifacts/crap_after.json
```

Ran: ranked after rows for
`crates/openwepp-climate-runtime-adapter/src/lib.rs`:

```text
adapt_breakpoint                             line 415  CC 22.0  coverage 96.29629629629629   CRAP 22.024589747497842
SharedClimateRuntimeInputError::code         line 166  CC 19.0  coverage 100.0               CRAP 19.0
SharedClimateRuntimeInputError::fmt_message  line 201  CC 19.0  coverage 100.0               CRAP 19.0
adapt_no_breakpoint                          line 362  CC 15.0  coverage 93.61702127659575   CRAP 15.058513046242162
solve_eqroot                                 line 701  CC 12.0  coverage 77.14285714285715   CRAP 13.71960349854227
build_no_breakpoint_event_shape              line 537  CC 11.0  coverage 81.42857142857143   CRAP 11.77503498542274
build_disaggregation_shape                   line 615  CC 7.0   coverage 95.23809523809523   CRAP 7.005291005291006
build_climate_runtime_request                line 304  CC 6.0   coverage 96.0                CRAP 6.002304
build_dblex_shape                            line 660  CC 6.0   coverage 96.7741935483871    CRAP 6.001208418649928
build_const_shape                            line 645  CC 2.0   coverage 0.0                 CRAP 6.0
adapt_daily_forcing                          line 348  CC 5.0   coverage 100.0               CRAP 5.0
resolve_iclig                                line 767  CC 4.0   coverage 100.0               CRAP 4.0
require_non_negative                         line 755  CC 3.0   coverage 88.88888888888889   CRAP 3.0123456790123457
require_finite                               line 747  CC 2.0   coverage 80.0                CRAP 2.032
SharedClimateRuntimeInputError::fmt          line 194  CC 2.0   coverage 100.0               CRAP 2.0
select_day_forcing                           line 335  CC 1.0   coverage 100.0               CRAP 1.0
```

Static: final scoped target `SharedClimateRuntimeInputError::fmt` CRAP is
`2.0`. Newly extracted `fmt_message` CRAP is `19.0`, below the `30` threshold.
The related `code` branch table is now fully covered with CRAP `19.0`.

Static: `cargo crap` emitted duplicate rows for some functions and the same
repo-wide LCOV warning pattern seen before; the duplicated target rows reported
identical CRAP values.
