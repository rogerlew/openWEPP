# CQR13 Coverage Closure

Status: complete.

Ran:

- Before LCOV:
  `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr13-runtime-core-types-complexity-001/artifacts/lcov_before.info`
- After LCOV:
  `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr13-runtime-core-types-complexity-001/artifacts/lcov_after.info`

Static:

- target file: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
- before target-file line coverage: `497/515`
- before target-file function coverage: `20/20`
- after target-file line coverage: `497/515`
- after target-file function coverage: `20/20`

Result: coverage did not regress. Target-file line coverage is above the
ADR-0021 science-tier line threshold.
