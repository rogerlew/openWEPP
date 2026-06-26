# Line-Count Governance Checklist

Status: queued.
Evidence mode: not-run.

Record `.rs` line counts for touched files. Files at or above `2000` lines
require WARN disposition; non-exempt files at or above `3000` lines block
closure until refactored or explicitly exempted by accepted governance.
