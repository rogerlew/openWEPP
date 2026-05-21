# Gate Results — ARCH04

Evidence: Ran
Date: 2026-05-21 UTC

## Required Gates

| gate | command | result | notes |
| --- | --- | --- | --- |
| 1 | `cargo fmt --check` | pass | no formatting drift |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass | includes pedantic lint profile under `-D warnings` |
| 3 | `cargo test --workspace` | pass | includes ARCH04 integration test target |
| 4 | `cargo deny check` | pass | non-failing `license-not-encountered` warnings only |

## Command Evidence Excerpts

- `cargo clippy --workspace --all-targets -- -D warnings`
  - `Checking openwepp-topology ...`
  - `Finished 'dev' profile ...`
- `cargo test --workspace`
  - `topology_graph_validation_gate` test suite: `6 passed; 0 failed`
- `cargo deny check`
  - final line: `advisories ok, bans ok, licenses ok, sources ok`

## Gate Verdict
- ARCH04 gate status: `PASS`
