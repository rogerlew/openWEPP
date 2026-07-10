# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Pre-decomposition characterization | PASS | Detached scaffold worktree: `1` passed, `5` skipped; `characterization.md`. |
| Focused crate nextest | PASS | `cargo nextest run -p openwepp-input-contract --profile quick`; `17/17` passed, exit 0. |
| Focused crate clippy | PASS | `cargo clippy -p openwepp-input-contract --all-targets -- -D warnings`, exit 0. |
| Target LLVM coverage / CRAP | PASS | 97.924% production lines, 91.279% regions; 0 rows >30; after artifacts. |
| `git diff --check` | PASS | Exit 0 after source and initial artifact edits. |
| Package documentation lint | PASS | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-b02-03-soil-parser-001 --format plain`; 15 files, 0 errors/warnings. |
| `cargo fmt --check` | PASS | Delegated workspace closure, exit 0; `/tmp/openwepp-cqr-b02-t03-closure/fmt.{log,exit}`. |
| Workspace clippy | PASS | Delegated `cargo clippy --workspace --all-targets -- -D warnings`, exit 0. |
| Workspace full nextest | PASS | Delegated `cargo nextest run --workspace --profile full`: 1621 passed, 3 skipped, 4 slow, 595.609s. |
| `cargo deny check` | PASS | Delegated workspace closure, exit 0; advisories, bans, licenses, and sources all OK. |

No authority-suite, cohort-fixture, or required-case binding changed, so the
root anti-evasion guard does not apply.
