# SIMIMPL29 Contract Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Canonical contract authority was amended before final SIMIMPL29 closure
  disposition.
- `SC-SNOWFREEZE-001` advanced to `contract_version: 8` and added explicit
  SIMIMPL29 authority for baseline snow kernel (`snowd`/`melt`) hourly state
  publication, runtime carry-state persistence, and typed active-hourly symbol
  failure posture.
- `docs/specifications/science-contracts/index.md` was updated to register the
  SIMIMPL29 amendment summary and residual `frost.hourly.*` follow-on scope.

## Ran
- `rg -n "contract_version: 8|SIMIMPL29 Snow Kernel Port and Hourly State Closure|GAP-SNOWFREEZE-002|GAP-SNOWFREEZE-005|2026-05-25.*\\| 8 \\|" docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `rg -n "SC-SNOWFREEZE-001.*SIMIMPL29" docs/specifications/science-contracts/index.md`
