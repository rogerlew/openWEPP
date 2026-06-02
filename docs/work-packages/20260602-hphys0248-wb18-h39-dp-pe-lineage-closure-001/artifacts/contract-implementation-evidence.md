# Contract Implementation Evidence

Status: completed

Evidence mode: Static

Static:
- Amended `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  from contract version `21` to `22`.
- Added `REF-PERC-LEGACY-HOURLY-BOTK` and `INV-PERC-014`, requiring hourly
  bottom-layer restrictive conductivity to follow pinned baseline
  `perc.for`/`purk.for` lineage:
  `Ksi_eff = (dg_i + ui_bdrkth)/(dg_i/Ksi + ui_bdrkth/kslast)`.
- Amended `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  from contract version `73` to `74`.
- Added `INV-WATBAL-036` so H39 hourly `Dp`/`Pe` closure evidence must use
  `SC-PERC-001#INV-PERC-014` and remain `HOLD` if unrestricted bottom `Ksi`,
  daily-only harmonic conductivity, or missing `ui_bdrkth` is used.
