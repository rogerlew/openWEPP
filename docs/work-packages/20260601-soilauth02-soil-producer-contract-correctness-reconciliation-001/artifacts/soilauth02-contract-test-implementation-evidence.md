# SOILAUTH02 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: Static + Ran

## Scope
Contract-derived test coverage added/updated for canonical producer envelopes.

## Added/Updated Test Surfaces
Static:
- Updated `tests/integration/infile_soil_parser_contract.rs`:
  - strict-mode acceptance assertions for canonical quoted/policy-first/per-OFE
    restrictive envelopes.
- Added `tests/integration/soilauth02_soil_producer_reconciliation_contract.rs`:
  - canonical `9002` policy-first + omitted `avke` parses in strict+compat;
  - canonical `7778` per-OFE restrictive-row normalization parses in
    strict+compat;
  - canonical double-quoted policy tokens parse in strict+compat.
- Added fixture:
  - `tests/fixtures/infile/soil/canonical_9002_double_quoted_policy.sol`

Ran:
- `cargo test --test infile_soil_parser_contract --test soilauth02_soil_producer_reconciliation_contract`
  - result: pass (`14 + 3` tests).
