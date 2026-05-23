# PL13 Process Contract Authority

Status: `complete`
Evidence mode: `Static`

## Canonical Authority

PL13 growth transition execution authority is defined by canonical science
contracts, not by package notes:

- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
  - `contract_version: 5`
  - Added PL13 scheduler growth-transition state-domain/reset authority with
    invariants `INV-RESIDUE-014` and `INV-RESIDUE-015`.
- `docs/specifications/science-contracts/index.md`
  - Updated notes for `SC-RESIDUE-001` to record PL12+PL13 transition authority.

## Governing PL13 Authority Statements

Static:

1. Scheduler growth dispatch consumes projected PL controls through typed
   context assembly and deterministic annual/perennial action selection.
2. Growth transition payload assembly enforces explicit state-domain guards for
   `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, and `hia`.
3. Reset-class actions emit explicit zero-state post-transition payloads; no
   implicit defaulting/clamping is permitted.
4. Invalid growth transition control/state domains are typed hard failures.

## Upstream/Dependency Authority

- Runtime-projected management controls are anchored in
  `SC-INFILE-MANAGEMENT-001` and PL11 schedule/growth projection authority.
- Kernel-governance profile requirements are enforced through:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
