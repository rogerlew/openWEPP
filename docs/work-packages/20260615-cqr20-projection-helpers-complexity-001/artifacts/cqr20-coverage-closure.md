# CQR20 Coverage Closure

Status: complete.

Ran: before LCOV:

```bash
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr20-projection-helpers-complexity-001/artifacts/lcov_before.info
```

Ran: after LCOV:

```bash
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr20-projection-helpers-complexity-001/artifacts/lcov_after.info
```

Ran: target-file coverage changed as follows:

```text
before lines     599/796 75.25%
after lines      754/842 89.55%
before functions 44/48 91.67%
after functions  52/54 96.30%
```

Static: coverage increased for the scoped target file. Added characterization
covers every annual extension branch, mismatch label path, unsupported annual
management option path, and representative projection day/fraction domain
errors.
