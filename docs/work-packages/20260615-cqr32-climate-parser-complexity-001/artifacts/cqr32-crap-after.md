# CQR32 CRAP After

Ran:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr32-climate-parser-complexity-001/artifacts/lcov_after.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr32-climate-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr32-climate-parser-complexity-001/artifacts/crap_after.json
```

Result: exit `0`.

Warning: `cargo crap` reported the established `126` source files with no
matching LCOV entry.

## Target Closure

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `ClimateParseError::fmt` | 223 | 1.0 | 100.0 | 1.0 |
| `ClimateParseError::write_display` | 229 | 15.0 | 100.0 | 15.0 |

Conclusion: the scoped target and the newly extracted helper are both CRAP
`<= 30`.

## Same-File Rows Above CRAP 30 After

| Function | CRAP | Disposition |
|---|---:|---|
| `parse_climate_from_str` | 70.29345120154446 | Out of scope for CQR32. |
| `parse_breakpoint_day` | 37.21131672584451 | Out of scope for CQR32. |
| `parse_no_breakpoint_day` | 33.84660901228949 | Out of scope for CQR32. |
