# SOILAUTH02 Implementation and Test Evidence

Status: complete  
Evidence mode: Static + Ran

## Scope
Parser/contract reconciliation implementation and validation evidence.

## Production/Contract Implementation
Static:
- `crates/openwepp-input-contract/src/parsers/soil.rs`
  - policy-first parsing enabled for policy datvers in strict+compat,
  - quoted-header parsing (`single` + `double`) enabled for
    `7778/9002/9003/9005`,
  - omitted `avke` normalization retained and ratified across strict+compat,
  - per-OFE restrictive-row parsing/normalization enabled in strict+compat with
    identical-row enforcement and trailing-row consistency guard.
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `tests/integration/infile_soil_parser_contract.rs`
- `tests/integration/soilauth02_soil_producer_reconciliation_contract.rs`
- `tests/fixtures/infile/soil/canonical_9002_double_quoted_policy.sol`
- `tests/fixtures/infile/soil/fixtures.sha256`
- `tests/fixtures/infile/soil/fixtures.provenance.yaml`

## Executed Validation
Ran:
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --test infile_soil_parser_contract --test soilauth02_soil_producer_reconciliation_contract` -> pass
- `cargo test --workspace` -> fail (unrelated `auth05_*` FC authority tests)
- `cargo test --test auth05_level4_constitutive_authority_hardening_contract` -> fail (same mismatch)
- `cargo deny check` -> pass (non-blocking duplicate/license-not-encountered warnings)
