# Contract Implementation Evidence

Status: completed

Evidence mode: static

Static:

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` advanced to
  contract version `17`.
- Added `REF-EVAP-LEGACY-PMET-DEMAND` for
  `/workdir/wepp-forest_260430_baseline/src/evappm.for:181-388`.
- Added `REF-EVAP-LEGACY-SUNMAP-RADPOT` for
  `/workdir/wepp-forest_260430_baseline/src/sunmap.for:181-234`.
- Added `INV-EVAP-021` requiring PMET-mode WB11 demand seeding to publish the
  migrated `evappm.for` demand subset and label branch `evappm_pmet`.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` advanced
  to contract version `89` with `INV-WATBAL-049`, tying WB13 `Ep` and storage
  residual claims to migrated `SC-EVAP-001#INV-EVAP-021` evidence.
- The package used existing `SC-PLANT-001` canopy-height authority and
  `SC-CLIMATE-001` climate-forcing authority for runtime seam publication of
  `canhgt`, `deglat`, and `elevm`.

Scope note:

- HPHYS0263 authorizes the WB11 PMET demand-seed subset. It does not close full
  `evappm.for` routine migration because post-ET soil evaporation
  redistribution in `evappm.for:391-454` remains unported.
