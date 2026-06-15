# Gate Results

Status: complete.

Ran:

| Gate | Result | Notes |
| --- | --- | --- |
| `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/lcov_before.info` | exit `0` | before LCOV |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/crap_before.json` | exit `0` | cargo-crap emitted existing unmatched-LCOV warning |
| `cargo test --test infile_management_parser_contract perennial -- --nocapture` | exit `0` | before production refactor, `9` passed |
| `cargo test --test infile_management_parser_contract perennial -- --nocapture` | exit `0` | after production refactor/formatting, `9` passed |
| `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/lcov_after.info` | exit `0` | after LCOV |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/crap_after.json` | exit `0` | cargo-crap emitted existing unmatched-LCOV warning |
| `cargo fmt --check` | exit `0` | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit `0` | passed |
| `cargo test --workspace` | exit `0` | passed |
| `cargo deny check` | exit `0` | `advisories ok, bans ok, licenses ok, sources ok` |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr11-management-parser-complexity-001 --format json` | exit `0` | passed after artifact finalization |
| `git diff --check` | exit `0` | passed |

Static: all current-scope gates have direct evidence and are not deferred.
