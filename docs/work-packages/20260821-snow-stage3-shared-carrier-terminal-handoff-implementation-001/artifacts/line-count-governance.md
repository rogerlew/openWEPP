# Line-count governance

Status: `BLOCK / STATIC THRESHOLD REQUIRES FOLLOW-ON REFACTOR`

Evidence class: `Static`.

`wc -l` reports 1,226 lines for the new
`snow_stage3_terminal_handoff.rs`, 99 lines for the persisted restart module,
and 431 lines for the package integration test. The edited
`direct_runtime/03_executor.rs` is 2,246 lines after the owner-aware scheduler
method was added; this crosses the repository 2,000-line WARN threshold but
remains below the 3,000-line block threshold. The new method belongs to the
existing direct publication orchestration boundary; follow-up refactoring must
not obscure the publication/owner commit ordering required by this package.

The amended owner-wiring set also touched the existing
`v9_real_consumer_shadow.rs`, which is now 3,151 lines (3,150 at the baseline),
and its test module is 2,032 lines. The former exceeds the repository 3,000-line
block threshold; the latter exceeds the 2,000-line warning threshold. This is a
pre-existing large-module boundary incremented by one line for the required
typed-stack clone, not evidence that the package should hide the size. A
separate refactor package is required before treating line-count governance as
closed.

Inside `nix develop`, `cargo fmt --all -- --check` and
`cargo check --workspace` passed. Follow-on source Clippy findings were
cleaned; the workspace-wide strict run still reports unrelated existing test
lint failures.
