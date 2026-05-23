# PL13 SC Contract Amendment Plan

Status: `complete`
Evidence mode: `Static`

## Planned Amendments

1. Extend `SC-RESIDUE-001` with explicit PL13 growth-transition authority,
   growth state-domain invariants, and reset payload obligations.
2. Reconcile science-contract registry notes in
   `docs/specifications/science-contracts/index.md`.

## Executed Amendments

- `SC-RESIDUE-001`
  - `contract_version` updated from `4` to `5`.
  - Expanded scheduler transition sections to include PL13 growth authority:
    - growth transition inputs/outputs
    - growth transition algorithm steps
    - PL13 branch/guard rows
  - Added invariants and guard-map entries:
    - `INV-RESIDUE-014`
    - `INV-RESIDUE-015`
  - Added PL13-specific test-vector obligations.
  - Added revision-history entry for version `5`.

- `science-contracts/index.md`
  - Updated `SC-RESIDUE-001` notes to include PL12+PL13 transition authority.

## Amendment Disposition

All planned PL13 contract-authority amendments were completed before declaring
PL13 implementation complete.
