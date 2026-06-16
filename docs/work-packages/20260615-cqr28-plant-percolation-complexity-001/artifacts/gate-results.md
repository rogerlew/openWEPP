# Gate Results

Status: complete-with-warnings.

Ran:

| Gate | Result |
|---|---|
| `cargo fmt --check` | Passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --workspace` | Passed |
| `cargo deny check` | Passed |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov` before/after | Passed |
| `cargo crap --workspace --lcov ... --min 0 --format json` before/after | Passed with repeated 126 source-map warnings |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr28-plant-percolation-complexity-001 --format json` | Passed: 22 files scanned, 0 errors, 0 warnings |
| `git diff --check` | Passed |
