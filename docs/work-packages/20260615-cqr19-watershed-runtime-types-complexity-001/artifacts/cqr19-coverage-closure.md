# CQR19 Coverage Closure

Status: complete.

Ran: before LCOV command:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001/artifacts/lcov_before.info
```

Ran: before target-file coverage from `lcov_before.info`:

```text
lines 39/221 17.65%
functions 7/9 77.78%
```

Ran: after LCOV command:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001/artifacts/lcov_after.info
```

Ran: after target-file coverage from `lcov_after.info`:

```text
lines 249/256 97.27%
functions 16/16 100.00%
```

Static: line and function coverage improved for the target file. The targeted
characterization tests exercise every watershed and climate runtime error
`code()` and `Display` branch.

Static: `cargo crap` emitted the expected repository-wide warning about source
files without matching LCOV entries for integration/support files in both before
and after runs. The target file had matching LCOV entries in both runs.
