# CQR27 CRAP After

Status: complete.

Ran: after metric capture:

| Command | Result |
| --- | --- |
| `cargo llvm-cov clean --workspace` | pass |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/lcov_after.info` | pass |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/crap_after.json` | pass with LCOV source-map warnings |

Ran: target file LCOV after:

| File | Lines | Functions |
| --- | ---: | ---: |
| `crates/openwepp-input-contract/src/parsers/management.rs` | `816/1147` (`71.14%`) | `45/54` (`83.33%`) |

Ran: CQR27 target and extracted helpers after refactor:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `parse_yearly_annual_fallow` | `1113` | `4.0` | `100.0%` | `4.0` |
| `parse_yearly_annual_fallow_header` | `1138` | `5.0` | `100.0%` | `5.0` |
| `parse_yearly_annual_extension` | `1172` | `19.0` | `95.0%` | `19.045125` |
| `parse_yearly_annual_cut_records` | `1206` | `6.0` | `100.0%` | `6.0` |
| `parse_yearly_annual_cut_entry` | `1221` | `6.0` | `100.0%` | `6.0` |

Ran: CQR27 closure threshold is satisfied. The target and all newly extracted
helpers are CRAP `<= 30`.

Ran: target-file rows over CRAP `30` after refactor are all non-target rows:

| Function | CRAP |
| --- | ---: |
| `parse_operation_section` | `202.41794168163776` |
| `ManagementParseError::fmt` | `182.0` |
| `parse_contour_section` | `162.47732020392675` |
| `parse_management_from_str` | `53.99497965384546` |
| `parse_initial_section` | `43.59771428571429` |
| `parse_plant_section` | `30.000000000000018` |

Warnings: `cargo crap` emitted LCOV source-map warnings for 126 workspace
test/support source files. The target file was present in LCOV.
