# Contract Implementation Evidence

Status: complete
Evidence mode: static

Static: amended canonical contracts

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Added `INV-WATBAL-042`.
  - Added `HPHYS0255 MOFE Storage Projection Addendum`.
  - Added revision history entry `81`.
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - Added `INV-SOIL-016`.
  - Added `HPHYS0255 MOFE Storage-Scope Addendum`.
  - Added revision history entry `22`.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - Added `INV-SYSTEM-029`.
  - Added `HPHYS0255 MOFE Storage-Lineage Publication Addendum`.
  - Added revision history entry `78`.

Static: contract decision

- MOFE `Area` may aggregate contributor geometry under MOFE04.
- Storage fields remain simulation-owned WB11/WB13 runtime lineage under the
  current `single-runtime-wb11-state` policy.
- Static area-weighted storage synthesis from OFE soil rows remains
  non-authoritative.
