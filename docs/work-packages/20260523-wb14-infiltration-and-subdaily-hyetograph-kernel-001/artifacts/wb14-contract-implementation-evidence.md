# WB14 Contract Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Scope
Implemented WB14 canonical authority amendments for production infiltration and
subdaily hyetograph-driven runoff reconciliation.

## Contract Files Amended
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/index.md`

## WB14 Contract Changes
- Added WB14 infiltration + hyetograph authority addendum to
  `SC-RUNOFFPART-001` with:
  - Green-Ampt lineage deterministic branch rules,
  - explicit hyetograph interval semantics (`timem_####`, `intsty_####`),
  - typed guard codes `HKERNEL-WB14-RUNOFF-E-001..003`,
  - WB14 contract-derived vectors.
- Added WB14 coupling addendum to `SC-WATBAL-001` declaring computed
  infiltration authority in runoff reconciliation and deprecating externally
  seeded infiltration as required acceptance-path input.
- Added WB14 hyetograph forcing/coupling requirements to `SC-CLIMATE-001` for
  runoff-consumer payload validity.
- Updated science-contract registry notes and review date alignment in
  `docs/specifications/science-contracts/index.md`.

## Version Bumps
- `SC-RUNOFFPART-001`: `4 -> 5`
- `SC-WATBAL-001`: `8 -> 9`
- `SC-CLIMATE-001`: `3 -> 4`
