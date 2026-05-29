# hillstab06-contract-implementation-evidence

Status: complete  
Evidence mode: Static

## Canonical Contract Amendments
- Updated `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  from `v20` to `v21` (`last_reviewed: 2026-05-29`):
  - added baseline-compatible WB16 near-zero runoff branch semantics
    (`Q < 1.0e-8`),
  - added floor-first peak canonicalization posture and positivity-domain
    guard intent for near-zero intermediates.
- Updated `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  from `v38` to `v40` (`last_reviewed: 2026-05-29`):
  - aligned WB16 addendum with near-zero runoff compatibility branch,
  - added contract-test vector for near-zero positive runoff handling.
- Updated `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  from `v13` to `v14` (`last_reviewed: 2026-05-29`):
  - added `INV-CLIMATE-011` (finite `tmax/tmin` valid even when `tmax < tmin`),
  - added CLIM18 daily thermal-inversion compatibility addendum.

## Provenance Notes
- WB16 behavior authority traced to baseline `appmth.for` near-zero runoff path
  and peak-floor canonicalization semantics.
- Climate inversion compatibility authority anchored to baseline `hr_tmp.for`
  treatment where ordering-only rejection is not authoritative.

## Contract-First Compliance
- Canonical `SC-*` authority was amended before any production runtime/kernel
  edits.
