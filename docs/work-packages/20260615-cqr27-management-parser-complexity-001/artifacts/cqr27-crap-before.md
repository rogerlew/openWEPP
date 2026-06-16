# CQR27 CRAP Before

Status: complete.

Ran: before metric capture:

| Command | Result |
| --- | --- |
| `cargo llvm-cov clean --workspace` | pass |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/lcov_before.info` | pass |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/crap_before.json` | pass with LCOV source-map warnings |

Ran: target file LCOV before:

| File | Lines | Functions |
| --- | ---: | ---: |
| `crates/openwepp-input-contract/src/parsers/management.rs` | `749/1114` (`67.24%`) | `40/49` (`81.63%`) |

Ran: live CQR27 target before refactor:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `parse_yearly_annual_fallow` | `1113` | `35.0` | `40.67796610169492%` | `290.7314769280208` |

Ran: target-file rows over CRAP `30` before, as emitted by `cargo crap`.
Rows are duplicated in the JSON output by crate/test context:

| Function | CRAP |
| --- | ---: |
| `parse_yearly_annual_fallow` | `290.7314769280208` |
| `parse_operation_section` | `202.41794168163776` |
| `ManagementParseError::fmt` | `182.0` |
| `parse_contour_section` | `162.47732020392675` |
| `parse_management_from_str` | `53.99497965384546` |
| `parse_initial_section` | `43.59771428571429` |
| `parse_plant_section` | `30.000000000000018` |

Static: CQR27 scope is limited to `parse_yearly_annual_fallow`; other rows are
baseline work for later ranked CQR packages unless a row is already live-metric
closed when reached.
