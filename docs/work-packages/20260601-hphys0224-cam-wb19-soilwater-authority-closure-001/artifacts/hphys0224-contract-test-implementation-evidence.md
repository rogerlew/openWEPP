# HPHYS0224 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Added Contract-Derived Coverage

1. New integration test:
   - `tests/integration/hphys0224_wb19_withdrawal_soilwater_cap_contract.rs`
   - Validates:
     - suite/registry/SC addendum presence,
     - in-domain WB19 lateral/drainage withdrawal behavior,
     - typed over-withdrawal hard-fail behavior.

2. New Level-4 external-authority suite:
   - `docs/specifications/external-authority/suites/cas_l4_subhyd_withdrawal_soilwater_cap_001.md`
   - `tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001/withdrawal_soilwater_cap_cases.json`
   - `fixtures.sha256` + `fixtures.provenance.yaml`
   - Registry entry in `docs/specifications/external-authority/registry.yaml`.

3. Gate/fixture-integrity coverage updates:
   - Added test target to `Cargo.toml`.
   - Updated `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
     suite-doc and fixture-root coverage arrays.

## Ran Evidence

- `cargo test --test hphys0224_wb19_withdrawal_soilwater_cap_contract` (pass)
- `cargo test --test auth06_fixture_provenance_hash_enforcement_contract --test hphys0224_wb19_withdrawal_soilwater_cap_contract` (pass)
- `sha256sum --check --strict` under
  `tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001`
  (pass)
