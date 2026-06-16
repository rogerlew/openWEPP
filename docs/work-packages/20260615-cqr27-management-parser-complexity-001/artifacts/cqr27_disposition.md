# CQR27 Disposition

Status: complete.

Decision: accept CQR27 as complete-with-warnings.

Static: Review Agent A findings: none.

Static: Review Agent B findings: none.

Ran: final target CRAP is `4.0`, below the `<= 30` threshold.

Ran: all newly extracted helpers are below the `<= 30` threshold.

Ran: target-file coverage did not regress: line coverage improved from
`67.24%` to `71.14%`, and function coverage improved from `81.63%` to
`83.33%`.

Static: production Rust changes are private helper extraction only. Public
parser API, grammar, error IDs, diagnostic strings, defaults,
strict/compatibility behavior, and parsed output shape are unchanged.

Ran: required cargo gates passed:

- `cargo fmt --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo deny check`.

Warnings:

- `cargo crap` reported LCOV source-map warnings for 126 workspace
  test/support source files. The CQR27 target file was represented in LCOV.
- Non-target management parser rows over CRAP `30` remain for later ranked CQR
  work.
