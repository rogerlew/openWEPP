# HPHYS0249 Disposition

Status: HOLD

Evidence mode: static + ran

Decision:

- HOLD. The package completed the targeted contract-first WB17 correction and
  passed final gates, but full `H1..H39` semantic parity remains `0/39`.

What closed:

- WB17 soil evaporation now mutates layer storage before aggregate writeback.
- Residue-capped evaporation restores un-evaporated residue to top-layer
  storage and clears `wb17_residue_interception`.
- WB17 root uptake now executes as `PlantRootUptake` after WB19
  drainage/lateral and before WB12/WB13 aggregate publication.
- Contract-derived WB17 tests pass `9/9`.
- Workspace gates pass.

What remains open:

| Family | Status | Evidence |
|---|---|---|
| `Ep` | open | `0/39`, mean abs mean `1.739422`, max abs `7.780000` |
| `Snow-Water` | open | `0/39`, mean abs mean `58.195696`, max abs `562.470000` |
| `RM` | open | `0/39`, mean abs mean `2.301802`, max abs `204.850510` |
| `Q` | open | `0/39`, mean abs mean `0.925027`, max abs `194.715728` |
| `Total-Soil` / `SoilWaterTotal` | open | `0/39`, mean abs mean `131.293228`, max abs `565.718633` |

Promotion rule:

- Do not mark HPHYS0249 `GO`; unresolved semantic residuals are known contract
  and continuation blockers.
