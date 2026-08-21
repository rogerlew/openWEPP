Status: complete
Evidence mode: Ran

Ran:

- `cargo check` for orchestrator, persisted-restart, and runner — PASS.
- Orchestrator library: 737 passed, 1 ignored.
- Affected-crate full nextest: 994 passed, 24 skipped.
- Persisted restart unit/doc tests — PASS in the affected-crate run.
- Focused Stage-3 integration tests — 12 passed.
- Authority anti-evasion and Auth11 obligation guards — PASS.

The final affected-crate full run was recorded as
`20260821-stage3-closure-crates-full-final`.
