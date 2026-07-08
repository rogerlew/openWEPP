# Publication Operand Lineage

Status: `passed`

Evidence mode: `Static:` source review plus `Ran:` W7R reconstruction.

W7R does not change sediment-sensitive publication formulas, schemas, units, or
normalization bases. The accepted fixture proves the existing lineage on
nonzero sediment.

Summary:

- `total_detachment_kg`: generated HBP latest event payload, published as
  `totalwatsed3.tdet`.
- `total_deposition_kg`: generated HBP latest event payload, published as
  `totalwatsed3.tdep`.
- `sediment_yield_kg`: typed routed channel state, published as
  `ebe_pw0.sediment_yield` and `totalwatsed3.sed_del`.
- `runoff_volume_m3`: typed routed channel state, published as `runvol`,
  `ebe_pw0.runoff_volume`, and `chanwb.Inflow`.

Detailed lineage and rejected aliases are in `operand-lineage.md`.
