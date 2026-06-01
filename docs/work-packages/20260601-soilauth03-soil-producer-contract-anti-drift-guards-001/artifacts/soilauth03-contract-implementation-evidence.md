# SOILAUTH03 Contract Implementation Evidence

Status: complete  
Evidence mode: Static

## Scope
Contract/spec authority updates completed before final guard-test closure.

## Updated Authority Surfaces
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
  - added explicit anti-drift suite authority linkage:
    `cas_l4_infile_soil_producer_contract_001`.
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
  - bumped contract version to `0.1.10`;
  - added evidence anchor `E-AUTH-SOL-01`;
  - added guard `G-SOL-014` for required producer-contract anti-drift
    obligations (required/hard-fail lane).
- `docs/specifications/external-authority/suites/cas_l4_infile_soil_producer_contract_001.md`
  - new Level-4 suite definition for `.sol` producer-contract anti-drift.
- `docs/specifications/external-authority/registry.yaml`
  - registered suite in required/hard-fail lane.
- `docs/specifications/external-authority/required-suite-obligations.json`
  - added machine-readable symbol/order/arity obligations and required fixture
    anchors for SOILAUTH03.
