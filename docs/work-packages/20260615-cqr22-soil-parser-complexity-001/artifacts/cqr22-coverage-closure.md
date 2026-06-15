# CQR22 Coverage Closure

Status: complete.

Ran: before LCOV:

```bash
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr22-soil-parser-complexity-001/artifacts/lcov_before.info
```

Ran: after LCOV:

```bash
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr22-soil-parser-complexity-001/artifacts/lcov_after.info
```

Ran: target-file coverage changed as follows:

```text
before lines     665/1023 65.00%
after lines      847/1124 75.36%
before functions 35/42 83.33%
after functions  45/52 86.54%
```

Static: coverage increased for the scoped target file. Added characterization
covers DATVER-specific policy row success variants, stable typed error codes,
fields, and messages for selected policy-row failure branches.
