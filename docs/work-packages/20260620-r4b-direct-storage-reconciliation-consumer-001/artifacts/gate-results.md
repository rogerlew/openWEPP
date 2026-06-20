# R4B Gate Results

Status: complete.
Evidence mode: Ran.

| Gate | Result | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | Ran after implementation. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after implementation. |
| `cargo test --workspace` | PASS | Ran after implementation. |
| `cargo deny check` | PASS | Ran after implementation. |
| Focused R4B tests | PASS | `cargo test -p openwepp-hillslope-orchestrator r4b_ -- --nocapture` passed. |
| Runner direct-runtime counter tests | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture` passed. |
| No-compatibility proof | PASS | Forbidden-token scan, scheduler no-diff, runtime counters, and focused direct-span counters passed. |
| Default-disabled H2637 median `<= 676.67 s` | PASS | Median `641.14 s`. |
| Protected output identity | PASS | HBP/loss/WAT/plot hashes stable; PASS parquet row equivalence passed. |
| Markdown lint | PASS | `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260620-r4b-direct-storage-reconciliation-consumer-001 --format json` scanned 22 files with 0 errors and 0 warnings. |
| `git diff --check` | PASS | Ran after final artifact updates with no findings. |
