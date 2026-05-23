# PL12 Gate Results

Status: `complete`
Evidence mode: `Ran`

## Pre-Implementation Contract Gate

- `cargo test -p openwepp-hillslope-orchestrator pl12_contract_conformance -- --nocapture`
- Result: `FAILED` with expected `2` failing tests (recorded before production
  kernel edits).

## Post-Implementation PL12 Conformance Gate

- `cargo test -p openwepp-hillslope-orchestrator pl12_contract_conformance -- --nocapture`
- Result: `ok` (`2 passed`, `0 failed`).

## Required Rust Validation Gates

1. `cargo fmt --check`
- Result: `ok`

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: `ok`

3. `cargo test --workspace`
- Result: `ok`

4. `cargo deny check`
- Result: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)
- Note: `license-not-encountered` warnings reported for unmatched allowlist
  entries in `deny.toml`; command exited successfully.
