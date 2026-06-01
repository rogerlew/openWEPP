# SOILAUTH03 Guard Obligation Map

Status: complete  
Evidence mode: Static

## Scope
Machine-checkable SOILAUTH03 obligations map.

## Suite Anchor
- `suite_id`: `cas_l4_infile_soil_producer_contract_001`
- `registry`: `docs/specifications/external-authority/registry.yaml`
- `obligations`: `docs/specifications/external-authority/required-suite-obligations.json`
- `integration_test`: `tests/integration/soilauth03_soil_producer_contract_drift_guards_contract.rs`
- `gate posture`: `required` + `hard-fail`

## Required Obligation Families
1. Canonical symbol-presence map:
   - `required_symbols` in `soil_contract_obligations`.
   - Cross-checked against:
     - `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
     - `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
2. Datver row-envelope map:
   - `header_order_symbols`
   - `header_arity_by_datver`
   - `policy_row_order_by_datver`
   - `layer_arity_by_datver`
3. Required fixture anchors:
   - `compat_quoted_header_9002_policy_first.sol`
   - `compat_quoted_header_7778_per_ofe_restrictive.sol`
   - `canonical_9002_double_quoted_policy.sol`
4. Fixture-integrity anchors:
   - `tests/fixtures/infile/soil/fixtures.sha256`
   - `tests/fixtures/infile/soil/fixtures.provenance.yaml`
   - provenance keys: `source_repo`, `source_commit`, `source_path`,
     `source_sha256`, `transform_note`
