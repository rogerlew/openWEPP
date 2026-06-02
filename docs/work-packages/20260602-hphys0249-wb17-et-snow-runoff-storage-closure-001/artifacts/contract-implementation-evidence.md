# Contract Implementation Evidence

Status: complete

Evidence mode: static

Static:

- Amended `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  to version `10` with `INV-EVAP-015`, requiring WB17 `Ep`/`Es` production to
  mutate `wb18_perc_theta_####` as the runtime alias for baseline `st(i)`.
- Amended `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  to version `75` with `INV-WATBAL-037`, requiring post-WB17 aggregate
  `wb11_soil_water`/WB13 storage publication to recompute baseline
  `watcon = Σsoilw(i)`.
- Updated `docs/specifications/science-contracts/index.md` with HPHYS0249
  registry linkage for `SC-EVAP-001#INV-EVAP-015` and
  `SC-WATBAL-001#INV-WATBAL-037`.
