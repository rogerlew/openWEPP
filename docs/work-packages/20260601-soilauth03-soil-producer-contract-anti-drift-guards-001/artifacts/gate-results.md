# SOILAUTH03 Gate Results

Status: complete  
Evidence mode: Static + Ran

## Scope
SOILAUTH03 gate execution summary.

## Closure Measures
1. `MEASURE-SA03-001` -> pass  
   Required symbol/arity/order obligations are machine-checked and injected
   drift vectors fail.
2. `MEASURE-SA03-002` -> pass  
   Fixture provenance/hash guard fails on tampered lock and passes on locked
   state.
3. `MEASURE-SA03-003` -> pass  
   Release-gate posture is explicit for required soil producer-contract checks
   (`required` + `hard-fail`) and documented in release docs.

## Validation Commands
Ran:
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --test soilauth03_soil_producer_contract_drift_guards_contract` -> pass
- `cargo test --test auth11_required_suite_obligation_guards_contract` -> pass
- `cargo test --test auth06_fixture_provenance_hash_enforcement_contract` -> pass
- `cargo test --workspace` -> fail (unrelated pre-existing `auth05_*` FC authority failures)
- `cargo deny check` -> pass (warnings only)
