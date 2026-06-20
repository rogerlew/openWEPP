# Gate Results

Ran:

| Gate | Result |
|---|---|
| `cargo test -p openwepp-hillslope-orchestrator r5a_direct_skeleton -- --nocapture` | PASS |
| `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture` | PASS, 42 tests |
| `cargo test -p openwepp-runner r2a_ -- --nocapture` | PASS, 2 tests |
| no-compatibility source scan | PASS, no matches |
| scheduler/API diff review | PASS, empty diff |
| `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/r5-burndown-execplan.md --path docs/work-packages/20260620-r5a-full-day-direct-executor-lifecycle-001 --format json` | PASS, 25 files, 0 errors, 0 warnings |
| `git diff --check` | PASS |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --workspace` | PASS |
| `cargo deny check` | PASS |
| release build | PASS, `57.93 s`, `1111320 KB` |
| H2637 default-disabled reps | PASS, median `643.98 s <= 676.67 s` |
| protected output comparison | PASS, HBP/WAT byte identity, PASS row equivalence, loss/plot run-name-only differences |

Final gate verdict: PASS.
