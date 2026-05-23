# PL16 Gate Results

Status: `complete`
Evidence mode: `Static + Ran`

## Pre-Implementation Contract Gate Snapshot

Command:
```bash
cargo test --test parser_runtime_seam_integration pl16_contract_conformance_ -- --nocapture
```

Observed baseline during PL16 kickoff sequencing:
- `pl16_contract_conformance_scheduler_emits_equation_updated_annual_growth_state_on_active_day` failed
- `pl16_contract_conformance_scheduler_emits_equation_updated_perennial_growth_state_on_active_day` failed
- `pl16_contract_conformance_rejects_missing_growth_equation_symbol` failed

Failure posture recorded in kickoff baseline:
- active-growth path not yet satisfying equation-update assertions,
- required-symbol halt posture not yet aligned to PL16 test expectations.

## Post-Implementation and Repository Gates

1. `cargo fmt --check`
- Result: `ok`

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: `ok`

3. `cargo test --workspace`
- Result: `ok`

4. `cargo deny check`
- Result: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)
- Note: non-fatal `license-not-encountered` warnings were emitted for unmatched allowlist entries.

## Focused PL16 Verification Commands

- `cargo test --test parser_runtime_seam_integration pl16_contract_conformance_ -- --nocapture` -> `3 passed`
- `cargo test --test int10_plant_water_coupling_validation_contract -- --nocapture` -> `3 passed`
- `cargo test -p openwepp-hillslope-orchestrator` -> `51 passed`
