# INT10 Contract Implementation Evidence

Status: `complete`
Evidence mode: `Static`

## Canonical INT10 Contract Amendments

Implemented required INT10 coupled ordering/state-transfer authority in canonical
science-contract files:

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
  - `contract_version: 7 -> 8`
  - Added `INV-PLANT-017` and guard-map row for explicit
    `decomp -> growth -> watbal` lane ordering with typed hard-fail posture.
  - Added INT10 test-vector obligations.

- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
  - `contract_version: 5 -> 6`
  - Added INT10 coupled replay closure rule in algorithm authority,
    `BR-RES-INT10-ORDER`, `INV-RESIDUE-016`, and guard-map row.
  - Added INT10 ordering/state-transfer test-vector obligations.

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `contract_version: 6 -> 7`
  - Added coupled lane-entry invariant `INV-WATBAL-011` and guard-map row
    requiring valid plant-lane ordering preconditions before watbal completion.
  - Added INT10 coupled replay vectors under test-vector obligations.

- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `contract_version: 2 -> 3`
  - Added cross-lane publication invariant `INV-SYSTEM-011` and guard/disposition
    authority requiring successful coupled lane closure before system publish.

- `docs/specifications/science-contracts/index.md`
  - Updated registry notes for `SC-PLANT-001`, `SC-RESIDUE-001`,
    `SC-WATBAL-001`, and `SC-SYSTEM-001` to record INT10 authority updates.
