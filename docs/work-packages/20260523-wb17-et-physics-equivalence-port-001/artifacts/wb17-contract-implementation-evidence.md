# WB17 Contract Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Scope
Implemented canonical WB17 ET contract amendments to replace WB11 surrogate ET
authority with equation-driven ET partition semantics and explicit WB17 runtime
alias mapping.

## Contract Files Amended
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`

## WB17 Contract Changes
- Replaced WB11 surrogate ET algorithm authority in `SC-EVAP-001` with WB17
  equation-vector semantics:
  - `Esp = Eu * exp(-0.4 * L)`
  - `Etp = Eu - Esp`
  - residue partition `Er = min(Esp, wb17_residue_interception)`
  - component/closure outputs `Es`, `Ep`, `ET`, `Ws`
- Added explicit WB17 runtime alias authority in `SC-EVAP-001`:
  - `Eu -> wb11_et_demand`
  - `L -> lai`
  - `Er -> wb17_residue_interception`
- Updated WB17 branch/guard, invariant, guard-map, and contract-test vector
  obligations in `SC-EVAP-001`.
- Updated hydrology execution authority in `SC-WATBAL-001` to WB17 ET + WB11
  percolation/lateral/drain composite lane posture.
- Updated registry notes in `docs/specifications/science-contracts/index.md`
  for WB17 ET authority closure context.

## Version Bumps
- `SC-EVAP-001`: `6 -> 7`
- `SC-WATBAL-001`: `17 -> 18`
