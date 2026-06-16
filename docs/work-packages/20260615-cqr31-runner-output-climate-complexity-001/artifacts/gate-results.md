# Gate Results

Ran: `cargo fmt --check`

Result: passed.

Ran: `cargo clippy --workspace --all-targets -- -D warnings`

Result: passed.

Ran: `cargo test --workspace`

Result: passed.

Ran: `cargo deny check`

Result: passed with `advisories ok, bans ok, licenses ok, sources ok`.

Ran: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr31-runner-output-climate-complexity-001 --format json`

Result: passed with `21` files scanned, `0` errors, `0` warnings.

Ran: `git diff --check`

Result: passed.

Additional focused checks:

- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`: passed.
- `cargo test -p openwepp-runner publication_wb13`: passed, `31` passed,
  `0` failed.

Status: closure gates passed.
