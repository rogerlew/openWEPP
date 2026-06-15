# CQR12 CRAP Before

Status: complete.

Ran:

- `cargo crap --workspace --lcov docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/crap_before.json`

Static: before target-file CRAP rows, de-duplicated:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `seed_hillslope_runtime_surface_from_irrigation_depletion` | 52 | 33.0 | 0.0 | 1122.0 |
| `seed_hillslope_runtime_surface_from_frost` | 802 | 18.0 | 86.11111111111111 | 18.868055555555554 |
| `seed_fixeddate_irrigation_furrow_event` | 599 | 11.0 | 70.1492537313433 | 14.218480996665143 |
| `seed_hillslope_runtime_surface_from_snow` | 691 | 11.0 | 78.68852459016394 | 12.171186134522273 |
| `seed_fixeddate_irrigation_sprinkler_event` | 539 | 7.0 | 67.3076923076923 | 8.712114531178882 |

Target identity: live CQR12 target was
`seed_hillslope_runtime_surface_from_irrigation_depletion` at line `52`, CC
`33.0`, coverage `0.0`, CRAP `1122.0`.
