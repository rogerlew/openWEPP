# PL12 SC Contract Amendment Plan

Status: `complete`
Evidence mode: `Static`

## Planned Amendments

1. Extend `SC-PLANT-001` with explicit PL12 decomposition-transition dispatch
   authority and invariant coverage.
2. Extend `SC-RESIDUE-001` with kernel-profile-complete PL12 scheduler
   decomposition transition sections.
3. Reconcile science-contract registry notes in
   `docs/specifications/science-contracts/index.md`.

## Executed Amendments

- `SC-PLANT-001`
  - `contract_version` updated from `5` to `6`.
  - Added PL12 scheduler decomposition-transition algorithm language.
  - Added `INV-PLANT-016` and invariant guard-map row for deterministic typed
    decomposition dispatch semantics.
  - Added revision-history entry for version `6`.

- `SC-RESIDUE-001`
  - `contract_version` updated from `3` to `4`.
  - `last_reviewed` updated to `2026-05-23`.
  - Added required kernel-profile sections for PL12:
    - algorithm state surfaces
    - algorithm specification
    - branch/guard table
    - constants/parameters table
    - test-vector obligations
  - Added revision-history entry for version `4`.

- `science-contracts/index.md`
  - Updated `SC-PLANT-001` notes to include PL12 `INV-PLANT-016` authority.
  - Updated `SC-RESIDUE-001` notes and review date for PL12 additions.

## Amendment Disposition

All planned PL12 contract-authority amendments were completed before declaring
PL12 implementation complete.
