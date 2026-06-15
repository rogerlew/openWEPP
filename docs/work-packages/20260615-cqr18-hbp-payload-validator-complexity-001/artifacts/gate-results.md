# Gate Results

Status: complete.

Focused pre-gate commands already run:

```text
cargo test --test infile_hbp_parser_contract payload_validator -- --nocapture
```

Result before production refactor: exit code `0`; `3 passed`.

```text
cargo test --test infile_hbp_parser_contract -- --nocapture
```

Result after production refactor: exit code `0`; `24 passed`.

Metric commands already run:

- Ran: before `cargo llvm-cov --workspace --ignore-run-fail --lcov`
  - Result: exit code `0`; output saved to `lcov_before.info`.
- Ran: before `cargo crap --workspace --lcov ... --min 0 --format json`
  - Result: exit code `0`; output saved to `crap_before.json`.
- Ran: after `cargo llvm-cov --workspace --ignore-run-fail --lcov`
  - Result: exit code `0`; output saved to `lcov_after.info`.
- Ran: after `cargo crap --workspace --lcov ... --min 0 --format json`
  - Result: exit code `0`; output saved to `crap_after.json`.

Final required gates:

- Ran: `cargo fmt --check`
  - Result: exit code `0`.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings`
  - Initial result: exit code `101`; one `clippy::needless_return` in the
    new helper.
  - Fixed and reran: exit code `0`.
- Ran: `cargo test --workspace`
  - Result: exit code `0`.
- Ran: `cargo deny check`
  - Result: exit code `0`; output:
    `advisories ok, bans ok, licenses ok, sources ok`.
- Ran: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001 --format json`
  - Initial result before final artifact status update: exit code `0`,
    `files_scanned: 22`, `errors: 0`, `warnings: 0`.
- Ran: `git diff --check`
  - Initial result before final artifact status update: exit code `0`.

Final artifact-state reruns:

- Ran: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001 --format json`
  - Result: exit code `0`, `files_scanned: 22`, `errors: 0`,
    `warnings: 0`.
- Ran: `git diff --check`
  - Result: exit code `0`.
