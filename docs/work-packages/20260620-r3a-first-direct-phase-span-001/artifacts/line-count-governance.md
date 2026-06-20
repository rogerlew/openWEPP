# R3A Line-Count Governance

Status: queued.
Evidence mode: not run.

Before closure, run and record `.rs` line counts for touched Rust files.

Rules:

- files at or above 2000 lines are `WARN`;
- files at or above 3000 lines require refactor before closure unless an
  explicit exception with owner and sunset is recorded;
- touching `scheduler.rs` requires a strong justification and line-count
  closure plan because it is historically over 3000 lines.
