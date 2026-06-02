# Contract Implementation Evidence

Status: complete

Evidence mode: static

Static:

- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  contract version `26` adds `INV-SUBHYD-025` for WB19 lateral
  `fzdrfc(i) = max(drfc(i)-frzw(i),0)` capacity and withdrawal floors.
- `SC-SUBHYD-001` keeps hourly conductivity authority on unfrozen `drfc(i)`
  `fffx` weighting, matching pinned `watbal_hourly.for:695-717`.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  contract version `79` adds `INV-WATBAL-040`, tying the WB19 frozen-adjusted
  storage lineage to WB17 storage availability and WB13 aggregate publication.
- `docs/specifications/science-contracts/index.md` adds the HPHYS0252 registry
  note for `SC-SUBHYD-001#INV-SUBHYD-025` and
  `SC-WATBAL-001#INV-WATBAL-040`.
