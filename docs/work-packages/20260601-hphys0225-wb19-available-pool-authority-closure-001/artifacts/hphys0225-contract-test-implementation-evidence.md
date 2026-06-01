# HPHYS0225 Contract-Test Implementation Evidence

Status: completed  
Evidence mode: Static + Ran

## Added/Updated Tests

1. `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`
   - verifies suite/registry/contract addendum linkage,
   - verifies fixture-driven lateral/drainage withdrawals remain invariant under
     low/high legacy drainable-storage perturbations,
   - enforces source-level prohibition of legacy max-reconciliation expressions.
2. `tests/integration/auth06_fixture_provenance_hash_enforcement_contract.rs`
   - includes new suite doc + fixture root in required fixture-integrity checks.
3. `Cargo.toml`
   - registers `hphys0225_wb19_layer_pool_withdrawal_cap_contract` as an
     explicit integration test target.

## Fixture Additions

- `tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/layer_pool_withdrawal_cap_cases.json`
- `.../fixtures.sha256`
- `.../fixtures.provenance.yaml`

## Executed Evidence

- Ran: `cargo test --test hphys0225_wb19_layer_pool_withdrawal_cap_contract --test auth06_fixture_provenance_hash_enforcement_contract` (pass).

## Closure Measure Mapping

- `MEASURE-HP225-003`: satisfied (contract-derived test surfaces implemented plus preimplementation gate evidence captured separately).
- `MEASURE-HP225-005`: satisfied (targeted HPHYS0225 contract tests pass).
