# Gate Results

Status: complete.
Evidence mode: Ran.

Required gates:

| Gate | Result | Evidence |
|---|---:|---|
| `cargo fmt --check` | PASS | Exit `0`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Exit `0` after replacing direct float comparisons in tests with exact bit comparisons. |
| `cargo test --workspace` | PASS | Exit `0`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Focused R4A tests | PASS | `cargo test -p openwepp-hillslope-orchestrator r4a_ -- --nocapture`: `2 passed; 0 failed`. |
| Runner direct-runtime counter tests | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture`: `2 passed; 0 failed`. |
| No-compatibility proof | PASS | Source forbidden-token scan no matches; scheduler no-diff; runtime counters pass. |
| Default-disabled H2637 median `<= 676.67 s` | PASS | Final binary run: `644.01 / 646.84 / 643.66 s`, median `644.01 s`. |
| Protected output identity | PASS | HBP/loss/WAT/plot hashes stable; PASS DuckDB row equivalence `0/0` deltas. |
| Markdown lint | PASS | `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260620-r4a-direct-runoff-partition-span-001 --format json`: `0` errors, `0` warnings. |
| `git diff --check` | PASS | Exit `0`. |
