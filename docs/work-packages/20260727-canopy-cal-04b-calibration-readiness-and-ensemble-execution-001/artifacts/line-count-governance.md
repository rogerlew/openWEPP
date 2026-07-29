# Line-Count Governance

Status: `PASS`

Evidence class: `Ran`

All 11 package-local Rust files were measured. The largest is
`tools/executor/src/bin/readiness.rs` at 1,082 lines. No Rust file reaches the
2,000-line warning threshold or 3,000-line blocking threshold, and no exception
is required.

The package-local terminal validator is the largest Python file. Python is
outside the Rust threshold rule; its focused arithmetic and custody controls
are covered by the 38-test package Python suite and full result execution.
