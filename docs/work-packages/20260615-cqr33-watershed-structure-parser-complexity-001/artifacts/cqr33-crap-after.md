# CQR33 CRAP After

Ran:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/lcov_after.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/crap_after.json
```

Result: exit `0`.

Warn: `cargo crap` reported the established `126` LCOV source-map warnings.

## Target and Helper Rows

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `WatershedStructureParseError::fmt` | 224 | 1.0 | 100.0 | 1.0 |
| `WatershedStructureParseError::write_display` | 230 | 15.0 | 100.0 | 15.0 |
| `WatershedStructureParseError::contract_error_id` | 205 | 13.0 | 100.0 | 13.0 |
| `WatershedStructureParseError::source` | 326 | 3.0 | 100.0 | 3.0 |

Result: scoped target and extracted helper are both below CRAP `30`.

## Same-File Rows Above CRAP 30

| Function | CRAP | Scope |
|---|---:|---|
| `parse_watershed_structure_from_str` | 43.387638175639935 | Out of scope; unchanged parser control flow. |
