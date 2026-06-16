# CQR33 Gate Results

## Metrics and Focused Gates

| Command | Result | Notes |
|---|---|---|
| `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/lcov_before.info` | PASS | Before LCOV. |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/crap_before.json` | PASS | Before CRAP; 126 LCOV source-map warnings. |
| `cargo test --test infile_watershed_structure_parser_contract --no-fail-fast` | PASS | `20` passed. |
| `cargo clippy -p openwepp-input-contract --all-targets -- -D warnings` | PASS | Focused clippy. |
| `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/lcov_after.info` | PASS | After LCOV. |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001/artifacts/crap_after.json` | PASS | After CRAP; 126 LCOV source-map warnings. |

## Required Closure Gates

| Command | Result | Notes |
|---|---|---|
| `cargo fmt --check` | PASS | Initial formatting diff was fixed with `cargo fmt`; final check passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final workspace clippy. |
| `cargo test --workspace` | PASS | Full workspace tests and doctests passed. |
| `cargo deny check` | PASS | advisories ok, bans ok, licenses ok, sources ok. |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr33-watershed-structure-parser-complexity-001 --format json` | PASS | Scanned 21 files; 0 errors, 0 warnings. |
| `git diff --check` | PASS | No whitespace errors. |
