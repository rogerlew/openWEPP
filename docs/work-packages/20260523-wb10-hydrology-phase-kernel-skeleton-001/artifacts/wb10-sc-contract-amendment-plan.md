# WB10 SC Contract Amendment Plan

Status: `complete`
Evidence mode: `Static`

## Planned Amendments

1. Extend `SC-WATBAL-001` with explicit WB10 runoff/storage phase-entry routing
   authority and typed unsupported-class guard posture.
2. Extend `SC-EVAP-001` with explicit WB10 ET phase-entry routing authority and
   typed unsupported-class guard posture.
3. Extend `SC-PERC-001` with explicit WB10 percolation phase-entry routing
   authority and typed unsupported-class guard posture.
4. Extend `SC-SUBHYD-001` with explicit WB10 lateral/drainage phase-entry
   routing authority and typed unsupported-class guard posture.
5. Reconcile science-contract registry notes in
   `docs/specifications/science-contracts/index.md`.

## Executed Amendments

- `SC-WATBAL-001`
  - `contract_version` updated from `2` to `3`.
  - `last_reviewed` updated to `2026-05-23`.
  - Added required WB10 sections:
    - algorithm state surfaces,
    - algorithm specification,
    - branch and guard table,
    - constants and parameters table,
    - test-vector obligations.
  - Added routing invariants `INV-WATBAL-009` and `INV-WATBAL-010`.
  - Added revision-history row for version `3`.

- `SC-EVAP-001`
  - `contract_version` updated from `2` to `3`.
  - `last_reviewed` updated to `2026-05-23`.
  - Added required WB10 sections (state surfaces/specification/branch+guard/
    constants/test vectors).
  - Added routing invariants `INV-EVAP-011` and `INV-EVAP-012`.
  - Added revision-history row for version `3`.

- `SC-PERC-001`
  - `contract_version` updated from `2` to `3`.
  - `last_reviewed` updated to `2026-05-23`.
  - Added required WB10 sections (state surfaces/specification/branch+guard/
    constants/test vectors).
  - Added routing invariants `INV-PERC-010` and `INV-PERC-011`.
  - Added revision-history row for version `3`.

- `SC-SUBHYD-001`
  - `contract_version` updated from `2` to `3`.
  - `last_reviewed` updated to `2026-05-23`.
  - Added required WB10 sections (state surfaces/specification/branch+guard/
    constants/test vectors).
  - Added routing invariants `INV-SUBHYD-012`, `INV-SUBHYD-013`,
    `INV-SUBHYD-014`.
  - Added revision-history row for version `3`.

- `science-contracts/index.md`
  - Updated `SC-WATBAL-001`, `SC-EVAP-001`, `SC-PERC-001`, `SC-SUBHYD-001`
    `last_reviewed` fields and notes for WB10 phase-entry routing authority.

## Amendment Disposition

All planned WB10 contract-authority amendments were completed before WB10
implementation closeout.
