# Validation

## Focused Gates

- Ran: `cargo test -p openwepp-hillslope-orchestrator r5d_ -- --nocapture`
  - Result: PASS, 5 tests.
- Ran: `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`
  - Result: PASS, 55 tests.
- Ran: `cargo test -p openwepp-runner r2a_ -- --nocapture`
  - Result: PASS, 2 tests.
- Static: scheduler/API/public-output path diff:
  - `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  - `crates/openwepp-runner/src/api.rs`
  - Result: empty diff.

## Workspace Gates

- Ran: `cargo fmt --check`
  - Result: PASS.
- Ran: `git diff --check`
  - Result: PASS.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: PASS.
- Ran: `cargo test --workspace`
  - Result: PASS.
- Ran: `cargo deny check`
  - Result: PASS (`advisories ok, bans ok, licenses ok, sources ok`).
- Ran: `markdown-doc lint --path docs/work-packages/20260620-r5d-growth-transition-direct-phases-001 --format json`
  - Result: PASS, 16 files scanned, 0 errors, 0 warnings.
- Ran: `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/r5-burndown-execplan.md --path docs/work-packages/20260620-r5d-growth-transition-direct-phases-001 --format json`
  - Result: PASS, 19 files scanned, 0 errors, 0 warnings.
