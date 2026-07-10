# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Pre-decomposition public characterization | PASS | Detached scaffold worktree: 6 passed; `characterization.md`. |
| Focused CLI contract suite | PASS | `cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract --profile quick`; 6/6 passed. |
| Focused bin clippy/check/fmt | PASS | All exit 0 after final extraction. |
| Target LLVM coverage / CRAP | PASS | 94.416% production lines, 93.443% regions; 0 rows >30; after artifacts. |
| `git diff --check` | PASS | Exit 0 after source and artifact edits. |
| Package/catalog documentation lint | PASS | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-b02-05-runner-totalwatsed3-001 --path docs/work-packages/README.md --format plain`, exit 0. |
| `cargo fmt --check` | PASS | Delegated workspace closure, exit 0; `/tmp/openwepp-cqr-b02-t05-closure/fmt.{log,exit}`. |
| Workspace clippy | PASS | Delegated `cargo clippy --workspace --all-targets -- -D warnings`, exit 0. |
| Workspace full nextest | PASS | Delegated full profile: 1624 passed, 3 skipped, 4 slow, 594.103s. |
| `cargo deny check` | PASS | Delegated closure, exit 0; advisories, bans, licenses, and sources OK. |

No authority-suite, cohort-fixture, or required-case binding changed.
