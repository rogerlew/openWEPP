# Contract Implementation Evidence

Status: complete

Evidence mode: Static

Static:

- `SC-CLIMATE-001` bumped to contract version `21` and added
  `INV-CLIMATE-017` plus `OBL-CLIMATE-P-012`.
- `SC-SNOWFREEZE-001` bumped to contract version `50` and added
  `INV-SNOWFREEZE-045` plus `OBL-SNOWFREEZE-P-024`.
- `SC-WATBAL-001` bumped to contract version `143` and added
  `INV-WATBAL-093` plus `OBL-WATBAL-P-029`.
- `docs/specifications/science-contracts/index.md` now lists the HPHYS0319
  cross-contract authority.
- The contract authority requires fixed-baseline `stmtim` observe recovery
  paired with regenerated OpenWEPP `stmtim` traces before producer or
  water-balance ownership changes.
