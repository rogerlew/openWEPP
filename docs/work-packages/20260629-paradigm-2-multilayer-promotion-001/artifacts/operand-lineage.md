# Operand Lineage

Status: `COMPLETE`

Evidence class: Static.

| Field | Units | Source | Publication Surface | Authority | Notes |
| --- | --- | --- | --- | --- | --- |
| `MeltwaterTemperature` | `degC` | `DirectSnowStage3Diagnostics.meltwater_temperature_c` from R4G snow-coupling Stage 3 diagnostics | nullable `hillslope_wat` parquet column | `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-082` | Intensive flux temperature. Null when the Stage 3 arm is disabled or no routed meltwater temperature exists. Not serialized to HBP in this package. |

Direct-publication consumer proof:
`DirectPublicationWaterTemperatureOperands` reads the post-snow-coupling
`snow_coupling_shadow_projection.stage3_diagnostics` and the runner maps that
operand into `HillslopeWatRow.meltwater_temperature`.
