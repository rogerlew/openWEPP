# simimpl10-contract-implementation-evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Reviewed canonical authority for SIMIMPL10 scope:
  - `SC-WATBAL-001` CLIM05/CLIM06 coupling addenda and `INV-WATBAL-007/013`.
  - `SC-RUNOFFPART-001` CLIM05/CLIM06 runtime coupling addenda and `INV-RUNOFFPART-009` boundary posture.
  - `SC-SNOWFREEZE-001` and `SC-SOIL-001` frozen-soil boundary/domain rules.
- Determination: no new canonical `SC-*` amendment was required for SIMIMPL10; required winter/soil/frsoil/hydout-equivalent authority already exists and is implementation-ready.
- Implementation guard family selected for runner coupling closure surface: `HS-SIMCOUP-E-001`.

## Ran
- Authority scan commands executed:
  - `rg -n "winter|frsoil|hydout|infcap|runtime_ws_frz|INV-WATBAL-007|INV-WATBAL-013" docs/specifications/science-contracts/contracts/*.md`
  - `sed -n '400,560p' docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `sed -n '450,580p' docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
