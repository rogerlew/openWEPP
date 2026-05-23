# PL17 Gate Results

Status: `complete`
Evidence mode: `Static + Ran`

## Pre-Implementation Contract Gate Snapshot

Command:
```bash
cargo test --test parser_runtime_seam_integration pl17_contract_conformance_ -- --nocapture
```

Observed baseline during PL17 kickoff sequencing:
- `pl17_contract_conformance_requires_decomposition_rate_projection_symbols` failed
- `pl17_contract_conformance_scheduler_emits_equation_updated_annual_decomposition_state_on_active_day` failed
- `pl17_contract_conformance_scheduler_emits_equation_updated_perennial_decomposition_state_on_active_day` failed
- `pl17_contract_conformance_rejects_missing_decomposition_equation_symbol` failed

Failure posture recorded in kickoff baseline:
- decomposition equation parameters were not yet projected,
- decomposition payload assembly was still pass-through for tracked seed pools,
- missing decomposition equation symbols were not yet hard-failing at decomposition dispatch.

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

## Focused PL17 Verification Commands

- `cargo test --test parser_runtime_seam_integration pl17_contract_conformance_ -- --nocapture` -> `4 passed`
- `cargo test --test int10_plant_water_coupling_validation_contract -- --nocapture` -> `3 passed`
- `cargo test -p openwepp-hillslope-orchestrator` -> `51 passed`
