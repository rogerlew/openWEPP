# R4C Gate Results

Status: complete.
Evidence mode: Ran.

| Gate | Result | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | Ran after implementation and after the final source-test guard update. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Full workspace clippy passed. |
| `cargo test --workspace` | PASS | Full workspace test passed after final source-test guard update. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Focused R4C tests | PASS | `cargo test -p openwepp-hillslope-orchestrator r4c_ -- --nocapture`: 2 passed. |
| Focused R4B tests | PASS | `cargo test -p openwepp-hillslope-orchestrator r4b_ -- --nocapture`: 2 passed. |
| Runner direct-runtime counter tests | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture`: 2 passed. |
| No-compatibility proof | PASS | Forbidden-token scan over direct runtime sources had no matches; scheduler diff empty; runtime counters assert no direct compatibility-edge calls. |
| Default-disabled H2637 median `<= 676.67 s` | PASS | `637.63 s`, `640.25 s`, `639.19 s`; median `639.19 s`. |
| Protected output identity | PASS | Stable HBP/loss/plot/WAT hashes; PASS DuckDB row equivalence `12419/12419/0/0`, 17 columns. |
| Markdown lint | PASS | `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260620-r4c-direct-storage-input-producer-001 --format json`: 23 files, 0 errors, 0 warnings. |
| `git diff --check` | PASS | No whitespace errors; rerun with R4C untracked files temporarily marked intent-to-add so the full package diff was covered. |
