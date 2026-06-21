# Gate Results

Status: complete.
Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| R5E focused test | PASS | `cargo test -p openwepp-hillslope-orchestrator r5e_ -- --nocapture`: 1 passed. |
| Direct-runtime focused tests | PASS | `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`: 56 passed. |
| Runner default-disabled/direct counter tests | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture`: 2 passed. |
| No-compatibility source scan | PASS | Covered by direct-runtime focused tests, including `growth.rs`. |
| H2637 default-disabled benchmark | PASS | Reps `641.37 s`, `642.02 s`, `635.47 s`; median `641.37 s` <= `676.67 s`. |
| H2637 opt-in direct-skeleton endpoint/RSS | PASS | `r5e_h2637_direct_skeleton_rep1  638.33  229260`. |
| Protected output comparison | PASS | HBP/WAT/loss/plot byte-identical; PASS DuckDB row equivalence `12419` rows, zero bidirectional differences. |
| `cargo fmt --check` | PASS | Ran successfully. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran successfully. |
| `cargo test --workspace` | PASS | Ran successfully. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Scoped markdown lint | PASS | `markdown-doc lint --path docs/work-packages/20260621-r5e-full-ofe-day-endpoint-readiness-001 --format json`: 20 files scanned, 0 errors, 0 warnings. |
| `git diff --check` | PASS | Ran successfully after final package documentation edits. |
| Review and verification | PASS | Local dual review and verification artifacts complete. |
