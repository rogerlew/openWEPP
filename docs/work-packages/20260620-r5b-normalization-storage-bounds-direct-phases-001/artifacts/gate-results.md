# Gate Results

Ran:

| Gate | Result |
|---|---|
| `cargo test -p openwepp-hillslope-orchestrator r5b_ -- --nocapture` | PASS, 3 tests |
| `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture` | PASS, 45 tests |
| `cargo test -p openwepp-runner r2a_ -- --nocapture` | PASS, 2 tests |
| no-compatibility source scan | PASS, no matches |
| scheduler/API diff review | PASS, empty diff |
| `git diff --check` | PASS |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --workspace` | PASS |
| `cargo deny check` | PASS |
| release build | PASS, `58.74 s`, `1085960 KB` |
| H2637 default-disabled reps | PASS, median `643.38 s <= 676.67 s` |
| protected output comparison | PASS, HBP/WAT byte identity, PASS row equivalence, loss/plot run-name-only differences |
| scoped markdown lint | PASS |

Final gate verdict: PASS.
