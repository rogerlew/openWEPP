# CQR36 Disposition

Status: complete.

Decision: accept CQR36 as complete-with-warnings.

Static: Review Agent A findings: none.

Static: Review Agent B findings: none.

Ran: final target CRAP is `15.0`, below the `<= 30` threshold.

Ran: unique target-file CRAP rows over `30`: `0`.

Ran: target-file coverage improved:

- lines `624/892` to `877/998`;
- functions `23/30` to `37/42`.

Static: production edits are private decomposition only. Public parser API,
stable error IDs, parser compatibility, branch arity, typed guards, output
shape, runtime projection, and downstream behavior are preserved.

Ran: required cargo gates passed:

- `cargo fmt --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo deny check`.

Ran: required documentation and diff gates passed:

- `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001 --format json`;
- `git diff --check`.

Warnings:

- `cargo crap` reported LCOV source-map warnings for 126 workspace
  test/support source files. The CQR36 target file was represented in LCOV.
- The first closeout clippy run failed on two `needless_raw_string_hashes`
  warnings in new tests. The literals were corrected and clippy passed on
  rerun.
