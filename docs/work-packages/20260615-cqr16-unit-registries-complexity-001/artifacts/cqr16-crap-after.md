# CQR16 CRAP After

Status: complete.

Ran:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace \
  --ignore-run-fail --lcov \
  --output-path docs/work-packages/20260615-cqr16-unit-registries-complexity-001/artifacts/lcov_after.info
```

Ran:

```text
cargo crap --workspace \
  --lcov docs/work-packages/20260615-cqr16-unit-registries-complexity-001/artifacts/lcov_after.info \
  --min 0 --format json \
  --output docs/work-packages/20260615-cqr16-unit-registries-complexity-001/artifacts/crap_after.json
```

Static: `cargo crap` emitted the recurring no-matching-LCOV source-file warning
seen in earlier CQR packages, and wrote `crap_after.json`.

After target-file coverage:

| Metric | Value |
| --- | --- |
| Lines | `505/625 80.80%` |
| Functions | `26/31 83.87%` |

Target and new helper closure:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `format_boundary_required_field_error` | `302` | `11.0` | `97.95918367346938` | `11.00102848303003` |
| `format_boundary_alias_conflict_error` | `370` | `7.0` | `96.7741935483871` | `7.001644792051291` |
| `BoundaryUnitRegistryError::fmt` | `271` | `6.0` | `100.0` | `6.0` |
| `format_boundary_lookup_error` | `445` | `5.0` | `93.33333333333333` | `5.007407407407407` |
| `format_boundary_unit_shape_error` | `412` | `5.0` | `96.0` | `5.0016` |

Static: CQR16 target and all newly extracted helpers are CRAP `<= 30`.

WARN: pre-existing out-of-scope `validate_entry` remains unchanged above CRAP
`30` at `62.4742520806637`.
