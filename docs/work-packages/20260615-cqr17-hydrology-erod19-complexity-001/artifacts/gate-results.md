# Gate Results

Status: complete.

Focused pre-gate commands already run:

```text
cargo test -p openwepp-hillslope-orchestrator \
  cqr17_erod19_xcrit_classification_preserves_branch_vectors -- --nocapture
```

Result before production refactor: exit code `0`.

Result after production refactor: exit code `0`.

```text
cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings
```

Result: exit code `0`.

Final required gates:

- Ran: `cargo fmt --check`
  - Result: exit code `0`.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: exit code `0`.
- Ran: `cargo test --workspace`
  - Result: exit code `0`.
- Ran: `cargo deny check`
  - Result: exit code `0`; output:
    `advisories ok, bans ok, licenses ok, sources ok`.
- Ran: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001 --format json`
  - Initial result before final gate artifact update: exit code `0`,
    `files_scanned: 22`, `errors: 0`, `warnings: 0`.
- Ran: `git diff --check`
  - Initial result before final gate artifact update: exit code `0`.

Final artifact-state reruns:

- Ran: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001 --format json`
  - Result: exit code `0`, `files_scanned: 22`, `errors: 0`,
    `warnings: 0`.
- Ran: `git diff --check`
  - Result: exit code `0`.
