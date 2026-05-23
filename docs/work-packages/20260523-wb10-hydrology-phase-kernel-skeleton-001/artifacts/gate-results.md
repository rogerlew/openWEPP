# WB10 Gate Results

Status: `complete`
Evidence mode: `Ran`

## Pre-Implementation Contract Gate

- `cargo test -p openwepp-hillslope-orchestrator wb10_contract_conformance -- --nocapture`
- Result (pre-implementation baseline): `FAILED` with expected failing test:
  - `wb10_contract_conformance_hydrology_phase_classes_are_not_generic`

## Post-Implementation WB10 Conformance Gate

- `cargo test -p openwepp-hillslope-orchestrator wb10_contract_conformance -- --nocapture`
- Result: `ok` (`2 passed`, `0 failed`).

## Targeted Typed-Seam Regression Gates

- `cargo test -p openwepp-kernel-contract phase_class_hydrology_predicate_matches_contract -- --nocapture`
- Result: `ok` (`1 passed`, `0 failed`).

- `cargo test -p openwepp --test hillslope_consumer_boundary_integration -- --nocapture`
- Result: `ok` (`4 passed`, `0 failed`).

## Required Rust Validation Gates

1. `cargo fmt --check`
- Result: `ok`
- Note: one initial formatting diff in hydrology dispatch match was corrected
  with `cargo fmt` before final check.

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: `ok`

3. `cargo test --workspace`
- Result: `ok`

4. `cargo deny check`
- Result: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)
- Note: `license-not-encountered` warnings reported for unmatched allowlist
  entries in `deny.toml`; command exited successfully.
