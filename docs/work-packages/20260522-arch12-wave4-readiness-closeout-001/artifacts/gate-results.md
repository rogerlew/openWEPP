# Gate Results — ARCH12

Evidence: Ran
Date: 2026-05-22 UTC

## Required Gates

| gate | command | result | notes |
| --- | --- | --- | --- |
| 1 | `cargo fmt --check` | pass | Ran: formatting check returned success with no diffs. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass | Ran: strict clippy gate returned success for full workspace. |
| 3 | `cargo test --workspace` | pass | Ran: workspace unit/integration/doc test suites passed. |
| 4 | `cargo deny check` | pass | Ran: `advisories ok, bans ok, licenses ok, sources ok`; non-failing `license-not-encountered` warnings present. |

## Command Evidence Excerpts

- Ran: `cargo clippy --workspace --all-targets -- -D warnings`
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) ...`
- Ran: `cargo test --workspace`
  - `Running tests/integration/comparator_tier_routing_metadata.rs`
  - `test result: ok. 5 passed; 0 failed; ...`
- Ran: `cargo deny check`
  - Final line: `advisories ok, bans ok, licenses ok, sources ok`

## Gate Verdict

Ran: ARCH12 gate status is `PASS`.
