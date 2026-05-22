# Gate Results — ARCH11

Evidence: Ran
Date: 2026-05-22 UTC

## Required Gates

| gate | command | result | notes |
| --- | --- | --- | --- |
| 1 | `cargo fmt --check` | pass | no formatting drift |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass | strict clippy with `-D warnings` passed for all workspace crates |
| 3 | `cargo test --workspace` | pass | full workspace unit/integration/doc tests passed; includes ARCH11 comparator metadata integration test |
| 4 | `cargo deny check` | pass | advisories/bans/licenses/sources all ok; non-failing `license-not-encountered` warnings in allowlist |

## Command Evidence Excerpts

- `cargo clippy --workspace --all-targets -- -D warnings`
  - `Checking openwepp-comparator-metadata v0.1.0 ...`
  - `Checking openwepp-summary-accumulator v0.1.0 ...`
  - `Finished 'dev' profile [unoptimized + debuginfo] ...`
- `cargo test --workspace`
  - `Running tests/integration/comparator_tier_routing_metadata.rs`
  - `test result: ok. 5 passed; 0 failed; ...`
- `cargo deny check`
  - Final line: `advisories ok, bans ok, licenses ok, sources ok`

## Gate Verdict
- ARCH11 gate status: `PASS`
