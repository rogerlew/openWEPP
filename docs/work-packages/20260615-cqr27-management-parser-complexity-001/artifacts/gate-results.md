# Gate Results

Status: complete.

Ran: metric gates:

| Command | Result |
| --- | --- |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/lcov_before.info` | pass |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/crap_before.json` | pass with LCOV source-map warnings |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/lcov_after.info` | pass |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr27-management-parser-complexity-001/artifacts/crap_after.json` | pass with LCOV source-map warnings |

Ran: focused pre/post characterization gates:

| Command | Result |
| --- | --- |
| `cargo test --test infile_management_parser_contract` before production refactor | pass, `30` passed |
| `cargo test --test infile_management_parser_contract` after production refactor | pass, `30` passed |

Ran: required cargo gates:

| Command | Result |
| --- | --- |
| `cargo fmt --check` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo test --workspace` | pass |
| `cargo deny check` | pass |

Ran: final post-artifact gates:

| Command | Result |
| --- | --- |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr27-management-parser-complexity-001 --format json` | pass, `files_scanned=22`, `errors=0`, `warnings=0` |
| `git diff --check` | pass |
