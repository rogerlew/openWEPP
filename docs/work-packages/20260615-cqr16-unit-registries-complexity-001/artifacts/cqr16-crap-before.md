# CQR16 CRAP Before

Status: complete.

Ran:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace \
  --ignore-run-fail --lcov \
  --output-path docs/work-packages/20260615-cqr16-unit-registries-complexity-001/artifacts/lcov_before.info
```

Ran:

```text
cargo crap --workspace \
  --lcov docs/work-packages/20260615-cqr16-unit-registries-complexity-001/artifacts/lcov_before.info \
  --min 0 --format json \
  --output docs/work-packages/20260615-cqr16-unit-registries-complexity-001/artifacts/crap_before.json
```

Static: `cargo crap` emitted the recurring no-matching-LCOV source-file warning
seen in earlier CQR packages, and wrote `crap_before.json`.

Before target-file coverage:

| Metric | Value |
| --- | --- |
| Lines | `319/593 53.79%` |
| Functions | `20/27 74.07%` |

Before ranked rows in target file:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `BoundaryUnitRegistryError::fmt` | `272` | `22.0` | `0.0` | `506.0` |
| `OutputUnitRegistryError::fmt` | `71` | `13.0` | `0.0` | `182.0` |
| `validate_entry` | `724` | `19.0` | `50.617283950617285` | `62.4742520806637` |
| `BoundaryUnitRegistry::new` | `443` | `17.0` | `80.68181818181817` | `19.08351480324944` |

Target identity: `BoundaryUnitRegistryError::fmt`.
