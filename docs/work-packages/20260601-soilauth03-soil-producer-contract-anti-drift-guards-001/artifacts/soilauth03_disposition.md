# SOILAUTH03 Disposition

Status: complete  
Evidence mode: Static + Ran  
Decision: GO

## Scope
SOILAUTH03 objective is satisfied for declared anti-drift scope.

## Closure Summary
- Added required/hard-fail external-authority suite:
  `cas_l4_infile_soil_producer_contract_001`.
- Added machine-readable obligations for canonical `.sol`:
  - symbol presence,
  - header/policy/layer arity + order envelopes,
  - required case bindings.
- Added contract-derived integration lane:
  `soilauth03_soil_producer_contract_drift_guards_contract`.
- Added/updated locked fixture integrity surfaces for SOILAUTH03:
  - `tests/fixtures/infile/soil/fixtures.sha256`
  - `tests/fixtures/infile/soil/fixtures.provenance.yaml`
  - `tests/fixtures/infile/soil/soilauth03_guard_cases.json`
- Updated producer/parser authority docs and release runbook/readme to codify
  required hard-fail posture.

## Residual Notes
- Full workspace test gate still fails in unrelated pre-existing suite:
  `auth05_level4_constitutive_authority_hardening_contract`
  (`thetfc_0001` mismatch on `valid_9002`).
