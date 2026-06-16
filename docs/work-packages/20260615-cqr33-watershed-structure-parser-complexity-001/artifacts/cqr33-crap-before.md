# CQR33 CRAP Before

Ran:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/lcov_before.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/crap_before.json
```

Result: exit `0`.

Warn: `cargo crap` reported the established `126` LCOV source-map warnings.

## Target Rows

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `WatershedStructureParseError::fmt` | 224 | 15.0 | 0.0 | 240.0 |
| `WatershedStructureParseError::contract_error_id` | 205 | 13.0 | 93.33333333333333 | 13.050074074074073 |
| `WatershedStructureParseError::source` | 320 | 3.0 | 0.0 | 12.0 |

## Same-File Rows Above CRAP 30

| Function | CRAP | Scope |
|---|---:|---|
| `WatershedStructureParseError::fmt` | 240.0 | In scope for CQR33. |
| `parse_watershed_structure_from_str` | 43.387638175639935 | Out of scope; parser control-flow refactor not authorized by CQR33. |
