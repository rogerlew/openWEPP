# Gate Results

Status: complete
Evidence mode: Ran

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | exit 0 |
| `git diff --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo test --workspace` | exit 0 |
| `cargo deny check` | exit 0 |
| `cargo llvm-cov --workspace --ignore-run-fail --no-report` | exit 0 |
| `cargo crap --workspace --lcov .../lcov_after.info --min 0 --format json --output .../crap_after.json` | exit 0 |

Focused checks:

| Gate | Result |
| --- | --- |
| `cargo test --test infile_hbp_parser_contract` | exit 0, 21 passed |
| `cargo clippy -p openwepp-input-contract --all-targets -- -D warnings` | exit 0 |
| `cargo clippy --test infile_hbp_parser_contract -- -D warnings` | exit 0 |
