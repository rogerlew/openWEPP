# CQR09 Coverage Closure

Ran: before LCOV was generated with:

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/lcov_before.info`

Ran: after LCOV was generated with:

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/lcov_after.info`

Static: target-file LCOV summary:

- before: `737/1285` lines, `23/25` functions.
- after: `999/1404` lines, `31/33` functions.

Static: line coverage improved from approximately `57.35%` to approximately
`71.15%`, so the target file did not regress.

Static: WARN remains because the target file is still below the science-tier
module closure threshold from
`docs/decisions/0021-module-coverage-closure-thresholds.md`. This package is a
ranked CRAP burn-down package, not full module coverage closure.
