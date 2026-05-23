# PL13 Gate Results

Status: `complete`
Evidence mode: `Ran`

## Pre-Implementation Contract Gate

- `cargo test -p openwepp-hillslope-orchestrator pl13_contract_conformance -- --nocapture`
- Result (pre-implementation baseline): `FAILED` with expected `2` failing tests:
  - `pl13_contract_conformance_rejects_missing_growth_state_surface`
  - `pl13_contract_conformance_rejects_growth_state_domain_violation`

## Post-Implementation PL13 Conformance Gate

- `cargo test -p openwepp-hillslope-orchestrator pl13_contract_conformance -- --nocapture`
- Result: `ok` (`2 passed`, `0 failed`).

## Targeted PL13 Integration Gate

- `cargo test -p openwepp --test parser_runtime_seam_integration pl13_contract_conformance -- --nocapture`
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
- Note: `license-not-encountered` warnings were reported for unmatched
  allowlist entries in `deny.toml`; command exited successfully.
