# Contract Implementation Evidence

Status: complete

Evidence mode: static

Static: canonical HPHYS0251 authority was added before production code edits.

## Contract Amendments

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`: bumped to
  `contract_version: 12`; added `INV-EVAP-017`, guard-map row, required output
  layer `UPi_####`/`Ui_####` surfaces, HPHYS0251 SWU uptake-magnitude addendum,
  revised `GAP-EVAP-006`, and revision-history entry.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`: bumped to
  `contract_version: 78`; added `INV-WATBAL-039`, guard-map row, HPHYS0251
  WB17/WB13 uptake/storage coupling addendum, and revision-history entry.
- `docs/specifications/science-contracts/index.md`: added registry references
  for `SC-EVAP-001#INV-EVAP-017` and `SC-WATBAL-001#INV-WATBAL-039`.

## Authority Trace

- Static: baseline process authority remains
  `/workdir/wepp-forest_260430_baseline/src/swu.for` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Static: the contract text encodes `swu.for:122-191` uptake magnitude
  semantics: crop `pltol(itype)`, legacy tolerance normalization, cumulative
  root-depth `UPi`, actual layer `Ui`, final `Ep=ΣUi`, and `Ws=ΣUi/Etp`.
