# Gate Results

Status: complete.

Ran: `cargo fmt --check`

- Result: pass.

Ran: `cargo clippy --workspace --all-targets -- -D warnings`

- Result: pass.

Ran: `cargo test --workspace`

- Result: pass.

Ran: `cargo deny check`

- Result: pass.
- Output summary: `advisories ok, bans ok, licenses ok, sources ok`.

Ran: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr24-scheduler-seed-runtime-complexity-001 --format json`

- Result: pass.
- Summary: `files_scanned: 22`, `errors: 0`, `warnings: 0`.

Ran: `git diff --check`

- Result: pass.

Ran: before/after focused characterization:

- `cargo test -p openwepp-runner hillstab08_wb16_producer`
- Result: pass before and after production refactor.
