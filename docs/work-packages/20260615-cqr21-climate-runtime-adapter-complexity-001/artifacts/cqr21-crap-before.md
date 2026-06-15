# CQR21 CRAP Before

Status: complete.

Ran: before CRAP command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr21-climate-runtime-adapter-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr21-climate-runtime-adapter-complexity-001/artifacts/crap_before.json
```

Ran: ranked before rows for
`crates/openwepp-climate-runtime-adapter/src/lib.rs`:

```text
SharedClimateRuntimeInputError::fmt     line 195  CC 19.0  coverage 0.0                 CRAP 380.0
SharedClimateRuntimeInputError::code    line 166  CC 19.0  coverage 47.61904761904761   CRAP 70.88327394449847
adapt_breakpoint                        line 449  CC 22.0  coverage 96.29629629629629   CRAP 22.024589747497842
adapt_no_breakpoint                     line 396  CC 15.0  coverage 93.61702127659575   CRAP 15.058513046242162
solve_eqroot                            line 735  CC 12.0  coverage 77.14285714285715   CRAP 13.71960349854227
build_no_breakpoint_event_shape         line 571  CC 11.0  coverage 81.42857142857143   CRAP 11.77503498542274
build_disaggregation_shape              line 649  CC 7.0   coverage 95.23809523809523   CRAP 7.005291005291006
build_climate_runtime_request           line 338  CC 6.0   coverage 96.0                CRAP 6.002304
build_dblex_shape                       line 694  CC 6.0   coverage 96.7741935483871    CRAP 6.001208418649928
build_const_shape                       line 679  CC 2.0   coverage 0.0                 CRAP 6.0
adapt_daily_forcing                     line 382  CC 5.0   coverage 100.0               CRAP 5.0
resolve_iclig                           line 801  CC 4.0   coverage 100.0               CRAP 4.0
require_non_negative                    line 789  CC 3.0   coverage 88.88888888888889   CRAP 3.0123456790123457
require_finite                          line 781  CC 2.0   coverage 80.0                CRAP 2.032
select_day_forcing                      line 369  CC 1.0   coverage 100.0               CRAP 1.0
```

Ran: target-file before LCOV:

```text
lines 507/657 77.17%
functions 23/25 92.00%
```

Static: `cargo crap` emitted the repo-wide warning that 126 source files had no
matching LCOV entries. The target file had matching LCOV entries, matching prior
CQR package evidence posture.
