# SIMIMPL21 Legacy Provenance Citation Map

Status: complete
Evidence mode: static+ran
Date: 2026-05-25
Baseline anchor: `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

## Static
| Legacy source anchor | SIMIMPL21 contract targets | Authority role |
|---|---|---|
| `src/evap.for:458-564` | `SC-EVAP-001` | Stage-memory transitions (`s1`, `s2`, `tu`, `tv`) and deficit-coupled `Es` behavior. |
| `src/evap.for:609-668` | `SC-EVAP-001` | Layerwise soil evaporation extraction from `st(i)` with depth-aware allocation. |
| `src/swu.for:122-191` | `SC-EVAP-001`, `SC-PLANT-001` | Root-zone uptake/stress lineage (`UPi`, `Ui`, `watstr`) for ET and plant coupling. |
| `src/watbal.for:486-497,551-552,918-922,958-967` | `SC-WATBAL-001`, `SC-PLANT-001` | Baseline WB11 sequencing and post-uptake aggregate recomputation ordering. |
| `src/watbal.for:960-967` | `SC-WATBAL-001`, `SC-SOIL-001`, `SC-SYSTEM-001` | Aggregate lineage (`st(i)` -> `soilw(i)` -> `watcon`) for publication continuity. |
| `src/outfil.for:623-643` | `SC-WATBAL-001`, `SC-SYSTEM-001` | WB13 publication semantics for `Ep`, `Es`, `Er`, `Total-Soil`, `SoilWaterTotal`. |

## Ran
- `sed -n '450,700p' /workdir/wepp-forest_260430_baseline/src/evap.for`
- `sed -n '110,220p' /workdir/wepp-forest_260430_baseline/src/swu.for`
- `sed -n '470,990p' /workdir/wepp-forest_260430_baseline/src/watbal.for`
- `sed -n '600,700p' /workdir/wepp-forest_260430_baseline/src/outfil.for`
