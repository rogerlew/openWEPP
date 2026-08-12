# Forcing And Operand Provider Map

Status: `implemented / focused pass`

| Operand | Unit/time/area basis | Provider/owner | Fail-closed behavior |
|---|---|---|---|
| VIS/NIR direct/diffuse, longwave | W m-2 ground, interval-mean | forcing / energy | typed missing/nonfinite |
| precipitation | kg H2O m-2 ground per interval | forcing / hydrology | typed missing/phase unsupported |
| air/canopy reference T, humidity, CO2, pressure, wind | K, kg kg-1, Pa, m s-1 | forcing / atmosphere | typed domain; calm/nonneutral unsupported |
| soil potential, temperature, conductivity, frozen/accessibility | per layer, snapshot | hydrology | typed missing/profile mismatch |
| layer mineral NH4/NO3 | kg N m-2 ground | BGC | typed missing/profile mismatch |
| leaf/stem area and C/N pools | m2 or kg element m-2 ground | vegetation beginning state | typed identity/closure failure |
| caller `dt_s`, topology/tile fractions | s and ground fraction | orchestrator/config | typed nonpositive/sum/identity failure |
| V2 tile-top rain | kg H2O m-2 tile-ground per interval | exact tile forcing map / hydrology handoff | exact tile-set equality; missing/extra/nonfinite/negative fails before routing |
| V2 conditional LAI/WAI | m2 plant m-2 tile-ground | column engine from shared area and `C_s` | exact `LAI_s/C_s`, `WAI_s/C_s`; stand-area poison differs |
| V2 local layer demand/cap/use | kg H2O m-2 tile-ground per interval | occupancy solver; column boundary | exact occupancy/layer identity; multiply/divide by positive `f_t` once |

No operand is profile-averaged. Increment 2A accepts heterogeneous tile rain
explicitly because the current public `SnowFreeForcing.rain_kg_m2` remains
stand-shaped; public forcing integration is intentionally still fail-closed.
Every rate is converted once at the transaction ledger.
