# WB13 Contract-Test Implementation Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Implemented Test Target
- `tests/integration/wb13_daily_water_balance_output_surface_contract.rs`
- Registered in `Cargo.toml` as test target
  `wb13_daily_water_balance_output_surface_contract`

## Contract-Derived Tests
1. `wb13_contract_conformance_emits_canonical_25_column_rows_and_monotonic_order`
- Verifies canonical 25-column schema surface and deterministic key-order guard.

2. `wb13_contract_conformance_rejects_missing_required_profile_symbol`
- Verifies typed missing-symbol guard for required profile output fields.

3. `wb13_contract_conformance_rejects_non_finite_and_domain_invalid_symbols`
- Verifies non-finite and domain-invalid WB13 output rejection.

## Execution Evidence
Command:
```bash
cargo test --test wb13_daily_water_balance_output_surface_contract
```
Result: `3 passed; 0 failed`.
