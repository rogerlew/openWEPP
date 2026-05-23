# PL12 Process Contract Authority

Status: `complete`
Evidence mode: `Static`

## Canonical Authority

PL12 decomposition/residue transition execution authority is defined by canonical
science contracts, not by package notes:

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
  - `contract_version: 6`
  - Added PL12 scheduler decomposition-transition dispatch authority and
    `INV-PLANT-016` (deterministic typed transition selector semantics).
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
  - `contract_version: 4`
  - Added PL12 scheduler decomposition transition algorithm sections, guard
    table, constants table, and test-vector obligations.
- `docs/specifications/science-contracts/index.md`
  - Updated registry notes and review dates for `SC-PLANT-001` and
    `SC-RESIDUE-001` to reflect PL12 authority updates.

## Governing PL12 Authority Statements

Static:

1. Scheduler decomposition dispatch consumes projected PL11 annual/perennial
   transition-control payload families through typed context assembly.
2. Active-day decomposition transition selection is deterministic for annual and
   perennial branches.
3. Invalid payload domains, indexed-family closure violations, and invalid
   grazing windows are typed hard failures.
4. Silent defaults and clamping for invalid decomposition-transition state are
   prohibited.

## Upstream/Dependency Authority

- Runtime-projected symbol families and management domains are anchored in
  `SC-INFILE-MANAGEMENT-001` and PL11 projection authority carried in
  `SC-PLANT-001`.
- Kernel-governance profile requirements are enforced through:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
