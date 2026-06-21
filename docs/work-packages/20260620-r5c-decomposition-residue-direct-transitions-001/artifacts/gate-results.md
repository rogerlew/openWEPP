# Gate Results

Ran: complete.

| Gate | Evidence | Result |
|---|---|---|
| Focused R5C tests | `cargo test -p openwepp-hillslope-orchestrator r5c_ -- --nocapture` | PASS, 5 tests |
| Direct-runtime tests | `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture` | PASS, 50 tests |
| Runner default/opt-in counters | `cargo test -p openwepp-runner r2a_ -- --nocapture` | PASS, 2 tests |
| No-compat source scan | `rg` over `direct_runtime.rs` and `direct_runtime/*.rs` | PASS, no matches |
| Scheduler/API diff review | `git diff -- scheduler.rs 00_runner_intake_and_lane_setup.rs api.rs` | PASS, empty |
| Format | `cargo fmt --check` | PASS |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| Workspace tests | `cargo test --workspace` | PASS |
| Dependency policy | `cargo deny check` | PASS |
| Release build | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | PASS |
| H2637 default-disabled | Three release reps, median `643.96 s` | PASS |
| Protected outputs | HBP/WAT bytes, PASS row equivalence, normalized loss/plot | PASS |
| Whitespace | `git diff --check` | PASS |

## Notes

Static: R5C added a scoped `#[allow(clippy::too_many_lines)]` on
`DirectDayFrame::seed` after the explicit direct-frame constructor grew to 102
lines. This is dispositioned in line-count governance; new implementation and
focused tests live in split files below the WARN threshold.
