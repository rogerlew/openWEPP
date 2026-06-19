# PERFDEEP09 Line-Count Governance

Status: queued.
Evidence mode: not run.

Before closure, run and record `.rs` line counts for touched Rust files.

Rules:

- files at or above 2000 lines are `WARN`;
- files at or above 3000 lines require refactor before closure unless a
  generated/fixture exception is explicitly approved with owner and sunset plan;
- `scheduler.rs` is known to exceed 3000 lines and cannot be casually touched
  without closure evidence.
