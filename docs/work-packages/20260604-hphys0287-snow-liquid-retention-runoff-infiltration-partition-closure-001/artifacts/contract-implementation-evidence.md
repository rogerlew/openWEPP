# Contract Implementation Evidence

Status: complete
Evidence mode: Static

Static:
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-020` defines projected runtime snow state as a fail-closed domain surface before active/inactive snow branch selection and WB12/WB14 liquid partition.
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-020` now explicitly requires a complete runtime snow-state vector whenever any snow option/control/runtime state is projected; no-projection/no-snow compatibility is only allowed when none of those snow surfaces are present.
- `SC-RUNOFFPART-001#INV-RUNOFFPART-017` mirrors the WB12/WB14 partition guard and complete-vector requirement.
- `SC-WATBAL-001#INV-WATBAL-062` carries the same fail-closed publication invariant for WB13 `RM`, `Snow-Water`, `Total-Soil`, and `SoilWaterTotal` closure.

Disposition:
- Contract authority rejects canonicalize-and-proceed for material negative, non-finite, or partial projected runtime snow state.
- Valid no-snow/no-projection compatibility remains explicit and tested.
