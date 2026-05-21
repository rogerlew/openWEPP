# Gate Results — ARCH03

Evidence: Ran
Date: 2026-05-21 UTC

## Required Gates

| gate | command | result | notes |
| --- | --- | --- | --- |
| 1 | `cargo fmt --check` | pass | no formatting drift |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass | includes pedantic lint set under `-D warnings` |
| 3 | `cargo test --workspace` | pass | existing integration suites + ARCH03 suites passed |
| 4 | `cargo deny check` | pass | emitted non-failing `license-not-encountered` warnings for unmatched allowlist entries in `deny.toml` |

## Command Evidence Excerpts

- `cargo clippy --workspace --all-targets -- -D warnings`
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) ...`
- `cargo test --workspace`
  - New ARCH03 suites passed:
    - `sim_contract_status_taxonomy` (6 passed)
    - `sim_contract_closure_checks` (6 passed)
    - `sim_contract_symbol_alias_registry` (5 passed)
- `cargo deny check`
  - Final line: `advisories ok, bans ok, licenses ok, sources ok`

## Gate Verdict
- ARCH03 gate status: `PASS`
