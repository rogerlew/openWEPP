# CQR32 CRAP Before

Ran:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr32-climate-parser-complexity-001/artifacts/lcov_before.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr32-climate-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr32-climate-parser-complexity-001/artifacts/crap_before.json
```

Result: exit `0`.

Warning: `cargo crap` reported the established `126` source files with no
matching LCOV entry.

## Target Identity

| Function | Line | Cyclomatic | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `ClimateParseError::fmt` | 223 | 15.0 | 0.0 | 240.0 |

The workspace report emits duplicate identical rows for the same function; the
table records the unique target identity.

## Same-File Rows Above CRAP 30 Before

| Function | CRAP | Disposition |
|---|---:|---|
| `ClimateParseError::fmt` | 240.0 | In scope for CQR32. |
| `parse_climate_from_str` | 70.29345120154446 | Out of scope. |
| `parse_breakpoint_day` | 37.21131672584451 | Out of scope. |
| `parse_no_breakpoint_day` | 33.84660901228949 | Out of scope. |
