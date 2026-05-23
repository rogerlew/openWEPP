# WB15 Contract Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Scope
Implemented canonical WB15 contract amendments for canopy interception coupling,
including typed guard posture and explicit runoff/storage closure equations.

## Contract Files Amended
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/index.md`

## WB15 Contract Changes
- Added WB15 canopy-interception coupling addendum to `SC-WATBAL-001`:
  - required inputs `cancov`, `lai`, `vdmt`,
  - Eq. [5.1.2] lineage interception form,
  - explicit runoff/storage equation coupling with `I`,
  - typed guard mapping for runoff/storage phases,
  - WB15 test-vector obligations.
- Added WB15 interception-runtime addendum to `SC-RUNOFFPART-001`:
  interception-before-infiltration behavior and runoff reconciliation updates.
- Added WB15 ET-coupling addendum to `SC-EVAP-001`:
  interception `I` remains distinct from ET outputs.
- Added WB15 producer obligations to `SC-PLANT-001`:
  required finite/domain-valid canopy driver payloads.
- Updated science-contract registry notes in
  `docs/specifications/science-contracts/index.md`.

## Version Bumps
- `SC-WATBAL-001`: `11 -> 12`
- `SC-RUNOFFPART-001`: `7 -> 8`
- `SC-EVAP-001`: `5 -> 6`
- `SC-PLANT-001`: `10 -> 11`
