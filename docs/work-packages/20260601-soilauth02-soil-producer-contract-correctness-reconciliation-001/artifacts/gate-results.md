# SOILAUTH02 Gate Results

Status: complete  
Evidence mode: Static + Ran

## Scope
SOILAUTH02 gate execution summary.

## Closure Measures
1. `MEASURE-SA02-001` -> pass
2. `MEASURE-SA02-002` -> pass
3. `MEASURE-SA02-003` -> pass

## Validation Commands
Ran:
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> fail (unrelated `auth05_*` failures)
- `cargo deny check` -> pass (warnings only)
- `cargo test --test infile_soil_parser_contract --test soilauth02_soil_producer_reconciliation_contract` -> pass
- `cargo test --test auth05_level4_constitutive_authority_hardening_contract` -> fail (`thetfc_0001` mismatch)
