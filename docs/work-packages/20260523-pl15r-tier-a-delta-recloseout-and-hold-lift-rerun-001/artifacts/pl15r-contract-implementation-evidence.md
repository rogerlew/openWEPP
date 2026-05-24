# PL15R Contract Implementation Evidence

Status: `complete`
Evidence mode: `Static`

## Implemented Contract Amendments

### `SC-SYSTEM-001`

- Contract version updated: `8 -> 9`.
- Added `INV-SYSTEM-015` (PL15R Tier-A recloseout supersession invariant):
  refreshed hold-lift governance must classify active blockers from the latest
  schema-aligned PL14R strict replay evidence set, with explicit risk-acceptance
  reference only when post-supersession unresolved blockers remain.
- Added corresponding rows in:
  - invariant guard map
  - boundary disposition map
  - revision history (`version 9`, `2026-05-23`)

### `SC-WATBAL-001`

- Contract version updated: `16 -> 17`.
- Added `INV-WATBAL-015` (schema-aligned replay supersession invariant):
  Tier-A `H5.wat.dat` residual classification must evaluate canonical 25-column
  strict replay plus keyed day-by-day parity before retaining blockers.
- Added corresponding rows in:
  - invariant guard map
  - boundary disposition map
  - WB13 contract-test vectors
  - revision history (`version 17`, `2026-05-23`)

### `science-contracts/index.md`

- Updated notes for `SC-SYSTEM-001` and `SC-WATBAL-001` to include PL15R
  supersession-governance amendments (`INV-SYSTEM-015`, `INV-WATBAL-015`).

## Sequencing Conformance

- Contract amendments were implemented before PL15R decision artifacts and before
  any closeout decision-surface updates.
