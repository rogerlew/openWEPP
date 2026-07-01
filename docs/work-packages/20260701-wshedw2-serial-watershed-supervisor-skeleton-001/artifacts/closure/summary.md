# WSHED-W2 Closure Gate Summary

Status: `PASS`

Ran by final `comparator_suite_runner` on the current tree:

- `cargo fmt --check`: `PASS` (`exit 0`)
- `cargo clippy --workspace --all-targets -- -D warnings`: `PASS` (`exit 0`)
- `cargo nextest run --workspace --profile full`: `PASS` (`exit 0`)
  - Summary: `1280 tests run: 1280 passed (1 slow), 1 skipped`
- `cargo deny check`: `PASS` (`exit 0`)
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture`:
  `PASS` (`exit 0`)
  - Summary: `20 passed; 0 failed`

Current logs:

- `cargo-fmt-check.log`
- `cargo-clippy-full.log`
- `cargo-nextest-full.log`
- `cargo-deny-check.log`
- `focused-w2-gate.log`
- `command-log.json`

Earlier failed comparator/reviewer findings were accepted and fixed before this
current-tree closure run.
