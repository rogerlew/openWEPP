# PL14 Contract Implementation Evidence

Status: `complete`
Evidence mode: `Static`

## Canonical PL14 Contract Amendments

Implemented required PL14 replay authority/guard behavior in canonical
science-contract files:

- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `contract_version: 3 -> 4`
  - Added `INV-SYSTEM-012` (strict replay artifact/provenance completeness).
  - Added guard-map and boundary-disposition authority for PL14 replay staging.
  - Added strict Tier-A lane tolerance authority:
    `TOL-SYSTEM-006 (abs_tol=0, rel_tol=0)`.

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `contract_version: 7 -> 8`
  - Added `INV-WATBAL-012` (WB13 replay-candidate schema/order/artifact
    completeness invariant).
  - Added invariant guard-map row and boundary-disposition row for PL14 replay
    staging failures.
  - Extended WB13 contract-test vectors with explicit no-fallback replay
    artifact rule.

- `docs/specifications/science-contracts/index.md`
  - Updated registry notes for `SC-SYSTEM-001` and `SC-WATBAL-001` to record
    PL14 contract authority changes.
