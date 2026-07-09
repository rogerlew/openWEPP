# Gate Results

Evidence label: Static.

Status: `QUEUED`

Required gates to record before completion or hold:

- `git diff --check`
- markdown/doc lint for touched docs
- focused tests for `openwepp-snowbench` behavior
- target-module coverage/CRAP after implementation
- output/API identity evidence
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

Heavy full-workspace gates require `comparator_suite_runner` unless unavailable;
if unavailable, record command-level fallback evidence before running locally.
