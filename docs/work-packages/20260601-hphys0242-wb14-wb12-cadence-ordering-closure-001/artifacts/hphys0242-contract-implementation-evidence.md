# HPHYS0242 Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Static

- `SC-WATBAL-001` advanced to `contract_version: 71` and now encodes
  `INV-WATBAL-034`, which requires hourly WB14/WB12 tail cadence to observe
  same-pass WB14 infiltration lineage, final-hour ET, WB19 drainage-before-
  lateral tail outputs, saturation-runoff addback, and same-pass storage
  reconciliation.
- `SC-RUNOFFPART-001` advanced to `contract_version: 26` and now encodes
  `INV-RUNOFFPART-014`, including `surdra = Σui_SCrunf(ii)` as WB14 runoff
  addback authority.
- `SC-EVAP-001` advanced to `contract_version: 9` and now encodes
  `INV-EVAP-014`, requiring final-hour ET to observe same-pass WB14
  infiltration lineage instead of stale prior-pass storage.
- `SC-CLIMATE-001` advanced to `contract_version: 15` and now encodes
  `INV-CLIMATE-012`, preserving `ninten`/`nbrkpt` and `timem`/`intsty`
  forcing cadence aliases for hourly WB14 execution.
- `SC-PERC-001` advanced to `contract_version: 20` and now encodes
  `INV-PERC-012`, keeping WB18 hourly percolation before final-hour ET and the
  WB19 tail.
- `SC-SUBHYD-001` advanced to `contract_version: 24` and now encodes
  `INV-SUBHYD-023`, requiring WB19 hourly drainage-before-lateral execution,
  `ui_SCrunf(ii)` publication, and `ui_LfCrf(ii)` copy-forward authority.

## Authority Notes

- Baseline provenance was taken from `/workdir/wepp-forest_260430_baseline`
  at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Contract text preserves legacy WEPP symbols for the touched cadence and
  carry surfaces (`ui_LFtstp`, `ui_SCrunf`, `ui_LfCrf`, `surdra`, `F`,
  `ninten`/`nbrkpt`, `timem`/`intsty`) with openWEPP aliases where needed.
