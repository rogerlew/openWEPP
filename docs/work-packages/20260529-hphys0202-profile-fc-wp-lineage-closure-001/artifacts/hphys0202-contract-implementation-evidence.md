# HPHYS0202 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Canonical contract updates
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Added `HPHYS0202 ProfileFC/ProfileWP Layer-Aggregation Lineage Closure`.
  - Clarified that FC/WP adapter seed symbols are non-authoritative for WB13
    publication.
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - Extended HPARITY02 seed addendum with HPHYS0202 authority split:
    FC/WP seed symbols are diagnostic carry surfaces; publication is
    layer-authoritative aggregation.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - Added explicit WB13 coupling requirement that FC/WP publications are
    `Σ(thetfc_i * dg_i)` / `Σ(thetdr_i * dg_i)` (`mm`).
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - Added HPHYS0202 WB13 profile FC/WP publication-lineage addendum.
  - Clarified system variable-table lineage wording for profile columns.
- `docs/specifications/science-contracts/index.md`
  - Updated contract registry notes for SC-PERC-001, SC-SOIL-001,
    SC-SYSTEM-001, and SC-WATBAL-001 with HPHYS0202 lineage closure posture.

## Version-trace note
- Static: revision-history updates were recorded in each amended canonical
  `SC-*` contract.
