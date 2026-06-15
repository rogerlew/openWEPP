# CQR21 Coverage Closure

Status: complete.

Ran: before LCOV:

```bash
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr21-climate-runtime-adapter-complexity-001/artifacts/lcov_before.info
```

Ran: after LCOV:

```bash
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr21-climate-runtime-adapter-complexity-001/artifacts/lcov_after.info
```

Ran: target-file coverage changed as follows:

```text
before lines     507/657 77.17%
after lines      713/765 93.20%
before functions 23/25 92.00%
after functions  26/27 96.30%
```

Static: coverage increased for the scoped target file. Added characterization
covers every `SharedClimateRuntimeInputError` variant's stable error code and
display string.
