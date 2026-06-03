# WB17/WB18/Storage Diagnosis

Status: completed

Evidence mode: ran

## Diagnosis

- Ran: HPHYS0260 diagnostics executed under
  `/tmp/hphys0260_20260603T035231Z`.
- Ran: H1/H7/H39 post-`plant_root_uptake` trace rows show
  `Ep = ΣUi_####`, aggregate `Ui = ΣUi_####`, aggregate
  `UPi = ΣUPi_####`, `Ws = Ep/Etp`, and no `Ui > UPi` layer violations.
- Ran: H1/H7/H39 post-WB18 trace rows show `D = Pe` and recomputed aggregate
  storage equals traced `wb11_soil_water`.
- Ran: H1/H7/H39 post-WB13 trace rows show final `Total-Soil` and
  `SoilWaterTotal` equal traced `wb11_soil_water` after unit conversion.

## Interpretation

- Static: the stable day-1 `Ep`, `Dp`, `Total-Soil`, and
  `SoilWaterTotal` residuals are not trace-publication or final-storage
  identity defects.
- Static: continuation should focus baseline-authoritative
  magnitude/initialization lineage.
