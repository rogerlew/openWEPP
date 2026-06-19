# Line-Count Governance

Status: queued.
Evidence mode: not run.

Before production edits, record current line counts for touched Rust files.

Rules:

- 2000+ lines is WARN and needs disposition.
- 3000+ non-exempt touched files require refactor or another governance-closing
  action before completion.
- Avoid touching `scheduler.rs` if hard isolation can be achieved elsewhere.
- If `scheduler.rs` must be touched, record the split/closure plan before the
  edit and complete it before `READY-FOR-R2`.
