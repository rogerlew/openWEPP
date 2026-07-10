# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Pre-decomposition characterization | PASS | Detached scaffold worktree: 1 passed, 333 skipped; `characterization.md`. |
| Focused crate nextest | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick`; 339 tests started, exit 0. |
| Focused crate clippy | PASS | `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`, exit 0. |
| Target LLVM coverage / CRAP | PASS | 97.8873% production-only lines, 97.4170% regions; zero rows >30; after artifacts. |
| `git diff --check` | PASS | Exit 0 after final source/artifact edits. |
| Package/catalog documentation lint | PASS | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-b02-02-direct-runtime-audit-error-helpers-001 --path docs/work-packages/README.md --format plain`; 22 files, 0 errors/warnings. |
| `cargo fmt --check` | PASS | Delegated workspace closure, exit 0; `/tmp/openwepp-cqr-b02-t02-closure/fmt.{log,exit}`. |
| Workspace clippy | PASS | Delegated `cargo clippy --workspace --all-targets -- -D warnings`, exit 0; `clippy.{log,exit}`. |
| Workspace full nextest | PASS | Delegated `cargo nextest run --workspace --profile full`: 1609 passed, 3 skipped, 4 slow, 583.483s; `nextest-full.{log,exit}`. |
| `cargo deny check` | PASS | Delegated workspace closure, exit 0; `deny.{log,exit}`. |

No authority-suite, cohort-fixture, or required-case binding changed, so the
root anti-evasion guard does not apply.
