# PL11 Gate Results

Status: `complete`
Evidence mode: `Ran`

## Pre-Implementation Contract Gate

- `cargo test --test parser_runtime_seam_integration pl10b_contract_conformance -- --ignored`
- Result: `FAILED` with expected 5 failing transferred conformance tests (recorded pre-edit baseline).

## Post-Implementation Conformance Gate

- `cargo test --test parser_runtime_seam_integration pl10b_contract_conformance`
- Result: `ok` (`5 passed`, `0 failed`).

## Required Rust Validation Gates

1. `cargo fmt --check`
- Result: `ok`

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: `ok`

3. `cargo test --workspace`
- Result: `ok` (workspace integration and unit suites passed)

4. `cargo deny check`
- Result: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)
- Note: `license-not-encountered` warnings reported for unmatched allowlist entries in `deny.toml`; command exit status remained success.
