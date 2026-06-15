# CQR16 Coverage Closure

Status: complete.

Before target-file coverage from `lcov_before.info`:

| Metric | Value |
| --- | --- |
| Lines | `319/593 53.79%` |
| Functions | `20/27 74.07%` |

After target-file coverage from `lcov_after.info`:

| Metric | Value |
| --- | --- |
| Lines | `505/625 80.80%` |
| Functions | `26/31 83.87%` |

Static: focused characterization increased coverage for
`BoundaryUnitRegistryError::fmt` from `0.0` to `100.0`.

Static: `OutputUnitRegistryError::fmt` also improved from CRAP `182.0` to
`13.0` through characterization coverage, without production changes to that
formatter.

WARN: target-file coverage remains below full ADR-0021 module closure
threshold. CQR16 scope was target/helper CRAP closure, and coverage improved.
