# Verification

Status: executed-held.

## Static

- Static: R7E policy, CLI, and manifest implementation is in:
  - `crates/openwepp-runner/src/api.rs`;
  - `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`;
  - `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`;
  - `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- Static: production direct scheduler isolation remains enforced by
  `r7c_direct_production_source_excludes_compatibility_entrypoints`.
- Static: remaining R7F blocker is executable/static evidence in
  `r7f_remaining_direct_day_input_builder_compatibility_edge_is_accounted`.
- Static: production direct compatibility-edge counter now accounts for the
  interleaved day-input builder edge.

## Ran

- Ran: `cargo test -p openwepp-runner r7e_ -- --nocapture` passed.
- Ran: `cargo fmt --check` passed.
- Ran: `cargo test -p openwepp-runner r7 -- --nocapture` passed:
  11 tests passed, 0 failed, 121 filtered out.
- Ran: `cargo test -p openwepp-runner r6 -- --nocapture` passed:
  11 unit tests and 1 CLI contract test passed, 0 failed.
- Ran: `cargo fmt --check` passed after docs/code updates.
- Ran: `git diff --check` passed.
- Ran: `markdown-doc lint --path docs/work-packages/20260623-r7e-r7h-direct-runtime-completion-001 --path docs/work-packages/README.md --path docs/ROADMAP.md --path docs/architecture/array-native-runtime-specification.md --format json`
  scanned 18 files with 0 errors and 0 warnings.

## Not Run

- Not run: full H2637 same-binary benchmark. R7G is blocked because R7F found
  a counted hot compatibility edge; a performance closure claim before removing
  that edge would be misleading.
- Not run: full workspace `cargo clippy --workspace --all-targets -- -D
  warnings`, `cargo test --workspace`, and `cargo deny check`. This package is
  not closing R7 completion; it is held at a named R7F boundary after focused
  implementation and verification.
- Not run: release anti-evasion guard suite. R7H is blocked behind R7F.
