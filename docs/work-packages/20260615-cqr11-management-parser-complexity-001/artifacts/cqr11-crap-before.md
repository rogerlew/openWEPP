# CQR11 CRAP Before

Status: complete.

Static: target file was
`crates/openwepp-input-contract/src/parsers/management.rs`.

Static: before target-file line count was `1592`.

Static: before suppression census found the existing crate-level
`#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]` at line `1`.
No function-local CQR11 suppression existed.

Ran:

```console
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/lcov_before.info
```

Result: exit `0`.

Ran:

```console
cargo crap --workspace --lcov docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/crap_before.json
```

Result: exit `0` with the existing cargo-crap warning that `125` source files
had no matching LCOV entry.

Before LCOV target-file summary: `608/1068` lines covered and `32/42`
functions covered.

Before CRAP target identity:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `parse_yearly_perennial` | 1191 | 37.0 | 0.0 | 1406.0 |

Highest target-file CRAP rows before:

| Function | CRAP | Disposition |
| --- | ---: | --- |
| `parse_yearly_perennial` | 1406.0 | CQR11 target |
| `parse_yearly_annual_fallow` | 290.7314769280208 | out of scope |
| `parse_operation_section` | 202.41794168163776 | out of scope |
| `ManagementParseError::fmt` | 182.0 | out of scope |
| `parse_contour_section` | 162.47732020392675 | out of scope |
| `parse_management_from_str` | 53.99497965384546 | out of scope |
| `parse_initial_section` | 43.59771428571429 | out of scope |
| `parse_plant_section` | 30.000000000000018 | out of scope |
