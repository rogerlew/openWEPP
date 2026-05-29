# HILLSTAB07 Contract Implementation Evidence

Status: complete  
Evidence mode: Static

## Contract Amendments Applied

1. `SC-RUNOFFPART-001` updated to `contract_version: 22`:
   - WB16 required surfaces now include provenance outputs:
     `wb16_ealpha_compatibility_seed_used`, `wb16_ealpha_seed_policy`.
   - WB16 rule set now encodes:
     - canonical `m=1.5` producer authority (`rdat.for` lineage),
     - canonical `ealpha` producer-chain lineage
       (`frcfac -> rdat(alpha) -> alphay -> eplane`),
     - explicit compatibility-seed governance obligations and warning id
       `SIMPIPE-W-003`.
   - Added non-promotable producer-migration gap row `GAP-RUNOFFPART-005`.

2. `SC-WATBAL-001` updated to `contract_version: 41`:
   - WB16 required coupling surfaces now include provenance outputs:
     `wb16_ealpha_compatibility_seed_used`, `wb16_ealpha_seed_policy`.
   - Added matching canonical `m`/`ealpha` producer authority and
     compatibility-seed governance obligations.
   - Added non-promotable producer-migration gap row `GAP-WATBAL-005`.

3. `docs/specifications/science-contracts/index.md` updated:
   - `SC-RUNOFFPART-001` and `SC-WATBAL-001` `last_reviewed` dates moved to
     `2026-05-29` with HILLSTAB07 note summary.
