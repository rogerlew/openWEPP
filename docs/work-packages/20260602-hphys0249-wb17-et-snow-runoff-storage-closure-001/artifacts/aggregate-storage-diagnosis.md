# Aggregate Storage Diagnosis

Status: complete

Evidence mode: ran

Ran:

- Full-suite HPHYS0249 metrics show aggregate storage worsened after replacing
  scalar `Es` over-withdrawal with layer-first upper-zone evaporation.

| Column | HPHYS0248 Mean Abs Mean | HPHYS0249 Mean Abs Mean | Δ | HPHYS0248 Max Abs | HPHYS0249 Max Abs | Δ |
|---|---:|---:|---:|---:|---:|---:|
| `Total-Soil` | 119.105962 | 131.293228 | 12.187267 | 548.715973 | 565.718633 | 17.002660 |
| `SoilWaterTotal` | 119.105962 | 131.293228 | 12.187267 | 548.715973 | 565.718633 | 17.002660 |

Assessment:

- This is expected after correcting WB17 `Es`: the old scalar path removed too
  much water from aggregate storage and partially masked independent storage
  deficits.
- Next storage focus should not revert the WB17 layer extraction. It should
  close `Ep` root-depth/growth activation and snow/runoff timing, then reassess
  `Total-Soil`/`SoilWaterTotal`.
