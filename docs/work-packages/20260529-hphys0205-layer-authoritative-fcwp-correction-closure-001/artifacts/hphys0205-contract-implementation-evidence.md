# HPHYS0205 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Canonical contract updates
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - Added HPHYS0205 authority requirement: authoritative runtime
    `thetfc_####`/`thetdr_####` symbols must carry corrected moisture lineage,
    not raw parser-theta lineage when corrected lineage is available.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Added HPHYS0205 corrected-layer authority closure addendum and WB13
    publication/reconciliation requirements.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - Added explicit WB13 FC/WP coupling requirement that authoritative layer
    symbols are corrected-lineage in corrected-lane contexts.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - Added system-boundary corrected-layer projection addendum and typed
    reconciliation posture.
- `docs/specifications/science-contracts/index.md`
  - Updated registry notes for SC-SOIL-001, SC-WATBAL-001, SC-PERC-001,
    and SC-SYSTEM-001 with HPHYS0205 lineage authority scope.

## Revision trace note
- Static: revision-history rows were added in each amended canonical `SC-*`
  contract for HPHYS0205.
