# Gate Results

Ran: `cargo fmt --check`

Result: passed.

Ran: `cargo clippy --workspace --all-targets -- -D warnings`

Result: passed.

Ran: `cargo test --workspace`

Result: passed.

Ran: `cargo deny check`

Result: passed with `advisories ok, bans ok, licenses ok, sources ok`.

Ran: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr30-erod13-wave1-complexity-001 --format json`

Result: passed with `22` files scanned, `0` errors, `0` warnings.

Ran: `git diff --check`

Result: passed.

Additional focused checks:

- `cargo test --test erod13_wave1_core_kernel_contract`: passed, `7` passed,
  `0` failed.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`:
  passed.

Status: closure gates passed.
