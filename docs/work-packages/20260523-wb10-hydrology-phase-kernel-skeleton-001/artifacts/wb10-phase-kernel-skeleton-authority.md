# WB10 Phase-Kernel Skeleton Authority

Status: `complete`
Evidence mode: `Static`

## Canonical Authority

WB10 hydrology phase-entry scaffolding authority is defined by canonical
science contracts, not by package notes:

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `contract_version: 3`
  - Added WB10 runoff/storage phase-entry routing authority and invariants
    `INV-WATBAL-009` and `INV-WATBAL-010`.
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  - `contract_version: 3`
  - Added WB10 ET phase-entry routing authority and invariants
    `INV-EVAP-011` and `INV-EVAP-012`.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - `contract_version: 3`
  - Added WB10 percolation phase-entry routing authority and invariants
    `INV-PERC-010` and `INV-PERC-011`.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `contract_version: 3`
  - Added WB10 lateral/drainage phase-entry routing authority and invariants
    `INV-SUBHYD-012`, `INV-SUBHYD-013`, and `INV-SUBHYD-014`.
- `docs/specifications/science-contracts/index.md`
  - Updated registry notes and review dates (`2026-05-23`) for the four WB10
    touched contracts.

## Governing WB10 Authority Statements

Static:

1. Scheduler hydrology phases for ET/percolation/lateral/drainage/runoff/
   storage route through explicit typed hydrology phase classes.
2. Unsupported or mismatched scheduler phase-class combinations are invalid
   runtime states and must hard-fail with typed status code `HS-HYDRO-E-001`.
3. Generic hydrology class is reserved to scheduler generic hydrology phases
   (`normalization`, `storage_bounds`, `closure_diagnostics`).
4. Silent class reassignment/defaulting/clamping is prohibited.

## Governance and Procedure Anchors

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
