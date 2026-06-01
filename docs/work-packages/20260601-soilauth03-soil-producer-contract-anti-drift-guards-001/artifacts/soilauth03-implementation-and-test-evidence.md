# SOILAUTH03 Implementation and Test Evidence

Status: complete  
Evidence mode: Static + Ran

## Scope
SOILAUTH03 implementation and validation evidence.

## Production/Contract Implementation
Static:
- `docs/specifications/external-authority/suites/cas_l4_infile_soil_producer_contract_001.md`
- `docs/specifications/external-authority/registry.yaml`
- `docs/specifications/external-authority/required-suite-obligations.json`
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `tests/fixtures/infile/soil/soilauth03_guard_cases.json`
- `tests/fixtures/infile/soil/fixtures.sha256`
- `tests/fixtures/infile/soil/fixtures.provenance.yaml`
- `tests/integration/soilauth03_soil_producer_contract_drift_guards_contract.rs`
- `Cargo.toml`
- `tools/release/README.md`
- `docs/governance/openwepp-release-procedure-draft.md`

## Executed Validation
Ran:
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --test soilauth03_soil_producer_contract_drift_guards_contract` -> pass
- `cargo test --test auth11_required_suite_obligation_guards_contract` -> pass
- `cargo test --test auth06_fixture_provenance_hash_enforcement_contract` -> pass
- `cargo test --workspace` -> fail (pre-existing unrelated `auth05_*` FC authority failures)
- `cargo deny check` -> pass (warnings only)
