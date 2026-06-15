# CQR11 CRAP After

Status: complete-with-warnings.

Static: target function after refactor remains `parse_yearly_perennial`; all
new production helpers are private to `management.rs`.

Ran:

```console
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/lcov_after.info
```

Result: exit `0`.

Ran:

```console
cargo crap --workspace --lcov docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/crap_after.json
```

Result: exit `0` with the existing cargo-crap warning that `125` source files
had no matching LCOV entry.

After LCOV target-file summary: `749/1114` lines covered and `40/49` functions
covered.

The cargo-crap workspace report emits duplicate identical rows for some
workspace entries; the table below records unique target/helper rows.

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `parse_yearly_perennial` | 1191 | 4.0 | 100.0 | 4.0 |
| `parse_yearly_perennial_header` | 1225 | 6.0 | 100.0 | 6.0 |
| `validate_yearly_perennial_mgtopt` | 1243 | 6.0 | 100.0 | 6.0 |
| `parse_yearly_perennial_payload` | 1265 | 8.0 | 95.0 | 8.008000000000001 |
| `parse_yearly_perennial_cut_days` | 1291 | 5.0 | 100.0 | 5.0 |
| `parse_yearly_perennial_cut_day` | 1308 | 6.0 | 86.66666666666667 | 6.085333333333334 |
| `parse_yearly_perennial_grazing_cycles` | 1325 | 5.0 | 100.0 | 5.0 |
| `parse_yearly_perennial_grazing_cycle` | 1342 | 9.0 | 100.0 | 9.0 |

Closure: the target and every new helper are CRAP `<= 30`.

WARN: pre-existing out-of-scope target-file rows remain above `30`, led by
`parse_yearly_annual_fallow`, `parse_operation_section`,
`ManagementParseError::fmt`, `parse_contour_section`,
`parse_management_from_str`, and `parse_initial_section`.
