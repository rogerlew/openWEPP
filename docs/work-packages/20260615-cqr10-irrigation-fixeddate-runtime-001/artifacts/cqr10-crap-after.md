# CQR10 CRAP After

Status: complete-with-warnings.

Ran: after CRAP was generated with:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/crap_after.json
```

Ran: command exited `0` with the known `125 source files had no matching entry
in the LCOV report` warning.

Target-file rows after refactor:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `seed_hillslope_runtime_surface_from_irrigation_depletion` | 52 | 33.0 | 0.0 | 1122.0 |
| `seed_hillslope_runtime_surface_from_frost` | 802 | 18.0 | 86.11111111111111 | 18.868055555555554 |
| `seed_fixeddate_irrigation_furrow_event` | 599 | 11.0 | 70.1492537313433 | 14.218480996665143 |
| `seed_hillslope_runtime_surface_from_snow` | 691 | 11.0 | 78.68852459016394 | 12.171186134522273 |
| `seed_fixeddate_irrigation_sprinkler_event` | 539 | 7.0 | 67.3076923076923 | 8.712114531178882 |
| `seed_fixeddate_irrigation_event_schedule` | 489 | 7.0 | 83.78378378378379 | 7.208951098651609 |
| `validate_fixeddate_irrigation_header` | 376 | 6.0 | 77.77777777777779 | 6.395061728395061 |
| `build_hillslope_runtime_surface_from_irrigation_depletion` | 36 | 2.0 | 0.0 | 6.0 |
| `seed_fixeddate_irrigation_header_symbols` | 414 | 5.0 | 87.87878787878788 | 5.044522358571945 |
| `seed_hillslope_runtime_surface_from_irrigation_fixeddate` | 341 | 4.0 | 100.0 | 4.0 |
| `seed_fixeddate_irrigation_event` | 472 | 4.0 | 100.0 | 4.0 |
| `seed_fixeddate_irrigation_events` | 458 | 3.0 | 100.0 | 3.0 |
| `fixeddate_event_next_record` | 677 | 3.0 | 100.0 | 3.0 |
| `build_hillslope_runtime_surface_from_snow` | 7 | 2.0 | 100.0 | 2.0 |
| `build_hillslope_runtime_surface_from_frost` | 21 | 2.0 | 100.0 | 2.0 |
| `build_hillslope_runtime_surface_from_irrigation_fixeddate` | 326 | 2.0 | 100.0 | 2.0 |
| `FixedDateProjectionState::advance` | 367 | 2.0 | 100.0 | 2.0 |
| `snow_runtime_boundary_value` | 773 | 1.0 | 42.10526315789473 | 1.194051611022015 |
| `FixedDateProjectionState::new` | 360 | 1.0 | 100.0 | 1.0 |

Closure:

- PASS: scoped target
  `seed_hillslope_runtime_surface_from_irrigation_fixeddate` is CRAP `4.0`.
- PASS: every newly extracted fixed-date helper is CRAP `<= 14.218480996665143`.
- WARN: `seed_hillslope_runtime_surface_from_irrigation_depletion` remains
  CRAP `1122.0`; this row is pre-existing and out of CQR10 scope.
