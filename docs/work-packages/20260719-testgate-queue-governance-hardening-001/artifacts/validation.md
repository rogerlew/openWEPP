# Focused Validation

Ran: 2026-07-19 UTC.

| Check | Result | Evidence |
| --- | --- | --- |
| Rust formatting | PASS | `cargo fmt --check`. |
| Focused CI contract | PASS | `cargo nextest run --profile quick --test testgate_ci_executor_contract`: 2 passed, 0 skipped. |
| Workflow YAML | PASS | Ruby YAML loaded both changed workflows with aliases enabled. |
| Queue-drain hook | PASS | `bash -n` and exact-image runtime probe; immutable hook exited 1. |
| Markdown | PASS | Canonical `markdown-doc lint` returned zero errors/warnings on changed guidance and package files. |
| Diff integrity | PASS | `git diff --check`. |

`actionlint` was unavailable. No production Rust changed, and the package
explicitly excludes full Nextest, coverage, CRAP, and live broad execution.
