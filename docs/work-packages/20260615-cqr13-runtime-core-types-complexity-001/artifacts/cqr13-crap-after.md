# CQR13 CRAP After

Status: complete.

Ran:

- `cargo crap --workspace --lcov docs/work-packages/20260615-cqr13-runtime-core-types-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr13-runtime-core-types-complexity-001/artifacts/crap_after.json`

Static: after target-file CRAP rows, de-duplicated:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `HillslopeRuntimeInputError::soil_core_code` | 393 | 14.0 | 93.75 | 14.0478515625 |
| `HillslopeRuntimeInputError::fmt_soil_core` | 507 | 14.0 | 97.72727272727273 | 14.002300901577762 |
| `HillslopeRuntimeInputError::pl_projection_code` | 475 | 12.0 | 92.85714285714286 | 12.052478134110787 |
| `HillslopeRuntimeInputError::slope_shape_code` | 436 | 10.0 | 91.66666666666666 | 10.05787037037037 |
| `HillslopeRuntimeInputError::snow_frost_irrigation_code` | 491 | 10.0 | 91.66666666666666 | 10.05787037037037 |
| `HillslopeRuntimeInputError::soil_layer_code` | 411 | 10.0 | 91.66666666666666 | 10.05787037037037 |
| `HillslopeRuntimeInputError::fmt_slope_shape` | 728 | 10.0 | 97.5 | 10.0015625 |
| `HillslopeRuntimeInputError::fmt_snow_frost_irrigation` | 1110 | 10.0 | 97.77777777777777 | 10.001097393689987 |
| `HillslopeRuntimeInputError::fmt_soil_layer` | 575 | 10.0 | 98.27586206896551 | 10.000512526138833 |
| `HillslopeRuntimeInputError::slope_numeric_code` | 450 | 9.0 | 90.9090909090909 | 9.060856498873028 |
| `HillslopeRuntimeInputError::fmt_slope_numeric` | 796 | 9.0 | 97.67441860465115 | 9.001018778220786 |
| `HillslopeRuntimeInputError::code` | 324 | 9.0 | 100.0 | 9.0 |
| `HillslopeRuntimeInputError::fmt` | 1188 | 9.0 | 100.0 | 9.0 |

Closure: every current target-file function is CRAP `<= 14.0478515625`, below
the CQR threshold `30`. No new helpers were extracted.
