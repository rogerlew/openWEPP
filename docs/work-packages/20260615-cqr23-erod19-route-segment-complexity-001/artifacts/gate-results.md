# Gate Results

Status: complete.

Ran: `cargo fmt --check`

- Exit code: `0`

Ran: `cargo clippy --workspace --all-targets -- -D warnings`

- Exit code: `0`
- Summary: finished dev profile for workspace targets with no warnings.

Ran: `cargo test --workspace`

- Exit code: `0`
- Summary: workspace unit tests, integration tests, runner tests, and doc tests
  passed. Existing tests with intentional internal negative checksum probes
  printed expected failure lines while their Rust tests passed.

Ran: `cargo deny check`

- Exit code: `0`
- Summary: `advisories ok, bans ok, licenses ok, sources ok`

Ran:

- `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001 --format json`

Exit code: `0`

Summary: `files_scanned` `22`, `errors` `0`, `warnings` `0`.

Ran:

- `git diff --check`

Exit code: `0`
