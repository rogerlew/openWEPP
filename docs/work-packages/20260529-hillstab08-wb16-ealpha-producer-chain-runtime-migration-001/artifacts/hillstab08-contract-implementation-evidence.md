# HILLSTAB08 Contract Implementation Evidence

Status: complete  
Evidence mode: static

## Scope
- WB16 `ealpha` producer-chain authority migration closure across:
  - `SC-RUNOFFPART-001`
  - `SC-WATBAL-001`
  - science-contract registry index notes

## Static Evidence
- Updated `SC-RUNOFFPART-001`:
  - `contract_version` advanced `22 -> 23`.
  - WB16 rules now require runtime-produced provenance
    (`wb16_ealpha_compatibility_seed_used=false`,
    `wb16_ealpha_seed_policy=runtime_provided`) when producer inputs are
    present.
  - Compatibility path retained as explicit degradation branch with
    `SIMPIPE-W-003` warning semantics.
  - Added runtime-producer contract-test vector and retained compatibility
    vector.
  - Dispositioned `GAP-RUNOFFPART-005` to `closed`.
- Updated `SC-WATBAL-001`:
  - `contract_version` advanced `41 -> 42`.
  - Mirrored WB16 runtime-producer/compatibility provenance requirements and
    vector obligations.
  - Dispositioned `GAP-WATBAL-005` to `closed`.
- Updated `docs/specifications/science-contracts/index.md` notes for
  `SC-RUNOFFPART-001` and `SC-WATBAL-001` to record HILLSTAB08 closure.
