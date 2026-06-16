# CQR36 CRAP Before

Status: complete.

Ran:
`cargo llvm-cov clean --workspace`

Ran:
`cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/lcov_before.info`

Ran:
`cargo crap --workspace --lcov docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/crap_before.json`

Ran: target-file LCOV for
`crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`:

| Metric | Covered | Total | Percent |
| --- | ---: | ---: | ---: |
| Lines | 624 | 892 | 69.955156950673% |
| Functions | 23 | 30 | 76.666666666667% |

Ran: highest unique target-file CRAP rows:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `parse_impoundment` | 655 | 73.0 | 69.81132075471697% | 219.61488342725883 |
| `WatershedImpoundmentParseError::fmt` | 387 | 12.0 | 0.0% | 156.0 |
| `parse_watershed_impoundment_from_str` | 502 | 23.0 | 85.8267716535433% | 24.50612849257195 |
| `parse_i32` | 1292 | 5.0 | 30.303030303030305% | 13.464117761638425 |
| `ImpWarningCode::as_str` | 65 | 3.0 | 0.0% | 12.0 |
| `WatershedImpoundmentParseError::source` | 479 | 3.0 | 0.0% | 12.0 |
| `parse_culvert` | 1063 | 11.0 | 90.47619047619048% | 11.10452434942231 |
| `WatershedImpoundmentParseError::contract_error_id` | 370 | 11.0 | 92.3076923076923% | 11.055075102412381 |

Ran: unique target-file CRAP rows over `30`: `2`.

Warning: `cargo crap` reported 126 source files with no matching LCOV entry.
The target file was represented in LCOV.
