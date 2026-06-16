# Verification Agent B

Ran:

```text
cargo crap --workspace --lcov docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/crap_after.json
```

Result: target and helpers closed under CRAP `30`; final target
`Wb11HydrologyKernelGuardError::fmt` CRAP is `1.0`.

Warn: `cargo crap` emitted 126 LCOV source-map warnings.
