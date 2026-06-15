# CQR12 CRAP After

Status: complete.

Ran:

- `cargo crap --workspace --lcov docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/crap_after.json`

Static: after target-file CRAP rows, de-duplicated:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `seed_hillslope_runtime_surface_from_frost` | 866 | 18.0 | 86.11111111111111 | 18.868055555555554 |
| `seed_fixeddate_irrigation_furrow_event` | 663 | 11.0 | 70.1492537313433 | 14.218480996665143 |
| `seed_hillslope_runtime_surface_from_snow` | 755 | 11.0 | 78.68852459016394 | 12.171186134522273 |
| `seed_irrigation_depletion_furrow_period` | 302 | 9.0 | 94.20289855072464 | 9.015780389578367 |
| `seed_irrigation_depletion_header_symbols` | 68 | 8.0 | 83.92857142857143 | 8.26567055393586 |
| `seed_irrigation_depletion_sprinkler_period` | 242 | 7.0 | 94.23076923076923 | 7.009409137460173 |
| `seed_irrigation_depletion_period_header_symbols` | 163 | 4.0 | 95.45454545454545 | 4.001502629601803 |
| `seed_irrigation_depletion_period` | 147 | 4.0 | 100.0 | 4.0 |
| `seed_irrigation_depletion_trigger_symbol` | 189 | 3.0 | 95.45454545454545 | 3.000845229151014 |
| `irrigation_depletion_system_type_value` | 61 | 3.0 | 100.0 | 3.0 |
| `seed_irrigation_depletion_date_symbols` | 214 | 3.0 | 100.0 | 3.0 |
| `seed_irrigation_depletion_periods` | 137 | 3.0 | 100.0 | 3.0 |
| `seed_hillslope_runtime_surface_from_irrigation_depletion` | 51 | 2.0 | 100.0 | 2.0 |

Closure: the scoped target and every newly extracted depletion helper are CRAP
`<= 30`. The maximum new-helper CRAP is
`seed_irrigation_depletion_furrow_period` at `9.015780389578367`.
