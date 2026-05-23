# WB13 Contract Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Scope
Implemented WB13 canonical authority amendments for comparator-ready daily
water-balance output-surface behavior (`H5.wat.dat` equivalent).

## Contract Files Amended
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/index.md`

## WB13 Contract Changes
- Added WB13 output-surface authority for canonical 25-column schema and
  deterministic row-ordering key `(Y, J, OFE)`.
- Added WB13 output guard-code authority:
  - `HKERNEL-WB13-HWAT-E-001` (missing)
  - `HKERNEL-WB13-HWAT-E-002` (non-finite)
  - `HKERNEL-WB13-HWAT-E-003` (domain/order/schema)
- Added WB13 cross-contract coupling addenda for ET (`Ep/Es/Er`), percolation
  and profile storage (`Dp`, profile storage fields), runoff/runon (`Q/QOFE`,
  `UpStrmQ`, `RM`, `P`), and subsurface/drainage (`latqcc`, `Tile`, `SubRIn`).
- Added WB13 contract-derived vector obligations.

## Version Bumps
- `SC-WATBAL-001`: `5 -> 6`
- `SC-EVAP-001`: `4 -> 5`
- `SC-PERC-001`: `4 -> 5`
- `SC-RUNOFFPART-001`: `3 -> 4`
- `SC-SUBHYD-001`: `5 -> 6`
