# CQR36 CRAP After

Status: complete.

Ran:
`cargo llvm-cov clean --workspace`

Ran:
`cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/lcov_after.info`

Ran:
`cargo crap --workspace --lcov docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/crap_after.json`

Ran: target-file LCOV for
`crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`:

| Metric | Covered | Total | Percent |
| --- | ---: | ---: | ---: |
| Lines | 877 | 998 | 87.875751503006% |
| Functions | 37 | 42 | 88.095238095238% |

Ran: highest unique target-file CRAP rows:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `parse_watershed_impoundment_from_str` | 502 | 23.0 | 85.8267716535433% | 24.50612849257195 |
| `parse_impoundment_curve_fields` | 1157 | 13.0 | 69.35483870967742% | 17.863763384914904 |
| `parse_impoundment` | 701 | 15.0 | 100.0% | 15.0 |
| `parse_i32` | 1458 | 5.0 | 30.303030303030305% | 13.464117761638425 |
| `ImpWarningCode::as_str` | 65 | 3.0 | 0.0% | 12.0 |
| `WatershedImpoundmentParseError::fmt` | 387 | 12.0 | 100.0% | 12.0 |
| `parse_culvert` | 1229 | 11.0 | 90.47619047619048% | 11.10452434942231 |
| `WatershedImpoundmentParseError::contract_error_id` | 370 | 11.0 | 100.0% | 11.0 |
| `parse_drop_spillway` | 789 | 10.0 | 95.45454545454545% | 10.00939143501127 |
| `parse_emergency_spillway` | 933 | 9.0 | 92.85714285714286% | 9.029518950437318 |
| `parse_perforated_riser` | 1062 | 9.0 | 100.0% | 9.0 |
| `parse_emergency_rating_curve` | 982 | 8.0 | 76.92307692307693% | 8.786527082385069 |

Ran: unique target-file CRAP rows over `30`: `0`.

Warning: `cargo crap` reported 126 source files with no matching LCOV entry.
The target file was represented in LCOV.
