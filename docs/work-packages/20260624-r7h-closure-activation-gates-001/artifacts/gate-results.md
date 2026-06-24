# Gate Results

Evidence class: Ran, updated at package close.

| Gate | Result | Evidence |
| --- | --- | --- |
| Formatting | pass | `cargo fmt --check` |
| Focused orchestrator R7G | pass | `cargo test -p openwepp-hillslope-orchestrator r7g_ -- --nocapture` |
| Focused runner R7G | pass | `cargo test -p openwepp-runner r7g_ -- --nocapture` |
| Focused runner R7E | pass | `cargo test -p openwepp-runner r7e_ -- --nocapture` |
| Release CLI build | pass | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` |
| H2637 direct endpoint | pass endpoint / fail timing | `113.53 s`, `1083636 KiB`, budget `<=91.2 s` |
| Direct no-compat counters | pass | Manifest `compatibility_edge_invocations=0` |
| Protected output parity | fail / current-code matrix incomplete | Retained compatibility comparison differs for HBP/WAT/PASS/loss/plot; current compatibility rerun skipped after timing failure. |
| Default activation | blocked | Direct timing and parity are not green. |

Final workspace gates:

- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed.
- `git diff --check`: passed.
- `wctl doc-lint`: passed with zero findings but scanned zero files because
  the wrapper runs staged-only lint and this package was not staged.
- `markdown-doc lint --path docs/work-packages/20260624-r7h-closure-activation-gates-001 --path docs/work-packages/README.md --path docs/ROADMAP.md --format json`:
  passed; scanned `10` files, `0` errors, `0` warnings.
