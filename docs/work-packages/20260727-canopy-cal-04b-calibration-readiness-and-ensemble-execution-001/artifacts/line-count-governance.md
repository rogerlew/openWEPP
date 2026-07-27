# Line-Count Governance

Status: `PASS`

Evidence class: `Ran`

All 11 package-local Rust files were measured. The largest is
`tools/executor/src/bin/readiness.rs` at 1,082 lines. No Rust file reaches the
2,000-line warning threshold or 3,000-line blocking threshold, and no exception
is required.

The largest package-local Python file is `tools/validate.py` at 1,724 lines;
this is retained as context and is outside the Rust threshold rule.

