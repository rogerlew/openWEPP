# SIMIMPL28 Contract Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Canonical contract authority was amended before production edits.
- `SC-CLIMATE-001` advanced to `contract_version: 12` and added SIMIMPL28
  hourly forcing synthesis authority for `sunmap`, `radcur`, `hr_tmp`, and
  `stmtim` lineage plus required `winter.hourly.*` and `snow.hourly.*`
  forcing symbols.
- `SC-SNOWFREEZE-001` advanced to `contract_version: 7` and split staged
  closure between SIMIMPL28 forcing emission scope and SIMIMPL29 kernel-state
  hourly families.
- `docs/specifications/science-contracts/index.md` was updated to register
  SIMIMPL28 authority additions.

## Ran
- `rg -n "contract_version: 12|SIMIMPL28 Hourly Winter Forcing Synthesis Addendum|SIMIMPL28 Contract-Test Vectors" docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `rg -n "contract_version: 7|SIMIMPL28 Forcing-Emission Scope Clarification|GAP-SNOWFREEZE-002" docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `rg -n "SC-CLIMATE-001|SC-SNOWFREEZE-001" docs/specifications/science-contracts/index.md`
